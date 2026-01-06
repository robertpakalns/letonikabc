#![no_std]
extern crate alloc;

use alloc::{string::String, vec::Vec};

mod parse_html;

pub struct ParseOut {
    pub markdown: String,
    pub header_lines: Vec<usize>,
}

pub fn parse(html: &str) -> ParseOut {
    let (markdown, header_lines) = parse_html::parse(html);

    ParseOut {
        markdown,
        header_lines,
    }
}
