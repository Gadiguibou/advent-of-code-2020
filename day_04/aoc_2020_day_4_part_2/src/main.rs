use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::io;

static REQUIRED_FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
fn main() -> io::Result<()> {
    let input = read_to_string("../input")?;
    let entries = input.split("\n\n").map(|e| e.split_ascii_whitespace());

    let mut valid_entries: u32 = 0;

    'outer: for entry in entries {
        let mut fields = HashMap::new();
        entry
            .filter_map(|field| field.split_once(":"))
            .for_each(|(f, v)| {
                let _ = fields.insert(f, v);
            });

        for field in REQUIRED_FIELDS.iter() {
            if !validate_field(field, &fields) {
                continue 'outer;
            }
        }

        valid_entries += 1;
    }

    println!("{}", valid_entries);

    Ok(())
}

fn validate_field(field: &str, fields: &HashMap<&str, &str>) -> bool {
    let field_value = match fields.get(field) {
        Some(v) => v,
        None => return false,
    };

    lazy_static! {
        static ref HCL_RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    }

    lazy_static! {
        static ref PID_RE: Regex = Regex::new(r"^\d{9}$").unwrap();
    }

    match field {
        "byr" => {
            if let Ok(yr) = field_value.parse::<u32>() {
                yr >= 1920 && yr <= 2002
            } else {
                false
            }
        }
        "iyr" => {
            if let Ok(yr) = field_value.parse::<u32>() {
                yr >= 2010 && yr <= 2020
            } else {
                false
            }
        }
        "eyr" => {
            if let Ok(yr) = field_value.parse::<u32>() {
                yr >= 2020 && yr <= 2030
            } else {
                false
            }
        }
        "hgt" => {
            if let Some(Some(h)) = field_value
                .strip_suffix("cm")
                .map(|s| s.parse::<u32>().ok())
            {
                h >= 150 && h <= 193
            } else if let Some(Some(h)) = field_value
                .strip_suffix("in")
                .map(|s| s.parse::<u32>().ok())
            {
                h >= 59 && h <= 76
            } else {
                false
            }
        }
        "hcl" => HCL_RE.is_match(field_value),
        "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(field_value),
        "pid" => PID_RE.is_match(field_value),
        _ => {
            println!("Unknown field {} ignored", field);
            false
        }
    }
}
