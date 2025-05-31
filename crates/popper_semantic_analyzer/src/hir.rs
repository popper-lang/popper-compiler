mod codegen;

use popper_ast::ast::{LangAst, LangNode};
use popper_ast::layer::Ast;
use popper_index::Idx;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeDescriptorId(u32);

impl popper_index::Idx for NodeDescriptorId {
    const MAX_ID: usize = u32::MAX as usize;
    const MAX: Self = NodeDescriptorId(u32::MAX);

    fn new(val: usize) -> Self {
        NodeDescriptorId(u32::new(val))
    }

    fn index(self) -> usize {
        self.0 as usize
    }
}

#[derive(Default)]
pub struct NodeDescriptor {
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct HirNodeId(pub u32);

impl Idx for HirNodeId {
    const MAX_ID: usize = u32::MAX as usize;
    const MAX: Self = HirNodeId(u32::MAX);

    fn new(val: usize) -> Self {
        HirNodeId(u32::new(val))
    }

    fn index(self) -> usize {
        self.0 as usize
    }
}

pub struct HirNode {
    pub node: LangNode,
    pub descriptor: NodeDescriptorId,
}

pub struct Hir {
    root: HirNodeId,
    nodes: Vec<HirNode>,
    descriptors: Vec<NodeDescriptor>,
}

impl Hir {

    pub fn create_from_ast(ast: &LangAst) -> Hir {
        let mut hir = Hir::new();

        for node in &ast.nodes {
            let descriptor_id = hir.add_descriptor(NodeDescriptor::default());
            let node_id = hir.add_node(node.clone(), descriptor_id);
            if ast.root.index() == node_id.index() {
                hir.set_root(node_id);
            }
        }

        hir
    }
    pub fn new() -> Self {
        Hir {
            root: HirNodeId::new(0),
            nodes: Vec::new(),
            descriptors: Vec::new(),
        }
    }

    pub fn set_root(&mut self, root: HirNodeId) {
        self.root = root;
    }

    pub fn add_node(&mut self, node: LangNode, descriptor: NodeDescriptorId) -> HirNodeId {
        let id = HirNodeId::new(self.nodes.len());
        self.nodes.push(HirNode { node, descriptor });
        id
    }

    pub fn add_descriptor(&mut self, descriptor: NodeDescriptor) -> NodeDescriptorId {
        let id = NodeDescriptorId::new(self.descriptors.len());
        self.descriptors.push(descriptor);
        id
    }
}

impl Ast for Hir {
    type NodeId = HirNodeId;
    type Node = HirNode;

    fn add(&mut self, node: Self::Node) -> Self::NodeId {
        self.add_node(node.node, node.descriptor)
    }

    fn get(&self, id: Self::NodeId) -> &HirNode {
        &self.nodes[id.index()]
    }

    fn get_mut(&mut self, id: Self::NodeId) -> &mut HirNode {
        &mut self.nodes[id.index()]
    }

    fn nodes(&self) -> impl Iterator<Item = Self::NodeId> {
        self.nodes.iter().map(|_| HirNodeId::new(self.nodes.len()))
    }

    fn root(&self) -> Self::NodeId {
        self.root
    }

    
}