use std::sync::Mutex;
use lazy_static::lazy_static;
use popper_ast::file::{FileId, FileTable};

pub struct Context {
    file_table: FileTable,
}


impl Context {
    pub fn new() -> Context {
        Context {
            file_table: FileTable::new(),
        }
    }

    pub fn file_table(&self) -> &FileTable {
        &self.file_table
    }

    pub fn file_table_mut(&mut self) -> &mut FileTable {
        &mut self.file_table
    }

    pub fn file_table_into(self) -> FileTable {
        self.file_table
    }

    pub fn get_file(&self, id: FileId) -> Option<&popper_ast::file::File> {
        self.file_table.get(id)
    }
}

lazy_static! {
    pub static ref CONTEXT: Mutex<Context> = Mutex::new(Context::new());
}

pub fn with_context<F, R>(f: F) -> Option<R>
where
    F: FnOnce(&mut Context) -> R,
{
    let mut context = CONTEXT.lock().ok()?;
    Some(f(&mut context))
}

pub fn with_context_unwrap<F, R>(f: F) -> R
where
    F: FnOnce(&mut Context) -> R,
{
    let mut context = CONTEXT.lock().unwrap();
    f(&mut context)
}
