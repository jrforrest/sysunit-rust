use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use unit::Definition;

use glob;
use serde_yaml;

pub struct Loader<'a> {
    unit_definitions: Vec<Box<Definition<'a>>>,
    deserialized_units: Vec<DeserializedUnit>,
}

impl<'a> Loader<'a> {
    pub fn new() -> Loader<'a> {
        Loader{unit_definitions: Vec::new(), deserialized_units: Vec::new()}
    }

    pub fn find(&self, name: &str) -> &'a Definition {
        self.unit_definitions.iter().find(|ud| ud.name == name).expect("Could not find unit!")
    }

    pub fn load(&mut self, path: &str) {
        let glob_path = Path::new(path).join("*.yml");

        let paths = glob::glob(&glob_path.as_os_str().to_string_lossy()).expect("globbodobledoo");

        for unit_definition_path in paths.filter_map(Result::ok) {

            let deserialized_unit: DeserializedUnit = {
                let mut file = File::open(unit_definition_path.to_str().unwrap()).
                    expect("Could not open unit file");
                let mut source_string = String::new();

                file.read_to_string(&mut source_string).expect("Could not read unit file");

                serde_yaml::from_str(source_string.as_str()).expect("Could not deserialize unit")
            };

            let definition: Definition<'a> = Definition::new(
                deserialized_unit.name.as_str(),
                deserialized_unit.check.as_str(),
                deserialized_unit.apply.as_str()
            );

            self.deserialized_units.push(deserialized_unit);
            self.unit_definitions.push(Box::new(definition));
        }
    }
}

#[derive(Deserialize)]
struct DeserializedUnit {
    name: String,
    check: String,
    apply: String,
    deps: Option<Vec<String>>
}
