use std::rc::Rc;
use std::cell::RefCell;

use crate::unit::{Instance, RunState, ArgSet, InstanceId};
use crate::execution::execute;
use crate::error::Error;

mod instance_cache;
mod loader;

use self::instance_cache::InstanceCache;

pub struct Resolver {
    pub ordered_instances: Vec<Rc<RefCell<Instance>>>,
    instance_cache: InstanceCache,
}

impl Resolver {
    pub fn new() -> Resolver {
        let instance_cache = InstanceCache::new();

        Resolver {
            instance_cache: instance_cache,
            ordered_instances: Vec::new(),
        }
    }

    pub fn resolve(&mut self, unit_name: &str, args_str: &str) -> Result<(), Error> {
        let instance_id = InstanceId::build(unit_name, args_str)?;
        let instance = self.instance_cache.get(instance_id)?;

        self.ordered_instances = Vec::new();
        self.visit(instance)?;

        Ok(())
    }

    fn visit(&mut self, instance_rc: Rc<RefCell<Instance>>) -> Result<(), Error>{
        let instance_clone = Rc::clone(&instance_rc);
        let instance_refcell = &instance_clone;
        let run_state = instance_refcell.borrow().run_state.clone();

        match run_state {
            RunState::Init => {
                set_state(&instance_refcell, RunState::Resolving);
                for child in self.get_deps(&instance_refcell.borrow())?.iter() {
                    let clone = Rc::clone(&child);
                    self.visit(clone)?;
                }
                set_state(&instance_refcell, RunState::Resolved);
                self.ordered_instances.push(Rc::clone(&instance_clone));
                return Ok(())
            },
            RunState::Resolving => {
                let definition = Rc::clone(&instance_refcell.borrow().definition_rc);

                let error = Error::new(
                    format!("Circular dependency on {}", definition.name)
                ); 
                return Err(error)
            },
            RunState::Resolved => Ok(()),
        }
    }

    fn get_deps(&mut self, instance: &Instance) -> Result<Vec<Rc<RefCell<Instance>>>, Error> {
        let execution_result = execute(instance, "deps")?;
        let definition = Rc::clone(&instance.definition_rc);

        if execution_result.exit_code != 0 {
            let error = Error::new(format!("Unit {} deps exited with {}: {}",
                definition.name,
                execution_result.exit_code,
                execution_result.stderr
            ));

            return Err(error);
        }

        execution_result.stdout.lines()
            .filter(|line| !line.is_empty())
            .map(|line| parse_dependency(instance, line))
            .map(|instance_id| {
                match instance_id {
                    Ok(id) => self.instance_cache.get(id),
                    Err(e) => Err(e),
                }
            })
            .collect::<Result<Vec<Rc<RefCell<Instance>>>, Error>>()
    }
}

fn parse_dependency(instance: &Instance, line: &str) -> Result<InstanceId, Error> {
    let parts = line.split(":").collect::<Vec<&str>>();
    match parts.len() {
        1 => Ok(InstanceId::new(line.to_string(), ArgSet::new())),
        2 =>  Ok(InstanceId::build(parts[0], parts[1])?),
        _ => {
            let definition_rc = Rc::clone(&instance.definition_rc);
            let msg = format!(
                "Unit {} deps parse error on string: {}",
                definition_rc.name,
                line.to_string()
            );

            let error = Error::new(msg);

            Err(error)
        }
    }
}

fn set_state(instance_refcell: &RefCell<Instance>, run_state: RunState) {
    let mut instance = instance_refcell.borrow_mut();
    instance.run_state = run_state;
}
