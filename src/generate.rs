use pulldown_cmark::{Event, Tag, CodeBlockKind, LinkType};
use vertigo::{VDomElement, VDomText, VDomNode, node_attr::attr};
use std::collections::VecDeque;

use super::table_state::TableState;

/// Converts an event iterator into Vertigo tree
pub fn generate_tree<'a>(events: impl Iterator<Item=Event<'a>>) -> Vec<VDomNode> {
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
                            table_children.push(VDomElement::new("tbody", vec![], children).into());
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
                    Some(VDomText::new(text.trim().to_string()).into())
                }
            },

            Event::Code(text) => {
                let children = vec![VDomText::new(text.to_string()).into()];
                Some(generate_codeblock(CodeBlockKind::Indented, children))
            }

            Event::Html(html) =>
                // TODO:
                Some(VDomText::new(html.to_string()).into()),

            Event::FootnoteReference(text) =>
                // TODO:
                Some(VDomText::new(text.to_string()).into()),

            Event::SoftBreak =>
                // TODO: ?
                None,

            Event::HardBreak =>
                Some(generate_el("br", vec![])),

            Event::Rule =>
                Some(generate_el("hr", vec![])),

            Event::TaskListMarker(checked) => {
                let mut attrs = vec![
                    attr("type", "checkbox")
                ];
                if checked {
                    attrs.push(attr("checked", "1"))
                }
                Some(VDomElement::new("input", attrs, vec![]).into())
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

fn generate_tag(tag: Tag, table_state: &mut TableState, children: Vec<VDomNode>) -> VDomNode {
    match tag {
        Tag::Paragraph =>
            VDomElement::new("p", vec![], children).into(),

        Tag::Heading(level) => {
            let el = match level {
                1 => "h1",
                2 => "h2",
                3 => "h3",
                4 => "h4",
                5 => "h5",
                6 => "h6",
                _ => "h6",
            };
            generate_el(&el, children)
        }

        Tag::Table(_) => {
            VDomElement::new(
                "table",
                vec![attr("border", "1")], // temporary
                children
            ).into()
        }

        Tag::TableHead =>
            VDomElement::new(
                "thead",
                vec![],
                vec![VDomElement::new("tr", vec![], children).into()],
            ).into(),

        Tag::TableRow =>
            VDomElement::new("tr", vec![], children).into(),

        Tag::TableCell => {
            let attrs = if let Some(alignment) = table_state.alignment() {
                vec![attr("align", alignment)]
            } else {
                vec![]
            };
            let el = if table_state.head { "th" } else { "td" };
            VDomElement::new(el, attrs, children).into()
        }

        Tag::BlockQuote =>
            generate_el("blockquote", children),

        Tag::CodeBlock(info) =>
            generate_codeblock(info, children),

        Tag::List(Some(1)) =>
            generate_el("ol", children),

        Tag::List(Some(start)) => {
            VDomElement::new(
                "ol",
                vec![attr("start", start.to_string())],
                children
            ).into()
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
            VDomText::new("".to_string()).into(),

        Tag::Link(_link_type, _dest, _title) =>
            // TODO:
            VDomText::new("".to_string()).into(),

        Tag::Image(_link_type, _dest, _title) =>
            // TODO:
            VDomText::new("".to_string()).into(),

        Tag::FootnoteDefinition(_name) =>
            // TODO:
            VDomText::new("".to_string()).into(),
    }
}

fn generate_el(el: &'static str, children: Vec<VDomNode>) -> VDomNode {
    VDomElement::new(el, vec![], children).into()
}

fn generate_codeblock(info: CodeBlockKind, children: Vec<VDomNode>) -> VDomNode {
    let code_attrs = match info {
        CodeBlockKind::Indented => vec![],
        CodeBlockKind::Fenced(info) => {
            let lang = info.split(' ').next();
            match lang {
                Some("") |
                None => vec![],
                Some(lang) => vec![attr("class", format!("language-{}", lang))],
            }
        },
    };

    let code = VDomElement::new(
        "code",
        code_attrs,
        children,
    ).into();

    generate_el("pre", vec![code])
}
