// The wasm-pack uses wasm-bindgen to build and generate JavaScript binding file.
// Import the wasm-bindgen crate.
use wasm_bindgen::prelude::*;

// Our Add function
// wasm-pack requires "exported" functions
// to include #[wasm_bindgen]



#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

// https://rustwasm.github.io/wasm-bindgen/reference/types/string.html

#[wasm_bindgen]
pub fn hello_world() -> String {
    "Hello World From Rusky".into()
}


#[wasm_bindgen]
pub fn encrypt(input: String) -> String {
    return library::encrypt(&input).into();
}

#[wasm_bindgen]
pub fn decrypt(input: String) -> String {
    return library::decrypt(&input).into();
}