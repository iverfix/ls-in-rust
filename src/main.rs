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

use std::fs::{self, DirEntry};
use std::{io, time};

enum EntryType {
    Directory,
    Symlink,
    Executable,
    File,
}

impl EntryType {
    fn ansi_color_code(&self) -> &'static str {
        match self {
            EntryType::Directory => "1;34",
            EntryType::Symlink => "1;32",
            EntryType::Executable => "1;36",
            EntryType::File => "",
        }
    }
}

enum EntryError {
    Io(),
    Time(),
}

impl From<io::Error> for EntryError {
    fn from(_: io::Error) -> Self {
        EntryError::Io()
    }
}

impl From<time::SystemTimeError> for EntryError {
    fn from(_: time::SystemTimeError) -> Self {
        EntryError::Time()
    }
}

fn fetch_file_content(path: &str) -> Vec<DirEntry> {
    let dir_entries = fs::read_dir(path).expect("Data to be found");
    dir_entries.flatten().collect()
}

fn get_colored_string(entry: &DirEntry) -> std::io::Result<String> {
    let metadata = entry.metadata()?;
    let filetype;
    if metadata.is_file() {
        filetype = EntryType::File;
    } else if metadata.is_symlink() {
        filetype = EntryType::Symlink;
    } else if metadata.is_dir() {
        filetype = EntryType::Directory;
    } else {
        filetype = EntryType::Executable;
    }

    Ok(format!(
        "\x1b[{}m{}\x1b[0m",
        filetype.ansi_color_code(),
        entry.file_name().to_string_lossy()
    ))
}

fn create_entry_string(entry: &DirEntry) -> Result<String, EntryError> {
    let mut entry_string = String::new();

    let file_name = get_colored_string(entry)?;
    entry_string.push_str(&file_name);
    entry_string.push(' ');

    let metadata = entry.metadata()?;
    let accessed = metadata.accessed()?;
    let elapsed = accessed.elapsed()?;

    entry_string.push_str(&elapsed.as_secs().to_string());

    Ok(entry_string)
}

fn main() {
    let files = fetch_file_content(".");

    for file in files {
        println!(
            "{}",
            create_entry_string(&file).unwrap_or(String::from(" "))
        )
    }
}
