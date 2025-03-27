// Based on https://github.com/pulldown-cmark/pulldown-cmark/blob/master/pulldown-cmark/src/html.rs

use pulldown_cmark::{
    Alignment, BlockQuoteKind, CodeBlockKind, CowStr, Event, Event::*, HeadingLevel, LinkType, Tag,
    TagEnd,
};
use std::{
    collections::{HashMap, VecDeque},
    rc::Rc,
};
use vertigo::{log, Css, DomElement, DomNode, DomText};

#[cfg(feature = "syntect")]
use crate::highlighting::highlight;
use crate::styling::CMarkStyle;

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

    styling: Rc<CMarkStyle>,

    #[cfg(feature = "syntect")]
    in_code_block: Option<CowStr<'a>>,
}

impl<'a, I> VertigoWriter<'a, I>
where
    I: Iterator<Item = Event<'a>>,
{
    fn new(iter: I, styling: CMarkStyle) -> Self {
        Self {
            iter,
            in_non_writing_block: false,
            table_state: TableState::Head,
            table_alignments: vec![],
            table_cell_index: 0,
            numbers: HashMap::new(),
            soc: VecDeque::new(),
            styling: Rc::new(styling),
            #[cfg(feature = "syntect")]
            in_code_block: None,
        }
    }

    fn run(mut self) -> DomNode {
        self.push_element_styled(DomElement::new("div"), &self.styling.clone().container);
        while let Some(event) = self.iter.next() {
            match event {
                Start(tag) => {
                    self.start_tag(tag);
                }
                End(tag) => {
                    self.end_tag(tag);
                }
                Text(text) => {
                    if !self.in_non_writing_block {
                        #[cfg(feature = "syntect")]
                        if let Some(ref info) = self.in_code_block {
                            for el in highlight(info, &text) {
                                self.add_child(el)
                            }
                        } else {
                            self.add_child(DomText::new(text));
                        }
                        #[cfg(not(feature = "syntect"))]
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
                    let mut element = DomElement::new("sup")
                        .attr("class", "footnote-reference")
                        .child(link);
                    if !self.styling.sup.groups.is_empty() {
                        element = element.css(&self.styling.sup);
                    }
                    self.add_child(element);
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
        let styling = self.styling.clone();
        match &tag {
            Tag::HtmlBlock => {}
            Tag::Paragraph => {
                self.push_elname("p", &styling.p);
            }
            Tag::Heading {
                level,
                id,
                classes,
                attrs: _, // Vertigo doesn't support dynamic attributes keys
            } => {
                let (el_name, css) = match level {
                    HeadingLevel::H1 => ("h1", &styling.h1),
                    HeadingLevel::H2 => ("h2", &styling.h2),
                    HeadingLevel::H3 => ("h3", &styling.h3),
                    HeadingLevel::H4 => ("h4", &styling.h4),
                    HeadingLevel::H5 => ("h5", &styling.h5),
                    HeadingLevel::H6 => ("h6", &styling.h6),
                };
                let element = DomElement::new(el_name);
                if let Some(id) = id {
                    element.add_attr("id", id);
                }
                if !classes.is_empty() {
                    let value = classes.join(" ");
                    element.add_attr("class", value);
                }
                self.push_element_styled(element, css);
            }
            Tag::Table(alignments) => {
                self.table_alignments = alignments.clone();
                self.push_element_styled(DomElement::new("table"), &styling.table)
            }
            Tag::TableHead => {
                self.table_state = TableState::Head;
                self.table_cell_index = 0;
                self.push_elname("thead", &styling.thead);
                self.push_elname("tr", &styling.tr);
            }
            Tag::TableRow => {
                self.table_cell_index = 0;
                self.push_elname("tr", &styling.tr);
            }
            Tag::TableCell => {
                let (el_name, style) = match self.table_state {
                    TableState::Head => ("th", &styling.th),
                    TableState::Body => ("td", &styling.td),
                };
                let element = DomElement::new(el_name);
                match self.table_alignments.get(self.table_cell_index) {
                    Some(&Alignment::Left) => element.add_attr("style", "text-align: left"),
                    Some(&Alignment::Center) => element.add_attr("style", "text-align: center"),
                    Some(&Alignment::Right) => element.add_attr("style", "text-align: right"),
                    _ => (),
                }
                self.push_element_styled(element, style);
            }
            #[cfg(feature = "syntect")]
            Tag::CodeBlock(info) => {
                if let CodeBlockKind::Fenced(info) = info {
                    self.push_element_styled(DomElement::new("pre"), &styling.codeblock);
                    // TODO: info
                    self.in_code_block = Some(info.clone());
                }
            }
            #[cfg(not(feature = "syntect"))]
            Tag::CodeBlock(info) => {
                self.push_element_styled(DomElement::new("pre"), &styling.codeblock);
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
                self.push_element_styled(element, &styling.blockquote);
            }
            Tag::List(Some(1)) => self.push_elname("ol", &styling.ol),
            Tag::List(Some(start)) => {
                self.push_element_styled(DomElement::new("ol").attr("start", start), &styling.ol);
            }
            Tag::List(None) => self.push_elname("ul", &styling.ul),
            Tag::Item => self.push_elname("li", &styling.li),
            Tag::DefinitionList => self.push_elname("dl", &styling.dl),
            Tag::DefinitionListTitle => self.push_elname("dt", &styling.dt),
            Tag::DefinitionListDefinition => self.push_elname("dd", &styling.dd),
            Tag::Subscript => self.push_elname("sub", &styling.sub),
            Tag::Superscript => self.push_elname("sup", &styling.sup),
            Tag::Emphasis => self.push_elname("em", &styling.em),
            Tag::Strong => self.push_elname("strong", &styling.strong),
            Tag::Strikethrough => self.push_elname("del", &styling.del),
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
                self.push_element_styled(element, &styling.a);
            }
            Tag::Image {
                link_type: _,
                dest_url,
                title,
                id: _,
            } => {
                let mut element = DomElement::new("img")
                    .attr("src", dest_url)
                    .attr("alt", self.raw_text());

                if !styling.img.groups.is_empty() {
                    element = element.css(&styling.img);
                }
                if !title.is_empty() {
                    element.add_attr("title", title);
                }
                self.add_child(element);
            }
            Tag::FootnoteDefinition(name) => {
                let len = self.numbers.len() + 1;
                let number = *self.numbers.entry(name.clone()).or_insert(len);
                let mut sup_element = DomElement::new("sup")
                    .attr("class", "footnote-definition-label")
                    .child(DomText::new(number.to_string()));
                if !styling.sup.groups.is_empty() {
                    sup_element = sup_element.css(&styling.sub);
                }
                self.push_node(
                    DomElement::new("div")
                        .attr("class", "footnote-definition")
                        .attr("id", name)
                        .child(sup_element),
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
                self.push_elname("tbody", &self.styling.clone().tbody);
                self.table_state = TableState::Body;
            }
            TagEnd::TableCell => {
                self.pop_node();
                self.table_cell_index += 1;
            }
            TagEnd::CodeBlock => {
                // </code> or </code></pre>
                self.pop_node();
                #[cfg(feature = "syntect")]
                {
                    self.in_code_block = None;
                }
                #[cfg(not(feature = "syntect"))]
                {
                    self.pop_node();
                }
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

    fn push_element_styled(&mut self, element: DomElement, css: &Css) {
        let mut element = element;
        if !css.groups.is_empty() {
            element = element.css(css)
        }
        self.soc.push_front(element.into());
    }

    fn push_elname(&mut self, name: impl Into<String>, css: &Css) {
        let name = name.into();
        let mut element = DomElement::new(name);
        if !css.groups.is_empty() {
            element = element.css(css)
        }
        self.push_node(element);
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
pub fn generate_tree<'a, I>(iter: I, styling: CMarkStyle) -> DomNode
where
    I: Iterator<Item = Event<'a>>,
{
    VertigoWriter::new(iter, styling).run()
}
