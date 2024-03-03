use popper_ast::*;
use popper_mir::mir_ast::{
    Body, Declare, Function as MirFunction, Ir, List, MirCompile, Module, Type as MirType,
};
use popper_mir::mir_compiler::MirCompiler;

fn main() {
    let ast = vec![Statement::Function(Function::new(
        "main".to_string(),
        Arguments::new(
            vec![Argument::new(
                "a".to_string(),
                Type::new(Default::default(), TypeKind::Int, Default::default()),
                Default::default(),
            )],
            Default::default(),
        ),
        Type::new(Default::default(), TypeKind::Int, Default::default()),
        vec![Statement::Return(Return::new(
            Some(Expression::Constant(Constant::Ident(Ident::new(
                Default::default(),
                "a".to_string(),
            )))),
            Default::default(),
        ))],
        Default::default(),
    ))];

    let mut compiler = MirCompiler::new(ast, "test".to_string());

    let ir = compiler.compile();

    println!("{}", ir.compile());
}
