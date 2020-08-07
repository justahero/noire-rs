#![allow(dead_code)]
#![allow(unused_variables)]

extern crate cgmath;
extern crate gl;
extern crate noire;
extern crate notify;

use cgmath::*;
use std::error::Error;
use std::time::{Instant};

use noire::math::*;
use noire::math::{Camera, Color};
use noire::mesh::{Cube, Mesh, Node, Plane, Scene};
use noire::render::{FrameBuffer, Program, Spotlight, Texture, Uniform};
use noire::render::traits::*;
use noire::render::{Capability, CullMode, Point2, Size};
use noire::{core::Timer, render::{OpenGLWindow, RenderWindow, Window}};

fn main() -> Result<(), Box<dyn Error>> {
    let window_size = Size::new(1024, 1024);
    let mut window = RenderWindow::create(&window_size, "Hello This is window")
        .expect("Failed to create Render Window");

    window.enable(Capability::DepthTest);
    window.enable(Capability::CullFace);
    window.set_cullmode(CullMode::Back);

    let vertex_file = String::from("./examples/04-spotlight/shaders/light_vertex.glsl");
    let fragment_file = String::from("./examples/04-spotlight/shaders/light_fragment.glsl");
    let mut light_program: Program = Program::compile_from_files(&vertex_file, &fragment_file).unwrap();

    // create shader program
    let vertex_file = String::from("./examples/04-spotlight/shaders/scene_vertex.glsl");
    let fragment_file = String::from("./examples/04-spotlight/shaders/scene_fragment.glsl");
    let mut scene_program: Program = Program::compile_from_files(&vertex_file, &fragment_file).unwrap();

    let mut cube = Node::new(Mesh::create_cube(Cube::create(1.4), Color::rgb(0.15, 0.15, 0.2)).unwrap());
    let mut plane = Node::new(Mesh::create_plane(Plane::create(10.0, 10.0), Color::rgb(0.2, 0.25, 0.25)).unwrap());

    cube.translate(vec3(0.0, 2.0, 0.0));
    plane.translate(vec3(0.0, -1.0, 0.0));

    let mut scene = Scene::new();
    scene.add_node(&mut cube);
    scene.add_node(&mut plane);

    let mut camera = Camera::new();
    camera
        .perspective(60.0, window.aspect(), 0.1, 80.0)
        .lookat(
            point3(0.0, 6.0, -5.5),
            point3(0.0, 1.0, -1.0),
            vec3(0.0, 1.0, 0.0)
        );

    let mut spot_light = Spotlight::new(Color::rgb(0.65, 0.55, 0.40));
    spot_light.set_perspective(55.0, 1.0, 0.1, 100.0);
    spot_light.set_lookat(
        point3(-0.5, 8.0, 2.0),
        point3(-0.5, 0.0, 1.0),
        vec3(0.0, 1.0, 0.0),
    );

    // Textures & Frame Buffers
    let depth_texture_size = Size::new(1024, 1024);

    let mut shadow_texture = Texture::create_depth_texture().unwrap();
    shadow_texture.bind();
    shadow_texture.set_size(&depth_texture_size).unwrap();
    shadow_texture.clamp_to_edge();
    shadow_texture.nearest();
    shadow_texture.unbind();

    // let mut screen_rect = ScreenRect::new().unwrap();

    let mut shadow_frame_buffer = FrameBuffer::create().unwrap();
    shadow_frame_buffer.set_depth_buffer(&shadow_texture).expect("Set depth buffer failed");

    let start_time = Instant::now();
    let timer = Timer::now();

    loop {
        let elapsed = timer.elapsed_in_seconds() as f32;

        //----------------------------------------------------------
        // render light first
        shadow_frame_buffer.bind();
        window.set_viewport(&Point2::default(), &shadow_texture.size);
        window.clear(0.0, 0.0, 0.0, 1.0);
        window.clear_depth(1.0);
        window.set_cullmode(CullMode::Front);

        let light_space_matrix = spot_light.projection * spot_light.view;
        let camera_space_matrix = camera.projection * camera.view;

        light_program.bind();
        light_program.uniform("u_lightSpaceMatrix", light_space_matrix.into());

        // render all nodes
        scene.nodes(&mut |node| {
            light_program
                .uniform("u_model", node.model_view.into());

            node.draw();
        });

        shadow_frame_buffer.unbind();
        light_program.unbind();

        //----------------------------------------------------------
        // Render Scene / Camera
        window.reset_viewport();
        window.set_cullmode(CullMode::Back);
        window.clear(0.0, 0.0, 0.0, 1.0);
        window.clear_depth(1.0);

        let unit: i32 = 0;

        shadow_texture.bind();
        scene_program.bind();
        scene_program
            .uniform("u_cameraPos", camera.position.into())
            .uniform("u_cameraSpaceMatrix", camera_space_matrix.into())
            .uniform("u_lightView", spot_light.view.into())
            .uniform("u_lightRot", normal_matrix(&spot_light.view).into())
            .uniform("u_lightProj", spot_light.projection.into())
            .uniform("u_lightPos", spot_light.pos.into())
            .uniform("u_lightColor", spot_light.color.into())
            .uniform("u_sShadowMap", Uniform::Integer(unit).into());

        // render all nodes
        scene.nodes(&mut |node| {
            scene_program
                .uniform("u_ambientColor", Color::rgb(0.1, 0.1, 0.1).into())
                .uniform("u_diffuseColor", node.mesh.color.into())
                .uniform("u_model", node.model_view.into());

            node.draw();
        });

        scene_program.unbind();
        shadow_texture.unbind();

        // screen_rect.render(&window, &Point2::default(), &Size::new(384, 384), &mut shadow_texture);

        //----------------------------------------------------------
        // display everything on screen
        window.swap_buffers();

        // handle events
        window.poll_events();
        if window.should_close() {
            return Ok(());
        }
    }
}
