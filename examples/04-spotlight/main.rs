#![allow(dead_code)]
#![allow(unused_variables)]

extern crate cgmath;
extern crate gl;
extern crate noire;
extern crate notify;

use cgmath::*;
use std::time::Instant;

use noire::math::*;
use noire::math::{Camera, Color};
use noire::mesh::{Cube, Mesh, Node, Plane, Scene};
use noire::render::{FrameBuffer, Program, Spotlight, Texture};
use noire::render::traits::*;
use noire::render::{Capability, CullMode, Point2, Size};
use noire::render::{OpenGLWindow, RenderWindow, Window};

fn main() {
    let window_size = Size::new(1024, 1024);
    let mut window = RenderWindow::create(&window_size, "Hello This is window")
        .expect("Failed to create Render Window");

    window.enable(Capability::DepthTest);
    window.enable(Capability::CullFace);
    window.set_cullmode(CullMode::Back);

    // create shader program
    let vertex_file = String::from("./examples/04-spotlight/shaders/scene_vertex.glsl");
    let fragment_file = String::from("./examples/04-spotlight/shaders/scene_fragment.glsl");
    let mut scene_program: Program = Program::compile_from_files(&vertex_file, &fragment_file).unwrap();

    let vertex_file = String::from("./examples/04-spotlight/shaders/light_vertex.glsl");
    let fragment_file = String::from("./examples/04-spotlight/shaders/light_fragment.glsl");
    let mut light_program: Program = Program::compile_from_files(&vertex_file, &fragment_file).unwrap();

    let light_pos = point3(-2.5, 0.0, 1.0);

    let mut cube = Node::new(Mesh::create_cube(Cube::create(0.75), Color::rgb(1.0, 1.0, 1.0)).unwrap());
    let mut plane = Node::new(Mesh::create_plane(Plane::create(10.0, 10.0), Color::rgb(1.0, 1.0, 1.0)).unwrap());

    plane.translate(Vector3{ x: 0.0, y: -3.0, z: 0.0});

    let mut scene = Scene::new();
    scene.add_node(&cube);
    scene.add_node(&plane);

    let mut camera = Camera::new();
    camera
        .perspective(60.0, window.aspect(), 0.1, 80.0)
        .lookat(
            point3(0.0, 6.0, -5.5),
            point3(0.0, 1.0, -1.0),
            vec3(0.0, 1.0, 0.0)
        );

    let mut spot_light = Spotlight::new();
    spot_light.set_lookat(
        point3(-0.5, 8.0, 2.0),
        point3(0.5, 0.0, 1.0),
        vec3(0.0, 1.0, 0.0),
    );

    // Textures & Frame Buffers
    let light_texture_size = Size::new(1024, 1024);
    let mut light_texture = Texture::create2d().unwrap();
    light_texture.bind();
    light_texture.set_size(&light_texture_size).unwrap();
    light_texture.clamp_to_edge();
    light_texture.nearest();
    light_texture.unbind();

    let mut shadow_texture = Texture::create_depth_texture().unwrap();
    shadow_texture.bind();
    shadow_texture.set_size(&light_texture_size).unwrap();
    shadow_texture.clamp_to_edge();
    shadow_texture.nearest();
    shadow_texture.unbind();

    let mut shadow_frame_buffer = FrameBuffer::create().unwrap();
    shadow_frame_buffer.bind();
    shadow_frame_buffer.set_texture(&light_texture).expect("Set texture failed");
    shadow_frame_buffer.set_depth_buffer(&shadow_texture).expect("Set depth buffer failed");
    shadow_frame_buffer.unbind();

    let start_time = Instant::now();

    loop {
        let now = Instant::now();
        let elapsed = now.duration_since(start_time);
        let elapsed = (elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9) as f32;

        // clear scene
        window.clear(0.0, 0.0, 0.0, 0.0);
        window.clear_depth(1.0);

        //----------------------------------------------------------
        // render light first
        shadow_frame_buffer.bind();
        window.set_viewport(&Point2::default(), &shadow_texture.size);
        window.clear(0.0, 0.0, 0.0, 1.0);
        window.clear_depth(1.0);
        window.set_cullmode(CullMode::Front);

        light_program.bind();
        light_program
            .uniform("u_lightView", spot_light.view.into())
            .uniform("u_lightProj", spot_light.projection.into());

        // render all nodes
        scene.nodes(&mut |node| {
            light_program
                .uniform("u_model", node.model_view.into());

            node.draw();
        });

        shadow_frame_buffer.unbind();


        //----------------------------------------------------------
        // Render Scene / Camera
        window.reset_viewport();
        window.set_cullmode(CullMode::Back);
        window.clear(0.0, 0.0, 0.0, 1.0);
        window.clear_depth(1.0);

        scene_program.bind();
        scene_program
            .uniform("u_camProj", camera.projection.into())
            .uniform("u_camView", camera.view.into())
            .uniform("u_lightView", spot_light.view.into())
            .uniform("u_lightRot", normal_matrix(&spot_light.view).into())
            .uniform("u_lightProj", spot_light.projection.into())
            .sampler("u_sShadowMap", 0, &shadow_texture)
            .uniform("u_shadowMapSize", light_texture_size.into());

        // render plane!
        let model_view = camera.view * plane.model_view;
        let model_view_proj = camera.projection * model_view;
        let normal_matrix: Matrix3<f32> = convert_to_matrix3(&model_view).invert().unwrap().transpose();

        plane.draw();

        // animate and render plane
        let rotate_x = Matrix4::from_angle_x(Rad::from(Deg(elapsed * 22.5)));
        let rotate_y = Matrix4::from_angle_y(Rad::from(Deg(elapsed * 45.0)));

        let model_view = camera.view * cube.model_view * rotate_y * rotate_x;
        let model_view_proj = camera.projection * model_view;
        let normal_matrix: Matrix3<f32> = convert_to_matrix3(&model_view).invert().unwrap().transpose();

        // render all nodes
        scene.nodes(&mut |node| {
            scene_program
                .uniform("u_ambientColor", Color::rgb(0.1, 0.1, 0.1).into())
                .uniform("u_model", node.model_view.into())
                .uniform("u_normalModel", node.normal_view().into());

            node.draw();
        });

        scene_program.unbind();


        //----------------------------------------------------------
        // display everything on screen
        window.swap_buffers();

        // handle events
        window.poll_events();
        if window.should_close() {
            return;
        }
    }
}
