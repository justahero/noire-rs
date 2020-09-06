pub mod bind_group;
pub mod binding;
pub mod pipeline;
pub mod state;

pub use bind_group::{BindGroupDescriptor};
pub use binding::*;
pub use pipeline::*;
pub use state::*;

#[derive(Debug, Clone, Copy)]
pub enum FrontFace {
    /// Counter Clock wise
    Ccw,
    /// Clockwise
    Cw,
}

impl From<FrontFace> for wgpu::FrontFace {
    fn from(face: FrontFace) -> Self {
        match face {
            FrontFace::Ccw => wgpu::FrontFace::Ccw,
            FrontFace::Cw => wgpu::FrontFace::Cw,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CullMode {
    Front,
    Back,
    None,
}

impl From<CullMode> for wgpu::CullMode {
    fn from(mode: CullMode) -> Self {
        match mode {
            CullMode::Front => wgpu::CullMode::Front,
            CullMode::Back => wgpu::CullMode::Back,
            CullMode::None => wgpu::CullMode::None,
        }
    }
}

pub struct RasterizationStateDescriptor {
    pub front_face: FrontFace,
    pub cull_mode: CullMode,
    pub depth_bias: i32,
    pub depth_bias_slope_scale: f32,
    pub depth_bias_clamp: f32,
    pub clamp_depth: bool,
}

impl Default for RasterizationStateDescriptor {
    fn default() -> Self {
        Self {
            front_face: FrontFace::Ccw,
            cull_mode: CullMode::Back,
            depth_bias: 0,
            depth_bias_slope_scale: 0.0,
            depth_bias_clamp: 0.0,
            clamp_depth: false,
        }
    }
}

impl From<RasterizationStateDescriptor> for wgpu::RasterizationStateDescriptor {
    fn from(val: RasterizationStateDescriptor) -> Self {
        wgpu::RasterizationStateDescriptor {
            front_face: val.front_face.into(),
            cull_mode: val.cull_mode.into(),
            depth_bias: val.depth_bias,
            depth_bias_slope_scale: val.depth_bias_slope_scale,
            depth_bias_clamp: val.depth_bias_clamp,
            clamp_depth: val.clamp_depth,
        }
    }
}

pub enum PrimitiveTopology {
    PointList,
    LineList,
    LineStrip,
    TriangleList,
    TriangleStrip,
}

impl From<PrimitiveTopology> for wgpu::PrimitiveTopology {
    fn from(val: PrimitiveTopology) -> Self {
        match val {
            PrimitiveTopology::PointList => wgpu::PrimitiveTopology::PointList,
            PrimitiveTopology::LineList => wgpu::PrimitiveTopology::LineList,
            PrimitiveTopology::LineStrip => wgpu::PrimitiveTopology::LineStrip,
            PrimitiveTopology::TriangleList => wgpu::PrimitiveTopology::TriangleList,
            PrimitiveTopology::TriangleStrip => wgpu::PrimitiveTopology::TriangleStrip,
        }
    }
}
