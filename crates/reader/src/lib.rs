#![no_std]
extern crate alloc;

mod md2html;
mod search;
mod tests;

pub use md2html::{MdOut, convert};

pub use search::find_substr;
