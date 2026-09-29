pub use pulldown_cmark::{self, Options};
use pulldown_cmark::{Event, Parser};
use vertigo::DomNode;

mod generate;
mod styling;
pub use styling::CMarkStyle;

#[cfg(feature = "syntect")]
mod highlighting;

#[cfg(test)]
mod tests;

/// Converts a CommonMark string to Vertigo tree with default options and styling.
///
/// NOTE: Tables are enabled by default.
pub fn to_vertigo(text: &str) -> DomNode {
    to_vertigo_opts(text, Options::ENABLE_TABLES)
}

/// Converts a CommonMark string to Vertigo tree with provided [Options] and default styling.
pub fn to_vertigo_opts(text: &str, opts: Options) -> DomNode {
    let parser = Parser::new_ext(text, opts);
    generate::generate_tree(parser, CMarkStyle::default())
}

/// Converts a CommonMark string to Vertigo tree with default options and provided [styling](CMarkStyle).
pub fn to_vertigo_styled(text: &str, style: CMarkStyle) -> DomNode {
    let parser = Parser::new_ext(text, Options::ENABLE_TABLES);
    generate::generate_tree(parser, style)
}

/// Converts a CommonMark string to Vertigo tree with provided [Options] and provided [styling](CMarkStyle).
///
/// NOTE: If you want highlighted code block, just enable `syntect` feature.
pub fn to_vertigo_opts_styled(text: &str, opts: Options, style: CMarkStyle) -> DomNode {
    let parser = Parser::new_ext(text, opts);
    generate::generate_tree(parser, style)
}

/// Converts a stream of pulldown-cmark [events](Event) to Vertigo tree with provided [styling](CMarkStyle).
///
/// Lets the caller adjust the events before rendering: drop links with an unwanted scheme,
/// sanitize raw HTML or replace a paragraph with an embed.
///
/// ```
/// use vertigo_cmark::{CMarkStyle, events_to_vertigo, pulldown_cmark::{Event, Parser, Tag}};
///
/// // links become plain text
/// let events = Parser::new("Visit [example](https://example.com)")
///     .filter(|event| !matches!(event, Event::Start(Tag::Link { .. }) | Event::End(pulldown_cmark::TagEnd::Link)));
/// let _node = events_to_vertigo(events, CMarkStyle::default());
/// ```
pub fn events_to_vertigo<'a, I>(events: I, style: CMarkStyle) -> DomNode
where
    I: IntoIterator<Item = Event<'a>>,
    I::IntoIter: 'a,
{
    generate::generate_tree(events.into_iter(), style)
}
