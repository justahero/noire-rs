use crate::render::{Drawable, VertexArrayObject, VertexBuffer, Point2, Primitive, RenderError, Size, Program, ProgramError, Shader, ShaderType, Texture, Uniform, Bindable, Capability, RenderWindow, OpenGLWindow, VertexAttributeDescriptor, vertex_buffer::VertexType};

pub struct ScreenRect {
    /// holds all vertex information
    pub vao: VertexArrayObject,
    /// the program to render the Quad with
    pub program: Program,
}

fn create_vertex_shader() -> Shader {
    let source = r###"
    #version 330
    layout(location = 0) in vec2 position;
    layout(location = 1) in vec2 texcoord;
    out vec2 texCoord;

    void main(void) {
        texCoord = texcoord;
        gl_Position = vec4(position, 0.0, 1.0);
    }
    "###;

    Shader::create(source, ShaderType::Vertex).unwrap()
}

fn create_fragment_shader() -> Shader {
    let source = r###"
    #version 330
    uniform sampler2D diffuse;
    uniform float znear;
    uniform float zfar;
    in vec2 texCoord;
    out vec4 out_color;
    float linearDepth(vec2 uv) {
        float depth = texture(diffuse, uv).r;
        return (2.0 * znear) / (zfar + znear - depth * (zfar - znear));
    }
    void main(void) {
        float color = linearDepth(texCoord);
        out_color = vec4(vec3(color), 1.0);
    }
    "###;

    Shader::create(source, ShaderType::Fragment).unwrap()
}

fn create_program() -> Result<Program, ProgramError> {
    let vertex_shader = create_vertex_shader();
    let fragment_shader = create_fragment_shader();

    Program::create(vertex_shader, fragment_shader)
}

fn create_vertices() -> Vec<f32> {
    vec![
       -1.0,  1.0,
       -1.0, -1.0,
        1.0,  1.0,
       -1.0, -1.0,
        1.0, -1.0,
        1.0,  1.0,
    ]
}

fn create_texcoords() -> Vec<f32> {
    vec![
        0.0,  1.0,
        0.0,  0.0,
        1.0,  1.0,
        0.0,  0.0,
        1.0,  0.0,
        1.0,  1.0,
    ]
}

impl ScreenRect {
    pub fn new() -> Result<Self, RenderError> {
        let mut vao = VertexArrayObject::new(Primitive::Triangles);

        let vertices = VertexAttributeDescriptor::new("position", VertexType::Float, 2, 0);
        let texcoords = VertexAttributeDescriptor::new("texcoord", VertexType::Float, 2, 1);

        vao.add_vb(VertexBuffer::create(&create_vertices(), vec![vertices]));
        vao.add_vb(VertexBuffer::create(&create_texcoords(), vec![texcoords]));

        let program = create_program()?;

        Ok(ScreenRect {
            vao,
            program,
        })
    }

    /// Renders the screen rect
    pub fn render(&mut self, window: &RenderWindow, point: &Point2<u32>, size: &Size<u32>, texture: &mut Texture) -> &mut Self {
        texture.bind();
        self.bind();
        window.set_viewport(&point, &size);
        window.disable(Capability::DepthTest);

        self.program.uniform("diffuse", Uniform::Integer(0).into());
        self.program.uniform("znear", 0.1.into());
        self.program.uniform("zfar", 25.0.into());
        self.vao.draw();

        window.enable(Capability::DepthTest);
        window.reset_viewport();
        self.unbind();
        texture.unbind();

        self
    }
}

impl Bindable for ScreenRect {
    fn bind(&mut self) -> &mut Self {
        self.program.bind();
        self.vao.bind();
        self
    }

    fn unbind(&mut self) -> &mut Self {
        self.vao.unbind();
        self.program.unbind();
        self
    }

    fn bound(&self) -> bool {
        return true;
    }
}
