use crate::filesystem::fetch_entry_data;
use crate::filesystem::get_colored_string;
use std::fs::DirEntry;
use std::io;
use std::time;

mod output;

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

pub fn display_line() {}

pub fn create_entry_string(entry: &DirEntry) -> Result<String, EntryError> {
    let mut entry_string = String::new();

    let file_name = get_colored_string(entry)?;
    entry_string.push_str(&file_name);
    entry_string.push(' ');

    let metadata = entry.metadata()?;
    let accessed = metadata.accessed()?;
    let elapsed = accessed.elapsed()?;

    let entry_data = fetch_entry_data(entry)?;

    // Logic for potentially fetcing user flags
    // let permissions: u32 = metadata.permissions().mode();
    // println!("{:o}", permissions);
    //

    //entry_string.push_str(&elapsed.as_secs().to_string());
    entry_string = format!(
        "{} {} {} {}",
        entry_data.n_hard_links, entry_data.user_name, entry_data.group_name, file_name
    );

    Ok(entry_string)
}
