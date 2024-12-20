use std::collections::{HashMap, HashSet};

use aoc_util::Parse;

fn main() {
    let input = include_str!("../input/05.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn parse_input(input: &str) -> (HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>) {
    let (first, second) = input.split_once("\n\n").unwrap();

    let mut rules = HashMap::new();
    for line in first.lines() {
        let (a, b) = line.pair("|");
        rules.entry(a).or_insert_with(HashSet::new).insert(b);
    }

    let mut orderings = Vec::<Vec<i32>>::new();
    for line in second.lines() {
        orderings.push(line.list(","));
    }
    (rules, orderings)
}

fn is_valid_ordering(ordering: &[i32], rules: &HashMap<i32, HashSet<i32>>) -> bool {
    for (i, page) in ordering.iter().enumerate() {
        if let Some(afters) = rules.get(page) {
            if afters.iter().any(|a| ordering[..i].contains(a)) {
                return false;
            }
        }
    }
    true
}

fn make_ordering_valid(ordering: &mut [i32], rules: &HashMap<i32, HashSet<i32>>) -> bool {
    if is_valid_ordering(ordering, rules) {
        return false;
    }

    let sort_key = ordering
        .iter()
        .map(|x| {
            (
                *x,
                rules.get(x).map_or(0, |afters| {
                    ordering.iter().filter(|y| afters.contains(y)).count()
                }),
            )
        })
        .collect::<HashMap<i32, usize>>();
    ordering.sort_by_key(|x| sort_key[x]);
    true
}

fn part_1(input: &str) -> i32 {
    let (rules, orderings) = parse_input(input);

    let mut sum = 0;
    for ordering in orderings {
        if is_valid_ordering(&ordering, &rules) {
            sum += ordering[ordering.len() / 2];
        }
    }
    sum
}

fn part_2(input: &str) -> i32 {
    let (rules, orderings) = parse_input(input);

    let mut sum = 0;
    for mut ordering in orderings {
        if make_ordering_valid(&mut ordering, &rules) {
            sum += ordering[ordering.len() / 2];
        }
    }
    sum
}

#[test]
fn test_part_1() {
    let data = r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
    assert_eq!(part_1(data), 143);
}

#[test]
fn test_part_2() {
    let data = r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
    assert_eq!(part_2(data), 123);
}
