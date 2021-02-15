use std::{fs, io::prelude::*, process::exit, fmt::Debug};
use toml::Value;
use crate::error_handler::{Error,error::error_reporter};

#[derive(Clone,Debug)]
pub struct Config {
    pub project_name: String,
    pub project_version: String,
    pub root: String,
    pub dependencies: Vec<(String, String)>,
    pub authors: Vec<String>,
}

pub fn load_projects() -> Config {
    let mut conf_struct: Config = Config{project_name:String::new(),project_version:String::new(),root:String::new(),dependencies:vec!(),authors:vec!()};

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
            exit(-1);
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

    let config_toml: Value = source.parse().unwrap();
    match &config_toml {
        Value::Table(config_table) => {
            match config_table.get("project") {
                None => {
                    let mut conf_error = error_template.clone();
                    conf_error.message = format!("Could not find [[project]], please make sure there is a [[project]] in the config file");
                    error_reporter(conf_error);
                    exit(-1);
                }
                Some(config_project_array) => {
                    match config_project_array {
                        Value::Array(config_project_table) => {
                            for config_project_table_partition in config_project_table.iter() {
                                match config_project_table_partition {
                                    Value::Table(config_project_element) => {
                                        let project_name = match config_project_element.get("name") {
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

                                        let project_version = match config_project_element.get("version") {
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

                                        let root = match config_project_element.get("root") {
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

                                        let authors = match config_project_element.get("version") {
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

                                        conf_struct = Config {authors, root, project_version, project_name, dependencies};
                                    }
                                    _ => {
                                        let mut conf_error = error_template.clone();
                                        conf_error.message = format!("Internal error finding config_project_element");
                                        error_reporter(conf_error);
                                        exit(-1)
                                    }
                                }
                            };
                        },
                        _ => {
                            let mut conf_error = error_template.clone();
                            conf_error.message = format!("Internal error finding config_project_table");
                            error_reporter(conf_error);
                            exit(-1)
                        }
                    }
                }
            }
            match config_table.get("dependencies") {
                Some(dependencies_array) => {
                    match dependencies_array  {
                        Value::Array(dependencies_table) => {
                            match &dependencies_table[0] {
                                Value::Table(dependencies_list) => {
                                    for dependency in dependencies_list.iter() {
                                        match dependency.1 {
                                            Value::String(dependency_version) => {
                                                conf_struct.dependencies.append(&mut vec!((dependency.0.clone(),dependency_version.clone())));
                                            }
                                            _ => {
                                                let mut conf_error = error_template.clone();
                                                conf_error.message = format!("Dependencies version must be a string");
                                                conf_error.line_text = format!("{}",dependency.0);
                                                error_reporter(conf_error);
                                                exit(-1)
                                            }
                                        }
                                    }
                                }
                                _ => {
                                    let mut conf_error = error_template.clone();
                                    conf_error.message = format!("General failure reading dependencies");
                                    error_reporter(conf_error);
                                    exit(-1)
                                }
                            }
                        }
                        _ => {
                            let mut conf_error = error_template.clone();
                            conf_error.message = format!("Empty config file");
                            error_reporter(conf_error);
                            exit(-1)
                        }
                    }
                }
                None => {}
            }
        }
        _ => {
            let mut conf_error = error_template.clone();
            conf_error.message = format!("Empty config file");
            error_reporter(conf_error);
            exit(-1)
        }
    }
    return conf_struct
}