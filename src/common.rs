pub use {
    super::config::*,
    rand::{
        distributions::{uniform::SampleUniform, Distribution, Uniform},
        rngs::StdRng,
        seq::SliceRandom,
        SeedableRng,
    },
    std::{
        ffi::OsStr,
        fs::File,
        io::{BufReader, BufWriter, Error, ErrorKind, Read, Result, Write},
        path::{Path, PathBuf},
    },
};

pub fn some_or_default<T: PartialEq<&'static Path>, D: Into<T>>(path: T, default: D) -> T {
    if path == Path::new("-") {
        default.into()
    } else {
        path
    }
}

pub fn new_reader(input: PathBuf) -> Result<BufReader<File>> {
    let path = some_or_default(input, "/dev/stdin");
    let file = std::fs::OpenOptions::new()
        .read(true)
        .write(false)
        .append(false)
        .create(false)
        .open(path)?;
    Ok(BufReader::new(file))
}

pub fn new_writer(output: PathBuf) -> Result<BufWriter<File>> {
    let path = some_or_default(output, "/dev/stdout");
    let file = std::fs::OpenOptions::new()
        .read(false)
        .write(true)
        .append(true)
        .create(true)
        .open(path)?;
    Ok(BufWriter::new(file))
}

pub fn new_seed(input: Option<PathBuf>) -> Result<StdRng> {
    let random = match input {
        None => StdRng::from_entropy(),
        Some(input) => StdRng::from_seed({
            let mut r = [0u8; 32];
            new_reader(input)?.read_exact(&mut r)?;
            r
        }),
    };
    Ok(random)
}

pub fn max_number_width_if(condition: bool, min: f64, max: f64, float: usize) -> usize {
    if condition {
        let len = |n| format!("{n:.float$}").len();
        usize::max(len(max), len(min))
    } else {
        0
    }
}

pub fn new_uniform(min: f64, max: f64) -> Result<Uniform<f64>> {
    if min == max {
        return Err(Error::new(ErrorKind::Other, "Given range is empty"));
    }
    Ok(Uniform::from(f64::min(min, max)..=f64::max(min, max)))
}
