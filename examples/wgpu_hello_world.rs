use renderer::{App, Window, WindowHandler, WindowMode, WindowSettings};
use resources::Resources;

extern crate noire;
extern crate futures;
extern crate wgpu;

pub struct Example {
}

impl WindowHandler for Example {
    fn load(window: &Window, _resources: &Resources, _renderer: &mut renderer::Renderer) -> Self where Self: Sized {
        todo!()
    }

    fn update(&mut self, _resources: &Resources) {
    }

    fn render(&mut self, _window: &mut Window, app: &mut renderer::Renderer) {
        todo!()
    }
}

fn main() {
    let settings = WindowSettings::default()
        .with_title("Test")
        .with_mode(WindowMode::Windowed);

    Example::run(settings);
}
