use crate::{display::EntryError, filesystem::get_colored_string};
use std::fs::DirEntry;

pub fn display_short(entry: &DirEntry) -> Result<String, EntryError> {
    let file_name = get_colored_string(entry)?;
    Ok(file_name)
}
