use vertigo::{
    dom,
    inspect::{log_start, DomDebugFragment},
};

use crate::to_vertigo;

#[test]
fn codeblock() {
    log_start();
    let _el1 = to_vertigo(
        r#"
Example of rust code:

```rust
let x = 2 + 2;
if x == 4 {
    println!("{}", x);
}
```
"#,
    );
    let el1_str = DomDebugFragment::from_log().to_pseudo_html();

    log_start();
    let _el2 = dom! {
        <div>
            <p>"Example of rust code:"</p>
            <pre style="background-color: black">
                <span style="color: rgba(180, 142, 173, 255); background_color: rgba(43, 48, 59, 255)">"let"</span>
                <span style="color: rgba(192, 197, 206, 255); background_color: rgba(43, 48, 59, 255)">" x "</span>
                <span style="color: rgba(192, 197, 206, 255); background_color: rgba(43, 48, 59, 255)">"="</span>
                <span style="color: rgba(192, 197, 206, 255); background_color: rgba(43, 48, 59, 255)">" "</span>
                <span style="color: rgba(208, 135, 112, 255); background_color: rgba(43, 48, 59, 255)">"2"</span>
                <span style="color: rgba(192, 197, 206, 255); background_color: rgba(43, 48, 59, 255)">" "</span>
                <span style="color: rgba(192, 197, 206, 255); background_color: rgba(43, 48, 59, 255)">"+"</span>
                <span style="color: rgba(192, 197, 206, 255); background_color: rgba(43, 48, 59, 255)">" "</span>
                <span style="color: rgba(208, 135, 112, 255); background_color: rgba(43, 48, 59, 255)">"2"</span>
                <span style="color: rgba(192, 197, 206, 255); background_color: rgba(43, 48, 59, 255)">";"</span>
                <span style="color: rgba(192, 197, 206, 255); background_color: rgba(43, 48, 59, 255)">"\n"</span>
                <span style="color: rgba(180, 142, 173, 255); background_color: rgba(43, 48, 59, 255)">"if"</span>
                <span style="color: rgba(192, 197, 206, 255); background_color: rgba(43, 48, 59, 255)">" x "</span>
                <span style="color: rgba(192, 197, 206, 255); background_color: rgba(43, 48, 59, 255)">"="</span>
                <span style="color: rgba(192, 197, 206, 255); background_color: rgba(43, 48, 59, 255)">"="</span>
                <span style="color: rgba(192, 197, 206, 255); background_color: rgba(43, 48, 59, 255)">" "</span>
                <span style="color: rgba(208, 135, 112, 255); background_color: rgba(43, 48, 59, 255)">"4"</span>
                <span style="color: rgba(192, 197, 206, 255); background_color: rgba(43, 48, 59, 255)">" "</span>
                <span style="color: rgba(192, 197, 206, 255); background_color: rgba(43, 48, 59, 255)">"{"</span>
                <span style="color: rgba(192, 197, 206, 255); background_color: rgba(43, 48, 59, 255)">"\n"</span>
                <span style="color: rgba(192, 197, 206, 255); background_color: rgba(43, 48, 59, 255)">"    "</span>
                <span style="color: rgba(192, 197, 206, 255); background_color: rgba(43, 48, 59, 255)">"println!"</span>
                <span style="color: rgba(192, 197, 206, 255); background_color: rgba(43, 48, 59, 255)">"("</span>
                <span style="color: rgba(192, 197, 206, 255); background_color: rgba(43, 48, 59, 255)">"\""</span>
                <span style="color: rgba(208, 135, 112, 255); background_color: rgba(43, 48, 59, 255)">"{}"</span>
                <span style="color: rgba(192, 197, 206, 255); background_color: rgba(43, 48, 59, 255)">"\""</span>
                <span style="color: rgba(192, 197, 206, 255); background_color: rgba(43, 48, 59, 255)">","</span>
                <span style="color: rgba(192, 197, 206, 255); background_color: rgba(43, 48, 59, 255)">" x"</span>
                <span style="color: rgba(192, 197, 206, 255); background_color: rgba(43, 48, 59, 255)">")"</span>
                <span style="color: rgba(192, 197, 206, 255); background_color: rgba(43, 48, 59, 255)">";"</span>
                <span style="color: rgba(192, 197, 206, 255); background_color: rgba(43, 48, 59, 255)">"\n"</span>
                <span style="color: rgba(192, 197, 206, 255); background_color: rgba(43, 48, 59, 255)">"}"</span>
                <span style="color: rgba(192, 197, 206, 255); background_color: rgba(43, 48, 59, 255)">"\n"</span>
            </pre>
        </div>
    };
    let el2_str = DomDebugFragment::from_log().to_pseudo_html();

    assert_eq!(el1_str, el2_str);
}

#[test]
fn codeblock_no_info() {
    log_start();
    let _el1 = to_vertigo(
        r#"
Example of unknown code:
```
example
```
"#,
    );
    let el1_str = DomDebugFragment::from_log().to_pseudo_html();

    log_start();
    let _el2 = dom! {
        <div>
            <p>"Example of unknown code:"</p>
            <pre style="background-color: black">
                <span style="color: rgba(192, 197, 206, 255); background_color: rgba(43, 48, 59, 255)">"example\n"</span>
            </pre>
        </div>
    };
    let el2_str = DomDebugFragment::from_log().to_pseudo_html();

    assert_eq!(el1_str, el2_str);
}
