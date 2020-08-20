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
use noire::math::{Camera, Color};
use noire::mesh::{Cube, Mesh, Plane};
use noire::render::{Bindable, Capability, Drawable, Program, Shader, VertexArrayObject};
use noire::{core::Timer, render::{OpenGLWindow, RenderWindow, Size, Window}};

use notify::*;
use std::sync::mpsc::channel;
use std::time::{Duration, Instant};
use std::thread;
use std::thread::JoinHandle;
use utils::app_dir;

fn main() {
    let app_dir = app_dir().unwrap();
    let window_size = Size::new(600, 600);
    let mut window = RenderWindow::create(&window_size, "Hello This is window")
        .expect("Failed to create Render Window");

    window.enable(Capability::DepthTest);

    // create shader program
    let vertex_file = app_dir.join("examples/01-spinning-cube/shaders/vertex.glsl");
    let fragment_file = app_dir.join("examples/01-spinning-cube/shaders/fragment.glsl");
    let mut program: Program = Program::compile_from_files(&vertex_file, &fragment_file).unwrap();

    // create vertex data
    let mut mesh = Mesh::create_cube(Cube::create(1.0), Color::rgb(1.0, 1.0, 1.0)).unwrap();

    let timer = Timer::now();

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
        let elapsed = timer.elapsed_in_seconds() as f32;

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

        mesh.vao.bind();
        mesh.vao.draw();
        mesh.vao.unbind();

        program.unbind();

        window.swap_buffers();

        window.poll_events();
        if window.should_close() {
            return;
        }
    }
}
