#[cfg(test)]
mod tests {
    use super::super::parse;

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
        assert_eq!(headers, vec![1, 2]); // Two headers
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
}
