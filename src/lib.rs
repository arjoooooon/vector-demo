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
    let renderer = eng::renderer::Renderer::default();
    let camera  = eng::renderer::Camera::default();

    // Hardcoded cube
    let vec0 = Vec3::new(0.0, 0.0, 0.0);
    let vec1 = Vec3::new(50.0, 0.0, 0.0);
    let vec2 = Vec3::new(0.0, 50.0, 0.0);
    let vec3 = Vec3::new(50.0, 50.0, 0.0);
    let vec4 = Vec3::new(0.0, 0.0, 50.0);
    let vec5 = Vec3::new(50.0, 0.0, 50.0);
    let vec6 = Vec3::new(0.0, 50.0, 50.0);
    let vec7 = Vec3::new(50.0, 50.0, 50.0);

    let points = [vec0, vec1, vec2, vec3, vec4, vec5, vec6, vec7];
    let connections = [0, 1, 1, 3, 3, 2, 2, 0, 4, 5, 5, 7, 7, 6, 6, 4, 0, 4, 1, 5, 2, 6, 3, 7];

    let vec_points: Vec<Vec3> = Vec3::new(points);
    let vec_connections: Vec<u32> = Vec3::new(connections);
    
    let cube = eng::renderer::GameObject::new(Vec3::default(), orientation: IDENTITY4X4,
                                              points: vec_points, connections: vec_connections);
}