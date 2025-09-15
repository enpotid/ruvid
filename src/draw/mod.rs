use crate::prelude::*;

#[derive(Debug)]
pub struct DrawCommands {
    pub commands: Vec<DrawCommand>,
}

#[derive(Debug)]
pub enum DrawCommand {
    Triangle { p1: Vec3, p2: Vec3, p3: Vec3 },
    Cricle { p: Vec3, radius: f32 },
    Line { p1: Vec3, p2: Vec3 },
    Dot { p: Vec3 },

    Wait { second: f32 },
}

impl DrawCommands {
    pub fn new() -> Self {
        DrawCommands {
            commands: Vec::new(),
        }
    }
}
