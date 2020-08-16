use crate::math::Color;
use crate::render::{Primitive, Program, Shader, VertexBuffer};
use crate::render::{Uniform, Bindable, Drawable, vertex_buffer::VertexType, VertexAttributeDescriptor};

static VERTEX_SHADER: &str = r#"
#version 330

uniform vec2 u_resolution;
uniform float u_pointSize = 1.0;

layout(location = 0) in vec2 position;
layout(location = 1) in vec3 color;

out vec3 vColor;

void main() {
    float x = (-1.0) + 2.0 * (position.x / u_resolution.x);
    float y = (-1.0) + 2.0 * (position.y / u_resolution.y);

    vColor = color;
    gl_PointSize = u_pointSize;
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

fn generate_vao(vb: &mut VertexBuffer) -> u32 {
    let mut id = 0;

    unsafe { gl::GenVertexArrays(1, &mut id); }
    unsafe { gl::BindVertexArray(id); }

    vb.bind();

    let mut offset = 0;
    for attribute in &vb.attributes {
        unsafe {
            gl::VertexAttribPointer(
                attribute.location,
                attribute.components as i32,
                attribute.vertex_type.into(),
                gl::FALSE,
                vb.stride() as i32,
                offset as *const gl::types::GLvoid,
            );
            gl::EnableVertexAttribArray(attribute.location)
        }

        offset += attribute.stride();
    }

    unsafe { gl::BindVertexArray(0); }

    id
}

/// Helper struct to collect all vertices
struct VertexBatch {
    pub primitive: Primitive,
    pub vao: u32,
    pub vb: VertexBuffer,
    pub count: usize,
}

impl VertexBatch {
    pub fn new(primitive: Primitive, count: usize) -> Self {
        let attributes = vec![
            VertexAttributeDescriptor::new("position", VertexType::Float, 2, 0),
            VertexAttributeDescriptor::new("color", VertexType::Float, 3, 1),
        ];

        let mut vb = VertexBuffer::dynamic(count, attributes);
        let vao = generate_vao(&mut vb);

        VertexBatch {
            vao,
            vb,
            primitive,
            count,
        }
    }

    /// Appends the vertex data
    pub fn append(&mut self, data: &[f32]) {
        self.vb.write(&data, self.count);
        self.count += data.len() / self.vb.components() as usize;
    }

    /// Returns true if VertexBuffer is filled with vertex data to capacity
    pub fn filled(&self) -> bool {
        self.count >= self.vb.size() - self.vb.stride() as usize
    }

    fn bind(&self) {
        for attribute in &self.vb.attributes {
            unsafe { gl::EnableVertexAttribArray(attribute.location); }
        }
    }

    fn unbind(&self) {
        for attribute in &self.vb.attributes {
            unsafe { gl::DisableVertexAttribArray(attribute.location); }
        }
    }
}

impl Drawable for VertexBatch {
    fn draw(&mut self) {
        unsafe { gl::BindVertexArray(self.vao) };
        self.bind();
        if self.count > 0 {
            unsafe { gl::DrawArrays(self.primitive.into(), 0, self.count as i32); }
            self.count = 0;
        }
        self.unbind();
        unsafe { gl::BindVertexArray(0) };
    }
}

pub struct Canvas2D {
    /// Width of the Canvas2D
    pub width: u32,
    /// Height of the Canvas2D
    pub height: u32,
    /// compiled shader program to render primitives
    program: Program,
    /// color to render the next primitive with
    draw_color: Color,
    /// size of the point
    point_size: f32,
    /// VAO Buffer with vertex data for rects
    rects: VertexBatch,
    /// VAO Buffer with vertex data for lines
    lines: VertexBatch,
    /// VAO Buffer with vertex data for points
    points: VertexBatch,
}

/// Compiles the used shader program
fn compile_program() -> Program {
    let vertex_shader = Shader::create_vertex(&VERTEX_SHADER).unwrap();
    let fragment_shader = Shader::create_fragment(&FRAGMENT_SHADER).unwrap();

    Program::create(vertex_shader, fragment_shader).unwrap()
}

impl Canvas2D {
    /// Create a new instance of the canvas
    pub fn new(width: u32, height: u32) -> Self {
        let program = compile_program();

        let rects = VertexBatch::new(Primitive::Triangles, 512);
        let lines = VertexBatch::new(Primitive::Lines, 512);
        let points = VertexBatch::new(Primitive::Points, 512);

        Canvas2D {
            width,
            height,
            program,
            draw_color: Color::BLACK,
            point_size: 1.0,
            rects,
            lines,
            points,
        }
    }

    /// Sets the point size (if available)
    pub fn set_pointsize(&mut self, size: f32) {
        self.point_size = size;
    }

    /// Sets the color to render the next draw calls with
    pub fn set_color(&mut self, color: Color) {
        self.draw_color = color;
    }

    /// Draws a point
    pub fn draw_point(&mut self, x: f32, y: f32) {
        let c = &self.draw_color;
        let data = vec![x, y, c.r, c.g, c.b];

        self.points.append(&data);
        if self.points.filled() {
            self.points.draw();
        }
    }

    /// Draws a line
    pub fn draw_line(&mut self, start_x: f32, start_y: f32, end_x: f32, end_y: f32) {
        let c = &self.draw_color;
        let data = vec![
            start_x, start_y, c.r, c.g, c.b,
            end_x, end_y, c.r, c.g, c.b,
        ];

        self.lines.append(&data);
        if self.lines.filled() {
            self.lines.draw();
        }
    }

    /// Pushes the geometry for a rect, to be rendered
    pub fn draw_rect(&mut self, left: f32, top: f32, right: f32, bottom: f32) {
        let c = &self.draw_color;
        let data = vec![
            left, top, c.r, c.g, c.b,
            right, top, c.r, c.g, c.b,
            right, bottom, c.r, c.g, c.b,
            right, bottom, c.r, c.g, c.b,
            left, bottom, c.r, c.g, c.b,
            left, top, c.r, c.g, c.b,
        ];

        // not the most elegant solution, but should work okayish
        self.rects.append(&data);
        if self.rects.filled() {
            self.rects.draw();
        }
    }
}

impl Bindable for Canvas2D {
    fn bind(&mut self) -> &mut Self {
        self.program.bind();
        self.program.uniform("u_resolution", Uniform::Float2(self.width as f32, self.height as f32));
        self.program.uniform("u_pointSize", Uniform::Float(self.point_size));
        self
    }

    fn unbind(&mut self) -> &mut Self {
        self.rects.draw();
        self.lines.draw();
        self.program.unbind();
        self
    }

    fn bound(&self) -> bool {
        todo!()
    }
}
