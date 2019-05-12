use std::ptr;

use gl;

use render::traits::{Bindable, Drawable};
use render::vertex_buffer::VertexBuffer;

pub struct VertexArrayObject {
    id: u32,
    vbs: Vec<VertexBuffer>,
}

impl VertexArrayObject {
    pub fn new() -> VertexArrayObject {
        let mut id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }
        VertexArrayObject {
            id: id,
            vbs: vec![],
        }
    }

    pub fn add_vb(&mut self, vb: VertexBuffer) {
        self.vbs.push(vb);
    }
}

impl Bindable for VertexArrayObject {
    fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }

        let mut stride = 0;
        for (i, ref vb) in self.vbs.iter().enumerate() {
            vb.bind();
            unsafe {
                gl::VertexAttribPointer(i as u32, 2, gl::FLOAT, gl::FALSE, stride, ptr::null());
                gl::EnableVertexAttribArray(i as u32);
            }
            stride += vb.component_size();
        }
    }

    fn unbind(&self) {
        for (i, ref vb) in self.vbs.iter().enumerate() {
            vb.unbind();
            unsafe {
                gl::DisableVertexAttribArray(i as u32);
            }
        }
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}

impl Drawable for VertexArrayObject {
    fn draw(&self) {
        let count: i32 = self.vbs[0].count as i32;
        let render_type = self.vbs[0].render_type;
        unsafe {
            gl::DrawArrays(render_type, 0, count);
        }
    }
}

impl Drop for VertexArrayObject {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
        }
    }
}
