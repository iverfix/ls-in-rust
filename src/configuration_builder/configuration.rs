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

fn flag_g(config: &mut LsConfig) {}
