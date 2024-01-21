use popper_llvm::module::Module;
use popper_llvm::context::Context;
use popper_llvm::builder::Builder;

use popper_mir::mir_ast::{Declare, Module as MirModule, Type};
use popper_mir::mir_ast::Ir;

use popper_common::hash::hash_file;
use popper_llvm::types::TypeEnum;

macro_rules! cmd {
    ($cmd:ident $($e:expr)* ) => {
        std::process::Command::new(stringify!($cmd))
            $(.arg($e))*
            .output()
            .expect(concat!("Failed to execute ", stringify!($cmd)))
    };
}

#[derive(Debug, Clone, Copy)]
pub struct Compiler {
    pub context: Context,
    pub module: Module,
    pub builder: Builder,
    pub mir_module: MirModule,
    pub cdylib_used: Vec<String>
}

impl Compiler {
    pub fn new(mir_module: MirModule) -> Self {
        let context = Context::new();
        let module = Module::new(mir_module.name.as_str(), context);
        let builder = Builder::new(context);
        Self {
            context,
            module,
            builder,
            mir_module,
            cdylib_used: Vec::new()
        }
    }

    pub fn mir_ty_to_llvm_ty(&mut self, mir_ty: Type) -> TypeEnum {
        match mir_ty {
            Type::Int => {
                let ty = self.context.i64_type();
                TypeEnum::IntType(ty)
            },
            Type::String(len) => {
                let ty = self.context.i8_type().array(len as u64);]
                TypeEnum::ArrayType(ty)
            },
            Type::Float => {
                let ty = self.context.float_type();
                TypeEnum::FloatType(ty)
            },
            Type::Bool => {
                let ty = self.context.i1_type();
                TypeEnum::IntType(ty)
            },
            Type::Void => {
                let ty = self.context.void_type();
                TypeEnum::IntType(ty)
            },
            _ => unimplemented!()
        }
    }

    pub fn compile_cdylib(&mut self, path: String) {
        let path = std::path::Path::new(path.as_str());

        if ! path.exists() {
            panic!("File not found: {}", path.display());
        }
        let filename = path
            .with_extension("")
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        let hash = hash_file(path.display().to_string().as_str());
        let new_name = format!("librs{}_{}.dylib", hash, filename);
        let target_path = std::env::var("POPPER_PATH").unwrap();
        let target_path = std::path::Path::new(target_path.as_str());
        let lib_path = target_path.join("libs");

        if ! lib_path.exists() {
            std::fs::create_dir(lib_path.clone()).unwrap();
        }

        let file_path = lib_path.join(new_name);

        let output = cmd!(rustc "--crate-type=cdylib" path.display() "-o" file_path.display());

        if ! output.status.success() {
            panic!("Failed to compile cdylib: {}", output.status);
        }

        self.cdylib_used.push(file_path.display().to_string());
    }

    pub fn compile(&mut self) {
        for ir in self.mir_module.ir {
            self.compile_ir(ir);
        }
    }

    pub fn compile_ir(&mut self, ir: Ir) {
        match ir {
            Ir::LoadExternal(name) => {
                self.load_external(name.string);
            },
            Ir::LoadModule(module) => {
                self.load_module(module);
            },
            Ir::Declare(declare) => {
                self.declare(declare);
            },
        }
    }

    pub fn load_external(&mut self, path: String) {
        if ! path.exists() {
            panic!("File not found: {}", path.display());
        }

        self.compile_cdylib(path);
    }

    pub fn load_module(&mut self, mir_module: MirModule) {
        let mut compiler = Compiler::new(mir_module);
        compiler.compile();
        self.module.link(&compiler.module);
    }

    pub fn declare(&mut self, declare: Declare) {
        let mut args = Vec::new();
        for arg in declare.args.list {
            args.push(
                self.mir_ty_to_llvm_ty(arg)
            );
        }

        let func = self.mir_ty_to_llvm_ty(declare.ret).func(args, false);
        self.module.add_function(declare.name.as_str(), func);
    }

    pub fn compile_function(&mut self, func: popper_mir::mir_ast::Function) {
        let mut args = Vec::new();
        for arg in func.args.args {
            args.push(
                self.mir_ty_to_llvm_ty(arg.ty)
            );
        }

        let function = self.mir_ty_to_llvm_ty(func.ret).func(args, false);
        let fn_val = self.module.add_function(func.name.as_str(), func);
        let basic_block = self.context.append_basic_block("entry", fn_val);
        self.builder.position_at_end(basic_block);

        for stmt in func.body.body {
            self.compile_stmt(stmt);
        }
    }


}