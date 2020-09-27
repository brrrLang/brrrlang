use std::{fs, io::prelude::*, process::exit, fmt::Debug};
use toml::Value;
use dyn_clone::private::fmt::Formatter;

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
    let mut conf: Vec<Config> = vec![];

    let mut file = match fs::File::open("brrr.toml") {
        Err(a) => {
            eprintln!("Error opening file: {} \
        Folder should contain a brrr.toml", a);
            exit(-1)
        }
        Ok(a) => a
    };

    let mut source = String::new();
    match file.read_to_string(&mut source) {
        Err(a) => {
            eprintln!("Error reading file, brrr.toml\n{}", a);
            exit(-1)
        }
        _ => {}
    };

    let config: Value = source.parse().unwrap();
    match &config {
        Value::Table(a) => {
            match a.get("project") {
                None => {
                    eprintln!("Error reading config file, please make sure there is a [[project]] in the config file");
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
                                                    eprintln!("Name should be a string");
                                                    exit(-1)
                                                }
                                            },
                                            None => {
                                                eprintln!("Project should have a name");
                                                exit(-1);
                                            }
                                        };

                                        let project_version = match e.get("version") {
                                            Some(g) => match g {
                                                Value::String(h) => h.to_owned(),
                                                _ => {
                                                    eprintln!("Name should be a string");
                                                    exit(-1)
                                                }
                                            },
                                            None => {
                                                eprintln!("Project should have a name");
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

                                        Config {authors, root, project_version, project_name, dependencies}
                                    }
                                    _ => {
                                        eprintln!("You dumbfucked up, idiot");
                                        exit(-1)
                                    }
                                }
                            }).collect::<Vec<Config>>();
                        },
                        _ => {eprintln!("Error with the config, fuck you"); exit(-1)}
                    }
                }
            }
        }
        _ => {
            eprintln!("You dumbfucked up, idiot");
            exit(-1)
        }
    }

    conf
}