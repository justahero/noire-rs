#[derive(Debug)]
pub struct BindGroupDescriptor {
    /// Debug label of the bind group, useful when set for debugger
    pub label: Option<String>,
    /// The bind group layout that corresponds to this bing group
    pub layout: BindGroupLayout,
    ///& The resources to bind to this group
    pub entries: Vec<BindGroupEntry>,
}
