// Based on https://github.com/pulldown-cmark/pulldown-cmark/blob/master/pulldown-cmark/src/html.rs

use pulldown_cmark::Event;
use vertigo::{DomElement, DomNode, log};

use super::VertigoWriter;

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
                    let v_el = DomElement::new(tag.name);
                    for attr in tag.attributes {
                        v_el.add_attr(attr.name, attr.value);
                    }
                    self.push_node(v_el);

                    if tag.self_closing {
                        self.pop_node();
                    }
                }
                Token::EndTag(_tag) => {
                    consume_chars(self, &mut chars);
                    self.pop_node();
                }
                Token::Char(x) => {
                    chars.push(x);
                }
                Token::EndOfFile | Token::Comment(_) | Token::Doctype(_) => {}
            }
        }
    }
}
