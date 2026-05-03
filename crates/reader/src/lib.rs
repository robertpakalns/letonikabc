#![no_std]
extern crate alloc;

mod md2html;
mod tests;

pub use md2html::{MdOut, convert};
