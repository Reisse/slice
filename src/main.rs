use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process;

mod slice;
use slice::*;

macro_rules! fail {
    ( $($x:expr),* ) => {{
        eprintln!($($x,)*);
        process::exit(1);
    }}
}

fn help() -> ! {
    println!(
r#"Usage: slice [OPTION]... [FILE]
Print slice from FILE to standard output.
When slice is not specified, print whole file to standard output.

    -s, --slice BEGIN:END       specify slice to print
    -h, --help                  display this help and exit
    -v, --version               output version information and exit
    --                          end of options

BEGIN and END may be any combination of positive (denoting position
from the beginning) or negative (denoting position from the end) numbers.

Both LF and CRLF are recognized as newline characters.
Newlines are not preserved and are always replaced with LF in output.
Last line of the output will always end with LF."#
    );
    process::exit(0);
}

fn version() -> ! {
    println!("{} {}",
        option_env!("CARGO_PKG_NAME").unwrap_or("whatever"),
        option_env!("CARGO_PKG_VERSION").unwrap_or("unknown"));
    if let Some(authors) = option_env!("CARGO_PKG_AUTHORS") {
        println!("Written by {}", authors);
    }
    process::exit(0);
}

fn main() {
    let mut slice = Slice{ begin: 0, end: None };
    let mut filename: Option<String> = None;

    let mut named_args = true;
    let mut args_iter = env::args().into_iter().skip(1); // skip executable name
    loop {
        match args_iter.next() {
            Some(arg) => {
                if named_args && arg == "--" {
                    named_args = false;
                } else if named_args && (arg == "-h" || arg == "--help") {
                    help();
                } else if named_args && (arg == "-v" || arg == "--version") {
                    version();
                } else if named_args && (arg == "-s" || arg == "--slice") {
                    match args_iter.next() {
                        Some(next_arg) => match Slice::from_string(&next_arg) {
                            Ok(parsed_slice) => slice = parsed_slice,
                            Err(error) => fail!("Failed to parse slice \"{}\": {}", next_arg, error)
                        },
                        None => fail!("\"{}\" option provided without argument.", arg)
                    }
                } else if let None = filename {
                    filename = Some(arg);
                } else {
                    fail!("Unexpected argument \"{}\".", arg);
                }
            },
            None => break
        }
    }

    let stdin = io::stdin();
    let mut input: Box<dyn BufRead>;

    if let Some(filename) = filename {
        match File::open(&filename) {
            Ok(file) => input = Box::new(BufReader::new(file)),
            Err(error) => fail!("Failed to open file \"{}\": {}", filename, error)
        }
    } else {
        input = Box::new(stdin.lock());
    }

    if let Err(error) = slice_input(slice, &mut input, &mut io::stdout().lock()) {
        fail!("Failed to perform slice: {}", error);
    }
}