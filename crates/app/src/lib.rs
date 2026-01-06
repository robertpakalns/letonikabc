#![no_std]
extern crate alloc;

use alloc::{string::String, vec::Vec};
use wasm_bindgen::prelude::wasm_bindgen;
use wee_alloc::WeeAlloc;

use parser::{ParseOut, parse};
use reader::convert;

/// The WASM is built without the Rust standart library, which significantly reduces the binary size
///
/// Never use dependencies which do not support `no_std` and never use `std` methods
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    core::arch::wasm32::unreachable()
}

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

// WASM bindings
#[wasm_bindgen]
pub fn parse_html_to_markdown(html: String) -> ParseOutput {
    ParseOutput {
        inner: parse(&html),
    }
}

#[wasm_bindgen]
pub fn convert_parsed_markdown_to_html(md: String) -> String {
    convert(md)
}

#[wasm_bindgen]
pub struct ParseOutput {
    inner: ParseOut,
}

#[wasm_bindgen]
impl ParseOutput {
    #[wasm_bindgen(getter)]
    pub fn markdown(&self) -> String {
        self.inner.markdown.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn header_lines(&self) -> Vec<usize> {
        self.inner.header_lines.clone()
    }
}
