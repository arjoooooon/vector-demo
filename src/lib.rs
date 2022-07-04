mod utils;
mod eng;

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use eng::euler::{Vec3, Vec4, Mat3, Mat4};


extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

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
pub fn greet(name: &str) {
    alert(&format!("{}!", name));
}

#[wasm_bindgen]
pub struct GlobalWrapper {
    lines: Vec<f64>,
    renderer: eng::renderer::Renderer,
    camera: eng::renderer::Camera
}

#[wasm_bindgen]
impl GlobalWrapper {
    pub fn new() -> GlobalWrapper { 
        let mut renderer = eng::renderer::Renderer::default();
        renderer.update_aspect_ratio(1.158);

        let camera  = eng::renderer::Camera::default();let mut renderer = eng::renderer::Renderer::default();
        renderer.update_aspect_ratio(1.158);

        let camera  = eng::renderer::Camera::default();

        return GlobalWrapper {
            lines: Vec::new(),
            renderer: renderer,
            camera: camera
        }        
    }

    pub fn get_lines(&self) -> *const f64{
        return self.lines.as_ptr();
    }

    pub fn render_loop(&mut self) {

        // Hardcoded cube
        let vec0 = Vec3::new(0.0, 0.0, 0.0);
        let vec1 = Vec3::new(50.0, 0.0, 0.0);
        let vec2 = Vec3::new(0.0, 50.0, 0.0);
        let vec3 = Vec3::new(50.0, 50.0, 0.0);
        let vec4 = Vec3::new(0.0, 0.0, 50.0);
        let vec5 = Vec3::new(50.0, 0.0, 50.0);
        let vec6 = Vec3::new(0.0, 50.0, 50.0);
        let vec7 = Vec3::new(50.0, 50.0, 50.0);

        let vec_points: Vec<Vec3> = vec![vec0, vec1, vec2, vec3, vec4, vec5, vec6, vec7];
        let vec_connections: Vec<usize> = vec![0, 1, 1, 3, 3, 2, 2, 0, 4, 5, 5, 7, 7, 6, 6, 4, 0, 4, 1, 5, 2, 6, 3, 7];
        
        let cube = eng::renderer::GameObject::new(Vec3::default(), eng::euler::IDENTITY4X4,
                                                vec_points, vec_connections);

        let mut game_objects: Vec<eng::renderer::GameObject> = Vec::new(); 
        game_objects.push(cube);

        self.lines = self.renderer.render_frame(&game_objects, &self.camera);
    }
}