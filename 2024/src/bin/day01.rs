use aoc2024;
use std::collections::HashMap;

fn main() {
    let input = aoc2024::read_input(1);
    let (mut l1, mut l2) = parse_input(&input);

    println!("answer to part 1 = {}", part1(&mut l1, &mut l2));
    println!("answer to part 2 = {}", part2(&l1, &l2));
}

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut l1: Vec<i32> = Vec::new();
    let mut l2: Vec<i32> = Vec::new();

    let mut iter = input.split_whitespace();
    while let (Some(a), Some(b)) = (iter.next(), iter.next()) {
        l1.push(a.parse().unwrap());
        l2.push(b.parse().unwrap());
    }
    (l1, l2)
}

fn part1(l1: &mut Vec<i32>, l2: &mut Vec<i32>) -> i32 {
    l1.sort();
    l2.sort();

    l1.iter()
        .zip(l2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

fn part2(l1: & Vec<i32>, l2: & Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    for elem in l2 {
        *map.entry(elem).or_insert(0) += 1;
    }

    l1.iter()
        .map(|a| map.get(a).unwrap_or(&0)*a)
        .sum()
}
