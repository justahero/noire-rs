use std::cell::RefCell;

use math::{Color, Rect};
use render::{Primitive, Program, Shader, VertexArrayObject, VertexBuffer};
use crate::render::{Bindable, Drawable, Size, Uniform, vertex_buffer::{VertexType, VertexData}};

static VERTEX_SHADER: &str = r#"
#version 330

uniform vec2 u_resolution;

layout(location = 0) in vec2 position;
layout(location = 1) in vec3 color;

out vec3 vColor;

void main() {
    float x = (-1.0) + 2.0 * (position.x / u_resolution.x);
    float y = (-1.0) + 2.0 * (position.y / u_resolution.y);

    vColor = color;
    gl_Position = vec4(x, y, 0.0, 1.0);
}
"#;

static FRAGMENT_SHADER: &str = r#"
#version 330

in vec3 vColor;
out vec4 out_color;

void main() {
    out_color = vec4(vColor, 1.0);
}
"#;

pub struct Canvas2D {
    /// compiled shader program to render primitives
    program: Program,
    /// color to render the next primitive with
    draw_color: Color,
    /// store all line coordinates with colors with components: (x,y,r,g,b)
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
    pub fn clear(&self, _r: f32, _g: f32, _b: f32, _a: f32) -> &Self {
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
        lines.push(self.draw_color.red);
        lines.push(self.draw_color.green);
        lines.push(self.draw_color.blue);
        lines.push(end_x as f32);
        lines.push(end_y as f32);
        lines.push(self.draw_color.red);
        lines.push(self.draw_color.green);
        lines.push(self.draw_color.blue);
        self
    }

    /// Draw a rect
    pub fn draw_rect(&self, _rect: &Rect) -> &Self {
        self
    }

    /// Renders the content of the canvas.
    pub fn render(&mut self, size: &Size<u32>) {
        let mut lines = self.line_vertices.borrow_mut();

        if !lines.is_empty() {
            let vertex_data = VertexData::new(&lines[..], &[2, 3], VertexType::Float);
            let vb = VertexBuffer::new(&vertex_data);

            // create buffers
            let mut vao = VertexArrayObject::new(Primitive::Lines).unwrap();
            vao.add_vb(vb);

            // bind resources, uniforms, attributes
            self.program.bind();
            self.program.uniform("u_resolution", Uniform::Float2(size.width as f32, size.height as f32));

            vao.bind();
            vao.draw();
            vao.unbind();

            // unbind resources
            self.program.unbind();

            lines.clear();
        }
    }
}
