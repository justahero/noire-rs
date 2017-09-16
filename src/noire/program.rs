extern crate gl;
extern crate glfw;

#[allow(dead_code)]
use self::glfw::{Action, Context};

pub struct Variable {
    // TODO
}

pub struct Program {
    id: i32,
}

impl Program {
    fn new(&self, id: i32) -> Program {
        Program { id: id }
    }
}
