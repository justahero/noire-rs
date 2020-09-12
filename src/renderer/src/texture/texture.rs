use std::num::NonZeroU32;

use crate::{TextureFormat, TextureViewDimension, TextureDescriptor};

#[derive(Debug, Copy, Clone)]
pub enum TextureAspect {
    All,
    StencilOnly,
    DepthOnly,
}

impl From<TextureAspect> for wgpu::TextureAspect {
    fn from(val: TextureAspect) -> Self {
        match val {
            TextureAspect::All => wgpu::TextureAspect::All,
            TextureAspect::StencilOnly => wgpu::TextureAspect::StencilOnly,
            TextureAspect::DepthOnly => wgpu::TextureAspect::DepthOnly,
        }
    }
}

pub struct Texture {
    pub width: u32,
    pub height: u32,
}

bitflags::bitflags! {
    #[repr(transparent)]
    pub struct TextureUsage: u32 {
        /// Allows a texture to be the source
        const COPY_SRC = 1;
        /// Allows a texture to be the destination
        const COPY_DST = 2;
        /// Allows a texture to be a sampled texture in a bind group
        const SAMPLED = 4;
        /// Allows a texture to be a storage texture in a bind group
        const STORAGE = 8;
        /// Allows a texture to be an output attachment of a render pass
        const OUTPUT_ATTACHMENT = 16;
        /// None of the bits set
        const NONE = 0;
    }
}

impl From<TextureUsage> for wgpu::TextureUsage {
    fn from(val: TextureUsage) -> Self {
        wgpu::TextureUsage::from_bits(val.bits()).unwrap()
    }
}

#[derive(Debug)]
pub struct TextureViewDescriptor {
    /// Debug label of the texture view
    pub label: Option<String>,
    /// Format of the texture view
    pub format: Option<TextureFormat>,
    /// Dimension of the texture view
    pub dimension: Option<TextureViewDimension>,
    /// Aspect ratio of the texture
    pub aspect: TextureAspect,
    /// Base mip level
    pub base_mip_level: u32,
    /// Mip level count
    pub level_count: Option<NonZeroU32>,
    /// Base Array Layer
    pub base_array_layer: u32,
    /// Layer count
    pub array_layer_count: Option<NonZeroU32>,
}

impl TextureViewDescriptor {
    pub fn create_from_texture(descriptor: &TextureDescriptor) -> Self {
        Self {
            label: None,
            format: Some(descriptor.texture_format),
            dimension: Some(TextureViewDimension::D2),
            aspect: TextureAspect::All,
            base_mip_level: 0,
            level_count: NonZeroU32::new(1),
            base_array_layer: 0,
            array_layer_count: NonZeroU32::new(1),
        }
    }
}

impl<'a> From<TextureViewDescriptor> for wgpu::TextureViewDescriptor<'a> {
    fn from(descriptor: TextureViewDescriptor) -> Self {
        Self {
            label: None,
            format: descriptor.format.map(|f| f.into()),
            dimension: descriptor.dimension.map(|d| d.into()),
            aspect: descriptor.aspect.into(),
            base_mip_level: descriptor.base_mip_level,
            level_count: descriptor.level_count,
            base_array_layer: descriptor.base_array_layer,
            array_layer_count: descriptor.array_layer_count,
        }
    }
}
