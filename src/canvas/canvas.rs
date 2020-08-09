use std::cell::RefCell;

use crate::math::Color;
use crate::render::{Primitive, Program, Shader, VertexArrayObject, VertexBuffer};
use crate::render::{Bindable, Drawable, Uniform};

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
    /// store all point coordinates with colors, components: (x,yr,g,b)
    point_vertices: RefCell<Box<Vec<f32>>>,
    /// store all line coordinates with colors, components: (x,y,r,g,b)
    line_vertices: RefCell<Box<Vec<f32>>>,

    /// VAO to hold vertices for rects
    vao: VertexArrayObject,
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

        let vb = VertexBuffer::dynamic(6, &vec![2, 3]);
        let mut vao = VertexArrayObject::new(Primitive::Triangles);
        vao.add_vb(vb);

        Canvas2D {
            width,
            height,
            program,
            draw_color: Color::BLACK,
            point_size: 1.0,
            point_vertices: RefCell::new(Box::new(Vec::new())),
            line_vertices: RefCell::new(Box::new(Vec::new())),
            vao,
        }
    }

    /// Clears the canvas, sets it to given colors
    pub fn clear(&self, _r: f32, _g: f32, _b: f32, _a: f32) -> &Self {
        self
    }

    /// Sets the point size (if available)
    pub fn set_pointsize(&mut self, size: f32) -> &Self {
        self.point_size = size;
        self
    }

    /// Sets the color to render the next draw calls with
    pub fn set_color(&mut self, color: Color) -> &Self {
        self.draw_color = color;
        self
    }

    /// Draws a point
    pub fn draw_point(&self, x: f32, y: f32) -> &Self {
        let mut points = self.point_vertices.borrow_mut();
        points.push(x);
        points.push(y);
        points.append(&mut self.draw_color.rgb_vec());
        self
    }

    /// Draws a line
    pub fn draw_line(&self, start_x: f32, start_y: f32, end_x: f32, end_y: f32) -> &Self {
        let mut lines = self.line_vertices.borrow_mut();
        lines.push(start_x);
        lines.push(start_y);
        lines.append(&mut self.draw_color.rgb_vec());
        lines.push(end_x);
        lines.push(end_y);
        lines.append(&mut self.draw_color.rgb_vec());
        self
    }

    /// Pushes the geometry for a rect, to be rendered
    pub fn draw_rect(&mut self, left: f32, top: f32, right: f32, bottom: f32) {
        /*
        let mut rects = self.rect_vertices.borrow_mut();
        rects.push(left);
        rects.push(top);
        rects.append(&mut self.draw_color.rgb_vec());
        rects.push(right);
        rects.push(top);
        rects.append(&mut self.draw_color.rgb_vec());
        rects.push(right);
        rects.push(bottom);
        rects.append(&mut self.draw_color.rgb_vec());
        rects.push(right);
        rects.push(bottom);
        rects.append(&mut self.draw_color.rgb_vec());
        rects.push(left);
        rects.push(bottom);
        rects.append(&mut self.draw_color.rgb_vec());
        rects.push(left);
        rects.push(top);
        rects.append(&mut self.draw_color.rgb_vec());
        self
        */

        // TODO draw rect immediately and check if this works well enough

        let c = &self.draw_color;
        let data = vec![
            left, top, c.r, c.g, c.b,
            right, top, c.r, c.g, c.b,
            right, bottom, c.r, c.g, c.b,
            right, bottom, c.r, c.g, c.b,
            left, bottom, c.r, c.g, c.b,
            left, top, c.r, c.g, c.b,
        ];

        let vb = self.vao.get_vb(0);
        vb.borrow_mut().write(&data);

        self.vao.draw();
    }

    /// Renders the content of the canvas.
    /// The function resizes the Renderbuffer if the framebuffer size is different
    pub fn render(&mut self) {
        // self.render_rects();
        // self.render_lines();
        // self.render_points();
    }

    /// Renders all points
    fn _render_points(&mut self) {
        /*
        let mut points = self.point_vertices.borrow_mut();

        if !points.is_empty() {
            let vertex_data = VertexData::new(&points[..], &[2, 3], VertexType::Float);
            let vb = VertexBuffer::new(&vertex_data);

            // create buffers
            let mut vao = VertexArrayObject::new(Primitive::Points);
            vao.add_vb(vb);

            vao.bind();
            vao.draw();
            vao.unbind();

            points.clear();
        }
        */
    }

    /// Renders all lines using VertexBuffer and VAO
    fn _render_lines(&mut self) {
        /*
        let mut lines = self.line_vertices.borrow_mut();

        if !lines.is_empty() {
            let vertex_data = VertexData::new(&lines[..], &[2, 3], VertexType::Float);
            let vb = VertexBuffer::new(&vertex_data);

            // create buffers
            let mut vao = VertexArrayObject::new(Primitive::Lines);
            vao.add_vb(vb);

            vao.bind();
            vao.draw();
            vao.unbind();

            lines.clear();
        }
        */
    }
}

impl Bindable for Canvas2D {
    fn bind(&mut self) -> &mut Self {
        self.vao.bind();
        self.program.bind();
        self.program.uniform("u_resolution", Uniform::Float2(self.width as f32, self.height as f32));
        self.program.uniform("u_pointSize", Uniform::Float(self.point_size));
        self
    }

    fn unbind(&mut self) -> &mut Self {
        self.program.unbind();
        self.vao.unbind();
        self
    }

    fn bound(&self) -> bool {
        todo!()
    }
}
