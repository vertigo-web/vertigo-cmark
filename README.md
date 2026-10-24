# vertigo-cmark

Converts CommonMark string into rendered vertigo DomElement.

[![crates.io](https://img.shields.io/crates/v/vertigo-cmark)](https://crates.io/crates/vertigo-cmark)
[![Documentation](https://docs.rs/vertigo-cmark/badge.svg)](https://docs.rs/vertigo-cmark)
![MIT or Apache 2.0 licensed](https://img.shields.io/crates/l/vertigo-cmark.svg)
[![Dependency Status](https://deps.rs/crate/vertigo-cmark/0.1.4/status.svg)](https://deps.rs/crate/vertigo-cmark/0.1.4)
[![CI](https://github.com/vertigo-web/vertigo-cmark/actions/workflows/pipeline.yaml/badge.svg)](https://github.com/vertigo-web/vertigo-cmark/actions/workflows/pipeline.yaml)
[![downloads](https://img.shields.io/crates/d/vertigo-cmark.svg)](https://crates.io/crates/vertigo-cmark)

See [Changelog](https://github.com/vertigo-web/vertigo-cmark/blob/master/CHANGES.md) for recent features.

## Example

Dependencies:

```toml
vertigo = "0.14"
vertigo-cmark = "0.1"
```

```rust
use vertigo::{DomNode, dom, main};

const CONTENT: &str = r#"
# Hello world

## Paragraph

Lorem ipsum dolor sit amet, __consectetur__ adipiscing elit, sed do
```eiusmod tempor incididunt```
ut *labore* et dolore magna aliqua.

## List

* Lorem ipsum
* dolor sit amet
* consectetur adipiscing elit

## Table

| Lorem           | Ipsum          |
| --------------- | -------------- |
| dolor sit amet  | consectetur    |
| adipiscing elit | sed do eiusmod |
"#;

#[main]
fn app() -> DomNode {
    let content = vertigo_cmark::to_vertigo(CONTENT);
    dom! {
        <html>
            <head/>
            <body>{ content }</body>
        </html>
    }
}
```

![image](example.png)

## Features

- [x] Regular, bod, italic, strike-through text
- [x] Headings
- [x] Paragraphs
- [x] Tables
- [x] Tables with merged cells and several header rows (`table_spans`)
- [x] Blockquotes
- [x] Codeblocks
- [x] Code highlighting (with `syntect` feature)
- [x] Lists (numbers, bullets)
- [x] Rules
- [x] Task list markers
- [x] Footnotes
- [x] Soft/hard breaks
- [x] Links
- [x] Images
- [x] Html (with `html` feature), sanitized by default

## Security

Markdown can carry raw HTML and links to any address, and vertigo-cmark builds
DOM from them directly — there is no HTML string to clean afterwards. So by
default everything goes through `SafeHtml`: elements and attributes from
`ammonia`'s default whitelist and links and images only with safe schemes.
Scripts, styles, event handlers, frames, forms and `javascript:` addresses
don't get into the page, so rendering text from users is safe.

For content you control, `TrustedHtml` renders everything as written. Anything
in between is an own `HtmlPolicy` — usually delegating to `SafeHtml` all but
a few elements:

```rust
use std::borrow::Cow;
use vertigo_cmark::{
    CMarkStyle, ElementAction, HtmlPolicy, SafeHtml, events_to_vertigo,
    pulldown_cmark::Parser,
};

/// Safe HTML plus YouTube players.
struct WithVideos;

impl HtmlPolicy for WithVideos {
    fn element(&self, name: &str) -> ElementAction {
        match name {
            "iframe" => ElementAction::Keep,
            name => SafeHtml.element(name),
        }
    }

    fn attribute<'v>(
        &self,
        element: &str,
        name: &str,
        value: &'v str,
    ) -> Option<Cow<'v, str>> {
        match (element, name) {
            ("iframe", "src") => value
                .starts_with("https://www.youtube.com/embed/")
                .then_some(Cow::Borrowed(value)),
            _ => SafeHtml.attribute(element, name, value),
        }
    }

    fn destination<'u>(
        &self,
        url: &'u str,
        image: bool,
    ) -> Option<Cow<'u, str>> {
        SafeHtml.destination(url, image)
    }
}

let content = events_to_vertigo(
    Parser::new(CONTENT),
    CMarkStyle::default(),
    &WithVideos,
);
```

## Tables with merged cells

GFM tables have one header row and no merged cells. `table_spans` adds
MultiMarkdown's syntax: an empty cell without a space (`||`) joins the cell on
its left, a cell of `^^` joins the one above, and every row above the delimiter
row belongs to the header — or none, when the delimiter row comes first.

```text
| Row | Valid votes             || Amount per vote |
| ^^  | percent | number of votes | ^^              |
| --- | ------- | --------------- | --------------- |
| 1   | to 5%   |                 | 5.77            |
```

pulldown-cmark's events can't carry `colspan` or `rowspan`, so
`table_spans::events` gives the document's events with such tables as raw HTML
around the events of their cells. Both `events_to_vertigo` (with the `html`
feature) and `pulldown_cmark::html::push_html` render them:

```rust
use vertigo_cmark::{
    CMarkStyle, SafeHtml, events_to_vertigo, pulldown_cmark::Options, table_spans,
};

let content = events_to_vertigo(
    table_spans::events(CONTENT, Options::ENABLE_TABLES),
    CMarkStyle::default(),
    &SafeHtml,
);
```

`table_spans::Table` reads such a table (`parse`) and writes it back with the
columns lined up (`write`) — e.g. for an editor.
