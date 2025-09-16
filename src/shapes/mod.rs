use crate::prelude::*;

pub mod circle;
pub mod line;
pub mod triangle;

pub trait Shape {
    fn scale(self: Box<Self>, factor: f32) -> Box<dyn Shape>;
    fn draw(&self) -> DrawCommands;
}
