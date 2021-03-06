# WGPU Renderer

This crate uses the [wgpu](https://github.com/gfx-rs/wgpu-rs/) crate, an idiomatic Rust wrapper over wgpu-cure. It's meant as a general purpose graphics layer that supports multiple graphics backends, e.g. Vulkan, DirectX etc.

The following crates are also used

* [spirv_reflect](https://docs.rs/spirv-reflect/0.2.3/spirv_reflect/) - a crate to use reflection of a Spir-V shader, it fills in a shader layout from source.


## Shader Compilation

Shaders are compiled with [shaderc](https://github.com/google/shaderc) package. The `renderer` crate is using the [shaderc-rs](https://crates.io/crates/shaderc) library to wrap the binary. `shaderc` requires the following packages to be installed:

* [Python 3](https://www.python.org/downloads/)
* download [ShaderC](https://github.com/google/shaderc) libraries & binaries for the OS (e.g. Windows 10)

The following paths need to be set up:

* set `PATH` variable to set `./shaderc/bin`.
* to link statically set `SHADERC_LIB_DIR` environment variable to `/shaderc/lib`.


## References

* Vulkan Tutorial - Descriptor layout and buffer: https://vulkan-tutorial.com/Uniform_buffers/Descriptor_layout_and_buffer
* Learn WGPU: https://sotrh.github.io/learn-wgpu/#what-is-wgpu
  * Setup swap chain: https://sotrh.github.io/learn-wgpu/beginner/tutorial2-swapchain/#state-new
* https://docs.rs/winit/0.22.2/winit/
