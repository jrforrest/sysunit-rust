use loader::Loader;
use std::rc::Rc;
use unit::{Instance, Definition};

pub struct Instantiator<'a>  {
    loader: &'a Loader<'a>,
    loaded_instances: Vec<Rc<Instance<'a>>>
}

impl <'a> Instantiator<'a> {
    pub fn new(loader: &'a Loader) -> Instantiator<'a> {
        Instantiator{loader: loader, loaded_instances: Vec::new()}
    }

    pub fn instantiate(&mut self, name: &str) -> Rc<Instance<'a>> {
        match self.find_loaded(name) {
            Some(found_instance) => found_instance,
            None => self.create_instance(name)
        }
    }

    pub fn create_instance(&mut self, name: &str) -> Rc<Instance<'a>> {
        let definition: &'a Definition = self.loader.find(name);
        let instance_rc = Rc::new(Instance::new(definition));

        self.loaded_instances.push(instance_rc.clone());

        instance_rc
    }

    fn find_loaded(&self, name: &str) -> Option<Rc<Instance<'a>>> {
        match self.loaded_instances.iter().find(|i| i.get_name() == name) {
            Some(x) => Some(x.clone()),
            None => None
        }
    }
}
