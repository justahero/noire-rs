#![crate_type="lib"]
#![crate_type="dylib"]

extern crate backtrace;
extern crate cgmath;
extern crate gl;
extern crate glfw;
#[macro_use]
extern crate lazy_static;
extern crate notify;
extern crate rand;
extern crate rand_chacha;
extern crate regex;

pub mod canvas;
pub mod input;
pub mod math;
pub mod mesh;
pub mod render;
