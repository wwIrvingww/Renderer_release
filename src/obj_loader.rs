use nalgebra_glm::{Vec3, Vec2};
use tobj;
use crate::vertex::Vertex;

pub struct ObjLoader {
    pub vertices: Vec<Vertex>,
}

impl ObjLoader {
    pub fn load(file_path: &str) -> Result<Self, tobj::LoadError> {
        let (models, _) = tobj::load_obj(file_path, &tobj::LoadOptions {
            triangulate: true, // Esto asegura que las caras cuadradas se conviertan en triángulos
            single_index: true,
            ..Default::default()
        })?;
        
        let mut vertices = Vec::new();

        for model in models {
            let mesh = &model.mesh;
            for v in mesh.positions.chunks(3) {
                vertices.push(Vertex {
                    position: Vec3::new(v[0], v[1], v[2]),
                    ..Vertex::default()
                });
            }
        }

        Ok(Self { vertices })
    }

    pub fn get_vertices(&self) -> Vec<Vertex> {
        self.vertices.clone()
    }
}
