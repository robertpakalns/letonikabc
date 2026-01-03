#![no_std]
extern crate alloc;

use alloc::string::String;

pub mod md2html;

pub fn convert(md: String) -> String {
    md2html::convert(&md)
}
