#[cfg(test)]
mod tests {
    use super::super::parse;
    use alloc::vec;

    #[test]
    fn simple_paragraph() {
        let html = "<p>Hello World!</p>";
        let (md, headers) = parse(html);

        assert_eq!(md, "Hello World!");
        assert!(headers.is_empty());
    }

    #[test]
    fn multiple_paragraphs() {
        let html = "<p>First paragraph.</p><p>Second paragraph.</p>";
        let (md, headers) = parse(html);

        assert_eq!(md, "First paragraph.\nSecond paragraph.");
        assert!(headers.is_empty());
    }

    #[test]
    fn headers() {
        let html = "<h1>Title</h1><h2>Subtitle</h2><p>Text</p>";
        let (md, headers) = parse(html);

        assert_eq!(md, "# Title\n## Subtitle\nText");
        assert_eq!(headers, vec![1, 2]);
    }

    #[test]
    fn span_and_br() {
        let html = "<p>Line 1<br>Line 2<span>inside span</span></p>";
        let (md, headers) = parse(html);

        assert_eq!(md, "Line 1\nLine 2[inside span]");
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

        assert_eq!(md, "# Main Title\nIntro text\n## Subheading\nMore text");
        assert_eq!(headers, vec![1, 3]);
    }

    #[test]
    fn multiple_line_breaks() {
        let html = "<p>Line 1<br><br>Line 2</p>";
        let (md, headers) = parse(html);

        assert_eq!(md, "Line 1\n\nLine 2");
        assert!(headers.is_empty());
    }

    #[test]
    fn paragraph_with_span_only() {
        let html = "<p><span>Important</span> text</p>";
        let (md, headers) = parse(html);

        assert_eq!(md, "[Important] text");
        assert!(headers.is_empty());
    }

    #[test]
    fn consecutive_headers() {
        let html = "<h1>H1</h1><h2>H2</h2><h1>Another H1</h1>";
        let (md, headers) = parse(html);

        assert_eq!(md, "# H1\n## H2\n# Another H1");
        assert_eq!(headers, vec![1, 2, 3]);
    }

    #[test]
    fn paragraph_with_extra_whitespace() {
        let html = "<p>   Text with  no     big  spaces   </p>";
        let (md, headers) = parse(html);

        assert_eq!(md, "Text with no big spaces");
        assert!(headers.is_empty());
    }

    #[test]
    fn html_with_unsupported_tags() {
        let html = "<p>Hello <em>World</em>!</p>";
        let (md, headers) = parse(html);

        // Ignore the tag
        assert_eq!(md, "Hello World!");
        assert!(headers.is_empty());
    }

    #[test]
    fn empty_paragraph() {
        let html = "<p>   </p>";
        let (md, headers) = parse(html);

        assert_eq!(md, "");
        assert!(headers.is_empty());
    }

    #[test]
    fn nested_spans() {
        let html = "<p>Start <span>middle <span>inner</span></span> end</p>";
        let (md, headers) = parse(html);

        assert_eq!(md, "Start [middle [inner]] end");
        assert!(headers.is_empty());
    }

    #[test]
    fn paragraph_with_italics() {
        let html = "<p>This is <i>italic</i> text and <i>more italics</i>.</p>";
        let (md, headers) = parse(html);

        assert_eq!(md, "This is _italic_ text and _more italics_.");
        assert!(headers.is_empty());
    }

    #[test]
    fn person_span_simple() {
        let html = r#"<p><span class="person">TRAI</span>: Hello.</p>"#;
        let (md, headers) = parse(html);

        assert_eq!(md.trim(), "@[TRAI]: Hello.");
        assert!(headers.is_empty());
    }

    #[test]
    fn multiple_person_spans() {
        let html = r#"
            <p><span class="person">PERSON1</span>: First line.</p>
            <p><span class="person">PERSON2</span>: Second line.</p>
        "#;

        let (md, headers) = parse(html);

        assert_eq!(
            md.trim(),
            "@[PERSON1]: First line.\n@[PERSON2]: Second line."
        );
        assert!(headers.is_empty());
    }

    #[test]
    fn mixed_person_and_regular_span() {
        let html = r#"<p><span class="person">PERSON1</span> meets <span>someone</span>.</p>"#;

        let (md, headers) = parse(html);
        assert_eq!(md.trim(), "@[PERSON1] meets [someone].");
        assert!(headers.is_empty());
    }

    #[test]
    fn nested_person_span() {
        let html = r#"<p><span class="person">PERSON1 <span>Inner</span></span></p>"#;

        let (md, headers) = parse(html);
        assert_eq!(md.trim(), "@[PERSON1 [Inner]]");
        assert!(headers.is_empty());
    }
}
