<!-- markdownlint-configure-file { "no-duplicate-heading": { "siblings_only": true } } -->

<!-- markdownlint-disable-next-line first-line-h1 -->
## Unreleased

### Security

* Raw HTML and destinations of links and images no longer get into the tree
  as written. `<script>`, event handler attributes (`onerror`), `javascript:`
  links, `<iframe srcdoc>` and the like ran scripts in the page rendering
  markdown — XSS for any text coming from users. They now go through
  `SafeHtml`: `ammonia`'s default whitelist of elements and attributes and safe
  URL schemes

### Changed

* **Breaking:** `to_vertigo` and its variants render raw HTML and links through
  `SafeHtml`, so elements and attributes outside its whitelist (`class`,
  `style`, `target`, `<iframe>`…) are dropped. `TrustedHtml` renders everything
  as before — for content you control

### Added

* `events_to_vertigo` renders a stream of pulldown-cmark events, so the caller
  can adjust them first (e.g. turn a link into an embed), with a given policy;
  `pulldown_cmark` is re-exported
* `HtmlPolicy` decides what of raw HTML and link destinations gets into
  the tree; `SafeHtml`, `TrustedHtml` and `is_safe_url` for own policies

### Fixed

* Raw HTML: void elements without `/>` (`<br>`, `<img src="…">`) no longer
  swallow the rest of the document, which rendered as an empty `div`
* Raw HTML: an end tag without its start tag no longer closes the markdown
  element around it, and tags left open close with their markdown element
  or the document
* Raw HTML: text after the last tag in an HTML event (a line of an HTML block)
  is kept

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
