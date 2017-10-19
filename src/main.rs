extern crate clap;
#[macro_use]
extern crate error_chain;
extern crate json;

mod args_and_usage;

use args_and_usage::parse_args;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, stdin, stdout};

quick_main!(|| -> Result<()> {
    let args = parse_args();
    let stdin_handle = stdin();
    let stdout_handle = stdout();
    // We need a reader for the input file


    // Setup a buffered input reader
    // Use an input file if provided, othere wise use stdin
    let mut input_reader: Box<BufRead> = if args.input_option.is_some() {
        let input_path = args.input_option.unwrap();
        let input_file = File::open(&input_path).chain_err(|| {
            format!("Can't open input file: {}", input_path.display())
        })?;
        Box::new(BufReader::new(input_file))
    } else {
        Box::new(BufReader::new(stdin_handle.lock()))
    };

    // Setup a buffered output reader
    // Use output file if providd, otherwise use stdout
    let mut output_writer: Box<Write> = if args.output_option.is_some() {
        let output_path = args.output_option.unwrap();
        let output_file = File::create(&output_path).chain_err(|| {
            format!("Cant create output file: {}", output_path.display())
        })?;
        Box::new(BufWriter::new(output_file))
    } else {
        Box::new(stdout_handle.lock())
    };

    // Read all the input into a buffer
    let mut input_buffer = String::new();
    input_reader.read_to_string(&mut input_buffer).chain_err(
        || "Couldn't read input contents",
    )?;

    // Parse the json object
    let json_object = json::parse(&input_buffer)?;

    // Stringify the json
    let output = if args.not_pretty {
        json::stringify(json_object)
    } else {
        json::stringify_pretty(json_object, args.indent_spaces)
    };

    // Write to output
    output_writer.write_all(output.as_bytes())?;

    Ok(())
});

error_chain! {
    foreign_links {
        IO(std::io::Error);
        Json(json::JsonError);
    }
}
