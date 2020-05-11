use crate::error::Error;
use crate::execution::Target;
use crate::reporting::{Mode, report_execution};
use crate::resolver::Resolver;
use crate::operation::Operation;

pub type RunResult = Result<(), Error>;

pub fn run(
    unit_name: &str, 
    operation_name: &str,
    args_str: &str,
    adapter: &str,
    reporting_mode: Mode
) -> RunResult {
    let operation = Operation::from_str(operation_name)?;
    let target = Target::try_new(adapter)?;
    let mut resolver = Resolver::new(&target);
    resolver.resolve(unit_name, args_str)?;

    for instance_rc in resolver.ordered_instances.iter() {
        let instance = &instance_rc.borrow();

        let run_result = target.execute(instance, operation)?;
        report_execution(&run_result, reporting_mode);
    }

    return Ok(())
}
