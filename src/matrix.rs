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
    #[clap(
        short,
        long,
        help = "set an column delimiter sequence",
        value_name = "column_delimiter",
        default_value = ", "
    )]
    pub delimiter: String,
    #[clap(
        long,
        help = "set an row begin sequence",
        value_name = "row_begin",
        default_value = ""
    )]
    pub row_begin: String,
    #[clap(
        long,
        help = "set an end sequence",
        value_name = "end",
        default_value = ";\n"
    )]
    pub row_end: String,
    #[clap(
        long,
        help = "set an row end sequence",
        value_name = "row_end",
        default_value = "[\n"
    )]
    pub begin: String,
    #[clap(
        long,
        help = "set an begin sequence",
        value_name = "begin",
        default_value = "]\n"
    )]
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
    pub matrix_type: MatrixType,
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

#[derive(Clone, Debug, Parser, ValueEnum)]
pub enum MatrixType {
    Homogeneous,
    Diagonal,
    UpperTriangular,
    LowerTriangular,
    Symmetric,
}
use MatrixType::*;

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
    let range = new_uniform(min, max)?;
    let float = float.unwrap_or(0);

    let max_number_width = max_number_width_if(pretty, min, max, float);

    let mut symmetric_buffer: Vec<Vec<f64>> = Vec::new();
    if let Symmetric = &matrix_type {
        if rows != columns {
            return Err(Error::new(
                ErrorKind::Other,
                "Symmetric matrix must be square",
            ));
        }
        symmetric_buffer = (0..rows)
            .map(|_| (0..columns).map(|_| 0.0).collect())
            .collect();
    };

    let mut output = new_writer(output)?;
    let mut random = new_seed(seed)?;

    output.write_all(begin.as_bytes())?;
    for row in 0..rows {
        output.write_all(row_begin.as_bytes())?;
        for column in 0..columns {
            if column != 0 {
                output.write_all(delimiter.as_bytes())?;
            }
            let random = match &matrix_type {
                Diagonal if column != row => 0.0,
                UpperTriangular if column < row => 0.0,
                LowerTriangular if column > row => 0.0,
                Symmetric if column < row => symmetric_buffer[column][row],
                _ => range.sample(&mut random),
            };
            if let (Symmetric, true) = (&matrix_type, column > row) {
                symmetric_buffer[row][column] = random;
            }
            write!(output, "{random:>max_number_width$.float$}")?;
        }
        output.write_all(row_end.as_bytes())?;
    }
    output.write_all(end.as_bytes())?;

    Ok(())
}
