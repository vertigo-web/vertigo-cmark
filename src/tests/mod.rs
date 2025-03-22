use vertigo::{dom, inspect::{log_start, DomDebugFragment}};

use crate::{to_vertigo, to_vertigo_opts};

#[cfg(not(feature = "syntect"))]
mod code;

#[cfg(feature = "syntect")]
mod code_highlighting;

mod lists;
mod table;

#[test]
fn text() {
    log_start();
    let _el1 = to_vertigo("foo bar");
    let el1_str = DomDebugFragment::from_log().to_pseudo_html();

    log_start();
    let _el2 = dom! { <div><p>"foo bar"</p></div> };
    let el2_str = DomDebugFragment::from_log().to_pseudo_html();

    assert_eq!(el1_str, el2_str);
}

#[test]
fn heading_rule() {
    log_start();
    let _el1 = to_vertigo(r#"
# Heading 1

foo

---

bar
"#);
    let el1_str = DomDebugFragment::from_log().to_pseudo_html();

    log_start();
    let _el2 = dom! {
        <div>
            <h1>"Heading 1"</h1>
            <p>"foo"</p>
            <hr />
            <p>"bar"</p>
        </div>
    };
    let el2_str = DomDebugFragment::from_log().to_pseudo_html();

    assert_eq!(el1_str, el2_str);
}

#[test]
fn text_bold_inline() {
    log_start();
    let _el1 = to_vertigo("foo __spam__ bar");
    let el1_str = DomDebugFragment::from_log().to_pseudo_html();

    log_start();
    let _el2 = dom! { <div><p>"foo <strong>spam</strong> bar"</p></div> };
    let el2_str = DomDebugFragment::from_log().to_pseudo_html();

    assert_eq!(el1_str, el2_str);
}

#[test]
fn text_italics_inline() {
    log_start();
    let _el1 = to_vertigo("foo *spam* bar");
    let el1_str = DomDebugFragment::from_log().to_pseudo_html();

    log_start();
    let _el2 = dom! { <div><p>"foo <em>spam</em> bar"</p></div> };
    let el2_str = DomDebugFragment::from_log().to_pseudo_html();

    assert_eq!(el1_str, el2_str);
}

#[test]
fn paragraph_line_join() {
    log_start();
    let _el1 = to_vertigo(r#"Some text.
Another line for the same paragraph.
And another"#);
    let el1_str = DomDebugFragment::from_log().to_pseudo_html();

    log_start();
    let _el2 = dom! {
        <div>
            <p>"Some text. Another line for the same paragraph. And another"</p>
        </div>
    };
    let el2_str = DomDebugFragment::from_log().to_pseudo_html();

    assert_eq!(el1_str, el2_str);
}

#[test]
fn two_paragraphs() {
    log_start();
    let _el1 = to_vertigo(r#"Some text.

Line for second paragraph.
Another line of second paragraph."#);
    let el1_str = DomDebugFragment::from_log().to_pseudo_html();

    log_start();
    let _el2 = dom! {
        <div>
            <p>"Some text."</p>
            <p>"Line for second paragraph. Another line of second paragraph."</p>
        </div>
    };
    let el2_str = DomDebugFragment::from_log().to_pseudo_html();

    assert_eq!(el1_str, el2_str);
}

#[test]
fn two_paragraphs_2() {
    log_start();
    let _el1 = to_vertigo(r#"Some text.
Some more text.

Line for second paragraph.
Another line of second paragraph."#);
    let el1_str = DomDebugFragment::from_log().to_pseudo_html();

    log_start();
    let _el2 = dom! {
        <div>
            <p>"Some text. Some more text."</p>
            <p>"Line for second paragraph. Another line of second paragraph."</p>
        </div>
    };
    let el2_str = DomDebugFragment::from_log().to_pseudo_html();

    assert_eq!(el1_str, el2_str);
}

#[test]
fn brackets() {
    log_start();
    let _el1 = to_vertigo(r#"Text with [two or more] brackets"#);
    let el1_str = DomDebugFragment::from_log().to_pseudo_html();

    log_start();
    let _el2 = dom! {
        <div>
            <p>"Text with [two or more] brackets"</p>
        </div>
    };
    let el2_str = DomDebugFragment::from_log().to_pseudo_html();

    assert_eq!(el1_str, el2_str);
}

#[test]
fn math() {
    use super::Options;
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_MATH);

    log_start();
    let _el1 = to_vertigo_opts(r#"
Some $2+2=4$ equation.

And more complex one:

$$a^2 + 2b + c = 0$$"#, opts);

    let el1_str = DomDebugFragment::from_log().to_pseudo_html();

    log_start();
    let _el2 = dom! {
        <div>
            <p>"Some "<span class="math math-inline">"2+2=4"</span>" equation."</p>
            <p>"And more complex one:"</p>
            <p><span class="math math-display">"a^2 + 2b + c = 0"</span></p>
        </div>
    };
    let el2_str = DomDebugFragment::from_log().to_pseudo_html();

    assert_eq!(el1_str, el2_str);
}

#[test]
fn footnotes() {
    use super::Options;
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_FOOTNOTES);

    log_start();
    let _el1 = to_vertigo_opts(r#"
This is sentence with footnote[^1]. I hope it works[^2].

[^1]: In fact, this is a footnote.

[^2]: Or at least the test passes."#, opts);

    let el1_str = DomDebugFragment::from_log().to_pseudo_html();

    log_start();
    let _el2 = dom! {
        <div>
            <p>
                "This is sentence with footnote"
                <sup class="footnote-reference"><a href="#1">"1"</a></sup>
                ". I hope it works"
                <sup class="footnote-reference"><a href="#2">"2"</a></sup>
                "."
            </p>
            <div class="footnote-definition" id="1">
                <sup class="footnote-definition-label">"1"</sup>
                <p>"In fact, this is a footnote."</p>
            </div>
            <div class="footnote-definition" id="2">
                <sup class="footnote-definition-label">"2"</sup>
                <p>"Or at least the test passes."</p>
            </div>
        </div>
    };
    let el2_str = DomDebugFragment::from_log().to_pseudo_html();

    assert_eq!(el1_str, el2_str);
}

#[test]
fn all_headings() {
    log_start();
    let _el1 = to_vertigo(r#"
# Heading 1

## Heading 2

### Heading 3

#### Heading 4

##### Heading 5

###### Heading 6
"#);
    let el1_str = DomDebugFragment::from_log().to_pseudo_html();

    log_start();
    let _el2 = dom! {
        <div>
            <h1>"Heading 1"</h1>
            <h2>"Heading 2"</h2>
            <h3>"Heading 3"</h3>
            <h4>"Heading 4"</h4>
            <h5>"Heading 5"</h5>
            <h6>"Heading 6"</h6>
        </div>
    };
    let el2_str = DomDebugFragment::from_log().to_pseudo_html();

    assert_eq!(el1_str, el2_str);
}

#[test]
fn blockquote() {
    use super::Options;
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_GFM);

    log_start();
    let _el1 = to_vertigo_opts(r#"
Usual text for testing:

> Lorem ipsum

> [!NOTE]
> dolor sit amet

> [!TIP]
> dolor sit amet

> [!IMPORTANT]
> dolor sit amet

> [!WARNING]
> dolor sit amet

> [!CAUTION]
> dolor sit amet
"#, opts);
    let el1_str = DomDebugFragment::from_log().to_pseudo_html();

    log_start();
    let _el2 = dom! {
        <div>
            <p>"Usual text for testing:"</p>
            <blockquote>
                <p>"Lorem ipsum"</p>
            </blockquote>
            <blockquote class="markdown-alert-note">
                <p>"dolor sit amet"</p>
            </blockquote>
            <blockquote class="markdown-alert-tip">
                <p>"dolor sit amet"</p>
            </blockquote>
            <blockquote class="markdown-alert-important">
                <p>"dolor sit amet"</p>
            </blockquote>
            <blockquote class="markdown-alert-warning">
                <p>"dolor sit amet"</p>
            </blockquote>
            <blockquote class="markdown-alert-caution">
                <p>"dolor sit amet"</p>
            </blockquote>
        </div>
    };
    let el2_str = DomDebugFragment::from_log().to_pseudo_html();

    assert_eq!(el1_str, el2_str);
}

#[test]
fn link() {
    log_start();
    let _el1 = to_vertigo(r#"
[Vertigo Cmark](https://github.com/vertigo-web/vertigo-cmark)

[Vertigo][vertigo]

[do not email him](mailto:him@example.com)

[vertigo]: https://github.com/vertigo-web/vertigo "Vertigo Code"
"#);
    let el1_str = DomDebugFragment::from_log().to_pseudo_html();

    log_start();
    let _el2 = dom! {
        <div>
            <p><a href="https://github.com/vertigo-web/vertigo-cmark">"Vertigo Cmark"</a></p>
            <p>
                <a href="https://github.com/vertigo-web/vertigo" title="Vertigo Code">
                    "Vertigo"
                </a>
            </p>
            <p><a href="mailto:him@example.com">"do not email him"</a></p>
        </div>
    };
    let el2_str = DomDebugFragment::from_log().to_pseudo_html();

    assert_eq!(el1_str, el2_str);
}

#[test]
fn image() {
    log_start();
    let _el1 = to_vertigo(r#"
![This is a cat](cat.png)

![This is a dog][dog]

[dog]: dog.jpg "An image of a dog"
"#);
    let el1_str = DomDebugFragment::from_log().to_pseudo_html();

    log_start();
    let _el2 = dom! {
        <div>
            <p><img alt="This is a cat" src="cat.png" /></p>
            <p><img alt="This is a dog" title="An image of a dog" src="dog.jpg" /></p>
        </div>
    };
    let el2_str = DomDebugFragment::from_log().to_pseudo_html();

    assert_eq!(el1_str, el2_str);
}
