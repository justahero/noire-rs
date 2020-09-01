pub mod event;
pub mod window;
pub mod windows;

pub use event::*;
pub use window::*;
pub use windows::*;

pub mod prelude {
    pub use crate::{};
}

use winit::{
    event::{DeviceEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
};
