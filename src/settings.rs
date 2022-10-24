use clap::{error::ErrorKind, Parser};
use std::{ffi::OsString, path::PathBuf};

#[derive(Debug, Parser)]
#[clap(infer_subcommands = true)]
#[clap(disable_help_subcommand = true, disable_version_flag = true)]
#[clap(help_template = "usage: {bin} [<operation>] [options] <fields>\noperations:\n{subcommands}")]
#[clap(subcommand_value_name = "operation")]
pub enum Settings {
    #[clap(display_order = 1)]
    #[clap(short_flag = 'R', long_flag = "range")]
    #[clap(help_template = "usage: {bin} {usage}\n{options}")]
    Range {
        #[clap(value_name = "start")]
        start: f64,
        #[clap(value_name = "end")]
        end: f64,
        #[clap(
            short,
            long,
            help = "set an output precision",
            value_name = "precision"
        )]
        float: Option<usize>,
        #[clap(short, help = "do not print new line", num_args = 0)]
        no_newline: bool,
        #[clap(
            short,
            long,
            help = "print output to file",
            value_name = "path",
            default_value = "-"
        )]
        output: PathBuf,
        #[clap(long, help = "get raw random from file", value_name = "path")]
        seed: Option<PathBuf>,
    },
    #[clap(display_order = 2)]
    #[clap(short_flag = 'A', long_flag = "array")]
    #[clap(help_template = "usage: {bin} {usage}\n{options}")]
    Array {
        #[clap(value_name = "start")]
        start: f64,
        #[clap(value_name = "end")]
        end: f64,
        #[clap(value_name = "length")]
        length: usize,
        #[clap(
            short,
            long,
            help = "set an output separator sequence",
            value_name = "separator",
            default_value = ", "
        )]
        separator: OsString,
        #[clap(
            short,
            long,
            help = "set an output precision",
            value_name = "precision"
        )]
        float: Option<usize>,
        #[clap(short, help = "do not print new line", num_args = 0)]
        no_newline: bool,
        #[clap(
            short,
            long,
            help = "print output to file",
            value_name = "path",
            default_value = "-"
        )]
        output: PathBuf,
        #[clap(long, help = "get raw random from file", value_name = "path")]
        seed: Option<PathBuf>,
    },
    #[clap(display_order = 3)]
    #[clap(short_flag = 'S', long_flag = "string")]
    #[clap(help_template = "usage: {bin} {usage}\n{options}")]
    String {
        #[clap(value_name = "letters")]
        letters: String,
        #[clap(value_name = "length")]
        length: Option<usize>,
        #[clap(short = 'r', long, help = "do not repeat elements", num_args = 0)]
        shuffle: bool,
        #[clap(
            short,
            long,
            help = "set an input divier sequence",
            value_name = "divier",
            default_value = ","
        )]
        divier: String,
        #[clap(
            short,
            long,
            help = "set an output separator sequence",
            value_name = "separator",
            default_value = ", "
        )]
        separator: OsString,
        #[clap(short, help = "do not print new line", num_args = 0)]
        no_newline: bool,
        #[clap(short, long, help = "take letters from file", value_name = "path")]
        input: Option<PathBuf>,
        #[clap(
            short,
            long,
            help = "print output to file",
            value_name = "path",
            default_value = "-"
        )]
        output: PathBuf,
        #[clap(long, help = "get raw random from file", value_name = "path")]
        seed: Option<PathBuf>,
    },
    #[clap(display_order = 4)]
    #[clap(short_flag = 'V', long_flag = "version")]
    #[clap(help_template = "usage: {bin} {usage}")]
    Version {},
}

pub fn parse() -> Result<Option<Settings>, String> {
    // Reduce code repetitions
    use std::env::args_os;
    let parse_from = |i: Box<dyn Iterator<Item = OsString>>| match Settings::try_parse_from(i) {
        Ok(s) => Ok(Some(s)),
        Err(e) if e.to_string() == "" => Ok(None),
        Err(e) => Err(e),
    };
    let mut ret = parse_from(Box::new(args_os()));

    // Select default operation if needed
    // This is little weird hack but i did not find another appropriate solution with derived parser
    if let Err(ref err) = ret {
        if err.kind() == ErrorKind::InvalidSubcommand || err.kind() == ErrorKind::UnknownArgument {
            // Assume operation is not explicitly selected
            ret = parse_from(Box::new(
                args_os()
                    .skip(1)
                    .rev()
                    .chain([OsString::from("-R")])
                    .chain(args_os().take(1))
                    .rev(),
            ));
            if let Err(ref err) = ret {
                if err.kind() == ErrorKind::MissingRequiredArgument {
                    // Assume one limit is not entred
                    ret = parse_from(Box::new(
                        args_os()
                            .skip(1)
                            .rev()
                            .chain([OsString::from("0")])
                            .chain([OsString::from("-R")])
                            .chain(args_os().take(1))
                            .rev(),
                    ));
                }
            }
        }
    }

    ret.map_err(|e| e.to_string())
}
