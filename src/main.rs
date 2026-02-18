pub mod configuration_builder;
pub mod display;
pub mod filesystem;

use crate::{configuration_builder::build_configuration, filesystem::fetch_entries};

fn main() {
    let config = build_configuration();
    let files = fetch_entries(".");

    for file in files {
        display::display(&file, &config);
    }
}
