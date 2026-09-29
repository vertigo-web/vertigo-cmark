pub use pulldown_cmark::{self, Options};
use pulldown_cmark::{Event, Parser};
use vertigo::DomNode;

mod generate;
mod policy;
mod styling;
pub use policy::{ElementAction, HtmlPolicy, SafeHtml, TrustedHtml, is_safe_url};
pub use styling::CMarkStyle;

#[cfg(feature = "syntect")]
mod highlighting;

#[cfg(test)]
mod tests;

/// Converts a CommonMark string to Vertigo tree with default options and styling.
///
/// NOTE: Tables are enabled by default.
///
/// Raw HTML and link destinations are filtered by [`SafeHtml`], so the text may come
/// from anyone — see [`HtmlPolicy`] for other policies.
pub fn to_vertigo(text: &str) -> DomNode {
    to_vertigo_opts(text, Options::ENABLE_TABLES)
}

/// Converts a CommonMark string to Vertigo tree with provided [Options] and default styling.
///
/// Raw HTML and link destinations are filtered by [`SafeHtml`].
pub fn to_vertigo_opts(text: &str, opts: Options) -> DomNode {
    let parser = Parser::new_ext(text, opts);
    generate::generate_tree(parser, CMarkStyle::default(), &SafeHtml)
}

/// Converts a CommonMark string to Vertigo tree with default options and provided [styling](CMarkStyle).
///
/// Raw HTML and link destinations are filtered by [`SafeHtml`].
pub fn to_vertigo_styled(text: &str, style: CMarkStyle) -> DomNode {
    let parser = Parser::new_ext(text, Options::ENABLE_TABLES);
    generate::generate_tree(parser, style, &SafeHtml)
}

/// Converts a CommonMark string to Vertigo tree with provided [Options] and provided [styling](CMarkStyle).
///
/// NOTE: If you want highlighted code block, just enable `syntect` feature.
///
/// Raw HTML and link destinations are filtered by [`SafeHtml`].
pub fn to_vertigo_opts_styled(text: &str, opts: Options, style: CMarkStyle) -> DomNode {
    let parser = Parser::new_ext(text, opts);
    generate::generate_tree(parser, style, &SafeHtml)
}

/// Converts a stream of pulldown-cmark [events](Event) to Vertigo tree with provided
/// [styling](CMarkStyle) and [policy](HtmlPolicy) for raw HTML and link destinations.
///
/// Lets the caller adjust the events before rendering — e.g. replace a paragraph with an
/// embed — and choose the policy: [`SafeHtml`] for text from anyone, [`TrustedHtml`] only for
/// content you control, or your own.
///
/// ```
/// use vertigo_cmark::{CMarkStyle, SafeHtml, events_to_vertigo, pulldown_cmark::{Event, Parser}};
///
/// // soft line breaks as hard ones
/// let events = Parser::new("First line\nsecond line").map(|event| match event {
///     Event::SoftBreak => Event::HardBreak,
///     event => event,
/// });
/// let _node = events_to_vertigo(events, CMarkStyle::default(), &SafeHtml);
/// ```
pub fn events_to_vertigo<'a, I>(events: I, style: CMarkStyle, policy: &'a dyn HtmlPolicy) -> DomNode
where
    I: IntoIterator<Item = Event<'a>>,
    I::IntoIter: 'a,
{
    generate::generate_tree(events.into_iter(), style, policy)
}
