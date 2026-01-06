use super::flag::Flag;

pub enum LsFormat {
    Grid,
    Long,
}

#[derive(Default)]
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
}

pub struct LsConfig {
    pub show_hidden_entries: bool,
    pub format: LsFormat,
    pub long_list_settings: Option<LongListSettings>,
}

impl Default for LsConfig {
    fn default() -> Self {
        Self {
            show_hidden_entries: false,
            format: LsFormat::Grid,
            long_list_settings: None,
        }
    }
}

pub fn build_config(cli_flags: Vec<Flag>) -> LsConfig {
    let mut config = LsConfig::default();

    for flag in cli_flags {
        match flag {
            Flag::G => flag_g(&mut config),
            Flag::CapG => flag_cap_g(&mut config),
            Flag::O => flag_o(&mut config),
            Flag::L => flag_l(&mut config),
            Flag::A => flag_a(&mut config),
        }
    }

    config
}

fn flag_g(config: &mut LsConfig) {
    config.show_hidden_entries = false;
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
    config.format = LsFormat::Long;
    let long_config = config
        .long_list_settings
        .get_or_insert_with(LongListSettings::enable_all);
    long_config.show_owner_group = false;
}

fn flag_l(config: &mut LsConfig) {
    config.format = LsFormat::Long;
    config.long_list_settings = Some(LongListSettings::enable_all());
}

fn flag_a(config: &mut LsConfig) {
    config.show_hidden_entries = true;
}
