use std::collections::HashMap;
use crate::unit::{Instance, DefinitionRc, InstanceId};
use std::rc::Rc;
use std::cell::RefCell;
use crate::loader::load_unit;
use crate::error::Error;

pub type InstanceRc = Rc<RefCell<Instance>>;

type DefinitionRcResult = Result<DefinitionRc, Error>;

pub struct InstanceCache {
    lookup_table: HashMap<String, InstanceRc>,
    definition_cache: DefinitionCache,
}

struct DefinitionCache {
    lookup_table: HashMap<String, DefinitionRc>
}

impl InstanceCache {
    pub fn new() -> InstanceCache {
        InstanceCache {
            lookup_table: HashMap::new(),
            definition_cache: DefinitionCache::new(),
        }
    }

    pub fn get(&mut self, instance_id: InstanceId) -> Result<InstanceRc, Error> {
        let signature = instance_id.signature();
        match self.lookup_table.get(&signature) {
            Some(instance) => Ok(Rc::clone(instance)),
            None => {
                let definition_rc = self.definition_cache.get(&instance_id.name)?;
                let instance = Instance::new(definition_rc, instance_id);
                let cell = RefCell::new(instance);
                let rc = Rc::new(cell);
                let rc_clone = Rc::clone(&rc);

                self.lookup_table.insert(signature, rc);

                return Ok(rc_clone)
            }
        }
    }
}

impl DefinitionCache {
    pub fn new() -> DefinitionCache {
        DefinitionCache { lookup_table: HashMap::new() }
    }

    pub fn get(&mut self, unit_name: &str) -> DefinitionRcResult {
        let unit_name_string = unit_name.to_string();

        match self.lookup_table.get(&unit_name_string) {
            Some(definition) => Ok(Rc::clone(definition)),
            None => {
                let definition = load_unit(unit_name)?;
                let rc = Rc::new(definition);
                let rc_clone = Rc::clone(&rc);

                self.lookup_table.insert(unit_name_string, rc);

                Ok(rc_clone)
            }
        }
    }
}
