use crate::LoadOp;

pub trait WgpuFrom<T> {
    fn from(val: T) -> Self;
}

pub trait WgpuInto<U> {
    fn wgpu_into(self) -> U;
}

impl<T, U> WgpuInto<U> for T
where
    U: WgpuFrom<T>,
{
    fn wgpu_into(self) -> U {
        U::from(self)
    }
}

impl WgpuFrom<&LoadOp<f32>> for wgpu::LoadOp<f32> {
    fn from(op: &LoadOp<f32>) -> Self {
        match op {
            LoadOp::Clear(value) => wgpu::LoadOp::Clear(value.clone()),
            LoadOp::Load => wgpu::LoadOp::Load,
        }
    }
}
