pub mod circle;

pub trait Shape {
    fn scale(self: Box<Self>, factor: f32) -> Box<dyn Shape>;
}
