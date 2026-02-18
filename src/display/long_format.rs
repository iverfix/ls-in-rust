use crate::display::EntryError;
use crate::display::string_formating::get_colored_string;
use crate::filesystem::Entry;

pub fn display_long_format(entry: &Entry) -> Result<String, EntryError> {
    create_entry_string(entry)
}

fn create_entry_string(entry: &Entry) -> Result<String, EntryError> {
    // let metadata = entry.metadata()?;
    // let accessed = metadata.accessed()?;
    // let elapsed = accessed.elapsed()?;
    //
    // let entry_data = fetch_entry_data(entry)?;

    // Logic for potentially fetcing user flags
    // let permissions: u32 = metadata.permissions().mode();
    // println!("{:o}", permissions);
    //

    //entry_string.push_str(&elapsed.as_secs().to_string());
    let entry_string = format!(
        "{} {} {} {}",
        entry.n_hard_links,
        entry.user_name,
        entry.group_name,
        get_colored_string(entry)?
    );

    Ok(entry_string)
}
