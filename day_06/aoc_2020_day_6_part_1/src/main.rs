#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]

use std::{collections::HashSet, fs::read_to_string, io};

fn main() -> io::Result<()> {
    let input = read_to_string("../input")?;
    let groups = input.split("\n\n");

    let mut total: usize = 0;

    groups.for_each(|g| {
        let mut questions_with_yes = HashSet::new();
        g.chars().for_each(|c| {
            match c {
                '\n' => (),
                c => {
                    questions_with_yes.insert(c);
                }
            };
        });
        total += questions_with_yes.len();
    });

    println!("{}", total);

    Ok(())
}
