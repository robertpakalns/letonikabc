#![no_std]
extern crate alloc;

use alloc::{string::String, vec::Vec};

mod parse_html;
mod tests;

pub struct ParseOut {
    pub markdown: String,
    pub header_lines: Vec<usize>,
    pub hash: String,
}

pub fn parse(html: &str) -> ParseOut {
    let (markdown, header_lines, hash) = parse_html::parse(html);

    ParseOut {
        markdown,
        header_lines,
        hash,
    }
}
