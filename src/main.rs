extern crate gl;
extern crate glfw;

mod noire;

use glfw::{Action, Context, Key};
use noire::shader::*;
use noire::program::*;
use noire::traits::*;
use noire::vertex::*;

use std::time::Instant;

static VS_SRC: &'static str = r##"
varying vec2 vUV;

attribute vec2 position;

void main(void) {
  vUV = position;
  gl_Position = vec4(position.xy, 0.0, 1.0);
}
"##;

static FS_SRC: &'static str = r##"
#define PI 3.14159265359

uniform vec2 u_resolution;
uniform float u_time;

float opUnion(float d1, float d2) {
  return max(d1, d2);
}

float opSubtract(float d1, float d2) {
  return max(-d1, d2);
}

float opIntersect(float d1, float d2) {
  return min(d1, d2);
}

vec2 opRepetition(vec2 p, vec2 c) {
  return mod(p, c) - 0.5 * c;
}

vec2 rotate(in vec2 st, in float angle) {
  vec2 result = st - 0.5;
  result = mat2(cos(angle), -sin(angle),
                sin(angle),  cos(angle)) * result;
  result += 0.5;
  return result;
}

vec2 tile(in vec2 st, in vec2 zoom) {
  return fract(st * zoom);
}

vec2 grid(in vec2 st, in vec2 zoom) {
  return floor(st * zoom);
}

float circle(in vec2 st, float radius) {
  vec2 dist = st - vec2(0.5);
  return 1.0 - smoothstep(radius - (radius * 0.01),
                          radius + (radius * 0.01),
                          dot(st, st) * 4.0);
}

float ring(in vec2 st, float radius, float inner) {
  float d1 = circle(st, radius);
  float d2 = circle(st, radius - inner);
  return opSubtract(d1, d2);
}

void main() {
  vec2 st = gl_FragCoord.xy / u_resolution;

  st = st * vec2(12.0);

  float d = 0.0;
  d = ring(opRepetition(st + vec2(2.5, 2.0), vec2(5.0, 4.0)), 2.5, 0.1);
  d = opUnion(d, ring(opRepetition(st + vec2(0.0), vec2(5.0, 4.0)), 2.5, 0.1));

  d = opUnion(d, circle(opRepetition(st + vec2(0.75), vec2(1.5)), 0.25));
  d = opUnion(d, circle(opRepetition(st + vec2(1.50), vec2(1.5)), 0.25));

  vec3 color = vec3(d);

  gl_FragColor = vec4(color, 1.0);
}
"##;

static VERTICES: [f32; 6] = [0.0, 0.5, 0.5, -0.5, -0.5, -0.5];

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    let (mut window, events) =
        glfw.create_window(400, 300, "Hello This is window", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window");

    // load gl functions
    gl::load_with(|s| window.get_proc_address(s) as *const _);

    window.set_key_polling(true);
    window.make_current();
    glfw.set_swap_interval(glfw::SwapInterval::None);

    // initialize GL shader stuff
    let vertex_shader = Shader::create(VS_SRC, ShaderType::VertexShader).unwrap();
    let pixel_shader = Shader::create(FS_SRC, ShaderType::PixelShader).unwrap();

    let program = Program::create(vertex_shader, pixel_shader).unwrap();
    let vertex_buffer = VertexBuffer::create(&VERTICES, 2);

    let mut vao = VertexArrayObject::new();
    vao.add_vb(vertex_buffer);

    let start_time = Instant::now();

    while !window.should_close() {
        let now = Instant::now();
        let elapsed = now.duration_since(start_time);
        let elapsed = (elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9) as f32;

        program.apply();
        program.uniform2f("u_resolution", 400.0, 300.0);
        program.uniform1f("u_time", elapsed);

        unsafe {
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        vao.draw();

        window.swap_buffers();

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        _ => {}
    }
}
