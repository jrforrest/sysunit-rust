#[macro_use]
extern crate serde_derive;
extern crate glob;
extern crate serde_yaml;

mod unit;
mod loader;
mod adapter;
mod executor;
mod host;

use unit::{Instance, Operation};
use loader::Loader;
use host::Host;
use executor::Executor;

fn main() {
    let mut loader = Loader::new();
    loader.load("./units");

    let definition = loader.find("yup");
    let instance = definition.get_instance();

    let host = Host::new();
    let executor = Executor::new(&host, &instance);

    executor.perform(Operation::Apply).expect("failed to apply");
}
