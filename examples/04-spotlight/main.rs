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
use noire::render::opengl::get_render_error;
use noire::render::{Bindable, Drawable, FrameBuffer, Program, Shader, Texture, VertexArrayObject};
use noire::render::{Capability, Primitive, Size};
use noire::render::{OpenGLWindow, RenderWindow, Window};
use noire::render::spot_light::*;

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

    let light_pos = point3(-2.5, 0.0, 1.0);

    // create vertex data
    let cube = Mesh::create_cube(Cube::create(0.75)).unwrap();
    let mut plane = Mesh::create_plane(Plane::create(10.0, 10.0)).unwrap();
    plane.translate(Vector3{ x: 0.0, y: -3.0, z: 0.0});

    let mut camera = Camera::new();
    camera
        .perspective(60.0, window.aspect(), 0.1, 80.0)
        .lookat(
            point3(0.0, 4.0, -3.5),
            point3(0.0, 0.0, 0.0),
            vec3(0.0, 1.0, 0.0)
        );

    let light_pos = point3(-2.5, 0.0, 1.0);

    let mut spot_light = Spotlight::new();
    spot_light.set_lookat(
        light_pos.clone(),
        point3(0.5, 0.0, 1.0),
        vec3(0.0, 1.0, 0.0),
    );

    // Textures & Frame Buffers
    let light_texture_size = Size{ width: 1024, height: 1024 };
    let mut light_texture = Texture::create2d().unwrap();
    light_texture.bind();
    light_texture.set_size(&light_texture_size).unwrap();
    light_texture.clamp_to_edge();
    light_texture.nearest();
    light_texture.unbind();

    let mut light_depth_texture = Texture::create_depth_texture().unwrap();
    light_depth_texture.bind();
    light_depth_texture.set_size(&light_texture_size).unwrap();
    light_depth_texture.clamp_to_edge();
    light_depth_texture.nearest();
    light_depth_texture.unbind();

    let mut light_frame_buffer = FrameBuffer::create().unwrap();
    light_frame_buffer.bind();
    light_frame_buffer.set_texture(&light_texture).expect("Set texture failed");
    light_frame_buffer.set_depth_buffer(&light_depth_texture).expect("Set depth buffer failed");
    light_frame_buffer.unbind();

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
            .uniform("u_shininess", 64.0.into());

        plane.vao.bind();
        plane.vao.draw();
        plane.vao.unbind();

        // animate and render plane
        let rotate_x = Matrix4::from_angle_x(Rad::from(Deg(elapsed * 22.5)));
        let rotate_y = Matrix4::from_angle_y(Rad::from(Deg(elapsed * 45.0)));

        let model_view = camera.view * cube.model_view * rotate_y * rotate_x;
        let model_view_proj = camera.projection * model_view;
        let normal_matrix: Matrix3<f32> = convert_to_matrix3(&model_view).invert().unwrap().transpose();

        program.uniform("u_modelView", model_view.into());
        program.uniform("u_modelViewProjection", model_view_proj.into());
        program.uniform("u_normalMatrix", normal_matrix.into());
        program.uniform("u_objectColor", Color::rgb(0.2, 0.5, 0.95).into());
        program.uniform("u_shininess", 16.0.into());

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
