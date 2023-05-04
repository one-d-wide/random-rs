use std::{
    convert::From,
    error::Error as ErrorTrait,
    fmt::{Debug, Formatter, Result, Write},
    io::{Error, ErrorKind},
};

pub struct CustomError(String);

impl Debug for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.0)
    }
}

impl From<Error> for CustomError {
    fn from(err: Error) -> Self {
        let mut ret = Self(if err.kind() == ErrorKind::Other {
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
