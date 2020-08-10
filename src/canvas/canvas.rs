use std::cell::RefCell;

use crate::math::Color;
use crate::render::{Primitive, Program, Shader, VertexBuffer};
use crate::render::{Uniform, Bindable, VertexArrayObject, Drawable};

static VERTEX_SHADER: &str = r#"
#version 330

uniform vec2 u_resolution;
uniform float u_pointSize = 1.0;

layout(location = 0) in vec3 position;
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

/// Helper struct to collect all vertices
struct DrawBatch {
    pub vao: VertexArrayObject,
    pub count: usize,
}

impl DrawBatch {
    pub fn new(primitive: Primitive, count: usize) -> Self {
        let vb = VertexBuffer::dynamic(count, vec![3, 3]);
        let mut vao = VertexArrayObject::new(primitive);
        vao.add_vb(vb);

        DrawBatch {
            vao,
            count,
        }
    }

    /// Appends the vertex data
    pub fn append(&mut self, data: &[f32]) {
        if let Some(vb) = self.vao.get_vb_mut(0) {
            vb.write_offset(&data, self.count);
            self.count += data.len() / 6;
        }
    }

    /// Returns true if VertexBuffer is filled with vertex data to capacity
    pub fn filled(&self) -> bool {
        if let Some(vb) = self.vao.get_vb(0) {
            self.count >= vb.size()
        } else {
            false
        }
    }
}

impl Drawable for DrawBatch {
    fn draw(&mut self) {
        if self.count > 0 {
            unsafe {
                gl::DrawArrays(self.vao.primitive.into(), 0, self.count as i32);
            }
        }
        self.count = 0;
    }
}

impl Bindable for DrawBatch {
    fn bind(&mut self) -> &mut Self {
        self.vao.bind();
        self
    }

    fn unbind(&mut self) -> &mut Self {
        // render any outstanding vertices
        self.draw();
        self.vao.unbind();
        self
    }

    fn bound(&self) -> bool {
        todo!()
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

    rects: DrawBatch,

    /// The number of shapes to render
    shapes_count: usize,
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

        let rects = DrawBatch::new(Primitive::Triangles, 6 * 2048);

        Canvas2D {
            width,
            height,
            program,
            draw_color: Color::BLACK,
            point_size: 1.0,
            rects,
            shapes_count: 0,
        }
    }

    /// Clears the canvas, sets it to given colors
    pub fn clear(&self, _r: f32, _g: f32, _b: f32, _a: f32) {
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
    pub fn draw_point(&self, x: f32, y: f32) {
        /*
        let mut points = self.point_vertices.borrow_mut();
        points.push(x);
        points.push(y);
        points.append(&mut self.draw_color.rgb_vec());
        self
        */
    }

    /// Draws a line
    pub fn draw_line(&self, start_x: f32, start_y: f32, end_x: f32, end_y: f32) {
        /*
        let mut lines = self.line_vertices.borrow_mut();
        lines.push(start_x);
        lines.push(start_y);
        lines.append(&mut self.draw_color.rgb_vec());
        lines.push(end_x);
        lines.push(end_y);
        lines.append(&mut self.draw_color.rgb_vec());
        self
        */
    }

    /// Pushes the geometry for a rect, to be rendered
    pub fn draw_rect(&mut self, left: f32, top: f32, right: f32, bottom: f32) {
        let c = &self.draw_color;
        let data = vec![
            left, top, self.zoffset(), c.r, c.g, c.b,
            right, top, self.zoffset(), c.r, c.g, c.b,
            right, bottom, self.zoffset(), c.r, c.g, c.b,
            right, bottom, self.zoffset(), c.r, c.g, c.b,
            left, bottom, self.zoffset(), c.r, c.g, c.b,
            left, top, self.zoffset(), c.r, c.g, c.b,
        ];

        // not the most elegant solution, but should work okayish
        self.rects.append(&data);
        if self.rects.filled() {
            self.rects.draw();
        }

        self.inc_shapes();
    }

    fn inc_shapes(&mut self) {
        self.shapes_count += 1;
    }

    /// Returns the next z value
    fn zoffset(&self) -> f32 {
        1.0 / ((self.shapes_count + 1) as f32)
    }
}

impl Bindable for Canvas2D {
    fn bind(&mut self) -> &mut Self {
        self.rects.bind();
        self.program.bind();
        self.program.uniform("u_resolution", Uniform::Float2(self.width as f32, self.height as f32));
        self.program.uniform("u_pointSize", Uniform::Float(self.point_size));
        self
    }

    fn unbind(&mut self) -> &mut Self {
        self.rects.unbind();
        self.program.unbind();
        // reset the count value
        self.shapes_count = 0;
        self
    }

    fn bound(&self) -> bool {
        todo!()
    }
}
