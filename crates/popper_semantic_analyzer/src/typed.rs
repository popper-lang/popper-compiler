use popper_ast::ast::LangNodeId;
use popper_ast::type_::Type;

pub struct Typed {
    node_id: LangNodeId,
    ty: Type
}