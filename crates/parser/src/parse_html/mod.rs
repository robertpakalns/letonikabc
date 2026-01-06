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

pub fn parse(html: &str) -> (String, Vec<usize>) {
    let mut chars = html.chars().peekable();
    let mut buffer = String::new();
    let mut lines = Vec::new();
    let mut header_lines = Vec::new();
    let mut element_state: Option<El> = None;
    let mut span_depth = 0;

    while let Some(&ch) = chars.peek() {
        if ch == '<' {
            chars.next();
            let mut tag_text = String::new();
            while let Some(&c) = chars.peek() {
                chars.next();
                if c == '>' {
                    break;
                }
                tag_text.push(c);
            }

            let tag = tag_text.trim();
            let is_end = tag.starts_with('/');
            let tag_name = tag
                .trim_start_matches('/')
                .split_whitespace()
                .next()
                .unwrap_or("")
                .to_lowercase();

            match (is_end, tag_name.as_str()) {
                (true, "p" | "h1" | "h2") => flush_buffer(
                    &mut element_state,
                    &mut buffer,
                    &mut lines,
                    &mut header_lines,
                ),
                (true, "span") => {
                    if span_depth > 0 {
                        span_depth -= 1;
                        buffer.push(']');
                    }
                }
                (false, "p") => {
                    flush_buffer(
                        &mut element_state,
                        &mut buffer,
                        &mut lines,
                        &mut header_lines,
                    );
                    element_state = Some(El::Paragraph);
                }
                (false, "h1") => {
                    flush_buffer(
                        &mut element_state,
                        &mut buffer,
                        &mut lines,
                        &mut header_lines,
                    );
                    element_state = Some(El::Heading(1));
                }
                (false, "h2") => {
                    flush_buffer(
                        &mut element_state,
                        &mut buffer,
                        &mut lines,
                        &mut header_lines,
                    );
                    element_state = Some(El::Heading(2));
                }
                (false, "br") => {
                    if let Some(El::Paragraph) = element_state {
                        let trimmed = buffer.trim();
                        lines.push(trimmed.to_string());
                        buffer.clear();
                    }
                }
                (false, "span") => {
                    if element_state.is_some() {
                        span_depth += 1;
                        buffer.push('[');
                    }
                }
                _ => {}
            }
        } else if element_state.is_some() {
            buffer.push(chars.next().unwrap());
        } else {
            chars.next();
        }
    }

    flush_buffer(
        &mut element_state,
        &mut buffer,
        &mut lines,
        &mut header_lines,
    );

    let markdown = lines.join("\n");
    (markdown, header_lines)
}
