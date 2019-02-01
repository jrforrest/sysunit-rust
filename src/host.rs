use adapter::Adapter;
use unit::{Operation, Instance};

pub struct Host<'a> {
    adapter: Adapter<'a>,
}

impl<'a> Host<'a> {
    pub fn new() -> Host<'a> {
        let adapter = Adapter::new("local", "/bin/sh");
        Host{adapter: adapter}
    }

    pub fn check(&self, instance: &Instance) -> Result<(), ()> {
       self.adapter.run(&instance, Operation::Check)
    }

    pub fn apply(&self, instance: &Instance) -> Result<(), ()> {
        match self.adapter.run(&instance, Operation::Check) {
            Err(()) => {
                self.adapter.run(&instance, Operation::Apply)
            }
            Ok(()) => Ok(()),
        }
    }
}
