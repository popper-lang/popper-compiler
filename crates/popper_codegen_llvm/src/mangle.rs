pub fn mangle_static_method(ctx: &str, parent: &str, s: &str) -> String {
    format!("{}:{}::{}", ctx, parent, s)
}

pub fn mangle_method(ctx: &str, parent: &str, s: &str) -> String {
    format!("{}:{}.{}", ctx, parent, s)
}
