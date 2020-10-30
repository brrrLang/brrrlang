use std::{fs, process};
use std::fs::File;
use std::io::prelude::*;

use dyn_clone::private::fs::DirEntry;
use rayon::prelude::*;

use crate::error_handler;
use crate::import_manager::*;
use crate::token;

pub fn load_directory() -> Vec<ParsedFile> {
    let ye =
        // Get all files in the src directory
        fs::read_dir("./src")
            .unwrap()
            // Unwrap all paths
            .map(|a| a.unwrap())
            // Keep only files ending in .bf
            .filter(|path| path.file_name().to_str().unwrap().to_string().ends_with(".bl"))
            // Convert to a parallel iterator for Z O O M
            .collect::<Vec<DirEntry>>()
            .par_iter()
            // Read and parse all files
            .map(|path| {
                println!("Parsing {:?}", path);
                let mut data = String::new();
                // Open the file
                return match File::open(path.path()).unwrap().read_to_string(&mut data) {
                    // Check there were no errors when the file was red
                    Err(e) => {
                        eprintln!("Error reading file {} \n", e);
                        None
                    }
                    // Parse the opened file
                    _ => {
                        // Add the files to a list of files
                        Some(ParsedFile {
                            lines: token::tokenizer::tokenize(&data, &num_cpus::get(), &0),
                            file_path: path.file_name().to_str().unwrap().to_string(),
                        })
                    }
                };
            })
            // Remove any Nones
            .filter(|a| a.is_some())
            .collect::<Vec<Option<ParsedFile>>>();

    // Unwrap all files
    let mut d = vec!();
    for a in ye {
        d.push(a.unwrap());
    }

    d
}
