use std::io::prelude::*;

use ssh2::Channel;
use log::debug;

use crate::error::Error;

#[derive(Debug)]
pub struct ChannelResult {
    pub stdout: String,
    pub stderr: String,
    pub exit_status: i32,
}

pub fn close_channel(channel: &mut Channel) -> Result<ChannelResult, Error> {
    channel.send_eof().map_err(|e|
        wrap_error!("Failed to send EOF on channel: {}", e)
    )?;

    channel.close().map_err(|e|
        wrap_error!("Failed to send close on channel: {}", e)
    )?;

    let mut output = String::new();
    let mut stderr = String::new();

    loop {
        channel.wait_eof().map_err(|e| {
            debug!("Wait EOF on Channel error: {:?}", e);
            Error::new(format!("Failed to wait for EOF on channel: {}", e))
        })?;

        channel.read_to_string(&mut output).map_err(|e| {
            debug!("Command output read error: {:?}", e);
            Error::new(format!("Failed to read unit output: {}", e))
        })?;

        channel.stderr().read_to_string(&mut stderr).map_err(|e| {
            debug!("Command stderr read error: {:?}", e);
            Error::new(format!("Failed to read unit stderr: {}", e))
        })?;

        if channel.eof() { break };
    }

    let exit_code = channel.exit_status()
        .map_err(|e| Error::new(format!(
            "Failed to get exit code: {}", e
        )))?;

    debug!("eof: {:?}", channel.wait_eof());

    let channel_result = ChannelResult {
        stdout: output,
        stderr: stderr,
        exit_status: exit_code
    };

    Ok(channel_result)
}
