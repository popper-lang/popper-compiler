use crate::ast::{LangAst, LangNodeId};


pub trait Layer {
    type Inner: Ast;
    
    type Output;
    fn handle(&mut self, ast: &Self::Inner, node: <<Self as Layer>::Inner as Ast>::NodeId) -> Self::Output;
    
}

pub trait Ast {
    type NodeId: popper_index::Idx;
    type Node;
    fn add(&mut self, node: Self::Node) -> Self::NodeId;
    fn get(&self, node: Self::NodeId) -> &Self::Node;
    fn get_mut(&mut self, node: Self::NodeId) -> &mut Self::Node;
    fn nodes(&self) -> impl Iterator<Item = Self::NodeId>;
    
    fn root(&self) -> Self::NodeId;
    
    fn apply_layer<L: Layer<Inner = Self>>(&mut self, layer: &mut L) -> L::Output {
        layer.handle(self, self.root())
    }
}