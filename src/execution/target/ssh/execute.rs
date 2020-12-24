use crate::error::{Error, BoxedResult};
use crate::unit::{Instance, DefinitionType};
use crate::operation::Operation;
use crate::execution::Execution;

use log::debug;
use ssh2::Session;
use shell_escape::unix::escape;

use super::close_channel::close_channel;

pub fn execute(
    unit: &Instance,
    session: &Session,
    unit_path: String,
    operation: Operation
) -> BoxedResult<Execution> {
    let mut channel = session.channel_session().map_err(|e| {
        wrap_error!("Channel Initialization Error: {}", e)
    })?;

    use std::borrow::Cow;

    let arg_str = unit.id.args.vec.iter().map(|arg|
        format!("{}={}", arg.name, escape(Cow::from(&arg.value)))
    ).collect::<Vec<String>>().join(" ");

    let command_string = match unit.definition_rc.definition_type {
        DefinitionType::Executable => format!("{} {} {}",
            arg_str,
            unit_path,
            operation.to_str()),
        DefinitionType::Directory => format!("cd {}; {} ./unit {}",
            unit_path,
            arg_str,
            operation.to_str())
    };

    debug!("command string: {}", command_string);

    channel.exec(command_string.as_str()).map_err(|e|
        wrap_error!("Failed to execute unit: {}", e)
    )?;

    let channel_result = close_channel(&mut channel)?;

    let execution = Execution {
        unit_name: unit.definition_rc.name.clone(),
        exit_code: channel_result.exit_status,
        stdout: channel_result.stdout,
        stderr: channel_result.stderr
    };

    debug!("execution: {:?}", execution);

    Ok(execution)
}
