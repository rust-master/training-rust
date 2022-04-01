extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct DemoClient {}

#[wasm_bindgen]
impl DemoClient {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        log("New hit Rust!");
        Self {}
    }

    pub fn update(&mut self, _time: f32, _height: f32, _width: f32) -> Result<(), JsValue> {
        log("Rust update!");
        Ok(())
    }

    pub fn render(&self) {}
}
