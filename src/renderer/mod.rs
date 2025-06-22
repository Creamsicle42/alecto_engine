use glium::{CapabilitiesSource, implement_vertex, uniform};
use std::{collections::HashMap, sync::Arc};

use eyre::Result;

use glium::{Display, Surface, glutin::surface::WindowSurface};
use pollster::FutureExt;

use winit::{
    dpi::PhysicalSize,
    event_loop::{self, ActiveEventLoop},
    window::Window,
};

const VERTEX_SHADER_SRC: &str = r#"
    #version 330 core

    layout (location = 0) in vec3 position;
    layout (location = 1) in vec3 color;

    uniform mat4 matrix;

    smooth out vec3 vertex_color;

    void main() {
        gl_Position = matrix * vec4(position, 1.0);
        vertex_color = color;
    }
"#;

const FRAGMENT_SHADER_SRC: &str = r#"
    #version 330 core

    smooth in vec3 vertex_color;
    layout (location = 0) out vec4 color;

    void main() {
        color = vec4(vertex_color, 1.0);
    }
"#;

#[derive(Debug)]
pub struct RenderState {
    window: Window,
    device: Display<WindowSurface>,
    vertex_buffer: glium::vertex::VertexBuffer<Vertex>,
    program: glium::Program,
    ticks: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

implement_vertex!(Vertex, position, color);

impl RenderState {
    pub fn setup_state(event_loop: &ActiveEventLoop) -> RenderState {
        let (window, device) = glium::backend::glutin::SimpleWindowBuilder::new()
            .with_title("Alecto")
            .build(event_loop);
        let vertex_data = vec![
            Vertex {
                position: [0.0, 0.0, 0.0],
                color: [0.0, 0.0, 1.0],
            },
            Vertex {
                position: [1.0, 0.0, 0.0],
                color: [0.0, 1.0, 0.0],
            },
            Vertex {
                position: [0.0, 1.0, 0.0],
                color: [1.0, 0.0, 0.0],
            },
        ];
        let vertex_buffer = glium::vertex::VertexBuffer::new(&device, &vertex_data).unwrap();
        let program = glium::program::Program::from_source(
            &device,
            VERTEX_SHADER_SRC,
            FRAGMENT_SHADER_SRC,
            None,
        )
        .unwrap();
        RenderState {
            window,
            device,
            vertex_buffer,
            program,
            ticks: 0,
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {}

    pub fn request_redraw(&self) {
        self.window.request_redraw()
    }

    pub fn redraw(&mut self) -> Result<()> {
        let indices = glium::index::IndexBuffer::new(
            &self.device,
            glium::index::PrimitiveType::TrianglesList,
            &[0_u32, 1_u32, 2_u32],
        )
        .unwrap();
        self.ticks += 1;
        self.vertex_buffer.write(&[
            Vertex {
                position: [0.0, 0.5 + (0.01 * self.ticks as f32).cos() * 0.5, 0.0],
                color: [0.0, 0.0, 1.0],
            },
            Vertex {
                position: [1.0, 0.0, 0.0],
                color: [0.0, 1.0, 0.0],
            },
            Vertex {
                position: [0.0, 1.0, 0.0],
                color: [1.0, 0.0, 0.0],
            },
        ]);

        let uniforms = uniform! {
            matrix: [
                [2.0, 0.0, 0.0, 0.0],
                [0.0, -2.0, 0.0, 0.0],
                [0.0, 0.0, 2.0, 0.0],
                [-1.0, 1.0, 0.0, 1.0_f32],
            ]
        };

        let mut frame = self.device.draw();

        frame.clear_color(0.0, 0.0, 0.0, 0.0);
        frame
            .draw(
                &self.vertex_buffer,
                &indices,
                &self.program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
        frame.finish();
        Ok(())
    }
}
