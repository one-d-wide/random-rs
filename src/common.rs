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

pub fn new_reader(path: PathBuf) -> Result<BufReader<Box<dyn Read>>> {
    if path.as_path() == Path::new("-") {
        return Ok(BufReader::new(Box::new(std::io::stdin())));
    }
    let file = std::fs::OpenOptions::new()
        .read(true)
        .write(false)
        .append(false)
        .create(false)
        .open(path)?;
    Ok(BufReader::new(Box::new(file)))
}

pub fn new_writer(path: PathBuf) -> Result<BufWriter<Box<dyn Write>>> {
    if path.as_path() == Path::new("-") {
        return Ok(BufWriter::new(Box::new(std::io::stdout())));
    }
    let file = std::fs::OpenOptions::new()
        .read(false)
        .write(true)
        .append(true)
        .create(true)
        .open(path)?;
    Ok(BufWriter::new(Box::new(file)))
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
