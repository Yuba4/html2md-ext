use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn html_to_markdown(html: &str) -> Result<String, JsError> {
    let converter = htmd::HtmlToMarkdown::builder()
        .skip_tags(vec!["script", "style", "noscript", "template"])
        .build();
    converter
        .convert(html)
        .map_err(|error| JsError::new(&error.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_html_and_drops_scripts() {
        let html = "<html><head><style>h1{}</style></head><body><h1>Title</h1><script>alert(1)</script><p>Hello <a href=\"https://example.com\">link</a></p></body></html>";
        let markdown = html_to_markdown(html).unwrap();
        assert_eq!(markdown, "# Title\n\nHello [link](https://example.com)");
    }
}
