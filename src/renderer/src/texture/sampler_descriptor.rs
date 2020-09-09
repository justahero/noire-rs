use std::num::NonZeroU8;

use crate::CompareFunction;

#[derive(Debug)]
pub enum AddressMode {
    /// Clamp the value to the edge of the texture
    ClampToEdge,
    /// Repeat the texture in a tiling fashion
    Repeat,
    /// Repeat the texture, mirroring it every repeat
    MirrorRepeat,
}

impl From<AddressMode> for wgpu::AddressMode {
    fn from(mode: AddressMode) -> Self {
        match mode {
            AddressMode::ClampToEdge => wgpu::AddressMode::ClampToEdge,
            AddressMode::Repeat => wgpu::AddressMode::Repeat,
            AddressMode::MirrorRepeat => wgpu::AddressMode::MirrorRepeat,
        }
    }
}

#[derive(Debug)]
pub enum FilterMode {
    /// Nearest neighbor sampling
    Nearest,
    /// Linear interpolation
    Linear,
}

#[derive(Debug)]
pub struct SamplerDescriptor {
    /// Debug label of the sampler
    pub label: Option<String>,
    /// How to deal with out of bounds accesses in the u / x direction
    pub address_mode_u: AddressMode,
    /// How to deal with out of bounds accesses in the v / y direction
    pub address_mode_v: AddressMode,
    /// How to deal with out of bounds accesses in the w / z direction
    pub address_mode_w: AddressMode,
    /// How to filter the texture when it needs to be magnified
    pub mag_filter: FilterMode,
    /// How to filter the texture when it needs to be minified
    pub min_filter: FilterMode,
    /// How to filter between mip map levels
    pub mipmap_filter: FilterMode,
    /// Minimum level of detail to use
    pub lod_min_clamp: f32,
    /// Maximum level of detail to use
    pub lod_max_clamp: f32,
    /// If enabled, this is a comparison sampler using the given comparison function
    pub compare: Option<CompareFunction>,
    /// Valid values: 1, 2, 4, 8, 16
    pub anisotropy_clamp: Option<NonZeroU8>,
}

impl Default for SamplerDescriptor {
    fn default() -> Self {
        Self {
            label: None,
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Nearest,
            mipmap_filter: FilterMode::Nearest,
            lod_min_clamp: -100.0,
            lod_max_clamp: 100.0,
            compare: Some(CompareFunction::LessEqual),
            anisotropy_clamp: std::num::NonZeroU8::new(1),
        }
    }
}
