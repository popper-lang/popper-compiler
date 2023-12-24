mod core;

use std::collections::HashMap;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use serde::{Deserialize, Serialize};

use std::process::Command;

pub type LLVMSourceCode = String;

pub trait PopInternalModule {
    fn manage(&mut self, module: Module, builder: Builder, context: Context);
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct PopLibraryInfo {
    pub name: String,
    pub author: String,
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum PopModuleType {
    Internal,
    External,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum PopLangageType {
    LLVM,
    Rust,
    Popper
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct PopLibrary {
    pub info: PopLibraryInfo,
    pub module_type: PopModuleType,
    pub langage_type: PopLangageType,
    pub submodules: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct PopModule {
    pub name: String,
    pub langage_type: PopLangageType,
    pub env: HashMap<String, String>
}


pub struct PopLLVMModule {
    pub filename: String,
}

impl PopLLVMModule {
    pub fn new(filename: String) -> PopLLVMModule {
        PopLLVMModule {
            filename,
        }
    }
}

impl PopInternalModule for PopLLVMModule {
    fn manage(&mut self, module: Module, builder: Builder, context: Context) -> LLVMSourceCode {
        return std::fs::read_to_string(self.filename.as_str()).unwrap();
    }
}

pub struct PopRustModule {
    pub filename: String,
}

impl PopRustModule {
    pub fn new(file_name: String) -> PopRustModule {
        PopRustModule {
            filename: file_name,
        }
    }
}


impl PopInternalModule for PopRustModule {
    fn manage(&mut self, module: Module, builder: Builder, context: Context) -> LLVMSourceCode {
        return unsafe {
            String::from_utf8_unchecked(
                Command::new("rustc")
                                            .arg("-O")
                                            .arg("--emit=llvm-ir")
                                            .arg(self.filename.as_str())
                                            .output()
                    .unwrap()
                    .stdout
            )
        }
    }
}

pub fn install_module(path: String) -> HashMap<String, LLVMSourceCode> {
    let path = std::path::Path::new(path.as_str());
    let libjson_path = path.join("lib.json");

    let libjson = std::fs::read_to_string(libjson_path).unwrap();

    let module: PopLibrary = serde_json::from_str(libjson.as_str()).unwrap();
    let submodules = module.submodules;

    let mut env = HashMap::new();

    for submodule in submodules {
        let path = libjson_path + "/" + submodule.as_str();
        let path = std::path::Path::new(path.as_str());
        let module_path = path.join("module.json");
        let module_json = std::fs::read_to_string(module_path).unwrap();
        let module: PopModule = serde_json::from_str(module_json.as_str()).unwrap();

        for (name, file) in module.env {
            let content = std::fs::read_to_string(path.join(file.as_str())).unwrap();
            env.insert(name, content);
        }
    }

    return env;

}