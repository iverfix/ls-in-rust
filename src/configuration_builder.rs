use configuration::LsConfig;
use configuration::LsFormat;
pub mod arg_parser;
pub mod configuration;

// Separaate argument parsing and configuration builder through flags

pub fn parse_args() -> LsConfig {
    LsConfig {
        show_hidden_entries: false,
        format: LsFormat::Grid,
        long_list_settings: None,
    }
}

pub fn build_configuration() -> LsConfig {
    let arguments = arg_parser::parse_cli_arguments();

    parse_args()
}
