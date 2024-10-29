use std::fs;

use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    project: String,
    description: String,
    author: String,
    package: String,
    language: String,
    framework: String,
    version: String
}

#[derive(Deserialize)]
struct Lines;

#[derive(Deserialize)]
struct Data {
    config: Config
}

pub struct Project;
impl Project {
    pub fn read(filename: &str) -> Data {
        let contents = fs::read_to_string(filename).ok();
        toml::from_str::<Data>(&contents.unwrap()).unwrap()
    }
}