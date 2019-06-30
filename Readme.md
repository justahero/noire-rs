noire-rs
--------

A minimal 3d rendering library written in Rust, it's mostly a port from existing code. It is mostly written to play with GL shaders.

# Build

First install [cargo-watch](https://github.com/passcod/cargo-watch) to constantly compile the project while changing code.

```shell
cargo install cargo-watch
```

Then run the watch task

```shell
cargo watch -c -x "+nightly build"
```

# Samples

Currently there are only two samples, to run them:

```shell
cargo run --example raymarching
```

or

```shell
cargo run --example triangles
```

The latter example also supports live editing of the shader files, check shader files, open files in `./examples/triangles/shaders` in your text editor, edit and save it. If the new shader code compiles successfully, changes are applied immediately or otherwise error logs are displayed in the terminal.
