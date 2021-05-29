use crate::{IndexFormat, TextureFormat};

pub mod bind_group;
pub mod pipeline;
pub mod pipeline_layout;
pub mod state;
pub mod uniform;

pub use bind_group::*;
pub use pipeline::*;
pub use pipeline_layout::*;
pub use state::*;
pub use uniform::*;

#[derive(Debug, Clone, Copy)]
pub enum FrontFace {
    /// Counter Clock wise
    Ccw,
    /// Clockwise
    Cw,
}

impl Default for FrontFace {
    fn default() -> Self {
        Self::Ccw
    }
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
pub enum Face {
    /// Front face cull mode
    Front,
    /// Back face cull mode
    Back,
}

impl From<Face> for wgpu::Face {
    fn from(mode: Face) -> Self {
        match mode {
            Face::Front => wgpu::Face::Front,
            Face::Back => wgpu::Face::Back,
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

impl Default for PrimitiveTopology {
    fn default() -> Self {
        PrimitiveTopology::TriangleList
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum PolygonMode {
    /// Polygons are filled
    Fill = 0,
    /// Polygons are drawn as lines
    Line = 1,
    /// Polygons are drawn as points
    Point = 2,
}

impl Default for PolygonMode {
    fn default() -> Self {
        PolygonMode::Fill
    }
}

impl From<PolygonMode> for wgpu::PolygonMode {
    fn from(mode: PolygonMode) -> Self {
        match mode {
            PolygonMode::Fill => wgpu::PolygonMode::Fill,
            PolygonMode::Line => wgpu::PolygonMode::Line,
            PolygonMode::Point => wgpu::PolygonMode::Point,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct PrimitiveState {
    /// The primitive topology used to interpret vertices
    pub topology: PrimitiveTopology,
    /// Required index format when indices are given
    pub strip_index_format: Option<IndexFormat>,
    /// The face to consider the front
    pub front_face: FrontFace,
    /// The face culling mode
    pub cull_mode: Option<Face>,
    /// If set to true, the polygon depth is clamped to 0-1 range instead of being clipped
    pub clamp_depth: bool,
    /// Controls way each polygon is rasterized
    pub polygon_mode: PolygonMode,
    /// If set to true the primitives are rendered with conservative overestimation
    pub conservative: bool,
}

impl From<PrimitiveState> for wgpu::PrimitiveState {
    fn from(val: PrimitiveState) -> Self {
        Self {
            topology: val.topology.into(),
            strip_index_format: val.strip_index_format.map(IndexFormat::into),
            front_face: val.front_face.into(),
            cull_mode: val.cull_mode.map(Face::into),
            clamp_depth: val.clamp_depth,
            polygon_mode: val.polygon_mode.into(),
            conservative: val.conservative,
        }
    }
}

#[derive(Debug)]
pub struct StencilFaceState {
    pub compare: CompareFunction,
    pub fail_op: StencilOperation,
    pub depth_fail_op: StencilOperation,
    pub pass_op: StencilOperation,
}

#[derive(Debug)]
pub struct StencilState {
    pub front: StencilFaceState,
    pub back: StencilFaceState,
    pub read_mask: u32,
    pub write_mask: u32,
}

impl From<&StencilState> for wgpu::StencilState {
    fn from(val: &StencilState) -> Self {
        wgpu::StencilState {
            front: (&val.front).into(),
            back: (&val.back).into(),
            read_mask: val.read_mask,
            write_mask: val.write_mask,
        }
    }
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct DepthBiasState {
    pub constant: i32,
    pub slope_scale: f32,
    pub clamp: f32,
}

impl From<DepthBiasState> for wgpu::DepthBiasState {
    fn from(val: DepthBiasState) -> Self {
        Self {
            constant: val.constant,
            slope_scale: val.slope_scale,
            clamp: val.clamp,
        }
    }
}

#[derive(Debug)]
pub struct DepthStencilState {
    pub format: TextureFormat,
    pub depth_write_enabled: bool,
    pub depth_compare: CompareFunction,
    pub stencil: StencilState,
    pub bias: DepthBiasState,
}

impl Default for DepthStencilState {
    fn default() -> Self {
        Self {
            format: TextureFormat::Depth32Float,
            depth_write_enabled: true,
            depth_compare: CompareFunction::Less,
            stencil: StencilState {
                front: StencilFaceState::IGNORE,
                back: StencilFaceState::IGNORE,
                read_mask: 0,
                write_mask: 0,
            },
            bias: DepthBiasState::default(),
        }
    }
}

impl From<&DepthStencilState> for wgpu::DepthStencilState {
    fn from(val: &DepthStencilState) -> Self {
        wgpu::DepthStencilState {
            format: val.format.into(),
            depth_write_enabled: val.depth_write_enabled,
            depth_compare: val.depth_compare.into(),
            stencil: (&val.stencil).into(),
            bias: val.bias.into(),
        }
    }
}

impl From<&StencilFaceState> for wgpu::StencilFaceState {
    fn from(val: &StencilFaceState) -> Self {
        wgpu::StencilFaceState {
            compare: val.compare.into(),
            fail_op: val.fail_op.into(),
            depth_fail_op: val.depth_fail_op.into(),
            pass_op: val.pass_op.into(),
        }
    }
}

impl StencilFaceState {
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

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct MultisampleState {
    /// Number of samples calculated per pixel (for MSAA), For non sampled textures this is `1`
    pub count: u32,
    /// Bitmask that restricts the samples of a pixel modified by this pipeline.
    /// All samples can be enabled using the value `!0`
    pub mask: u64,
    /// When enabled, produces another sample mask per pixel based on the alpha output value.
    pub alpha_to_coverage_enabled: bool,
}

impl Default for MultisampleState {
    fn default() -> Self {
        Self {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        }
    }
}

impl From<MultisampleState> for wgpu::MultisampleState {
    fn from(val: MultisampleState) -> Self {
        Self {
            count: val.count,
            mask: val.mask,
            alpha_to_coverage_enabled: val.alpha_to_coverage_enabled,
        }
    }
}
