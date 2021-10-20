use std::{collections::HashSet, fs::File, io::{self, BufRead, BufReader}};

fn main() -> io::Result<()> {
    let f = File::open("../input")?;
    let lines = BufReader::new(f).lines().filter_map(|l| l.ok());

    let ids = lines
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
        .collect::<HashSet<u32>>();

    let max = ids.iter().max().unwrap();
    let min = ids.iter().min().unwrap();

    let possible_seats = (*min..*max + 1).collect::<HashSet<u32>>();

    let available_seats = ids.symmetric_difference(&possible_seats).collect::<Vec<&u32>>();

    assert_eq!(available_seats.len(), 1);

    println!("{}", available_seats[0]);

    Ok(())
}

fn parse_binary(bits: &[bool]) -> u32 {
    bits.iter().fold(0, |acc, b| (acc << 1) | *b as u32)
}
