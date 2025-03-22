pub use pulldown_cmark::Options;
use pulldown_cmark::Parser;
use vertigo::DomNode;

mod generate;

#[cfg(feature = "syntect")]
mod highlighting;

#[cfg(test)]
mod tests;

/// Converts a CommonMark string to Vertigo tree with default options.
pub fn to_vertigo(text: &str) -> DomNode {
    to_vertigo_opts(text, Options::ENABLE_TABLES)
}

/// Converts a CommonMark string to Vertigo tree with provided [Options].
pub fn to_vertigo_opts(text: &str, opts: Options) -> DomNode {
    let parser = Parser::new_ext(text, opts);
    generate::generate_tree(parser)
}
