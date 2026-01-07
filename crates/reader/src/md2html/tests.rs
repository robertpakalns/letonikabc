#[cfg(test)]
mod tests {
    use super::super::convert;

    #[test]
    fn headers() {
        let md = "# H1\n## H2\n### H3\n#### H4\n##### H5\n###### H6";

        assert_eq!(
            convert(md),
            "<h1>H1</h1><h2>H2</h2><h3>H3</h3><h4>H4</h4><h5>H5</h5><h6>H6</h6>"
        );
    }

    #[test]
    fn mixed_content() {
        let md = "# H1\nNormal text\n## H2";

        assert_eq!(convert(md), "<h1>H1</h1><p>Normal text</p><h2>H2</h2>");
    }

    #[test]
    fn no_headers() {
        let md = "Just some text without headers.";

        assert_eq!(convert(md), "<p>Just some text without headers.</p>");
    }

    #[test]
    fn span_inside_paragraph() {
        let md = "This is a [span] inside a paragraph.";

        assert_eq!(
            convert(md),
            "<p>This is a <span>[span]</span> inside a paragraph.</p>"
        );
    }

    #[test]
    fn multiline_paragraph_with_span() {
        let md = "Line one with [span]\nLine two continues.";

        assert_eq!(
            convert(md),
            "<p>Line one with <span>[span]</span></p><p>Line two continues.</p>"
        );
    }

    #[test]
    fn multiple_spans() {
        let md = "Text with [span1] and [span2] in the same paragraph.";

        assert_eq!(
            convert(md),
            "<p>Text with <span>[span1]</span> and <span>[span2]</span> in the same paragraph.</p>"
        );
    }

    #[test]
    fn span_at_start_and_end() {
        let md = "[start] paragraph and ending with [end]";

        assert_eq!(
            convert(md),
            "<p><span>[start]</span> paragraph and ending with <span>[end]</span></p>"
        );
    }

    #[test]
    fn unclosed_bracket() {
        let md = "This is [not closed span";

        assert_eq!(convert(md), "<p>This is [not closed span</p>");
    }

    #[test]
    fn multiple_lines() {
        let md = "Line 1\nLine 2\nLine 3";

        assert_eq!(convert(md), "<p>Line 1</p><p>Line 2</p><p>Line 3</p>");
    }

    #[test]
    fn simple_italic() {
        let md = "This is _italic_ text.";
        let html = convert(md);

        let expected = "<p>This is <i>italic</i> text.</p>";
        assert_eq!(html, expected);
    }

    #[test]
    fn multiple_italics() {
        let md = "This _first_ and _second_ italics.";
        let html = convert(md);

        let expected = "<p>This <i>first</i> and <i>second</i> italics.</p>";
        assert_eq!(html, expected);
    }

    #[test]
    fn italics_with_span() {
        let md = "Text with _italic [span]_ example.";
        let html = convert(md);

        let expected = "<p>Text with <i>italic <span>[span]</span></i> example.</p>";
        assert_eq!(html, expected);
    }

    #[test]
    fn unmatched_underscore() {
        let md = "This is _italic without closing.";
        let html = convert(md);

        let expected = "<p>This is <i>italic without closing.</i></p>";
        assert_eq!(html, expected);
    }

    #[test]
    fn heading_with_italics() {
        let md = "# Heading with _italic_ text";
        let html = convert(md);

        let expected = "<h1>Heading with <i>italic</i> text</h1>";
        assert_eq!(html, expected);
    }

    #[test]
    fn empty_italics() {
        let md = "Empty _ _ italics";
        let html = convert(md);

        let expected = "<p>Empty <i> </i> italics</p>";
        assert_eq!(html, expected);
    }
}
