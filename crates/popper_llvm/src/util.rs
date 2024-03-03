#[macro_export]
macro_rules! c_str {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const i8
    };

    ($s:expr) => {
        $s.as_ptr() as *const i8
    };
}
