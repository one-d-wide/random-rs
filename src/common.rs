use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Result},
    path::PathBuf,
};

use rand::{rngs::StdRng, SeedableRng};

const HINT_BUFLEN: usize = 65536;

pub fn init_i(input: PathBuf, hint_length: usize) -> Result<BufReader<File>> {
    Ok(BufReader::with_capacity(
        if hint_length < HINT_BUFLEN {
            hint_length
        } else {
            HINT_BUFLEN
        },
        std::fs::OpenOptions::new()
            .read(true)
            .write(false)
            .append(false)
            .create(false)
            .open(if input == PathBuf::from("-") {
                PathBuf::from("/dev/stdin")
            } else {
                input
            })?,
    ))
}

pub fn init_o(output: PathBuf, hint_length: usize) -> Result<BufWriter<File>> {
    let hint_length = hint_length * 8;
    Ok(BufWriter::with_capacity(
        if hint_length < HINT_BUFLEN {
            hint_length
        } else {
            HINT_BUFLEN
        },
        std::fs::OpenOptions::new()
            .read(false)
            .write(true)
            .append(true)
            .create(true)
            .open(if output == PathBuf::from("-") {
                PathBuf::from("/dev/stdout")
            } else {
                output
            })?,
    ))
}

pub fn init_seed(input: Option<PathBuf>) -> Result<StdRng> {
    Ok(match input {
        None => StdRng::from_entropy(),
        Some(input) => StdRng::from_seed({
            let mut r = [0u8; 32];
            self::init_i(input, 32)?.read_exact(&mut r)?;
            r
        }),
    })
}
