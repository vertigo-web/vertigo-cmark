pub use pulldown_cmark::Options;
use pulldown_cmark::Parser;
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
