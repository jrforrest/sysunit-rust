use unit::{Operation, Instance};
use host::Host;

pub struct Executor<'a> {
    instance: &'a Instance<'a>,
    host: &'a Host<'a>
}

impl<'a> Executor<'a> {
    pub fn new(host: &'a Host<'a>, instance: &'a Instance<'a>) -> Executor<'a> {
        Executor{instance: instance, host: host}
    }

    pub fn perform(&self, operation: Operation) -> Result<(), ()>{
        for dep in self.instance.iterate_dependencies() {
            let executor = Executor::new(self.host, dep);

            executor.perform(operation)?;
        }

        match operation {
            Operation::Check => self.host.check(&self.instance),
            Operation::Apply => self.host.apply(&self.instance),
        }
    }
}
