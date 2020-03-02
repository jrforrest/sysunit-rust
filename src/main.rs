use std::process::exit;
use clap::{App, Arg};

extern crate crypto;

mod error;
mod unit;
mod loader;
mod execution;
mod resolver;
mod engine;
mod reporting;
mod instance_cache;

use engine::run;

fn main() {
    let matches = App::new("Sysunit")
        .version("0.1")
        .arg(Arg::with_name("operation").required(true))
        .arg(Arg::with_name("unit").required(true))
        .arg(Arg::with_name("params").required(false))
        .get_matches();

    let unit_name = matches.value_of("unit").unwrap();
    let operation = matches.value_of("operation").unwrap();
    let arg_str = matches.value_of("params").unwrap_or("");

    match run(unit_name, operation, arg_str) {
        Ok(_) => exit(0),
        Err(e) => {
            println!("{}", e.msg);
            exit(1)
        },
    }
}
