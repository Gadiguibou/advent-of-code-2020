use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let f = File::open("../input")?;
    let forest: Vec<_> = BufReader::new(f)
        .lines()
        .filter_map(|l| l.ok())
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();

    let angles = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let result = &angles
        .iter()
        .fold(1, |acc, e| acc * explore_angle(&forest, e.0, e.1));

    println!("{}", result);

    Ok(())
}

fn explore_angle(forest: &Vec<Vec<char>>, right: usize, down: usize) -> i64 {
    let mut current = (0, 0);
    let mut count = 0;

    for line in forest.iter().step_by(down) {
        if line[current.1] == '#' {
            count += 1;
        }

        current = (
            current.0 + down,
            (current.1 + right) % forest[current.0].len(),
        )
    }

    println!("{}", count);
    count
}
