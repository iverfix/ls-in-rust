pub mod configuration_builder;
pub mod display;
pub mod filesystem;

use crate::{configuration_builder::build_configuration, filesystem::fetch_entries};

fn main() -> Result<(), String> {
    let config = build_configuration()?;
    let files = fetch_entries(".");

    display::display(&files, &config);

    Ok(())
}
