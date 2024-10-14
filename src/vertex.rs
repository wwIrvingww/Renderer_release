use nalgebra_glm::Vec3;
use crate::color::Color;

#[derive(Clone, Debug)]
pub struct Vertex {
    pub position: Vec3,
    pub color: Color,
}

impl Default for Vertex {
    fn default() -> Self {
        Vertex {
            position: Vec3::new(0.0, 0.0, 0.0),
            color: Color::new(255, 255, 255),
        }
    }
}
