#![no_std]
extern crate alloc;

mod parse_html;
mod tests;

pub use crate::parse_html::create_hash_from;
pub use parse_html::HtmlOut;

pub fn parse(html: &str) -> HtmlOut {
    parse_html::parse(html)
}
