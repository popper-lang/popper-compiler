pub mod object;
pub mod compiler;

#[macro_export]
// a command macro
macro_rules! cmd {
    ($cmd:ident) => {
        std::process::Command::new(stringify!($cmd))
        .output()
        .expect("failed to execute process")
    };
    ($cmd:ident $($arg:expr)*) => {
        std::process::Command::new(stringify!($cmd))
            .args(&[$($arg),*])
            .output()
            .expect("failed to execute process")
    };
}
