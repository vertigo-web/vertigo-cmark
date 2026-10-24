// Based on https://github.com/pulldown-cmark/pulldown-cmark/blob/master/pulldown-cmark/src/html.rs

use pulldown_cmark::Event;
use vertigo::{Css, DomElement, DomNode, log};

use super::VertigoWriter;
use crate::{CMarkStyle, policy::ElementAction};

/// Elements without content and end tag — `<br>` or `<img src="…">` don't open anything,
/// even without `/>`.
const VOID_ELEMENTS: &[&str] = &[
    "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "source", "track",
    "wbr",
];

/// Elements of a table holding rows or cells, not text — white space between their tags
/// is no content of theirs, as browsers parse HTML.
const TABLE_STRUCTURE: &[&str] = &["table", "thead", "tbody", "tfoot", "tr"];

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
                if inner_text.trim().is_empty() && myself.in_table_structure() {
                    return;
                }
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

        let policy = self.policy;
        for token in NaiveParser::new(&input.to_string()).flatten() {
            if let Some((removed, nested)) = &mut self.removing {
                match &token {
                    Token::StartTag(tag)
                        if tag.name == *removed && !VOID_ELEMENTS.contains(&tag.name.as_str()) =>
                    {
                        *nested += 1;
                    }
                    Token::EndTag(tag) if tag.name == *removed => match nested {
                        0 => self.removing = None,
                        nested => *nested -= 1,
                    },
                    _ => {}
                }
                continue;
            }
            match token {
                Token::StartTag(tag) => {
                    consume_chars(self, &mut chars);
                    let void = VOID_ELEMENTS.contains(&tag.name.as_str());
                    match policy.element(&tag.name) {
                        ElementAction::Keep => {}
                        // end tag of an unwrapped element has nothing to close and is skipped
                        ElementAction::Unwrap => continue,
                        // browsers don't close `<script />` — its content goes on until
                        // `</script>` anyway
                        ElementAction::Remove => {
                            if !void {
                                self.removing = Some((tag.name, 0));
                            }
                            continue;
                        }
                    }
                    let empty = void || tag.self_closing;
                    let name = (!empty).then(|| tag.name.clone());
                    let v_el = DomElement::new(tag.name.clone());
                    for attr in tag.attributes {
                        if let Some(value) = policy.attribute(&tag.name, &attr.name, &attr.value) {
                            v_el.add_attr(attr.name, value.into_owned());
                        }
                    }
                    // a table of raw HTML looks like a markdown one
                    let styling = self.styling.clone();
                    match table_style(&styling, &tag.name) {
                        Some(css) => self.push_element_styled(v_el, css),
                        None => self.push_node(v_el),
                    }

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

    /// Whether the innermost element is a raw HTML table's, holding rows or cells.
    fn in_table_structure(&self) -> bool {
        self.html_open.last().is_some_and(|(name, depth)| {
            *depth == self.soc.len() && TABLE_STRUCTURE.contains(&name.as_str())
        })
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

/// The style of a table's element from [`CMarkStyle`], for the same element of raw HTML.
fn table_style<'s>(styling: &'s CMarkStyle, name: &str) -> Option<&'s Css> {
    match name {
        "table" => Some(&styling.table),
        "thead" => Some(&styling.thead),
        "tbody" => Some(&styling.tbody),
        "tr" => Some(&styling.tr),
        "th" => Some(&styling.th),
        "td" => Some(&styling.td),
        _ => None,
    }
}
