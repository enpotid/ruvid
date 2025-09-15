use crate::prelude::*;

pub struct Line {
    pub start: Vec3,
    pub end: Vec3,
}

impl Line {
    pub fn new(start: Vec3, end: Vec3) -> Box<Self> {
        Box::new(Self { start, end })
    }
}

impl Shape for Line {
    fn scale(mut self: Box<Self>, factor: f32) -> Box<dyn Shape> {
        let midpoint = (self.start + self.end) / 2.0;
        self.start = factor * (self.start - midpoint) + midpoint;
        self.end = factor * (self.end - midpoint) + midpoint;
        self
    }

    fn draw(&self) -> DrawCommands {
        DrawCommands {
            commands: vec![DrawCommand::Line {
                p1: self.start,
                p2: self.end,
            }],
        }
    }
}
