#![no_std]
extern crate alloc;

use alloc::{string::String, vec::Vec};
use wasm_bindgen::prelude::wasm_bindgen;

mod macros;
mod parse_html;

#[wasm_bindgen]
pub struct ParseOutput {
    markdown: String,
    header_lines: Vec<usize>,
}

#[wasm_bindgen]
impl ParseOutput {
    #[wasm_bindgen(constructor)]
    pub fn new(markdown: String, header_lines: Vec<usize>) -> ParseOutput {
        ParseOutput {
            markdown,
            header_lines,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn markdown(&self) -> String {
        self.markdown.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn header_lines(&self) -> Vec<usize> {
        self.header_lines.clone()
    }
}

pub fn parse(html: String) -> ParseOutput {
    let (markdown, header_lines) = parse_html::parse(&html);

    ParseOutput::new(markdown, header_lines)
}
