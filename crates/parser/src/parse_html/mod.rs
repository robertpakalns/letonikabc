use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};

use utils::El;

mod tests;

fn flush_buffer(
    element_state: &mut Option<El>,
    buffer: &mut String,
    lines: &mut Vec<String>,
    header_lines: &mut Vec<usize>,
) {
    if let Some(el) = element_state.take() {
        let text = buffer.trim();
        if !text.is_empty() {
            match el {
                El::Paragraph => lines.push(text.to_string()),
                El::Heading(level) => {
                    lines.push(format!("{} {}", "#".repeat(level), text));
                    header_lines.push(lines.len());
                }
            }
        }
        buffer.clear();
    }
}

fn start_element(
    state: &mut Option<El>,
    buffer: &mut String,
    lines: &mut Vec<String>,
    headers: &mut Vec<usize>,
    el: El,
) {
    flush_buffer(state, buffer, lines, headers);
    *state = Some(el);
}

pub fn parse(html: &str) -> (String, Vec<usize>, String) {
    let mut chars = html.chars().peekable();

    let mut buffer = String::new();
    let mut lines = Vec::new();
    let mut header_lines = Vec::new();

    let mut element_state: Option<El> = None;
    let mut span_depth = 0;
    let mut italic_depth = 0;

    while let Some(&ch) = chars.peek() {
        if ch != '<' {
            chars.next();
            if element_state.is_some() {
                if ch.is_whitespace() {
                    if !buffer.ends_with(' ') {
                        buffer.push(' ');
                    }
                } else {
                    buffer.push(ch);
                }
            }
            continue;
        }

        // Parse tag
        chars.next(); // <
        let mut tag = String::new();
        while let Some(c) = chars.next() {
            if c == '>' {
                break;
            }
            tag.push(c);
        }

        let tag = tag.trim();
        let end = tag.starts_with('/');
        let name = tag
            .trim_start_matches('/')
            .split_whitespace()
            .next()
            .unwrap_or("")
            .to_lowercase();

        match (end, name.as_str()) {
            (true, "p" | "h1" | "h2") => flush_buffer(
                &mut element_state,
                &mut buffer,
                &mut lines,
                &mut header_lines,
            ),
            (true, "span") if span_depth > 0 => {
                span_depth -= 1;
                buffer.push(']');
            }
            (true, "i") if italic_depth > 0 => {
                italic_depth -= 1;
                buffer.push('_');
            }

            (false, "p") => start_element(
                &mut element_state,
                &mut buffer,
                &mut lines,
                &mut header_lines,
                El::Paragraph,
            ),
            (false, "h1") => start_element(
                &mut element_state,
                &mut buffer,
                &mut lines,
                &mut header_lines,
                El::Heading(1),
            ),
            (false, "h2") => start_element(
                &mut element_state,
                &mut buffer,
                &mut lines,
                &mut header_lines,
                El::Heading(2),
            ),
            (false, "br") => {
                if let Some(El::Paragraph) = element_state {
                    lines.push(buffer.trim().to_string());
                    buffer.clear();
                }
            }
            (false, "span") if element_state.is_some() => {
                span_depth += 1;
                if tag.contains("class=\"person\"") {
                    buffer.push_str("@[");
                } else {
                    buffer.push('[');
                }
            }
            (false, "i") if element_state.is_some() => {
                italic_depth += 1;
                buffer.push('_');
            }

            // Unsupported tag = ignore
            _ => {}
        }
    }

    flush_buffer(
        &mut element_state,
        &mut buffer,
        &mut lines,
        &mut header_lines,
    );

    let md = lines.join("\n");
    let hash = create_hash(&md);

    (md, header_lines, hash)
}

pub fn create_hash(md: &str) -> String {
    blake3::hash(md.as_bytes()).to_hex().to_string()
}
