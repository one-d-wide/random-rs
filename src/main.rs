mod common;
mod config;
mod error;

mod array;
mod custom_matrix;
mod matrix;
mod range;
mod string;
mod version;

fn main() -> Result<(), error::CustomError> {
    let config = match self::config::parse() {
        Err(err) => {
            print!("{}", err);
            std::process::exit(1)
        }
        Ok(None) => return Ok(()),
        Ok(Some(config)) => config,
    };

    use self::config::Config::*;
    let result = match config {
        Range(config) => self::range::subcommand(config),
        Array(config) => self::array::subcommand(config),
        String(config) => self::string::subcommand(config),
        Matrix(config) => self::matrix::subcommand(config),
        CustomMatrix(config) => self::custom_matrix::subcommand(config),
        Version(config) => self::version::subcommand(config),
    };
    Ok(result?)
}
