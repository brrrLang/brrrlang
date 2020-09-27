use std::{fs, io::prelude::*, process::exit, fmt::Debug};
use toml::Value;
use dyn_clone::private::fmt::Formatter;
use crate::error_handler::{Error,error::error_reporter};

#[derive(Clone)]
pub struct Config {
    pub project_name: String,
    pub project_version: String,
    pub root: String,
    pub dependencies: Vec<(String, String)>,
    pub authors: Vec<String>,
}

impl Debug for Config {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{name: {}, version: {}, root: {}, dependencies: {:?}, authors - {:?}}}", self.project_name, self.project_version, self.root, self.dependencies, self.authors)
    }
}

pub fn load_projects() -> Vec<Config> {
    let conf: Vec<Config>;

    let error_template: Error = Error::new(
        String::from("Config parsing"),
         -1,
        String::from("Unknown Error"),
        String::new(),
    );

    let mut file = match fs::File::open("brrr.toml") {
        Ok(a) => a,
        Err(a) => {
            let mut conf_error = error_template.clone();
            conf_error.message = format!("Error opening file: {} \
            Folder should contain a brrr.toml", a);
            error_reporter(conf_error);
            exit(-1); //Defiantly still a young language
        }
    };

    let mut source = String::new();
    match file.read_to_string(&mut source) {
        Err(a) => {
            let mut conf_error = error_template.clone();
            conf_error.message = format!("Error reading file, brrr.toml:\n{}", a);
            error_reporter(conf_error);
        }
        _ => {}
    };

    let config: Value = source.parse().unwrap();
    match &config {
        Value::Table(a) => {
            match a.get("project") {
                None => {
                    let mut conf_error = error_template.clone();
                    conf_error.message = format!("Could not find [[project]], please make sure there is a [[project]] in the config file");
                    error_reporter(conf_error);
                    exit(-1);
                }
                Some(b) => {
                    match b {
                        Value::Array(c) => {
                            conf = c.iter().map(|d| {
                                match d {
                                    Value::Table(e) => {
                                        let project_name = match e.get("name") {
                                            Some(g) => match g {
                                                Value::String(h) => h.to_owned(),
                                                _ => {
                                                    let mut conf_error = error_template.clone();
                                                    conf_error.message = format!("Project name should be a string");
                                                    error_reporter(conf_error);
                                                    exit(-1);
                                                }
                                            },
                                            None => {
                                                let mut conf_error = error_template.clone();
                                                conf_error.message = format!("Project should have a name");
                                                error_reporter(conf_error);
                                                exit(-1);
                                            }
                                        };

                                        let project_version = match e.get("version") {
                                            Some(g) => match g {
                                                Value::String(h) => h.to_owned(),
                                                _ => {
                                                    let mut conf_error = error_template.clone();
                                                    conf_error.message = format!("Project version should be a string");
                                                    error_reporter(conf_error);
                                                    exit(-1)
                                                }
                                            },
                                            None => {
                                                let mut conf_error = error_template.clone();
                                                conf_error.message = format!("Project should have a version");
                                                error_reporter(conf_error);
                                                exit(-1);
                                            }
                                        };

                                        let root = match e.get("root") {
                                            Some(g) => match g {
                                                Value::String(h) => h.to_owned(),
                                                _ => {
                                                    "src/main.bl".to_string()
                                                }
                                            },
                                            None => {
                                                "src/main.bl".to_string()
                                            }
                                        };

                                        let authors = match e.get("version") {
                                            Some(g) => match g {
                                                Value::Array(h) => {
                                                    h.iter().map(|i| match i {
                                                        Value::String(j) => j.to_string(),
                                                        _ => "".to_string()
                                                    }).collect()
                                                }
                                                _ => {
                                                    vec![]
                                                }
                                            },
                                            None => {
                                                vec![]
                                            }
                                        };

                                        let dependencies = vec![];

                                        return Config {authors, root, project_version, project_name, dependencies};
                                    }
                                    _ => {
                                        let mut conf_error = error_template.clone();
                                        conf_error.message = format!("Unknown error");
                                        error_reporter(conf_error);
                                        exit(-1)
                                    }
                                }
                            }).collect::<Vec<Config>>();
                        },
                        _ => {
                            let mut conf_error = error_template.clone();
                            conf_error.message = format!("Unknown error");
                            error_reporter(conf_error);
                            exit(-1)
                        }
                    }
                }
            }
        }
        _ => {
            let mut conf_error = error_template.clone();
            conf_error.message = format!("Unknown error");
            error_reporter(conf_error);
            exit(-1)
        }
    }

    return conf
}