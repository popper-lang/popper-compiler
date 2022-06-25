


#[macro_export]
macro_rules! import_expr {
    ( $($ex:ident -> $ast:ident), * )  => {
        $(
            use crate::expr::$ex::$ast;
        )*

        #[derive(Clone)]
        pub enum Expr {
            $(
                $ast($ast),
            )*
        }


    }
}
