use serde_yaml;
use std::collections::HashMap;
use std::env;

pub struct Config {
    pub map: HashMap<String, serde_yaml::Value>,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Config {
        let args: Vec<String> = env::args().collect();
        let file_path = &args[args.len() - 1];

        let file = std::fs::File::open(file_path).unwrap();
        let map: HashMap<String, serde_yaml::Value> = serde_yaml::from_reader(file).unwrap();

        Config { map }
    }
}
