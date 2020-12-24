use std::collections::HashSet;

struct Transporter {
    transported_units: HashSet<String>,
}

impl Transporter {
    pub fn new() -> Transporter {
        Transporter {
            transported_units: HashSet::new(),
        }
    }

    pub fn send_unit(&mut self, unit: &Instance) ->

    pub fn unit_path(&self, unit: &Instance) -> Result<String, Error> {
        match self.transported_units.get(unit.id.signature()) {
            Some(path) => path,
            None => panic!("Unit instance {} not present on host {}",
                unit.id.signature(),
                url.to_string()
            )
        }
    }
}
