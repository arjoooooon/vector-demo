use std::f64::consts::PI;
use std::string::String;

use super::euler::{IDENTITY4X4};
use super::euler::{Vec3, Vec4, Mat3, Mat4};

#[derive(Clone, Copy, Debug)]
pub struct Connection(u32, u32);

#[derive(Clone, Debug)]
pub struct GameObject {
    pub position: Vec3,
    pub orientation: Mat4, 
    pub points: Vec<Vec3>,
    pub connections: Vec<Connection>,
    pos: Vec4,                           // This contains a 1 in the `w` position
}

impl GameObject {
    pub fn new(position: Vec3, orientation: Mat4, points: Vec::<Vec3>, connections: Vec::<Connection>) -> GameObject {
        return GameObject {
            position: position,
            orientation: orientation,
            points: points,
            connections: connections,
            pos: Vec4{elems: [position[0], position[1], position[2], 1.0]}
        }
    }

    pub fn default() -> GameObject {
        return GameObject {
            position: Vec3{..Vec3::default()},
            orientation: IDENTITY4X4,
            points: Vec::<Vec3>::new(),
            connections: Vec::<Connection>::new(),
            pos: Vec4{..Vec4::default()},
        };
    }

    pub fn from_file(filename: String) -> GameObject {
        return GameObject{..GameObject::default() };
    }
}

pub struct Camera {
    pub position: Vec3,
    pub orientation: Mat4,
    pos: Vec4
}

impl Camera {
    pub fn new(position: Vec3, orientation: Mat4) -> Camera {
        return Camera {
            position: position,
            orientation: orientation,
            pos: Vec4{elems:[position[0], position[1], position[2], 1.0]}
        };
    }
    pub fn default() -> Camera {
        return Camera {
            position: Vec3{..Vec3::default()},
            orientation: IDENTITY4X4,
            pos: Vec4{..Vec4::default()},
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Line {
    pub tail_x: f64,
    pub tail_y: f64,
    pub head_x: f64,
    pub head_y: f64
}

#[derive(Clone, Copy)]
pub struct Renderer {
    pub FOV: f64,
    pub CLOSE_PLANE: f64,
    pub FAR_PLANE: f64,
    pub ASP: f64,
    pub F: f64,
    pub PROJECTION_MATRIX: Mat4 
}

impl Renderer {
    pub fn new(FOV: f64, CLOSE_PLANE: f64, FAR_PLANE: f64, ASP: f64, F: f64) -> Renderer {
        let mat_proj: Mat4 = Mat4 {elems: [
            ASP*F, 0.0, 0.0,                               0.0,
            0.0,   F,   0.0,                               0.0,
            0.0,   0.0, FAR_PLANE/(FAR_PLANE-CLOSE_PLANE), -(CLOSE_PLANE*FAR_PLANE)/(FAR_PLANE-CLOSE_PLANE),
            0.0,   0.0, 1.0,                               0.0
        ]};

        return Renderer {
            FOV: FOV,
            CLOSE_PLANE: CLOSE_PLANE,
            FAR_PLANE: FAR_PLANE,
            ASP: ASP,
            F: 1.0/(FOV/2.0).tan(),
            PROJECTION_MATRIX: mat_proj
        };
    }

    pub fn default() -> Renderer {
        return Renderer {
            FOV: PI/2.0,
            CLOSE_PLANE: 1.0,
            FAR_PLANE: 1000.0,
            ASP: 1.0,
            F: 1.0 / (PI/4.0).tan(),
            PROJECTION_MATRIX: Mat4{elems: [
                1.0 * 1.0/(PI/4.0).tan(), 0.0, 0.0, 0.0,
                0.0, 1.0/(PI/4.0).tan(), 0.0, 0.0,
                0.0, 0.0, 1000.0/(1000.0-1.0), -(1.0*1000.0)/(1000.0-1.0),
                0.0, 0.0, 1.0, 0.0
            ]}
        }
    }

    pub fn render_frame(self, objects: &Vec::<GameObject>, camera: &Camera) -> Vec::<Line> {
        let mut vectors_to_render: Vec::<Line> = Vec::<Line>::new(); 
        
        for object in objects {
            let num_connections: usize = object.connections.len();
            
            let position = &object.position;
            let translation = super::euler::translation_matrix(position[0], position[1], position[2]);
            
            let world_to_camera_matrix = Mat4{elems: [
                camera.orientation[0][0], camera.orientation[1][0], camera.orientation[2][0],  -camera.position[0],
                camera.orientation[0][1], camera.orientation[1][1], camera.orientation[2][1],  -camera.position[1],
                camera.orientation[0][2], camera.orientation[1][2], camera.orientation[2][2],  -camera.position[2],
                0.0,                      0.0,                      0.0,                       1.0,
            ]};

            let mut projected_points: Vec::<Vec4> = Vec::<Vec4>::new();

            for point in &object.points {
                let mut current_point = Vec4{elems:[point[0], point[1], point[2], 1.0]}; // Add W element to point
                
                // Transform to worldspace
                current_point = object.orientation * current_point;
                current_point = translation * current_point;

                // Transform to cameraspace
                current_point = world_to_camera_matrix * current_point;

                // Transform to projection space
                current_point = self.PROJECTION_MATRIX * current_point;
                current_point = current_point / current_point[3];

                projected_points.push(current_point);
            }
            
            for idx in 0..num_connections {
                let point1 = &projected_points[2*idx];
                let point2 = &projected_points[2*idx+1];
                
                vectors_to_render.push(Line{
                    tail_x: point1[0], tail_y: point1[1],
                    head_x: point2[0], head_y: point2[1]
                });
            } 
        }

        return vectors_to_render;
    }

}