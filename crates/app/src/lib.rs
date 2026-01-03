#![no_std]
extern crate alloc;

use alloc::string::String;
use wasm_bindgen::prelude::wasm_bindgen;
use wee_alloc::WeeAlloc;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    core::arch::wasm32::unreachable()
}

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

use parser::{ParseOutput, parse};
use reader::convert;

#[wasm_bindgen]
pub fn parse_html_to_markdown(html: String) -> ParseOutput {
    parse(html)
}

#[wasm_bindgen]
pub fn convert_parsed_markdown_to_html(md: String) -> String {
    convert(md)
}
