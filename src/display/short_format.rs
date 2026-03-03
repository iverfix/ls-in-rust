use crate::{display::string_formating::get_colored_string, filesystem::Entry};

pub fn display_short_format(entries: &Vec<Entry>) {
    let mut output: String = String::new();
    for entry in entries {
        let colored = get_colored_string(entry).unwrap_or_default();
        output.push_str(&colored);
        output.push(' ');
    }
    println!("{output}");
}
