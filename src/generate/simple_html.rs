//! A small tokenizer of raw HTML, for the `simple_html` feature.
//!
//! It reads what markdown authors write by hand - tags with attributes, text, comments - and
//! is a few kilobytes of wasm where `html5tokenizer`, behind the `html` feature, is about 155:
//! most of that one is the table of the 2231 named character references HTML knows.
//!
//! What it leaves out of how browsers read HTML:
//! - of the named character references, only the common ones are known (see [`NAMED`]).
//!   Another one stays as it is written, `&name;`
//! - a numeric reference into the C1 controls (`&#150;`) is that control, not the
//!   Windows-1252 character browsers make of it
//! - what browsers recover from in their own ways - a `<` inside a tag, a quote in an attribute
//!   name - may come out as other tags and text than in a browser. Never as something the
//!   [`HtmlPolicy`](crate::HtmlPolicy) didn't see: every element and attribute goes through it
//!   whatever the tokenizer made of the text.

/// A tag's attribute, its name in lower case and its value with character references read.
pub(super) struct Attribute {
    pub name: String,
    pub value: String,
}

/// A start or an end tag, its name in lower case.
pub(super) struct Tag {
    pub name: String,
    /// By their names, the order html5tokenizer gives them in.
    pub attributes: Vec<Attribute>,
    pub self_closing: bool,
}

/// The same shape as `html5tokenizer::Token`, of which [`super::html`] takes these.
pub(super) enum Token {
    StartTag(Tag),
    EndTag(Tag),
    Char(char),
}

/// The named character references known here, the ones hand-written HTML uses.
const NAMED: &[(&str, char)] = &[
    ("amp", '&'),
    ("lt", '<'),
    ("gt", '>'),
    ("quot", '"'),
    ("apos", '\''),
    ("nbsp", '\u{a0}'),
    ("shy", '\u{ad}'),
    ("ensp", '\u{2002}'),
    ("emsp", '\u{2003}'),
    ("thinsp", '\u{2009}'),
    ("zwnj", '\u{200c}'),
    ("zwj", '\u{200d}'),
    ("ndash", '–'),
    ("mdash", '—'),
    ("hellip", '…'),
    ("bull", '•'),
    ("middot", '·'),
    ("lsquo", '‘'),
    ("rsquo", '’'),
    ("sbquo", '‚'),
    ("ldquo", '“'),
    ("rdquo", '”'),
    ("bdquo", '„'),
    ("laquo", '«'),
    ("raquo", '»'),
    ("copy", '©'),
    ("reg", '®'),
    ("trade", '™'),
    ("sect", '§'),
    ("para", '¶'),
    ("deg", '°'),
    ("plusmn", '±'),
    ("times", '×'),
    ("divide", '÷'),
    ("minus", '−'),
    ("micro", 'µ'),
    ("frac12", '½'),
    ("frac14", '¼'),
    ("frac34", '¾'),
    ("sup2", '²'),
    ("sup3", '³'),
    ("le", '≤'),
    ("ge", '≥'),
    ("ne", '≠'),
    ("larr", '←'),
    ("rarr", '→'),
    ("uarr", '↑'),
    ("darr", '↓'),
    ("harr", '↔'),
    ("euro", '€'),
    ("pound", '£'),
    ("yen", '¥'),
    ("cent", '¢'),
];

/// Elements whose content is text up to their end tag, with no tags in it: with character
/// references read (`true`) or as it is written.
const TEXT_ELEMENTS: &[(&str, bool)] = &[
    ("script", false),
    ("style", false),
    ("xmp", false),
    ("iframe", false),
    ("noembed", false),
    ("noframes", false),
    ("textarea", true),
    ("title", true),
];

/// The tokens of a piece of raw HTML. Comments and `<!DOCTYPE>` give none, and neither does a
/// tag the input ends in the middle of.
pub(super) fn tokens(input: &str) -> Vec<Token> {
    let mut out = Vec::new();
    let mut rest = input;

    while let Some(at) = rest.find('<') {
        text(&rest[..at], true, &mut out);
        rest = &rest[at..];

        let Some((token, len)) = markup(rest) else {
            // not a tag: `1 < 2`
            out.push(Token::Char('<'));
            rest = &rest[1..];
            continue;
        };
        rest = &rest[len..];

        let Some(token) = token else {
            continue;
        };

        let content = match &token {
            // `<script />` too: browsers don't close it there
            Token::StartTag(tag) => TEXT_ELEMENTS
                .iter()
                .find(|(name, _)| *name == tag.name)
                .map(|(name, references)| (end_tag_at(rest, name), *references)),
            _ => None,
        };
        out.push(token);

        if let Some((end, references)) = content {
            text(&rest[..end], references, &mut out);
            rest = &rest[end..];
        }
    }

    text(rest, true, &mut out);
    out
}

/// Where the end tag of a text element starts in `rest`, the end of `rest` without one.
fn end_tag_at(rest: &str, name: &str) -> usize {
    let bytes = rest.as_bytes();
    let mut from = 0;

    while let Some(found) = rest[from..].find("</") {
        let at = from + found;
        let after = at + 2 + name.len();
        let named = bytes
            .get(at + 2..after)
            .is_some_and(|written| written.eq_ignore_ascii_case(name.as_bytes()));

        if named && bytes.get(after).is_none_or(|next| ends_name(*next)) {
            return at;
        }
        from = at + 2;
    }

    rest.len()
}

fn ends_name(byte: u8) -> bool {
    byte.is_ascii_whitespace() || byte == b'/' || byte == b'>'
}

/// What `rest`, starting with `<`, is: a tag, or a comment or a declaration (no token), with
/// how many bytes of `rest` it takes. `None` when the `<` is text.
fn markup(rest: &str) -> Option<(Option<Token>, usize)> {
    let bytes = rest.as_bytes();

    match *bytes.get(1)? {
        b'!' if rest.starts_with("<!--") => {
            let body = &rest[4..];
            // `<!-->` and `<!--->` are whole comments, empty ones
            let len = if body.starts_with('>') {
                5
            } else if body.starts_with("->") {
                6
            } else {
                body.find("-->").map_or(rest.len(), |end| 4 + end + 3)
            };
            Some((None, len))
        }
        // `<!DOCTYPE html>`, `<?xml ?>`, and `</>` with what browsers take for a comment
        b'!' | b'?' => Some((None, up_to_closing(rest))),
        b'/' => match bytes.get(2) {
            Some(byte) if byte.is_ascii_alphabetic() => Some(tag(rest, 2, false)),
            Some(_) => Some((None, up_to_closing(rest))),
            None => None,
        },
        byte if byte.is_ascii_alphabetic() => Some(tag(rest, 1, true)),
        _ => None,
    }
}

/// The length of `rest` up to its first `>`, with it.
fn up_to_closing(rest: &str) -> usize {
    rest.find('>').map_or(rest.len(), |end| end + 1)
}

/// The tag in `rest` whose name starts at `at`, and how many bytes of `rest` it takes. No token
/// when `rest` ends before the tag does.
fn tag(rest: &str, mut at: usize, start: bool) -> (Option<Token>, usize) {
    let bytes = rest.as_bytes();
    let unfinished = (None, rest.len());

    let name_from = at;
    while bytes.get(at).is_some_and(|byte| !ends_name(*byte)) {
        at += 1;
    }
    let mut tag = Tag {
        name: rest[name_from..at].to_ascii_lowercase(),
        attributes: Vec::new(),
        self_closing: false,
    };

    loop {
        while bytes.get(at).is_some_and(u8::is_ascii_whitespace) {
            at += 1;
        }

        match bytes.get(at) {
            None => return unfinished,
            Some(b'>') => {
                at += 1;
                break;
            }
            Some(b'/') => {
                at += 1;
                if bytes.get(at) == Some(&b'>') {
                    tag.self_closing = true;
                    at += 1;
                    break;
                }
                continue;
            }
            Some(_) => {}
        }

        // The name, to a white space, `/`, `>` or `=` - but an `=` it starts with is its own
        let name_from = at;
        at += 1;
        while bytes
            .get(at)
            .is_some_and(|byte| !ends_name(*byte) && *byte != b'=')
        {
            at += 1;
        }
        let name = rest[name_from..at].to_ascii_lowercase();

        while bytes.get(at).is_some_and(u8::is_ascii_whitespace) {
            at += 1;
        }

        let mut value = String::new();
        if bytes.get(at) == Some(&b'=') {
            at += 1;
            while bytes.get(at).is_some_and(u8::is_ascii_whitespace) {
                at += 1;
            }

            let written = match bytes.get(at) {
                None => return unfinished,
                Some(quote @ (b'"' | b'\'')) => {
                    let from = at + 1;
                    let Some(len) = rest[from..].find(char::from(*quote)) else {
                        return unfinished;
                    };
                    at = from + len + 1;
                    &rest[from..from + len]
                }
                Some(_) => {
                    let from = at;
                    while bytes
                        .get(at)
                        .is_some_and(|byte| !byte.is_ascii_whitespace() && *byte != b'>')
                    {
                        at += 1;
                    }
                    &rest[from..at]
                }
            };
            decode(written, &mut |ch| value.push(ch));
        }

        // The first of two attributes of a name is the one that counts, as in browsers
        let at_name = tag
            .attributes
            .iter()
            .position(|other| other.name >= name)
            .unwrap_or(tag.attributes.len());
        if tag
            .attributes
            .get(at_name)
            .is_none_or(|other| other.name != name)
        {
            tag.attributes.insert(at_name, Attribute { name, value });
        }
    }

    let token = if start {
        Token::StartTag(tag)
    } else {
        // An end tag's attributes and its `/` mean nothing
        Token::EndTag(Tag {
            attributes: Vec::new(),
            self_closing: false,
            ..tag
        })
    };

    (Some(token), at)
}

fn text(written: &str, references: bool, out: &mut Vec<Token>) {
    if references {
        decode(written, &mut |ch| out.push(Token::Char(ch)));
    } else {
        out.extend(written.chars().map(Token::Char));
    }
}

/// Gives `written` char by char, its character references read: `&#233;`, `&#xE9;` and the
/// names of [`NAMED`] (`&eacute;` isn't one, and stays as it is).
fn decode(written: &str, push: &mut dyn FnMut(char)) {
    let mut rest = written;

    while let Some(at) = rest.find('&') {
        rest[..at].chars().for_each(&mut *push);
        rest = &rest[at..];

        match reference(rest) {
            Some((ch, len)) => {
                push(ch);
                rest = &rest[len..];
            }
            None => {
                push('&');
                rest = &rest[1..];
            }
        }
    }

    rest.chars().for_each(push);
}

/// The character reference `rest` starts with, and its length.
fn reference(rest: &str) -> Option<(char, usize)> {
    let body = &rest[1..];

    if let Some(number) = body.strip_prefix('#') {
        let (digits, radix) = match number.strip_prefix(['x', 'X']) {
            Some(hex) => (hex, 16),
            None => (number, 10),
        };
        let len = digits
            .bytes()
            .take_while(|byte| char::from(*byte).is_digit(radix))
            .count();
        if len == 0 {
            return None;
        }

        // A number too big for a `u32` is no character either
        let code = u32::from_str_radix(&digits[..len], radix).unwrap_or(u32::MAX);
        let ch = match char::from_u32(code) {
            Some('\0') | None => char::REPLACEMENT_CHARACTER,
            Some(ch) => ch,
        };

        let mut taken = rest.len() - digits.len() + len;
        // Browsers read it without the `;` too
        if rest.as_bytes().get(taken) == Some(&b';') {
            taken += 1;
        }
        return Some((ch, taken));
    }

    let len = body.bytes().take_while(u8::is_ascii_alphanumeric).count();
    if body.as_bytes().get(len) != Some(&b';') {
        return None;
    }
    let name = &body[..len];
    let (_, ch) = NAMED.iter().find(|(known, _)| *known == name)?;

    Some((*ch, 1 + len + 1))
}

#[cfg(test)]
mod tests {
    use super::{Token, tokens};

    /// The tokens as text: `<a href="x">`, `</a>` and text between quotes.
    fn read(input: &str) -> String {
        let mut out = String::new();
        let mut in_text = false;
        let close_text = |out: &mut String, in_text: &mut bool| {
            if *in_text {
                out.push('"');
                *in_text = false;
            }
        };

        for token in tokens(input) {
            match token {
                Token::Char(ch) => {
                    if !in_text {
                        out.push('"');
                        in_text = true;
                    }
                    out.push(ch);
                }
                Token::StartTag(tag) => {
                    close_text(&mut out, &mut in_text);
                    out.push('<');
                    out.push_str(&tag.name);
                    for attr in &tag.attributes {
                        out.push_str(&format!(" {}={:?}", attr.name, attr.value));
                    }
                    out.push_str(if tag.self_closing { "/>" } else { ">" });
                }
                Token::EndTag(tag) => {
                    close_text(&mut out, &mut in_text);
                    out.push_str(&format!("</{}>", tag.name));
                }
            }
        }
        close_text(&mut out, &mut in_text);
        out
    }

    #[test]
    fn tags_and_text() {
        assert_eq!(
            read(r#"See <a href="https://example.com/?a=1&amp;b=2" target=_blank>this</a>."#),
            r#""See "<a href="https://example.com/?a=1&b=2" target="_blank">"this"</a>".""#
        );
        assert_eq!(
            read("<BR><Img SRC='a.png' Alt=\"\" />"),
            r#"<br><img alt="" src="a.png"/>"#
        );
        assert_eq!(read("</DIV ></p/>"), "</div></p>");
    }

    #[test]
    fn attributes() {
        // by their names, as html5tokenizer gives them
        assert_eq!(
            read("<input disabled checked = yes>"),
            r#"<input checked="yes" disabled="">"#
        );
        assert_eq!(
            read("<a title='say \"hi\"' title=second>"),
            r#"<a title="say \"hi\"">"#
        );
        assert_eq!(read("<a href=x/y>"), r#"<a href="x/y">"#);
        assert_eq!(read("<a\n  href=\"x\"\n>"), r#"<a href="x">"#);
        assert_eq!(read("<a / href=x>"), r#"<a href="x">"#);
    }

    #[test]
    fn character_references() {
        assert_eq!(
            read("a&nbsp;b &lt;i&gt; &copy; &mdash;"),
            "\"a\u{a0}b <i> © —\""
        );
        assert_eq!(read("&#233;&#xE9;&#XE9;&#233"), "\"éééé\"");
        assert_eq!(
            read("&#0;&#x110000;&#xD800;&#99999999999;"),
            "\"\u{fffd}\u{fffd}\u{fffd}\u{fffd}\""
        );
        // not known, not a reference at all
        assert_eq!(
            read("&eacute; & &amp &#; &#x; &;"),
            "\"&eacute; & &amp &#; &#x; &;\""
        );
        assert_eq!(
            read(r#"<a href="?a=1&copy=2&amp;c=3">"#),
            r#"<a href="?a=1&copy=2&c=3">"#
        );
        assert_eq!(
            read(r#"<a href="jav&#x61;script:x">"#),
            r#"<a href="javascript:x">"#
        );
    }

    #[test]
    fn what_is_no_tag() {
        assert_eq!(read("1 < 2, <3, <>, x<"), r#""1 < 2, <3, <>, x<""#);
        assert_eq!(read("a <!-- <b> --> c <!---> d <!-->e"), r#""a  c  d e""#);
        assert_eq!(read("<!DOCTYPE html><?xml ?></>x</ y>z"), r#""xz""#);
    }

    #[test]
    fn input_ending_in_a_tag() {
        assert_eq!(read("a <b"), r#""a ""#);
        assert_eq!(read("a <b title=\"x"), r#""a ""#);
        assert_eq!(read("a <b title="), r#""a ""#);
        assert_eq!(read("a <!-- b"), r#""a ""#);
        assert_eq!(read("a <"), r#""a <""#);
        assert_eq!(read("a </"), r#""a </""#);
    }

    #[test]
    fn text_elements() {
        assert_eq!(
            read("<script>if (a < b && c) { '</b>' }</SCRIPT >x"),
            r#"<script>"if (a < b && c) { '</b>' }"</script>"x""#
        );
        assert_eq!(
            read("<style>a > b { } &amp;</style>"),
            r#"<style>"a > b { } &amp;"</style>"#
        );
        assert_eq!(
            read("<title>a <b> &amp; c</title>"),
            r#"<title>"a <b> & c"</title>"#
        );
        assert_eq!(
            read("<script>never closed </scripty>"),
            r#"<script>"never closed </scripty>""#
        );
        assert_eq!(read("<script/>x<b>"), r#"<script/>"x<b>""#);
    }

    /// The same input through html5tokenizer, as [`read`] writes it.
    #[cfg(feature = "html")]
    fn read_by_html5tokenizer(input: &str) -> String {
        use html5tokenizer::{NaiveParser, Token};

        let mut out = String::new();
        let mut in_text = false;

        for token in NaiveParser::new(&input.to_string()).flatten() {
            if !matches!(token, Token::Char(_)) && in_text {
                // a comment ends no text in `read` either
                if matches!(token, Token::StartTag(_) | Token::EndTag(_)) {
                    out.push('"');
                    in_text = false;
                }
            }
            match token {
                Token::Char(ch) => {
                    if !in_text {
                        out.push('"');
                        in_text = true;
                    }
                    out.push(ch);
                }
                Token::StartTag(tag) => {
                    out.push('<');
                    out.push_str(&tag.name);
                    for attr in tag.attributes {
                        out.push_str(&format!(" {}={:?}", attr.name, attr.value));
                    }
                    out.push_str(if tag.self_closing { "/>" } else { ">" });
                }
                Token::EndTag(tag) => out.push_str(&format!("</{}>", tag.name)),
                Token::EndOfFile | Token::Comment(_) | Token::Doctype(_) => {}
            }
        }
        if in_text {
            out.push('"');
        }
        out
    }

    /// Hand-written HTML comes out of this tokenizer as out of html5tokenizer.
    #[cfg(feature = "html")]
    #[test]
    fn same_as_html5tokenizer() {
        for input in [
            r#"See <a href="https://example.com/?a=1&amp;b=2" target=_blank rel='noopener'>this</a>."#,
            "<BR><Img SRC='a.png' Alt=\"\" />",
            "</DIV ></p/>",
            "<input disabled checked = yes>",
            "<a title='say \"hi\"' title=second>",
            "<a href=x/y>",
            "<a\n  href=\"x\"\n>",
            "<a / href=x>",
            "<a b=2 a=1 c>",
            "a&nbsp;b &lt;i&gt; &copy; &mdash; &hellip; &bdquo;x&rdquo; &euro;",
            "&#233;&#xE9;&#XE9;&#233",
            "&#0;&#x110000;&#xD800;&#99999999999;",
            "& &#; &#x; &; &nosuchname;",
            r#"<a href="?a=1&copy=2&amp;c=3">"#,
            r#"<a href="jav&#x61;script:x">"#,
            "1 < 2, <3, <>, x<",
            "a <!-- <b> --> c <!---> d <!-->e",
            "<!DOCTYPE html></>x</ y>z",
            "a <b",
            "a <b title=\"x",
            "a <b title=",
            "a <!-- b",
            "a <",
            "a </",
            "<script>if (a < b && c) { '</b>' }</SCRIPT >x",
            "<style>a > b { } &amp;</style>",
            "<title>a <b> &amp; c</title>",
            "<textarea>a <b> &lt; c</textarea><i>",
            "<script>never closed </scripty>",
            "<script/>x<b>",
            "<iframe><b></iframe><xmp><i></xmp>",
            "<table><tr><td colspan=\"2\">a</td></tr></table>",
            "<div class=\"note\">\n  <p>Text <em>here</em></p>\n</div>\n",
        ] {
            assert_eq!(read(input), read_by_html5tokenizer(input), "{input}");
        }
    }
}
