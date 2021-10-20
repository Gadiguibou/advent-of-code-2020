use std::collections::HashMap;
use std::fs::read_to_string;
use std::io::{self};

fn main() -> io::Result<()> {
    let input = read_to_string("../input")?;
    let entries = input.split("\n\n").map(|e| e.split_ascii_whitespace());

    let required_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let mut valid_entries: u32 = 0;

    'outer: for entry in entries {
        let mut fields = HashMap::new();
        entry
            .filter_map(|field| field.split_once(":"))
            .for_each(|(f, v)| {
                let _ = fields.insert(f, v);
            });

        for field in required_fields.iter() {
            if !fields.contains_key(field) {
                continue 'outer;
            }
        }

        valid_entries += 1;
    }

    println!("{}", valid_entries);

    Ok(())
}
