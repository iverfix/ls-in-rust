// File system module
//
// File type, with correct typing
//
// Input argument parser
//
// Input -> Parsing -> Set config -> fetch relevant files and dirs -> Format output -> Display
//
// Need an interface to figure out the terminal width
//
// Outwards dependencies: Terminal "gui" interface (for fetching terminal sizes), file system interface

pub mod configuration_builder;
pub mod display;
pub mod filesystem;

use crate::{configuration_builder::build_configuration, display::create_entry_string};
use std::fs::{self, DirEntry};

fn fetch_file_content(path: &str) -> Vec<DirEntry> {
    let dir_entries = fs::read_dir(path).expect("Data to be found");
    dir_entries.flatten().collect()
}
fn main() {
    let config = build_configuration();

    let files = fetch_file_content(".");

    for file in files {
        println!(
            "{}",
            create_entry_string(&file).unwrap_or(String::from(" "))
        )
    }

    println!("Config {}", config.show_hidden_entries);
}
