//! Tables with merged cells: GFM tables with MultiMarkdown's `||` and `^^`, and with several
//! header rows or none.
//!
//! ```text
//! | Row | Valid votes              || Amount per vote |
//! | ^^  | percent | number of votes | ^^              |
//! | --- | ------- | --------------- | --------------- |
//! | 1   | to 5%   |                 | 5.77            |
//! ```
//!
//! * A table is a run of lines starting with `|` (indented by up to three spaces) with
//!   a delimiter row (`| --- | :---: |`) among them. The rows above it are the header, the rows
//!   below it — the body; the delimiter row may come first, for a table without a header.
//!   The table ends with the first line that doesn't start with `|`.
//! * An empty cell without even a space between its pipes (`||`) joins the cell on its left:
//!   `colspan`.
//! * A cell of just `^^` joins the cell above it, when that one starts in the same column and
//!   spans as many columns (`| ^^ ||` under a cell of two columns): `rowspan`. Cells don't merge
//!   across the end of the header — there `^^` stays text.
//! * `\|` is a `|` within a cell. A shorter row gets empty cells, a longer one adds a column.
//!   Alignment from the delimiter row (`:---:`) goes into `align` of the cells.
//!
//! pulldown-cmark knows only plain GFM tables, and its events can't carry `colspan` or
//! `rowspan`. [`events`] gives the whole document as events in which such a table is raw HTML
//! (`Event::Html`: `<table>`, `<tr>`, `<th colspan="2">`…) around the events of each cell's
//! content. `pulldown_cmark::html::push_html` writes them as they are, and
//! [`events_to_vertigo`](crate::events_to_vertigo) builds the elements — with the `html`
//! feature, which renders raw HTML — and gives them the table styles of
//! [`CMarkStyle`](crate::CMarkStyle):
//!
//! ```
//! use vertigo_cmark::{
//!     CMarkStyle, SafeHtml, events_to_vertigo, pulldown_cmark::Options, table_spans,
//! };
//!
//! let text = "| Votes ||\n| --- | --- |\n| yes | no |";
//! let _node = events_to_vertigo(
//!     table_spans::events(text, Options::ENABLE_TABLES),
//!     CMarkStyle::default(),
//!     &SafeHtml,
//! );
//! ```
//!
//! [`Table`] is the table itself — to read ([`Table::parse`]) and write back ([`Table::write`]),
//! e.g. in an editor, or to turn into HTML of other content than Markdown ([`Table::to_html`]).

use std::{iter, ops::Range};

use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};

/// A column in [`Table::write`] is padded to its longest cell, but to no more than this many
/// characters — one long cell would push every row.
pub const MAX_WIDTH: usize = 30;

/// The narrowest column in [`Table::write`]: room for the delimiter row's `:-:`.
const MIN_WIDTH: usize = 3;

const ROWSPAN: &str = "^^";

/// A table with merged cells.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Table {
    /// How many rows from the top make the header (`<thead>`).
    pub head_rows: usize,
    /// Row by row, the cells starting in that row, left to right. Every column of every row
    /// belongs to one cell — starting there or spanning over it.
    pub rows: Vec<Vec<Cell>>,
    /// Alignment of the columns, from the delimiter row; may be shorter than the table is wide.
    pub align: Vec<Align>,
}

/// A cell of a [`Table`].
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Cell {
    /// The column the cell starts in, from 0.
    pub column: usize,
    pub colspan: usize,
    pub rowspan: usize,
    /// The content as written, without the spaces round it and with `\|` as `|`.
    pub text: String,
}

impl Cell {
    /// A cell of one column and one row.
    pub fn new(column: usize, text: impl Into<String>) -> Self {
        Self {
            column,
            colspan: 1,
            rowspan: 1,
            text: text.into(),
        }
    }
}

/// Alignment of a column: `:---`, `:---:`, `---:`.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Align {
    #[default]
    None,
    Left,
    Center,
    Right,
}

/// A part of a table on its way to HTML: a tag, or a cell's content.
enum Piece<'t> {
    Html(&'t str),
    Content(&'t Cell),
}

impl Table {
    /// How many columns the table has.
    pub fn columns(&self) -> usize {
        self.rows
            .iter()
            .flatten()
            .map(|cell| cell.column + cell.colspan)
            .max()
            .unwrap_or(0)
    }

    /// The table written in `text` — the lines of one of [`blocks`]. `None` when they don't
    /// make one: there's no delimiter row, nothing but it, or a line isn't a row.
    pub fn parse(text: &str) -> Option<Self> {
        let lines: Vec<&str> = text
            .lines()
            .filter(|line| !line.trim().is_empty())
            .collect();
        let delimiter = lines.iter().position(|line| is_delimiter_row(line))?;
        if lines.len() < 2 || !lines.iter().all(|line| is_row(line)) {
            return None;
        }
        let align: Vec<Align> = split_row(lines[delimiter])
            .iter()
            .map(|cell| align_of(cell.trim()))
            .collect();
        let written: Vec<Vec<(String, usize)>> = lines
            .iter()
            .enumerate()
            .filter(|(index, _)| *index != delimiter)
            .map(|(_, line)| join_left(split_row(line)))
            .collect();
        let columns = written
            .iter()
            .map(|row| row.iter().map(|(_, span)| span).sum::<usize>())
            .max()
            .unwrap_or(0)
            .max(align.len());

        let head_rows = delimiter;
        let mut rows: Vec<Vec<Cell>> = Vec::with_capacity(written.len());
        // for every place of every row: the row and the index in it of the cell taking it
        let mut owners: Vec<Vec<(usize, usize)>> = Vec::with_capacity(written.len());
        for (index, row) in written.into_iter().enumerate() {
            let width: usize = row.iter().map(|(_, span)| span).sum();
            let padding = iter::repeat_n((String::new(), 1), columns - width);
            let first_of_section = index == 0 || index == head_rows;
            let mut cells = Vec::with_capacity(row.len());
            let mut taken = Vec::with_capacity(columns);
            for (text, colspan) in row.into_iter().chain(padding) {
                let column = taken.len();
                if text == ROWSPAN && !first_of_section {
                    let (above_row, above_index) = owners[index - 1][column];
                    let above = &mut rows[above_row][above_index];
                    if above.column == column && above.colspan == colspan {
                        above.rowspan += 1;
                        taken.extend(iter::repeat_n((above_row, above_index), colspan));
                        continue;
                    }
                }
                taken.extend(iter::repeat_n((index, cells.len()), colspan));
                cells.push(Cell {
                    column,
                    colspan,
                    rowspan: 1,
                    text,
                });
            }
            rows.push(cells);
            owners.push(taken);
        }

        Some(Self {
            head_rows,
            rows,
            align,
        })
    }

    /// The table as text with the columns lined up, `|` in cells as `\|`, and `^^` under
    /// a cell spanning rows. [`Table::parse`] reads it back.
    pub fn write(&self) -> String {
        let columns = self.columns().max(1);
        let rows = self.layout(columns);
        let width = |text: &str| text.chars().count();

        let mut widths = vec![MIN_WIDTH; columns];
        for &(column, _, ref text) in rows.iter().flatten().filter(|(_, span, _)| *span == 1) {
            widths[column] = widths[column].max(width(text).min(MAX_WIDTH));
        }
        // a merged cell longer than its columns widens the last of them, narrower cells first
        for span in 2..=columns {
            for &(column, _, ref text) in rows.iter().flatten().filter(|(_, s, _)| *s == span) {
                let room = spanned(&widths, column, span);
                let last = column + span - 1;
                if width(text) > room {
                    widths[last] =
                        (widths[last] + width(text) - room).min(widths[last].max(MAX_WIDTH));
                }
            }
        }

        let row_line = |row: &[(usize, usize, String)]| {
            let mut line = String::new();
            for (column, span, text) in row {
                let room = spanned(&widths, *column, *span);
                line.push_str("| ");
                line.push_str(text);
                line.extend(iter::repeat_n(' ', room.saturating_sub(width(text))));
                line.push(' ');
                line.extend(iter::repeat_n('|', span - 1));
            }
            line.push('|');
            line
        };
        let mut delimiter = String::new();
        for (column, width) in widths.iter().enumerate() {
            let dashes = |count: usize| "-".repeat(count);
            delimiter.push_str("| ");
            delimiter.push_str(&match self.align.get(column).copied().unwrap_or_default() {
                Align::None => dashes(*width),
                Align::Left => format!(":{}", dashes(width - 1)),
                Align::Center => format!(":{}:", dashes(width - 2)),
                Align::Right => format!("{}:", dashes(width - 1)),
            });
            delimiter.push(' ');
        }
        delimiter.push('|');

        let head = self.head_rows.min(rows.len());
        let mut lines: Vec<String> = rows[..head].iter().map(|row| row_line(row)).collect();
        lines.push(delimiter);
        lines.extend(rows[head..].iter().map(|row| row_line(row)));
        lines.join("\n")
    }

    /// Row by row, what [`Table::write`] puts in the row: `(column, colspan, text)` — escaped
    /// cells, `^^` under cells spanning rows and empty cells where the table has a gap.
    fn layout(&self, columns: usize) -> Vec<Vec<(usize, usize, String)>> {
        // what starts at each place: the colspan and the text
        let mut places: Vec<Vec<Option<(usize, String)>>> =
            vec![vec![None; columns]; self.rows.len()];
        for (index, row) in self.rows.iter().enumerate() {
            for cell in row.iter().filter(|cell| cell.column < columns) {
                places[index][cell.column] = Some((cell.colspan, cell.text.replace('|', "\\|")));
                for below in places.iter_mut().take(index + cell.rowspan).skip(index + 1) {
                    below[cell.column] = Some((cell.colspan, ROWSPAN.to_string()));
                }
            }
        }
        places
            .into_iter()
            .map(|places| {
                let mut row = Vec::with_capacity(columns);
                let mut column = 0;
                while column < columns {
                    // a cell over the places of another one: the first one counts
                    let (span, text) = places[column].clone().unwrap_or((1, String::new()));
                    let span = span.clamp(1, columns - column);
                    row.push((column, span, text));
                    column += span;
                }
                row
            })
            .collect()
    }

    /// The table as HTML: `<table>`, `<thead>` and `<tbody>`, a row in a line, with the content
    /// of each cell made by `content` — for raw HTML, e.g., the text as it is.
    pub fn to_html(&self, mut content: impl FnMut(&str) -> String) -> String {
        let mut html = String::new();
        self.walk(|piece| match piece {
            Piece::Html(tags) => html.push_str(tags),
            Piece::Content(cell) => html.push_str(&content(&cell.text)),
        });
        html
    }

    /// The table as HTML, piece by piece.
    fn walk(&self, mut piece: impl FnMut(Piece<'_>)) {
        let head = self.head_rows.min(self.rows.len());
        piece(Piece::Html("<table>\n"));
        for (index, row) in self.rows.iter().enumerate() {
            if index == 0 && head > 0 {
                piece(Piece::Html("<thead>\n"));
            }
            if index == head {
                piece(Piece::Html("<tbody>\n"));
            }
            piece(Piece::Html("<tr>"));
            let name = if index < head { "th" } else { "td" };
            for cell in row {
                let mut open = format!("<{name}");
                if cell.colspan > 1 {
                    open.push_str(&format!(" colspan=\"{}\"", cell.colspan));
                }
                if cell.rowspan > 1 {
                    open.push_str(&format!(" rowspan=\"{}\"", cell.rowspan));
                }
                match self.align.get(cell.column).copied().unwrap_or_default() {
                    Align::None => {}
                    Align::Left => open.push_str(" align=\"left\""),
                    Align::Center => open.push_str(" align=\"center\""),
                    Align::Right => open.push_str(" align=\"right\""),
                }
                open.push('>');
                piece(Piece::Html(&open));
                piece(Piece::Content(cell));
                piece(Piece::Html(&format!("</{name}>")));
            }
            piece(Piece::Html("</tr>\n"));
            if index + 1 == head {
                piece(Piece::Html("</thead>\n"));
            }
        }
        if self.rows.len() > head {
            piece(Piece::Html("</tbody>\n"));
        }
        // no line break after it: in a vertigo tree it would be a text node next to the table
        piece(Piece::Html("</table>"));
    }
}

/// Width of a cell spanning `span` columns from `column`, so that it ends where they end.
fn spanned(widths: &[usize], column: usize, span: usize) -> usize {
    widths[column..column + span].iter().sum::<usize>() + 2 * (span - 1)
}

/// A line of a table: `|` after at most three spaces.
pub fn is_row(line: &str) -> bool {
    let rest = line.trim_start_matches(' ');
    line.len() - rest.len() <= 3 && rest.starts_with('|')
}

/// The delimiter row under the header: `| --- | :---: | ---: |`.
pub fn is_delimiter_row(line: &str) -> bool {
    if !is_row(line) {
        return false;
    }
    let cells = split_row(line);
    !cells.is_empty()
        && cells.iter().all(|cell| {
            let cell = cell.trim();
            let cell = cell.strip_prefix(':').unwrap_or(cell);
            let cell = cell.strip_suffix(':').unwrap_or(cell);
            !cell.is_empty() && cell.bytes().all(|byte| byte == b'-')
        })
}

fn align_of(cell: &str) -> Align {
    match (cell.starts_with(':'), cell.ends_with(':')) {
        (true, true) => Align::Center,
        (true, false) => Align::Left,
        (false, true) => Align::Right,
        (false, false) => Align::None,
    }
}

/// The cells of a row as written between its pipes; `\|` doesn't split. The pipe at the end
/// of the row may be left out.
fn split_row(line: &str) -> Vec<&str> {
    let Some(line) = line.trim_start_matches(' ').strip_prefix('|') else {
        return Vec::new();
    };
    let bytes = line.as_bytes();
    let mut cells = Vec::new();
    let (mut start, mut index) = (0, 0);
    while index < bytes.len() {
        match bytes[index] {
            b'\\' if bytes.get(index + 1) == Some(&b'|') => index += 2,
            b'|' => {
                cells.push(&line[start..index]);
                start = index + 1;
                index += 1;
            }
            _ => index += 1,
        }
    }
    let rest = &line[start..];
    if !rest.trim().is_empty() {
        cells.push(rest);
    }
    cells
}

/// Cells with the empty ones (`||`) joined to the cell on their left: `(text, colspan)`.
fn join_left(cells: Vec<&str>) -> Vec<(String, usize)> {
    let mut joined: Vec<(String, usize)> = Vec::with_capacity(cells.len());
    for cell in cells {
        match joined.last_mut() {
            Some((_, span)) if cell.is_empty() => *span += 1,
            _ => joined.push((cell.trim().replace("\\|", "|"), 1)),
        }
    }
    joined
}

/// The lines of the text: `(start, end)`, the end before `\r\n` or `\n`.
fn lines(text: &str) -> impl Iterator<Item = (usize, usize)> + '_ {
    let mut start = 0;
    text.split_inclusive('\n').map(move |line| {
        let content = line.trim_end_matches('\n').trim_end_matches('\r');
        let range = (start, start + content.len());
        start += line.len();
        range
    })
}

/// Where tables are in the text, judged by its lines alone: runs of rows with a delimiter row
/// and at least one other row. In Markdown some of them may be in code or in raw HTML —
/// [`find`] leaves those out.
pub fn blocks(text: &str) -> Vec<Range<usize>> {
    let mut blocks = Vec::new();
    // the run of rows so far: its range, whether it has a delimiter row and how many rows
    let mut run: Option<(Range<usize>, bool, usize)> = None;
    let mut finish = |run: Option<(Range<usize>, bool, usize)>| {
        if let Some((range, true, 2..)) = run {
            blocks.push(range);
        }
    };
    for (start, end) in lines(text) {
        let line = &text[start..end];
        if !is_row(line) {
            finish(run.take());
            continue;
        }
        let delimiter = is_delimiter_row(line);
        match &mut run {
            Some((range, has_delimiter, count)) => {
                range.end = end;
                *has_delimiter |= delimiter;
                *count += 1;
            }
            None => run = Some((start..end, delimiter, 1)),
        }
    }
    finish(run);
    blocks
}

/// The tables of a Markdown text, where Markdown has them: not in code, raw HTML, quotations,
/// lists or footnotes — pulldown-cmark reads plain GFM tables there.
pub fn find(text: &str, options: Options) -> Vec<(Range<usize>, Table)> {
    let candidates = blocks(text);
    if candidates.is_empty() {
        return Vec::new();
    }
    let elsewhere: Vec<Range<usize>> = Parser::new_ext(text, options)
        .into_offset_iter()
        .filter_map(|(event, range)| {
            matches!(
                event,
                Event::Start(
                    Tag::CodeBlock(_)
                        | Tag::HtmlBlock
                        | Tag::BlockQuote(_)
                        | Tag::List(_)
                        | Tag::FootnoteDefinition(_)
                )
            )
            .then_some(range)
        })
        .collect();
    candidates
        .into_iter()
        .filter(|block| {
            !elsewhere
                .iter()
                .any(|range| range.start < block.end && block.start < range.end)
        })
        .filter_map(|block| Table::parse(&text[block.clone()]).map(|table| (block, table)))
        .collect()
}

/// The events of a Markdown text — those of `Parser::new_ext(text, options)`, but tables with
/// merged cells (see the [module](self)) come as raw HTML around the events of their cells'
/// content. Without [`Options::ENABLE_TABLES`] there are no tables of either kind.
///
/// Each table gives way to a placeholder before the text is parsed, so the parser still reads
/// the document as a whole: footnotes and link definitions work around the tables. A cell's
/// content is read as the cell of a GFM table — inline only, `# text` is no heading — and
/// a footnote reference in it refers to the footnotes of the document.
pub fn events(text: &str, options: Options) -> Vec<Event<'static>> {
    let tables = match options.contains(Options::ENABLE_TABLES) {
        true => find(text, options),
        false => Vec::new(),
    };
    if tables.is_empty() {
        return Parser::new_ext(text, options)
            .map(Event::into_static)
            .collect();
    }

    let mut source = String::with_capacity(text.len());
    let mut last = 0;
    for (index, (range, _)) in tables.iter().enumerate() {
        source.push_str(&text[last..range.start]);
        source.push_str(&format!("\n\n{}\n\n", placeholder(index)));
        last = range.end;
    }
    source.push_str(&text[last..]);

    let parsed: Vec<Event<'_>> = Parser::new_ext(&source, options).collect();
    let labels: Vec<String> = parsed
        .iter()
        .filter_map(|event| match event {
            Event::Start(Tag::FootnoteDefinition(label)) => Some(label.to_string()),
            _ => None,
        })
        .collect();

    let mut out = Vec::with_capacity(parsed.len());
    let mut next = 0;
    let mut index = 0;
    while index < parsed.len() {
        if let [
            Event::Start(Tag::HtmlBlock),
            Event::Html(html),
            Event::End(TagEnd::HtmlBlock),
            ..,
        ] = &parsed[index..]
            && let Some((_, table)) = tables.get(next)
            && html.trim_end() == placeholder(next)
        {
            table_events(table, &labels, options, &mut out);
            next += 1;
            index += 3;
            continue;
        }
        out.push(parsed[index].clone().into_static());
        index += 1;
    }
    out
}

fn placeholder(index: usize) -> String {
    format!("<!--table-spans-{index}-->")
}

fn table_events(table: &Table, labels: &[String], options: Options, out: &mut Vec<Event<'static>>) {
    let mut tags = String::new();
    let flush = |tags: &mut String, out: &mut Vec<Event<'static>>| {
        if !tags.is_empty() {
            out.push(Event::Html(std::mem::take(tags).into()));
        }
    };
    table.walk(|piece| match piece {
        Piece::Html(html) => tags.push_str(html),
        Piece::Content(cell) => {
            flush(&mut tags, out);
            cell_events(&cell.text, labels, options, out);
        }
    });
    flush(&mut tags, out);
}

/// The events of a cell's content: what pulldown-cmark makes of it as the cell of a GFM table,
/// with the document's footnotes defined.
fn cell_events(text: &str, labels: &[String], options: Options, out: &mut Vec<Event<'static>>) {
    if text.is_empty() {
        return;
    }
    let mut source = format!("| {} |\n| - |\n", text.replace('|', "\\|"));
    for label in labels {
        source.push_str(&format!("\n[^{label}]: -\n"));
    }
    let mut inside = false;
    for event in Parser::new_ext(&source, options) {
        match event {
            Event::Start(Tag::TableCell) => inside = true,
            Event::End(TagEnd::TableCell) => break,
            event if inside => out.push(event.into_static()),
            _ => {}
        }
    }
}
