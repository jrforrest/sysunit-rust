use crate::error::Error;
use crate::execution::Target;
use crate::ui::{Mode, report_execution};
use crate::resolver::{resolve, InstanceVec};
use crate::operation::Operation;
use crate::unit::{ApplicationState};

pub type RunResult = Result<(), Error>;

pub fn run(
    unit_name: &str, 
    operation_name: &str,
    args_str: &str,
    target_url: Option<&str>,
    adapter: Option<&str>,
    reporting_mode: Mode
) -> RunResult {
    let operation = Operation::from_str(operation_name)?;
    let mut target = Target::try_new(target_url, adapter)?;

    let ordered_unit_instances = resolve(&mut target, unit_name, args_str)?;

    let mut engine = Engine {
        ordered_instances: ordered_unit_instances,
        target: &mut target,
        operation: operation,
        reporting_mode: reporting_mode
    };

    engine.run()?;

    return Ok(())
}

struct Engine <'a> {
    reporting_mode: Mode,
    ordered_instances: InstanceVec,
    target: &'a mut Target,
    operation: Operation,
}

impl <'a> Engine <'a> {
    pub fn run(&mut self) -> RunResult {
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

    pub fn apply(&mut self) -> RunResult {
        for instance in self.ordered_instances.iter()
            .map(|rc| rc.borrow() )
            .filter(|i|
                match &i.application_state {
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

    pub fn rollback(&mut self) -> RunResult {
        for instance in self.ordered_instances.iter()
            .map(|rc| rc.borrow())
            .filter(|i|
                match i.application_state {
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

    pub fn check(&mut self, report: bool) -> RunResult {
        for instance in self.ordered_instances.iter() {
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
