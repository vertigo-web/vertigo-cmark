<!-- markdownlint-configure-file { "no-duplicate-heading": { "siblings_only": true } } -->

<!-- markdownlint-disable-next-line first-line-h1 -->
## Unreleased

### Added

* `events_to_vertigo` renders a stream of pulldown-cmark events, so the caller can adjust them
  first (drop unsafe links, sanitize raw HTML, turn a link into an embed); `pulldown_cmark`
  is re-exported

### Fixed

* Raw HTML: void elements without `/>` (`<br>`, `<img src="…">`) no longer swallow the rest
  of the document, which rendered as an empty `div`
* Raw HTML: an end tag without its start tag no longer closes the markdown element around it,
  and tags left open close with their markdown element or the document
* Raw HTML: text after the last tag in an HTML event (a line of an HTML block) is kept

## 0.1.3 - 2027-09-08

### Changed

* Upgrade to vertigo 0.13, minimal compatible version is 0.10.1

## 0.1.2 - 2027-07-01

### Added

* `browser_defaults` to [CMarkStyle](CMarkStyle)

## 0.1.1 - 2026-05-01

### Fixed

* Disabling `html` now correctly removes `html` code from pulldown-cmark (>200KB wasm reduction)

### Changed

* Box iterator in `generate_tree` (0.1KB wasm reduction or more depending on cmark usage)

### Internals

* Re-enabled nightly tests

## 0.1.0 - 2025-10-03

### Added

* Regular, bod, italic, strike-through text
* Headings
* Paragraphs
* Tables
* Blockquotes
* Codeblocks
* Code highlighting (with `syntect` feature)
* Lists (numbers, bullets)
* Rules
* Task list markers
* Footnotes
* Soft/hard breaks
* Links
* Images
* Html
