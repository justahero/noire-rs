pub mod window;

pub use window::*;

pub mod prelude {
    pub use crate::{};
}

use winit::{
    event,
    event::{DeviceEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
};
