use configuration::LsConfig;
pub mod arg_parser;
pub mod configuration;
pub mod flag;

pub fn build_configuration() -> LsConfig {
    let arguments = arg_parser::parse_cli_arguments();
    configuration::build_config(arguments)
}
