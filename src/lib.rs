use std::error::Error;
use clap::{App, Arg} 

type OpsResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank: bool,
}

pub fn get_args() -> OpsResult<Config> {
    let matches = App::new("chat")
        .version("0.1.0")
        .author("Paul Vidal <u1d4lp@gmail.com>")
        .about("A simple cat clone built in Rust")
        //
        .get_matches();

        Ok(Config {
            files: ...,
            number_lines: ...,
            number_nonblank: ...,
        })
}