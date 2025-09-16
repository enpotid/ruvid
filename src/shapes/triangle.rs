use crate::prelude::*;

pub struct Triangle {
    pub p1: Vec3,
    pub p2: Vec3,
    pub p3: Vec3,
}

impl Triangle {
    pub fn new(p1: Vec3, p2: Vec3, p3: Vec3) -> Box<Self> {
        Box::new(Self { p1, p2, p3 })
    }
}

impl Shape for Triangle {
    fn scale(mut self: Box<Self>, factor: f32) -> Box<dyn Shape> {
        let midpoint = (self.p1 + self.p2 + self.p3) / 3.0;
        self.p1 = factor * (self.p1 - midpoint) + midpoint;
        self.p2 = factor * (self.p2 - midpoint) + midpoint;
        self.p3 = factor * (self.p3 - midpoint) + midpoint;
        self
    }

    fn draw(&self) -> DrawCommands {
        DrawCommands {
            commands: vec![DrawCommand::Triangle {
                p1: self.p1,
                p2: self.p2,
                p3: self.p3,
            }],
        }
    }
}
