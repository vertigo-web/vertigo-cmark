use pulldown_cmark::{Parser};
use vertigo::VDomElement;

mod generate;

#[cfg(test)]
mod tests;

/// Converts a CommonMark string to Vertigo tree
pub fn to_vertigo(text: &str) -> VDomElement {
    let parser = Parser::new(text);

    let children = generate::generate_tree(parser);

    VDomElement::new("div", vec![], children)
}
