use syntect::easy::HighlightLines;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{Color, FontStyle, Style, ThemeSet};
use syntect::util::LinesWithEndings;
use vertigo::DomElement;

pub fn highlight(info: &str, s: &str) -> Vec<DomElement> {
    // Load these once at the start of your program
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let syntax = ps.find_syntax_by_token(info).unwrap_or_else(|| ps.find_syntax_plain_text());
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-eighties.dark"]);

    let mut output = vec![];
    for line in LinesWithEndings::from(s) { // LinesWithEndings enables use of newlines mode
        let ranges: Vec<(Style, &str)> = h.highlight_line(line, &ps).unwrap();
        let line = generate_line(&ranges);
        output.extend(line);
    }
    output
}

pub fn generate_line(inputs: &[(Style, &str)]) -> Vec<DomElement> {
    let mut spans = vec![];
    for (style, txt) in inputs {
        spans.push(generate_span(style, txt));
    }
    spans
}

pub fn generate_span(style: &Style, txt: &str) -> DomElement {
    let color = color_to_css(&style.foreground);
    let background_color = color_to_css(&style.background);
    let mut style_value = format!("color: {color}; background_color: {background_color}");

    if !style.font_style.is_empty() {
        if style.font_style.contains(FontStyle::BOLD) {
            style_value.push_str(" font-weight: bold;");
        }
        if style.font_style.contains(FontStyle::ITALIC) {
            style_value.push_str(" font-style: italic;");
        }
        if style.font_style.contains(FontStyle::UNDERLINE) {
            style_value.push_str(" text-decoration: underline;");
        }
    }

    DomElement::new("span")
        .attr("style", style_value)
        .child_text(txt)
}

pub fn color_to_css(color: &Color) -> String {
    let Color { r, g, b, a } = color;
    format!("rgba({r}, {g}, {b}, {a})")
}
