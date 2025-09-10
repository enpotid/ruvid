use crate::prelude::*;

pub struct Circle {
    pub position: Vec3,
    pub radius: f32,
}

impl Circle {
    pub fn new(position: Vec3, radius: f32) -> Box<Self> {
        Box::new(Self { position, radius })
    }
}

impl Shape for Circle {
    fn scale(mut self: Box<Self>, factor: f32) -> Box<dyn Shape> {
        self.radius *= factor;
        self
    }
}
