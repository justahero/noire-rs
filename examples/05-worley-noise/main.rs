#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

extern crate cgmath;
extern crate gl;
extern crate noire;
extern crate notify;

use gl::types::*;

use noire::canvas::Canvas2D;
use noire::math::{Color, PerlinNoise, random_f32, Rect};
use noire::{core::Timer, render::{OpenGLWindow, RenderWindow, Size, Window, Capability, Program, VertexArrayObject, Bindable, Drawable, Uniform}};
use std::time::Instant;
use cgmath::{Vector3, Vector2, Matrix3, InnerSpace, Rad, Matrix4, Vector4, Deg};
use utils::app_dir;

fn main() {
    let app_dir = app_dir().unwrap();
    let window_size = Size::new(800, 800);
    let mut window = RenderWindow::create(&window_size, "Hello This is window")
        .expect("Failed to create Render Window");

    // create shader program
    let vertex_file = app_dir.join("examples/05-worley-noise/shaders/vertex.glsl");
    let fragment_file = app_dir.join("examples/05-worley-noise/shaders/fragment.glsl");
    let mut program = Program::compile_from_files(&vertex_file, &fragment_file).unwrap();

    // create vertex data
    let mut vao = VertexArrayObject::screen_rect();

    let timer = Timer::now();

    // randomly generate feature points
    let num_points = 500;
    let mut points: Vec<Vector3<f32>> = (0..num_points).into_iter().map( |_| {
        Vector3::new(random_f32(1.0), random_f32(1.0), random_f32(1.0))
    }).collect();

    let translation_matrix = Matrix4::<f32>::from_translation(Vector3::new(0.5, 0.5, 0.5));

    loop {
        let elapsed = timer.elapsed_in_seconds() as f32;

        // animate all points
        let depth = 0.5 + ((0.5 * elapsed).cos() * 0.4);
        let axis = Vector3::<f32>::new(0.5, 0.0, -0.5).normalize();
        let rotation_matrix = Matrix4::from_axis_angle(axis, Deg((0.125 * elapsed).sin()));
        let view = translation_matrix * rotation_matrix;
        for p in &mut points {
            let v = rotation_matrix * Vector4::new(p.x - 0.5, p.y - 0.5, p.z - 0.5, 1.0);
            p.x = v.x + 0.5;
            p.y = v.y + 0.5;
            p.z = v.z + 0.5;
        }

        let size = window.get_framebuffer_size();
        window.reset_viewport();
        window.clear(0.3, 0.3, 0.3, 1.0);

        program.bind();
        program.uniform("u_resolution", size.into());
        program.uniform("u_depth", depth.into());
        program.uniform("u_time", elapsed.into());

        let verts = points
            .iter()
            .map(|v| [v.x, v.y, v.z])
            .collect::<Vec<[f32; 3]>>();
        program.uniform("u_featurePoints[0]", Uniform::Vec3Array(verts));

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
