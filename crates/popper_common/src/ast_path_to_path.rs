

/// Converts a path from the AST to a path that can be used by the compiler.
pub fn ast_path_to_path(path: popper_ast::PathImport) -> std::path::PathBuf {
    let segments = path.segments;
    let mut path = std::path::PathBuf::new();
    for segment in segments {
        path.push(segment.name);
    }
    path.set_extension("pop");
    path
}