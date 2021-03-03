use pulldown_cmark::Alignment;
use std::collections::VecDeque;

#[derive(Default)]
pub(crate) struct TableState {
    // Stack of alignments for table building, most inner on top
    pub soa: VecDeque<Vec<Alignment>>,
    pub head: bool,
    pub cell_index: usize,
}

impl TableState {
    pub fn alignment(&self) -> Option<String> {
        self.soa.front()
            .map(|als| als.get(self.cell_index))
            .flatten()
            .and_then(|al| match al {
                Alignment::Left => Some("left".to_string()),
                Alignment::Center => Some("center".to_string()),
                Alignment::Right => Some("right".to_string()),
                _ => None
            })
    }
}