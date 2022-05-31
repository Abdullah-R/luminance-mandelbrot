use luminance_derive::{Semantics, Vertex};


pub const VS_STR: &str = include_str!("vs.glsl");
pub const FS_STR: &str = include_str!("fs.glsl");

// pub const VERTICES: [Vertex; 3] = [
//   Vertex::new(
//     VertexPosition::new([-0.5, -0.5]),
//     VertexRGB::new([255, 0, 0]),
//   ),
//   Vertex::new(
//     VertexPosition::new([0.5, -0.5]),
//     VertexRGB::new([0, 255, 0]),
//   ),
//   Vertex::new(
//     VertexPosition::new([0., 0.5]),
//     VertexRGB::new([0, 0, 255])
//   ),
// ];


#[derive(Copy, Clone, Debug, Semantics)]
pub enum VertexSemantics {
    #[sem(name = "position", repr = "[f32; 2]", wrapper = "VertexPosition")]
    Position,
    #[sem(name = "color", repr = "[u8; 3]", wrapper = "VertexRGB")]
    Color,
}

#[derive(Clone, Copy, Debug, Vertex)]
#[vertex(sem = "VertexSemantics")]
pub struct Vertex {
    #[allow(dead_code)]
    position: VertexPosition,

    #[allow(dead_code)]
    #[vertex(normalized = "true")]
    color: VertexRGB,
}

use std::ops::*;

#[derive(Clone, Copy)]
pub struct CompNum {
    real: f32,
    imag: f32
}

impl Add for CompNum {
    type Output = CompNum;    

    fn add(self, other: CompNum) -> CompNum{
        CompNum {
            real: self.real + other.real,
            imag: self.imag + other.imag
        }
    }
}

impl Sub for CompNum {
    type Output = CompNum;

    fn sub(self, other: CompNum) -> CompNum {
        CompNum {
            real: self.real - other.real,
            imag: self.imag - other.imag
        }
    }
}

impl Mul for CompNum {
    type Output = CompNum;    

    fn mul(self, other: CompNum) -> CompNum{
        CompNum {
            real: self.real * other.real - self.imag * other.imag,
            imag: self.imag * other.real + self.real * other.imag
        }
    }
}

impl CompNum {

    pub fn new(x:f32, y:f32) -> CompNum {
        CompNum{real: x, imag: y}
    }

    pub fn abs(self) -> f32{
        (self.real.powi(2) + self.imag.powi(2)).sqrt()
    }
    
    pub fn mandel(self) -> bool{
        let mut j = CompNum {real: 0., imag: 0.};
        for i in 1..200 {
            j = (j * j) + self;
            if j.abs() > 2. {break;}
        }
        if j.abs() > 2. {false} else {true}
    }
}