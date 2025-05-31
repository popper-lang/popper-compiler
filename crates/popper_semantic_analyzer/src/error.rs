use popper_error_core::Diagnostics;
use popper_error_macro::Diagnostics;
use popper_ast::ast::Span;
use popper_ast::type_::Type;

#[derive(Diagnostics, Debug)]
#[message("Symbol `{name}` not found")]
#[code = 1001]
#[label = "Symbol not found"]
#[note = "Ensure the symbol is declared before use"]
pub struct SymbolNotFound {
    pub name: String,
    
    #[span]
    pub span: Span,
}

#[derive(Diagnostics, Debug)]
#[message("Type mismatch: expected `{expected}`, found `{found}`", )]
#[code = 1002]
#[label = "found {found}"]
#[note = "Check the types of the variables or expressions involved"]
pub struct TypeMismatch {
    pub expected: String,
    pub found: String,

    #[span]
    pub span: Span,
}

#[derive(Diagnostics, Debug)]
#[message("Expected a function but found `{name}` of type `{type_}`")]
#[code = 1003]
#[label = "Not a function"]
#[note = "Ensure the symbol is a function and not a variable or type"]
pub struct NotAFunction {
    pub name: String,
    pub type_: Type,
    
    #[span]
    pub span: Span,
}

#[derive(Diagnostics, Debug)]
#[message("Argument count mismatch: expected {expected}, found {found}")]
#[code = 1004]
#[label = "Argument count mismatch"]
#[note = "Check the number of arguments passed to the function"]
pub struct ArgumentCountMismatch {
    pub expected: usize,
    pub found: usize,
    #[span]
    pub span: Span,
}


#[derive(Diagnostics, Debug)]
#[message("Return statement not allowed outside of a function")]
#[code = 1005]
#[label = "Return not in function"]
#[note = "Ensure return statements are used within function bodies"]
pub struct ReturnNotInFunction {
    #[span]
    pub span: Span,
}

#[derive(Debug, Diagnostics)]
pub enum SemanticError {
    SymbolNotFound(SymbolNotFound),
    TypeMismatch(TypeMismatch),
    NotAFunction(NotAFunction),
    ArgumentCountMismatch(ArgumentCountMismatch),
    ReturnNotInFunction(ReturnNotInFunction),
}

impl SemanticError {
    pub fn symbol_not_found(name: String, span: Span) -> Self {
        SemanticError::SymbolNotFound(SymbolNotFound { name, span })
    }
    pub fn type_mismatch(expected: String, found: String, span: Span) -> Self {
        SemanticError::TypeMismatch(TypeMismatch {
            expected,
            found,
            span,
        })
    }
    
    pub fn not_a_function(name: String, type_: Type, span: Span) -> Self {
        SemanticError::NotAFunction(NotAFunction { name, type_, span })
    }
    
    pub fn argument_count_mismatch(expected: usize, found: usize, span: Span) -> Self {
        SemanticError::ArgumentCountMismatch(ArgumentCountMismatch {
            expected,
            found,
            span,
        })
    }
    
    pub fn return_not_in_function(span: Span) -> Self {
        SemanticError::ReturnNotInFunction(ReturnNotInFunction { span })
    }
}

#[macro_export]
macro_rules! semantic_error {
    (symbol ($symbol:expr) not found  in $span:expr) => {
        $crate::error::SemanticError::symbol_not_found(stringify!($symbol).to_string(), $span)
    };
    (symbol ($symbol:expr) not found) => {
        $crate::error::SemanticError::symbol_not_found($symbol.clone(), $symbol.span)
    };
    (type mismatch expected ($expected:tt) found ($found:tt) in $span:expr) => {
        $crate::error::SemanticError::type_mismatch(
            stringify!($expected).to_string(),
            stringify!($found).to_string(),
            $span,
        )
    };
}

pub type Result<T> = std::result::Result<T, SemanticError>;