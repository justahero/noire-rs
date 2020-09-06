use crate::BindingDescriptor;

#[derive(Debug)]
pub struct BindGroupDescriptor {
    pub set: u32,
    pub bindings: Vec<BindingDescriptor>,
}

impl BindGroupDescriptor {
    pub fn new(set: u32, bindings: Vec<BindingDescriptor>) -> Self {
        Self {
            set,
            bindings
        }
    }
}
