//! The metadata block (front matter) a document starts with: `---` fences around YAML, or
//! `+++` fences around TOML, as static site generators write them.

use pulldown_cmark::{Event, MetadataBlockKind, Options, Parser, Tag, TagEnd};

/// What the parser needs to recognize both kinds of metadata blocks. Add it to the options of
/// a document that may start with one, or its fences render as rules and headings.
pub const METADATA_OPTIONS: Options = Options::ENABLE_YAML_STYLE_METADATA_BLOCKS
    .union(Options::ENABLE_PLUSES_DELIMITED_METADATA_BLOCKS);

/// The metadata block of a document, as written between its fences. Parse `text` with a
/// YAML or TOML parser as `kind` says; this crate doesn't pick one for you.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Metadata {
    pub kind: MetadataBlockKind,
    pub text: String,
}

/// The metadata block `text` starts with, if any. Only the very beginning counts, as for the
/// renderer: a `---` later in the document is a thematic break.
///
/// ```
/// use vertigo_cmark::{metadata, pulldown_cmark::MetadataBlockKind};
///
/// let document = "+++\nid = \"cave\"\n+++\n\n# The cave\n";
/// let block = metadata(document).unwrap();
/// assert_eq!(block.kind, MetadataBlockKind::PlusesStyle);
/// assert_eq!(block.text, "id = \"cave\"\n");
/// ```
pub fn metadata(text: &str) -> Option<Metadata> {
    let mut events = Parser::new_ext(text, METADATA_OPTIONS);
    let Some(Event::Start(Tag::MetadataBlock(kind))) = events.next() else {
        return None;
    };
    let mut block = String::new();
    for event in events {
        match event {
            Event::Text(text) => block.push_str(&text),
            Event::End(TagEnd::MetadataBlock(_)) => break,
            _ => {}
        }
    }
    Some(Metadata { kind, text: block })
}
