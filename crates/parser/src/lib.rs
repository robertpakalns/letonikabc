use wasm_bindgen::prelude::wasm_bindgen;

mod macros;
mod parse_html;

#[wasm_bindgen]
pub struct Output {
    markdown: String,
    header_lines: Vec<usize>,
}

#[wasm_bindgen]
impl Output {
    #[wasm_bindgen(constructor)]
    pub fn new(markdown: String, header_lines: Vec<usize>) -> Output {
        Output {
            markdown,
            header_lines,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn markdown(&self) -> String {
        self.markdown.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn header_lines(&self) -> Vec<usize> {
        self.header_lines.clone()
    }
}

// html5ever IS A BLOATWARE; REPLACE IT
#[wasm_bindgen]
pub fn test() -> Output {
    let html = r#"
        <h1>:P</h1>
        <h2>:P</h2>
        <p>First paragraph</p>
        <p><span class="person">Bob</span>: Second paragraph</p>

                                        <h1>:P</h1>
    "#;

    let (markdown, header_lines) = parse_html::parse(html);

    Output::new(markdown, header_lines)
}
