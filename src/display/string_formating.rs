use std::fs::DirEntry;

use crate::filesystem::EntryType;

pub fn get_colored_string(entry: &DirEntry) -> std::io::Result<String> {
    let metadata = entry.metadata()?;
    let filetype = if metadata.is_file() {
        EntryType::File
    } else if metadata.is_symlink() {
        EntryType::Symlink
    } else if metadata.is_dir() {
        EntryType::Directory
    } else {
        EntryType::Executable
    };

    Ok(format!(
        "\x1b[{}m{}\x1b[0m",
        filetype.ansi_color_code(),
        entry.file_name().to_string_lossy()
    ))
}
