use crate::prelude::*;

pub struct Circle {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
}

impl Circle {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            x: 0.0,
            y: 0.0,
            radius: 1.0,
        })
    }
}

impl Shape for Circle {
    fn scale(mut self: Box<Self>, factor: f32) -> Box<dyn Shape> {
        self.radius *= factor;
        self
    }
}
