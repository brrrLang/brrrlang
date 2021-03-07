mod error_handler;
mod config;
mod compiler;

#[macro_use]
extern crate pest_derive;

#[macro_use]
extern crate clap;

use clap::{App};
use crate::config::Config;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let yaml = load_yaml!("clap.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    let conf = Config::load();

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    if let Some(_matches) = matches.subcommand_matches("build") {
        info("Compiling".parse().unwrap(), format!("{}@{}", conf.project.name, conf.project.version));
        let files = index_dir("src");
        for file in files.iter() {
            info("Compile file".to_string(), file.display().to_string());
            let source = fs::read_to_string(file).unwrap();
            let config_clone = conf.clone();
            // std::thread::spawn(move || {
                compiler::compile(source, config_clone)
            // });
        }
    }
}

/// Finds all files in a given directory
fn index_dir<P: AsRef<Path>>(dir: P) -> Vec<PathBuf> {
    let mut p = vec!();

    match fs::read_dir(dir) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(paths) => for path in paths {
            let dir = path.unwrap();
            if dir.metadata().unwrap().is_dir() {
                p.extend(index_dir(dir.path()));
            } else {
                p.push(dir.path());
            }
        },
    }


    p
}

fn info(msg: String, data: String) {
    println!("{} {} {}",
             ansi_term::Color::Green.bold().paint("=>"),
             ansi_term::Color::Green.paint(
                 format!("{}",
                         msg)),
             data);
}
