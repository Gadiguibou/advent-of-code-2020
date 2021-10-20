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
        println!("{}, {}", p_start_value, p_end_value);
        if sum == 2020 {
            println!("{} + {} = 2020", p_start_value, p_end_value);
            break p_start_value * p_end_value;
        } else if sum > 2020 {
            p_end_value = p_end
                .next()
                .expect("no more values to consume in reversed iterator");
        } else {
            p_start_value = p_start
                .next()
                .expect("no more values to consume in iterator");
        }
    };

    println!("{}", result);

    Ok(())
}
