use std::slice;

#[derive(Clone, Copy)]
pub enum Operation {Check, Apply}

pub struct Definition<'a> {
    pub name: String,
    pub check: String,
    pub apply: String,

    dependencies: Vec<Instance<'a>>,
}

pub struct Instance<'a> {
    definition: &'a Definition<'a>,
}

impl<'a> Definition<'a> {
    pub fn new<'b>(name: &'b str, check: &'b str, apply: &'b str) -> Definition<'a> {
        Definition{
            name: String::from(name),
            check: String::from(check),
            apply: String::from(apply),
            dependencies: Vec::new()
        }
    }

    pub fn get_instance(&self) -> Instance {
        Instance::new(&self)
    }

    pub fn depends_on(&mut self, child_instance: Instance<'a>) {
        self.dependencies.push(child_instance);
    }

    pub fn iterate_dependencies(&self) -> slice::Iter<Instance<'a>> {
        self.dependencies.iter()
    }
}

impl<'a> Instance<'a> {
    pub fn new(definition: &'a Definition) -> Instance<'a> {
        Instance{definition: definition}
    }

    pub fn get_name(&self) -> &String {
        &self.definition.name
    }

    pub fn command_for(&self, operation: Operation) -> &'a String {
        match operation {
            Operation::Apply => &self.definition.apply,
            Operation::Check => &self.definition.check,
        }
    }

    pub fn iterate_dependencies(&self) -> slice::Iter<Instance<'a>> {
        self.definition.iterate_dependencies()
    }
}
