use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use unit::Definition;

pub struct Loader<'a> {
    unit_definitions: Vec<Box<Definition<'a>>>,
    source_str: String,
}

impl<'a> Loader<'a> {
    pub fn new() -> Loader<'a> {
        Loader{unit_definitions: Vec::new(), source_str: String::new()}
    }

    pub fn find(&self, name: &str) -> &'a Definition {
        self.unit_definitions.iter().find(|ud| ud.name == name).expect("Could not find unit!")
    }

    pub fn load(&mut self, path: &str) {
        let glob_path = Path::new(path).join("*.yml");

        let paths = glob::glob(&glob_path.as_os_str().to_string_lossy()).expect("globbodobledoo");

        for unit_definition_path in paths.filter_map(Result::ok) {
            let file = File::open(unit_definition_path.to_str().unwrap()).
                expect("Could not open unit file");
            file.read_to_string(&mut self.source_str);

            let deserialized_unit: DeserializedUnit<'a> =
                serde_yaml::from_str(&self.source_str).expect("Could not deserialize unit");

            let definition: Definition<'a> = Definition::new(
                deserialized_unit.name, "hi", "joe"
            );

            self.unit_definitions.push(Box::new(definition));
        }
    }
}

#[derive(Deserialize)]
struct DeserializedUnit<'a> {
    #[serde(borrow)]
    name: &'a str,
}
