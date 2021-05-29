use crate::TextureFormat;

#[derive(Clone, Copy, Debug)]
pub struct BlendState {
    pub color: BlendComponent,
    pub alpha: BlendComponent,
}

impl From<BlendState> for wgpu::BlendState {
    fn from(state: BlendState) -> Self {
        Self {
            color: state.color.into(),
            alpha: state.alpha.into(),
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct ColorTargetState {
    pub format: TextureFormat,
    pub blend: Option<BlendState>,
    pub write_mask: ColorWrite,
}

impl Default for ColorTargetState {
    fn default() -> Self {
        Self {
            format: TextureFormat::Bgra8UnormSrgb,
            blend: Some(BlendState {
                color: BlendComponent::COLOR_BLEND,
                alpha: BlendComponent::ALPHA_BLEND,
            }),
            write_mask: ColorWrite::ALL,
        }
    }
}

impl From<&ColorTargetState> for wgpu::ColorTargetState {
    fn from(val: &ColorTargetState) -> Self {
        wgpu::ColorTargetState {
            format: val.format.into(),
            blend: val.blend.map(BlendState::into),
            write_mask: val.write_mask.into(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BlendComponent {
    pub src_factor: BlendFactor,
    pub dst_factor: BlendFactor,
    pub operation: BlendOperation,
}

impl From<BlendComponent> for wgpu::BlendComponent {
    fn from(val: BlendComponent) -> Self {
        wgpu::BlendComponent {
            src_factor: val.src_factor.into(),
            dst_factor: val.dst_factor.into(),
            operation: val.operation.into(),
        }
    }
}

impl BlendComponent {
    pub const COLOR_BLEND: Self = Self {
        src_factor: BlendFactor::SrcAlpha,
        dst_factor: BlendFactor::OneMinusSrcAlpha,
        operation: BlendOperation::Add,
    };

    pub const ALPHA_BLEND: Self = Self {
        src_factor: BlendFactor::One,
        dst_factor: BlendFactor::One,
        operation: BlendOperation::Add,
    };

    pub const REPLACE: Self = Self {
        src_factor: BlendFactor::One,
        dst_factor: BlendFactor::Zero,
        operation: BlendOperation::Add,
    };
}

#[derive(Debug, Clone, Copy)]
pub enum BlendFactor {
    /// 0.0
    Zero,
    /// 1.0
    One,
    /// Source
    Src,
    /// 1.0 - Source
    OneMinusSrc,
    /// Source Alpha
    SrcAlpha,
    /// 1.0 - Source Alpha
    OneMinusSrcAlpha,
    /// Dest color
    Dst,
    /// 1.0 - Dest color
    OneMinusDst,
    /// Dest Alpha
    DstAlpha,
    /// 1.0 - Dest Alpha
    OneMinusDstAlpha,
    /// Source Alpha Saturated
    SrcAlphaSaturated,
    /// Constant
    Constant,
    /// 1.0 - Constant
    OneMinusConstant,
}

impl From<BlendFactor> for wgpu::BlendFactor {
    fn from(val: BlendFactor) -> Self {
        match val {
            BlendFactor::Zero => wgpu::BlendFactor::Zero,
            BlendFactor::One => wgpu::BlendFactor::One,
            BlendFactor::Src => wgpu::BlendFactor::Src,
            BlendFactor::OneMinusSrc => wgpu::BlendFactor::OneMinusSrc,
            BlendFactor::SrcAlpha => wgpu::BlendFactor::SrcAlpha,
            BlendFactor::OneMinusSrcAlpha => wgpu::BlendFactor::OneMinusSrcAlpha,
            BlendFactor::Dst => wgpu::BlendFactor::Dst,
            BlendFactor::OneMinusDst => wgpu::BlendFactor::OneMinusDst,
            BlendFactor::DstAlpha => wgpu::BlendFactor::DstAlpha,
            BlendFactor::OneMinusDstAlpha => wgpu::BlendFactor::OneMinusDstAlpha,
            BlendFactor::SrcAlphaSaturated => wgpu::BlendFactor::SrcAlphaSaturated,
            BlendFactor::Constant => wgpu::BlendFactor::Constant,
            BlendFactor::OneMinusConstant => wgpu::BlendFactor::OneMinusConstant,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BlendOperation {
    Add,
    Subtract,
    ReverseSubtract,
    Min,
    Max,
}

impl From<BlendOperation> for wgpu::BlendOperation {
    fn from(val: BlendOperation) -> Self {
        match val {
            BlendOperation::Add => wgpu::BlendOperation::Add,
            BlendOperation::Subtract => wgpu::BlendOperation::Subtract,
            BlendOperation::ReverseSubtract => wgpu::BlendOperation::ReverseSubtract,
            BlendOperation::Min => wgpu::BlendOperation::Min,
            BlendOperation::Max => wgpu::BlendOperation::Max,
        }
    }
}

bitflags::bitflags! {
    #[repr(transparent)]
    pub struct ColorWrite: u32 {
        const RED = 1;
        const GREEN = 2;
        const BLUE = 4;
        const ALPHA = 8;
        const COLOR = 7;
        const ALL = 15;
    }
}

impl Default for ColorWrite {
    fn default() -> Self {
        ColorWrite::ALL
    }
}

impl From<ColorWrite> for wgpu::ColorWrite {
    fn from(val: ColorWrite) -> Self {
        wgpu::ColorWrite::from_bits(val.bits()).unwrap()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CompareFunction {
    Never,
    Less,
    Equal,
    LessEqual,
    Greater,
    NotEqual,
    GreaterEqual,
    Always,
}

impl From<CompareFunction> for wgpu::CompareFunction {
    fn from(val: CompareFunction) -> Self {
        match val {
            CompareFunction::Never => wgpu::CompareFunction::Never,
            CompareFunction::Less => wgpu::CompareFunction::Less,
            CompareFunction::Equal => wgpu::CompareFunction::Equal,
            CompareFunction::LessEqual => wgpu::CompareFunction::LessEqual,
            CompareFunction::Greater => wgpu::CompareFunction::Greater,
            CompareFunction::NotEqual => wgpu::CompareFunction::NotEqual,
            CompareFunction::GreaterEqual => wgpu::CompareFunction::GreaterEqual,
            CompareFunction::Always => wgpu::CompareFunction::Always,
        }
    }
}
