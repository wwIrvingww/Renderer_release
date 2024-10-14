use nalgebra_glm::{Mat4, Vec3};
use crate::vertex::Vertex;

pub fn vertex_shader(vertex: &Vertex, model_matrix: &Mat4) -> Vertex {
    let transformed_position = model_matrix * vertex.position.to_homogeneous();
    Vertex {
        position: Vec3::new(transformed_position.x, transformed_position.y, transformed_position.z),
        ..*vertex
    }
}
