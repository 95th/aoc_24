fn parse_input(input: &str) -> impl Iterator<Item = Vec<i32>> + '_ {
    input.lines().map(|line| {
        line.split_ascii_whitespace()
            .map(|x| x.parse().unwrap())
            .collect()
    })
}

fn is_safe(levels: &[i32]) -> bool {
    let mut increasing = 0;
    let mut decreasing = 0;
    for i in 1..levels.len() {
        let diff = levels[i] - levels[i - 1];
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }
        if diff > 0 {
            increasing += 1;
        } else if diff < 0 {
            decreasing += 1;
        }
    }
    increasing == levels.len() - 1 || decreasing == levels.len() - 1
}

fn is_almost_safe(levels: &[i32]) -> bool {
    if is_safe(levels) {
        return true;
    }

    for i in 0..levels.len() {
        let mut new_levels = levels.to_vec();
        new_levels.remove(i);
        if is_safe(&new_levels) {
            return true;
        }
    }

    false
}

pub fn part_1(input: &str) -> usize {
    let mut safe = 0;
    for levels in parse_input(input) {
        if is_safe(&levels) {
            safe += 1;
        }
    }
    safe
}

pub fn part_2(input: &str) -> usize {
    let mut safe = 0;
    for levels in parse_input(input) {
        if is_almost_safe(&levels) {
            safe += 1;
        }
    }
    safe
}

#[test]
fn run_part_1() {
    let input = include_str!("input.txt");
    println!("Result: {}", part_1(input));
}

#[test]
fn run_part_2() {
    let input = include_str!("input.txt");
    println!("Result: {}", part_2(input));
}

#[test]
fn test_part_1() {
    let data = r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    assert_eq!(part_1(data), 2);
}

#[test]
fn test_part_2() {
    let data = r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    assert_eq!(part_2(data), 4);
}

#[test]
fn test_part_2_1() {
    let data = r"1 3 2 4 50";
    assert_eq!(part_2(data), 0);
}

#[test]
fn test_part_2_2() {
    let data = r"10 1 2 4 7";
    assert_eq!(part_2(data), 1);
}

#[test]
fn test_part_2_3() {
    let data = r"4 10 9 8 7";
    assert_eq!(part_2(data), 1);
}

#[test]
fn test_part_2_4() {
    let data = r"4 4 5 6 7";
    assert_eq!(part_2(data), 1);
}

#[test]
fn test_part_2_5() {
    let data = r"4 4 3 2 1";
    assert_eq!(part_2(data), 1);
}

#[test]
fn test_part_2_6() {
    let data = r"1 0 4 5 6";
    assert_eq!(part_2(data), 1);
}
