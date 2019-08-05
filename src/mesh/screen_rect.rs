use render::{Drawable, VertexArrayObject, VertexBuffer};
use render::{Point2, Primitive, RenderError, Size};
use render::{Program, ProgramError, Shader, ShaderType, Texture, Uniform};
use render::{Bindable, Capability, RenderWindow, OpenGLWindow};

pub struct ScreenRect<'a> {
    /// holds all vertex information
    pub vao: VertexArrayObject,
    /// the program to render the Quad with
    pub program: Program,
    /// The texture reference to render
    pub texture: &'a Texture,
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

impl<'a> ScreenRect<'a> {
    pub fn create(texture: &'a Texture) -> Result<Self, RenderError> {
        let mut vao = VertexArrayObject::new()?;

        vao.add_vb(VertexBuffer::create(&create_vertices(), 2, Primitive::Triangles));
        vao.add_vb(VertexBuffer::create(&create_texcoords(), 2, Primitive::Triangles));

        let program = match create_program() {
            Ok(program) => program,
            Err(err) => return Err(RenderError{ message: err.message }),
        };

        Ok(ScreenRect {
            vao,
            program,
            texture,
        })
    }

    /// Renders the screen rect
    pub fn render(&mut self, window: &RenderWindow, point: &Point2<u32>, size: &Size<u32>) -> &mut Self {
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

        self
    }
}

impl<'a> Bindable for ScreenRect<'a> {
    fn bind(&self) -> &Self {
        self.texture.bind();
        self.program.bind();
        self.vao.bind();
        self
    }

    fn unbind(&self) -> &Self {
        self.vao.unbind();
        self.program.unbind();
        self.texture.unbind();
        self
    }

    fn bound(&self) -> bool {
        return true;
    }
}
