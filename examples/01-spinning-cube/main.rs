#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

extern crate cgmath;
extern crate gl;
extern crate noire;
extern crate notify;

use gl::types::*;

use cgmath::*;

use noire::math::*;
use noire::math::color::Color;
use noire::math::camera::*;
use noire::mesh;
use noire::mesh::mesh::Mesh;
use noire::mesh::cube::Cube;
use noire::render::shader::*;
use noire::render::program::*;
use noire::render::traits::*;
use noire::render::vertex::*;
use noire::render::{Capability};
use noire::render::window::{OpenGLWindow,RenderWindow,Window};

use notify::*;
use std::sync::mpsc::channel;
use std::time::{Duration, Instant};
use std::thread;
use std::thread::JoinHandle;

fn main() {
    let mut window = RenderWindow::create(600, 600, "Hello This is window")
        .expect("Failed to create Render Window");

    window.enable(Capability::DepthTest);

    // create shader program
    let vertex_file = String::from("./examples/01-spinning-cube/shaders/vertex.glsl");
    let fragment_file = String::from("./examples/01-spinning-cube/shaders/fragment.glsl");
    let program: Program = Program::compile_from_files(&vertex_file, &fragment_file).unwrap();

    // create vertex data
    let mesh = Mesh::create(Cube::create(1.0));
    let vao = mesh.vao;

    let start_time = Instant::now();

    let mut camera = Camera::new();
    camera
        .perspective(60.0, window.aspect(), 0.1, 80.0)
        .lookat(
            point3(0.0, 1.0, -2.5),
            point3(0.0, 0.0, 0.0),
            vec3(0.0, 1.0, 0.0)
        );
    let light_pos = vec3(-4.0, 0.0, 2.0);

    loop {
        let now = Instant::now();
        let elapsed = now.duration_since(start_time);
        let elapsed = (elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9) as f32;

        window.clear(0.95, 0.95, 0.95, 1.0);
        window.clear_depth(1.0);

        let size = window.get_framebuffer_size();

        let rotate_x = Matrix4::from_angle_x(Rad::from(Deg(elapsed * 22.5)));
        let rotate_y = Matrix4::from_angle_y(Rad::from(Deg(elapsed * 45.0)));

        let model_view = camera.view * rotate_y * rotate_x;
        let model_view_proj = camera.projection * model_view;
        let normal_matrix: Matrix3<f32> = convert_to_matrix3(&model_view).invert().unwrap().transpose();

        program.bind();

        program.uniform("u_cameraPos", camera.position.into());
        program.uniform("u_resolution", size.into());
        program.uniform("u_time", elapsed.into());
        program.uniform("u_modelView", model_view.into());
        program.uniform("u_modelViewProjection", model_view_proj.into());
        program.uniform("u_normalMatrix", normal_matrix.into());
        program.uniform("u_lightPos", light_pos.into());

        vao.bind();
        vao.draw();
        vao.unbind();

        program.unbind();

        window.swap_buffers();

        window.poll_events();
        if window.should_close() {
            return;
        }
    }
}
