use crate::settings::Settings;

use std::{
    io::{Error, ErrorKind, Read, Write},
    os::unix::ffi::OsStrExt,
};

use rand::seq::SliceRandom;

pub fn string(settings: Settings) -> Result<(), Error> {
    if let Settings::String {
        mut letters,
        length,
        divier,
        separator,
        no_newline,
        input,
        output,
        seed,
        shuffle,
    } = settings
    {
        if let Some(input) = input {
            super::common::init_i(input, 2048)?.read_to_string(&mut letters)?;
        }

        let mut letters: Vec<&str> = if divier.is_empty() {
            // std::str::split method counts begin and end as empty slices
            letters
                .as_str()
                .split_terminator(divier.as_str())
                .skip(1)
                .collect()
        } else {
            letters.as_str().split(divier.as_str()).collect()
        };

        let length = match length {
            Some(l) => l,
            None => letters.len(),
        };

        let mut output = super::common::init_o(output, length)?;
        let mut random = super::common::init_seed(seed)?;

        let separator = separator.as_os_str().as_bytes();

        if shuffle {
            if length > letters.len() {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "Can not shuffle: length is greater then letters array",
                ));
            }
            let (slice, _) = letters.partial_shuffle(&mut random, length);
            for (i, e) in slice.iter().enumerate() {
                if i != 0 {
                    output.write_all(separator)?;
                }
                output.write_all(e.as_bytes())?;
            }
        } else {
            for i in 0..length {
                if i != 0 {
                    output.write_all(separator)?;
                }
                output.write_all(letters.choose(&mut random).unwrap().as_bytes())?;
            }
        }
        if !no_newline {
            output.write_all("\n".as_bytes())?;
        };
        Ok(())
    } else {
        unreachable!();
    }
}
