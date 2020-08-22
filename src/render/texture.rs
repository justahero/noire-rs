use std::{fmt, ptr};
use gl;

use super::{Bindable, Format, PixelType, Size, opengl::get_error};

#[derive(Debug)]
pub enum TextureError {
    GenerationFailed,
    ImageSizeFailed(u32, u32),
    MipMapFailed,
}

impl fmt::Display for TextureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            TextureError::GenerationFailed => "Call to GenTextures failed".to_string(),
            TextureError::ImageSizeFailed(width, height) => format!("Failed to set size ({}x{}) failed", width, height),
            TextureError::MipMapFailed => "Failed to generate mip map levels".to_string(),
        };
        write!(f, "{}", s)
    }
}

/// Specific the Format of the Pixel Data
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum PixelFormat {
    RGB = gl::RGB,
    RGBA = gl::RGBA,
    BGRA = gl::BGRA,
    DepthComponent = gl::DEPTH_COMPONENT,
    DepthStencil = gl::DEPTH_STENCIL,
}

impl From<PixelFormat> for gl::types::GLenum {
    fn from(pixel_format: PixelFormat) -> Self {
        match pixel_format {
            PixelFormat::RGB => gl::RGB,
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
            gl::RGB => PixelFormat::RGB,
            gl::RGBA => PixelFormat::RGBA,
            gl::BGRA => PixelFormat::BGRA,
            gl::DEPTH_COMPONENT => PixelFormat::DepthComponent,
            gl::DEPTH_STENCIL => PixelFormat::DepthStencil,
            _ => panic!("Unknown pixel format found: {}", pixel_format),
        }
    }
}

/// Calls the OpenGL function to generate new texture id
fn generate_texture() -> u32 {
    let mut id = 0;
    unsafe {
        gl::GenTextures(1, &mut id);
    }
    id
}

/// A texture struct
#[derive(Debug)]
pub struct Texture {
    /// OpenGL id
    pub id: u32,
    /// Target type of the Texture
    pub target: u32,
    /// Texture internal format
    pub format: Format,
    /// Texture pixel format
    pub pixel_format: PixelFormat,
    /// The width of the Texture in pixels
    pub width: u32,
    /// The height of the Texture in pixels
    pub height: u32,
    /// The data type of the pixel data
    pub pixel_type: PixelType,
    /// The unit to set the texture to in a shader
    pub unit: u32,
}

/// A Texture object
impl Texture {
    /// Creates a new Texture object
    pub fn create_2d(width: u32, height: u32, format: Format, unit: u32) -> Result<Self, TextureError> {
        Texture::create(
            gl::TEXTURE_2D,
            width,
            height,
            format,
            unit,
            PixelFormat::RGBA,
            PixelType::UnsignedByte
        )
    }

    /// Creates a depth texture with given width and height
    pub fn create_depth(width: u32, height: u32) -> Result<Self, TextureError> {
        Texture::create(
            gl::TEXTURE_2D,
            width,
            height,
            Format::DepthComponent,
            0,
            PixelFormat::DepthComponent,
            PixelType::Float,
        )
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
    fn create(
        target: gl::types::GLenum,
        width: u32,
        height: u32,
        format: Format,
        unit: u32,
        pixel_format: PixelFormat,
        pixel_type: PixelType
    ) -> Result<Self, TextureError> {
        let id = generate_texture();

        get_error().map_err(|_e| TextureError::GenerationFailed)?;

        unsafe {
            gl::BindTexture(target, id);
        }

        unsafe {
            gl::TexImage2D(
                target,
                0,
                format as i32,
                width as i32,
                height as i32,
                0,
                pixel_format.into(),
                pixel_type.into(),
                ptr::null(),
            );
        }

        get_error().map_err(|_e| TextureError::ImageSizeFailed(width, height))?;

        unsafe {
            gl::TexParameteri(target, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(target, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        }

        unsafe {
            gl::GenerateMipmap(target);
        }

        Ok(Texture {
            id,
            width,
            height,
            target,
            format,
            unit,
            pixel_format,
            pixel_type,
        })
    }

    /// Return the size of the texture
    pub fn size(&self) -> Size<u32> {
        Size::new(self.width, self.height)
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
    fn bind(&mut self) -> &mut Self {
        // debug_assert!(!self.bound());

        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + self.unit);
            gl::BindTexture(self.target, self.id);
        }
        self
    }

    fn unbind(&mut self) -> &mut Self {
        // debug_assert!(self.bound());

        unsafe {
            // gl::ActiveTexture(gl::TEXTURE0);
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
