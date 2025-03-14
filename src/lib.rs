pub use pulldown_cmark::Options;
use pulldown_cmark::Parser;
use vertigo::DomNode;

mod generate;

#[cfg(test)]
mod tests;

/// Converts a CommonMark string to Vertigo tree with default options.
pub fn to_vertigo(text: &str) -> DomNode {
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    // opts.insert(Options::ENABLE_FOOTNOTES);
    // opts.insert(Options::ENABLE_STRIKETHROUGH);
    // opts.insert(Options::ENABLE_TASKLISTS);
    // opts.insert(Options::ENABLE_SMART_PUNCTUATION);

    to_vertigo_opts(text, opts)
}

/// Converts a CommonMark string to Vertigo tree with provided [Options].
pub fn to_vertigo_opts(text: &str, opts: Options) -> DomNode {
    let parser = Parser::new_ext(text, opts);
    generate::generate_tree(parser)
}
