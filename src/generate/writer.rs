// Based on https://github.com/pulldown-cmark/pulldown-cmark/blob/master/pulldown-cmark/src/html.rs

use pulldown_cmark::{Alignment, CowStr, Event};
use std::{
    collections::{HashMap, VecDeque},
    rc::Rc,
};
use vertigo::{Css, DomElement, DomNode, log};

use crate::styling::CMarkStyle;

pub(super) enum TableState {
    Head,
    Body,
}

pub(super) struct VertigoWriter<'a, I> {
    /// Iterator supplying events.
    pub(super) iter: I,

    /// Whether if inside a metadata block (text should not be written)
    pub(super) in_non_writing_block: bool,

    pub(super) table_state: TableState,
    pub(super) table_alignments: Vec<Alignment>,
    pub(super) table_cell_index: usize,
    pub(super) numbers: HashMap<CowStr<'a>, usize>,

    // Stack of nested nodes
    pub(super) soc: VecDeque<DomNode>,

    pub(super) styling: Rc<CMarkStyle>,

    #[cfg(feature = "syntect")]
    pub(super) in_code_block: Option<CowStr<'a>>,
}

impl<'a, I> VertigoWriter<'a, I>
where
    I: Iterator<Item = Event<'a>>,
{
    pub fn new(iter: I, styling: CMarkStyle) -> Self {
        Self {
            iter,
            in_non_writing_block: false,
            table_state: TableState::Head,
            table_alignments: vec![],
            table_cell_index: 0,
            numbers: HashMap::new(),
            soc: VecDeque::new(),
            styling: Rc::new(styling),
            #[cfg(feature = "syntect")]
            in_code_block: None,
        }
    }

    pub(super) fn push_node(&mut self, node: impl Into<DomNode>) {
        self.soc.push_front(node.into());
    }

    pub(super) fn push_element_styled(&mut self, element: DomElement, css: &Css) {
        let mut element = element;
        if !css.groups.is_empty() {
            element = element.css(css)
        }
        self.soc.push_front(element.into());
    }

    pub(super) fn push_elname(&mut self, name: impl Into<String>, css: &Css) {
        let name = name.into();
        let mut element = DomElement::new(name);
        if !css.groups.is_empty() {
            element = element.css(css)
        }
        self.push_node(element);
    }

    pub(super) fn pop_node(&mut self) -> Option<DomNode> {
        if let Some(child) = self.soc.pop_front() {
            match self.soc.front_mut() {
                Some(parent) => {
                    match parent {
                        DomNode::Node { node } => node.add_child(child),
                        _ => {
                            unreachable!("Can't push children to non-element node");
                        }
                    }
                    return None;
                }
                None => return Some(child),
            }
        }
        None
    }

    pub(super) fn add_child(&mut self, child: impl Into<DomNode>) {
        if let Some(parent) = self.soc.front_mut() {
            match parent {
                DomNode::Node { node } => node.add_child(child),
                _ => log::error!("Can't push child to non-element node (2)"),
            }
        } else {
            log::error!("Can't add child without parent node")
        }
    }

    pub(super) fn add_child_name(&mut self, child_name: impl Into<String>) {
        self.add_child(DomElement::new(child_name.into()));
    }
}
