#[derive(Debug)]
pub struct VertexAttributeDescriptor {
    /// The shader location of the variable
    pub location: u32,
    /// The offset
    pub offset: u32,
}
