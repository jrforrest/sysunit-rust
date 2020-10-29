use std::process::exit;
use clap::{App, Arg};

mod error;
mod unit;
mod execution;
mod resolver;
mod engine;
mod ui;
mod fs_util;
mod operation;

use engine::run;
use ui::Mode;

fn main() {
    env_logger::init();

    let matches = App::new("Sysunit")
        .version("0.1")
        .arg(Arg::with_name("operation").required(true))
        .arg(Arg::with_name("unit").required(true))
        .arg(Arg::with_name("params").required(false))
        .arg(Arg::with_name("reporting-mode")
            .short("r")
            .long("reporting-mode")
            .value_name("MODE")
            .help("Sets format of reporting")
            .takes_value(true)
            .possible_values(&["min", "full"])
        )
        .arg(Arg::with_name("target")
            .short("t")
            .long("target")
            .value_name("TARGET_URL")
            .help("Specifies the target against which the unit operation will be executed")
            .takes_value(true)
            .required(false)
        )
        .arg(Arg::with_name("adapter")
            .short("a")
            .long("adapter")
            .value_name("ADAPTER_NAME")
            .help("specifies the adapter with which to execute")
            .takes_value(true)
            .required(false)
        )
        .get_matches();

    let unit_name = matches.value_of("unit").unwrap();
    let operation = matches.value_of("operation").unwrap();
    let arg_str = matches.value_of("params").unwrap_or("");
    let adapter_name = matches.value_of("adapter");
    let target_url_str = matches.value_of("target");

    let reporting_mode_value = 
        matches.value_of("reporting-mode").unwrap_or("min");

    let reporting_mode = match reporting_mode_value {
        "min" => Mode::Minimal,
        "full" => Mode::Full,
        _ => panic!("Impossible reporting-mode: {}", reporting_mode_value)
    };

    match run(unit_name, operation, arg_str, target_url_str, adapter_name, reporting_mode) {
        Ok(_) => exit(0),
        Err(e) => {
            println!("{}", e.msg);
            exit(1)
        },
    }
}
