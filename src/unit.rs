use std::rc::Rc;

use crypto::sha1::Sha1;
use crypto::digest::Digest;

use crate::error::Error;

#[derive(Debug)]
pub struct Definition {
    pub name: String,
    pub path: String,
    pub definition_type: DefinitionType,
}

impl Definition {
    pub fn new(name: &str, path: &str, definition_type: DefinitionType) -> Definition {
        Definition {
            name: name.to_string(),
            path: path.to_string(),
            definition_type: definition_type,
        }
    }
}

#[derive(Debug)]
pub enum DefinitionType {
    Directory,
    Executable
}

pub type DefinitionRc = Rc<Definition>;

#[derive(Clone, Debug)]
pub struct Argument {
    pub name: String,
    pub value: String
}

#[derive(Clone, Debug)]
pub struct ArgSet {
    pub vec: Vec<Argument>,
}

impl ArgSet {
    pub fn new() -> ArgSet {
        ArgSet { vec: Vec::new() }
    }

    pub fn parse(args_str: &str) -> Result<ArgSet, Error> {
        let mut vec: Vec<Argument> = Vec::new();

        for pair in args_str.split(",") {
            if pair.trim().is_empty() { continue }

            let parts = pair.split("=").collect::<Vec<&str>>();
            if parts.len() != 2 {
                return Err(Error::new(format!(
                    "Args parse error: {} should be in form of key=value",
                    pair
                )))
            }

            let argument = Argument {
                name: parts[0].to_string(),
                value: parts[1].to_string()
            };

            vec.push(argument)
        }

        return Ok(ArgSet { vec: vec })
    }

    pub fn sha1(&self) -> String {
        let mut hasher = Sha1::new();

        for arg in self.vec.iter() {
            hasher.input_str(&arg.name);
            hasher.input_str(&arg.value);
        }

        hasher.result_str().to_string()
    }
}

#[derive(Debug)]
pub struct Instance {
    pub definition_rc: DefinitionRc,
    pub run_state: RunState,
    pub id: InstanceId
}

#[derive(Debug, Clone)]
pub enum RunState {
    Init,
    Resolving,
    Resolved
}

impl Instance {
    pub fn new(definition_rc: DefinitionRc, instance_id: InstanceId) -> Instance {
        return Instance {
            definition_rc: definition_rc,
            run_state: RunState::Init,
            id: instance_id
        }
    }
}

#[derive(Debug)]
pub struct InstanceId {
    pub name: String,
    pub args: ArgSet
}

impl InstanceId {
    pub fn build(name: &str, args_str: &str) -> Result<InstanceId, Error> {
        let args = ArgSet::parse(args_str)?;
        Ok(InstanceId { name: name.to_string(), args: args })
    }

    pub fn new(name: String, args: ArgSet) -> InstanceId {
        InstanceId { name: name, args: args }
    }

    pub fn signature(&self) -> String {
        format!("{}-{}", self.name, self.args.sha1()).to_string()
    }
}
