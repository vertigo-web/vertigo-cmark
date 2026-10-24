use pulldown_cmark::Options;
use vertigo::{
    Css,
    dev::inspect::{DomDebugFragment, log_start},
    dom,
};

use crate::{
    CMarkStyle, SafeHtml, events_to_vertigo,
    table_spans::{Align, Cell, Table, blocks, events, find, is_delimiter_row},
};

const VOTES: &str = "\
| Wiersz | Głosy ważne            || Kwota za głos |
| ^^     | procent | liczba głosów | ^^            |
| ------ | ------- | ------------- | ------------- |
| 1      | do 5%   |               | 5,77 zł       |";

fn cell(column: usize, colspan: usize, rowspan: usize, text: &str) -> Cell {
    Cell {
        column,
        colspan,
        rowspan,
        text: text.to_string(),
    }
}

fn options() -> Options {
    Options::ENABLE_TABLES | Options::ENABLE_FOOTNOTES
}

#[cfg(feature = "html")]
fn html(text: &str) -> String {
    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, events(text, options()).into_iter());
    html
}

#[test]
fn merged_cells_and_two_header_rows() {
    assert_eq!(
        Table::parse(VOTES),
        Some(Table {
            head_rows: 2,
            rows: vec![
                vec![
                    cell(0, 1, 2, "Wiersz"),
                    cell(1, 2, 1, "Głosy ważne"),
                    cell(3, 1, 2, "Kwota za głos"),
                ],
                vec![cell(1, 1, 1, "procent"), cell(2, 1, 1, "liczba głosów")],
                vec![
                    cell(0, 1, 1, "1"),
                    cell(1, 1, 1, "do 5%"),
                    cell(2, 1, 1, ""),
                    cell(3, 1, 1, "5,77 zł"),
                ],
            ],
            align: vec![Align::None; 4],
        })
    );
}

#[test]
fn written_back_as_read() {
    let table = Table::parse(VOTES).unwrap();
    assert_eq!(table.write(), VOTES);
    for text in [
        "| a | b |\n|---|:-:|\n| 1 | 2 |",
        "|---|---|\n| bez | nagłówka |",
        "| tylko nagłówek |\n|---|",
        "| a || b |\n| ^^ || c |\n|---|---|---|\n| 1 | 2 | 3 |",
        "| a\\|b | c |\n|---|---|",
    ] {
        let table = Table::parse(text).unwrap();
        assert_eq!(Table::parse(&table.write()), Some(table), "{text}");
    }
}

#[test]
fn rows_span_cells_of_the_same_width() {
    // `| ^^ ||` under a cell of two columns
    let table = Table::parse("| a || b |\n| ^^ || c |\n|---|---|---|").unwrap();
    assert_eq!(
        table.rows,
        vec![
            vec![cell(0, 2, 2, "a"), cell(2, 1, 1, "b")],
            vec![cell(2, 1, 1, "c")],
        ]
    );
    // under a cell of another width, at the top and across the end of the header — text
    let table =
        Table::parse("| ^^ | a |\n| b || c |\n| ^^ | d | e |\n|---|---|---|\n| ^^ | f | g |")
            .unwrap();
    assert_eq!(
        table.rows,
        vec![
            vec![cell(0, 1, 1, "^^"), cell(1, 1, 1, "a"), cell(2, 1, 1, "")],
            vec![cell(0, 2, 1, "b"), cell(2, 1, 1, "c")],
            vec![cell(0, 1, 1, "^^"), cell(1, 1, 1, "d"), cell(2, 1, 1, "e")],
            vec![cell(0, 1, 1, "^^"), cell(1, 1, 1, "f"), cell(2, 1, 1, "g")],
        ]
    );
}

#[test]
fn short_and_long_rows() {
    let table = Table::parse("| a | b |\n|---|---|\n| 1 |\n| 1 | 2 | 3\n| x\\|y | z |  ").unwrap();
    assert_eq!(table.columns(), 3);
    assert_eq!(
        table.rows,
        vec![
            vec![cell(0, 1, 1, "a"), cell(1, 1, 1, "b"), cell(2, 1, 1, "")],
            vec![cell(0, 1, 1, "1"), cell(1, 1, 1, ""), cell(2, 1, 1, "")],
            vec![cell(0, 1, 1, "1"), cell(1, 1, 1, "2"), cell(2, 1, 1, "3")],
            vec![cell(0, 1, 1, "x|y"), cell(1, 1, 1, "z"), cell(2, 1, 1, "")],
        ]
    );
    // an empty cell at the start of a row has nothing to join
    let table = Table::parse("|| a |\n|---|---|").unwrap();
    assert_eq!(
        table.rows,
        vec![vec![cell(0, 1, 1, ""), cell(1, 1, 1, "a")]]
    );
}

#[test]
fn not_tables() {
    assert_eq!(Table::parse("| a | b |\n| c | d |"), None);
    assert_eq!(Table::parse("|---|"), None);
    assert_eq!(Table::parse("| a |\n|---|\nb"), None);
    assert!(is_delimiter_row("  | :--- | ---: | :-: |"));
    assert!(!is_delimiter_row("    |---|"));
    assert!(!is_delimiter_row("|---|a|"));
    assert!(!is_delimiter_row("---|---"));
}

#[test]
fn columns_lined_up() {
    let table = Table {
        head_rows: 1,
        rows: vec![
            vec![cell(0, 2, 1, "Tytuł")],
            vec![cell(0, 1, 1, "Zażółć"), cell(1, 1, 1, "a|b")],
            vec![cell(0, 1, 1, &"x".repeat(40)), cell(1, 1, 1, "")],
        ],
        align: vec![Align::Left, Align::Center],
    };
    assert_eq!(
        table.write(),
        format!(
            "| Tytuł                                ||\n\
             | :----------------------------- | :--: |\n\
             | Zażółć                         | a\\|b |\n\
             | {} |      |",
            "x".repeat(40)
        )
    );
    // a merged cell longer than its columns widens the last one
    let table = Table {
        head_rows: 1,
        rows: vec![
            vec![cell(0, 2, 1, "Długi tytuł nad dwiema")],
            vec![cell(0, 1, 1, "a"), cell(1, 1, 1, "b")],
        ],
        align: Vec::new(),
    };
    assert_eq!(
        table.write(),
        "| Długi tytuł nad dwiema ||\n\
         | --- | ----------------- |\n\
         | a   | b                 |"
    );
}

#[test]
fn table_blocks() {
    let text =
        "Tekst\n| a |\n|---|\n| 1 |\nDalej\n\n| bez | separatora |\n\n|---|\n\n  | b |\r\n  |---|";
    let found: Vec<&str> = blocks(text).into_iter().map(|range| &text[range]).collect();
    assert_eq!(found, ["| a |\n|---|\n| 1 |", "  | b |\r\n  |---|"]);
}

#[test]
fn markdown_tables_only() {
    let text =
        "```\n| a |\n|---|\n```\n\n> | b |\n> |---|\n\n<div>\n| c |\n|---|\n</div>\n\n| d |\n|---|";
    let found: Vec<&str> = find(text, options())
        .into_iter()
        .map(|(range, _)| &text[range])
        .collect();
    assert_eq!(found, ["| d |\n|---|"]);
}

#[cfg(feature = "html")]
#[test]
fn merged_cells_as_html() {
    assert_eq!(
        html(VOTES),
        "<table>\n<thead>\n\
         <tr><th rowspan=\"2\">Wiersz</th><th colspan=\"2\">Głosy ważne</th><th rowspan=\"2\">Kwota za głos</th></tr>\n\
         <tr><th>procent</th><th>liczba głosów</th></tr>\n\
         </thead>\n<tbody>\n\
         <tr><td>1</td><td>do 5%</td><td></td><td>5,77 zł</td></tr>\n\
         </tbody>\n</table>"
    );
    assert_eq!(
        html("|---|:-:|\n| a | b |"),
        "<table>\n<tbody>\n<tr><td>a</td><td align=\"center\">b</td></tr>\n</tbody>\n</table>"
    );
}

#[cfg(feature = "html")]
#[test]
fn cells_are_inline_markdown() {
    assert_eq!(
        html(
            "Zdanie[^1].\n| **a** `x\\|y` | # b |\n|---|---|\n| [c](/c) | - d |\n\n[^1]: Źródło.\n"
        ),
        "<p>Zdanie<sup class=\"footnote-reference\"><a href=\"#1\">1</a></sup>.</p>\n\
         <table>\n<thead>\n\
         <tr><th><strong>a</strong> <code>x|y</code></th><th># b</th></tr>\n\
         </thead>\n<tbody>\n\
         <tr><td><a href=\"/c\">c</a></td><td>- d</td></tr>\n\
         </tbody>\n</table>\n\
         <div class=\"footnote-definition\" id=\"1\"><sup class=\"footnote-definition-label\">1</sup>\n\
         <p>Źródło.</p>\n</div>\n"
    );
    // a footnote referred to only from a table
    let rendered = html("| a[^x] |\n|---|\n\n[^x]: Przypis.");
    assert!(
        rendered.contains("<th>a<sup class=\"footnote-reference\"><a href=\"#x\">1</a></sup></th>"),
        "{rendered}"
    );
}

#[cfg(feature = "html")]
#[test]
fn plain_markdown_stays_as_it_was() {
    for text in [
        "Tekst\n\n```\n| a |\n|---|\n```",
        "> | a |\n> |---|",
        "a | b\n--|--\n1 | 2",
        "| a |\n| b |",
    ] {
        let mut plain = String::new();
        pulldown_cmark::html::push_html(
            &mut plain,
            pulldown_cmark::Parser::new_ext(text, options()),
        );
        assert_eq!(html(text), plain, "{text}");
    }
    // without tables in the options there are none
    let mut rendered = String::new();
    pulldown_cmark::html::push_html(
        &mut rendered,
        events("| a |\n|---|", Options::empty()).into_iter(),
    );
    assert_eq!(rendered, "<p>| a |\n|---|</p>\n");
}

#[cfg(feature = "html")]
#[test]
fn table_right_under_a_paragraph() {
    assert_eq!(
        html("Tekst\n| a |\n|---|\n| 1 |\nDalej"),
        "<p>Tekst</p>\n\
         <table>\n<thead>\n<tr><th>a</th></tr>\n</thead>\n<tbody>\n<tr><td>1</td></tr>\n</tbody>\n</table>\n\
         <p>Dalej</p>\n"
    );
}

#[cfg(feature = "html")]
#[test]
fn rendered_like_markdown_tables() {
    let style = CMarkStyle {
        table: Css::str("width: 100%"),
        th: Css::str("font-weight: bold"),
        ..CMarkStyle::default()
    };

    log_start();
    let _el1 = events_to_vertigo(
        events("| a ||\n|---|---|\n| b | c |", options()),
        style,
        &SafeHtml,
    );
    let el1_str = DomDebugFragment::from_log().to_pseudo_html();

    log_start();
    let _el2 = dom! {
        <div>
            <table style="width: 100%">
                <thead>
                    <tr>
                        <th colspan="2" style="font-weight: bold">"a"</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>"b"</td>
                        <td>"c"</td>
                    </tr>
                </tbody>
            </table>
        </div>
    };
    let el2_str = DomDebugFragment::from_log().to_pseudo_html();

    assert_eq!(el1_str, el2_str);
}
