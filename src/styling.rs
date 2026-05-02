use vertigo::Css;

#[derive(Clone)]
pub struct CMarkStyle {
    pub blockquote: Css,
    pub codeblock: Css,
    pub container: Css,
    pub dd: Css,
    pub del: Css,
    pub dl: Css,
    pub dt: Css,
    pub em: Css,
    pub h1: Css,
    pub h2: Css,
    pub h3: Css,
    pub h4: Css,
    pub h5: Css,
    pub h6: Css,
    pub img: Css,
    pub li: Css,
    pub a: Css,
    pub ol: Css,
    pub p: Css,
    pub strong: Css,
    pub sub: Css,
    pub sup: Css,
    pub table: Css,
    pub tbody: Css,
    pub td: Css,
    pub th: Css,
    pub thead: Css,
    pub tr: Css,
    pub ul: Css,
}

impl Default for CMarkStyle {
    fn default() -> Self {
        Self {
            blockquote: Css::default(),
            codeblock: Css::default(),
            container: Css::default(),
            dd: Css::default(),
            del: Css::default(),
            dl: Css::default(),
            dt: Css::default(),
            em: Css::default(),
            h1: Css::default(),
            h2: Css::default(),
            h3: Css::default(),
            h4: Css::default(),
            h5: Css::default(),
            h6: Css::default(),
            img: Css::default(),
            li: Css::default(),
            a: Css::default(),
            ol: Css::default(),
            p: Css::default(),
            strong: Css::default(),
            sub: Css::default(),
            sup: Css::default(),
            table: Css::str("border: 1"),
            tbody: Css::default(),
            td: Css::default(),
            th: Css::default(),
            thead: Css::default(),
            tr: Css::default(),
            ul: Css::default(),
        }
    }
}

impl CMarkStyle {
    /// Returns a style preset that restores browser-default typography,
    /// suitable for use when Tailwind preflight has reset all element styles.
    pub fn browser_defaults() -> Self {
        Self {
            p: Css::str("margin: 0.75em 0; line-height: 1.6;"),
            h1: Css::str("font-size: 2em;     font-weight: 700; margin: 0.67em 0;"),
            h2: Css::str("font-size: 1.5em;   font-weight: 700; margin: 0.75em 0;"),
            h3: Css::str("font-size: 1.25em;  font-weight: 600; margin: 0.83em 0;"),
            h4: Css::str("font-size: 1.1em;   font-weight: 600; margin: 0.83em 0;"),
            h5: Css::str("font-size: 1em;     font-weight: 600; margin: 0.83em 0;"),
            h6: Css::str("font-size: 0.875em; font-weight: 600; margin: 0.83em 0; color: #6b7280;"),
            ul: Css::str("list-style-type: disc;    padding-left: 1.5em; margin: 0.5em 0;"),
            ol: Css::str("list-style-type: decimal; padding-left: 1.5em; margin: 0.5em 0;"),
            li: Css::str("margin: 0.25em 0;"),
            blockquote: Css::str(
                "border-left: 4px solid #d1d5db; padding-left: 1em; margin: 0.75em 0; color: #6b7280; font-style: italic;",
            ),
            codeblock: Css::str(
                "background: #f3f4f6; border-radius: 0.375em; padding: 0.75em 1em; overflow-x: auto; font-family: monospace; font-size: 0.875em; margin: 0.75em 0; display: block;",
            ),
            a: Css::str("color: #4f46e5; text-decoration: underline;"),
            strong: Css::str("font-weight: 700;"),
            em: Css::str("font-style: italic;"),
            table: Css::str("border-collapse: collapse; width: 100%; margin: 0.75em 0; border: 1"),
            th: Css::str(
                "border: 1px solid #d1d5db; padding: 0.5em 0.75em; background: #f9fafb; font-weight: 600; text-align: left;",
            ),
            td: Css::str("border: 1px solid #d1d5db; padding: 0.5em 0.75em;"),
            ..Self::default()
        }
    }
}
