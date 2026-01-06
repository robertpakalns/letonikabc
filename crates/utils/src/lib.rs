#![no_std]
extern crate alloc;

/// State of an element
pub enum El {
    Paragraph,
    Heading(usize),
}

/// Works like println!, but in a browser console
///
/// Usage: `console_log!(...);`
#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => {
        web_sys::console::log_1(&format!($($t)*).into());
    }
}
