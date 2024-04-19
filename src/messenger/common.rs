use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufReader;
use xml::attribute::OwnedAttribute;
use xml::EventReader;

pub fn parse_attributes(attributes: &Vec<OwnedAttribute>) -> HashMap<&str, &str> {
    let mut hash: HashMap<&str, &str> = HashMap::new();
    for attribute in attributes {
        hash.insert(&*attribute.name.local_name, &*attribute.value);
    }
    hash
}

pub fn get_parser(path: &str) -> Result<EventReader<BufReader<File>>, io::Error> {
    let file = File::open(&path)?;
    let file = BufReader::new(file);
    Ok(EventReader::new(file))
}
