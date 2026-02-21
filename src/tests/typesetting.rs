use vertigo::{
    dev::inspect::{DomDebugFragment, log_start},
    dom,
};

use crate::to_vertigo;

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
    let _el1 = to_vertigo(
        r#"
# Heading 1

foo

---

bar
"#,
    );
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
    let _el1 = to_vertigo(
        r#"Some text.
Another line for the same paragraph.
And another"#,
    );
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
    let _el1 = to_vertigo(
        r#"Some text.

Line for second paragraph.
Another line of second paragraph."#,
    );
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
    let _el1 = to_vertigo(
        r#"Some text.
Some more text.

Line for second paragraph.
Another line of second paragraph."#,
    );
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
fn all_headings() {
    log_start();
    let _el1 = to_vertigo(
        r#"
# Heading 1

## Heading 2

### Heading 3

#### Heading 4

##### Heading 5

###### Heading 6
"#,
    );
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
