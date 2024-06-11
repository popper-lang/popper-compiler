use crate::consts::{ConstKind, Ident, TypeId};
use crate::expr::Expr;
use crate::labels::{Label, LabelId};
use crate::types::Types;


macro_rules! commands {
    (
        $(#[$attr:meta])*
        struct $name:ident($cname:ident) {
            $(
                $field_name:ident : $field_type:ty [$($field_arg_type:ident),*]
            ),*
        }
    ) => {
        $(#[$attr])*
        #[derive(Debug, Clone)]
        pub struct $name {
            $(
               pub $field_name: $field_type
            ),*
        }

        impl $name {
            pub fn new($($field_name: $field_type),*) -> Self {
                Self {
                    $($field_name),*
                }
            }
        }

        impl Command for $name {
            fn signature() -> Vec<ArgType> {
                vec![$( $(ArgType::$field_arg_type),* ),*]
            }

            fn name() -> String {
                String::from(stringify!($cname))
            }
        }
    };

    (
        $(
            $(#[$attr:meta])*
            struct $name:ident($cname:ident) {
                $(
                    $field_name:ident : $field_type:ty [$($field_arg_type:ident),*]
                ),*
            }
        )*
    ) => {
        $(
            commands! {
                $(#[$attr])*
                struct $name($cname) {
                    $(
                        $field_name: $field_type [$($field_arg_type),*]
                    ),*
                }
            }
        )*
    };
}
pub trait Command {
    fn signature() -> Vec<ArgType>;
    fn name() -> String;

    fn stringify(&self) -> String {
        String::from("")
    }


}


pub enum ArgType {
    Const,
    Ident,
    Label,
    Expr,
    None
}

impl ArgType {
    pub fn is_matching(&self, e: Expr) -> bool {
        matches!(
            (self, e),
            (ArgType::Const, Expr::Const(_)) | 
            (ArgType::Ident, Expr::Ident(_)) | 
            (ArgType::Label, Expr::Label(_)) |
            (ArgType::None, _) |
            (ArgType::Expr, _)
        )
    }

}

commands! {
    struct Const(const) {
        kind: ConstKind [Const]
    }

    struct Ref(ref) {
        ident: Ident [Ident]
    }

    struct LLVMLoadPtr(llvm_load_ptr) {
        ptr: Ident [Ident],
        as_type: Types [None]
    }
    
    struct LLVMStore(llvm_store) {
        ptr: Ident [Ident],
        as_type: Types [None]
    }

    struct CmpEq(cmp_eq) {
        left: Expr [Expr],
        right: Expr [Expr]
    }
    
    struct CmpNe(cmp_ne) {
        left: Expr [Expr],
        right: Expr [Expr]
    }
    
    struct CmpGt(cmp_gt) {
        left: Expr [Expr],
        right: Expr [Expr]
    }
    
    struct CmpLt(cmp_lt) {
        left: Expr [Expr],
        right: Expr [Expr]
    }
    
    struct CmpGe(cmp_ge) {
        left: Expr [Expr],
        right: Expr [Expr]
    }
    
    struct CmpLe(cmp_le) {
        left: Expr [Expr],
        right: Expr [Expr]
    }
    
    struct Add(add) {
        left: Expr [Expr],
        right: Expr [Expr]
    }
    
    struct Sub(sub) {
        left: Expr [Expr],
        right: Expr [Expr]
    }
    
    struct Mul(mul) {
        left: Expr [Expr],
        right: Expr [Expr]
    }
    
    struct Div(div) {
        left: Expr [Expr],
        right: Expr [Expr]
    }
    
    struct Br(br) {
        cond: Expr [Expr],
        true_branch: LabelId [Label],
        false_branch: LabelId [Label]
    }
    
    struct Call(call) {
        function: String [None],
        args: Vec<Expr> [Expr]
    }
    
    struct Ret(ret) {
        value: Expr [Expr]
    }
    
    struct CopyVal(copy_val) {
        val: Expr [Expr]
    }
    
    struct GetElementPtrStruct(get_element_ptr) {
        ptr: Ident [Ident],
        index: Expr [Expr],
        target_type: Types [None],
        struct_id: TypeId [None]
    }

}

#[derive(Debug, Clone)]
pub enum CommandEnum {
    Const(Const),
    Ref(Ref),
    LLVMLoadPtr(LLVMLoadPtr),
    LLVMStore(LLVMStore),
    CmpEq(CmpEq),
    CmpNe(CmpNe),
    CmpGt(CmpGt),
    CmpLt(CmpLt),
    CmpGe(CmpGe),
    CmpLe(CmpLe),
    Add(Add),
    Sub(Sub),
    Mul(Mul),
    Div(Div),
    Call(Call),
    Br(Br),
    Ret(Ret),
    CopyVal(CopyVal),
    GetElementPtrStruct(GetElementPtrStruct)
}

