// Based on https://github.com/pulldown-cmark/pulldown-cmark/blob/master/pulldown-cmark/src/html.rs

use pulldown_cmark::{Event, TagEnd};

use super::writer::{TableState, VertigoWriter};

impl<'a, I> VertigoWriter<'a, I>
where
    I: Iterator<Item = Event<'a>>,
{
    pub(super) fn end_tag(&mut self, tag: TagEnd) {
        match tag {
            TagEnd::HtmlBlock => {}
            TagEnd::Table => {
                // </tbody></table>
                self.pop_node();
                self.pop_node();
            }
            TagEnd::TableHead => {
                // </tr></thead><tbody>
                self.pop_node();
                self.pop_node();
                self.push_elname("tbody", &self.styling.clone().tbody);
                self.table_state = TableState::Body;
            }
            TagEnd::TableCell => {
                self.pop_node();
                self.table_cell_index += 1;
            }
            TagEnd::CodeBlock => {
                // </code> or </code></pre>
                self.pop_node();
                #[cfg(feature = "syntect")]
                {
                    self.in_code_block = None;
                }
                #[cfg(not(feature = "syntect"))]
                {
                    self.pop_node();
                }
            }
            TagEnd::TableRow
            | TagEnd::Paragraph
            | TagEnd::Heading(_)
            | TagEnd::BlockQuote(_)
            | TagEnd::List(_)
            | TagEnd::Item
            | TagEnd::DefinitionList
            | TagEnd::DefinitionListTitle
            | TagEnd::DefinitionListDefinition
            | TagEnd::Subscript
            | TagEnd::Superscript
            | TagEnd::Emphasis
            | TagEnd::Strong
            | TagEnd::Strikethrough
            | TagEnd::Link
            | TagEnd::FootnoteDefinition => {
                self.pop_node();
            }
            TagEnd::Image => {} // shouldn't happen, handled in start
            TagEnd::MetadataBlock(_) => {
                self.in_non_writing_block = false;
            }
        }
    }
}
