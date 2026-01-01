use std::fs::DirEntry;

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

pub fn get_colored_string(entry: &DirEntry) -> std::io::Result<String> {
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
