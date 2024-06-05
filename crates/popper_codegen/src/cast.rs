use popper_llvm::context::Context;
use popper_llvm::types::{Type, TypeBuilder, TypeEnum};
use popper_mir::types::Types;

pub(crate) fn cast_type(context: Context, types: Types) -> TypeEnum {
    match types {
        Types::Int => context.i64_type().to_type_enum(),
        Types::Float => context.float_type().to_type_enum(),
        Types::Bool => context.bool_type().to_type_enum(),
        Types::Unit => unsafe { context.void_type().as_type_enum() },
        Types::String(_) => context.i8_type().ptr().to_type_enum(),
        Types::LLVMPtr => context.i8_type().ptr().to_type_enum(),
        Types::List(sub_ty, l) => {
            let sub_ty = cast_type(context, *sub_ty);
            let sub_ty = sub_ty.array(l as u64);
            sub_ty.to_type_enum()
        },
        Types::Ptr(sub_ty) => {
            let sub_ty = cast_type(context, *sub_ty);
            sub_ty.ptr().to_type_enum()
        },
        Types::Struct(name, ty) => {
            let tys = ty.iter().map(|t| cast_type(context, t.clone())).collect::<Vec<_>>();
            let ty = context.named_struct_type(&name);
            ty.set_body(&tys, false);
            ty.to_type_enum()
        },
        Types::Label => panic!("Cannot cast to label type"),
    }
}
