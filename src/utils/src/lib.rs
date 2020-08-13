#![crate_type="lib"]
#![crate_type="dylib"]

extern crate image;

pub use self::image_gif_recorder::ImageSetRecorder;

pub mod image_gif_recorder;
