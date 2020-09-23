mod app;
mod event;
mod event_handler;

pub mod prelude {
    pub use crate::{
        app::App,
        event::WindowEvent,
        event_handler::EventHandler,
    };
}
