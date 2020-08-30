pub mod window;
pub mod windows;

pub use window::*;
pub use windows::*;

pub mod prelude {
    pub use crate::{};
}

use winit::{
    event,
    event::{DeviceEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
};
