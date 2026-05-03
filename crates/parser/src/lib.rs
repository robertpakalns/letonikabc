#![no_std]
extern crate alloc;

mod parse_html;
mod tests;

pub use crate::parse_html::{HtmlOut, create_hash_from, parse};
