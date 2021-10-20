use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() -> io::Result<()> {
    let f = File::open("../input")?;
    let lines = BufReader::new(f).lines().filter_map(|l| l.ok());

    let max = lines
        .map(|line| {
            let boarding_pass = line.split_at(7);
            let row_id: Vec<bool> = boarding_pass
                .0
                .chars()
                .map(|c| match c {
                    'F' => false,
                    'B' => true,
                    _ => panic!("Found unexpected character in row id."),
                })
                .collect();
            let column_id: Vec<bool> = boarding_pass
                .1
                .chars()
                .map(|c| match c {
                    'L' => false,
                    'R' => true,
                    _ => panic!("Found unexpected character in column id."),
                })
                .collect();

            let row = parse_binary(&row_id);
            let column = parse_binary(&column_id);
            let seat_id = row * 8 + column;

            println!("row {}, column {}, seat ID {}", row, column, seat_id);

            seat_id
        })
        .max()
        .unwrap();

    println!("{}", max);

    Ok(())
}

fn parse_binary(bits: &[bool]) -> u32 {
    bits.iter().fold(0, |acc, b| (acc << 1) | *b as u32)
}
