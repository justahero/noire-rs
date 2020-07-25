use std::cell::RefCell;

use math::{Color, Rect, Vector2};
use render::{Primitive, Program, Shader, VertexArrayObject, VertexBuffer};
use crate::render::{Bindable, Drawable};

static VERTEX_SHADER: &str = r#"
#version 330

in vec2 position;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
}
"#;

static FRAGMENT_SHADER: &str = r#"
#version 330

in vec4 in_color;
out vec4 out_color;

void main() {
    out_color = vec4(1.0, 0.0, 0.0, 1.0);
}
"#;

pub struct Canvas2D {
    /// compiled shader program to render primitives
    program: Program,
    /// color to render the next primitive with
    draw_color: Color,
    /// store all line coordinates
    line_vertices: RefCell<Box<Vec<f32>>>,
}

/// Compiles the used shader program
fn compile_program() -> Program {
    let vertex_shader = Shader::create_vertex(&VERTEX_SHADER).unwrap();
    let fragment_shader = Shader::create_fragment(&FRAGMENT_SHADER).unwrap();

    Program::create(vertex_shader, fragment_shader).unwrap()
}

impl Canvas2D {
    /// Create a new instance of the canvas
    pub fn new() -> Self {
        let program = compile_program();

        Canvas2D {
            program,
            draw_color: Color::BLACK,
            line_vertices: RefCell::new(Box::new(Vec::new())),
        }
    }

    /// Clears the canvas, sets it to given colors
    pub fn clear(&self, r: f32, g: f32, b: f32, a: f32) -> &Self {
        self
    }

    pub fn set_color(&mut self, color: Color) -> &Self {
        self.draw_color = color;
        self
    }

    /// Draws a point
    pub fn draw_point(&self, x: f32, y: f32) -> &Self {
        self
    }

    /// Draws a line
    pub fn draw_line(&self, start_x: i32, start_y: i32, end_x: i32, end_y: i32) -> &Self {
        let mut lines = self.line_vertices.borrow_mut();
        lines.push(start_x as f32);
        lines.push(start_y as f32);
        lines.push(end_x as f32);
        lines.push(end_y as f32);
        self
    }

    /// Draw a rect
    pub fn draw_rect(&self, rect: &Rect) -> &Self {
        self
    }

    /// Renders the content of the canvas.
    pub fn render(&mut self) {
        let lines = self.line_vertices.borrow();

        if !lines.is_empty() {
            // create buffers
            let vb = VertexBuffer::create(&lines[..], 2, Primitive::Lines);
            let mut vao = VertexArrayObject::new().unwrap();
            vao.add_vb(vb);

            // bind resources, uniforms, attributes
            self.program.bind();

            vao.bind();
            vao.draw();
            vao.unbind();

            // unbind resources
            self.program.unbind();
        }
    }
}
