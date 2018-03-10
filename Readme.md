noire-rs
--------

A minimal 3d rendering library written in Rust, it's mostly a port from existing code. It is mostly written to play with GL shaders.

# Build

To constantly compile the project on code changes run:

```shell
cargo watch -c -x "build"
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
