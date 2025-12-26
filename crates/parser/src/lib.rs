use wasm_bindgen::prelude::wasm_bindgen;

mod macros;
mod parse_html;

#[wasm_bindgen]
pub fn test() -> String {
    let html = r#"
        <h1>:P</h1>
        <h2>:P</h2>
        <h3>:P</h3>
        <h4>:P</h4>
        <p>First paragraph</p>
        <p><span class="person">Bob</span>: Second paragraph</p>
    "#;

    parse_html::parse(html);

    html.to_string()
}
