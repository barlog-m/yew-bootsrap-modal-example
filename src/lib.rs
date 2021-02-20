mod app;
mod modal;
mod bootstrap;

use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    panic_hook_init();
    log_init();

    yew::start_app::<app::App>();

    Ok(())
}

pub fn log_init() {
    wasm_logger::init(wasm_logger::Config::default());
}

pub fn panic_hook_init() {
    console_error_panic_hook::set_once();
}
