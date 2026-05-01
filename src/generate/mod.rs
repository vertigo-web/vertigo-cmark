// Based on https://github.com/pulldown-cmark/pulldown-cmark/blob/master/pulldown-cmark/src/html.rs

use pulldown_cmark::Event;
use vertigo::DomNode;

use crate::styling::CMarkStyle;

#[cfg(feature = "html")]
mod html;

mod end_tag;
mod raw_text;
mod run;
mod start_tag;

mod writer;
use writer::VertigoWriter;

/// Iterate over an iterator of pulldown's events, generate DomNode for each `Event`,
/// structure it into DOM tree and return the root node.
pub fn generate_tree<'a, I>(iter: I, styling: CMarkStyle) -> DomNode
where
    I: Iterator<Item = Event<'a>> + 'a,
{
    VertigoWriter::new(Box::new(iter), styling).run()
}
