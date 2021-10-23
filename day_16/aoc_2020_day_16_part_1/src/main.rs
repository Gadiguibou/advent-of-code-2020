fn main() -> anyhow::Result<()> {
    let input = include_str!("../../input");
    let mut sections = input.split("\n\n");

    let rules = parse_rules(sections.next().expect("No rules section found"));

    let _my_ticket = parse_ticket(
        sections
            .next()
            .expect("No ticket section found")
            .lines()
            .nth(1)
            .expect("Missing \"your ticket:\""),
    );

    let nearby_tickets = sections
        .next()
        .expect("No nearby tickets found")
        .lines()
        .skip(1)
        .map(|l| parse_ticket(l));

    let ticket_scanning_error_rate: u16 = nearby_tickets
        .into_iter()
        .flatten()
        .filter(|num| !number_meets_rules(*num, &rules))
        .sum();

    println!("{}", ticket_scanning_error_rate);

    Ok(())
}

type Rule = ((u16, u16), (u16, u16));

fn parse_rules(rules_lines: &str) -> Vec<Rule> {
    let mut rules = vec![];

    for l in rules_lines.lines() {
        let (_section_name, rule) = l
            .split_once(": ")
            .expect("Rule did not contain separator \": \"");
        let (range_1, range_2) = rule
            .split_once(" or ")
            .expect("Rule did not contain \" or \" separator");

        let ranges = [range_1, range_2].map(|range| {
            let (start, end) = range
                .split_once("-")
                .expect("Range expression did not contain \"-\" separator");
            (start.parse().unwrap(), end.parse().unwrap())
        });

        rules.push((ranges[0], ranges[1]));
    }

    rules
}

fn parse_ticket(ticket_line: &str) -> Vec<u16> {
    ticket_line.split(',').map(|s| s.parse().unwrap()).collect()
}

fn number_meets_rules(num: u16, rules: &[Rule]) -> bool {
    rules.iter().any(|(range_1, range_2)| {
        (num >= range_1.0 && num <= range_1.1) || (num >= range_2.0 && num <= range_2.1)
    })
}
