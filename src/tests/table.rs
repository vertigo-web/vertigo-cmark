use vertigo::{
    dom,
    inspect::{log_start, DomDebugFragment},
};

use crate::to_vertigo;

#[test]
fn table_1() {
    log_start();
    let _el1 = to_vertigo(
        r##" foo|bar
 ---|---
 baz|bim
 "##,
    );
    let el1_str = DomDebugFragment::from_log().to_pseudo_html();

    log_start();
    let _el2 = dom! {
        <div>
            <table style="border: 1">
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
    let _el1 = to_vertigo(
        r##"
| Head cell  | Another      |
| ---------- | ------------ |
| Cell text  | Another cell |
| More cells | *below...*   |
| ```Inlines``` | __allowed__ |
"##,
    );
    let el1_str = DomDebugFragment::from_log().to_pseudo_html();

    log_start();
    let _el2 = dom! {
        <div>
            <table style="border: 1">
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
    let _el1 = to_vertigo(
        r##"# Something

I'm saying something

| Head 1 | Head 2 |
| ------ | ------ |
| Cell 1 | Cell 2 |
"##,
    );
    let el1_str = DomDebugFragment::from_log().to_pseudo_html();

    log_start();
    let _el2 = dom! {
        <div>
            <h1>"Something"</h1>
            <p>"I'm saying something"</p>
            <table style="border: 1">
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
