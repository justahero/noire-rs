#![crate_type="lib"]
#![crate_type="dylib"]

extern crate backtrace;
extern crate cgmath;
extern crate gl;
// TODO remove here
extern crate glfw;
extern crate image;
extern crate notify;
extern crate opensimplex;
extern crate rand;
extern crate rand_chacha;
extern crate regex;
extern crate renderer;
extern crate utils;

pub mod canvas;
pub mod core;
pub mod input;
pub mod math;
pub mod mesh;
pub mod render;
