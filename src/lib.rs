use pulldown_cmark::{Parser, Options};
use vertigo::VDomElement;

mod generate;
mod table_state;

#[cfg(test)]
mod tests;

/// Converts a CommonMark string to Vertigo tree
pub fn to_vertigo(text: &str) -> VDomElement {
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    // opts.insert(Options::ENABLE_FOOTNOTES);
    // opts.insert(Options::ENABLE_STRIKETHROUGH);
    // opts.insert(Options::ENABLE_TASKLISTS);
    // opts.insert(Options::ENABLE_SMART_PUNCTUATION);

    let parser = Parser::new_ext(text, opts);

    let children = generate::generate_tree(parser);

    VDomElement::new("div", vec![], children)
}
