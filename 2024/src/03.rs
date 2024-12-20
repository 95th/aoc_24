use aoc_util::Parse;

fn main() {
    let input = include_str!("../input/03.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn parse_input(input: &str) -> Vec<[i32; 2]> {
    input.parse_regex(r"mul\((\d{1,3}),(\d{1,3})\)", |x| x)
}

fn part_1(input: &str) -> i32 {
    let mut sum = 0;
    for [a, b] in parse_input(input) {
        sum += a * b;
    }
    sum
}

fn part_2(input: &str) -> i32 {
    let mut sum = 0;

    let mut updated_input = String::from("do()");
    updated_input.push_str(input);

    for sub_input in updated_input.split("don't()").filter_map(|part| {
        let (_a, b) = part.split_once("do()")?;
        Some(b)
    }) {
        for [a, b] in parse_input(sub_input) {
            sum += a * b;
        }
    }
    sum
}

#[test]
fn test_part_1() {
    let data = r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    assert_eq!(part_1(data), 161);
}

#[test]
fn test_part_2() {
    let data = r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    assert_eq!(part_2(data), 48);
}
