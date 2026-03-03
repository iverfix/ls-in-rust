use crate::configuration_builder::configuration::LsConfig;
use crate::configuration_builder::configuration::LsFormat;
use crate::display::long_format::display_long_format;
use crate::display::short_format::display_short_format;
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

pub fn display(entries: &Vec<Entry>, configuration: &LsConfig) {
    match configuration.format {
        LsFormat::Grid => display_short_format(entries),
        LsFormat::Long => display_long_format(entries),
    }
}
