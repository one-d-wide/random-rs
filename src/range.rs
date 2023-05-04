use super::common::*;

#[derive(Debug, Parser)]
pub struct Subcommand {
    #[clap(value_name = "min")]
    pub min: f64,
    #[clap(value_name = "max")]
    pub max: f64,
    #[clap(
        short,
        long,
        help = "set an output precision",
        value_name = "precision"
    )]
    pub float: Option<usize>,
    #[clap(short, help = "do not print new line", num_args = 0)]
    pub no_newline: bool,
    #[clap(
        short,
        long,
        help = "print output to file",
        value_name = "path",
        default_value = "-"
    )]
    pub output: PathBuf,
    #[clap(long, help = "get raw random from file", value_name = "path")]
    pub seed: Option<PathBuf>,
}

pub fn subcommand(
    Subcommand {
        min,
        max,
        float,
        no_newline,
        output,
        seed,
    }: Subcommand,
) -> Result<()> {
    super::array::subcommand(super::array::Subcommand {
        min,
        max,
        float,
        no_newline,
        output,
        seed,
        pretty: false,
        length: 1,
        delimiter: "".to_string(),
    })
}
