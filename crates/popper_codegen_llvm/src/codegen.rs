use crate::context::{AggregateInfo, CodegenCtxLLVM, CodegenObjLLVM, CodegenStateLLVM, VaContext};
use crate::mangle::*;
use crate::types::LLVMType;
use crate::values::LLVMValue;
use inkwell::AddressSpace;
use inkwell::types::BasicType;
use inkwell::types::BasicTypeEnum;
use inkwell::values::BasicValue;
use popper_ast::ast::TypeDeclKind;
use popper_ast::ast::{Expr, LangNodeKind};
use popper_ast::layer::{Ast, Layer};
use popper_ast::type_::Fields;
use popper_ast::type_::Type;
use popper_semantic_analyzer::hir::{Hir, HirNodeId, MathOpKind};
use std::collections::HashMap;
use std::sync::Arc;

pub struct PopperCodegenLLVM<'state> {
    state: Arc<CodegenStateLLVM<'state>>,
    ctx: &'state CodegenCtxLLVM,
    vars: HashMap<String, CodegenObjLLVM<'state>>,
    aggregates: HashMap<String, AggregateInfo<'state>>,
    parent_ty: Option<Type>,
    parent_ctx: Option<String>,
    current_va_context: Option<VaContext<'state>>,
}

impl<'state> PopperCodegenLLVM<'state> {
    pub fn new(ctx: &'state CodegenCtxLLVM) -> PopperCodegenLLVM<'state> {
        let module = ctx.create_module("unnamed");
        let builder = ctx.create_builder();
        PopperCodegenLLVM {
            state: Arc::new(CodegenStateLLVM { module, builder }),
            ctx,
            vars: HashMap::new(),
            aggregates: HashMap::new(),
            parent_ty: None,
            parent_ctx: None,
            current_va_context: None,
        }
    }

    pub fn state(&self) -> Arc<CodegenStateLLVM<'state>> {
        self.state.clone()
    }

    pub fn create_obj(&self, value: LLVMValue<'state>) -> CodegenObjLLVM<'state> {
        CodegenObjLLVM {
            state: self.state.clone(),
            value,
            popper_ty: None,
        }
    }

    pub fn create_obj_with_ty(&self, value: LLVMValue<'state>, ty: Type) -> CodegenObjLLVM<'state> {
        CodegenObjLLVM {
            state: self.state.clone(),
            value,
            popper_ty: Some(ty),
        }
    }

    pub fn execute_main_function(&mut self) {
        let execution_engine = self
            .state
            .module
            .create_jit_execution_engine(inkwell::OptimizationLevel::Default)
            .unwrap();

        let main_func = execution_engine.get_function_value("main").unwrap();

        unsafe { execution_engine.run_function_as_main(main_func, &[]) };
    }

    pub fn to_llvm_type(&self, ty: &Type) -> LLVMType<'state> {
        match ty {
            Type::Int => self.ctx.i64_type().as_basic_type_enum().into(),
            Type::String => self
                .ctx
                .ptr_type(AddressSpace::default())
                .as_basic_type_enum()
                .into(),
            Type::Bool => self.ctx.bool_type().as_basic_type_enum().into(),
            Type::Void => self.ctx.void_type().into(),
            Type::Char => self.ctx.i8_type().as_basic_type_enum().into(),
            Type::Type(_) => self.ctx.i64_type().as_basic_type_enum().into(),
            Type::Ptr(_) => self
                .ctx
                .ptr_type(AddressSpace::default())
                .as_basic_type_enum()
                .into(),
            Type::Function(args, ret, is_popper_vararg, is_c_vararg, ..) => {
                let mut llvm_args: Vec<inkwell::types::BasicMetadataTypeEnum> = args
                    .iter()
                    .map(|arg| self.to_llvm_type(arg).into_metadata_type())
                    .collect::<Vec<_>>();

                if *is_popper_vararg && !*is_c_vararg {
                    llvm_args.push(self.ctx.ptr_type(inkwell::AddressSpace::default()).into());
                    llvm_args.push(self.ctx.ptr_type(inkwell::AddressSpace::default()).into());
                    llvm_args.push(self.ctx.i64_type().into());
                }

                let ret_ty = self.to_llvm_type(ret);
                if ret_ty.is_void_type() {
                    ret_ty
                        .into_void_type()
                        .fn_type(llvm_args.as_slice(), *is_c_vararg)
                        .into()
                } else {
                    ret_ty
                        .into_basic_type()
                        .fn_type(llvm_args.as_slice(), *is_c_vararg)
                        .into()
                }
            }
            Type::SelfType => self
                .ctx
                .ptr_type(AddressSpace::default())
                .as_basic_type_enum()
                .into(),
            Type::AggregateType(name, fields, kind) => {
                if self.aggregates.contains_key(name) {
                    return LLVMType::BasicType(BasicTypeEnum::StructType(
                        self.aggregates.get(name).unwrap().ty,
                    ));
                }
                if *kind == TypeDeclKind::Struct {
                    let mut field_types = Vec::new();
                    let mut field_indices = HashMap::new();
                    for (i, (field_name, field_ty)) in fields.fields.iter().enumerate() {
                        field_types.push(self.to_llvm_type(field_ty).into_basic_type());
                        field_indices.insert(field_name.clone(), i as u32);
                    }
                    let struct_type = self.ctx.opaque_struct_type(&format!("struct.{}", name));
                    struct_type.set_body(&field_types, false);
                    LLVMType::BasicType(BasicTypeEnum::StructType(struct_type))
                } else {
                    let size = ty.size_of();
                    let array_type = self.ctx.i8_type().array_type(size as u32);
                    let struct_type = self.ctx.opaque_struct_type(&format!("union.{}", name));
                    struct_type.set_body(&[array_type.into()], false);
                    LLVMType::BasicType(BasicTypeEnum::StructType(struct_type))
                }
            }
            Type::List(ty, count) => self
                .to_llvm_type(ty)
                .into_basic_type()
                .array_type(*count as u32)
                .as_basic_type_enum()
                .into(),
            _ => unimplemented!("Type {:?} not implemented yet", ty),
        }
    }

    pub fn load_object(&mut self, obj: &CodegenObjLLVM<'state>) -> Option<CodegenObjLLVM<'state>> {
        if !obj.can_be_loaded() {
            return Some(obj.clone());
        }
        let ty = self.to_llvm_type(obj.popper_ty.as_ref()?);
        let llvm_v = self
            .state
            .builder
            .build_load(
                ty.into_basic_type(),
                obj.value.clone().into_basic_value().into_pointer_value(),
                "loadtmp",
            )
            .unwrap();
        Some(self.create_obj(llvm_v.into()))
    }

    pub fn compile_lvalue(
        &mut self,
        ast: &Hir,
        node_id: HirNodeId,
    ) -> Option<CodegenObjLLVM<'state>> {
        let node = ast.get(node_id);
        let ast_node = &node.node;
        match &ast_node.kind {
            LangNodeKind::Expr(expr) => match expr {
                Expr::Ident(id) => {
                    let name = ast.get_symbol(id.0).name.clone();
                    self.vars.get(&name).cloned()
                }
                Expr::Deref(d) => {
                    let hir_inner_node = (*d).into();
                    let v = self.handle(ast, hir_inner_node)?;
                    Some(v)
                }
                Expr::FieldAccess { base, field } => {
                    let mut base_obj = self.compile_lvalue(ast, HirNodeId::from(*base))?;
                    let mut base_ty = ast.get_type(HirNodeId::from(*base))?;
                    if let Type::Ptr(inner) = base_ty {
                        base_ty = *inner;
                        let load = self
                            .state
                            .builder
                            .build_load(
                                self.to_llvm_type(&Type::Ptr(Box::new(base_ty.clone())))
                                    .into_basic_type()
                                    .into_pointer_type(),
                                base_obj.value.into_basic_value().into_pointer_value(),
                                "deref_base_lval",
                            )
                            .unwrap();
                        base_obj = self.create_obj(load.into());
                    }
                    if let Type::AggregateType(ref name, _, TypeDeclKind::Struct) = base_ty {
                        let struct_info = self.aggregates.get(name)?;
                        let field_idx = struct_info.fields.get(&ast.get_symbol(field.0).name)?;
                        let gep = self
                            .state
                            .builder
                            .build_struct_gep(
                                struct_info.ty,
                                base_obj.value.into_basic_value().into_pointer_value(),
                                *field_idx,
                                "fieldtmp",
                            )
                            .ok()?;
                        Some(self.create_obj(gep.as_basic_value_enum().into()))
                    } else if let Type::AggregateType(name, fields, TypeDeclKind::Union) = base_ty {
                        let field_name = ast.get_symbol(field.0).name.clone();
                        let union_info = self.aggregates.get(&name)?;
                        let field_idx = union_info.fields.get(&field_name)?;
                        let field_ty = fields.get_field(&field_name)?;
                        let llvm_field_ty = self.to_llvm_type(field_ty);

                        let gep = self
                            .state
                            .builder
                            .build_struct_gep(
                                llvm_field_ty.into_basic_type(),
                                base_obj.value.into_basic_value().into_pointer_value(),
                                *field_idx,
                                "fieldtmp",
                            )
                            .ok()?;
                        Some(self.create_obj(gep.as_basic_value_enum().into()))
                    } else {
                        println!("Field access on non-struct/union type: {:?}", base_ty);
                        None
                    }
                }
                Expr::Index { base, index } => unsafe {
                    let base_obj = self.compile_lvalue(ast, HirNodeId::from(*base))?;
                    let index_obj = self.handle(ast, HirNodeId::from(*index))?;
                    let base_ty = ast.get_type(HirNodeId::from(*base))?;
                    if let Type::List(elem_ty, _) = base_ty {
                        let llvm_elem_ty = self.to_llvm_type(&elem_ty);
                        let gep = self
                            .state
                            .builder
                            .build_gep(
                                llvm_elem_ty.into_basic_type(),
                                base_obj.value.into_basic_value().into_pointer_value(),
                                &[index_obj.value.into_basic_value().into_int_value()],
                                "indextmp",
                            )
                            .ok()?;
                        Some(self.create_obj(gep.as_basic_value_enum().into()))
                    } else if let Type::Ptr(inner_ty) = base_ty {
                        let llvm_ptr_ty = self.to_llvm_type(&Type::Ptr(inner_ty.clone()));
                        let llvm_inner_ty = self.to_llvm_type(&inner_ty);
                        let actual_ptr = self
                            .state
                            .builder
                            .build_load(
                                llvm_ptr_ty.into_basic_type().into_pointer_type(),
                                base_obj.value.into_basic_value().into_pointer_value(),
                                "load_ptr_for_index",
                            )
                            .unwrap()
                            .into_pointer_value();

                        let gep = self
                            .state
                            .builder
                            .build_gep(
                                llvm_inner_ty.into_basic_type(),
                                actual_ptr,
                                &[index_obj.value.into_basic_value().into_int_value()],
                                "indextmp",
                            )
                            .ok()?;
                        Some(self.create_obj(gep.as_basic_value_enum().into()))
                    } else {
                        println!("Indexing on non-list/non-pointer type: {:?}", base_ty);
                        None
                    }
                },
                _ => None,
            },
            _ => None,
        }
    }

    pub fn compile_cmp(
        &mut self,
        ast: &Hir,
        left_id: HirNodeId,
        right_id: HirNodeId,
        predicate: inkwell::IntPredicate,
    ) -> Option<CodegenObjLLVM<'state>> {
        let left = self.handle(ast, left_id)?;
        let right = self.handle(ast, right_id)?;

        let left_val = left.value.into_basic_value().into_int_value();
        let right_val = right.value.into_basic_value().into_int_value();

        Some(
            self.create_obj(
                self.state
                    .builder
                    .build_int_compare(predicate, left_val, right_val, "cmptmp")
                    .ok()?
                    .as_basic_value_enum()
                    .into(),
            ),
        )
    }
}

impl<'state> Layer for PopperCodegenLLVM<'state> {
    type Inner = Hir;
    type Output = Option<CodegenObjLLVM<'state>>;

    fn handle(&mut self, ast: &Hir, node_id: HirNodeId) -> Option<CodegenObjLLVM<'state>> {
        if ast.is_unreachable(node_id) {
            return None;
        }
        let node = ast.get(node_id);

        let ast_node = &node.node;
        match &ast_node.kind {
            LangNodeKind::Expr(expr) => match expr {
                Expr::Ref(r) => {
                    let hir_node = (*r).into();
                    let value = self.compile_lvalue(ast, hir_node)?;
                    Some(value)
                }
                Expr::Deref(d) => {
                    let hir_inner_node = (*d).into();
                    let ty = ast.get_type(node_id).unwrap();
                    let v = self.handle(ast, hir_inner_node)?;
                    let llvm_v = self
                        .state
                        .builder
                        .build_load(
                            self.to_llvm_type(&ty).into_basic_type(),
                            v.value.into_basic_value().into_pointer_value(),
                            "loadtmp",
                        )
                        .unwrap();
                    Some(self.create_obj(llvm_v.into()))
                }
                Expr::Char(c) => Some(
                    self.create_obj(
                        self.ctx
                            .i8_type()
                            .const_int(*c as u64, false)
                            .as_basic_value_enum()
                            .into(),
                    ),
                ),
                Expr::Int(i) => {
                    let expected_ty = ast.get_type(node_id);
                    if let Some(Type::Char) = expected_ty {
                        Some(
                            self.create_obj(
                                self.ctx
                                    .i8_type()
                                    .const_int(*i as u64, false)
                                    .as_basic_value_enum()
                                    .into(),
                            ),
                        )
                    } else {
                        Some(
                            self.create_obj(
                                self.ctx
                                    .i64_type()
                                    .const_int(*i as u64, false)
                                    .as_basic_value_enum()
                                    .into(),
                            ),
                        )
                    }
                }
                Expr::String(s) => {
                    let bytes = s.as_bytes();
                    let i8_type = self.ctx.i8_type();
                    let mut char_vals = Vec::with_capacity(bytes.len() + 1);
                    for &b in bytes {
                        char_vals.push(i8_type.const_int(b as u64, false));
                    }
                    char_vals.push(i8_type.const_int(0, false));

                    let str_array = i8_type.const_array(&char_vals);
                    let global = self
                        .state
                        .module
                        .add_global(str_array.get_type(), None, ".str.");
                    global.set_initializer(&str_array);
                    global.set_linkage(inkwell::module::Linkage::Internal);
                    global.set_constant(true);

                    Some(self.create_obj(global.as_pointer_value().as_basic_value_enum().into()))
                }
                Expr::Bool(b) => Some(
                    self.create_obj(
                        self.ctx
                            .bool_type()
                            .const_int(*b as u64, false)
                            .as_basic_value_enum()
                            .into(),
                    ),
                ),
                Expr::Ident(id) => {
                    let name = ast.get_symbol(id.0);
                    if let Some(var) = self.vars.get(&name.name) {
                        let ty = ast.get_type(node_id)?;
                        if var.value.is_function_value() {
                            return Some(var.clone());
                        }
                        let llvm_v = self
                            .state
                            .builder
                            .build_load(
                                self.to_llvm_type(&ty).into_basic_type(),
                                var.value.clone().into_basic_value().into_pointer_value(),
                                "loadtmp",
                            )
                            .unwrap();
                        Some(self.create_obj(llvm_v.into()).with_popper_ty(ty))
                    } else {
                        println!("Unload {:?}", name);
                        None
                    }
                }
                Expr::Add(left_id, right_id) => unsafe {
                    let math_op_kind = ast.get_math_op_kind(node_id).unwrap();
                    let left = self.handle(ast, HirNodeId::from(*left_id))?;
                    let right = self.handle(ast, HirNodeId::from(*right_id))?;
                    // get type of left and right and check if they are int or pointer and do the appropriate operation
                    let left_ty = ast.get_type(HirNodeId::from(*left_id))?;
                    let right_ty = ast.get_type(HirNodeId::from(*right_id))?;
                    if math_op_kind == MathOpKind::IntAndInt {
                        let left_val = left.value.into_basic_value().into_int_value();
                        let right_val = right.value.into_basic_value().into_int_value();

                        Some(
                            self.create_obj(
                                self.state
                                    .builder
                                    .build_int_add(left_val, right_val, "addtmp")
                                    .ok()?
                                    .as_basic_value_enum()
                                    .into(),
                            ),
                        )
                    } else if math_op_kind == MathOpKind::PtrAndInt {
                        let left_val = left.value.into_basic_value().into_pointer_value();
                        let right_val = right.value.into_basic_value().into_int_value();

                        let inner_type = if let Type::Ptr(inner) = left_ty {
                            inner
                        } else {
                            panic!("Expected pointer type for left operand, got {:?}", left_ty);
                        };

                        let llvm_inner_type = self.to_llvm_type(&inner_type);

                        Some(
                            self.create_obj(
                                self.state
                                    .builder
                                    .build_gep(
                                        llvm_inner_type.into_basic_type(),
                                        left_val,
                                        &[right_val],
                                        "ptraddtmp",
                                    )
                                    .ok()?
                                    .as_basic_value_enum()
                                    .into(),
                            ),
                        )
                    } else if math_op_kind == MathOpKind::IntAndPtr {
                        let left_val = left.value.into_basic_value().into_int_value();
                        let right_val = right.value.into_basic_value().into_pointer_value();

                        let inner_type = if let Type::Ptr(inner) = right_ty {
                            inner
                        } else {
                            panic!(
                                "Expected pointer type for right operand, got {:?}",
                                right_ty
                            );
                        };

                        let llvm_inner_type = self.to_llvm_type(&inner_type);

                        Some(
                            self.create_obj(
                                self.state
                                    .builder
                                    .build_gep(
                                        llvm_inner_type.into_basic_type(),
                                        right_val,
                                        &[left_val],
                                        "ptraddtmp",
                                    )
                                    .ok()?
                                    .as_basic_value_enum()
                                    .into(),
                            ),
                        )
                    } else {
                        println!("Unsupported math operation: {:?}", math_op_kind);
                        None
                    }
                },
                Expr::Sub(left_id, right_id) => {
                    let left = self.handle(ast, HirNodeId::from(*left_id))?;
                    let right = self.handle(ast, HirNodeId::from(*right_id))?;

                    let left_val = left.value.into_basic_value().into_int_value();
                    let right_val = right.value.into_basic_value().into_int_value();

                    Some(
                        self.create_obj(
                            self.state
                                .builder
                                .build_int_sub(left_val, right_val, "subtmp")
                                .ok()?
                                .as_basic_value_enum()
                                .into(),
                        ),
                    )
                }
                Expr::Mul(left_id, right_id) => {
                    let left = self.handle(ast, HirNodeId::from(*left_id))?;
                    let right = self.handle(ast, HirNodeId::from(*right_id))?;

                    let left_val = left.value.into_basic_value().into_int_value();
                    let right_val = right.value.into_basic_value().into_int_value();

                    Some(
                        self.create_obj(
                            self.state
                                .builder
                                .build_int_mul(left_val, right_val, "multmp")
                                .ok()?
                                .as_basic_value_enum()
                                .into(),
                        ),
                    )
                }
                Expr::Eq(left_id, right_id) => self.compile_cmp(
                    ast,
                    HirNodeId::from(*left_id),
                    HirNodeId::from(*right_id),
                    inkwell::IntPredicate::EQ,
                ),
                Expr::Gt(left_id, right_id) => self.compile_cmp(
                    ast,
                    HirNodeId::from(*left_id),
                    HirNodeId::from(*right_id),
                    inkwell::IntPredicate::SGT,
                ),
                Expr::Lt(left_id, right_id) => self.compile_cmp(
                    ast,
                    HirNodeId::from(*left_id),
                    HirNodeId::from(*right_id),
                    inkwell::IntPredicate::SLT,
                ),
                Expr::GtEq(left_id, right_id) => self.compile_cmp(
                    ast,
                    HirNodeId::from(*left_id),
                    HirNodeId::from(*right_id),
                    inkwell::IntPredicate::SGE,
                ),
                Expr::LtEq(left_id, right_id) => self.compile_cmp(
                    ast,
                    HirNodeId::from(*left_id),
                    HirNodeId::from(*right_id),
                    inkwell::IntPredicate::SLE,
                ),
                Expr::FieldAccess { base, field } => unsafe {
                    let mut base_obj = self.compile_lvalue(ast, HirNodeId::from(*base))?;
                    let mut base_ty = ast.get_type(HirNodeId::from(*base))?;
                    if let Type::Ptr(inner) = base_ty {
                        base_ty = *inner;
                        let load = self
                            .state
                            .builder
                            .build_load(
                                self.to_llvm_type(&Type::Ptr(Box::new(base_ty.clone())))
                                    .into_basic_type()
                                    .into_pointer_type(),
                                base_obj.value.into_basic_value().into_pointer_value(),
                                "deref_base_rval",
                            )
                            .unwrap();
                        base_obj = self.create_obj(load.into());
                    }
                    if let Type::AggregateType(ref name, _, TypeDeclKind::Struct) = base_ty {
                        let struct_info = self.aggregates.get(name)?;
                        let field_name = ast.get_symbol(field.0).name.clone();
                        let field_idx = struct_info.fields.get(&field_name)?;
                        let gep = self
                            .state
                            .builder
                            .build_struct_gep(
                                struct_info.ty,
                                base_obj.value.into_basic_value().into_pointer_value(),
                                *field_idx,
                                "fieldtmp",
                            )
                            .ok()?;

                        let ty = struct_info.ty.get_field_type_at_index(*field_idx).unwrap();
                        // load the field to return it as a value
                        let field_value = self
                            .state
                            .builder
                            .build_load(ty, gep, "fieldloadtmp")
                            .unwrap();

                        Some(self.create_obj(field_value.into()))
                    } else if let Type::AggregateType(name, fields, TypeDeclKind::Union) = base_ty {
                        let field_name = ast.get_symbol(field.0).name.clone();
                        let field_ty = fields.get_field(&field_name)?;
                        let union_info = self.aggregates.get(&name).unwrap();
                        let array = self
                            .state
                            .builder
                            .build_struct_gep(
                                union_info.ty.clone(),
                                base_obj.value.into_basic_value().into_pointer_value(),
                                0,
                                "fieldtmp",
                            )
                            .unwrap();
                        let field_llvm_ty = self.to_llvm_type(field_ty);
                        let field_ptr = self
                            .state
                            .builder
                            .build_load(field_llvm_ty.into_basic_type(), array, "fieldloadtmp")
                            .unwrap();

                        Some(self.create_obj(field_ptr.into()))
                    } else {
                        println!("Field access on non-struct type: {:?}", base_ty);
                        None
                    }
                },
                Expr::TypeDeclInstance {
                    type_name: struct_name,
                    fields,
                } => {
                    let struct_ty = ast.get_type(node_id)?;
                    if let Type::AggregateType(_, _, TypeDeclKind::Struct) = struct_ty {
                        let struct_name = ast.get_symbol(struct_name.0).name.clone();
                        let struct_info = self.aggregates.get(&struct_name)?.clone();
                        let struct_alloca = self
                            .state
                            .builder
                            .build_alloca(struct_info.ty, "structtmp")
                            .ok()?;

                        for (field_ident, field_value_id) in fields {
                            let field_name = ast.get_symbol(field_ident.0).name.clone();
                            let field_idx = struct_info.fields.get(&field_name)?;
                            let gep = self
                                .state
                                .builder
                                .build_struct_gep(
                                    struct_info.ty,
                                    struct_alloca,
                                    *field_idx,
                                    "fieldtmp",
                                )
                                .ok()?;

                            let field_value = self.handle(ast, HirNodeId::from(*field_value_id))?;
                            self.state
                                .builder
                                .build_store(gep, field_value.value.into_basic_value())
                                .unwrap();
                        }

                        // load the struct instance to return it as a value
                        let struct_instance = self
                            .state
                            .builder
                            .build_load(struct_info.ty, struct_alloca, "structinstancetmp")
                            .ok()?;

                        Some(self.create_obj(struct_instance.into()))
                    } else if let Type::AggregateType(ref name, _, TypeDeclKind::Union) = struct_ty
                    {
                        let llvm_ty = self.to_llvm_type(&struct_ty);
                        let v = self.handle(ast, HirNodeId::from(fields.first().unwrap().1))?;
                        let aggregate = self.aggregates.get(name).cloned().unwrap();
                        let alloca = self
                            .state
                            .builder
                            .build_alloca(llvm_ty.into_basic_type(), "uniontmp")
                            .ok()?;

                        let first_field = self
                            .state
                            .builder
                            .build_struct_gep(aggregate.ty, alloca, 0, "unionfieldtmp")
                            .unwrap();

                        self.state
                            .builder
                            .build_store(first_field, v.value.into_basic_value())
                            .unwrap();
                        let union_instance = self
                            .state
                            .builder
                            .build_load(aggregate.ty, alloca, "unioninstancetmp")
                            .unwrap();

                        Some(self.create_obj(union_instance.into()))
                    } else {
                        panic!(
                            "Type declaration instance for non-struct/union type: {:?}",
                            struct_ty
                        );
                    }
                }
                Expr::List(elements) => {
                    let ptr_ty = ast.get_type(node_id).unwrap();
                    if let Type::List(elem_ty, count) = ptr_ty {
                        let llvm_elem_ty = self.to_llvm_type(&*elem_ty).into_basic_type();
                        let array_ty = llvm_elem_ty.array_type(count as u32);
                        let array_alloca =
                            self.state.builder.build_alloca(array_ty, "listtmp").ok()?;

                        for (i, elem_id) in elements.iter().enumerate() {
                            let elem_value = self.handle(ast, HirNodeId::from(*elem_id))?;
                            let elem_ptr = unsafe {
                                self.state
                                    .builder
                                    .build_gep(
                                        llvm_elem_ty,
                                        array_alloca,
                                        &[self.ctx.i32_type().const_int(i as u64, false)],
                                        "elemptrtmp",
                                    )
                                    .unwrap()
                            };
                            self.state
                                .builder
                                .build_store(elem_ptr, elem_value.value.into_basic_value())
                                .unwrap();
                        }

                        // load the array to return it as a value
                        let array_instance = self
                            .state
                            .builder
                            .build_load(array_ty, array_alloca, "listinstancetmp")
                            .ok()?;

                        Some(self.create_obj(array_instance.into()))
                    } else {
                        println!("List literal with non-list type: {:?}", ptr_ty);
                        None
                    }
                }
                Expr::BuiltinCall { name, args } => match name.as_str() {
                    "vaIs" => {
                        let va_ctx = self
                            .current_va_context
                            .as_ref()
                            .expect("Cannot use @vaIs outside a variadic function");
                        let current_index = self
                            .state
                            .builder
                            .build_load(self.ctx.i64_type(), va_ctx.index_ptr, "idx")
                            .unwrap()
                            .into_int_value();

                        let type_ptr = unsafe {
                            self.state
                                .builder
                                .build_in_bounds_gep(
                                    self.ctx.i64_type(),
                                    va_ctx.types_ptr,
                                    &[current_index],
                                    "typtr",
                                )
                                .unwrap()
                        };
                        let actual_type_id = self
                            .state
                            .builder
                            .build_load(self.ctx.i64_type(), type_ptr, "tyval")
                            .unwrap()
                            .into_int_value();

                        let expected_ty = ast.get_type(HirNodeId::from(args[0])).unwrap();
                        let expected_ty = if let Type::Type(Some(t)) = expected_ty {
                            *t.ty
                        } else {
                            expected_ty
                        };
                        let expected_id = crate::type_hash(&expected_ty);

                        let cmp = self
                            .state
                            .builder
                            .build_int_compare(
                                inkwell::IntPredicate::EQ,
                                actual_type_id,
                                self.ctx.i64_type().const_int(expected_id, false),
                                "iscmp",
                            )
                            .unwrap();
                        Some(
                            self.create_obj(cmp.as_basic_value_enum().into())
                                .with_popper_ty(Type::Bool),
                        )
                    }
                    "vaLen" => {
                        let va_ctx = self
                            .current_va_context
                            .as_ref()
                            .expect("Cannot use @vaLen outside a variadic function");
                        Some(
                            self.create_obj(va_ctx.count_val.as_basic_value_enum().into())
                                .with_popper_ty(Type::Int),
                        )
                    }
                    "vaArg" => {
                        let va_ctx = self
                            .current_va_context
                            .as_ref()
                            .expect("Cannot use @vaArg outside a variadic function");
                        let ty = ast.get_type(node_id).unwrap();
                        let llvm_ty = self.to_llvm_type(&ty).into_basic_type();

                        let current_index = self
                            .state
                            .builder
                            .build_load(self.ctx.i64_type(), va_ctx.index_ptr, "idx")
                            .unwrap()
                            .into_int_value();

                        let arg_ptr_ptr = unsafe {
                            self.state
                                .builder
                                .build_in_bounds_gep(
                                    self.ctx.i8_type().ptr_type(AddressSpace::default()),
                                    va_ctx.args_ptr,
                                    &[current_index],
                                    "argptrptr",
                                )
                                .unwrap()
                        };
                        let arg_i8_ptr = self
                            .state
                            .builder
                            .build_load(
                                self.ctx.ptr_type(AddressSpace::default()),
                                arg_ptr_ptr,
                                "argi8ptr",
                            )
                            .unwrap()
                            .into_pointer_value();

                        let typed_ptr = self
                            .state
                            .builder
                            .build_bit_cast(
                                arg_i8_ptr,
                                llvm_ty.ptr_type(AddressSpace::default()),
                                "typedptr",
                            )
                            .unwrap()
                            .into_pointer_value();
                        let final_val = self
                            .state
                            .builder
                            .build_load(llvm_ty, typed_ptr, "finalval")
                            .unwrap();

                        let next_index = self
                            .state
                            .builder
                            .build_int_add(
                                current_index,
                                self.ctx.i64_type().const_int(1, false),
                                "nextidx",
                            )
                            .unwrap();
                        self.state
                            .builder
                            .build_store(va_ctx.index_ptr, next_index)
                            .unwrap();

                        Some(self.create_obj(final_val.into()).with_popper_ty(ty))
                    }
                    _ => None,
                },
                Expr::Index { base, index } => {
                    let base_obj = self.compile_lvalue(ast, HirNodeId::from(*base))?;
                    let base_ty = ast.get_type(HirNodeId::from(*base))?;
                    if let Type::List(elem_ty, _) = base_ty {
                        let llvm_elem_ty = self.to_llvm_type(&*elem_ty).into_basic_type();
                        let index_value = self.handle(ast, HirNodeId::from(*index))?;
                        let index_val = index_value.value.into_basic_value().into_int_value();
                        let elem_ptr = unsafe {
                            self.state
                                .builder
                                .build_gep(
                                    llvm_elem_ty,
                                    base_obj.value.into_basic_value().into_pointer_value(),
                                    &[index_val],
                                    "elemptrtmp",
                                )
                                .unwrap()
                        };
                        let elem_value = self
                            .state
                            .builder
                            .build_load(llvm_elem_ty, elem_ptr, "elemloadtmp")
                            .ok()?;

                        Some(self.create_obj(elem_value.into()))
                    } else if let Type::Ptr(inner_ty) = base_ty {
                        let llvm_ptr_ty =
                            self.to_llvm_type(&Type::Ptr(Box::new((*inner_ty).clone())));
                        let llvm_inner_ty = self.to_llvm_type(&inner_ty).into_basic_type();
                        let index_value = self.handle(ast, HirNodeId::from(*index))?;
                        let index_val = index_value.value.into_basic_value().into_int_value();

                        let actual_ptr = self
                            .state
                            .builder
                            .build_load(
                                llvm_ptr_ty.into_basic_type().into_pointer_type(),
                                base_obj.value.into_basic_value().into_pointer_value(),
                                "load_ptr_for_index",
                            )
                            .unwrap()
                            .into_pointer_value();

                        let elem_ptr = unsafe {
                            self.state
                                .builder
                                .build_gep(llvm_inner_ty, actual_ptr, &[index_val], "elemptrtmp")
                                .unwrap()
                        };
                        let elem_value = self
                            .state
                            .builder
                            .build_load(llvm_inner_ty, elem_ptr, "elemloadtmp")
                            .ok()?;

                        Some(self.create_obj(elem_value.into()))
                    } else {
                        println!("Indexing on non-list/non-pointer type: {:?}", base_ty);
                        None
                    }
                }
                e => {
                    println!("Skipping {:?}", e);
                    None
                }
            },

            LangNodeKind::Block(b) => {
                for stmt_id in b {
                    self.handle(ast, HirNodeId::from(*stmt_id));
                }
                None
            }
            LangNodeKind::Let(l) => {
                let value = self.handle(ast, HirNodeId::from(l.value))?;
                let ty = ast.get_type(node_id)?;
                let llvm_ty = self.to_llvm_type(&ty);
                let name = &ast.get_symbol(l.name.0).name;
                let alloca = self
                    .state
                    .builder
                    .build_alloca(llvm_ty.into_basic_type(), &name)
                    .ok()?;
                self.state
                    .builder
                    .build_store(alloca, value.value.clone().into_basic_value())
                    .unwrap();
                self.vars.insert(
                    name.clone(),
                    self.create_obj(alloca.as_basic_value_enum().into()),
                );
                Some(value)
            }
            LangNodeKind::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let cond = self.handle(ast, HirNodeId::from(*condition))?;
                let cond_val = cond.value.into_basic_value().into_int_value();

                let function = self.state.builder.get_insert_block()?.get_parent()?;

                let then_bb = self.ctx.append_basic_block(function, "then");
                let else_bb = self.ctx.append_basic_block(function, "else");
                let mut merge_bb = None;
                self.state
                    .builder
                    .build_conditional_branch(cond_val, then_bb, else_bb)
                    .unwrap();

                self.state.builder.position_at_end(then_bb);
                self.handle(ast, HirNodeId::from(*then_branch));
                let bb = self.state.builder.get_insert_block().unwrap();
                if bb.get_terminator().is_none() {
                    let new_merge_bb = self.ctx.append_basic_block(function, "ifcont");
                    self.state
                        .builder
                        .build_unconditional_branch(new_merge_bb)
                        .unwrap();
                    merge_bb = Some(new_merge_bb);
                }

                self.state.builder.position_at_end(else_bb);
                if let Some(else_branch_id) = else_branch {
                    self.handle(ast, HirNodeId::from(*else_branch_id));
                    let bb = self.state.builder.get_insert_block().unwrap();
                    if bb.get_terminator().is_none() {
                        if merge_bb.is_none() {
                            merge_bb = Some(self.ctx.append_basic_block(function, "ifcont"));
                        }
                        self.state
                            .builder
                            .build_unconditional_branch(merge_bb.unwrap())
                            .unwrap();
                    }
                } else {
                    if merge_bb.is_none() {
                        merge_bb = Some(self.ctx.append_basic_block(function, "ifcont"));
                    }
                    self.state
                        .builder
                        .build_unconditional_branch(merge_bb.unwrap())
                        .unwrap();
                }

                if merge_bb.is_some() {
                    self.state.builder.position_at_end(merge_bb.unwrap());
                }
                None
            }
            LangNodeKind::ExtendDecl {
                target_type,
                methodes,
            } => {
                self.parent_ty = Some(target_type.clone());
                for method in methodes {
                    self.handle(ast, HirNodeId::from(*method));
                }
                None
            }
            LangNodeKind::ContextDecl { name, extensions } => {
                let name = ast.get_symbol(name.0).name.clone();
                self.parent_ctx = Some(name);
                for ext in extensions {
                    self.handle(ast, HirNodeId::from(*ext));
                }
                self.parent_ctx = None;
                None
            }
            LangNodeKind::InContext { context_name, body } => {
                let context_name = ast.get_symbol(context_name.0).name.clone();
                self.parent_ctx = Some(context_name);
                let res = self.handle(ast, HirNodeId::from(*body));
                self.parent_ctx = None;
                res
            }
            LangNodeKind::FunctionDef {
                name,
                attrs,
                params,
                ret,
                body,
                is_expr: _,
                is_vararg,
            } => {
                let name = ast.get_symbol(name.0).name.clone();
                let name = if let Some(parent_ctx) = &self.parent_ctx {
                    let parent_ty = self.parent_ty.as_ref().unwrap();
                    let is_static = attrs.contains(&popper_ast::attribute::Attribute::Static);
                    let parent_ty_info = parent_ty.get_type_info();
                    dbg!(if is_static {
                        mangle_static_method(parent_ctx, &parent_ty_info.name, &name)
                    } else {
                        mangle_method(parent_ctx, &parent_ty_info.name, &name)
                    })
                } else {
                    name.clone()
                };
                let fn_hir_ty = ast.get_type(node_id).unwrap();
                let (param_types_from_hir, ret_ty_from_hir) =
                    if let popper_ast::type_::Type::Function(p, r, ..) = fn_hir_ty {
                        (p, *r)
                    } else {
                        unreachable!()
                    };

                let ret_ty = self.to_llvm_type(&ret_ty_from_hir);
                let is_c_vararg = attrs.contains(&popper_ast::attribute::Attribute::StdCallC);
                let mut param_types = param_types_from_hir
                    .iter()
                    .map(|ty| Some(self.to_llvm_type(ty).into_metadata_type()))
                    .collect::<Option<Vec<_>>>()?;

                if *is_vararg && !is_c_vararg {
                    param_types.push(self.ctx.ptr_type(AddressSpace::default()).into());
                    param_types.push(self.ctx.ptr_type(AddressSpace::default()).into());
                    param_types.push(self.ctx.i64_type().into());
                }

                let fn_type = if ret_ty.is_void_type() {
                    ret_ty
                        .into_void_type()
                        .fn_type(param_types.as_slice(), is_c_vararg)
                } else {
                    ret_ty
                        .into_basic_type()
                        .fn_type(param_types.as_slice(), is_c_vararg)
                };
                let function = self.state.module.add_function(&name, fn_type, None);
                let obj = self.create_obj(function.into());
                self.vars.insert(name, obj.clone());
                if let Some(body) = body {
                    let entry_bb = self.ctx.append_basic_block(function, "entry");
                    self.state.builder.position_at_end(entry_bb);

                    for (i, param) in params.iter().enumerate() {
                        let name = ast.get_symbol(param.name.0).name.clone();
                        let llvm_param = function.get_nth_param(i as u32).unwrap();
                        llvm_param.set_name(&name);
                        let ty = param.ty.clone();
                        let llvm_ty = self.to_llvm_type(&ty);
                        let alloca = self
                            .state
                            .builder
                            .build_alloca(llvm_ty.into_basic_type(), &name)
                            .ok()?;
                        self.state.builder.build_store(alloca, llvm_param).unwrap();
                        self.vars.insert(
                            name,
                            self.create_obj_with_ty(alloca.as_basic_value_enum().into(), ty),
                        );
                    }
                    if *is_vararg && !is_c_vararg {
                        let index_alloca = self
                            .state
                            .builder
                            .build_alloca(self.ctx.i64_type(), "va_index")
                            .unwrap();
                        self.state
                            .builder
                            .build_store(index_alloca, self.ctx.i64_type().const_int(0, false))
                            .unwrap();

                        let param_count = function.count_params();
                        let types_ptr = function
                            .get_nth_param(param_count - 3)
                            .unwrap()
                            .into_pointer_value();
                        let args_ptr = function
                            .get_nth_param(param_count - 2)
                            .unwrap()
                            .into_pointer_value();

                        let count_val = function
                            .get_nth_param(param_count - 1)
                            .unwrap()
                            .into_int_value();

                        self.current_va_context = Some(VaContext {
                            types_ptr,
                            args_ptr,
                            index_ptr: index_alloca,
                            count_val,
                        });
                    } else if *is_vararg && is_c_vararg {
                        // Normally we don't define C varargs in Popper body, but just in case
                        // (ignoring va_start for now)
                    }
                    self.handle(ast, HirNodeId::from(*body));
                    self.current_va_context = None;
                    if ret.clone() == Type::Void {
                        self.state.builder.build_return(None).unwrap();
                    }
                }

                Some(obj)
            }
            LangNodeKind::TypeDecl { name, fields, kind } => {
                let name = ast.get_symbol(name.0).name.clone();
                let mut f = Vec::new();
                for field in fields {
                    f.push((ast.get_symbol(field.name.0).name.clone(), field.ty.clone()));
                }
                let ty = Type::AggregateType(name.clone(), Fields { fields: f }, *kind);
                let ty = self.to_llvm_type(&ty);
                let field_map = fields
                    .iter()
                    .enumerate()
                    .map(|(i, field)| (ast.get_symbol(field.name.0).name.clone(), i as u32))
                    .collect::<HashMap<_, _>>();
                self.aggregates.insert(
                    name,
                    AggregateInfo {
                        ty: ty.into_basic_type().into_struct_type(),
                        fields: field_map,
                        type_decl_kind: *kind,
                    },
                );
                None
            }
            LangNodeKind::Return(ret) => {
                let obj = self.handle(ast, HirNodeId::from(*ret))?;

                self.state()
                    .builder
                    .build_return(Some(&obj.value.into_basic_value()))
                    .expect("TODO: panic message");
                None
            }
            LangNodeKind::FunctionCall { function, args } => {
                let mut is_instance_method = false;
                let mut is_static_method = false;
                let mut base_obj = None;
                let mut func_name = None;

                let func_node = ast.get(HirNodeId::from(*function));
                if let LangNodeKind::Expr(popper_ast::ast::Expr::FieldAccess { base, field }) =
                    &func_node.node.kind
                {
                    let mut base_ty = ast.get_type(HirNodeId::from(*base)).unwrap();
                    let field_name = ast.get_symbol(field.0).name.clone();
                    let current_ctx = self.parent_ctx.as_ref().unwrap();

                    if let Type::Type(Some(t)) = base_ty {
                        is_static_method = true;

                        func_name = Some(mangle_static_method(current_ctx, &t.name, &field_name));
                    } else {
                        let mut is_field = false;
                        if let Type::AggregateType(ref name, _, _) = base_ty {
                            if let Some(struct_info) = self.aggregates.get(name) {
                                if struct_info.fields.get(&field_name).is_some() {
                                    is_field = true;
                                }
                            }
                        }
                        if !is_field {
                            is_instance_method = true;
                            let base_hir_ty = ast.get_type(HirNodeId::from(*base)).unwrap();
                            let base_is_ptr =
                                matches!(base_hir_ty, popper_ast::type_::Type::Ptr(_));

                            if !base_is_ptr {
                                if let Some(lval) = self.compile_lvalue(ast, HirNodeId::from(*base))
                                {
                                    base_obj = Some(lval);
                                } else {
                                    base_obj = Some(self.handle(ast, HirNodeId::from(*base))?);
                                }
                            } else {
                                base_obj = Some(self.handle(ast, HirNodeId::from(*base))?);
                            }
                            let current_ctx = self.parent_ctx.as_ref().unwrap();
                            func_name = Some(mangle_method(
                                &current_ctx,
                                &base_ty.get_type_info().name,
                                &field_name,
                            ))
                        }
                    }
                }

                let func_obj = if is_instance_method || is_static_method {
                    let name = func_name.unwrap();
                    let fn_val = self.state.module.get_function(&name).unwrap_or_else(|| {
                        panic!("Failed to find method function '{}' in LLVM module", name)
                    });
                    self.create_obj(fn_val.into())
                } else {
                    self.handle(ast, HirNodeId::from(*function))?
                };

                let func = func_obj.value.into_function_value();
                let func_ty = ast.get_type(HirNodeId::from(*function)).unwrap();
                let (is_popper_vararg, is_c_vararg, fixed_args_count) =
                    if let Type::Function(params, _, v, c, ..) = func_ty {
                        (v, c, params.len())
                    } else {
                        (false, false, 0)
                    };

                let mut llvm_args = Vec::new();
                if let Some(base) = base_obj {
                    llvm_args.push(base.value.into_metadata_value());
                }

                let expected_args_offset = if is_instance_method { 1 } else { 0 };
                for arg_id in args.iter().take(fixed_args_count - expected_args_offset) {
                    let arg = self.handle(ast, HirNodeId::from(*arg_id))?;
                    llvm_args.push(arg.value.into_metadata_value());
                }

                if is_popper_vararg && !is_c_vararg {
                    let va_count = args.len() - (fixed_args_count - expected_args_offset);
                    let va_types = self
                        .state
                        .builder
                        .build_array_alloca(
                            self.ctx.i64_type(),
                            self.ctx.i64_type().const_int(va_count as u64, false),
                            "va_types",
                        )
                        .unwrap();
                    let va_args = self
                        .state
                        .builder
                        .build_array_alloca(
                            self.ctx.i8_type().ptr_type(AddressSpace::default()),
                            self.ctx.i64_type().const_int(va_count as u64, false),
                            "va_args",
                        )
                        .unwrap();

                    for (i, arg_id) in args
                        .iter()
                        .skip(fixed_args_count - expected_args_offset)
                        .enumerate()
                    {
                        let arg_val = self.handle(ast, HirNodeId::from(*arg_id))?;
                        let arg_type = ast.get_type(HirNodeId::from(*arg_id)).unwrap();
                        let type_hash = crate::type_hash(&arg_type);

                        let type_ptr = unsafe {
                            self.state
                                .builder
                                .build_in_bounds_gep(
                                    self.ctx.i64_type(),
                                    va_types,
                                    &[self.ctx.i64_type().const_int(i as u64, false)],
                                    "typtr",
                                )
                                .unwrap()
                        };
                        self.state
                            .builder
                            .build_store(type_ptr, self.ctx.i64_type().const_int(type_hash, false))
                            .unwrap();

                        let val_alloca = self
                            .state
                            .builder
                            .build_alloca(self.to_llvm_type(&arg_type).into_basic_type(), "vtmp")
                            .unwrap();
                        self.state
                            .builder
                            .build_store(val_alloca, arg_val.value.into_basic_value())
                            .unwrap();
                        let i8_ptr = self
                            .state
                            .builder
                            .build_bit_cast(
                                val_alloca,
                                self.ctx.i8_type().ptr_type(AddressSpace::default()),
                                "casttmp",
                            )
                            .unwrap();

                        let arg_ptr = unsafe {
                            self.state
                                .builder
                                .build_in_bounds_gep(
                                    self.ctx.i8_type().ptr_type(AddressSpace::default()),
                                    va_args,
                                    &[self.ctx.i64_type().const_int(i as u64, false)],
                                    "argptr",
                                )
                                .unwrap()
                        };
                        self.state.builder.build_store(arg_ptr, i8_ptr).unwrap();
                    }

                    llvm_args.push(va_types.into());
                    llvm_args.push(va_args.into());
                    llvm_args.push(self.ctx.i64_type().const_int(va_count as u64, false).into());
                } else if is_c_vararg {
                    for arg_id in args.iter().skip(fixed_args_count) {
                        let arg = self.handle(ast, HirNodeId::from(*arg_id))?;
                        llvm_args.push(arg.value.into_metadata_value());
                    }
                }

                let call_site = self
                    .state
                    .builder
                    .build_call(func, &llvm_args, "calltmp")
                    .unwrap();

                let v = call_site.try_as_basic_value();
                if v.is_basic() {
                    Some(
                        self.create_obj(
                            v.expect_basic("BUG: expected basic value from function call")
                                .into(),
                        ),
                    )
                } else {
                    None
                }
            }
            LangNodeKind::Assign { lhs, rhs } => {
                let rhs_val = self.handle(ast, HirNodeId::from(*rhs))?;
                let lhs_obj = self.compile_lvalue(ast, HirNodeId::from(*lhs))?;

                self.state
                    .builder
                    .build_store(
                        lhs_obj.value.into_basic_value().into_pointer_value(),
                        rhs_val.value.clone().into_basic_value(),
                    )
                    .unwrap();
                Some(rhs_val)
            }
            LangNodeKind::While { condition, body } => {
                let function = self
                    .state
                    .builder
                    .get_insert_block()
                    .unwrap()
                    .get_parent()
                    .unwrap();

                let cond_bb = self.ctx.append_basic_block(function, "cond");
                let body_bb = self.ctx.append_basic_block(function, "body");
                let after_bb = self.ctx.append_basic_block(function, "after");

                self.state
                    .builder
                    .build_unconditional_branch(cond_bb)
                    .unwrap();

                self.state.builder.position_at_end(cond_bb);
                let cond_val = self
                    .handle(ast, HirNodeId::from(*condition))?
                    .value
                    .into_basic_value()
                    .into_int_value();
                self.state
                    .builder
                    .build_conditional_branch(cond_val, body_bb, after_bb)
                    .unwrap();

                self.state.builder.position_at_end(body_bb);
                self.handle(ast, HirNodeId::from(*body));
                if self
                    .state
                    .builder
                    .get_insert_block()
                    .unwrap()
                    .get_terminator()
                    .is_none()
                {
                    self.state
                        .builder
                        .build_unconditional_branch(cond_bb)
                        .unwrap();
                }

                self.state.builder.position_at_end(after_bb);
                None
            }
            e => {
                println!("Skipping {:?}", e);
                None
            }
        }
    }
}
