use loader::Loader;
use unit::{Instance};

pub struct Instantiator<'a>  {
    loader: &'a Loader<'a>,
}

impl <'a> Instantiator<'a> {
    pub fn new(loader: &'a Loader) -> Instantiator<'a> {
        Instantiator{loader: loader}
    }

    pub fn instantiate(&self, name: &str) -> Instance {
        let definition = self.loader.find(name);

        Instance::new(definition)
    }
}
