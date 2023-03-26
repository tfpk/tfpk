#![recursion_limit = "1024"]

mod app;
mod home;
mod glyphs;

use wasm_bindgen::prelude::*;
use crate::app::App;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is the entry point for the web app
#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
    Ok(())
}
