use std::num::NonZeroU32;

use crate::{BindingDescriptor, ShaderStage};

#[derive(Debug)]
pub enum BindingType {

}

#[derive(Debug)]
pub struct BindGroupDescriptor {
    pub set: u32,
    pub bindings: Vec<BindGroupLayoutEntry>,
}

impl BindGroupDescriptor {
    pub fn new(set: u32, bindings: Vec<BindGroupLayoutEntry>) -> Self {
        Self {
            set,
            bindings,
        }
    }
}

/// Describes a single binding inside a BindGroup
///
/// For example in shader
///
/// layout(set = 0, binding = 1) uniform;
///
#[derive(Debug)]
pub struct BindGroupLayoutEntry {
    /// Debug label
    pub label: Option<String>,
    /// The Binding index, must match shader index and unique inside a BindGroupLayout
    pub binding: u32,
    /// Which shader can see this binding
    pub visibility: ShaderStage,
    /// The type of binding
    pub binding_type: BindingType,
    /// Indices if this entry is an array, must be 1 or greater
    pub count: Option<NonZeroU32>,
}

impl From<&BindGroupLayoutEntry> for wgpu::BindGroupLayoutEntry {
    fn from(entry: &BindGroupLayoutEntry) -> Self {
        Self {
            binding: entry.binding,
            visibility: entry.visibility.into(),
            // TODO currently set one type directly
            ty: wgpu::BindingType::UniformBuffer {
                dynamic: true,
                min_binding_size: None,
            },
            count: entry.count,
        }
    }
}

#[derive(Debug)]
pub struct BindGroupLayoutDescriptor {
    /// Debug label of the bind group layout
    pub label: Option<String>,
    /// Array of entrie in this BindGroupLayout
    pub entries: Vec<BindGroupLayoutEntry>,
}
