use wasm_bindgen::prelude::wasm_bindgen;

mod macros;
mod parse_html;

#[wasm_bindgen]
pub fn test() -> String {
    let html = r#"
        <!-- wfrgrrg -->
        <html>
            <body>
                <h1>biba</h1>
                <p>boba</p>
            </body>
        </html>
    "#;

    parse_html::parse(html);

    html.to_string()
}
