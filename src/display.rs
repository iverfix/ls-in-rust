use crate::filesystem::get_colored_string;
use std::fs::DirEntry;
use std::io;
use std::time;

pub enum EntryError {
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

pub fn display_long_format() {
    println!("Long format");
}

pub fn create_entry_string(entry: &DirEntry) -> Result<String, EntryError> {
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
