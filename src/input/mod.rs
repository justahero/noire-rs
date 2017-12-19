pub mod mouse;

#[derive(Copy, Clone, Debug)]
pub enum Motion {
    /// x and y in window coordinates
    MousePos(f64, f64),
    /// x and y in relative coordinates
    MouseRelative(f64, f64),
}

#[derive(Copy, Clone, Debug)]
pub enum Input {
    Move(Motion),
}
