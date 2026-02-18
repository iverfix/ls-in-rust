use std::fs::{self, DirEntry};
use std::os::unix::fs::MetadataExt;
use users::{get_group_by_gid, get_user_by_uid};

pub enum EntryType {
    Directory,
    Symlink,
    Executable,
    File,
}

pub struct Entry {
    pub user_name: String,
    pub group_name: String,
    pub n_hard_links: i32,
    pub entry_type: EntryType,
    pub file_name: String,
}

fn parse_entry_type(dir_entry: &DirEntry) -> EntryType {
    match dir_entry.metadata() {
        Ok(metadata) => {
            if metadata.is_dir() {
                EntryType::Directory
            } else if metadata.is_symlink() {
                EntryType::Symlink
            } else if metadata.is_file() {
                EntryType::File
            } else {
                EntryType::Executable
            }
        }
        Err(_) => EntryType::File,
    }
}

fn parse_entry(dir_entry: &DirEntry) -> std::io::Result<Entry> {
    let metadata = dir_entry.metadata()?;

    let user_name = get_user_by_uid(metadata.uid())
        .map(|user| user.name().to_string_lossy().to_string())
        .unwrap_or_default();

    let group_name = get_group_by_gid(metadata.gid())
        .map(|group| group.name().to_string_lossy().to_string())
        .unwrap_or_default();

    let file_name = dir_entry.file_name().into_string().unwrap_or_default();

    Ok(Entry {
        user_name,
        group_name,
        n_hard_links: 0,
        entry_type: parse_entry_type(dir_entry),
        file_name,
    })
}

pub fn fetch_entries(path: &str) -> Vec<Entry> {
    let dir_entries = match fs::read_dir(path) {
        Ok(entries) => entries,
        Err(_) => return Vec::new(),
    };

    dir_entries
        .filter_map(|entry| {
            let entry = entry.ok()?;
            parse_entry(&entry).ok()
        })
        .collect()
}
