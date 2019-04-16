#[cfg(test)]
mod tests {
    use unicode_normalization::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn run() {
        for x in "test".nfc() {}
    }
}
