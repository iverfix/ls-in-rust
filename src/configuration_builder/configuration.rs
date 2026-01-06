use std::collections::HashMap;

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

pub fn build_config(cli_args: Vec<String>) -> LsConfig {
    let mut config = LsConfig::default();
    let function_map = build_functional_map();

    for arg in cli_args {
        match function_map.get(&arg) {
            Some(function) => function(&mut config),
            _ => println!("Could not find flag"),
        }
    }

    config
}

fn build_functional_map() -> HashMap<String, fn(&mut LsConfig)> {
    let mut map: HashMap<String, fn(&mut LsConfig)> = HashMap::new();
    map.insert("g".to_string(), flag_g);
    map.insert("G".to_string(), flag_cap_g);
    map.insert("o".to_string(), flag_o);
    map.insert("l".to_string(), flag_l);
    map.insert("a".to_string(), flag_a);

    map
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
