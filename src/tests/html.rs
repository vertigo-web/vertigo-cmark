use vertigo::{
    dev::inspect::{DomDebugFragment, log_start},
    dom,
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

<a href="https://github.com/vertigo-web/vertigo" title="Vertigo">Vertigo Code</a>
"#,
    );
    let el1_str = DomDebugFragment::from_log().to_pseudo_html();

    log_start();
    let _el2 = dom! {
        <div>
            <p><a href="https://github.com/vertigo-web/vertigo-cmark">"Vertigo <strong>Cmark</strong>"</a></p>
            <p><a href="mailto:him@example.com">"Do not email him"</a></p>
            <p><a href="https://github.com/vertigo-web/vertigo" title="Vertigo">"Vertigo Code"</a></p>
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

#[test]
fn void_elements_without_slash() {
    log_start();
    let _el1 = to_vertigo(r#"Line<br>next <img src="https://example.com/img.jpg"> after"#);
    let el1_str = DomDebugFragment::from_log().to_pseudo_html();

    log_start();
    let _el2 = dom! {
        <div>
            <p>"Line"<br />"next "<img src="https://example.com/img.jpg" />" after"</p>
        </div>
    };
    let el2_str = DomDebugFragment::from_log().to_pseudo_html();

    assert_eq!(el1_str, el2_str);
}

#[test]
fn end_tag_without_start_is_ignored() {
    log_start();
    let _el1 = to_vertigo("First </span>paragraph\n\nSecond </div>paragraph");
    let el1_str = DomDebugFragment::from_log().to_pseudo_html();

    log_start();
    let _el2 = dom! {
        <div>
            <p>"First ""paragraph"</p>
            <p>"Second ""paragraph"</p>
        </div>
    };
    let el2_str = DomDebugFragment::from_log().to_pseudo_html();

    assert_eq!(el1_str, el2_str);
}

#[test]
fn unclosed_tag_closes_with_markdown_element() {
    log_start();
    let _el1 = to_vertigo("Some <span>unclosed *text*\n\nNext paragraph");
    let el1_str = DomDebugFragment::from_log().to_pseudo_html();

    log_start();
    let _el2 = dom! {
        <div>
            <p>"Some "<span>"unclosed "<em>"text"</em></span></p>
            <p>"Next paragraph"</p>
        </div>
    };
    let el2_str = DomDebugFragment::from_log().to_pseudo_html();

    assert_eq!(el1_str, el2_str);
}

#[test]
fn end_tag_does_not_close_markdown_element() {
    log_start();
    let _el1 = to_vertigo("<span>crossing *emphasis</span> end*");
    let el1_str = DomDebugFragment::from_log().to_pseudo_html();

    log_start();
    let _el2 = dom! {
        <div>
            <p><span>"crossing "<em>"emphasis"" end"</em></span></p>
        </div>
    };
    let el2_str = DomDebugFragment::from_log().to_pseudo_html();

    assert_eq!(el1_str, el2_str);
}

#[test]
fn unclosed_block_closes_with_document() {
    log_start();
    let _el1 = to_vertigo("<div title=\"box\">\n\nInside\n");
    let el1_str = DomDebugFragment::from_log().to_pseudo_html();

    log_start();
    let _el2 = dom! {
        <div>
            <div title="box">"\n"<p>"Inside"</p></div>
        </div>
    };
    let el2_str = DomDebugFragment::from_log().to_pseudo_html();

    assert_eq!(el1_str, el2_str);
}

#[test]
fn text_line_of_html_block() {
    log_start();
    let _el1 = to_vertigo("<div title=\"box\">\nText in block\n</div>");
    let el1_str = DomDebugFragment::from_log().to_pseudo_html();

    log_start();
    let _el2 = dom! {
        <div>
            <div title="box">"\n""Text in block\n"</div>
        </div>
    };
    let el2_str = DomDebugFragment::from_log().to_pseudo_html();

    assert_eq!(el1_str, el2_str);
}
