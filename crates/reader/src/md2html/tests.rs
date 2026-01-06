#[cfg(test)]
mod tests {
    use super::super::convert;

    #[test]
    fn headers() {
        let md = "# H1\n## H2\n### H3\n#### H4\n##### H5\n###### H6";
        let expected = "<h1>H1</h1><h2>H2</h2><h3>H3</h3><h4>H4</h4><h5>H5</h5><h6>H6</h6>";
        assert_eq!(convert(md), expected);
    }

    #[test]
    fn mixed_content() {
        let md = "# H1\nNormal text\n## H2";
        let expected = "<h1>H1</h1><p>Normal text</p><h2>H2</h2>";
        assert_eq!(convert(md), expected);
    }

    #[test]
    fn no_headers() {
        let md = "Just some text without headers.";
        let expected = "<p>Just some text without headers.</p>";
        assert_eq!(convert(md), expected);
    }

    #[test]
    fn span_inside_paragraph() {
        let md = "This is a [span] inside a paragraph.";
        let expected = "<p>This is a <span>[span]</span> inside a paragraph.</p>";
        assert_eq!(convert(md), expected);
    }

    #[test]
    fn multiline_paragraph_with_span() {
        let md = "Line one with [span]\nLine two continues.";
        let expected = "<p>Line one with <span>[span]</span></p><p>Line two continues.</p>";
        assert_eq!(convert(md), expected);
    }

    #[test]
    fn multiple_spans() {
        let md = "Text with [span1] and [span2] in the same paragraph.";
        let expected =
            "<p>Text with <span>[span1]</span> and <span>[span2]</span> in the same paragraph.</p>";
        assert_eq!(convert(md), expected);
    }

    #[test]
    fn span_at_start_and_end() {
        let md = "[start] paragraph and ending with [end]";
        let expected = "<p><span>[start]</span> paragraph and ending with <span>[end]</span></p>";
        assert_eq!(convert(md), expected);
    }

    #[test]
    fn unclosed_bracket() {
        let md = "This is [not closed span";
        let expected = "<p>This is [not closed span</p>";
        assert_eq!(convert(md), expected);
    }

    #[test]
    fn multiple_lines() {
        let md = "Line 1\nLine 2\nLine 3";
        let expected = "<p>Line 1</p><p>Line 2</p><p>Line 3</p>";
        assert_eq!(convert(md), expected);
    }
}
