use configuration::LsConfig;
pub mod arg_parser;
pub mod configuration;

// Separaate argument parsing and configuration builder through flags

pub fn build_configuration() -> LsConfig {
    let arguments = arg_parser::parse_cli_arguments();

    configuration::build_config(arguments)
}
