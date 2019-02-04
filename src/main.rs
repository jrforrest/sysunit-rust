#[macro_use]
extern crate serde_derive;
extern crate glob;
extern crate serde_yaml;

mod unit;
mod loader;
mod adapter;
mod executor;
mod host;
mod instantiator;

use unit::Operation;
use loader::Loader;
use host::Host;
use executor::Executor;
use instantiator::Instantiator;

fn main() {
    let mut loader = Loader::new();
    loader.load("./units");

    let mut instantiator = Instantiator::new(&loader);
    let instance = instantiator.instantiate("yup");

    let host = Host::new();
    let executor = Executor::new(&host, &instance);

    executor.perform(Operation::Apply).expect("failed to apply");
}
