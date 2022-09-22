use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type OpsResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank: bool,
}

pub fn run(config: Config) -> OpsResult<()> {
    for filename in config.files {
        println!("{}", filename);
    }
    Ok(())
}

pub fn get_args() -> OpsResult<Config> {
    let matches = App::new("chat")
        .version("0.1.0")
        .author("Paul Vidal <u1d4lp@gmail.com>")
        .about("A simple cat clone built in Rust")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("number_lines")
                .short("n")
                .long("number-lines")
                .help("Number lines")
                .takes_value(false)
                .conflicts_with("number_nonblank"),
        )
        .arg(
            Arg::with_name("number_nonblank")
                .short("b")
                .long("number-nonblank")
                .help("Number nonblank lines")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number_lines"),
        number_nonblank: matches.is_present("number_nonblank"),
    })
}

pub fn open(filename: &str) -> OpsResult<Box<dyn BufRead>> {
    if filename == "-" {
        Ok(Box::new(BufReader::new(io::stdin())))
    } else {
        Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}