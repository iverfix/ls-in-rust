use crate::configuration_builder::configuration::LsConfig;
use crate::configuration_builder::configuration::LsFormat;
use crate::display::long_format::display_long_format;
use crate::display::short_format::display_short;
use crate::filesystem::Entry;
use std::io;
use std::time;

mod long_format;
mod short_format;
mod string_formating;

pub enum EntryError {
    Io(),
    Time(),
}

impl From<io::Error> for EntryError {
    fn from(_: io::Error) -> Self {
        EntryError::Io()
    }
}

impl From<time::SystemTimeError> for EntryError {
    fn from(_: time::SystemTimeError) -> Self {
        EntryError::Time()
    }
}

pub fn display(entry: &Entry, configuration: &LsConfig) {
    let display_string = match configuration.format {
        LsFormat::Grid => display_short(entry),
        LsFormat::Long => display_long_format(entry),
    };

    if let Ok(display) = display_string {
        println!("{}", display)
    }
}
