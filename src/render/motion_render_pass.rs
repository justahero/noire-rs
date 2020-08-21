
use super::{Program, FrameBuffer, Shader, VertexArrayObject, Bindable, Texture, Uniform, Drawable};
use std::collections::VecDeque;

static VERTEX_SHADER: &str = r#"
#version 330

uniform vec2 u_resolution;

layout (location = 0) in vec2 position;

out vec2 vTexcoords;

void main() {
    vTexcoords = position;
    gl_Position = vec4(position, 0.0, 1.0);
}
"#;

static FRAGMENT_SHADER: &str = r#"
#version 330

uniform sampler2D u_texture0;

in vec2 vTexcoords;
out vec4 out_color;

void main() {
    out_color = vec4(vec3(1.0), 1.0);
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
    width: u32,
    /// The height of the render texture / frame buffer
    height: u32,
    /// The program to render the pass
    program: Program,
    /// The squad rectangle to render the full canvas.
    vao: VertexArrayObject,
    /// List of frame buffers / textures to render scene to
    render_targets: VecDeque<(Texture, FrameBuffer)>,
}

impl MotionRenderPass {
    /// The number of maximum frame buffers to sample canvas scene.
    pub const MAX_FRAME_BUFFERS: u32 = 1;

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

        (0..Self::MAX_FRAME_BUFFERS).for_each(|_i| {
            let mut frame_buffer = FrameBuffer::create().unwrap();
            let mut texture = Texture::create_2d(width, height).unwrap();
            texture.bind();
            frame_buffer.attach_texture(0, &texture).unwrap();
            render_targets.push_front((texture, frame_buffer));
        });

        Self {
            width,
            height,
            program,
            vao,
            render_targets,
        }
    }

    /// Returns the current
    pub fn current_framebuffer(&self) -> &FrameBuffer {
        let (_, frame_buffer) = self.render_targets.front().unwrap();
        frame_buffer
    }

    /// Sets the current frame buffer as render target
    ///
    pub fn set_render_target(&mut self) {
        let (_, frame_buffer) = self.render_targets.front_mut().unwrap();
        frame_buffer.bind();
    }

    /// Reset the frame buffer, render target is reset to back buffer
    ///
    pub fn reset(&mut self) {
        let (_, frame_buffer) = self.render_targets.front_mut().unwrap();
        frame_buffer.unbind();
        FrameBuffer::BACK.bind();
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
}

impl Bindable for MotionRenderPass {
    fn bind(&mut self) -> &mut Self {
        self.program.bind();
        self.program.uniform("u_resolution", Uniform::Float2(self.width as f32, self.height as f32));

        let (texture, _) = self.render_targets.front_mut().unwrap();
        self.program.sampler("u_texture0", 0, texture);

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
