use std::{collections::HashMap, env};

const VALID_FLAGS: [&str; 5] = ["a", "g", "G", "l", "o"];

fn long_to_short_format() -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();

    map.insert("all".to_string(), "a".to_string());
    map.insert("no-group".to_string(), "G".to_string());

    map
}

pub fn parse_cli_arguments() -> Vec<String> {
    let long_map = long_to_short_format();
    let mut output = Vec::new();

    for arg in env::args().skip(1) {
        if let Some(key) = arg.strip_prefix("--").and_then(|key| long_map.get(key)) {
            if !VALID_FLAGS.contains(&key.as_str()) {
                continue;
            }
            output.push(key.clone());
        } else if let Some(key) = arg.strip_prefix("-") {
            if !VALID_FLAGS.contains(&key) {
                continue;
            }
            output.push(key.to_string());
        }
    }

    output
}
