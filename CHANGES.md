<!-- markdownlint-configure-file { "no-duplicate-heading": { "siblings_only": true } } -->

<!-- markdownlint-disable-next-line first-line-h1 -->

## Unreleased

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
