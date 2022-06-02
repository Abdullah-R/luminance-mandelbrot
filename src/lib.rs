use std::ops::*;
use glium::*;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
}

implement_vertex!(Vertex, position);

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
        for _i in 1..50 {
            j = (j * j) + self;
        }
        j.abs() < 2.
    }
}