use itertools::izip;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::{fs::File, usize};

fn main() -> io::Result<()> {
    let f = File::open("../input")?;
    let lines: Vec<String> = BufReader::new(f)
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let tokens: Vec<_> = lines
        .iter()
        .map(|l| match l.split(" ").collect::<Vec<&str>>()[..3] {
            [m, l, p] => (m, l, p),
            _ => panic!("poorly formatted inputs"),
        })
        .collect();

    let multiplicities = tokens
        .iter()
        .map(|(m, _, _)| m.split_once("-").unwrap())
        .map(|(min, max)| (min.parse::<usize>().unwrap(), max.parse::<usize>().unwrap()));

    let letters = tokens.iter().map(|(_, l, _)| l.strip_suffix(":").unwrap());

    let passwords = tokens.iter().map(|(_, _, p)| p);

    let mut valid_passwords = 0;

    for (m, l, p) in izip!(multiplicities, letters, passwords) {
        let letter_appearances: usize = p.matches(l).count();
        if letter_appearances >= m.0 && letter_appearances <= m.1 {
            valid_passwords += 1;
        }
    }

    println!("{}", valid_passwords);

    Ok(())
}
