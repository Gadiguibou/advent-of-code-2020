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
    let groups = input.split("\n\n").map(|g| g.split('\n'));

    let mut total: usize = 0;

    groups.for_each(|g| {
        let questions_with_yes = g.map(|p| p.chars().collect::<HashSet<char>>());

        let common_questions_with_yes = questions_with_yes.reduce(|acc, s| {
            acc.intersection(&s)
                .map(ToOwned::to_owned)
                .collect::<HashSet<char>>()
        });

        if let Some(i) = common_questions_with_yes {
            total += i.len()
        }
    });

    println!("{}", total);

    Ok(())
}
