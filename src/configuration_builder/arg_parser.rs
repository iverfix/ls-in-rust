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
        if let Some(flag) = arg.strip_prefix("--").and_then(|flag| long_map.get(flag)) {
            output.push(flag.clone());
        } else if let Some(flag) = arg.strip_prefix("-")
            && VALID_FLAGS.contains(&flag)
        {
            output.push(flag.to_string());
        }
    }

    output
}
