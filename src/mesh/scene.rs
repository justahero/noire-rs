use crate::mesh::Node;

/// Stores objects in a basic manner
pub struct Scene<'a> {
    /// the list of nodes
    nodes: Vec<&'a mut Node>,
}

impl<'a> Scene<'a> {
    /// Create a new empty scene
    pub fn new() -> Self {
        Scene::default()
    }

    /// Adds a node to the scene
    pub fn add_node(&mut self, node: &'a mut Node) -> &mut Self {
        self.nodes.push(node);
        self
    }

    /// Iterate over all scene meshes, allow to set callback
    pub fn nodes<F>(&mut self, callback: &mut F)
    where F: FnMut(&mut Node) {
        for node in self.nodes.iter_mut() {
            callback(node);
        }
    }
}

impl Default for Scene<'_> {
    fn default() -> Self {
        Scene {
            nodes: Vec::new()
        }
    }
}
