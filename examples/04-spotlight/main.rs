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
use noire::render::{OpenGLWindow, RenderWindow, Window};

use notify::*;
use std::sync::mpsc::channel;
use std::time::{Duration, Instant};
use std::thread;
use std::thread::JoinHandle;

fn main() {
    let mut window = RenderWindow::create(1024, 1024, "Hello This is window")
        .expect("Failed to create Render Window");

    window.enable(Capability::DepthTest);

    // create shader program
    let vertex_file = String::from("./examples/04-spotlight/shaders/vertex.glsl");
    let fragment_file = String::from("./examples/04-spotlight/shaders/fragment.glsl");
    let mut program: Program = Program::compile_from_files(&vertex_file, &fragment_file).unwrap();

    // create vertex data
    let cube = Mesh::create_cube(Cube::create(0.75)).unwrap();
    let mut plane = Mesh::create_plane(Plane::create(10.0, 10.0)).unwrap();
    plane.translate(Vector3{ x: 0.0, y: -3.0, z: 0.0});

    let mut camera = Camera::new();
    camera
        .perspective(60.0, window.aspect(), 0.1, 80.0)
        .lookat(
            point3(0.0, 1.0, -2.5),
            point3(0.0, 0.0, 0.0),
            vec3(0.0, 1.0, 0.0)
        );
    let light_pos = vec3(-2.5, 0.0, 1.0);

    let start_time = Instant::now();

    loop {
        let now = Instant::now();
        let elapsed = now.duration_since(start_time);
        let elapsed = (elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9) as f32;

        // clear scene
        window.clear(0.0, 0.0, 0.0, 0.0);
        window.clear_depth(1.0);

        // set some basic shader uniform variables
        program.bind();
        program
            .uniform("u_cameraPos", camera.position.into())
            .uniform("u_resolution", window.get_framebuffer_size().into())
            .uniform("u_time", elapsed.into())
            .uniform("u_lightPos", light_pos.into());

        // render plane!
        let model_view = camera.view * plane.model_view;
        let model_view_proj = camera.projection * model_view;
        let normal_matrix: Matrix3<f32> = convert_to_matrix3(&model_view).invert().unwrap().transpose();

        program
            .uniform("u_modelView", model_view.into())
            .uniform("u_modelViewProjection", model_view_proj.into())
            .uniform("u_normalMatrix", normal_matrix.into())
            .uniform("u_objectColor", Color::rgb(0.4, 0.8, 0.25).into())
            .uniform("u_shininess", 4.0.into());

        plane.vao.bind();
        plane.vao.draw();
        plane.vao.unbind();

        // animate and render plane
        let rotate_x = Matrix4::from_angle_x(Rad::from(Deg(elapsed * 22.5)));
        let rotate_y = Matrix4::from_angle_y(Rad::from(Deg(elapsed * 45.0)));
        let model_view = camera.view * cube.model_view * rotate_y * rotate_x;;
        let model_view_proj = camera.projection * model_view;
        let normal_matrix: Matrix3<f32> = convert_to_matrix3(&model_view).invert().unwrap().transpose();

        program.uniform("u_modelView", model_view.into());
        program.uniform("u_modelViewProjection", model_view_proj.into());
        program.uniform("u_normalMatrix", normal_matrix.into());
        program.uniform("u_objectColor", Color::rgb(1.0, 1.0, 1.0).into());
        program.uniform("u_shininess", 32.0.into());

        cube.vao.bind();
        cube.vao.draw();
        cube.vao.unbind();

        // remove program
        program.unbind();

        // render scene
        window.swap_buffers();

        // handle events
        window.poll_events();
        if window.should_close() {
            return;
        }
    }
}
