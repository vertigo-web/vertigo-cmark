// Based on https://github.com/pulldown-cmark/pulldown-cmark/blob/master/pulldown-cmark/src/html.rs

use pulldown_cmark::{Event, Event::*};

use super::VertigoWriter;

impl<'a, I> VertigoWriter<'a, I>
where
    I: Iterator<Item = Event<'a>>,
{
    // run raw text, consuming end tag
    pub(super) fn raw_text(&mut self) -> String {
        let mut nest = 0;
        let mut writer = String::new();
        for event in self.iter.by_ref() {
            match event {
                Start(_) => nest += 1,
                End(_) => {
                    if nest == 0 {
                        break;
                    }
                    nest -= 1;
                }
                Html(_) => {}
                InlineHtml(text) | Code(text) | Text(text) => {
                    writer.push_str(&text);
                }
                InlineMath(text) => {
                    writer.push('$');
                    writer.push_str(&text);
                    writer.push('$');
                }
                DisplayMath(text) => {
                    writer.push_str("$$");
                    writer.push_str(&text);
                    writer.push_str("$$");
                }
                SoftBreak | HardBreak | Rule => {
                    writer.push(' ');
                }
                FootnoteReference(name) => {
                    let len = self.numbers.len() + 1;
                    let number = *self.numbers.entry(name).or_insert(len);
                    writer.push_str(&format!("[{}]", number));
                }
                TaskListMarker(true) => {
                    writer.push_str("[x]");
                }
                TaskListMarker(false) => {
                    writer.push_str("[ ]");
                }
            }
        }
        writer
    }
}
