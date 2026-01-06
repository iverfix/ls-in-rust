use super::flag::Flag;
use std::env;

pub fn parse_cli_arguments() -> Vec<Flag> {
    env::args()
        .skip(1)
        .filter_map(|arg| {
            let parse_result = if let Some(flag) = arg.strip_prefix("--") {
                Flag::try_from_long(flag)
            } else if let Some(flag) = arg.strip_prefix("-") {
                Flag::try_from(flag)
            } else {
                println!("Could not find flag");
                return None;
            };

            match parse_result {
                Ok(flag) => Some(flag),
                Err(_) => {
                    eprintln!("Unknown flag: {}", arg);
                    None
                }
            }
        })
        .collect()
}
