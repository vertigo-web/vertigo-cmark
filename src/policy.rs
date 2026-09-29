//! What of raw HTML and link destinations gets from markdown into the rendered tree.
//!
//! Markdown allows raw HTML and links to any destination, and vertigo-cmark builds DOM nodes
//! from them directly — there is no HTML string to clean afterwards with a sanitizer like
//! `ammonia`. So every element and attribute of raw HTML and every destination of a link
//! or an image goes through an [`HtmlPolicy`] first.
//!
//! [`SafeHtml`], used by [`to_vertigo`](crate::to_vertigo) and its variants, lets through
//! what markdown authors usually need and nothing that runs scripts: the same whitelist
//! as `ammonia`'s defaults. [`TrustedHtml`] renders everything as written — only for content
//! you control. For anything in between implement [`HtmlPolicy`] yourself, delegating
//! to [`SafeHtml`] what you don't change:
//!
//! ```
//! use std::borrow::Cow;
//! use vertigo_cmark::{ElementAction, HtmlPolicy, SafeHtml};
//!
//! /// Safe HTML plus YouTube players.
//! struct WithVideos;
//!
//! impl HtmlPolicy for WithVideos {
//!     fn element(&self, name: &str) -> ElementAction {
//!         match name {
//!             "iframe" => ElementAction::Keep,
//!             name => SafeHtml.element(name),
//!         }
//!     }
//!
//!     fn attribute<'v>(&self, element: &str, name: &str, value: &'v str) -> Option<Cow<'v, str>> {
//!         match (element, name) {
//!             ("iframe", "src") => value
//!                 .starts_with("https://www.youtube.com/embed/")
//!                 .then_some(Cow::Borrowed(value)),
//!             ("iframe", "allowfullscreen") => Some(Cow::Borrowed(value)),
//!             _ => SafeHtml.attribute(element, name, value),
//!         }
//!     }
//!
//!     fn destination<'u>(&self, url: &'u str, image: bool) -> Option<Cow<'u, str>> {
//!         SafeHtml.destination(url, image)
//!     }
//! }
//! ```
//!
//! Heading attributes (`{#id .class}`) and footnote labels are not checked: they end up
//! only in `id` and `class` and can't run anything.

use std::borrow::Cow;

/// What happens to an element of raw HTML.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ElementAction {
    /// The element stays, with the attributes [`HtmlPolicy::attribute`] lets through.
    Keep,
    /// Only the tag goes away, its content stays in place.
    Unwrap,
    /// The element goes away together with its content — like `<script>`.
    Remove,
}

/// Decides what of raw HTML and link destinations gets into the rendered tree.
pub trait HtmlPolicy {
    /// What happens to an element of raw HTML. The name is lowercase.
    fn element(&self, name: &str) -> ElementAction;

    /// The value an attribute of a kept element gets — the same one or changed —
    /// or `None`, which drops the attribute. Names are lowercase and character references
    /// in values are already resolved (`&#106;` is `j`).
    fn attribute<'v>(&self, element: &str, name: &str, value: &'v str) -> Option<Cow<'v, str>>;

    /// The destination of a markdown link or image (`image`), autolinks included, or `None`,
    /// which leaves the link text without a link and the image replaced by its description.
    fn destination<'u>(&self, url: &'u str, image: bool) -> Option<Cow<'u, str>>;
}

/// Elements and attributes from `ammonia`'s default whitelist, links and images only with safe
/// destinations ([`is_safe_url`]). Scripts, styles, event handlers, frames, forms
/// and `javascript:` addresses don't get through. Elements outside the whitelist lose just
/// their tags; `<script>` and `<style>` go away together with their content.
#[derive(Clone, Copy, Default, Debug)]
pub struct SafeHtml;

/// Everything as written, scripts included — only for content you control.
#[derive(Clone, Copy, Default, Debug)]
pub struct TrustedHtml;

/// Elements [`SafeHtml`] keeps.
const ELEMENTS: &[&str] = &[
    "a",
    "abbr",
    "acronym",
    "area",
    "article",
    "aside",
    "b",
    "bdi",
    "bdo",
    "blockquote",
    "br",
    "caption",
    "center",
    "cite",
    "code",
    "col",
    "colgroup",
    "data",
    "dd",
    "del",
    "details",
    "dfn",
    "div",
    "dl",
    "dt",
    "em",
    "figcaption",
    "figure",
    "footer",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "header",
    "hgroup",
    "hr",
    "i",
    "img",
    "ins",
    "kbd",
    "li",
    "map",
    "mark",
    "nav",
    "ol",
    "p",
    "pre",
    "q",
    "rp",
    "rt",
    "rtc",
    "ruby",
    "s",
    "samp",
    "small",
    "span",
    "strike",
    "strong",
    "sub",
    "summary",
    "sup",
    "table",
    "tbody",
    "td",
    "th",
    "thead",
    "time",
    "tr",
    "tt",
    "u",
    "ul",
    "var",
    "wbr",
];

/// Elements [`SafeHtml`] removes together with their content.
const REMOVED_WITH_CONTENT: &[&str] = &["script", "style"];

/// Attributes [`SafeHtml`] keeps on every element.
const GENERIC_ATTRIBUTES: &[&str] = &["lang", "title"];

/// Attributes [`SafeHtml`] keeps on particular elements.
const ELEMENT_ATTRIBUTES: &[(&str, &[&str])] = &[
    ("a", &["href", "hreflang"]),
    ("bdo", &["dir"]),
    ("blockquote", &["cite"]),
    ("col", &["align", "char", "charoff", "span"]),
    ("colgroup", &["align", "char", "charoff", "span"]),
    ("del", &["cite", "datetime"]),
    ("hr", &["align", "size", "width"]),
    ("img", &["align", "alt", "height", "src", "width"]),
    ("ins", &["cite", "datetime"]),
    ("ol", &["start"]),
    ("q", &["cite"]),
    ("table", &["align", "char", "charoff", "summary"]),
    ("tbody", &["align", "char", "charoff"]),
    (
        "td",
        &["align", "char", "charoff", "colspan", "headers", "rowspan"],
    ),
    ("tfoot", &["align", "char", "charoff"]),
    (
        "th",
        &[
            "align", "char", "charoff", "colspan", "headers", "rowspan", "scope",
        ],
    ),
    ("thead", &["align", "char", "charoff"]),
    ("tr", &["align", "char", "charoff"]),
];

/// Attributes with an address the browser opens or loads.
const URL_ATTRIBUTES: &[&str] = &[
    "action",
    "background",
    "cite",
    "data",
    "formaction",
    "href",
    "longdesc",
    "ping",
    "poster",
    "src",
    "xlink:href",
];

/// Schemes [`is_safe_url`] accepts — the same as `ammonia`'s defaults.
const SAFE_SCHEMES: &[&str] = &[
    "bitcoin",
    "ftp",
    "ftps",
    "geo",
    "http",
    "https",
    "im",
    "irc",
    "ircs",
    "magnet",
    "mailto",
    "mms",
    "mx",
    "news",
    "nntp",
    "openpgp4fpr",
    "sip",
    "sms",
    "smsto",
    "ssh",
    "tel",
    "url",
    "webcal",
    "wtai",
    "xmpp",
];

impl HtmlPolicy for SafeHtml {
    fn element(&self, name: &str) -> ElementAction {
        if REMOVED_WITH_CONTENT.contains(&name) {
            ElementAction::Remove
        } else if ELEMENTS.contains(&name) {
            ElementAction::Keep
        } else {
            ElementAction::Unwrap
        }
    }

    fn attribute<'v>(&self, element: &str, name: &str, value: &'v str) -> Option<Cow<'v, str>> {
        let listed = GENERIC_ATTRIBUTES.contains(&name)
            || ELEMENT_ATTRIBUTES
                .iter()
                .any(|(tag, attributes)| *tag == element && attributes.contains(&name));
        let safe = !URL_ATTRIBUTES.contains(&name) || is_safe_url(value);
        (listed && safe).then_some(Cow::Borrowed(value))
    }

    fn destination<'u>(&self, url: &'u str, _image: bool) -> Option<Cow<'u, str>> {
        is_safe_url(url).then_some(Cow::Borrowed(url))
    }
}

impl HtmlPolicy for TrustedHtml {
    fn element(&self, _name: &str) -> ElementAction {
        ElementAction::Keep
    }

    fn attribute<'v>(&self, _element: &str, _name: &str, value: &'v str) -> Option<Cow<'v, str>> {
        Some(Cow::Borrowed(value))
    }

    fn destination<'u>(&self, url: &'u str, _image: bool) -> Option<Cow<'u, str>> {
        Some(Cow::Borrowed(url))
    }
}

/// Whether the address is relative (`/page`, `#note`, `image.png`) or has one of the schemes
/// `ammonia` accepts by default (`https:`, `mailto:`…) — so never `javascript:`, `vbscript:`,
/// `data:` or `file:`. The scheme is read the way browsers read it: without spaces and control
/// characters around the address, without tabs and newlines inside (`java\tscript:`)
/// and regardless of case.
pub fn is_safe_url(url: &str) -> bool {
    scheme(url).is_none_or(|scheme| SAFE_SCHEMES.contains(&scheme.as_str()))
}

/// Scheme of the address in lowercase, `None` for a relative address.
fn scheme(url: &str) -> Option<String> {
    let cleaned: String = url
        .trim_matches(|character: char| character <= ' ')
        .chars()
        .filter(|character| !matches!(character, '\t' | '\n' | '\r'))
        .collect();
    let (candidate, _) = cleaned.split_once(':')?;
    let mut characters = candidate.chars();
    let first = characters.next()?;
    let is_scheme = first.is_ascii_alphabetic()
        && characters.all(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '+' | '-' | '.')
        });
    is_scheme.then(|| candidate.to_ascii_lowercase())
}
