noire-rs
--------

A minimal 3d rendering library written in Rust, it's mostly a port from existing code. It is first and foremost designed to experiment with Open GL shaders.

# Build

First install [cargo-watch](https://github.com/passcod/cargo-watch) to constantly compile the project while changing code.

```shell
cargo install cargo-watch
```

Then run the watch task

```shell
cargo watch -c -x "+nightly build"
```

# Examples

There are a number of example applications using this library, found in the [examples](./examples) folder. Run

```shell
cargo run --example
```

to list the examples.

To run one example. e.g. `triangles` run

```shell
cargo run --example triangles
```

The latter example also supports live editing of the shader files, check shader files, open files in `./examples/triangles/shaders` in your text editor, edit and save it. If the new shader code compiles successfully, changes are applied immediately or otherwise error logs are displayed in the terminal.

## Other Frameworks / Projects

Low level libraries

* [cgmath](https://github.com/rustgd/cgmath) - a linear algebra and math library for computer graphics
* [gfx-rs/wgpu](https://github.com/gfx-rs/wgpu-rs) - an idiomatic wrapper library around [WebGPU](https://www.w3.org/community/gpu/) implementation
* [image](https://github.com/image-rs/image) - a fast image processing library
* [spirv-reflect](https://github.com/gwihlidal/spirv-reflect-rs) - a reflection API to analyze SPIR-V shader byte code
* [ultraviolent](https://github.com/termhn/ultraviolet) - a fast linear & geometric algebra library
* [winit](https://github.com/rust-windowing/winit) - a cross platform window handling library

Engines / Frameworks

* [amethyst](https://github.com/amethyst/amethyst) - data driven & data oriented game engine
* [bevy](https://github.com/bevyengine/bevy/) - a modern data driven engine
* [coffee](https://github.com/hecrj/coffee) - a 2D game engine
* [ggez](https://github.com/ggez/ggez) - a 2D game engine to create Good Game Easily
* [piston](https://github.com/PistonDevelopers/piston) - a modular game engine, one of the first written
* [p5.js](https://p5js.org/) - a JavaScript library for creative coding based on [processing](https://github.com/processing/processing)
