use std::ptr;
use gl;

use render::Size;
use render::traits::{Bindable};

/// Specific the Format of the Pixel Data
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum PixelFormat {
    RGBA = gl::RGBA,
    BGRA = gl::BGRA,
    DepthComponent = gl::DEPTH_COMPONENT,
    DepthStencil = gl::DEPTH_STENCIL,
}

impl From<PixelFormat> for gl::types::GLenum {
    fn from(pixel_format: PixelFormat) -> Self {
        match pixel_format {
            PixelFormat::RGBA => gl::RGBA,
            PixelFormat::BGRA => gl::BGRA,
            PixelFormat::DepthComponent => gl::DEPTH_COMPONENT,
            PixelFormat::DepthStencil => gl::DEPTH_STENCIL,
        }
    }
}

impl From<gl::types::GLenum> for PixelFormat {
    fn from(pixel_format: gl::types::GLenum) -> Self {
        match pixel_format {
            gl::RGBA => PixelFormat::RGBA,
            gl::BGRA => PixelFormat::BGRA,
            gl::DEPTH_COMPONENT => PixelFormat::DepthComponent,
            gl::DEPTH_STENCIL => PixelFormat::DepthStencil,
            _ => panic!("Unknown pixel format found: {}", pixel_format),
        }
    }
}

/// A texture struct
pub struct Texture {
    pub id: u32,
    pub target: u32,
    pub format: u32,
    pub size: Size<u32>,
}

impl Texture {
    pub fn create2d() -> Result<Self, String> {
        let mut id = 0;
        let target = gl::TEXTURE_2D;

        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(target, id);
        }

        let format = gl::RGB;

        Ok(Texture {
            id,
            target,
            format,
            size: Size { width: 0, height: 0 },
        })
    }

    pub fn set_size(&mut self, size: Size<u32>) -> &mut Self {
        self.size = size;

        let level = 0;
        let pixel_type = gl::UNSIGNED_BYTE;

        unsafe {
            gl::TexImage2D(
                self.target,
                level,
                self.format as i32,
                size.width as i32,
                size.height as i32,
                0,
                PixelFormat::BGRA.into(),
                pixel_type,
                ptr::null(),
            );
        }

        self
    }

    pub fn linear(&self) -> &Self {
        unsafe {
            gl::TexParameteri(self.target, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(self.target, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        }
        self
    }

    /// Enables nearest neighbor interpolation
    pub fn nearest(&self) -> &Self {
        unsafe {
            gl::TexParameteri(self.target, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(self.target, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        }

        self
    }

    pub fn clamp_to_edge(&self) -> &Self {
        unsafe {
            gl::TexParameteri(self.target, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(self.target, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
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
