use std::{
    collections::{HashMap, HashSet},
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

    let valid_bag_colors = find_valid_bags(&rules, "shiny gold");

    println!("({}, {:?})", valid_bag_colors.len(), valid_bag_colors);

    Ok(())
}

fn find_valid_bags<'a>(
    rules: &'a HashMap<String, HashMap<String, usize>>,
    color: &'a str,
) -> HashSet<&'a str> {
    let mut possible_bag_colors = rules.keys().map(|s| &*s as &str).collect::<Vec<&str>>();
    let mut looking_for = Vec::new();
    let mut valid_bag_colors = HashSet::new();
    looking_for.push(color);
    loop {
        // println!("possible_bag_colors: {:?}", possible_bag_colors);
        // println!("looking_for: {:?}", looking_for);
        // println!("valid_bag_colors: {:?}", valid_bag_colors);

        let mut next_colors_to_look_for: Vec<&str> = Vec::new();
        looking_for.iter().for_each(|c| {
            if let Some(i) = possible_bag_colors.iter().position(|x| x == c) {
                possible_bag_colors.swap_remove(i);
            }

            possible_bag_colors.iter().for_each(|p| {
                if rules[*p].contains_key(*c) && !next_colors_to_look_for.contains(p) {
                    next_colors_to_look_for.push(p);
                }
            });
        });

        looking_for.iter().for_each(|&c| {
            valid_bag_colors.insert(c);
        });
        looking_for.clear();

        if next_colors_to_look_for.is_empty() {
            break;
        }

        next_colors_to_look_for
            .iter()
            .for_each(|c| looking_for.push(c));
    }

    valid_bag_colors.remove(color);

    valid_bag_colors
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
                    .expect("could not split mutltiplicity in entry")
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
