use super::flag::Flag;
use std::env;

pub fn parse_cli_arguments() -> Vec<Flag> {
    let mut result = Vec::new();
    for arg in env::args().skip(1) {
        if let Some(flag) = arg.strip_prefix("--") {
            if let Ok(f) = Flag::try_from_long(flag) {
                result.push(f);
            }
        } else if let Some(flag) = arg.strip_prefix("-") {
            for character in flag.chars() {
                if let Ok(f) = Flag::try_from(character) {
                    result.push(f);
                }
            }
        }
    }
    result
}
