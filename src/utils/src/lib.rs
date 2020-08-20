#![crate_type="lib"]
#![crate_type="dylib"]

extern crate image;

pub use self::app_dir::app_dir;
pub use self::image_gif_recorder::ImageSetRecorder;

pub mod app_dir;
pub mod image_gif_recorder;
