use popper_ast::ast::{LangAst, LangNode, LangNodeId, Symbol, SymbolId, SymbolTable};
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

#[derive(Default, Debug, Clone)]
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

    pub fn set_type(&mut self, ty: Type) {
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

    pub fn set_def_at(&mut self, node_id: HirNodeId) {
        for descriptor in &mut self.descriptors {
            if let NodeDescriptorKind::DefAt(existing_id) = descriptor {
                if *existing_id == node_id {
                    return; // DefAt already exists, no need to add again
                }
            }
        }
        let kind = NodeDescriptorKind::DefAt(node_id);
        self.descriptors.push(kind);
    }

    pub fn unreachable(&mut self) {
        for descriptor in &mut self.descriptors {
            if let NodeDescriptorKind::Unreachable = descriptor {
                return; // Unreachable already exists, no need to add again
            }
        }
        let kind = NodeDescriptorKind::Unreachable;
        self.descriptors.push(kind);
    }

    pub fn set_math_op_kind(&mut self, op: MathOpKind) {
        for descriptor in &mut self.descriptors {
            if let NodeDescriptorKind::MathOpTyped(existing_op) = descriptor {
                if *existing_op == op {
                    return; // MathOpKind already exists, no need to add again
                }
            }
        }
        let kind = NodeDescriptorKind::MathOpTyped(op);
        self.descriptors.push(kind);
    }

    pub fn is_unreachable(&self) -> bool {
        self.descriptors
            .iter()
            .any(|descriptor| matches!(descriptor, NodeDescriptorKind::Unreachable))
    }

    pub fn get(&self, id: NodeDescriptorId) -> &NodeDescriptorKind {
        &self.descriptors[id.index()]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeDescriptorKind {
    Used(usize),
    Type(Type),
    DefAt(HirNodeId),
    Unreachable,
    MathOpTyped(MathOpKind),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MathOpKind {
    PtrAndInt,
    IntAndPtr,
    IntAndInt,
    FloatAndFloat,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
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

#[derive(Debug)]
pub struct HirNode {
    pub node: LangNode,
    pub descriptor: NodeDescriptorId,
}

#[derive(Debug)]
pub struct Hir {
    root: HirNodeId,
    nodes: Vec<HirNode>,
    descriptors: Vec<NodeDescriptor>,
    symbol_table: SymbolTable,
}

impl Hir {
    pub fn create_from_ast(ast: &LangAst) -> Hir {
        let mut hir = Hir::new();
        hir.symbol_table = ast.symbol_table().clone();

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
            symbol_table: SymbolTable::new(),
        }
    }

    pub fn get_symbol(&self, id: SymbolId) -> &Symbol {
        self.symbol_table.get(id)
    }

    pub fn add_symbol(&mut self, name: &str) -> SymbolId {
        self.symbol_table.intern(name)
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

    pub fn get_descriptor(&self, id: HirNodeId) -> Option<&NodeDescriptor> {
        let descriptor_id = self.nodes[id.index()].descriptor;
        self.descriptors.get(descriptor_id.index())
    }

    pub fn get_type(&self, id: HirNodeId) -> Option<Type> {
        let descriptor_id = self.nodes[id.index()].descriptor;
        if let Some(descriptor) = self.descriptors.get(descriptor_id.index()) {
            for kind in &descriptor.descriptors {
                if let NodeDescriptorKind::Type(ty) = kind {
                    return Some(ty.clone());
                }
            }
        }
        None
    }

    pub fn mark_unreachable(&mut self, id: HirNodeId) {
        let descriptor_id = self.nodes[id.index()].descriptor;
        if let Some(descriptor) = self.descriptors.get_mut(descriptor_id.index()) {
            descriptor.unreachable();
        }
    }

    pub fn is_unreachable(&self, id: HirNodeId) -> bool {
        let descriptor_id = self.nodes[id.index()].descriptor;
        if let Some(descriptor) = self.descriptors.get(descriptor_id.index()) {
            return descriptor.is_unreachable();
        }
        false
    }

    pub fn set_math_op_kind(&mut self, id: HirNodeId, op: MathOpKind) {
        let descriptor_id = self.nodes[id.index()].descriptor;
        if let Some(descriptor) = self.descriptors.get_mut(descriptor_id.index()) {
            descriptor.set_math_op_kind(op);
        }
    }

    pub fn get_math_op_kind(&self, id: HirNodeId) -> Option<MathOpKind> {
        let descriptor_id = self.nodes[id.index()].descriptor;
        if let Some(descriptor) = self.descriptors.get(descriptor_id.index()) {
            for kind in &descriptor.descriptors {
                if let NodeDescriptorKind::MathOpTyped(op) = kind {
                    return Some(*op);
                }
            }
        }
        None
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
        (0..self.nodes.len()).map(|i| HirNodeId::new(i))
    }

    fn root(&self) -> Self::NodeId {
        self.root
    }
}
