use std::{fs, process::exit, fmt::Debug};
use crate::error_handler::{Error,error::error_reporter};
use std::collections::HashMap;
use serde::Deserialize;

#[derive(Clone,Debug,Deserialize)]
pub struct Config {
    pub project: Project,
    pub dependencies: HashMap<String, String>
}

#[derive(Clone,Debug,Deserialize)]
pub struct Project {
    pub name: String,
    pub version: String,
    pub author: Option<String>,
}

impl Config {
    pub fn load() -> Self {
        let source = match fs::read_to_string("brrr.toml") {
            Err(_) => {
                error_reporter(Error::new(
                    "Config".parse().unwrap(),
                    -1,
                    "Could not load brrr.toml".parse().unwrap(),
                    "".parse().unwrap()
                ));
                exit(-1);
            }
            Ok(e) => e
        };

        return match toml::from_str::<Config>(&source) {
            Err(_) => {
                error_reporter(Error::new(
                    "Config".parse().unwrap(),
                    -1,
                    "Could not parse brrr.toml".parse().unwrap(),
                    "".parse().unwrap()
                ));
                exit(-1);
            }
            Ok(a) => a
        };
    }
}
