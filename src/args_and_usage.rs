use clap::{App, Arg};
use std::path::PathBuf;

// Programmer defined constants
static PROGRAM_NAME: &'static str = "just";

// Derived constants
static VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Debug)]
pub struct Args {
    pub input_option: Option<PathBuf>,
    pub output_option: Option<PathBuf>,
    pub not_pretty: bool,
    pub indent_spaces: u16,
}

pub fn parse_args() -> Args {
    let args = App::new(PROGRAM_NAME)
        .version(VERSION)
        .author("Russell W. Bentley <russell.w.bentley@icloud.com>")
        .about("A tool for verifying and formatting json")
        .arg(
            Arg::with_name("INPUT")
                .help("The file to use as input, defaults to STDIN")
                .long("input")
                .value_name("input/file.json")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("OUTPUT")
                .help("The file to use as output, defaults to STDOUT")
                .long("output")
                .value_name("output/fie.ex")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("NOT_PRETTY")
                .help("Defualt is to pretty-print json, don't have to")
                .long("not-pretty"),
        )
        .arg(
            Arg::with_name("INDENT_SPACES")
                .help(
                    "If pretty printing, how many spaces for each level of indentations",
                )
                .long("indent-spaces")
                .value_name("n")
                .default_value("4")
                .takes_value(true),
        )
        .get_matches();

    let input_path_option = args.value_of("INPUT").map(PathBuf::from);
    let output_path_option = args.value_of("OUTPUT").map(PathBuf::from);
    let indent_spaces = str::parse::<u16>(args.value_of("INDENT_SPACES").unwrap())
        .expect("Must pass number to --indent-spaces");

    Args {
        input_option: input_path_option,
        output_option: output_path_option,
        not_pretty: args.is_present("NOT_PRETTY"),
        indent_spaces: indent_spaces,
    }
}
