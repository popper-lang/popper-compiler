use crate::command::CommandEnum;
use crate::debug::{DebugSection, VarDebugKind};
use crate::marks::MarksSection;
use crate::program::{Program, ProgramSection};
use crate::stmt::{Statement, StmtKind};

#[derive(Debug, Clone)]
pub struct Pretty {
    tab_level: usize,
    pretty: String,
    program: Program
}

impl Pretty {
    pub fn new(program: Program) -> Self {
        Self {
            tab_level: 0,
            pretty: String::new(),
            program
        }
    }

    pub fn add_line(&mut self, line: &str) {
        self.pretty.push_str(&format!("{}{}\n", "   ".repeat(self.tab_level), line));
    }

    pub fn add_tab(&mut self) {
        self.tab_level += 1;
    }

    pub fn remove_tab(&mut self) {
        self.tab_level -= 1;
    }

    pub fn get_pretty(&self) -> &str {
        &self.pretty
    }

    pub fn pretty_program(&mut self) {
        for section in self.program.programs.clone().into_iter() {
            match section {
                ProgramSection::Function(ref function) => {
                    self.pretty_function(function);
                },
                ProgramSection::ExternalFunction(ref external_function) => {
                    self.add_line(&format!("extern func {}({}): {};", external_function.name, self.pretty_args(&external_function.args), external_function.ret));
                },
                ProgramSection::TypeDecl(id, ty) => {
                    self.add_line(&format!("type {} = {};", id, ty));
                },
            }
        }
    }

    pub fn pretty_function(&mut self, function: &crate::function::Function) {
        let mut function = function.clone();
        self.add_line(&format!("func {}({}) -> {}", function.name, self.pretty_args(&function.args), function.ret));
        self.add_line("{");
        self.add_tab();
        for stmt in function.stmts.clone() {
            self.pretty_stmt(&stmt);
        }
        self.add_line("");
        self.pretty_dbg_section(function.dbg.clone());
        self.add_line("");
        self.pretty_mark_section(function.marks.clone());
        self.remove_tab();
        self.add_line("}");
    }

    fn pretty_args(&self, args: &[crate::types::Types]) -> String {
        args.iter().map(|arg| arg.to_string()).collect::<Vec<String>>().join(", ")
    }

    fn pretty_dbg_section(&mut self, dbg: DebugSection) {
        self.add_line("dbg:");
        self.add_tab();
        for entry in dbg.get_all_debug_info() {
            match entry.kind.clone() {
                VarDebugKind::Var(name) => {
                    self.add_line(&format!("{}: var '{}'", entry.id, name));
                },
                VarDebugKind::Internal => {
                    self.add_line(&format!("{}: internal", entry.id));
                },
                VarDebugKind::Use(n) => {
                    self.add_line(&format!("{}: use {}", entry.id, n));
                }
            }
        }
        self.remove_tab();
    }

    fn pretty_mark_section(&mut self, marks_section: MarksSection) {
        self.add_line("marks:");
        self.add_tab();
        for mark in marks_section.get_all_marks() {
            self.add_line(&format!("mark {}, {}", mark.id, mark.kind));
        }
        self.remove_tab();
    }

    fn pretty_stmt(&mut self, stmt: &Statement) {
        match stmt.get_kind() {
            StmtKind::LetDecl(let_decl) => {
                self.add_line(&format!("let {} : {}", let_decl.ident, let_decl.ty));
            },
            StmtKind::Assign(assign) => {
                self.add_line(&format!("{} = {}", assign.ident, self.pretty_command(assign.command.clone())));
            },
            StmtKind::Command(cmd) => {
                self.add_line(&self.pretty_command(cmd.clone()));
            }
        }
    }

    fn pretty_command(&self, cmd: CommandEnum) -> String {
        match cmd {
            CommandEnum::Const(constant) => {
                format!("const {}", constant.kind)
            },
            CommandEnum::Ref(reference) => {
                format!("ref {}", reference.ident)
            },
            CommandEnum::Add(add) => {
                format!("add {}, {}", add.left, add.right)
            },
            CommandEnum::Sub(sub) => {
                format!("sub {}, {}", sub.left, sub.right)
            },
            CommandEnum::Mul(mul) => {
                format!("mul {}, {}", mul.left, mul.right)
            },
            CommandEnum::Div(div) => {
                format!("div {}, {}", div.left, div.right)
            },
            CommandEnum::CmpEq(cmp) => {
                format!("cmp_eq {}, {}", cmp.left, cmp.right)
            },
            CommandEnum::CmpNe(cmp) => {
                format!("cmp_ne {}, {}", cmp.left, cmp.right)
            },

            CommandEnum::CmpLt(cmp) => {
                format!("cmp_lt {}, {}", cmp.left, cmp.right)
            },

            CommandEnum::CmpGt(cmp) => {
                format!("cmp_gt {}, {}", cmp.left, cmp.right)
            },

            CommandEnum::CmpLe(cmp) => {
                format!("cmp_le {}, {}", cmp.left, cmp.right)
            },

            CommandEnum::CmpGe(cmp) => {
                format!("cmp_ge {}, {}", cmp.left, cmp.right)
            },

            CommandEnum::LLVMStore(store) => {
                format!("llvm_store {} as {}", store.ptr, store.as_type)
            },

            CommandEnum::LLVMLoadPtr(load) => {
                format!("llvm_load_ptr {}", load.ptr)
            },

            CommandEnum::Call(call) => {
                format!(
                    "{}({})",
                    call.function,
                    call
                        .args
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")

                )
            },

            CommandEnum::Br(br) => {
                format!("br {}, {}, {}", br.cond, br.true_branch, br.false_branch)
            },

            CommandEnum::Ret(ret) => {
                format!("ret {}", ret.value)
            },

            CommandEnum::CopyVal(copy_val) => {
                format!("copy_val {}", copy_val.val)
            },
            
            CommandEnum::GetElementPtrStruct(gep) => {
                format!("get_element_ptr {}, {}", gep.ptr, gep.index)
            },
            
            CommandEnum::Write(write) => {
                format!("write {}, {}", write.ptr, write.value)
            },


        }
    }

    pub fn print_to_string(&self) -> String {
        self.pretty.clone()
    }

    pub fn print_to_stdout(&self) {
        println!("{}", self.pretty.clone());
    }

    pub fn print_to_file(&self, file: &str) {
        std::fs::write(file, self.pretty.clone()).unwrap();
    }

    pub fn print_to_stderr(&self) {
        eprintln!("{}", self.pretty.clone());
    }

}
