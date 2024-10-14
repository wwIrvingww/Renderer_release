mod color;
mod framebuffer;
mod obj_loader;
mod shader;
mod vertex;

use color::Color;
use nalgebra_glm::{Vec3, Mat4};
use obj_loader::ObjLoader;
use shader::vertex_shader;
use framebuffer::Framebuffer;

fn create_model_matrix(translation: Vec3, scale: f32) -> Mat4 {
    Mat4::new_nonuniform_scaling(&Vec3::new(scale, scale, scale))
}

fn main() {
    let width = 800;
    let height = 600;

    // Crear framebuffer
    let mut framebuffer = Framebuffer::new(width, height);

    // Cargar el modelo OBJ
    let obj_loader = ObjLoader::load("assets/cube.obj").expect("Error cargando el archivo OBJ");
    let vertices = obj_loader.get_vertices();

    // Crear matriz de modelo
    let translation = Vec3::new(0.0, 0.0, -5.0);
    let model_matrix = create_model_matrix(translation, 1.0);

    // Shader y transformación de vértices
    let transformed_vertices: Vec<_> = vertices
        .iter()
        .map(|v| vertex_shader(v, &model_matrix))
        .collect();

    // Rasterizar el resultado en el framebuffer
    framebuffer.clear(Color::new(0, 0, 0)); // Fondo negro
    framebuffer.rasterize_triangles(&transformed_vertices);

    framebuffer.render_window();
}
