// Based on https://github.com/pulldown-cmark/pulldown-cmark/blob/master/pulldown-cmark/src/html.rs

use pulldown_cmark::{
    Alignment, BlockQuoteKind, CodeBlockKind, Event, HeadingLevel, LinkType, Tag,
};
use vertigo::{DomElement, DomText};

use super::writer::{TableState, VertigoWriter};

impl<'a, I> VertigoWriter<'a, I>
where
    I: Iterator<Item = Event<'a>>,
{
    /// Pushes dom element on stack
    pub(super) fn start_tag(&mut self, tag: Tag<'a>) {
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
                    element.add_attr("id", id.as_ref());
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
                    element.add_attr("title", title.as_ref());
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
                    .attr("src", dest_url.as_ref())
                    .attr("alt", self.raw_text());

                if !styling.img.groups.is_empty() {
                    element = element.css(&styling.img);
                }
                if !title.is_empty() {
                    element.add_attr("title", title.as_ref());
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
                        .attr("id", name.as_ref())
                        .child(sup_element),
                );
            }
            Tag::MetadataBlock(_) => {
                self.in_non_writing_block = true;
            }
        }
    }
}
