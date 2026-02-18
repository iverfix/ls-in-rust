use crate::{
    display::{EntryError, string_formating::get_colored_string},
    filesystem::Entry,
};

pub fn display_short(entry: &Entry) -> Result<String, EntryError> {
    let file_name = get_colored_string(entry)?;
    Ok(file_name)
}
