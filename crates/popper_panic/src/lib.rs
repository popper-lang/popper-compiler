


pub struct PanicInfo {
    message: String,
    location: Location,
}

pub struct Location {
    file: String,
    line: u32,
    column: u32,
}

impl PanicInfo {
    pub fn new(message: String, location: Location) -> PanicInfo {
        PanicInfo { message, location }
    }
}

pub fn panic_handler(info: PanicInfo) -> ! {
    eprintln!("panic: {}", info.message);
    eprintln!("  at {}:{}:{}", info.location.file, info.location.line, info.location.column);
    std::process::exit(1);
}

macro_rules! popper_panic {
    ($msg:expr) => {
        panic_handler(PanicInfo::new($msg.to_string(), Location::new(file!(), line!(), column!()));
    };
}
