use crate::TextureFormat;

pub mod bind_group;
pub mod binding;
pub mod pipeline;
pub mod pipeline_layout;
pub mod state;

pub use bind_group::*;
pub use binding::*;
pub use pipeline::*;
pub use pipeline_layout::*;
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

#[derive(Debug)]
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

#[derive(Debug, Clone, Copy)]
pub enum PrimitiveTopology {
    PointList,
    LineList,
    LineStrip,
    TriangleList,
    TriangleStrip,
}

#[derive(Debug)]
pub struct DepthStencilStateDescriptor {
    pub format: TextureFormat,
    pub depth_write_enabled: bool,
    pub depth_compare: CompareFunction,
    pub stencil: StencilStateDescriptor,
}

impl Default for DepthStencilStateDescriptor {
    fn default() -> Self {
        Self {
            format: TextureFormat::Depth32Float,
            depth_write_enabled: true,
            depth_compare: CompareFunction::Less,
            stencil: StencilStateDescriptor {
                front: StencilStateFaceDescriptor::IGNORE,
                back: StencilStateFaceDescriptor::IGNORE,
                read_mask: 0,
                write_mask: 0,
            }
        }
    }
}

impl From<&DepthStencilStateDescriptor> for wgpu::DepthStencilStateDescriptor {
    fn from(val: &DepthStencilStateDescriptor) -> Self {
        wgpu::DepthStencilStateDescriptor {
            format: val.format.into(),
            depth_write_enabled: val.depth_write_enabled,
            depth_compare: val.depth_compare.into(),
            stencil: (&val.stencil).into(),
        }
    }
}

#[derive(Debug)]
pub struct StencilStateDescriptor {
    pub front: StencilStateFaceDescriptor,
    pub back: StencilStateFaceDescriptor,
    pub read_mask: u32,
    pub write_mask: u32,
}

impl From<&StencilStateDescriptor> for wgpu::StencilStateDescriptor {
    fn from(val: &StencilStateDescriptor) -> Self {
        wgpu::StencilStateDescriptor {
            front: (&val.front).into(),
            back: (&val.back).into(),
            read_mask: val.read_mask,
            write_mask: val.write_mask,
        }
    }
}

#[derive(Debug)]
pub struct StencilStateFaceDescriptor {
    pub compare: CompareFunction,
    pub fail_op: StencilOperation,
    pub depth_fail_op: StencilOperation,
    pub pass_op: StencilOperation,
}

impl From<&StencilStateFaceDescriptor> for wgpu::StencilStateFaceDescriptor {
    fn from(val: &StencilStateFaceDescriptor) -> Self {
        wgpu::StencilStateFaceDescriptor {
            compare: val.compare.into(),
            fail_op: val.fail_op.into(),
            depth_fail_op: val.depth_fail_op.into(),
            pass_op: val.pass_op.into(),
        }
    }
}

impl StencilStateFaceDescriptor {
    pub const IGNORE: Self = Self {
        compare: CompareFunction::Always,
        fail_op: StencilOperation::Keep,
        depth_fail_op: StencilOperation::Keep,
        pass_op: StencilOperation::Keep,
    };
}

#[derive(Debug, Copy, Clone)]
pub enum StencilOperation {
    Keep,
    Zero,
    Replace,
    Invert,
    IncrementClamp,
    DecrementClamp,
    IncrementWrap,
    DecrementWrap,
}

impl From<StencilOperation> for wgpu::StencilOperation {
    fn from(val: StencilOperation) -> Self {
        match val {
            StencilOperation::Keep => wgpu::StencilOperation::Keep,
            StencilOperation::Zero => wgpu::StencilOperation::Zero,
            StencilOperation::Replace => wgpu::StencilOperation::Replace,
            StencilOperation::Invert => wgpu::StencilOperation::Invert,
            StencilOperation::IncrementClamp => wgpu::StencilOperation::IncrementClamp,
            StencilOperation::DecrementClamp => wgpu::StencilOperation::DecrementClamp,
            StencilOperation::IncrementWrap => wgpu::StencilOperation::IncrementWrap,
            StencilOperation::DecrementWrap => wgpu::StencilOperation::DecrementWrap,
        }
    }
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

#[derive(Debug)]
pub enum UniformProperty {
    UInt,
    Int,
    IVec2,
    Float,
    UVec4,
    Vec2,
    Vec3,
    Vec4,
    Mat3,
    Mat4,
    Struct(Vec<UniformProperty>),
    Array(Box<UniformProperty>, usize),
}

impl UniformProperty {
    pub fn get_size(&self) -> u64 {
        match self {
            UniformProperty::UInt => 4,
            UniformProperty::Int => 4,
            UniformProperty::IVec2 => 4 * 2,
            UniformProperty::Float => 4,
            UniformProperty::UVec4 => 4 * 4,
            UniformProperty::Vec2 => 4 * 2,
            UniformProperty::Vec3 => 4 * 3,
            UniformProperty::Vec4 => 4 * 4,
            UniformProperty::Mat3 => 4 * 4 * 3,
            UniformProperty::Mat4 => 4 * 4 * 4,
            UniformProperty::Struct(properties) => {
                properties.iter().map(|p| p.get_size()).sum()
            }
            UniformProperty::Array(property, length) => property.get_size() * *length as u64,
        }
    }
}
