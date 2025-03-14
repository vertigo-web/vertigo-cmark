// Based on https://github.com/pulldown-cmark/pulldown-cmark/blob/master/pulldown-cmark/src/html.rs

use pulldown_cmark::{
    Alignment, BlockQuoteKind, CodeBlockKind, CowStr, Event, Event::*, HeadingLevel, LinkType, Tag,
    TagEnd,
};
use std::collections::{HashMap, VecDeque};
use vertigo::{log, DomElement, DomNode, DomText};

enum TableState {
    Head,
    Body,
}

struct VertigoWriter<'a, I> {
    /// Iterator supplying events.
    iter: I,

    /// Whether if inside a metadata block (text should not be written)
    in_non_writing_block: bool,

    table_state: TableState,
    table_alignments: Vec<Alignment>,
    table_cell_index: usize,
    numbers: HashMap<CowStr<'a>, usize>,

    // Stack of nested nodes
    soc: VecDeque<DomNode>,
}

impl<'a, I> VertigoWriter<'a, I>
where
    I: Iterator<Item = Event<'a>>,
{
    fn new(iter: I) -> Self {
        Self {
            iter,
            in_non_writing_block: false,
            table_state: TableState::Head,
            table_alignments: vec![],
            table_cell_index: 0,
            numbers: HashMap::new(),
            soc: VecDeque::new(),
        }
    }

    fn run(mut self) -> DomNode {
        self.push_elname("div");
        while let Some(event) = self.iter.next() {
            println!("Event: {:?}", event);
            match event {
                Start(tag) => {
                    self.start_tag(tag);
                }
                End(tag) => {
                    self.end_tag(tag);
                }
                Text(text) => {
                    if !self.in_non_writing_block {
                        self.add_child(DomText::new(text));
                    }
                }
                Code(text) => {
                    let element = DomElement::new("code").child(DomText::new(text));
                    self.add_child(element);
                }
                InlineMath(text) => {
                    let element = DomElement::new("span")
                        .attr("class", "math math-inline")
                        .child(DomText::new(text));
                    self.add_child(element);
                }
                DisplayMath(text) => {
                    let element = DomElement::new("span")
                        .attr("class", "math math-display")
                        .child(DomText::new(text));
                    self.add_child(element);
                }
                Html(_html) | InlineHtml(_html) => {
                    // TODO:
                }
                SoftBreak => {
                    // Add space to not glue sibling texts in render
                    self.add_child(DomText::new(" "));
                }
                HardBreak => {
                    self.add_child_name("br");
                }
                Rule => {
                    self.add_child_name("hr");
                }
                FootnoteReference(name) => {
                    let len = self.numbers.len() + 1;
                    let link = DomElement::new("a").attr("href", ["#", name.as_ref()].concat());
                    let number = *self.numbers.entry(name).or_insert(len);
                    link.add_child_text(number.to_string());
                    self.add_child(
                        DomElement::new("sup")
                            .attr("class", "footnote-reference")
                            .child(link),
                    );
                }
                TaskListMarker(true) => {
                    self.add_child(
                        DomElement::new("input")
                            .attr("disabled", "")
                            .attr("type", "checkbox")
                            .attr("checked", "checked"),
                    );
                }
                TaskListMarker(false) => {
                    self.add_child(
                        DomElement::new("input")
                            .attr("disabled", "")
                            .attr("type", "checkbox"),
                    );
                }
            }
        }
        self.pop_node().unwrap_or_else(|| {
            log::error!("Popping nesting did not produce root node!");
            DomElement::new("div").into()
        })
    }

    /// Pushes dom element on stack
    fn start_tag(&mut self, tag: Tag<'a>) {
        match &tag {
            Tag::HtmlBlock => {}
            Tag::Paragraph => {
                self.push_elname("p");
            }
            Tag::Heading {
                level,
                id,
                classes,
                attrs: _, // Vertigo doesn't support dynamic attributes keys
            } => {
                let el_name = match level {
                    HeadingLevel::H1 => "h1",
                    HeadingLevel::H2 => "h2",
                    HeadingLevel::H3 => "h3",
                    HeadingLevel::H4 => "h4",
                    HeadingLevel::H5 => "h5",
                    HeadingLevel::H6 => "h6",
                };
                let element = DomElement::new(el_name);
                if let Some(id) = id {
                    element.add_attr("id", id);
                }
                if !classes.is_empty() {
                    let value = classes.join(" ");
                    element.add_attr("class", value);
                }
                self.push_node(element);
            }
            Tag::Table(alignments) => {
                self.table_alignments = alignments.clone();
                self.push_node(DomElement::new("table").attr("border", "1"))
            }
            Tag::TableHead => {
                self.table_state = TableState::Head;
                self.table_cell_index = 0;
                self.push_elname("thead");
                self.push_elname("tr");
            }
            Tag::TableRow => {
                self.table_cell_index = 0;
                self.push_elname("tr");
            }
            Tag::TableCell => {
                let el_name = match self.table_state {
                    TableState::Head => "th",
                    TableState::Body => "td",
                };
                let element = DomElement::new(el_name);
                match self.table_alignments.get(self.table_cell_index) {
                    Some(&Alignment::Left) => element.add_attr("style", "text-align: left"),
                    Some(&Alignment::Center) => element.add_attr("style", "text-align: center"),
                    Some(&Alignment::Right) => element.add_attr("style", "text-align: right"),
                    _ => (),
                }
                self.push_node(element);
            }
            Tag::CodeBlock(info) => {
                self.push_elname("pre");
                let element = DomElement::new("code");
                match info {
                    CodeBlockKind::Fenced(info) => {
                        let lang = info.split(' ').next().unwrap_or_default();
                        if !lang.is_empty() {
                            element.add_attr("class", format!("language-{lang}"));
                        }
                    }
                    CodeBlockKind::Indented => {}
                };
                self.push_node(element);
            }
            Tag::BlockQuote(kind) => {
                let element = DomElement::new("blockquote");

                if let Some(kind) = kind {
                    let kind_value = match kind {
                        BlockQuoteKind::Note => "markdown-alert-note",
                        BlockQuoteKind::Tip => "markdown-alert-tip",
                        BlockQuoteKind::Important => "markdown-alert-important",
                        BlockQuoteKind::Warning => "markdown-alert-warning",
                        BlockQuoteKind::Caution => "markdown-alert-caution",
                    };
                    element.add_attr("class", kind_value);
                };
                self.push_node(element);
            }
            Tag::List(Some(1)) => self.push_elname("ol"),
            Tag::List(Some(start)) => {
                self.push_node(DomElement::new("ol").attr("start", start));
            }
            Tag::List(None) => self.push_elname("ul"),
            Tag::Item => self.push_elname("li"),
            Tag::DefinitionList => self.push_elname("dl"),
            Tag::DefinitionListTitle => self.push_elname("dt"),
            Tag::DefinitionListDefinition => self.push_elname("dd"),
            Tag::Subscript => self.push_elname("sub"),
            Tag::Superscript => self.push_elname("sup"),
            Tag::Emphasis => self.push_elname("em"),
            Tag::Strong => self.push_elname("strong"),
            Tag::Strikethrough => self.push_elname("del"),
            Tag::Link {
                link_type,
                dest_url,
                title,
                id: _,
            } => {
                let prefix = match link_type {
                    LinkType::Email => "mailto:",
                    _ => "",
                };
                let element = DomElement::new("a").attr("href", [prefix, dest_url].concat());
                if !title.is_empty() {
                    element.add_attr("title", title);
                }
                self.push_node(element);
            }
            Tag::Image {
                link_type: _,
                dest_url,
                title,
                id: _,
            } => {
                let element = DomElement::new("img")
                    .attr("src", dest_url)
                    .attr("alt", self.raw_text());
                if !title.is_empty() {
                    element.add_attr("title", title);
                }
                self.add_child(element);
            }
            Tag::FootnoteDefinition(name) => {
                let len = self.numbers.len() + 1;
                let number = *self.numbers.entry(name.clone()).or_insert(len);
                self.push_node(
                    DomElement::new("div")
                        .attr("class", "footnote-definition")
                        .attr("id", name)
                        .child(
                            DomElement::new("sup")
                                .attr("class", "footnote-definition-label")
                                .child(DomText::new(number.to_string())),
                        ),
                );
            }
            Tag::MetadataBlock(_) => {
                self.in_non_writing_block = true;
            }
        }
    }

    fn end_tag(&mut self, tag: TagEnd) {
        match tag {
            TagEnd::HtmlBlock => {}
            TagEnd::Table => {
                // </tbody></table>
                self.pop_node();
                self.pop_node();
            }
            TagEnd::TableHead => {
                // </tr></thead><tbody>
                self.pop_node();
                self.pop_node();
                self.push_elname("tbody");
                self.table_state = TableState::Body;
            }
            TagEnd::TableCell => {
                self.pop_node();
                self.table_cell_index += 1;
            }
            TagEnd::CodeBlock => {
                // </code></pre>
                self.pop_node();
                self.pop_node();
            }
            TagEnd::TableRow
            | TagEnd::Paragraph
            | TagEnd::Heading(_)
            | TagEnd::BlockQuote(_)
            | TagEnd::List(_)
            | TagEnd::Item
            | TagEnd::DefinitionList
            | TagEnd::DefinitionListTitle
            | TagEnd::DefinitionListDefinition
            | TagEnd::Subscript
            | TagEnd::Superscript
            | TagEnd::Emphasis
            | TagEnd::Strong
            | TagEnd::Strikethrough
            | TagEnd::Link
            | TagEnd::FootnoteDefinition => {
                self.pop_node();
            }
            TagEnd::Image => {} // shouldn't happen, handled in start
            TagEnd::MetadataBlock(_) => {
                self.in_non_writing_block = false;
            }
        }
    }

    // run raw text, consuming end tag
    fn raw_text(&mut self) -> String {
        let mut nest = 0;
        let mut writer = String::new();
        for event in self.iter.by_ref() {
            match event {
                Start(_) => nest += 1,
                End(_) => {
                    if nest == 0 {
                        break;
                    }
                    nest -= 1;
                }
                Html(_) => {}
                InlineHtml(text) | Code(text) | Text(text) => {
                    writer.push_str(&text);
                }
                InlineMath(text) => {
                    writer.push('$');
                    writer.push_str(&text);
                    writer.push('$');
                }
                DisplayMath(text) => {
                    writer.push_str("$$");
                    writer.push_str(&text);
                    writer.push_str("$$");
                }
                SoftBreak | HardBreak | Rule => {
                    writer.push(' ');
                }
                FootnoteReference(name) => {
                    let len = self.numbers.len() + 1;
                    let number = *self.numbers.entry(name).or_insert(len);
                    writer.push_str(&format!("[{}]", number));
                }
                TaskListMarker(true) => {
                    writer.push_str("[x]");
                }
                TaskListMarker(false) => {
                    writer.push_str("[ ]");
                }
            }
        }
        writer
    }

    fn push_node(&mut self, node: impl Into<DomNode>) {
        self.soc.push_front(node.into());
    }

    fn push_elname(&mut self, name: impl Into<String>) {
        let name = name.into();
        self.push_node(DomElement::new(name));
    }

    fn pop_node(&mut self) -> Option<DomNode> {
        if let Some(child) = self.soc.pop_front() {
            match self.soc.front_mut() {
                Some(parent) => {
                    match parent {
                        DomNode::Node { node } => node.add_child(child),
                        _ => {
                            unreachable!("Can't push children to non-element node");
                        }
                    }
                    return None;
                }
                None => return Some(child),
            }
        }
        None
    }

    fn add_child(&mut self, child: impl Into<DomNode>) {
        if let Some(parent) = self.soc.front_mut() {
            match parent {
                DomNode::Node { node } => node.add_child(child),
                _ => log::error!("Can't push child to non-element node (2)"),
            }
        } else {
            log::error!("Can't add child without parent node")
        }
    }

    fn add_child_name(&mut self, child_name: impl Into<String>) {
        self.add_child(DomElement::new(child_name.into()));
    }
}

/// Iterate over an iterator of pulldown's events, generate DomNode for each `Event`,
/// structure it into DOM tree and return the root node.
pub fn generate_tree<'a, I>(iter: I) -> DomNode
where
    I: Iterator<Item = Event<'a>>,
{
    VertigoWriter::new(iter).run()
}
