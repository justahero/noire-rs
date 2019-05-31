// use std::mem;
use std::ptr;

use gl;
use gl::types::*;

use render::traits::{Bindable};

/// A texture struct
pub struct Texture {
    pub id: u32,
}

impl Texture {
    pub fn create() -> Result<Self, String> {
        let mut id = 0;

        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
        }

        Ok(Texture {
            id
        })
    }

    pub fn linear(&mut self) -> &Self {
        self
    }
}

impl Bindable for Texture {
    fn bind(&self) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}
