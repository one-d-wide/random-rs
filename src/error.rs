use std::{
    convert::From,
    error::Error,
    fmt::{Debug, Formatter, Result, Write},
};

pub struct CustomError(String);

impl Debug for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.0)
    }
}

impl From<std::io::Error> for CustomError {
    fn from(err: std::io::Error) -> Self {
        let mut ret = Self(if err.kind() == std::io::ErrorKind::InvalidInput {
            err.to_string()
        } else {
            format!("({}): {}", err.kind(), err)
        });
        if let Some(src) = err.source() {
            ret.0.write_fmt(format_args!("; Source: {}", src)).unwrap();
        }
        ret
    }
}

// // Uniform error handling
// impl<T: std::error::Error> From<T> for CustomError {
//     fn from(err: T) -> Self {
//         Self(format!("{}", err.to_string()))
//     }
// }
