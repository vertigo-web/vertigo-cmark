use pulldown_cmark::{Event, Tag, CodeBlockKind, LinkType, HeadingLevel};
use vertigo::{
    DomElement, DomNode, DomText,
};
use std::collections::VecDeque;

use super::table_state::TableState;

/// Converts an event iterator into Vertigo tree
pub fn generate_tree<'a>(events: impl Iterator<Item=Event<'a>>) -> Vec<DomNode> {
    // Stack of children lists with current list on top
    let mut soc = VecDeque::new();
    let mut table_state = TableState::default();

    // Drop most outer list on the bottom
    soc.push_front(vec![]);

    for event in events {
        let new_child = match event {
            Event::Start(tag) => {
                soc.push_front(vec![]);
                match tag {
                    Tag::Table(als) => {
                        table_state.soa.push_front(als);
                    },
                    Tag::TableHead => {
                        if table_state.head {
                            vertigo::log::error!("Already in table head, ")
                        }
                        table_state.head = true
                    }
                    _ => ()
                }
                None
            }

            Event::End(tag) => {
                if let Some(children) = soc.pop_front() {
                    match tag {
                        Tag::TableHead => {
                            table_state.head = false;
                            // Put head on stack
                            soc.push_front(vec![generate_tag(tag, &mut table_state, children)]);
                            // Put new layer for regular rows
                            soc.push_front(vec![]);
                            None
                        }
                        Tag::Table(_) => {
                            // Regular rows from stack was popped first into children
                            // Additionally pop head element into new table_children
                            let mut table_children = soc.pop_front().unwrap_or_default();
                            // Create tbody element and add to table children
                            let el = DomElement::new("tbody").children(children);
                            table_children.push(el.into());
                            // Pop unused layer
                            soc.pop_front();
                            // Create table element with thead and tbody as children
                            Some(generate_tag(tag, &mut table_state, table_children))
                        }
                        _ => Some(generate_tag(tag, &mut table_state, children))
                    }
                } else {
                    vertigo::log::error!("Dangling end tag {:?} in cmark parser", tag);
                    None
                }
            },

            Event::Text(text) => {
                let trimmed_text = text.trim().to_string();
                if trimmed_text.is_empty() {
                    None
                } else {
                    Some(DomText::new(
                        // keep spaces to not glue text with sibling blocks
                        text.trim_matches(|c: char| c != ' ' && c.is_whitespace())
                        .to_string()
                    ).into())
                }
            },

            Event::Code(text) => {
                let children = vec![DomText::new(text.to_string()).into()];
                Some(generate_codeblock(CodeBlockKind::Indented, children))
            }

            Event::Html(html) =>
                // TODO:
                Some(DomText::new(html.to_string()).into()),

            Event::FootnoteReference(text) =>
                // TODO:
                Some(DomText::new(text.to_string()).into()),

            Event::SoftBreak =>
                // TODO: ?
                None,

            Event::HardBreak =>
                Some(generate_el("br", vec![])),

            Event::Rule =>
                Some(generate_el("hr", vec![])),

            Event::TaskListMarker(checked) => {
                let dom_element = DomElement::new("input");
                dom_element.add_attr("type", "checkbox");
                if checked {
                    dom_element.add_attr("checked", "1");
                }
                Some(dom_element.into())
            }
        };

        // Push new child into top list on the stack
        if let Some(child) = new_child {
            if let Some(children) = soc.front_mut() {
                children.push(child)
            }
        }
    }

    soc.pop_front().unwrap_or_else(|| {
        vertigo::log::error!("Empty stack at generate tree");
        vec![]
    })
}

fn generate_tag(tag: Tag, table_state: &mut TableState, children: Vec<DomNode>) -> DomNode {
    match tag {
        Tag::Paragraph =>
            DomElement::new("p")
                .children(children)
                .into(),

        Tag::Heading(level, _fragment, _classes) => {
            let el = match level {
                HeadingLevel::H1 => "h1",
                HeadingLevel::H2 => "h2",
                HeadingLevel::H3 => "h3",
                HeadingLevel::H4 => "h4",
                HeadingLevel::H5 => "h5",
                HeadingLevel::H6 => "h6",
            };
            generate_el(el, children)
        }

        Tag::Table(_) => {
            DomElement::new("table")
                .attr("border", "1")
                .children(children)
                .into()
        }

        Tag::TableHead =>
            DomElement::new("thead")
                .child(
                    DomElement::new("tr")
                        .children(children)
                )
                .into(),

        Tag::TableRow =>
            DomElement::new("tr")
                .children(children)
                .into(),

        Tag::TableCell => {
            let attrs = if let Some(alignment) = table_state.alignment() {
                vec![("align", alignment)]
            } else {
                vec![]
            };
            DomElement::new(if table_state.head { "th" } else { "td" })
                .attrs(attrs)
                .children(children)
                .into()
        }

        Tag::BlockQuote =>
            generate_el("blockquote", children),

        Tag::CodeBlock(info) =>
            generate_codeblock(info, children),

        Tag::List(Some(1)) =>
            generate_el("ol", children),

        Tag::List(Some(start)) => {
            DomElement::new("ol")
                .attr("start", start.to_string())
                .children(children)
                .into()
        }

        Tag::List(None) =>
            generate_el("ul", children),

        Tag::Item =>
            generate_el("li", children),

        Tag::Emphasis =>
            generate_el("em", children),

        Tag::Strong =>
            generate_el("strong", children),

        Tag::Strikethrough =>
            generate_el("del", children),

        Tag::Link(LinkType::Email, _dest, _title) =>
            // TODO:
            DomText::new("".to_string()).into(),

        Tag::Link(_link_type, _dest, _title) =>
            // TODO:
            DomText::new("".to_string()).into(),

        Tag::Image(_link_type, _dest, _title) =>
            // TODO:
            DomText::new("".to_string()).into(),

        Tag::FootnoteDefinition(_name) =>
            // TODO:
            DomText::new("".to_string()).into(),
    }
}

fn generate_el(el: &'static str, children: Vec<DomNode>) -> DomNode {
    DomElement::new(el)
        .children(children)
        .into()
}

fn generate_codeblock(info: CodeBlockKind, children: Vec<DomNode>) -> DomNode {
    let code_attrs = match info {
        CodeBlockKind::Indented => vec![],
        CodeBlockKind::Fenced(info) => {
            let lang = info.split(' ').next();
            match lang {
                Some("") |
                None => vec![],
                Some(lang) => vec![("class", ["language-{}", lang].concat())],
            }
        },
    };

    let code = DomElement::new("code")
        .attrs(code_attrs)
        .children(children)
        .into();

    generate_el("pre", vec![code])
}
