#![no_std]
extern crate alloc;

use alloc::{string::String, vec::Vec};
use wasm_bindgen::prelude::wasm_bindgen;
use wee_alloc::WeeAlloc;

mod macros;
mod parse_html;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    core::arch::wasm32::unreachable()
}

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Output {
    markdown: String,
    header_lines: Vec<usize>,
}

#[wasm_bindgen]
impl Output {
    #[wasm_bindgen(constructor)]
    pub fn new(markdown: String, header_lines: Vec<usize>) -> Output {
        Output {
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

#[wasm_bindgen]
pub fn test() -> Output {
    let html = r#"
        <h1>:P</h1>
        <h2>:P</h2>
        <p>First paragraph</p>
        <p><span class="person">Bob</span>: Second paragraph</p>

                                        <h1>:P</h1>
    "#;

    let (markdown, header_lines) = parse_html::parse(html);

    Output::new(markdown, header_lines)
}
