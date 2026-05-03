#[cfg(test)]
mod tests {
    use crate::md2html::{MdOut, convert};
    use alloc::vec;

    // PARAGRAPHS

    #[test]
    fn no_headings() {
        let md = "Just some text without headings.";
        let expected = "<p>Just some text without headings.</p>";
        let MdOut {
            html,
            heading_contents,
            ..
        } = convert(md);

        assert_eq!(html, expected);
        assert!(heading_contents.is_empty())
    }

    #[test]
    fn multiple_lines() {
        let md = "Line 1\nLine 2\nLine 3";
        let expected = "<p>Line 1</p><p>Line 2</p><p>Line 3</p>";
        let MdOut {
            html,
            heading_contents,
            ..
        } = convert(md);

        assert_eq!(html, expected);
        assert!(heading_contents.is_empty())
    }

    // HEADINGS

    #[test]
    fn headings() {
        let md = "# H1\n## H2\n### H3\n#### H4\n##### H5\n###### H6";
        let expected = "<h1>H1</h1><h2>H2</h2><h3>H3</h3><h4>H4</h4><h5>H5</h5><h6>H6</h6>";
        let MdOut {
            html,
            heading_lines,
            heading_contents,
            heading_levels,
        } = convert(md);

        assert_eq!(html, expected);
        assert_eq!(heading_lines, vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(heading_contents, vec!["H1", "H2", "H3", "H4", "H5", "H6"]);
        assert_eq!(heading_levels, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn mixed_content() {
        let md = "# H1\nNormal text\n## H2";
        let expected = "<h1>H1</h1><p>Normal text</p><h2>H2</h2>";
        let MdOut {
            html,
            heading_lines,
            heading_contents,
            heading_levels,
        } = convert(md);

        assert_eq!(html, expected);
        assert_eq!(heading_lines, vec![1, 3]);
        assert_eq!(heading_contents, vec!["H1", "H2"]);
        assert_eq!(heading_levels, vec![1, 2]);
    }

    // STYLING ([spans])

    #[test]
    fn span_inside_paragraph() {
        let md = "This is a [span] inside a paragraph.";
        let expected = "<p>This is a <span>[span]</span> inside a paragraph.</p>";
        let MdOut {
            html,
            heading_contents,
            ..
        } = convert(md);

        assert_eq!(html, expected);
        assert!(heading_contents.is_empty())
    }

    #[test]
    fn multiline_paragraph_with_span() {
        let md = "Line one with [span]\nLine two continues.";
        let expected = "<p>Line one with <span>[span]</span></p><p>Line two continues.</p>";
        let MdOut {
            html,
            heading_contents,
            ..
        } = convert(md);

        assert_eq!(html, expected);
        assert!(heading_contents.is_empty())
    }

    #[test]
    fn multiple_spans() {
        let md = "Text with [span1] and [span2] in the same paragraph.";
        let expected =
            "<p>Text with <span>[span1]</span> and <span>[span2]</span> in the same paragraph.</p>";
        let MdOut {
            html,
            heading_contents,
            ..
        } = convert(md);

        assert_eq!(html, expected);
        assert!(heading_contents.is_empty())
    }

    #[test]
    fn span_at_start_and_end() {
        let md = "[start] paragraph and ending with [end]";
        let expected = "<p><span>[start]</span> paragraph and ending with <span>[end]</span></p>";
        let MdOut {
            html,
            heading_contents,
            ..
        } = convert(md);

        assert_eq!(html, expected);
        assert!(heading_contents.is_empty())
    }

    #[test]
    fn unclosed_bracket() {
        let md = "This is [not closed span";
        let expected = "<p>This is [not closed span</p>";
        let MdOut {
            html,
            heading_contents,
            ..
        } = convert(md);

        assert_eq!(html, expected);
        assert!(heading_contents.is_empty())
    }

    // STYLING (_italics_)

    #[test]
    fn simple_italic() {
        let md = "This is _italic_ text.";
        let expected = "<p>This is <i>italic</i> text.</p>";
        let MdOut {
            html,
            heading_contents,
            ..
        } = convert(md);

        assert_eq!(html, expected);
        assert!(heading_contents.is_empty())
    }

    #[test]
    fn multiple_italics() {
        let md = "This _first_ and _second_ italics.";
        let expected = "<p>This <i>first</i> and <i>second</i> italics.</p>";
        let MdOut {
            html,
            heading_contents,
            ..
        } = convert(md);

        assert_eq!(html, expected);
        assert!(heading_contents.is_empty())
    }

    #[test]
    fn italics_with_span() {
        let md = "Text with _italic [span]_ example.";
        let expected = "<p>Text with <i>italic <span>[span]</span></i> example.</p>";
        let MdOut {
            html,
            heading_contents,
            ..
        } = convert(md);

        assert_eq!(html, expected);
        assert!(heading_contents.is_empty())
    }

    #[test]
    fn unmatched_underscore() {
        let md = "This is _italic without closing.";
        let expected = "<p>This is _italic without closing.</p>";
        let MdOut {
            html,
            heading_contents,
            ..
        } = convert(md);

        assert_eq!(html, expected);
        assert!(heading_contents.is_empty())
    }

    #[test]
    fn heading_with_italics() {
        let md = "# Heading with _italic_ text";
        let expected = "<h1>Heading with <i>italic</i> text</h1>";
        let MdOut {
            html,
            heading_contents,
            ..
        } = convert(md);

        assert_eq!(html, expected);
        assert_eq!(heading_contents, vec!["Heading with <i>italic</i> text"])
    }

    #[test]
    fn empty_italics() {
        let md = "Empty _ _ italics";
        let expected = "<p>Empty italics</p>";
        let MdOut {
            html,
            heading_contents,
            ..
        } = convert(md);

        assert_eq!(html, expected);
        assert!(heading_contents.is_empty())
    }

    // STYLING (@[person spans])

    #[test]
    fn person_span_simple() {
        let md = "@[PERSON1]: Hello.";
        let expected = "<p><span data-p>PERSON1</span>: Hello.</p>";
        let MdOut {
            html,
            heading_contents,
            ..
        } = convert(md);

        assert_eq!(html, expected);
        assert!(heading_contents.is_empty())
    }

    #[test]
    fn multiple_person_spans() {
        let md = "@[PERSON1]: First.\n@[PERSON2]: Second.";
        let expected =
            "<p><span data-p>PERSON1</span>: First.</p><p><span data-p>PERSON2</span>: Second.</p>";
        let MdOut {
            html,
            heading_contents,
            ..
        } = convert(md);

        assert_eq!(html, expected);
        assert!(heading_contents.is_empty())
    }

    #[test]
    fn person_and_regular_span() {
        let md = "@[PERSON1] meets [someone].";
        let expected = "<p><span data-p>PERSON1</span> meets <span>[someone]</span>.</p>";
        let MdOut {
            html,
            heading_contents,
            ..
        } = convert(md);

        assert_eq!(html, expected);
        assert!(heading_contents.is_empty())
    }

    #[test]
    fn person_span_with_italics() {
        let md = "_@[PERSON1]_ speaks.";
        let expected = "<p><i><span data-p>PERSON1</span></i> speaks.</p>";
        let MdOut {
            html,
            heading_contents,
            ..
        } = convert(md);

        assert_eq!(html, expected);
        assert!(heading_contents.is_empty())
    }
}
