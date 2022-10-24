mod array;
mod common;
mod error;
mod settings;
mod string;
mod version;

use settings::{parse, Settings};

fn main() -> Result<(), error::CustomError> {
    let settings = match parse() {
        Err(e) => {
            print!("{}", e);
            std::process::exit(1)
        }
        Ok(None) => return Ok(()),
        Ok(Some(s)) => s,
    };

    match settings {
        Settings::Range {
            start,
            end,
            float,
            no_newline,
            output,
            seed,
        } => {
            array::array(Settings::Array {
                start,
                end,
                float,
                no_newline,
                output,
                seed,
                length: 1,
                separator: std::ffi::OsString::new(),
            })?;
        }
        Settings::Array { .. } => array::array(settings)?,
        Settings::String { .. } => string::string(settings)?,
        Settings::Version { .. } => version::version(),
    }

    Ok(())
}
