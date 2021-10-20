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

    let positions = tokens
        .iter()
        .map(|(m, _, _)| m.split_once("-").unwrap())
        .map(|(low, high)| {
            (
                low.parse::<usize>().unwrap(),
                high.parse::<usize>().unwrap(),
            )
        });

    let letters = tokens.iter().map(|(_, l, _)| l.strip_suffix(":").unwrap());

    let passwords = tokens.iter().map(|(_, _, p)| p);

    let mut valid_passwords = 0;

    for (pos, l, p) in izip!(positions, letters, passwords) {
        println!("{} : {}", p, l);
        println!("{}, {}", pos.0, pos.1);
        let letter_appearances = (
            p.chars().nth(pos.0 - 1) == Some(l.chars().nth(0).unwrap()),
            p.chars().nth(pos.1 - 1) == Some(l.chars().nth(0).unwrap()),
        );
        println!(
            "{} ^ {} = {}",
            letter_appearances.0,
            letter_appearances.1,
            letter_appearances.0 ^ letter_appearances.1
        );

        if letter_appearances.0 ^ letter_appearances.1 {
            valid_passwords += 1;
        }
    }

    println!("{}", valid_passwords);

    Ok(())
}
