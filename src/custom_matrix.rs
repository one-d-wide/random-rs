use super::common::*;

#[derive(Debug, Parser)]
pub struct Subcommand {
    #[clap(value_name = "min")]
    pub min: f64,
    #[clap(value_name = "max")]
    pub max: f64,
    #[clap(value_name = "columns")]
    pub columns: usize,
    #[clap(value_name = "rows")]
    pub rows: usize,

    #[clap(value_name = "begin")]
    pub begin: String,
    #[clap(value_name = "row_begin")]
    pub row_begin: String,
    #[clap(value_name = "delimiter")]
    pub delimiter: String,
    #[clap(value_name = "row_end")]
    pub row_end: String,
    #[clap(value_name = "end")]
    pub end: String,

    #[clap(
        short,
        long,
        help = "print all numbers with the same width",
        value_name = "pretty",
        default_value = "false"
    )]
    pub pretty: bool,
    #[clap(
        short = 't',
        long = "type",
        help = "set an matrix type",
        value_name = "type",
        default_value = "homogeneous"
    )]
    pub matrix_type: super::matrix::MatrixType,
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
        columns,
        rows,
        delimiter,
        row_begin,
        row_end,
        begin,
        end,
        pretty,
        matrix_type,
        float,
        output,
        seed,
    }: Subcommand,
) -> Result<()> {
    super::matrix::subcommand(super::matrix::Subcommand {
        min,
        max,
        columns,
        rows,
        delimiter,
        row_begin,
        row_end,
        begin,
        end,
        pretty,
        matrix_type,
        float,
        output,
        seed,
    })
}
