use std::fs::DirEntry;
use std::os::unix::fs::MetadataExt;
use users::{get_group_by_gid, get_user_by_uid};

pub enum EntryType {
    Directory,
    Symlink,
    Executable,
    File,
}

impl EntryType {
    pub fn ansi_color_code(&self) -> &'static str {
        match self {
            EntryType::Directory => "1;34",
            EntryType::Symlink => "1;32",
            EntryType::Executable => "1;36",
            EntryType::File => "",
        }
    }
}

pub struct Entry {
    pub user_name: String,
    pub group_name: String,
    pub n_hard_links: i32,
    pub entry_type: EntryType,
}

pub fn fetch_entry_data(dir_entry: &DirEntry) -> std::io::Result<Entry> {
    let metadata = dir_entry.metadata()?;

    let user_id = metadata.uid();
    let group_id = metadata.gid();

    let user_name = get_user_by_uid(user_id)
        .map(|user| user.name().to_string_lossy().to_string())
        .unwrap_or_default();

    let group_name = get_group_by_gid(group_id)
        .map(|group| group.name().to_string_lossy().to_string())
        .unwrap_or_default();

    Ok(Entry {
        user_name,
        group_name,
        n_hard_links: 0,
        entry_type: EntryType::Executable,
    })
}
