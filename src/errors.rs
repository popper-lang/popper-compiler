use crate::value::Type;

pub trait DisplayError {
    fn display_error(&self) -> String;
}

#[derive(Debug, PartialEq, Clone)]
pub struct VarNotFoundError {
    pub var_name: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct VarAlreadyDefinedError {
    pub var_name: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TypeMismatchError {
    pub expected: Type,
    pub found: Type,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CannotAddError {
    pub left: String,
    pub right: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CannotSubError {
    pub left: String,
    pub right: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CannotMulError {
    pub left: String,
    pub right: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CannotDivError {
    pub left: String,
    pub right: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CannotModError {
    pub left: String,
    pub right: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CannotCompareError {
    pub left: String,
    pub right: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IsBuiltinError {
    pub name: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionNotFoundError {
    pub name: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IndexOutOfBoundsError {
    pub name: String,
    pub index: i32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionArgumentMismatchError {
    pub name: String,
    pub expected: usize,
    pub got: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AttrNotFoundError {
    pub attr_name: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FileNotFoundError {
    pub file_name: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct StructNotFoundError {
    pub name: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct EnumNotFoundError {
    pub name: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FieldEnumNotFoundError {
    pub name: String,
    pub field: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct InvalidCastNumberError {
    pub elt: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ItsAConstantError {
    pub var_name: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct  CannotPowError {
    pub left: String,
    pub right: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CannotUnaryOpError {
    pub operand: String
}

#[derive(Debug, PartialEq, Clone)]
pub struct MethodAlreadyExistsError {
    pub name: String,
}

impl DisplayError for VarNotFoundError {
    fn display_error(&self) -> String {
        format!("Variable {} not found", self.var_name)
    }
}

impl DisplayError for VarAlreadyDefinedError {
    fn display_error(&self) -> String {
        format!("Variable {} already defined", self.var_name)
    }
}

impl DisplayError for TypeMismatchError {
    fn display_error(&self) -> String {
        format!(
            "Type mismatch: expected {:?}, found {:?}",
            self.expected, self.found
        )
    }
}

impl DisplayError for CannotAddError {
    fn display_error(&self) -> String {
        format!("Cannot add {} and {}", self.left, self.right)
    }
}

impl DisplayError for CannotSubError {
    fn display_error(&self) -> String {
        format!("Cannot subtract {} from {}", self.left, self.right)
    }
}

impl DisplayError for CannotMulError {
    fn display_error(&self) -> String {
        format!("Cannot multiply {} and {}", self.left, self.right)
    }
}

impl DisplayError for CannotDivError {
    fn display_error(&self) -> String {
        format!("Cannot divide {} by {}", self.left, self.right)
    }
}

impl DisplayError for CannotModError {
    fn display_error(&self) -> String {
        format!("Cannot mod {} by {}", self.left, self.right)
    }
}

impl DisplayError for CannotCompareError {
    fn display_error(&self) -> String {
        format!("Cannot compare {} and {}", self.left, self.right)
    }
}

impl DisplayError for IsBuiltinError {
    fn display_error(&self) -> String {
        format!("Cannot create a builtin function {}", self.name)
    }
}

impl DisplayError for FunctionNotFoundError {
    fn display_error(&self) -> String {
        format!("Function {} not found", self.name)
    }
}

impl DisplayError for IndexOutOfBoundsError {
    fn display_error(&self) -> String {
        format!("Index {} out of bounds for {}", self.index, self.name)
    }
}

impl DisplayError for StructNotFoundError {
    fn display_error(&self) -> String {
        format!("Struct {} not found", self.name)
    }
}

impl DisplayError for AttrNotFoundError {
    fn display_error(&self) -> String {
        format!("Attribute {} not found", self.attr_name)
    }
}

impl DisplayError for FunctionArgumentMismatchError {
    fn display_error(&self) -> String {
        format!(
            "Function {} expected {} arguments, found {}",
            self.name, self.expected, self.got
        )
    }
}

impl DisplayError for FileNotFoundError {
    fn display_error(&self) -> String {
        format!("File {} not found", self.file_name)
    }
}

impl DisplayError for EnumNotFoundError {
    fn display_error(&self) -> String {
        format!("Enum {} not found", self.name)
    }
}

impl DisplayError for FieldEnumNotFoundError {
    fn display_error(&self) -> String {
        format!("Field {} not found in enum {}", self.field, self.name)
    }
}

impl DisplayError for InvalidCastNumberError {
    fn display_error(&self) -> String {
        format!("invalid number: {}", self.elt)
    }
}

impl DisplayError for ItsAConstantError {
    fn display_error(&self) -> String {
        format!("its a constant: {}", self.var_name)
    }
}

impl DisplayError for CannotPowError {
    fn display_error(&self) -> String {
        format!("Cannot pow '{}' and '{}'", self.left, self.right)
    }
}

impl DisplayError for MethodAlreadyExistsError {
    fn display_error(&self) -> String {
        format!("Method '{}' already exists", self.name)
    }
}

impl DisplayError for CannotUnaryOpError {
    fn display_error(&self) -> String {
        format!("cannot unary op: {}", self.operand)
    }
}



#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    VarNotFound(VarNotFoundError),
    VarAlreadyDefined(VarAlreadyDefinedError),
    TypeMismatch(TypeMismatchError),
    CannotAdd(CannotAddError),
    CannotSub(CannotSubError),
    CannotMul(CannotMulError),
    CannotDiv(CannotDivError),
    CannotMod(CannotModError),
    CannotCompare(CannotCompareError),
    FunctionNotFound(FunctionNotFoundError),
    IndexOutOfBounds(IndexOutOfBoundsError),
    StructNotFound(StructNotFoundError),
    AttrNotFound(AttrNotFoundError),
    EnumNotFound(EnumNotFoundError),
    FieldEnumNotFound(FieldEnumNotFoundError),
    InvalidCastNumber(InvalidCastNumberError),
    ItsAConstant(ItsAConstantError),
    SyntaxError(String),
    CannotPow(CannotPowError),
    TooManyArgumentsForFunction(FunctionArgumentMismatchError),
    NotEnoughArgumentsForFunction(FunctionArgumentMismatchError),
    MethodAlreadyExists(MethodAlreadyExistsError),
    CannotUnaryOp(CannotUnaryOpError),
}

impl DisplayError for Error {
    fn display_error(&self) -> String {
        match self {
            Error::VarNotFound(err) => err.display_error(),
            Error::VarAlreadyDefined(err) => err.display_error(),
            Error::TypeMismatch(err) => err.display_error(),
            Error::CannotAdd(err) => err.display_error(),
            Error::CannotSub(err) => err.display_error(),
            Error::CannotMul(err) => err.display_error(),
            Error::CannotDiv(err) => err.display_error(),
            Error::CannotMod(err) => err.display_error(),
            Error::CannotCompare(err) => err.display_error(),
            Error::FunctionNotFound(err) => err.display_error(),
            Error::IndexOutOfBounds(err) => err.display_error(),
            Error::StructNotFound(err) => err.display_error(),
            Error::AttrNotFound(err) => err.display_error(),
            Error::EnumNotFound(err) => err.display_error(),
            Error::FieldEnumNotFound(err) => err.display_error(),
            Error::InvalidCastNumber(err) => err.display_error(),
            Error::ItsAConstant(err) => err.display_error(),
            Error::CannotPow(err) => err.display_error(),
            Error::TooManyArgumentsForFunction(err) => err.display_error(),
            Error::NotEnoughArgumentsForFunction(err) => err.display_error(),
            Error::MethodAlreadyExists(err) => err.display_error(),
            Error::SyntaxError(err) => err.to_string(),
            Error::CannotUnaryOp(err) => err.display_error()
        }
    }
}

#[macro_export]
macro_rules! error {
    ($msg:expr, $line:expr, $pos:expr) => {
        panic!("[line {}, pos {}] {}", $line, $pos, $msg)
    };
    ($msg:expr, $line:expr, $pos:expr, $x:expr) => {
        panic!("[line {}, pos {}] {}", $line, $pos, format!($msg, $x))
    };
    ($msg:expr, $line:expr, $pos:expr, $($x:expr), *) => {
        panic!("[line {}, pos {}] {}", $line, $pos, format!($msg, $( $x )*))
    };
    ($msg:expr) => {
        panic!($msg)
    };
    ($msg:expr, $x:expr ) => {
        panic!($msg, $x)
    };
    ($msg:expr, $($x:expr), * ) => {
        panic!($msg, format!($msg, $( $x )*))
    };

}


pub(crate) use error;