use std::convert::TryFrom;
use std::fs::{File, metadata, read_dir};
use std::path::Path;
use std::io::prelude::*;
use std::convert::TryInto;

use ssh2::Session;

use crate::error::{Error, BoxedResult};
use crate::unit::{Instance, DefinitionType};
use crate::fs_util;

use super::close_channel::close_channel;

pub fn transport(unit: &Instance, session: &Session) -> BoxedResult<()> {
    match unit.definition_rc.definition_type {
        DefinitionType::Executable => transport_executable_unit(unit, session),
        DefinitionType::Directory => transport_directory_unit(unit, session)
    }
}

pub fn get_remote_path(unit: &Instance) -> String {
    format!("/tmp/sysunit-{}", unit.definition_rc.name)
}

fn transport_executable_unit(unit: &Instance, session: &Session) -> BoxedResult<()> {
    let remote_path_str = get_remote_path(unit);
    let remote_path = Path::new(&remote_path_str);
    let local_path = Path::new(&unit.definition_rc.path);

    transport_file(local_path, remote_path, session)
}

fn transport_directory_unit(unit: &Instance, session: &Session) -> BoxedResult<()> {
    let remote_path_str = get_remote_path(unit);
    let remote_path = Path::new(&remote_path_str);
    let local_path = Path::new(&unit.definition_rc.path);

    transport_directory(local_path, remote_path, session)
}

fn create_remote_directory(remote_path: &Path, session: &Session) -> BoxedResult<()> {
    let command_string = format!("mkdir -p {}", path_to_str(remote_path)?);
    let mut channel = session.channel_session().map_err(|e|
        wrap_error!("Channel Initialization Error: {}", e)
    )?;

    channel.exec(command_string.as_str())?;

    let channel_result = close_channel(&mut channel)?;

    if channel_result.exit_status != 0 {
        return Err(Box::new(Error::new(format!(
            "Could not create directory {} via SSH. \n\
            Exit Code: {} \n\
            Command output: {} \n\
            Error output: {} ",
            path_to_str(remote_path)?,
            channel_result.exit_status,
            channel_result.stderr,
            channel_result.stdout
        ))))
    }

    Ok(())
}

fn path_to_str<'a>(path: &'a Path) -> Result<&'a str, Error> {
    match path.to_str() {
        Some(s) => Ok(s),
        None => Err(Error::new(format!(
            "Path can not be converted to utf-8 string: {}",
            path.to_string_lossy()
        )))
    }
}

fn transport_directory(local_path: &Path, remote_path: &Path, session: &Session) -> BoxedResult<()> {
    create_remote_directory(remote_path, session)?;

    for entry in read_dir(local_path)? {
        let entry = entry?;
        let path = entry.path();
        let local_file_name = match path.file_name() {
            Some(s) => s,
            None => return Err(Box::new(Error::new(format!(
                "Could not get valid filename from local path: {}",
                path_to_str(&path)?
            ))))
        };
        let remote_file_name = remote_path.join(local_file_name);
        
        if path.is_dir() {
            transport_directory(&path, &remote_file_name, session)?;
        } else if path.is_file() {
            transport_file(&path, &remote_file_name, session)?;
        }
    }

    Ok(())
}

fn transport_file(local_path: &Path, remote_path: &Path, session: &Session) -> BoxedResult<()> {
    let mut local_file = File::open(local_path)?;

    let mut local_buf = Vec::new();
    local_file.read_to_end(&mut local_buf)?;

    let local_permissions: i32 = fs_util::unix::get_local_permissions(&metadata(local_path)?)
        .try_into().unwrap();
    let clear_sticky_bit_mask: i32 = 0b0111111111111111;
    let modified_sticky_permissions: i32  = local_permissions & clear_sticky_bit_mask;

    let mut scp_channel = session.scp_send(Path::new(&remote_path),
        modified_sticky_permissions,
        u64::try_from(local_buf.len()).unwrap(),
        None
    )?;

    scp_channel.write(&local_buf)?;
    scp_channel.flush()?;

    let channel_result = close_channel(&mut scp_channel)?;

    if channel_result.exit_status > 1 {
        let error = Error::new(format!(
            "SCP Failed, exit status: {}, output: {}, error output: {}",
            channel_result.exit_status,
            channel_result.stdout,
            channel_result.stderr
        ));

        return Err(Box::new(error));
    }

    Ok(())
}
