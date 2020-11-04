use wasm_bindgen::prelude::*;
use grass;

#[wasm_bindgen]
pub fn compile(s: &str, opts: JsValue) -> Result<String, JsValue> {
  let sass = grass::from_string(s.to_string(), &grass::Options::default());
  Ok(sass.unwrap())
}

