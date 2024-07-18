pub mod error;
pub mod restful;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Obj {}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn test() -> i32 {
    log("123");
    1
}

#[wasm_bindgen]
pub fn parse_restl(s: &str) -> String {
    log(format!("输入：{}", &s).as_str());
    match restful::parse_json_from_string(s.to_string()) {
        Ok(value) => value.to_string(),
        Err(err) => err.to_string(),
    }
}

#[wasm_bindgen]
pub fn create_obj() -> Obj {
    Obj {}
}
