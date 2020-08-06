#![crate_type="lib"]
#![crate_type="dylib"]

#[macro_use]
extern crate lazy_static;

pub use self::open_simplex_noise::OpenSimplexNoise;

pub mod open_simplex_noise;
