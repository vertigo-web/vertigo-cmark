use vertigo::{
    dom,
    inspect::{log_start, DomDebugFragment},
    Css,
};

use crate::{to_vertigo_opts_styled, to_vertigo_styled, Options};

static TEST_STYLE: &str = "color: green";

fn test_css() -> Css {
    Css::str(TEST_STYLE)
}

#[test]
fn blockquote() {
    log_start();
    let _el1 = to_vertigo_opts_styled(
        "> [!WARNING]
> dolor sit amet
",
        Options::ENABLE_GFM,
        crate::CMarkStyle {
            blockquote: test_css(),
            ..Default::default()
        },
    );
    let el1_str = DomDebugFragment::from_log().to_pseudo_html();

    log_start();
    // autocss_1 because https://github.com/vertigo-web/vertigo/issues/335
    let _el2 = dom! { <div><blockquote class="markdown-alert-warning autocss_1"><p>"dolor sit amet"</p></blockquote></div> };
    let el2_str = DomDebugFragment::from_log().to_pseudo_html();

    assert_eq!(el1_str, el2_str);
}

#[cfg(not(feature = "syntect"))]
#[test]
fn codeblock() {
    log_start();
    let _el1 = to_vertigo_styled(
        "```
dolor sit amet
```",
        crate::CMarkStyle {
            codeblock: test_css(),
            ..Default::default()
        },
    );
    let el1_str = DomDebugFragment::from_log().to_pseudo_html();

    log_start();
    let _el2 = dom! { <div><pre style={TEST_STYLE}><code>"dolor sit amet\n"</code></pre></div> };
    let el2_str = DomDebugFragment::from_log().to_pseudo_html();

    assert_eq!(el1_str, el2_str);
}

#[cfg(feature = "syntect")]
#[test]
fn codeblock() {
    log_start();
    let _el1 = to_vertigo_styled(
        "```
dolor sit amet
```",
        crate::CMarkStyle {
            codeblock: test_css(),
            ..Default::default()
        },
    );
    let el1_str = DomDebugFragment::from_log().to_pseudo_html();

    log_start();
    let _el2 = dom! { <div><pre style={TEST_STYLE}><span style="color: rgba(211, 208, 200, 255); background_color: rgba(45, 45, 45, 255)">"dolor sit amet\n"</span></pre></div> };
    let el2_str = DomDebugFragment::from_log().to_pseudo_html();

    assert_eq!(el1_str, el2_str);
}

#[test]
fn container() {
    log_start();
    let _el1 = to_vertigo_styled(
        "",
        crate::CMarkStyle {
            container: test_css(),
            ..Default::default()
        },
    );
    let el1_str = DomDebugFragment::from_log().to_pseudo_html();

    log_start();
    let _el2 = dom! { <div style={TEST_STYLE}></div> };
    let el2_str = DomDebugFragment::from_log().to_pseudo_html();

    assert_eq!(el1_str, el2_str);
}

#[test]
fn description() {
    const DD_STYLE: &str = "color: red";
    const DL_STYLE: &str = "color: green";
    const DT_STYLE: &str = "color: blue";

    log_start();
    let _el1 = to_vertigo_opts_styled(
        r#"
Coffee
  : Black hot drink
"#,
        Options::ENABLE_DEFINITION_LIST,
        crate::CMarkStyle {
            dd: Css::str(DD_STYLE),
            dl: Css::str(DL_STYLE),
            dt: Css::str(DT_STYLE),
            ..Default::default()
        },
    );
    let el1_str = DomDebugFragment::from_log().to_pseudo_html();

    log_start();
    let _el2 = dom! {
        <div>
            <dl style={DL_STYLE}>
                <dt style={DT_STYLE}>"Coffee"</dt>
                <dd style={DD_STYLE}>"Black hot drink"</dd>
            </dl>
        </div>
    };
    let el2_str = DomDebugFragment::from_log().to_pseudo_html();

    assert_eq!(el1_str, el2_str);
}

#[test]
fn headers() {
    const H1_STYLE: &str = "color: red";
    const H2_STYLE: &str = "color: green";
    const H3_STYLE: &str = "color: blue";
    const H4_STYLE: &str = "color: gray";
    const H5_STYLE: &str = "color: violet";
    const H6_STYLE: &str = "color: tomato";

    log_start();
    let _el1 = to_vertigo_styled(
        r#"
# Header 1

## Header 2

### Header 3

#### Header 4

##### Header 5

###### Header 6
"#,
        crate::CMarkStyle {
            h1: Css::str(H1_STYLE),
            h2: Css::str(H2_STYLE),
            h3: Css::str(H3_STYLE),
            h4: Css::str(H4_STYLE),
            h5: Css::str(H5_STYLE),
            h6: Css::str(H6_STYLE),
            ..Default::default()
        },
    );
    let el1_str = DomDebugFragment::from_log().to_pseudo_html();

    log_start();
    let _el2 = dom! {
        <div>
            <h1 style={H1_STYLE}>"Header 1"</h1>
            <h2 style={H2_STYLE}>"Header 2"</h2>
            <h3 style={H3_STYLE}>"Header 3"</h3>
            <h4 style={H4_STYLE}>"Header 4"</h4>
            <h5 style={H5_STYLE}>"Header 5"</h5>
            <h6 style={H6_STYLE}>"Header 6"</h6>
        </div>
    };
    let el2_str = DomDebugFragment::from_log().to_pseudo_html();

    assert_eq!(el1_str, el2_str);
}

#[test]
fn image() {
    log_start();
    let _el1 = to_vertigo_styled(
        "![This is a cat](cat.png)",
        crate::CMarkStyle {
            img: test_css(),
            ..Default::default()
        },
    );
    let el1_str = DomDebugFragment::from_log().to_pseudo_html();

    log_start();
    let _el2 =
        dom! { <div><p><img alt="This is a cat" src="cat.png" style={TEST_STYLE} /></p></div> };
    let el2_str = DomDebugFragment::from_log().to_pseudo_html();

    assert_eq!(el1_str, el2_str);
}

#[test]
fn link() {
    log_start();
    let _el1 = to_vertigo_styled(
        "[Vertigo Cmark](https://github.com/vertigo-web/vertigo-cmark)",
        crate::CMarkStyle {
            a: test_css(),
            ..Default::default()
        },
    );
    let el1_str = DomDebugFragment::from_log().to_pseudo_html();

    log_start();
    let _el2 = dom! { <div><p><a href="https://github.com/vertigo-web/vertigo-cmark" style={TEST_STYLE}>"Vertigo Cmark"</a></p></div> };
    let el2_str = DomDebugFragment::from_log().to_pseudo_html();

    assert_eq!(el1_str, el2_str);
}

#[test]
fn lists() {
    const UL_STYLE: &str = "color: red";
    const OL_STYLE: &str = "color: green";
    const LI_STYLE: &str = "color: blue";

    log_start();
    let _el1 = to_vertigo_styled(
        r#"
- One

1. Two
"#,
        crate::CMarkStyle {
            ul: Css::str(UL_STYLE),
            ol: Css::str(OL_STYLE),
            li: Css::str(LI_STYLE),
            ..Default::default()
        },
    );
    let el1_str = DomDebugFragment::from_log().to_pseudo_html();

    log_start();
    let _el2 = dom! {
        <div>
            <ul style={UL_STYLE}>
                <li style={LI_STYLE}>"One"</li>
            </ul>
            <ol style={OL_STYLE}>
                <li style={LI_STYLE}>"Two"</li>
            </ol>
        </div>
    };
    let el2_str = DomDebugFragment::from_log().to_pseudo_html();

    assert_eq!(el1_str, el2_str);
}

#[test]
fn paragraph() {
    log_start();
    let _el1 = to_vertigo_styled(
        "Foobar",
        crate::CMarkStyle {
            p: test_css(),
            ..Default::default()
        },
    );
    let el1_str = DomDebugFragment::from_log().to_pseudo_html();

    log_start();
    let _el2 = dom! { <div><p style={TEST_STYLE}>"Foobar"</p></div> };
    let el2_str = DomDebugFragment::from_log().to_pseudo_html();

    assert_eq!(el1_str, el2_str);
}

#[test]
fn spans() {
    const EM_STYLE: &str = "color: red";
    const STRONG_STYLE: &str = "color: green";
    const SUB_STYLE: &str = "color: blue";
    const SUP_STYLE: &str = "color: gray";
    const DEL_STYLE: &str = "color: violet";

    log_start();
    let _el1 = to_vertigo_opts_styled(
        r#"
Lorem **ipsum** *dolor* sit[^1] ~amet~ ~~plumeth~~

[^1]: Legend
"#,
        Options::ENABLE_FOOTNOTES | Options::ENABLE_STRIKETHROUGH | Options::ENABLE_SUBSCRIPT,
        crate::CMarkStyle {
            em: Css::str(EM_STYLE),
            strong: Css::str(STRONG_STYLE),
            sub: Css::str(SUB_STYLE),
            sup: Css::str(SUP_STYLE),
            del: Css::str(DEL_STYLE),
            ..Default::default()
        },
    );
    let el1_str = DomDebugFragment::from_log().to_pseudo_html();

    log_start();
    let _el2 = dom! {
        <div>
            <p>
                "Lorem "
                <strong style={STRONG_STYLE}>"ipsum"</strong>" "
                <em style={EM_STYLE}>"dolor"</em>" sit"
                <sup class="footnote-reference autocss_3"><a href="#1">"1"</a></sup>" "
                <sub style={SUB_STYLE}>"amet"</sub>" "
                <del style={DEL_STYLE}>"plumeth"</del>
            </p>
            <div class="footnote-definition" id="1"><sup class="footnote-definition-label autocss_4">"1"</sup><p>"Legend"</p></div>
        </div>
    };
    let el2_str = DomDebugFragment::from_log().to_pseudo_html();

    assert_eq!(el1_str, el2_str);
}

#[test]
fn table() {
    const TABLE_STYLE: &str = "color: red";
    const TBODY_STYLE: &str = "color: green";
    const TD_STYLE: &str = "color: blue";
    const TH_STYLE: &str = "color: navy";
    const THEAD_STYLE: &str = "color: gray";
    const TR_STYLE: &str = "color: violet";

    log_start();
    let _el1 = to_vertigo_styled(
        r##" foo|bar
 ---|---
 baz|bim
 "##,
        crate::CMarkStyle {
            table: Css::str(TABLE_STYLE),
            tbody: Css::str(TBODY_STYLE),
            td: Css::str(TD_STYLE),
            th: Css::str(TH_STYLE),
            thead: Css::str(THEAD_STYLE),
            tr: Css::str(TR_STYLE),
            ..Default::default()
        },
    );
    let el1_str = DomDebugFragment::from_log().to_pseudo_html();

    log_start();
    let _el2 = dom! {
        <div>
            <table style={TABLE_STYLE}>
                <thead style={THEAD_STYLE}>
                    <tr style={TR_STYLE}>
                        <th style={TH_STYLE}>"foo"</th>
                        <th style={TH_STYLE}>"bar"</th>
                    </tr>
                </thead>
                <tbody style={TBODY_STYLE}>
                    <tr style={TR_STYLE}>
                        <td style={TD_STYLE}>"baz"</td>
                        <td style={TD_STYLE}>"bim"</td>
                    </tr>
                </tbody>
            </table>
        </div>
    };
    let el2_str = DomDebugFragment::from_log().to_pseudo_html();

    assert_eq!(el1_str, el2_str);
}
