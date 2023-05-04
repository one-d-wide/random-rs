use super::common::*;

#[derive(Debug, Parser)]
pub struct Subcommand {
    #[clap(value_name = "min")]
    pub min: f64,
    #[clap(value_name = "max")]
    pub max: f64,
    #[clap(value_name = "length")]
    pub length: usize,
    #[clap(
        short,
        long,
        help = "set an output delimiter sequence",
        value_name = "delimiter",
        default_value = ", "
    )]
    pub delimiter: String,
    #[clap(
        short,
        long,
        help = "set an output precision",
        value_name = "precision"
    )]
    pub float: Option<usize>,
    #[clap(
        short,
        long,
        help = "print all numbers with the same width",
        value_name = "pretty",
        default_value = "false"
    )]
    pub pretty: bool,
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
        length,
        delimiter,
        float,
        pretty,
        no_newline,
        output,
        seed,
    }: Subcommand,
) -> Result<()> {
    let range = new_uniform(min, max)?;
    let float = float.unwrap_or(0);

    let max_number_width = max_number_width_if(pretty, min, max, float);

    let mut output = new_writer(output)?;
    let mut random = new_seed(seed)?;

    for column in 0..length {
        if column != 0 {
            output.write_all(delimiter.as_bytes())?;
        }
        let random = range.sample(&mut random);
        write!(output, "{random:>max_number_width$.float$}")?;
    }
    if !no_newline {
        output.write_all(b"\n")?;
    };

    Ok(())
}
