

#[macro_export]
macro_rules! to_slice_u8  {
    ($s:expr, $e:literal) => {
        TryInto::<[u8; $e]>::try_into($s.as_bytes()).unwrap()
    };
}


#[macro_export]
macro_rules! to_string {
    ($s:expr) => {
        std::str::from_utf8($s).unwrap()
    };
}