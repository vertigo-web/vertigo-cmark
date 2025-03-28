use vertigo::{
    dom,
    inspect::{log_start, DomDebugFragment},
};

use crate::{to_vertigo, to_vertigo_opts, Options};

#[test]
fn lists() {
    log_start();
    let _el1 = to_vertigo(
        r#"
Unordered list:

- One
- Two

Ordered list:

1. One
2. Two

List continuation:

3. Three
4. Four
"#,
    );
    let el1_str = DomDebugFragment::from_log().to_pseudo_html();

    log_start();
    let _el2 = dom! {
        <div>
            <p>"Unordered list:"</p>

            <ul>
                <li>"One"</li>
                <li>"Two"</li>
            </ul>

            <p>"Ordered list:"</p>

            <ol>
                <li>"One"</li>
                <li>"Two"</li>
            </ol>

            <p>"List continuation:"</p>

            <ol start="3">
                <li>"Three"</li>
                <li>"Four"</li>
            </ol>
        </div>
    };
    let el2_str = DomDebugFragment::from_log().to_pseudo_html();

    assert_eq!(el1_str, el2_str);
}

#[test]
fn definition_list() {
    let opts = Options::ENABLE_DEFINITION_LIST;

    log_start();
    let _el1 = to_vertigo_opts(
        r#"
Coffee
  : Black hot drink

Milk
  : White cold drink
"#,
        opts,
    );
    let el1_str = DomDebugFragment::from_log().to_pseudo_html();

    log_start();
    let _el2 = dom! {
        <div>
            <dl>
                <dt>"Coffee"</dt>
                <dd>"Black hot drink"</dd>
                <dt>"Milk"</dt>
                <dd>"White cold drink"</dd>
            </dl>
        </div>
    };
    let el2_str = DomDebugFragment::from_log().to_pseudo_html();

    assert_eq!(el1_str, el2_str);
}

#[test]
fn task_list() {
    let opts = Options::ENABLE_TASKLISTS;

    log_start();
    let _el1 = to_vertigo_opts(
        r#"
Implement in vertigo-cmark:

- [x] Rendering CommonMark,
- [ ] Frying pancakes."#,
        opts,
    );

    let el1_str = DomDebugFragment::from_log().to_pseudo_html();

    log_start();
    let _el2 = dom! {
        <div>
            <p>"Implement in vertigo-cmark:"</p>
            <ul>
                <li>
                    <input checked="checked" disabled="" type="checkbox" />
                    "Rendering CommonMark,"
                </li>
                <li>
                    <input disabled="" type="checkbox" />
                    "Frying pancakes."
                </li>
            </ul>
        </div>
    };
    let el2_str = DomDebugFragment::from_log().to_pseudo_html();

    assert_eq!(el1_str, el2_str);
}
