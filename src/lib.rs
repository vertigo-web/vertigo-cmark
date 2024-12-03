use pulldown_cmark::{Options, Parser};
use vertigo::{DomElement, DomNode};

mod generate;
mod table_state;

#[cfg(test)]
mod tests;

/// Converts a CommonMark string to Vertigo tree
pub fn to_vertigo(text: &str) -> DomNode {
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    // opts.insert(Options::ENABLE_FOOTNOTES);
    // opts.insert(Options::ENABLE_STRIKETHROUGH);
    // opts.insert(Options::ENABLE_TASKLISTS);
    // opts.insert(Options::ENABLE_SMART_PUNCTUATION);

    // Newlines will be removed by parser so add spaces
    // to not have things glued accidentally
    let text = text.replace('\n', " \n");

    let parser = Parser::new_ext(&text, opts);

    let children = generate::generate_tree(parser);

    let elem = DomElement::new("div");

    for child in children {
        elem.add_child(child);
    }

    elem.into()
}
