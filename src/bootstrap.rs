use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "bootstrap")]
extern "C" {
    #[wasm_bindgen]
    pub type Modal;

    #[wasm_bindgen(constructor)]
    pub fn new(e: web_sys::Element) -> Modal;

    #[wasm_bindgen(method)]
    pub fn toggle(this: &Modal);

    #[wasm_bindgen(method)]
    pub fn show(this: &Modal);

    #[wasm_bindgen(method)]
    pub fn hide(this: &Modal);
}

pub fn get_modal_by_id(id: &str) -> Modal {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let element = document.get_element_by_id(id).expect(&format!("no element with id '{}'", id));

    Modal::new(element)
}
