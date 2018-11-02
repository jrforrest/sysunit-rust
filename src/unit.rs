pub enum Operation {Check, Apply}

pub struct Definition<'a> {
    pub name: &'a str,
    pub check: &'a str,
    pub apply: &'a str
}

pub struct Instance<'a> {
    definition: &'a Definition<'a>,
}

impl<'a> Definition<'a> {
    pub fn new(name: &'a str, check: &'a str, apply: &'a str) -> Definition<'a> {
        Definition{name: name, check: check, apply: apply}
    }

    pub fn get_instance(&self) -> Instance {
        Instance::new(&self)
    }
}

impl<'a> Instance<'a> {
    pub fn new(definition: &'a Definition) -> Instance<'a> {
        Instance{definition: definition}
    }

    pub fn command_for(&self, operation: Operation) -> &'a str {
        match operation {
            Operation::Apply => self.definition.apply,
            Operation::Check => self.definition.check,
        }
    }
}
