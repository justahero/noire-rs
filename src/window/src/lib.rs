pub mod event;
pub mod monitor;
pub mod window;
pub mod windows;

pub use event::*;
pub use monitor::{Monitor, VideoMode};
pub use window::*;
pub use windows::*;

pub mod prelude {
    pub use crate::{};
}

use winit::{
    event::{DeviceEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
};
