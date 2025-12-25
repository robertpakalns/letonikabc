use wasm_bindgen::prelude::wasm_bindgen;

mod macros;
mod parse_html;

#[wasm_bindgen]
pub fn test() -> String {
    let html = r#"
        <p>First paragraph</p>
        <p><span class="person">Bob</span>: Second paragraph</p>
    "#;

    parse_html::parse(html);

    html.to_string()
}
