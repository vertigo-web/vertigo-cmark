// Based on https://github.com/pulldown-cmark/pulldown-cmark/blob/master/pulldown-cmark/src/html.rs

use pulldown_cmark::Event;
use vertigo::{DomElement, DomNode, log};

use super::VertigoWriter;

/// Elements without content and end tag — `<br>` or `<img src="…">` don't open anything,
/// even without `/>`.
const VOID_ELEMENTS: &[&str] = &[
    "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "source", "track",
    "wbr",
];

impl<'a, I> VertigoWriter<'a, I>
where
    I: Iterator<Item = Event<'a>>,
{
    pub(super) fn html(&mut self, input: &str) {
        use html5tokenizer::{NaiveParser, Token};

        let mut chars: Vec<char> = vec![];

        let consume_chars = |myself: &mut Self, chars: &mut Vec<char>| {
            if !chars.is_empty() {
                let inner_text: String = chars.drain(..).collect();
                if let Some(node) = myself.soc.front_mut() {
                    match node {
                        DomNode::Node { node } => {
                            node.add_child_text(inner_text);
                        }
                        _ => {
                            log::error!("Ignored inner text `{}` (invalid parent)", inner_text);
                        }
                    }
                } else {
                    log::error!("Ignored inner text `{}` (no parent)", inner_text);
                }
            }
        };

        for token in NaiveParser::new(&input.to_string()).flatten() {
            match token {
                Token::StartTag(tag) => {
                    consume_chars(self, &mut chars);
                    let void = tag.self_closing || VOID_ELEMENTS.contains(&tag.name.as_str());
                    let name = (!void).then(|| tag.name.clone());
                    let v_el = DomElement::new(tag.name);
                    for attr in tag.attributes {
                        v_el.add_attr(attr.name, attr.value);
                    }
                    self.push_node(v_el);

                    match name {
                        Some(name) => self.html_open.push((name, self.soc.len())),
                        None => {
                            self.pop_node();
                        }
                    }
                }
                Token::EndTag(tag) => {
                    consume_chars(self, &mut chars);
                    self.close_html(&tag.name);
                }
                Token::Char(x) => {
                    chars.push(x);
                }
                Token::EndOfFile | Token::Comment(_) | Token::Doctype(_) => {}
            }
        }
        // text after the last tag — a line of an HTML block
        consume_chars(self, &mut chars);
    }

    /// Closes element opened by raw HTML, together with raw HTML still open inside it.
    /// End tag without a start tag — or with a markdown element opened after it — is ignored,
    /// so it can't close the markdown element it sits in.
    fn close_html(&mut self, name: &str) {
        let Some(position) = self.html_open.iter().rposition(|(open, _)| open == name) else {
            return;
        };
        let (_, depth) = self.html_open[position];
        let inside = self.html_open.len() - position - 1;
        if self.soc.len() != depth + inside {
            return;
        }
        for _ in position..self.html_open.len() {
            self.pop_node();
        }
        self.html_open.truncate(position);
    }
}
