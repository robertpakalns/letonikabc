#![no_std]
extern crate alloc;

use alloc::string::String;

mod md2html;
mod tests;

pub fn convert(md: String) -> String {
    md2html::convert(&md)
}
