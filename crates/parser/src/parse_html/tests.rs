#[cfg(test)]
mod tests {
    use super::super::parse;
    use alloc::vec;

    #[test]
    fn simple_paragraph() {
        let html = "<p>Hello World!</p>";
        let (md, headers) = parse(html);

        assert_eq!(md.trim(), "Hello World!");
        assert!(headers.is_empty());
    }

    #[test]
    fn multiple_paragraphs() {
        let html = "<p>First paragraph.</p><p>Second paragraph.</p>";
        let (md, headers) = parse(html);

        let expected = "First paragraph.\nSecond paragraph.";
        assert_eq!(md.trim(), expected);
        assert!(headers.is_empty());
    }

    #[test]
    fn headers() {
        let html = "<h1>Title</h1><h2>Subtitle</h2><p>Text</p>";
        let (md, headers) = parse(html);

        let expected_md = "# Title\n## Subtitle\nText";
        assert_eq!(md.trim(), expected_md);
        assert_eq!(headers, vec![1, 2]);
    }

    #[test]
    fn span_and_br() {
        let html = "<p>Line 1<br>Line 2<span>inside span</span></p>";
        let (md, headers) = parse(html);

        let expected_md = "Line 1\nLine 2[inside span]";
        assert_eq!(md.trim(), expected_md);
        assert!(headers.is_empty());
    }

    #[test]
    fn empty_html() {
        let html = "";
        let (md, headers) = parse(html);

        assert_eq!(md, "");
        assert!(headers.is_empty());
    }

    #[test]
    fn nested_headers_and_paragraphs() {
        let html = "<h1>Main Title</h1><p>Intro text</p><h2>Subheading</h2><p>More text</p>";
        let (md, headers) = parse(html);

        let expected_md = "# Main Title\nIntro text\n## Subheading\nMore text";
        assert_eq!(md.trim(), expected_md);
        assert_eq!(headers, vec![1, 3]);
    }

    #[test]
    fn multiple_line_breaks() {
        let html = "<p>Line 1<br><br>Line 2</p>";
        let (md, headers) = parse(html);

        let expected_md = "Line 1\n\nLine 2";
        assert_eq!(md.trim(), expected_md);
        assert!(headers.is_empty());
    }

    #[test]
    fn paragraph_with_span_only() {
        let html = "<p><span>Important</span> text</p>";
        let (md, headers) = parse(html);

        let expected_md = "[Important] text";
        assert_eq!(md.trim(), expected_md);
        assert!(headers.is_empty());
    }

    #[test]
    fn consecutive_headers() {
        let html = "<h1>H1</h1><h2>H2</h2><h1>Another H1</h1>";
        let (md, headers) = parse(html);

        let expected_md = "# H1\n## H2\n# Another H1";
        assert_eq!(md.trim(), expected_md);
        assert_eq!(headers, vec![1, 2, 3]);
    }

    #[test]
    fn paragraph_with_extra_whitespace() {
        let html = "<p>   Text with    spaces   </p>";
        let (md, headers) = parse(html);

        let expected_md = "Text with    spaces";
        assert_eq!(md.trim(), expected_md);
        assert!(headers.is_empty());
    }

    #[test]
    fn html_with_unsupported_tags() {
        let html = "<p>Hello <em>World</em>!</p>";
        let (md, headers) = parse(html);

        // Ignore <em> and leave text
        let expected_md = "Hello World!";
        assert_eq!(md.trim(), expected_md);
        assert!(headers.is_empty());
    }

    #[test]
    fn empty_paragraph() {
        let html = "<p>   </p>";
        let (md, headers) = parse(html);

        assert_eq!(md.trim(), "");
        assert!(headers.is_empty());
    }

    #[test]
    fn nested_spans() {
        let html = "<p>Start <span>middle <span>inner</span></span> end</p>";
        let (md, headers) = parse(html);

        let expected_md = "Start [middle [inner]] end";
        assert_eq!(md.trim(), expected_md);
        assert!(headers.is_empty());
    }
}
