use pulldown_cmark::{Parser, Options};
use vertigo::DomElement;

mod generate;
mod table_state;

#[cfg(test)]
mod tests;

/// Converts a CommonMark string to Vertigo tree
pub fn to_vertigo(text: &str) -> DomElement {
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    // opts.insert(Options::ENABLE_FOOTNOTES);
    // opts.insert(Options::ENABLE_STRIKETHROUGH);
    // opts.insert(Options::ENABLE_TASKLISTS);
    // opts.insert(Options::ENABLE_SMART_PUNCTUATION);

    let parser = Parser::new_ext(text, opts);

    let children = generate::generate_tree(parser);

    let elem = DomElement::new("div");

    for child in children {
        elem.add_child(child);
    }

    elem
}
