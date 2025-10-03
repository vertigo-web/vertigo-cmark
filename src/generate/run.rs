// Based on https://github.com/pulldown-cmark/pulldown-cmark/blob/master/pulldown-cmark/src/html.rs

use pulldown_cmark::{Event, Event::*};
use vertigo::{DomElement, DomNode, DomText, log};

#[cfg(feature = "syntect")]
use crate::highlighting::highlight;

use super::VertigoWriter;

impl<'a, I> VertigoWriter<'a, I>
where
    I: Iterator<Item = Event<'a>>,
{
    pub(super) fn run(mut self) -> DomNode {
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
                Html(html) | InlineHtml(html) => {
                    #[cfg(feature = "html")]
                    self.html(&html);
                    #[cfg(not(feature = "html"))]
                    let _ = html;
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
}
