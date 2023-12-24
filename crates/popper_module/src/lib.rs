use popper_ast::PathImport;
use std::path::{PathBuf, Path};
use popper_ast::{Function, Type, TypeKind};
use popper_error::{Error, modulenotfound::ModuleNotFound};

pub static STD_LIB_PATH: &str = "./";

pub trait ModuleLoader {
    fn load(&self, path: PathImport) -> Result<String, Box<dyn Error>>;
    fn sign_fn(&self, path: PathImport) -> Vec<Function>;
}

pub fn stmt_path_to_path(stmt_path: PathImport) -> PathBuf {
    let mut path = PathBuf::new();
    for seg in stmt_path.segments {
        path.push(seg.name);
    }
    path
}

pub struct StdModuleLoader;

impl ModuleLoader for StdModuleLoader {
    fn load(&self, import_path: PathImport) -> Result<String, Box<dyn Error>> {
        let path = stmt_path_to_path(import_path.clone());
        let mut json_path = Path::new(STD_LIB_PATH).join(path.clone());
        json_path = json_path.join(
            import_path.segments.last().unwrap().name.clone()
        );
        json_path.set_extension("json");
        let functions: Vec<Function> = serde_json::from_str(&std::fs::read_to_string(json_path).unwrap()).unwrap();
        let mut string = String::new();

        for func in functions {
            let mut llvm_path = Path::new(STD_LIB_PATH).join(path.clone()).join(func.name.clone());
            llvm_path.set_extension("ll");

            let llvm = std::fs::read_to_string(llvm_path).unwrap();
            string.push_str(&llvm);

        }

        Ok(string)
    }

    fn sign_fn(&self, path_import: PathImport) -> Vec<Function> {
        let path = stmt_path_to_path(path_import.clone());
        let mut path = Path::new(STD_LIB_PATH).join(path);
        path = path.join(
            path_import.segments.last().unwrap().name.clone()
        );
        path.set_extension("json");
        dbg!(&path);
        let content = std::fs::read_to_string(path).unwrap();

        let json: Vec<Function> = serde_json::from_str(&content).unwrap();

        json

    }
}


pub fn load_module<T: ModuleLoader>(loader: T, path: PathImport) -> Result<String, Box<dyn Error>> {
    loader.load(path)
}







