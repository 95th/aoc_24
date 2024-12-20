use std::collections::HashMap;

use aoc_util::Grid;

fn main() {
    let input = include_str!("../input/08.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> u32 {
    let grid = Grid::from_bytes(input);
    let mut map = HashMap::new();

    for point in grid.points() {
        if grid[point] != b'.' {
            map.entry(grid[point]).or_insert_with(Vec::new).push(point);
        }
    }

    let mut antinodes = grid.with_fill(false);
    let mut count = 0;

    for points in map.values() {
        for i in 0..points.len() {
            let a = points[i];
            for &b in points.iter().skip(i + 1) {
                let dist = b - a;
                let before = a - dist;
                let after = b + dist;

                if let Some(false) = antinodes.replace(before, true) {
                    count += 1;
                }
                if let Some(false) = antinodes.replace(after, true) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn part_2(input: &str) -> u32 {
    let grid = Grid::from_bytes(input);

    let mut map = HashMap::new();

    for point in grid.points() {
        if grid[point] != b'.' {
            map.entry(grid[point]).or_insert_with(Vec::new).push(point);
        }
    }

    let mut antinodes = grid.with_fill(1);
    let mut count = 0;

    for points in map.values() {
        for i in 0..points.len() {
            let a = points[i];
            for &b in points.iter().skip(i + 1) {
                let dist = b - a;

                let mut before = a;
                while let Some(value) = antinodes.replace(before, 0) {
                    before -= dist;
                    count += value;
                }

                let mut after = b;
                while let Some(value) = antinodes.replace(after, 0) {
                    after += dist;
                    count += value;
                }
            }
        }
    }

    count
}

#[test]
fn test_part_1() {
    let data = r"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
    assert_eq!(part_1(data), 14);
}

#[test]
fn test_part_2() {
    let data = r"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
    assert_eq!(part_2(data), 34);
}
