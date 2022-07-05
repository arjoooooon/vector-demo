use std::f64::consts::PI;
use std::string::String;

use super::euler::{IDENTITY4X4};
use super::euler::{Vec3, Vec4, Mat3, Mat4};

#[derive(Clone, Debug)]
pub struct GameObject {
    pub position: Vec3,
    pub velocity: Vec3,
    pub orientation: Mat4, 
    pub angular_velocity: Mat4,
    pub points: Vec<Vec3>,
    pub connections: Vec<usize>,
    pos: Vec4,                           // This contains a 1 in the `w` position
}

pub fn make_cube(side_length: f64) -> GameObject {
    let hf: f64 = side_length / 2.0;
    
    let vec0 = Vec3::new(-hf, -hf, -hf);
    let vec1 = Vec3::new(hf, -hf, -hf);
    let vec2 = Vec3::new(-hf, hf, -hf);
    let vec3 = Vec3::new(hf, hf, -hf);
    let vec4 = Vec3::new(-hf, -hf, hf);
    let vec5 = Vec3::new(hf, -hf, hf);
    let vec6 = Vec3::new(-hf, hf, hf);
    let vec7 = Vec3::new(hf, hf, hf);
 
    let vec_points: Vec<Vec3> = vec![vec0, vec1, vec2, vec3, vec4, vec5, vec6, vec7];
    let vec_connections: Vec<usize> = vec![0, 1, 1, 3, 3, 2, 2, 0, 4, 5, 5, 7, 7, 6, 6, 4, 0, 4, 1, 5, 2, 6, 3, 7];

    return GameObject::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 0.0),
        IDENTITY4X4,
        IDENTITY4X4,
        vec_points,
        vec_connections);
}

impl GameObject {
    pub fn new(position: Vec3, velocity: Vec3, orientation: Mat4, angular_velocity: Mat4, points: Vec::<Vec3>, connections: Vec::<usize>) -> GameObject {
        return GameObject {
            position: position,
            velocity: velocity,
            orientation: orientation,
            angular_velocity: angular_velocity,
            points: points,
            connections: connections,
            pos: Vec4{elems: [position[0], position[1], position[2], 1.0]}
        }
    }

    pub fn default() -> GameObject {
        return GameObject {
            position: Vec3{..Vec3::default()},
            velocity: Vec3{..Vec3::default()},
            orientation: IDENTITY4X4,
            angular_velocity: IDENTITY4X4,
            points: Vec::<Vec3>::new(),
            connections: Vec::<usize>::new(),
            pos: Vec4{..Vec4::default()},
        };
    }

    pub fn tick(&mut self) {
        self.position = self.position + self.velocity;
        self.orientation = self.angular_velocity * self.orientation;
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
            position: Vec3{elems: [0.0, 0.0, -200.0]},
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

    pub fn update_aspect_ratio(&mut self, NEW_ASP: f64) {
        self.ASP = NEW_ASP;
        self.PROJECTION_MATRIX = Mat4{elems: [
            NEW_ASP * 1.0/(PI/4.0).tan(), 0.0, 0.0, 0.0,
            0.0, 1.0/(PI/4.0).tan(), 0.0, 0.0,
            0.0, 0.0, 1000.0/(1000.0-1.0), -(1.0*1000.0)/(1000.0-1.0),
            0.0, 0.0, 1.0, 0.0
        ]} 
    }

    pub fn render_frame(& mut self, objects: &Vec::<GameObject>, camera: &Camera) -> Vec::<f64> {
        let mut vectors_to_render: Vec::<f64> = Vec::<f64>::new(); 
        
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
           
            for idx in 0..num_connections/2 {
                let idx1: usize = object.connections[2*idx];
                let idx2: usize = object.connections[2*idx + 1];


                let tail = &projected_points[idx1];
                let head = &projected_points[idx2];
                
                vectors_to_render.push(tail[0]);
                vectors_to_render.push(tail[1]);
                vectors_to_render.push(head[0]);
                vectors_to_render.push(head[1]);
            } 
        }

        return vectors_to_render;
    }

}