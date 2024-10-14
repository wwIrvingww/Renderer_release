use crate::color::Color;
use crate::vertex::Vertex;

use minifb::{Key, Window, WindowOptions};

pub struct Framebuffer {
    width: usize,
    height: usize,
    buffer: Vec<u32>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![0; width * height],
        }
    }

    pub fn clear(&mut self, color: Color) {
        let packed_color = (255 << 24) | ((color.r as u32) << 16) | ((color.g as u32) << 8) | (color.b as u32);
        for pixel in self.buffer.iter_mut() {
            *pixel = packed_color;
        }
    }

    pub fn rasterize_triangles(&mut self, vertices: &[Vertex]) {
        // Implementar rasterización con z-buffer y baricéntricas
    }

    pub fn render_window(&self) {
        let mut window = Window::new(
            "Render Pipeline",
            self.width,
            self.height,
            WindowOptions::default(),
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

        while window.is_open() && !window.is_key_down(Key::Escape) {
            window
                .update_with_buffer(&self.buffer, self.width, self.height)
                .unwrap();
        }
    }
}
