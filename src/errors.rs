use std::fmt::Display;
use std::fs;
use std::io::{Read, Write};
use std::ops::Range;

static LOG_MODE: bool = false;
static LOG_PATH: &str = "/Users/antoine/Documents/popper-lang/popper.txt";
#[derive(Debug)]
pub enum ErrorType {
    TypeError,
    SyntaxError,
    NameError,
    AttributeError,
    FileNotFoundError,
}

impl Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

pub struct Error {
    error_type: ErrorType,
    msg: String,
    ext: Range<usize>,
    body: String,
}

impl Error {
    pub fn new(error_type: ErrorType, msg: &str, ext: Range<usize>, body: String) -> Self {
        Error {
            error_type,
            msg: msg.to_string(),
            ext,
            body,
        }
    }

    pub fn panic(&self) {
        if LOG_MODE {
            self.save_into_log(&LOG_PATH.to_string());
        }
        panic!("{}", self.fmt_error())
    }

    pub fn fmt_error(&self) -> String {
        format!(
            "({}) {} \n |{}:{}| {}",
            self.error_type,
            self.msg,
            self.ext.start,
            self.ext.end,
            &self.body
        )
    }

    pub fn save_into_log(&self, path: &String) {
        let mut file = fs::OpenOptions::new()
            .append(true)
            .read(true)
            .open(path)
            .expect("Error when open the file");

        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        file.write(format!("{}\n{}", content, self.fmt_error()).as_bytes())
            .expect("unable to write file");
    }
}

#[macro_export]
macro_rules! error {
    ($t:expr, $msg:expr, $ext:expr, $body:expr) => {
        Error::new($t, $msg, $ext, $body).panic()
    };
    ($t:expr, $msg:expr, $x:expr, $ext:expr, $body:expr) => {
        Error::new($t, format!($msg, $x), $body).panic()
    };
    ($t:expr, $msg:expr, $ext:expr, $body:expr, $($x:expr), *) => {
        Error::new($t, format!($msg, $( $x )*), $ext, $body).panic()
    };
    ($t:expr, $msg:expr, $ext:expr, $body:expr) => {
        Error::new($t, $msg, $ext, $body).panic()
    };
    ($t:expr, $msg:expr, $x:expr ,$ext:expr, $body:expr) => {
        Error::new($t,  format!($msg, $x), $ext, $body).panic()
    };

}

pub(crate) use error;
