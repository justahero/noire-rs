use gl;

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

    pub fn linear(&self) -> &Self {
        unsafe {
            gl::TexParameteri(self.id, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(self.id, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        }
        self
    }

    pub fn clamp_to_edge(&self) -> &Self {
        unsafe {
            gl::TexParameteri(self.id, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(self.id, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
        }
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
