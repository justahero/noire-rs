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

* [Coffee 2D game engine](https://github.com/hecrj/coffee)
