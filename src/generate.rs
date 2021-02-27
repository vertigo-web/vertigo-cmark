use pulldown_cmark::{Event, Tag, CodeBlockKind, LinkType};
use vertigo::{VDomElement, VDomText, VDomNode, node_attr::attr};
use std::collections::VecDeque;

/// Converts an event iterator into Vertigo tree
pub fn generate_tree<'a>(events: impl Iterator<Item=Event<'a>>) -> Vec<VDomNode> {
    // Stack of children lists with current list on top
    let mut soc = VecDeque::new();

    // Drop most outer list on the bottom
    soc.push_front(vec![]);

    for event in events {
        let new_child = match event {
            Event::Start(_tag) => {
                soc.push_front(vec![]);
                None
            }

            Event::End(tag) => {
                let children = soc.pop_front();
                if let Some(children) = children {
                    Some(generate_tag(tag, children))
                } else {
                    vertigo::log::error!("Dangling end tag {:?} in cmark parser", tag);
                    None
                }
            },

            Event::Text(text) =>
                Some(VDomText::new(text.to_string()).into()),

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

fn generate_tag(tag: Tag, children: Vec<VDomNode>) -> VDomNode {
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

        Tag::Table(_alignments) =>
            // TODO:
            VDomText::new("".to_string()).into(),

        Tag::TableHead =>
            // TODO:
            VDomText::new("".to_string()).into(),

        Tag::TableRow =>
            // TODO:
            VDomText::new("".to_string()).into(),

        Tag::TableCell =>
            // TODO:
            VDomText::new("".to_string()).into(),

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
            if let Some(lang) = lang {
                vec![attr("class", format!("language-{}", lang))]
            } else {
                vec![]
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
