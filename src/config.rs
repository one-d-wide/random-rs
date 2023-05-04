pub use {
    clap::{error::ErrorKind, Parser, ValueEnum},
    std::{ffi::OsString, path::PathBuf},
};

#[derive(Debug, Parser)]
#[clap(
    infer_subcommands = true,
    disable_help_subcommand = true,
    disable_version_flag = true,
    help_template = "usage: {bin} [<operation>] [options] <fields>\noperations:\n{subcommands}",
    subcommand_value_name = "operation"
)]
pub enum Config {
    #[clap(
        display_order = 0,
        short_flag = 'R',
        long_flag = "range",
        help_template = "usage: {usage}\n{options}"
    )]
    Range(super::range::Subcommand),
    #[clap(
        display_order = 1,
        short_flag = 'A',
        long_flag = "array",
        help_template = "usage: {usage}\n{options}"
    )]
    Array(super::array::Subcommand),
    #[clap(
        display_order = 2,
        short_flag = 'S',
        long_flag = "string",
        help_template = "usage: {usage}\n{options}"
    )]
    String(super::string::Subcommand),
    #[clap(
        display_order = 3,
        short_flag = 'M',
        long_flag = "matrix",
        help_template = "usage: {usage}\n{options}"
    )]
    Matrix(super::matrix::Subcommand),
    #[clap(
        display_order = 4,
        short_flag = 'C',
        long_flag = "custom-matrix",
        help_template = "usage: {usage}\n{options}"
    )]
    CustomMatrix(super::custom_matrix::Subcommand),
    #[clap(
        display_order = 5,
        short_flag = 'V',
        long_flag = "version",
        help_template = "usage: {usage}\n{options}"
    )]
    Version(super::version::Subcommand),
}

pub fn parse() -> Result<Option<Config>, String> {
    use std::env::args_os;
    let try_parse = |i: Box<dyn Iterator<Item = OsString>>| match Config::try_parse_from(i) {
        Ok(s) => Ok(Some(s)),
        Err(e) if e.to_string() == "" => Ok(None),
        Err(e) => Err(e),
    };

    let err = match try_parse(Box::new(args_os())) {
        Ok(ret) => return Ok(ret),
        Err(err) => err,
    };

    // Try substitute boilerplate
    if err.kind() == ErrorKind::InvalidSubcommand || err.kind() == ErrorKind::UnknownArgument {
        // Try `random-rs <min> <max>` -> `random-rs -R <min> <max>`
        let ret = try_parse(Box::new(
            args_os()
                .skip(1)
                .rev()
                .chain([OsString::from("-R")])
                .chain(args_os().take(1))
                .rev(),
        ));
        let err = match ret {
            Ok(ret) => return Ok(ret),
            Err(err) => err,
        };

        if err.kind() == ErrorKind::MissingRequiredArgument {
            // Try `random-rs <max>` -> `random-rs -R 0 <max>`
            let ret = try_parse(Box::new(
                args_os()
                    .skip(1)
                    .rev()
                    .chain([OsString::from("0")])
                    .chain([OsString::from("-R")])
                    .chain(args_os().take(1))
                    .rev(),
            ));
            if let Ok(ret) = ret {
                return Ok(ret);
            }
        }
    }

    Err(err.to_string())
}
