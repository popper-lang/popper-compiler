use crate::mir_ast::{Module, Ir, MirCompile, List, Type, Argument, Body, BodyFn, Value};

#[derive(Debug, Clone)]
pub struct Pretty {
    pub indent: usize,
    pub newline: bool,
    pub result: String,
    pub tab_size: usize,
}

impl Pretty {
    pub fn new(tab_size: usize) -> Self {
        Self {
            indent: 0,
            newline: true,
            result: String::new(),
            tab_size,
        }
    }

    pub fn indent(&mut self) {
        self.indent += 1;
    }

    pub fn unindent(&mut self) {
        self.indent -= 1;
    }

    pub fn newline(&mut self) {
        self.newline = true;
    }

    pub fn tab(&mut self) {
        self.result.push_str(&format!("{}", " ".repeat(self.indent * self.tab_size)));
    }

    pub fn pretty_module(&mut self, module: &Module) {
        self.result.push_str(&format!("module {} {{\n", module.name));
        self.indent();
        for ir in &module.ir {
            self.pretty_ir(ir);
        }
        self.unindent();
        self.result.push_str("}\n");
    }

    pub fn pretty_ir(&mut self, ir: &Ir) {
        self.tab();
        match ir {
            Ir::LoadModule(mir_string) => {
                self.result.push_str(&"load_module ".to_string());
                self.pretty_module(mir_string);
                self.result.push_str("\n");
            },
            Ir::LoadExternal(mir_string) => {
                self.result.push_str(&format!("load_external {}\n", mir_string));
            },
            Ir::Declare(declare) => {
                self.result.push_str(&format!("declare {} = args", declare.name));
                self.pretty_list(declare.args.clone());
                self.result.push_str(" ret");
                self.pretty_type(declare.ret.clone());
                self.result.push_str("\n");
            },
            Ir::Function(func) => {
                self.result.push_str("func ");
                self.pretty_type(func.ret.clone());
                self.result.push_str(&format!(" {}", func.name));
                self.pretty_args(func.args.clone().args);
                self.result.push_str(" {\n");
                self.indent();
                self.pretty_body_fn(&func.body);
                self.unindent();
                self.result.push_str("   }\n");

            }
        }

    }

    pub fn pretty_list<T: MirCompile + PartialEq>(&mut self, list: List<T>) {
        self.result.push_str(
            &format!(
                "[{}]",
                list.list
                    .iter()
                    .map(|x| x.compile())
                    .collect::<Vec<String>>()
                    .join(", ")
            )
        );
    }

    pub fn pretty_type(&mut self, t: Type) {
        self.result.push_str(t.compile().as_str());
    }

    pub fn pretty_args(&mut self, args: Vec<Argument >) {
        self.result.push_str(
            &format!(
                "({})",
                args
                    .iter()
                    .map(|x| x.compile())
                    .collect::<Vec<String>>()
                    .join(", ")
            )
        );
    }

    pub fn pretty_body_fn(&mut self, body: &Body) {
        for stmt in &body.body {
            self.pretty_stmt(stmt.clone());
        }
    }

    pub fn pretty_stmt(&mut self, body_fn: BodyFn) {
        self.tab();
        match body_fn {
            BodyFn::Add(add) => {
                self.result.push_str(&format!("add {}, ", add.name));
                self.pretty_value(add.lhs);
                self.result.push_str(", ");
                self.pretty_value(add.rhs);
                self.result.push_str("\n");
            },
            BodyFn::Alloc(alloc) => {
                self.result.push_str(&format!("alloc {}, ", alloc.name));
                self.pretty_type(alloc.ty);
                self.result.push_str("\n");
            },
            BodyFn::Store(store) => {
                self.result.push_str(&format!("store {}, ", store.name));
                self.pretty_value(store.value);
                self.result.push_str("\n");
            },
            BodyFn::Call(call) => {
                self.result.push_str(&format!("call {}, ", call.name));
                self.pretty_list(call.args);
                self.result.push_str(&format!(", {} \n", call.ret));
            },
            BodyFn::Return(ret) => {
                self.result.push_str("ret ");
                self.pretty_value(ret.value);
                self.result.push_str("\n");
            },
            BodyFn::Index(index) => {
                self.result.push_str(&format!("index {}, ", index.res));
                self.pretty_value(index.list);
                self.result.push_str(", ");
                self.pretty_value(index.index);
                self.result.push_str("\n");
            }
        }
    }

    pub fn pretty_value(&mut self, value: Value) {
        match value {
            Value::Const(constant) => {
                self.result.push_str(&format!("{}", constant.compile()));
            },
            Value::Variable(variable) => {
                self.result.push_str(&format!("{}", variable.compile()));
            },
        }
    }


}
