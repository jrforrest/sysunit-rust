use crate::error::Error;
use crate::execution::execute;
use crate::reporting::report_execution;
use crate::resolver::Resolver;

pub type RunResult = Result<(), Error>;

pub fn run(unit_name: &str, operation: &str, args_str: &str) -> RunResult {
    let mut resolver = Resolver::new();
    resolver.resolve(unit_name, args_str)?;

    println!("{:?}", resolver.ordered_instances);

    for instance_rc in resolver.ordered_instances.iter() {
        let instance = &instance_rc.borrow();

        let run_result = execute(instance, operation)?;
        report_execution(&run_result);
    }

    return Ok(())
}
