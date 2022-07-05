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
pub struct GlobalWrapper {
    lines: Vec<f64>,
    renderer: eng::renderer::Renderer,
    camera: eng::renderer::Camera,
    objects: Vec<eng::renderer::GameObject>
}

#[wasm_bindgen]
impl GlobalWrapper {
    pub fn new(object: u32) -> GlobalWrapper { 
        let renderer = eng::renderer::Renderer::default();
        let camera  = eng::renderer::Camera::default();

        let mut shape: eng::renderer::GameObject = eng::renderer::GameObject::default();
        if object == 1 {
            shape = eng::renderer::make_cube(50.0);
            shape.angular_velocity = eng::euler::y_rotation_matrix(0.1);
        }

        return GlobalWrapper {
            lines: Vec::new(),
            renderer: renderer,
            camera: camera,
            objects: vec![shape]
        }        
    }

    pub fn update_aspect_ratio(&mut self, value: f64) {
        self.renderer.update_aspect_ratio(value);
    }

    pub fn get_lines(&self) -> *const f64{
        return self.lines.as_ptr();
    }

    pub fn render_loop(&mut self) {
        for object in &mut self.objects {
            object.tick();
        }

        self.lines = self.renderer.render_frame(&self.objects, &self.camera);
    }
}