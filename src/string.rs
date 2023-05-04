use super::common::*;

#[derive(Debug, Parser)]
pub struct Subcommand {
    #[clap(value_name = "letters")]
    pub letters: String,
    #[clap(value_name = "length")]
    pub length: Option<usize>,
    #[clap(short = 'r', long, help = "do not repeat elements", num_args = 0)]
    pub shuffle: bool,
    #[clap(
        long,
        help = "set an input delimiter sequence",
        value_name = "input_delimiter",
        default_value = ","
    )]
    pub input_delimiter: String,
    #[clap(
        short,
        long,
        help = "set an output delimiter sequence",
        value_name = "delimiter",
        default_value = ", "
    )]
    pub delimiter: String,
    #[clap(short, help = "do not print new line", num_args = 0)]
    pub no_newline: bool,
    #[clap(short, long, help = "take letters from file", value_name = "path")]
    pub input: Option<PathBuf>,
    #[clap(
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
        mut letters,
        length,
        shuffle,
        input_delimiter,
        delimiter,
        no_newline,
        input,
        output,
        seed,
    }: Subcommand,
) -> Result<()> {
    if let Some(input) = input {
        new_reader(input)?.read_to_string(&mut letters)?;
    }

    let mut letters: &mut [&str] = &mut letters.split(&input_delimiter).collect::<Vec<&str>>();
    if input_delimiter.is_empty() {
        let len = letters.len();
        letters = &mut letters[1..len - 1];
    };

    let length = length.unwrap_or(letters.len());

    let mut output = new_writer(output)?;
    let mut random = new_seed(seed)?;

    if shuffle {
        if length > letters.len() {
            return Err(Error::new(
                ErrorKind::Other,
                "Can't shuffle: length is greater then letters array",
            ));
        }
        let (slice, _) = letters.partial_shuffle(&mut random, length);
        for (i, letter) in slice.iter().enumerate() {
            if i != 0 {
                output.write_all(delimiter.as_bytes())?;
            }
            output.write_all(letter.as_bytes())?;
        }
    } else {
        for i in 0..length {
            if i != 0 {
                output.write_all(delimiter.as_bytes())?;
            }
            output.write_all(letters.choose(&mut random).unwrap().as_bytes())?;
        }
    }

    if !no_newline {
        output.write_all(b"\n")?;
    };

    Ok(())
}
