use std::collections::{BTreeSet, HashMap, HashSet};

fn main() {
    let input = include_str!("../../input");
    let mut sections = input.split("\n\n");

    let rules = parse_rules(sections.next().expect("No rules section found"));

    let my_ticket = parse_ticket(
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

    let valid_tickets = nearby_tickets
        .filter(|ticket| ticket.iter().all(|num| number_meets_any_rule(*num, &rules)))
        .collect::<Vec<_>>();

    let solution = find_valid_field_order(&rules, &valid_tickets);

    println!("{:?}", solution);

    let solution = solution.expect("Could not find a valid solution");

    // Validate that solution is correct for all tickets -> Used because I initially made an off by one error...
    // assert_eq!(solution.len(), rules.len());

    // for ticket in valid_tickets {
    //     for (index, field) in ticket.iter().enumerate() {
    //         let rule_for_field = rules.iter().find(|rule| rule.0 == solution[index]).unwrap();
    //         assert!(number_meets_rule(*field, rule_for_field));
    //     }
    // }

    let indices_of_fields_starting_with_departure = solution
        .iter()
        .enumerate()
        .filter(|(_, field)| field.starts_with("departure"))
        .map(|(i, _)| i)
        .collect::<HashSet<_>>();

    let final_result: u64 = my_ticket
        .iter()
        .enumerate()
        .filter(|(i, _)| indices_of_fields_starting_with_departure.contains(i))
        .map(|(_, &num)| num as u64)
        .product();

    println!("{}", final_result);
}

type Rule = (&'static str, (u16, u16), (u16, u16));

fn parse_rules(rules_lines: &'static str) -> Vec<Rule> {
    let mut rules = vec![];

    for l in rules_lines.lines() {
        let (section_name, rule) = l
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

        rules.push((section_name, ranges[0], ranges[1]));
    }

    rules
}

fn parse_ticket(ticket_line: &str) -> Vec<u16> {
    ticket_line.split(',').map(|s| s.parse().unwrap()).collect()
}

const fn number_meets_rule(num: u16, rule: &Rule) -> bool {
    (num >= rule.1 .0 && num <= rule.1 .1) || (num >= rule.2 .0 && num <= rule.2 .1)
}

fn number_meets_any_rule(num: u16, rules: &[Rule]) -> bool {
    rules.iter().any(|rule| number_meets_rule(num, rule))
}

fn test_rule_index_combination(rule: &Rule, index: usize, tickets: &[Vec<u16>]) -> bool {
    tickets.iter().all(|ticket| {
        let num = ticket[index];
        number_meets_rule(num, rule)
    })
}

fn find_valid_field_order(rules: &[Rule], tickets: &[Vec<u16>]) -> Option<Vec<&'static str>> {
    _find_valid_field_order(rules, tickets, 0, BTreeSet::new(), &mut HashMap::new())
}

fn _find_valid_field_order(
    rules: &[Rule],
    tickets: &[Vec<u16>],
    start_index: usize,
    solutions_consumed: BTreeSet<&'static str>,
    memoized_solutions: &mut HashMap<BTreeSet<&str>, Option<Vec<&'static str>>>,
) -> Option<Vec<&'static str>> {
    if start_index == rules.len() {
        return Some(vec![]);
    }

    if let Some(solution) = memoized_solutions.get(&solutions_consumed) {
        return solution.clone();
    }

    for rule in rules.iter().rev() {
        if solutions_consumed.contains(&rule.0) {
            continue;
        }

        let is_possible = test_rule_index_combination(rule, start_index, tickets);

        if !is_possible {
            continue;
        }

        let mut next_solutions_consumed = solutions_consumed.clone();
        next_solutions_consumed.insert(rule.0);

        if let Some(smaller_solution) = _find_valid_field_order(
            rules,
            tickets,
            start_index + 1,
            next_solutions_consumed,
            memoized_solutions,
        ) {
            let solution = [rule.0].into_iter().chain(smaller_solution).collect();
            return Some(solution);
        }
    }

    memoized_solutions.insert(solutions_consumed, None);
    None
}
