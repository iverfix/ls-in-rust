use crate::filesystem::{Entry, EntryType};

fn ansi_color_code(entry_type: &EntryType) -> &'static str {
    match entry_type {
        EntryType::Directory => "1;34",
        EntryType::Symlink => "1;32",
        EntryType::Executable => "1;36",
        EntryType::File => "",
    }
}

pub fn get_colored_string(entry: &Entry) -> std::io::Result<String> {
    Ok(format!(
        "\x1b[{}m{}\x1b[0m",
        ansi_color_code(&entry.entry_type),
        entry.file_name
    ))
}
