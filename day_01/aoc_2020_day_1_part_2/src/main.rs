use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> io::Result<()> {
    let f = File::open("../input")?;
    let reader = BufReader::new(f);
    let mut expenses: Vec<i32> = reader
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect();

    expenses.sort();

    'entries: for entry in &expenses {
        let target = 2020 - entry;

        // iterate over entries using two pointers
        let mut p_start = expenses.iter();

        // println!("p_start");
        // for p_start_value in p_start {
        //     println!("{}", p_start_value);
        // }

        let mut p_end = expenses.iter().rev();

        // println!("p_end");
        // for p_end_value in p_end {
        //     println!("{}", p_end_value);
        // }

        let mut p_start_value = p_start.next().expect("no values in iterator");
        let mut p_end_value = p_end.next().expect("no values in reversed iterator");

        let result = loop {
            let sum = p_start_value + p_end_value;
            // println!("{}, {}", p_start_value, p_end_value);
            if sum == target {
                println!("{} + {} + {} = 2020", p_start_value, p_end_value, entry);
                break p_start_value * p_end_value * entry;
            } else if sum > target {
                p_end_value = match p_end.next() {
                    Some(value) => value,
                    None => continue 'entries,
                }
            } else {
                p_start_value = match p_start.next() {
                    Some(value) => value,
                    None => continue 'entries,
                }
            }
        };

        println!("{}", result);
    }

    Ok(())
}
