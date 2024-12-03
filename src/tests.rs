use vertigo::{dom, inspect::{log_start, DomDebugFragment}};

use super::to_vertigo;

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
fn heading() {
    log_start();
    let _el1 = to_vertigo(r#"
# Heading 1

foo

---

bar

```
example
```
"#);
    let el1_str = DomDebugFragment::from_log().to_pseudo_html();

    log_start();
    let _el2 = dom! {
        <div>
            <h1>"Heading 1"</h1>
            <p>"foo "</p>
            <hr />
            <p>"bar "</p>
            <pre>"example "</pre>
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
fn table_1() {
    log_start();
    let _el1 = to_vertigo(r##" foo|bar
 ---|---
 baz|bim
 "##);
    let el1_str = DomDebugFragment::from_log().to_pseudo_html();

    log_start();
    let _el2 = dom! {
        <div>
            <table border="1">
                <thead>
                    <tr>
                        <th>"foo"</th>
                        <th>"bar"</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>"baz"</td>
                        <td>"bim"</td>
                    </tr>
                </tbody>
            </table>
        </div>
    };
    let el2_str = DomDebugFragment::from_log().to_pseudo_html();

    assert_eq!(el1_str, el2_str);
}

#[test]
fn table_2() {
    log_start();
    let _el1 = to_vertigo(r##"
| Head cell  | Another      |
| ---------- | ------------ |
| Cell text  | Another cell |
| More cells | *below...*   |
| ```Inlines``` | __allowed__ |
"##);
    let el1_str = DomDebugFragment::from_log().to_pseudo_html();

    log_start();
    let _el2 = dom! {
        <div>
            <table border="1">
                <thead>
                    <tr>
                        <th>"Head cell"</th>
                        <th>"Another"</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>"Cell text"</td>
                        <td>"Another cell"</td>
                    </tr>
                    <tr>
                        <td>"More cells"</td>
                        <td><em>"below..."</em></td>
                    </tr>
                    <tr>
                        <td><code>"Inlines"</code></td>
                        <td><strong>"allowed"</strong></td>
                    </tr>
                </tbody>
            </table>
        </div>
    };
    let el2_str = DomDebugFragment::from_log().to_pseudo_html();

    assert_eq!(el1_str, el2_str);
}

#[test]
fn table_mixed() {
    log_start();
    let _el1 = to_vertigo(r##"# Something

I'm saying something

| Head 1 | Head 2 |
| ------ | ------ |
| Cell 1 | Cell 2 |
"##);
    let el1_str = DomDebugFragment::from_log().to_pseudo_html();

    log_start();
    let _el2 = dom! {
        <div>
            <h1>"Something"</h1>
            <p>"I'm saying something "</p>
            <table border="1">
                <thead>
                    <tr>
                        <th>"Head 1"</th>
                        <th>"Head 2"</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>"Cell 1"</td>
                        <td>"Cell 2"</td>
                    </tr>
                </tbody>
            </table>
        </div>
    };
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
            <p>"Some text. "</p>
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
            <p>"Some text. Some more text. "</p>
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
