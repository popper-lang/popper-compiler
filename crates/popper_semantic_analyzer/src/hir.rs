
use popper_ast::ast::{LangAst, LangNode, LangNodeId};
use popper_ast::layer::Ast;
use popper_ast::type_::Type;
use popper_index::Idx;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeDescriptorId(u32);

impl Idx for NodeDescriptorId {
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
    descriptors: Vec<NodeDescriptorKind>,
}

impl NodeDescriptor {
    pub fn new() -> Self {
        NodeDescriptor {
            descriptors: Vec::new(),
        }
    }

    pub fn add_used(&mut self, id: usize) -> NodeDescriptorId {
        let kind = NodeDescriptorKind::Used(id);
        self.descriptors.push(kind);
        NodeDescriptorId::new(self.descriptors.len() - 1)
    }
    
    pub fn incr_used(&mut self) {
        for descriptor in &mut self.descriptors {
            if let NodeDescriptorKind::Used(count) = descriptor {
                *count += 1;
                return; // Increment the first used count found
            }
        }
        self.add_used(1);
    }

    pub fn set_type(&mut self, ty: Type)  {
        for descriptor in &mut self.descriptors {
            if let NodeDescriptorKind::Type(existing_ty) = descriptor {
                if *existing_ty == ty {
                    return; // Type already exists, no need to add again
                }
            }
        }
        let kind = NodeDescriptorKind::Type(ty);
        self.descriptors.push(kind);
    }

    pub fn get(&self, id: NodeDescriptorId) -> &NodeDescriptorKind {
        &self.descriptors[id.index()]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NodeDescriptorKind {
    Used(usize),
    Type(Type),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct HirNodeId(pub u32);

impl Into<LangNodeId> for HirNodeId {
    fn into(self) -> LangNodeId {
        LangNodeId::new(self.index())
    }
}

impl From<LangNodeId> for HirNodeId {
    fn from(node_id: LangNodeId) -> Self {
        HirNodeId::new(node_id.index())
    }
}

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
    
    pub fn incr_used(&mut self, id: HirNodeId) {
        let descriptor_id = self.nodes[id.index()].descriptor;
        if let Some(descriptor) = self.descriptors.get_mut(descriptor_id.index()) {
            descriptor.incr_used();
        } 
    }
    
    pub fn set_type(&mut self, id: HirNodeId, ty: Type) {
        let descriptor_id = self.nodes[id.index()].descriptor;
        if let Some(descriptor) = self.descriptors.get_mut(descriptor_id.index()) {
            descriptor.set_type(ty);
        }
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