use pulldown_cmark::Parser;
use vertigo::{
    DomNode,
    dev::inspect::{DomDebugFragment, log_start},
    dom,
};

use crate::{CMarkStyle, HtmlPolicy, TrustedHtml, events_to_vertigo, is_safe_url, to_vertigo};

fn rendered(text: &str) -> String {
    log_start();
    let _node = to_vertigo(text);
    DomDebugFragment::from_log().to_pseudo_html()
}

fn rendered_with(text: &str, policy: &dyn HtmlPolicy) -> String {
    log_start();
    let _node = events_to_vertigo(Parser::new(text), CMarkStyle::default(), policy);
    DomDebugFragment::from_log().to_pseudo_html()
}

fn expected(build: impl FnOnce() -> DomNode) -> String {
    log_start();
    let _node = build();
    DomDebugFragment::from_log().to_pseudo_html()
}

#[test]
fn safe_urls() {
    for url in [
        "https://example.com/",
        "/page",
        "#note",
        "image.png",
        "mailto:me@example.com",
        "?q=javascript:alert(1)",
    ] {
        assert!(is_safe_url(url), "{url:?}");
    }
    for url in [
        "javascript:alert(1)",
        "JavaScript:alert(1)",
        " javascript:alert(1)",
        "java\tscript:alert(1)",
        "java\nscript:alert(1)",
        "\u{1}javascript:alert(1)",
        "vbscript:msgbox(1)",
        "data:text/html,<script>alert(1)</script>",
        "file:///etc/passwd",
    ] {
        assert!(!is_safe_url(url), "{url:?}");
    }
}

#[test]
fn script_links_become_text() {
    assert_eq!(
        rendered("[click](javascript:alert(1)) and <javascript:alert(2)> and [ok](/page)"),
        expected(|| dom! {
            <div>
                <p>
                    <span>"click"</span>
                    " and "
                    <span>"javascript:alert(2)"</span>
                    " and "
                    <a href="/page">"ok"</a>
                </p>
            </div>
        })
    );
}

#[test]
fn email_autolink_stays() {
    assert_eq!(
        rendered("<me@example.com>"),
        expected(|| dom! {
            <div>
                <p><a href="mailto:me@example.com">"me@example.com"</a></p>
            </div>
        })
    );
}

#[test]
fn unsafe_image_becomes_its_description() {
    assert_eq!(
        rendered("![a cat](javascript:alert(1)) ![a dog](https://example.com/dog.png)"),
        expected(|| dom! {
            <div>
                <p>"a cat"" "<img src="https://example.com/dog.png" alt="a dog" /></p>
            </div>
        })
    );
}

#[test]
fn trusted_html_keeps_every_link() {
    assert_eq!(
        rendered_with("[y](javascript:alert(2))", &TrustedHtml),
        expected(|| dom! {
            <div>
                <p><a href="javascript:alert(2)">"y"</a></p>
            </div>
        })
    );
}

#[cfg(feature = "html")]
mod raw_html {
    use std::borrow::Cow;

    use super::*;
    use crate::{ElementAction, SafeHtml};

    #[test]
    fn trusted_html_renders_everything() {
        assert_eq!(
            rendered_with(
                r#"<b onclick="alert(1)" style="color: red">x</b>"#,
                &TrustedHtml
            ),
            expected(|| dom! {
                <div>
                    <p><b onclick="alert(1)" style="color: red">"x"</b></p>
                </div>
            })
        );
    }

    #[test]
    fn script_goes_away_with_content() {
        assert_eq!(
            rendered("Before\n\n<script>alert(1)</script>\n\nAfter <script>alert(2)</script> end"),
            expected(|| dom! {
                <div>
                    <p>"Before"</p>
                    "\n"
                    <p>"After "" end"</p>
                </div>
            })
        );
    }

    #[test]
    fn style_goes_away_with_content() {
        assert_eq!(
            rendered("Text <style>body { display: none }</style> after"),
            expected(|| dom! {
                <div>
                    <p>"Text "" after"</p>
                </div>
            })
        );
    }

    #[test]
    fn unclosed_script_hides_the_rest() {
        assert_eq!(
            rendered("Text <script>alert(1)\n\nMore text"),
            expected(|| dom! {
                <div>
                    <p>"Text "</p>
                    <p></p>
                </div>
            })
        );
    }

    #[test]
    fn event_handlers_and_styles_are_dropped() {
        assert_eq!(
            rendered(
                r#"<img src="https://example.com/a.png" onerror="alert(1)" style="position: fixed" alt="a"> <b onclick="alert(2)" class="x" title="t">bold</b>"#
            ),
            expected(|| dom! {
                <div>
                    <p>
                        <img alt="a" src="https://example.com/a.png" />
                        " "
                        <b title="t">"bold"</b>
                    </p>
                </div>
            })
        );
    }

    #[test]
    fn script_addresses_are_dropped() {
        assert_eq!(
            rendered(
                r#"<a href="javascript:alert(1)">one</a> <a href="&#106;avascript:alert(2)">two</a> <a href=" JAVASCRIPT:alert(3)">three</a> <img src="javascript:alert(4)" alt="four"> <a href="https://example.com/">five</a>"#
            ),
            expected(|| dom! {
                <div>
                    <p>
                        <a>"one"</a>
                        " "
                        <a>"two"</a>
                        " "
                        <a>"three"</a>
                        " "
                        <img alt="four" />
                        " "
                        <a href="https://example.com/">"five"</a>
                    </p>
                </div>
            })
        );
    }

    #[test]
    fn frames_forms_and_svg_lose_their_tags() {
        assert_eq!(
            rendered(
                r#"<iframe srcdoc="<script>alert(1)</script>" src="https://example.com/"></iframe><form action="https://example.com/"><input name="password"><button>Log in</button></form><svg><script>alert(2)</script><text>svg text</text></svg><object data="x.swf"></object><base href="https://evil.example/">"#
            ),
            expected(|| dom! {
                <div>"Log in""svg text"</div>
            })
        );
    }

    /// Removes `<div>` with everything inside.
    struct NoDivs;

    impl HtmlPolicy for NoDivs {
        fn element(&self, name: &str) -> ElementAction {
            match name {
                "div" => ElementAction::Remove,
                name => SafeHtml.element(name),
            }
        }

        fn attribute<'v>(&self, element: &str, name: &str, value: &'v str) -> Option<Cow<'v, str>> {
            SafeHtml.attribute(element, name, value)
        }

        fn destination<'u>(&self, url: &'u str, image: bool) -> Option<Cow<'u, str>> {
            SafeHtml.destination(url, image)
        }
    }

    #[test]
    fn removed_element_takes_nested_ones_with_it() {
        assert_eq!(
            rendered_with("<div>a<div>b</div>c</div>d", &NoDivs),
            expected(|| dom! { <div>"d"</div> })
        );
    }

    /// Safe HTML plus YouTube players.
    struct WithVideos;

    impl HtmlPolicy for WithVideos {
        fn element(&self, name: &str) -> ElementAction {
            match name {
                "iframe" => ElementAction::Keep,
                name => SafeHtml.element(name),
            }
        }

        fn attribute<'v>(&self, element: &str, name: &str, value: &'v str) -> Option<Cow<'v, str>> {
            match (element, name) {
                ("iframe", "src") => value
                    .starts_with("https://www.youtube.com/embed/")
                    .then_some(Cow::Borrowed(value)),
                ("iframe", "allowfullscreen") => Some(Cow::Borrowed(value)),
                _ => SafeHtml.attribute(element, name, value),
            }
        }

        fn destination<'u>(&self, url: &'u str, image: bool) -> Option<Cow<'u, str>> {
            SafeHtml.destination(url, image)
        }
    }

    #[test]
    fn own_policy_lets_in_more() {
        assert_eq!(
            rendered_with(
                r#"<iframe src="https://www.youtube.com/embed/abc" allowfullscreen onload="alert(1)"></iframe>"#,
                &WithVideos
            ),
            expected(|| dom! {
                <div>
                    <iframe allowfullscreen="" src="https://www.youtube.com/embed/abc"></iframe>
                </div>
            })
        );
        // the rest stays as in `SafeHtml`
        assert_eq!(
            rendered_with(
                r#"<iframe src="https://evil.example/"></iframe>"#,
                &WithVideos
            ),
            expected(|| dom! { <div><iframe></iframe></div> })
        );
    }
}
