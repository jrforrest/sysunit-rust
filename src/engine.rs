use crate::error::Error;
use crate::execution::Target;
use crate::reporting::{Mode, report_execution};
use crate::resolver::Resolver;
use crate::operation::Operation;
use crate::unit::{ApplicationState};

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

    let engine = Engine {
        resolver: &resolver,
        target: &target,
        operation: operation,
        reporting_mode: reporting_mode
    };

    engine.run()?;

    return Ok(())
}

struct Engine <'a> {
    reporting_mode: Mode,
    resolver: &'a Resolver<'a>,
    target: &'a Target,
    operation: Operation,
}

impl <'a> Engine <'a> {
    pub fn run(&self) -> RunResult {
        match self.operation {
            Operation::Apply => {
                self.check(false)?;
                self.apply()
            },
            Operation::Rollback => {
                self.check(false)?;
                self.rollback()
            },
            Operation::Check => {
                self.check(true)
            },
            other => Err(Error::new(format!(
                "{} is not a supported top-level operation", other.to_str()
            )))
        }
    }

    pub fn apply(&self) -> RunResult {
        for instance in self.resolver.ordered_instances.iter()
            .map(|rc| rc.borrow() )
            .filter(|i|
                match &i.application_state {
                    Some(ApplicationState::Applied) => true,
                    _ => false
                }
            )
        {
            let run_result = self.target.execute(&instance, Operation::Apply)?;

            report_execution(&run_result, self.reporting_mode, self.operation);
        }

        Ok(())
    }

    pub fn rollback(&self) -> RunResult {
        for instance in self.resolver.ordered_instances.iter()
            .map(|rc| rc.borrow())
            .filter(|i|
                match i.application_state {
                    Some(ApplicationState::NotApplied(_)) => true,
                    _ => false
                }
            )
        {
            let run_result = self.target.execute(&instance, Operation::Apply)?;

            report_execution(&run_result, self.reporting_mode, self.operation);
        }

        Ok(())
    }

    pub fn check(&self, report: bool) -> RunResult {
        for instance in self.resolver.ordered_instances.iter() {
            let run_result = self.target.execute(&instance.borrow(), Operation::Check)?;
            let output_str = run_result.stdout.trim_end();

            let mut instance_mut = instance.borrow_mut();

            if output_str == "ok" {
                instance_mut.application_state = Some(ApplicationState::Applied);
            } else {
                instance_mut.application_state =
                    Some(ApplicationState::NotApplied(output_str.to_string()));
            }

            if report { report_execution(&run_result, self.reporting_mode, self.operation) }
        }

        Ok(())
    }
}
