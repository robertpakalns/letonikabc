#![no_std]
extern crate alloc;

use alloc::{string::String, vec::Vec};
use wasm_bindgen::prelude::wasm_bindgen;
use wee_alloc::WeeAlloc;

use parser::{HtmlOut, create_hash_from, parse};
use reader::{MdOut, convert};

/// The crate is built without the Rust standart library, which significantly reduces the binary size
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
pub fn parse_html(html: String) -> HtmlOutput {
    HtmlOutput {
        inner: parse(&html),
    }
}

#[wasm_bindgen]
pub fn parse_md(md: String) -> MdOutput {
    MdOutput {
        inner: convert(&md),
    }
}

#[wasm_bindgen]
pub struct HtmlOutput {
    inner: HtmlOut,
}

#[wasm_bindgen]
impl HtmlOutput {
    #[wasm_bindgen(getter)]
    pub fn markdown(&self) -> String {
        self.inner.markdown.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn hash(&self) -> String {
        self.inner.hash.clone()
    }
}

#[wasm_bindgen]
pub fn create_hash_from_markdown(md: &str) -> String {
    create_hash_from(md)
}

#[wasm_bindgen]
pub struct MdOutput {
    inner: MdOut,
}

#[wasm_bindgen]
impl MdOutput {
    #[wasm_bindgen(getter)]
    pub fn html(&self) -> String {
        self.inner.html.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn heading_lines(&self) -> Vec<usize> {
        self.inner.heading_lines.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn heading_contents(&self) -> Vec<String> {
        self.inner.heading_contents.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn heading_levels(&self) -> Vec<usize> {
        self.inner.heading_levels.clone()
    }
}

#[wasm_bindgen]
pub fn find_substr(str: &str, substr: &str, diacritic: bool, case_sensitive: bool) -> Vec<u32> {
    reader::find_substr(str, substr, diacritic, case_sensitive)
}
