use std::ops::{Index, Add, Sub, Mul, Div};
use std::io;
use std::io::Write;

// Struct Definitions

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub elems: [f64; 3],
}

#[derive(Debug, Clone, Copy)]
pub struct Vec4 {
    pub elems: [f64; 4],
}

#[derive(Debug, Clone, Copy)]
pub struct Mat3 {
    pub elems: [f64; 9],
}

#[derive(Debug, Clone, Copy)]
pub struct Mat4 {
    pub elems: [f64; 16],
}

// Method implementations
// []

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index<'a>(&'a self, idx: usize) -> &'a f64 {
        return &self.elems[idx];
    }
}

impl Index<usize> for Vec4 {
    type Output = f64;

    fn index<'a>(&'a self, idx: usize) -> &'a f64 {
        return &self.elems[idx];
    }
}

impl Index<usize> for Mat3 {
    type Output = [f64];

    fn index<'a>(&'a self, idx:usize) -> &'a [f64] {
        let start_idx = idx*3;
        let end_idx = start_idx + 3;
        return &self.elems[start_idx..end_idx];
    }
}

impl Index<usize> for Mat4 {
    type Output = [f64];

    fn index<'a>(&'a self, idx:usize) -> &'a [f64] {
        let start_idx = idx*4;
        let end_idx = start_idx + 4;
        return &self.elems[start_idx..end_idx];
    }
}

// +

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        return Vec3 {elems: [self.elems[0] + other.elems[0],
                      self.elems[1] + other.elems[1],
                      self.elems[2] + other.elems[2]]};
    }
}

impl Add for Vec4 {
    type Output = Vec4;

    fn add(self, other: Vec4) -> Vec4 {
        return Vec4 {elems: [self.elems[0] + other.elems[0],
                      self.elems[1] + other.elems[1],
                      self.elems[2] + other.elems[2],
                      self.elems[3] + other.elems[3]]};
    }
}

// -

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        return Vec3 {elems: [self.elems[0] - other.elems[0],
                      self.elems[1] - other.elems[1],
                      self.elems[2] - other.elems[2]]};
    }
}


impl Sub for Vec4 {
    type Output = Vec4;

    fn sub(self, other: Vec4) -> Vec4 {
        return Vec4 {elems: [self.elems[0] - other.elems[0],
                             self.elems[1] - other.elems[1],
                             self.elems[2] - other.elems[2],
                             self.elems[3] - other.elems[3]]};
    }
}

// Scalar Multiplication

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: f64) -> Vec3 {
        return Vec3 {elems: [
            scalar*self.elems[0], scalar*self.elems[1], scalar*self.elems[2]
        ]};
    }
}

impl Mul<f64> for Vec4 {
    type Output = Vec4;

    fn mul(self, scalar: f64) -> Vec4 {
        return Vec4 {elems: [
            scalar*self.elems[0], scalar*self.elems[1], scalar*self.elems[2], scalar*self.elems[3]
        ]};
    }
}

impl Mul<f64> for Mat3 {
    type Output = Mat3;

    fn mul(self, scalar: f64) -> Mat3 {
        return Mat3 {elems: [
            scalar*self.elems[0], scalar*self.elems[1], scalar*self.elems[2],
            scalar*self.elems[3], scalar*self.elems[4], scalar*self.elems[5],
            scalar*self.elems[6], scalar*self.elems[7], scalar*self.elems[8],
        ]}
    }
}

impl Mul<f64> for Mat4 {
    type Output = Mat4;

    fn mul(self, scalar: f64) -> Mat4 {
        return Mat4 {elems: [
            scalar*self.elems[0], scalar*self.elems[1], scalar*self.elems[2], scalar*self.elems[3],
            scalar*self.elems[4], scalar*self.elems[5], scalar*self.elems[6], scalar*self.elems[7],
            scalar*self.elems[8], scalar*self.elems[9], scalar*self.elems[10], scalar*self.elems[11],
            scalar*self.elems[12], scalar*self.elems[13], scalar*self.elems[14], scalar*self.elems[15],
        ]}
    }
}

// Scalar Division

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, scalar: f64)  -> Vec3 {
        return Vec3 {elems: [
            self.elems[0]/scalar, self.elems[1]/scalar, self.elems[2]/scalar
        ]};
    }
}

impl Div<f64> for Vec4 {
    type Output = Vec4;

    fn div(self, scalar: f64)  -> Vec4 {
        return Vec4 {elems: [
            self.elems[0]/scalar, self.elems[1]/scalar, self.elems[2]/scalar, self.elems[3]/scalar
        ]};
    }
}

impl Div<f64> for Mat3 {
    type Output = Mat3;

    fn div(self, scalar: f64)  -> Mat3 {
        return Mat3 {elems: [
            self.elems[0]/scalar, self.elems[1]/scalar, self.elems[2]/scalar,
            self.elems[3]/scalar, self.elems[4]/scalar, self.elems[5]/scalar,
            self.elems[6]/scalar, self.elems[7]/scalar, self.elems[8]/scalar
        ]};
    }
}

impl Div<f64> for Mat4 {
    type Output = Mat4;

    fn div(self, scalar: f64)  -> Mat4 {
        return Mat4 {elems: [
            self.elems[0]/scalar, self.elems[1]/scalar, self.elems[2]/scalar, self.elems[3]/scalar,
            self.elems[4]/scalar, self.elems[5]/scalar, self.elems[6]/scalar, self.elems[7]/scalar,
            self.elems[8]/scalar, self.elems[9]/scalar, self.elems[10]/scalar, self.elems[11]/scalar,
            self.elems[12]/scalar, self.elems[13]/scalar, self.elems[14]/scalar, self.elems[15]/scalar,
        ]};
    }
}

// Matrix-Vector Multiplication

impl Mul<Vec3> for Mat3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        return Vec3 { elems: [
            self[0][0] * other[0] + self[0][1] * other[1] + self[0][2] * other[2],
            self[1][0] * other[0] + self[1][1] * other[1] + self[1][2] * other[2],
            self[2][0] * other[0] + self[2][1] * other[1] + self[2][2] * other[2]
        ]}
    }
}

impl Mul<Vec4> for Mat4 {
    type Output = Vec4;

    fn mul(self, other: Vec4) -> Vec4 {
        return Vec4 { elems: [
            self[0][0] * other[0] + self[0][1] * other[1] + self[0][2] * other[2] + self[0][3] * other[3],
            self[1][0] * other[0] + self[1][1] * other[1] + self[1][2] * other[2] + self[1][3] * other[3],
            self[2][0] * other[0] + self[2][1] * other[1] + self[2][2] * other[2] + self[2][3] * other[3],
            self[3][0] * other[0] + self[3][1] * other[1] + self[3][2] * other[2] + self[3][3] * other[3],
        ]}
    }
}

// Matrix-Matrix Multiplication

impl Mul for Mat3 {
    type Output = Mat3;

    fn mul(self, other: Mat3) -> Mat3 {
        let mut init_values = [0.0; 9];
        
        for row in 0..3 {
            for col in 0..3 {
                init_values[3*row + col] = self[row][0] * other[0][col] +
                                           self[row][1] * other[1][col] +
                                           self[row][2] * other[2][col];
            }
        }

        return Mat3{elems: init_values};
    }
}

impl Mul for Mat4 {
    type Output = Mat4;

    fn mul(self, other: Mat4) -> Mat4 {
        let mut init_values = [0.0; 16];
        
        for row in 0..4 {
            for col in 0..4 {
                init_values[4*row + col] = self[row][0] * other[0][col] +
                                           self[row][1] * other[1][col] +
                                           self[row][2] * other[2][col] +
                                           self[row][3] * other[3][col]
            }
        }

        return Mat4{elems: init_values};
    }
}

// Other required functions

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        return Vec3{elems: [x, y, z]};
    }
    pub fn default() -> Vec3 {
        return Vec3{elems: [0.0, 0.0, 0.0]};
    }
    pub fn log(self) -> () {
        println!("<{}, {}, {}>", self[0], self[1], self[2]);
    }
    pub fn dot(self, other: Vec3) -> f64 {
        return self[0]*other[0] + self[1]*other[1] + self[2]*other[2];
    }
    pub fn cross(self, other: Vec3) -> Vec3 {
        return Vec3{elems: [
            self[1] * other[2] - self[2] * other[1],
            self[2] * other[0] - self[0] * other[2],
            self[0] * other[1] - self[1] * other[0]
        ]};
    }
}

impl Vec4 {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Vec4 {
        return Vec4 {elems: [x, y, z, w]};
    }
    pub fn default() -> Vec4 {
        return Vec4 {elems: [0.0, 0.0, 0.0, 1.0]};
    }
    pub fn log(self) -> () {
        println!("<{}, {}, {}, {}>", self[0], self[1], self[2], self[3]);
    }
    fn dot(self, other: Vec4) -> f64 {
        return self[0] * other[0] + self[1] * other[1] + self[2] * other[2] + self[3] * other[3];
    }
}

impl Mat3 {
    pub fn new(arr: [f64;9]) -> Mat3 {
        return Mat3 {elems: arr};
    }

    pub fn default() -> Mat3 {
        return Mat3{elems: [
            0.0, 0.0, 0.0, 
            0.0, 0.0, 0.0, 
            0.0, 0.0, 0.0, 
        ]}
    }

    pub fn log(self) -> () {
        for row in 0..3 {
            for col in 0..3 {
                print!("{} ", self[row][col]);
                io::stdout().flush().unwrap();
            }
            println!();
        }
    }
}

impl Mat4 {
    pub fn new(arr: [f64; 16]) -> Mat4 {
        return Mat4{elems: arr};
    }
    pub fn default() -> Mat4 {
        return Mat4{elems: [
            0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0,
        ]}
    }
    pub fn inverse(self) -> Mat4 {
        let mut inv = [0.0; 16];
        let mut m = [0.0; 16];
        let mut det;
        
        m = self.elems;        

        inv[0] = m[5]  * m[10] * m[15] -
                m[5]  * m[11] * m[14] -
                m[9]  * m[6]  * m[15] +
                m[9]  * m[7]  * m[14] +
                m[13] * m[6]  * m[11] -
                m[13] * m[7]  * m[10];

        inv[4] = -m[4]  * m[10] * m[15] +
                m[4]  * m[11] * m[14] +
                m[8]  * m[6]  * m[15] -
                m[8]  * m[7]  * m[14] -
                m[12] * m[6]  * m[11] +
                m[12] * m[7]  * m[10];

        inv[8] = m[4]  * m[9] * m[15] -
                m[4]  * m[11] * m[13] -
                m[8]  * m[5] * m[15] +
                m[8]  * m[7] * m[13] +
                m[12] * m[5] * m[11] -
                m[12] * m[7] * m[9];

        inv[12] = -m[4]  * m[9] * m[14] +
                m[4]  * m[10] * m[13] +
                m[8]  * m[5] * m[14] -
                m[8]  * m[6] * m[13] -
                m[12] * m[5] * m[10] +
                m[12] * m[6] * m[9];

        inv[1] = -m[1]  * m[10] * m[15] +
                m[1]  * m[11] * m[14] +
                m[9]  * m[2] * m[15] -
                m[9]  * m[3] * m[14] -
                m[13] * m[2] * m[11] +
                m[13] * m[3] * m[10];

        inv[5] = m[0]  * m[10] * m[15] -
                m[0]  * m[11] * m[14] -
                m[8]  * m[2] * m[15] +
                m[8]  * m[3] * m[14] +
                m[12] * m[2] * m[11] -
                m[12] * m[3] * m[10];

        inv[9] = -m[0]  * m[9] * m[15] +
                m[0]  * m[11] * m[13] +
                m[8]  * m[1] * m[15] -
                m[8]  * m[3] * m[13] -
                m[12] * m[1] * m[11] +
                m[12] * m[3] * m[9];

        inv[13] = m[0]  * m[9] * m[14] -
                m[0]  * m[10] * m[13] -
                m[8]  * m[1] * m[14] +
                m[8]  * m[2] * m[13] +
                m[12] * m[1] * m[10] -
                m[12] * m[2] * m[9];

        inv[2] = m[1]  * m[6] * m[15] -
                m[1]  * m[7] * m[14] -
                m[5]  * m[2] * m[15] +
                m[5]  * m[3] * m[14] +
                m[13] * m[2] * m[7] -
                m[13] * m[3] * m[6];

        inv[6] = -m[0]  * m[6] * m[15] +
                m[0]  * m[7] * m[14] +
                m[4]  * m[2] * m[15] -
                m[4]  * m[3] * m[14] -
                m[12] * m[2] * m[7] +
                m[12] * m[3] * m[6];

        inv[10] = m[0]  * m[5] * m[15] -
                m[0]  * m[7] * m[13] -
                m[4]  * m[1] * m[15] +
                m[4]  * m[3] * m[13] +
                m[12] * m[1] * m[7] -
                m[12] * m[3] * m[5];

        inv[14] = -m[0]  * m[5] * m[14] +
                m[0]  * m[6] * m[13] +
                m[4]  * m[1] * m[14] -
                m[4]  * m[2] * m[13] -
                m[12] * m[1] * m[6] +
                m[12] * m[2] * m[5];

        inv[3] = -m[1] * m[6] * m[11] +
                m[1] * m[7] * m[10] +
                m[5] * m[2] * m[11] -
                m[5] * m[3] * m[10] -
                m[9] * m[2] * m[7] +
                m[9] * m[3] * m[6];

        inv[7] = m[0] * m[6] * m[11] -
                m[0] * m[7] * m[10] -
                m[4] * m[2] * m[11] +
                m[4] * m[3] * m[10] +
                m[8] * m[2] * m[7] -
                m[8] * m[3] * m[6];

        inv[11] = -m[0] * m[5] * m[11] +
                m[0] * m[7] * m[9] +
                m[4] * m[1] * m[11] -
                m[4] * m[3] * m[9] -
                m[8] * m[1] * m[7] +
                m[8] * m[3] * m[5];

        inv[15] = m[0] * m[5] * m[10] -
                m[0] * m[6] * m[9] -
                m[4] * m[1] * m[10] +
                m[4] * m[2] * m[9] +
                m[8] * m[1] * m[6] -
                m[8] * m[2] * m[5];

        det = m[0] * inv[0] + m[1] * inv[4] + m[2] * inv[8] + m[3] * inv[12];

        det = 1.0 / det;
        let temp = Mat4{elems: inv};
        let inverted_matrix = temp * det;

        return inverted_matrix;
    }
    pub fn log(self) -> () {
        for row in 0..4 {
            for col in 0..4 {
                print!("{} ", self[row][col]);
                io::stdout().flush().unwrap();
            }
            println!();
        }
    }
}

// Other important functions and constants

#[allow(dead_code)]
pub fn x_rotation_matrix(theta: f64) -> Mat4 {
    return Mat4{elems: [
        1.0, 0.0,          0.0,           0.0,
        0.0, theta.cos(), -(theta.sin()), 0.0,
        0.0, theta.sin(),  theta.cos(),   0.0,
        0.0, 0.0,           0.0,          1.0
    ]};
}

#[allow(dead_code)]
pub fn y_rotation_matrix(theta: f64) -> Mat4 {
    return Mat4{elems: [
         theta.cos(),   0.0, theta.sin(), 0.0,
         0.0,           1.0, 0.0,         0.0,
        -theta.sin(), 0.0, theta.cos(), 0.0,
         0.0,           0.0, 0.0,         1.0
    ]};
}

#[allow(dead_code)]
pub fn z_rotation_matrix(theta: f64) -> Mat4 {
    return Mat4{elems: [
        theta.cos(), -(theta.sin()), 0.0, 0.0,
        theta.sin(),  theta.cos(),   0.0, 0.0, 
        0.0,          0.0,           1.0, 0.0, 
        0.0,          0.0,           0.0, 1.0
    ]}
}

pub fn translation_matrix(delta_x: f64, delta_y: f64, delta_z: f64) -> Mat4 {
    return Mat4 {elems: [
        1.0, 0.0, 0.0, delta_x,
        0.0, 1.0, 0.0, delta_y,
        0.0, 0.0, 1.0, delta_z,
        0.0, 0.0, 0.0,     1.0
    ]}
}

#[allow(dead_code)]
pub const IDENTITY3X3: Mat3 = Mat3 {elems: [
    1.0, 0.0, 0.0,
    0.0, 1.0, 0.0,
    0.0, 0.0, 1.0
]};

#[allow(dead_code)]
pub const IDENTITY4X4: Mat4 = Mat4{elems: [
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0, 
    0.0, 0.0, 1.0, 0.0,
    0.0, 0.0, 0.0, 1.0
]};