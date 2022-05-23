mod utils;
mod eng;

use wasm_bindgen::prelude::*;
use eng::euler::{Vec3, Vec4, Mat3, Mat4};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, vector-demo!");
}

#[wasm_bindgen]
pub fn render_loop() {

}