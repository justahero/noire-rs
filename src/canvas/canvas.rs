use std::cell::RefCell;

use math::{Color, Rect, Vector2};
use render::{Program, Shader};

static VERTEX_SHADER: &str = r#"
#version 330

void main() {
    gl_Position = vec4(0.0, 0.0, 0.0, 0.1);
}
"#;

static FRAGMENT_SHADER: &str = r#"
#version 330

in vec4 in_color;
out vec4 out_color;

void main() {
    out_color = in_color;
}
"#;

pub struct Canvas2D {
    /// compiled shader program to render primitives
    program: Program,
    /// color to render the next primitive with
    draw_color: Color,
    /// store all line coordinates
    line_vertices: RefCell<Box<Vec<Vector2<f32>>>>,
}

impl Canvas2D {
    /// Create a new instance of the canvas
    pub fn new() -> Self {
        let vertex_shader = Shader::create_vertex(&VERTEX_SHADER).unwrap();
        let fragment_shader = Shader::create_fragment(&FRAGMENT_SHADER).unwrap();

        let program = Program::create(vertex_shader, fragment_shader).unwrap();

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
    pub fn draw_line(&self, start_x: f32, start_y: f32, end_x: f32, end_y: f32) -> &Self {
        let mut lines = self.line_vertices.borrow_mut();
        lines.push(Vector2::<f32>{ x: start_x, y: start_y });
        lines.push(Vector2::<f32>{ x: end_x, y: end_y });
        self
    }

    /// Draw a rect
    pub fn draw_rect(&self, rect: &Rect) -> &Self {
        self
    }

    /// Renders the content of the canvas.
    pub fn render(&self) {
        let mut lines = self.line_vertices.borrow_mut();
    }
}
