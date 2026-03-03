use configuration::LsConfig;
pub mod arg_parser;
pub mod configuration;
pub mod flag;

pub fn build_configuration() -> Result<LsConfig, String> {
    let arguments = arg_parser::parse_cli_arguments()?;
    Ok(configuration::build_config(arguments))
}
