#[macro_export]
macro_rules! import_expr {
    ( $($ex:ident -> $ast:ident), * )  => {
        $(
            use crate::expr::$ex::$ast;
        )*

        #[derive(Clone, Debug)]
        pub enum Expr {
            $(
                $ast($ast),
            )*
            Empty
        }

        impl Evaluateur for Expr {
            fn eval(&self, vm: &mut Vm) -> Result<Value, Error> {
                match self {
                    Expr::Empty => Ok(Value::None),
                    $(
                        Expr::$ast(expr) => expr.eval(vm),
                    )*
                }
            }
        }


    }
}
