use vertigo::{dom, inspect::{log_start, DomDebugFragment}};

use crate::to_vertigo;

#[test]
fn codeblock() {
    log_start();
    let _el1 = to_vertigo(r#"
Example of rust code:

```rust
let x = 2;
let y = x + 1;
```
"#);
    let el1_str = DomDebugFragment::from_log().to_pseudo_html();

    log_start();
    let _el2 = dom! {
        <div>
            <p>"Example of rust code:"</p>
            <pre>
                <code class="language-rust">
"let x = 2;
let y = x + 1;
"
                </code>
            </pre>
        </div>
    };
    let el2_str = DomDebugFragment::from_log().to_pseudo_html();

    assert_eq!(el1_str, el2_str);
}
