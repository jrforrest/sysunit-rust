use crate::execution::Execution;

pub fn report_execution(execution: &Execution) {
    println!("[{}|{}] {}",
        execution.unit_name,
        execution.exit_code,
        execution.stdout)
}
