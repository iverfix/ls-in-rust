pub enum LsFormat {
    Grid,
    Long,
}

pub struct LongListSettings {
    show_filename: bool,
    show_owner_group: bool,
    show_user_group: bool,
    show_permissions: bool,
    show_byte_size: bool,
    show_write_time: bool,
    show_num_hard_links: bool,
}

pub struct LsConfig {
    pub show_hidden_entries: bool,
    pub format: LsFormat,
    pub long_list_settings: Option<LongListSettings>,
}

pub fn parse_args() -> LsConfig {
    LsConfig {
        show_hidden_entries: false,
        format: LsFormat::Grid,
        long_list_settings: None,
    }
}
