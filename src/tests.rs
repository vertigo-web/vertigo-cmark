use super::to_vertigo;

use vertigo::html;

#[test]
fn text() {
    let el1 = to_vertigo("foo bar");
    let el2 = html! { <div><p>"foo bar"</p></div> };

    assert_eq!(
        format!("{:?}", el1),
        format!("{:?}", el2),
    );
}

#[test]
fn heading() {
    let el1 = to_vertigo(r#"
# Heading 1

foo

---

bar

```
example
```
    "#);

    let el2 = html! {
        <div>
            <h1>"Heading 1"</h1>
            <p>"foo"</p>
            <hr />
            <p>"bar"</p>
            <pre><code>"example"</code></pre>
        </div>
    };

    assert_eq!(
        format!("{:?}", el1),
        format!("{:?}", el2),
    );
}

#[test]
fn table_1() {
    let el1 = to_vertigo(r##" foo|bar
 ---|---
 baz|bim
"##);

    let el2 = html! {
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

    assert_eq!(
        format!("{:?}", el1),
        format!("{:?}", el2),
    );
}

#[test]
fn table_2() {
    let el1 = to_vertigo(r##"
| Head cell  | Another      |
| ---------- | ------------ |
| Cell text  | Another cell |
| More cells | *below...*   |
| ```Inlines``` | __allowed__ |
"##);

    let el2 = html! {
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
                        <td><pre><code>"Inlines"</code></pre></td>
                        <td><strong>"allowed"</strong></td>
                    </tr>
                </tbody>
            </table>
        </div>
    };

    println!("Result: {:#?}", el1);

    assert_eq!(
        format!("{:?}", el1),
        format!("{:?}", el2),
    );
}

#[test]
fn table_mixed() {
    let el1 = to_vertigo(r##"# Something

I'm saying something

| Head 1 | Head 2 |
| ------ | ------ |
| Cell 1 | Cell 2 |
"##);

    let el2 = html! {
        <div>
            <h1>"Something"</h1>
            <p>"I'm saying something"</p>
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

    assert_eq!(
        format!("{:?}", el1),
        format!("{:?}", el2),
    );
}
