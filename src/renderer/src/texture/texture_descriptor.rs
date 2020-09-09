use crate::{TextureFormat, TextureUsage};

#[derive(Debug)]
pub struct Extend3d {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

#[derive(Debug, Copy, Clone)]
pub enum TextureDimension {
    D1 = 1,
    D2 = 2,
    D3 = 3,
}

#[derive(Debug)]
pub struct TextureDescriptor {
    /// Debug label of the texture
    pub label: Option<String>,
    /// Size of the texture
    pub size: Extend3d,
    /// Mip count of texture, for a texture with no extra mips this must be 1
    pub mip_level_count: u32,
    /// Sample count of the texture
    pub sample_count: u32,
    /// The dimension of the texture
    pub dimension: TextureDimension,
    /// The format of the texture
    pub texture_format: TextureFormat,
    /// Allowed usages of the texture
    pub usage: TextureUsage,
}

impl Default for TextureDescriptor {
    fn default() -> Self {
        Self {
            label: None,
            size: Extend3d { width: 512, height: 512, depth: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            texture_format: TextureFormat::Depth32Float,
            usage: TextureUsage::OUTPUT_ATTACHMENT
                | TextureUsage::SAMPLED
                | TextureUsage::COPY_SRC,
        }
    }
}

impl TextureDescriptor {
    /// Create a new descriptor for a 2d texture
    pub fn texture_2d(width: u32, height: u32, format: TextureFormat) -> Self {
        Self {
            label: None,
            size: Extend3d { width, height, depth: 1 },
            texture_format: format,
            .. Default::default()
        }
    }
}
