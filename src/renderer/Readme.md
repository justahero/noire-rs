# WGPU Renderer

This crate uses the [wgpu](https://github.com/gfx-rs/wgpu-rs/) crate, an idiomatic Rust wrapper over wgpu-cure. It's meant as a general purpose graphics layer that supports multiple graphics backends, e.g. Vulkan, DirectX etc.

## Shader Compilation

Shaders are compiled with [shaderc](https://github.com/google/shaderc) package. The `renderer` crate is using the [shaderc-rs](https://crates.io/crates/shaderc) library to wrap the binary. `shaderc` requires the following packages to be installed:

* [Python 3](https://www.python.org/downloads/)
* download [ShaderC](https://github.com/google/shaderc) libraries & binaries for the OS (e.g. Windows 10)

The following paths need to be set up:

* set `PATH` variable to set `./shaderc/bin`.
* to link statically set `SHADERC_LIB_DIR` environment variable to `/shaderc/lib`.


## References

* Learn WGPU: https://sotrh.github.io/learn-wgpu/#what-is-wgpu
  * Setup swap chain: https://sotrh.github.io/learn-wgpu/beginner/tutorial2-swapchain/#state-new
