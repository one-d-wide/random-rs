use crate::settings::Settings;

use std::{
    io::{Error, ErrorKind, Write},
    os::unix::ffi::OsStrExt,
};

use rand::distributions::{Distribution, Uniform};

pub fn array(settings: Settings) -> Result<(), Error> {
    if let Settings::Array {
        start,
        end,
        length,
        separator,
        float,
        no_newline,
        output,
        seed,
    } = settings
    {
        let mut output = super::common::init_o(output, length)?;

        if end == start {
            return Err(Error::new(ErrorKind::InvalidInput, "Given range is empty"));
        }
        let range = Uniform::from(if start < end { start..end } else { end..start });
        let float = if let Some(n) = float { n } else { 0 };
        let separator = separator.as_os_str().as_bytes();

        let mut random = super::common::init_seed(seed)?;

        for i in 0..length {
            if i != 0 {
                output.write_all(separator)?;
            }
            write!(output, "{:.*}", float, range.sample(&mut random))?;
        }
        if !no_newline {
            output.write_all("\n".as_bytes())?;
        };

        Ok(())
    } else {
        unreachable!();
    }
}
