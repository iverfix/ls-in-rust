use super::flag::Flag;
use std::env;

pub fn parse_cli_arguments() -> Result<Vec<Flag>, String> {
    let mut result = Vec::new();
    for arg in env::args().skip(1) {
        if let Some(flag) = arg.strip_prefix("--") {
            if let Ok(flag) = Flag::try_from_long(flag) {
                result.push(flag);
            } else {
                return Err(format!("Unexpected option argument: {}", arg));
            }
        } else if let Some(flag) = arg.strip_prefix("-") {
            for character in flag.chars() {
                if let Ok(flag) = Flag::try_from(character) {
                    result.push(flag);
                } else {
                    return Err(format!("Unexpected option argument: {}", arg));
                }
            }
        } else {
            return Err(format!("Unexpected positional argument: {}", arg));
        }
    }
    Ok(result)
}
