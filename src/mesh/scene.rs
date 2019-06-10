use mesh::Node;
use mesh::Mesh;

/// Stores objects in a basic manner
pub struct Scene {
    /// the list of nodes
    nodes: Vec<Node>,
}

impl Scene {
    /// Create a new empty scene
    pub fn new() -> Self {
        Scene::default()
    }

    /// Adds a node to the scene
    pub fn add_node(&mut self, node: Node) -> &mut Self {
        self.nodes.push(node);
        self
    }

    /// Wraps a mesh and adds it as a mesh to the scene
    pub fn add_mesh(&mut self, mesh: Mesh) -> &mut Self {
        self.add_node(Node::new(mesh));
        self
    }
}

impl Default for Scene {
    fn default() -> Self {
        Scene {
            nodes: Vec::new()
        }
    }
}
