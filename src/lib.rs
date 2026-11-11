pub use pulldown_cmark::{self, Options};
use pulldown_cmark::{Event, Parser};
use vertigo::DomNode;

mod generate;
mod metadata;
mod policy;
mod styling;
pub mod table_spans;
pub use metadata::{METADATA_OPTIONS, Metadata, metadata};
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
/// from anyone — see [`to_vertigo_with_policy`] for other policies.
pub fn to_vertigo(text: &str) -> DomNode {
    to_vertigo_with_policy(text, &SafeHtml)
}

/// Converts a CommonMark string to Vertigo tree with default options and styling, as
/// [`to_vertigo`] does, but with raw HTML and link destinations filtered by the provided
/// [policy](HtmlPolicy): [`TrustedHtml`] only for content you control, or your own.
///
/// For other options or styling, or events adjusted before rendering, see [`events_to_vertigo`].
///
/// ```
/// use std::borrow::Cow;
/// use vertigo_cmark::{ElementAction, HtmlPolicy, SafeHtml, to_vertigo_with_policy};
///
/// /// Safe HTML, links keeping `target="_blank"`.
/// struct NewTabLinks;
///
/// impl HtmlPolicy for NewTabLinks {
///     fn element(&self, name: &str) -> ElementAction {
///         SafeHtml.element(name)
///     }
///
///     fn attribute<'v>(&self, element: &str, name: &str, value: &'v str) -> Option<Cow<'v, str>> {
///         match (element, name, value) {
///             ("a", "target", "_blank") => Some(Cow::Borrowed(value)),
///             _ => SafeHtml.attribute(element, name, value),
///         }
///     }
///
///     fn destination<'u>(&self, url: &'u str, image: bool) -> Option<Cow<'u, str>> {
///         SafeHtml.destination(url, image)
///     }
/// }
///
/// let _node = to_vertigo_with_policy(
///     r#"See <a href="https://example.com/" target="_blank">this</a>"#,
///     &NewTabLinks,
/// );
/// ```
pub fn to_vertigo_with_policy(text: &str, policy: &dyn HtmlPolicy) -> DomNode {
    let parser = Parser::new_ext(text, Options::ENABLE_TABLES);
    generate::generate_tree(parser, CMarkStyle::default(), policy)
}

/// Converts a document that may start with a metadata block (front matter) to Vertigo tree,
/// as [`to_vertigo`] does, and returns the block apart, see [`metadata`]. The block itself is
/// not rendered.
///
/// ```
/// use vertigo_cmark::to_vertigo_with_metadata;
///
/// let (metadata, _node) = to_vertigo_with_metadata("---\ntitle: The cave\n---\n\nText");
/// assert_eq!(metadata.unwrap().text, "title: The cave\n");
/// ```
pub fn to_vertigo_with_metadata(text: &str) -> (Option<Metadata>, DomNode) {
    let parser = Parser::new_ext(text, Options::ENABLE_TABLES | METADATA_OPTIONS);
    let node = generate::generate_tree(parser, CMarkStyle::default(), &SafeHtml);
    (metadata(text), node)
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
