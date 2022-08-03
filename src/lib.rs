use std::ffi::CString;
use std::os::raw::c_char;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Repl {}

#[wasm_bindgen]
impl Repl {
    pub fn eval(&self, input: &str) -> *mut c_char {
        let you_typed = format!("You typed: {}", input);
        let output = CString::new(you_typed).unwrap();
        output.into_raw()
    }
    pub fn new() -> Repl {
        return Repl {};
    }
}
