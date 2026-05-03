#[cfg(test)]
mod tests {
    use crate::parse_html::{HtmlOut, parse};
    use alloc::vec;

    // PARAGRAPHS

    #[test]
    fn simple_paragraph() {
        let html = "<p>Hello World!</p>";
        let HtmlOut {
            markdown,
            heading_contents,
            ..
        } = parse(html);

        assert_eq!(markdown, "Hello World!");
        assert!(heading_contents.is_empty());
    }

    #[test]
    fn multiple_paragraphs() {
        let html = "<p>First paragraph.</p><p>Second paragraph.</p>";
        let HtmlOut {
            markdown,
            heading_lines,
            ..
        } = parse(html);

        assert_eq!(markdown, "First paragraph.\nSecond paragraph.");
        assert!(heading_lines.is_empty());
    }

    #[test]
    fn paragraph_with_extra_whitespace() {
        let html = "<p>   Text with  no     big  spaces   </p>";
        let HtmlOut {
            markdown,
            heading_lines,
            ..
        } = parse(html);

        assert_eq!(markdown, "Text with no big spaces");
        assert!(heading_lines.is_empty());
    }

    // HEADINGS

    #[test]
    fn headings() {
        let html = "<h1>Title</h1><h2>Subtitle</h2><p>Text</p>";
        let HtmlOut {
            markdown,
            heading_contents,
            heading_lines,
            heading_levels,
            ..
        } = parse(html);

        assert_eq!(markdown, "# Title\n## Subtitle\nText");
        assert_eq!(heading_contents, vec!["Title", "Subtitle"]);
        assert_eq!(heading_lines, vec![1, 2]);
        assert_eq!(heading_levels, vec![1, 2]);
    }

    #[test]
    fn more_headings() {
        let html = "<h1>H1</h1><h2>H2</h2><h3>H3</h3><h4>H4</h4><h5>H5</h5><h6>H6</h6>";
        let HtmlOut {
            markdown,
            heading_contents,
            heading_lines,
            heading_levels,
            ..
        } = parse(html);

        assert_eq!(
            markdown,
            "# H1\n## H2\n### H3\n#### H4\n##### H5\n###### H6"
        );
        assert_eq!(heading_contents, vec!["H1", "H2", "H3", "H4", "H5", "H6"]);
        assert_eq!(heading_lines, vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(heading_levels, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn nested_heading_and_paragraphs() {
        let html = "<h1>Main Title</h1><p>Intro text</p><h2>Subheading</h2><p>More text</p>";
        let HtmlOut {
            markdown,
            heading_contents,
            heading_lines,
            heading_levels,
            ..
        } = parse(html);

        assert_eq!(
            markdown,
            "# Main Title\nIntro text\n## Subheading\nMore text"
        );
        assert_eq!(heading_contents, vec!["Main Title", "Subheading"]);
        assert_eq!(heading_lines, vec![1, 3]);
        assert_eq!(heading_levels, vec![1, 2]);
    }

    #[test]
    fn consecutive_heading() {
        let html = "<h1>H1</h1><h2>H2</h2><h1>Another H1</h1>";
        let HtmlOut {
            markdown,
            heading_contents,
            heading_lines,
            heading_levels,
            ..
        } = parse(html);

        assert_eq!(markdown, "# H1\n## H2\n# Another H1");
        assert_eq!(heading_contents, vec!["H1", "H2", "Another H1"]);
        assert_eq!(heading_lines, vec![1, 2, 3]);
        assert_eq!(heading_levels, vec![1, 2, 1]);
    }

    // STYLING (_italics_)

    #[test]
    fn paragraph_with_italics() {
        let html = "<p>This is <i>italic</i> text and <i>more italics</i>.</p>";
        let HtmlOut {
            markdown,
            heading_lines,
            ..
        } = parse(html);

        assert_eq!(markdown, "This is _italic_ text and _more italics_.");
        assert!(heading_lines.is_empty());
    }

    // STYLING ([spans])

    #[test]
    fn paragraph_with_span_only() {
        let html = "<p><span>Important</span> text</p>";
        let HtmlOut {
            markdown,
            heading_lines,
            ..
        } = parse(html);

        assert_eq!(markdown, "[Important] text");
        assert!(heading_lines.is_empty());
    }

    #[test]
    fn nested_spans() {
        let html = "<p>Start <span>middle <span>inner</span></span> end</p>";
        let HtmlOut {
            markdown,
            heading_lines,
            ..
        } = parse(html);

        assert_eq!(markdown, "Start [middle [inner]] end");
        assert!(heading_lines.is_empty());
    }

    #[test]
    fn highlight_span() {
        let html = r#"<p><span class="highlight">text</span></p>"#;
        let HtmlOut {
            markdown,
            heading_lines,
            ..
        } = parse(html);

        assert_eq!(markdown, "text");
        assert!(heading_lines.is_empty());
    }

    // STYLING (@[person spans])

    #[test]
    fn person_span_simple() {
        let html = r#"<p><span class="person">TRAI</span>: Hello.</p>"#;
        let HtmlOut {
            markdown,
            heading_lines,
            ..
        } = parse(html);

        assert_eq!(markdown, "@[TRAI]: Hello.");
        assert!(heading_lines.is_empty());
    }

    #[test]
    fn multiple_person_spans() {
        let html = r#"
            <p><span class="person">PERSON1</span>: First line.</p>
            <p><span class="person">PERSON2</span>: Second line.</p>
        "#;
        let HtmlOut {
            markdown,
            heading_lines,
            ..
        } = parse(html);

        assert_eq!(
            markdown,
            "@[PERSON1]: First line.\n@[PERSON2]: Second line."
        );
        assert!(heading_lines.is_empty());
    }

    #[test]
    fn mixed_person_and_regular_span() {
        let html = r#"<p><span class="person">PERSON1</span> meets <span>someone</span>.</p>"#;
        let HtmlOut {
            markdown,
            heading_lines,
            ..
        } = parse(html);

        assert_eq!(markdown, "@[PERSON1] meets [someone].");
        assert!(heading_lines.is_empty());
    }

    #[test]
    fn nested_person_span() {
        let html = r#"<p><span class="person">PERSON1 <span>Inner</span></span></p>"#;
        let HtmlOut {
            markdown,
            heading_lines,
            ..
        } = parse(html);

        assert_eq!(markdown, "@[PERSON1 [Inner]]");
        assert!(heading_lines.is_empty());
    }

    // MISCELLANEOUS

    #[test]
    fn empty_html() {
        let html = "";
        let HtmlOut {
            markdown,
            heading_lines,
            ..
        } = parse(html);

        assert_eq!(markdown, "");
        assert!(heading_lines.is_empty());
    }

    #[test]
    fn empty_paragraph() {
        let html = "<p>   </p>";
        let HtmlOut {
            markdown,
            heading_lines,
            ..
        } = parse(html);

        assert_eq!(markdown, "");
        assert!(heading_lines.is_empty());
    }

    #[test]
    fn html_with_unsupported_tags() {
        let html = "<p>Hello <em>World</em>!</p>";
        let HtmlOut {
            markdown,
            heading_lines,
            ..
        } = parse(html);

        // Ignore the <em> tag because the function does not extract data from it
        assert_eq!(markdown, "Hello World!");
        assert!(heading_lines.is_empty());
    }

    #[test]
    fn span_and_br() {
        let html = "<p>Line 1<br>Line 2<span>inside span</span></p>";
        let HtmlOut {
            markdown,
            heading_lines,
            ..
        } = parse(html);

        assert_eq!(markdown, "Line 1\nLine 2[inside span]");
        assert!(heading_lines.is_empty());
    }

    #[test]
    fn multiple_line_breaks() {
        let html = "<p>Line 1<br><br>Line 2</p>";
        let HtmlOut {
            markdown,
            heading_lines,
            ..
        } = parse(html);

        assert_eq!(markdown, "Line 1\n\nLine 2");
        assert!(heading_lines.is_empty());
    }

    // SECURITY
    // JavaScript must not be present inside the markdown
    // Because HTMl is inserted directly into DOM
    // Otherwise readers are at risk of getting hacked

    #[test]
    fn js_injection_via_script() {
        let html = r#"<script>alert(1)</script>"#;
        let HtmlOut {
            markdown,
            heading_lines,
            ..
        } = parse(html);

        assert_eq!(markdown, "");
        assert!(heading_lines.is_empty());
    }

    #[test]
    fn js_injection_via_img_onerror() {
        let html = r#"<img src=x onerror=alert(1) />"#;
        let HtmlOut {
            markdown,
            heading_lines,
            ..
        } = parse(html);

        assert_eq!(markdown, "");
        assert!(heading_lines.is_empty());
    }

    #[test]
    fn js_injection_via_a_src() {
        let html = r#"<a href="javascript:alert(1)">Click me</a>"#;
        let HtmlOut {
            markdown,
            heading_lines,
            ..
        } = parse(html);

        assert_eq!(markdown, "");
        assert!(heading_lines.is_empty());
    }

    #[test]
    fn js_injection_via_svg_onload() {
        let html = r#"<svg onload=alert(1)></svg>"#;
        let HtmlOut {
            markdown,
            heading_lines,
            ..
        } = parse(html);

        assert_eq!(markdown, "");
        assert!(heading_lines.is_empty());
    }
}
