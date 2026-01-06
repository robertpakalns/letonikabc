use alloc::{format, string::String, vec::Vec};

use utils::El;

mod tests;

fn wrap_spans(text: &str) -> String {
    let mut result = String::new();
    let mut chars = text.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '[' {
            let mut span_content = String::new();
            let mut found_close = false;

            while let Some(&next_c) = chars.peek() {
                chars.next();
                if next_c == ']' {
                    found_close = true;
                    break;
                } else {
                    span_content.push(next_c);
                }
            }

            if found_close {
                result.push_str(&format!("<span>[{}]</span>", span_content));
            } else {
                result.push('[');
                result.push_str(&span_content);
            }
        } else {
            result.push(c);
        }
    }

    result
}

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
                El::Paragraph => {
                    // \n -> <br>
                    let text_with_br = text.replace('\n', "<br>");
                    // Wrap [content] in <span>
                    let text_with_span = wrap_spans(&text_with_br);
                    lines.push(format!("<p>{}</p>", text_with_span));
                }
                El::Heading(level) => {
                    lines.push(format!("<h{level}>{}</h{level}>", text));
                    header_lines.push(lines.len() - 1);
                }
            }
        }
        buffer.clear();
    }
}

pub fn convert(md: &str) -> String {
    let mut html = String::new();
    let mut element_state: Option<El> = None;
    let mut buffer = String::new();
    let mut lines = Vec::new();
    let mut header_lines = Vec::new();

    for line in md.lines() {
        let trimmed = line.trim_start();

        // Count #
        let mut count = 0;
        for c in trimmed.chars() {
            if c == '#' {
                count += 1;
            } else {
                break;
            }
        }

        if count > 0 && trimmed.get(count..count + 1) == Some(" ") {
            flush_buffer(
                &mut element_state,
                &mut buffer,
                &mut lines,
                &mut header_lines,
            );
            let level = if count > 6 { 6 } else { count };
            element_state = Some(El::Heading(level));
            buffer.push_str(&trimmed[count + 1..]);
        } else {
            if element_state.is_none() {
                element_state = Some(El::Paragraph);
            } else if matches!(element_state, Some(El::Heading(_))) {
                flush_buffer(
                    &mut element_state,
                    &mut buffer,
                    &mut lines,
                    &mut header_lines,
                );
                element_state = Some(El::Paragraph);
            }
            if !buffer.is_empty() {
                buffer.push('\n');
            }
            buffer.push_str(trimmed);
        }
    }

    flush_buffer(
        &mut element_state,
        &mut buffer,
        &mut lines,
        &mut header_lines,
    );

    for line in lines {
        html.push_str(&line);
    }

    html
}
