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

impl LongListSettings {
    fn enable_all() -> Self {
        Self {
            show_filename: true,
            show_owner_group: true,
            show_user_group: true,
            show_permissions: true,
            show_byte_size: true,
            show_write_time: true,
            show_num_hard_links: true,
        }
    }

    fn default() -> Self {
        Self {
            show_filename: false,
            show_owner_group: false,
            show_user_group: false,
            show_permissions: false,
            show_byte_size: false,
            show_write_time: false,
            show_num_hard_links: false,
        }
    }
}

pub struct LsConfig {
    pub show_hidden_entries: bool,
    pub format: LsFormat,
    pub long_list_settings: Option<LongListSettings>,
}

fn flag_g(config: &mut LsConfig) {
    config.show_hidden_entries = true;
    config.format = LsFormat::Long;
    config.long_list_settings = Some(LongListSettings::enable_all());
}

fn flag_cap_g(config: &mut LsConfig) {
    if let Some(long_config) = &mut config.long_list_settings {
        long_config.show_owner_group = false;
    } else {
        config.long_list_settings = Some(LongListSettings::default());
    }
}

fn flag_o(config: &mut LsConfig) {
    let mut long_config = LongListSettings::enable_all();
    long_config.show_owner_group = false;

    config.format = LsFormat::Long;
    config.long_list_settings = Some(long_config);
}

fn flag_l(config: &mut LsConfig) {
    config.format = LsFormat::Long;
    config.long_list_settings = Some(LongListSettings::enable_all());
}

fn flag_a(config: &mut LsConfig) {
    config.show_hidden_entries = true;
}
