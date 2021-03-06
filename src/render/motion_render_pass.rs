
use super::{Program, FrameBuffer, Shader, VertexArrayObject, Bindable, Texture, Drawable, Format, Uniform};
use std::collections::VecDeque;

static VERTEX_SHADER: &str = r#"
#version 330

layout (location = 0) in vec2 position;

out vec2 vTexcoords;

void main() {
    vTexcoords = 0.5 + position * 0.5;
    gl_Position = vec4(position, 0.0, 1.0);
}
"#;

static FRAGMENT_SHADER: &str = r#"
#version 330

uniform float sampleSize;

uniform sampler2D u_texture0;
uniform sampler2D u_texture1;
uniform sampler2D u_texture2;
uniform sampler2D u_texture3;
uniform sampler2D u_texture4;
uniform sampler2D u_texture5;

in vec2 vTexcoords;
out vec4 out_color;

void main() {
    // TODO find nicer decay values.
    vec4 c0 = texture(u_texture0, vTexcoords) * 1.0 / sampleSize;
    vec4 c1 = texture(u_texture1, vTexcoords) * 1.0 / sampleSize;
    vec4 c2 = texture(u_texture2, vTexcoords) * 1.0 / sampleSize;
    vec4 c3 = texture(u_texture3, vTexcoords) * 1.0 / sampleSize;
    vec4 c4 = texture(u_texture4, vTexcoords) * 1.0 / sampleSize;
    vec4 c5 = texture(u_texture5, vTexcoords) * 1.0 / sampleSize;

    out_color = c0 + c1 + c2 + c3 + c4 + c5;
}
"#;

/// Compiles the used shader program
fn compile_program(vertex: &str, fragment: &str) -> Program {
    let vertex_shader = Shader::create_vertex(vertex).unwrap();
    let fragment_shader = Shader::create_fragment(fragment).unwrap();

    Program::create(vertex_shader, fragment_shader).unwrap()
}

pub struct MotionRenderPass {
    /// The width of the render texture / frame buffer
    pub width: u32,
    /// The height of the render texture / frame buffer
    pub height: u32,
    /// The program to render the pass
    program: Program,
    /// The squad rectangle to render the full canvas.
    vao: VertexArrayObject,
    /// List of frame buffers / textures to render scene to
    render_targets: VecDeque<(Texture, FrameBuffer)>,
}

impl MotionRenderPass {
    /// The number of maximum frame buffers to sample canvas scene.
    pub const MAX_SAMPLES: u32 = 6;

    /// Creates a new instance of the render pass.
    ///
    /// ## Parameters
    /// * `width` - The width of the render texture
    /// * `height` - The height of the render texture
    ///
    pub fn new(width: u32, height: u32) -> Self {
        let program = compile_program(VERTEX_SHADER, FRAGMENT_SHADER);
        let vao = VertexArrayObject::screen_rect();
        let mut render_targets = VecDeque::new();

        (0..Self::MAX_SAMPLES).for_each(|unit| {
            let mut frame_buffer = FrameBuffer::create().unwrap();
            let mut texture = Texture::create_2d(width, height, Format::RGB, unit).unwrap();
            texture.bind();
            frame_buffer.attach_texture(0, &texture).unwrap();
            render_targets.push_back((texture, frame_buffer));
        });

        Self {
            width,
            height,
            program,
            vao,
            render_targets,
        }
    }

    /// Returns the number of used samples / render textures
    pub fn num_samples(&self) -> u32 {
        Self::MAX_SAMPLES
    }

    /// Returns the current frame buffer
    pub fn current_framebuffer(&self) -> &FrameBuffer {
        let (_, frame_buffer) = self.render_targets.front().unwrap();
        frame_buffer
    }

    /// Sets the current frame buffer as render target
    ///
    pub fn set_render_target(&mut self) {
        self.cycle_render_target();

        let (_, frame_buffer) = self.render_targets.front_mut().unwrap();
        frame_buffer.bind();
    }

    /// Reset the frame buffer, render target is reset to back buffer
    ///
    pub fn reset(&mut self) {
        let (_, frame_buffer) = self.render_targets.front_mut().unwrap();
        frame_buffer.unbind();
    }

    /// Renders the scene with motion blur pass by sampling the last few rendered frames.
    ///
    pub fn draw(&mut self) {
        self.bind();

        self.vao.bind();
        self.vao.draw();
        self.vao.unbind();

        self.unbind();
    }

    /// Cycles the frame buffers / textures in the queue
    fn cycle_render_target(&mut self) {
        let item = self.render_targets.pop_back().unwrap();
        self.render_targets.push_front(item);
    }
}

impl Bindable for MotionRenderPass {
    fn bind(&mut self) -> &mut Self {
        self.program.bind();
        self.program.uniform("sampleSize", Uniform::Float(Self::MAX_SAMPLES as f32));

        for (unit, (texture, _)) in self.render_targets.iter_mut().enumerate() {
            texture.bind();
            self.program.sampler(&format!("u_texture{}", unit), unit as u32, texture);
        }

        self
    }

    fn unbind(&mut self) -> &mut Self {
        self.program.unbind();
        self
    }

    fn bound(&self) -> bool {
        todo!()
    }
}
