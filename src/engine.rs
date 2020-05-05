use crate::error::Error;
use crate::execution::execute;
use crate::reporting::{Mode, report_execution};
use crate::resolver::Resolver;

pub type RunResult = Result<(), Error>;

pub fn run(
    unit_name: &str, 
    operation: &str,
    args_str: &str,
    reporting_mode: Mode
) -> RunResult {
    let mut resolver = Resolver::new();
    resolver.resolve(unit_name, args_str)?;

    for instance_rc in resolver.ordered_instances.iter() {
        let instance = &instance_rc.borrow();

        let run_result = execute(instance, operation)?;
        report_execution(&run_result, reporting_mode);
    }

    return Ok(())
}
