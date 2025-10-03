use vertigo::{
    dom,
    inspect::{DomDebugFragment, log_start},
};

use crate::to_vertigo;

#[test]
fn nested() {
    log_start();
    let _el1 = to_vertigo(r#"<div>Vertigo <strong>Cmark</strong> hereafter</div>"#);
    let el1_str = DomDebugFragment::from_log().to_pseudo_html();

    log_start();
    let _el2 = dom! {
        <div>
            <div>"Vertigo "<strong>"Cmark"</strong>" hereafter"</div>
        </div>
    };
    let el2_str = DomDebugFragment::from_log().to_pseudo_html();

    assert_eq!(el1_str, el2_str);
}

#[test]
fn nested_with_cmark_wont_work() {
    log_start();
    let _el1 = to_vertigo(r#"<p>Vertigo **Cmark** hereafter</p>"#);
    let el1_str = DomDebugFragment::from_log().to_pseudo_html();

    log_start();
    let _el2 = dom! {
        <div>
            <p>"Vertigo **Cmark** hereafter"</p>
        </div>
    };
    let el2_str = DomDebugFragment::from_log().to_pseudo_html();

    assert_eq!(el1_str, el2_str);
}

#[test]
fn anchor_with_cmark_works() {
    log_start();
    let _el1 = to_vertigo(
        r#"
<a href="https://github.com/vertigo-web/vertigo-cmark">Vertigo **Cmark**</a>

<a href="mailto:him@example.com">Do not email him</a>

<a href="https://github.com/vertigo-web/vertigo" target="_blank">Vertigo Code</a>
"#,
    );
    let el1_str = DomDebugFragment::from_log().to_pseudo_html();

    log_start();
    let _el2 = dom! {
        <div>
            <p><a href="https://github.com/vertigo-web/vertigo-cmark">"Vertigo <strong>Cmark</strong>"</a></p>
            <p><a href="mailto:him@example.com">"Do not email him"</a></p>
            <p><a href="https://github.com/vertigo-web/vertigo" target="_blank">"Vertigo Code"</a></p>
        </div>
    };
    let el2_str = DomDebugFragment::from_log().to_pseudo_html();

    assert_eq!(el1_str, el2_str);
}

#[test]
fn img() {
    log_start();
    let _el1 = to_vertigo(r#"Paragraph <img src="https://example.com/img.jpg" />"#);
    let el1_str = DomDebugFragment::from_log().to_pseudo_html();

    log_start();
    let _el2 = dom! {
        <div>
            <p>"Paragraph " <img src="https://example.com/img.jpg" /></p>
        </div>
    };
    let el2_str = DomDebugFragment::from_log().to_pseudo_html();

    assert_eq!(el1_str, el2_str);
}

#[test]
fn a_img() {
    log_start();
    let _el1 = to_vertigo(
        r#"Link: <a href="https://example.com/"><img src="https://example.com/img.jpg" /> <- click</a>"#,
    );
    let el1_str = DomDebugFragment::from_log().to_pseudo_html();

    log_start();
    let _el2 = dom! {
        <div>
            <p>"Link: " <a href="https://example.com/"><img src="https://example.com/img.jpg" />" <- click"</a></p>
        </div>
    };
    let el2_str = DomDebugFragment::from_log().to_pseudo_html();

    assert_eq!(el1_str, el2_str);
}
