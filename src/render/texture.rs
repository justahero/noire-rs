use std::ptr;
use gl;

use render::opengl::get_render_error;
use render::{Format, PixelType, RenderError, Size};
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
    /// OpenGL id
    pub id: u32,
    /// Target type of the Texture
    pub target: u32,
    /// Texture internal format
    pub format: Format,
    /// Texture pixel format
    pub pixel_format: PixelFormat,
    /// The size of the Texture in pixels
    pub size: Size<u32>,
    /// The data type of the pixel data
    pub pixel_type: PixelType,
}

/// A Texture object
impl Texture {
    /// Creates a new Texture object
    pub fn create2d() -> Result<Self, RenderError> {
        let mut id = 0;
        let target = gl::TEXTURE_2D;

        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(target, id);
        }

        let texture = Texture {
            id,
            target,
            format: Format::RGB,
            pixel_format: PixelFormat::BGRA,
            size: Size { width: 0, height: 0 },
            pixel_type: PixelType::UnsignedByte,
        };

        unsafe {
            gl::BindTexture(target, 0);
        }

        Ok(texture)
    }

    /// Creates a Texture with a depth level
    pub fn create_depth_texture() -> Result<Self, RenderError> {
        let mut id = 0;
        let target = gl::TEXTURE_2D;

        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(target, id);
        }

        let texture = Texture {
            id,
            target,
            format: Format::DepthComponent,
            pixel_format: PixelFormat::DepthComponent,
            size: Size { width: 0, height: 0 },
            pixel_type: PixelType::Float,
        };

        unsafe {
            gl::BindTexture(target, 0);
        }

        Ok(texture)
    }

    /// Sets the size of the Texture
    ///
    /// For more details check documentation on
    /// [TexImage2D](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexImage2D.xhtml)
    ///
    /// ## Arguments
    ///
    /// * `size` - the Size of the texture, best to provide a multiple of 2
    ///
    /// Returns either reference to self or an Error message
    pub fn set_size(&mut self, size: &Size<u32>) -> Result<&mut Self, RenderError> {
        self.size = *size;

        let format: gl::types::GLenum = self.format.into();

        unsafe {
            gl::TexImage2D(
                self.target,
                0,
                format as i32,
                size.width as i32,
                size.height as i32,
                0,
                self.pixel_format.into(),
                self.pixel_type.into(),
                ptr::null(),
            );

            gl::GenerateMipmap(self.target);
        }

        get_render_error()?;

        Ok(self)
    }

    /// Enable linear interpolation
    pub fn linear(&self) -> &Self {
        debug_assert!(self.bound());

        unsafe {
            gl::TexParameteri(self.target, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(self.target, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        }
        self
    }

    /// Enables nearest neighbor interpolation
    pub fn nearest(&self) -> &Self {
        debug_assert!(self.bound());

        unsafe {
            gl::TexParameteri(self.target, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(self.target, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        }
        self
    }

    pub fn clamp_to_edge(&self) -> &Self {
        debug_assert!(self.bound());

        unsafe {
            gl::TexParameteri(self.target, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(self.target, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
        }
        self
    }
}

impl Bindable for Texture {
    fn bind(&self) -> &Self {
        debug_assert!(!self.bound());

        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(self.target, self.id);
        }
        self
    }

    fn unbind(&self) -> &Self {
        debug_assert!(self.bound());

        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(self.target, 0);
        }
        self
    }

    fn bound(&self) -> bool {
        let mut id = 0;
        unsafe {
            gl::GetIntegerv(gl::TEXTURE_BINDING_2D, &mut id);
        }

        self.id == (id as u32)
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}
