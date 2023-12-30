

mod object;
pub mod compiler;


pub use inkwell::context::Context;



#[macro_export]
// a command macro
macro_rules! cmd {
    ($cmd:ident) => {
        Command::new(stringify!($cmd))
        .output()
        .expect("failed to execute process")
    };
    ($cmd:ident $($arg:expr)*) => {
        Command::new(stringify!($cmd))
            .args(&[$($arg),*])
            .output()
            .expect("failed to execute process")
    };
}
