use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader},
};

use regex::{Captures, Regex};

fn main() -> io::Result<()> {
    let f = File::open("../input")?;
    let lines = BufReader::new(f).lines().filter_map(|r| r.ok());
    let rule_re = Regex::new(r"^(?P<bag_color>.+) bags contain (?P<contents>.+).$")
        .expect("rules regex did not compile");
    let mut rules = HashMap::new();

    lines.for_each(|l| match rule_re.captures(&l) {
        Some(c) => parse_rule(c, &mut rules),
        None => panic!("Line: \"{}\" did not match rule regex", l),
    });

    let number_of_bags = number_of_needed_bags(&rules, "shiny gold") - 1;

    println!("{}", number_of_bags);

    Ok(())
}

fn number_of_needed_bags(rules: &HashMap<String, HashMap<String, usize>>, color: &str) -> u64 {
    if rules[color].is_empty() {
        return 1u64;
    }

    rules[color]
        .iter()
        .map(|(color, &m)| -> u64 { m as u64 * number_of_needed_bags(rules, color) })
        .sum::<u64>()
        + 1
}

fn parse_rule(c: Captures, rules: &mut HashMap<String, HashMap<String, usize>>) {
    if &c[2] == "no other bags" {
        rules.insert((&c[1]).to_string(), HashMap::with_capacity(0));
        return;
    }

    rules.insert(
        (&c[1]).to_string(),
        c[2].split(", ")
            .map(|i| {
                match i
                    .split_once(" ")
                    .expect("could not split multiplicity in entry")
                {
                    ("1", rest) => (
                        rest.strip_suffix(" bag")
                            .expect("could not strip suffix")
                            .to_string(),
                        1,
                    ),
                    (n, rest) => (
                        rest.strip_suffix(" bags")
                            .expect("could not strip suffix")
                            .to_string(),
                        n.parse::<usize>().expect("could not parse multiplicity"),
                    ),
                }
            })
            .collect::<HashMap<String, usize>>(),
    );
}
