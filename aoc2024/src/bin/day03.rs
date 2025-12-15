use aoc2024;
use itertools::Either;

fn main() 
{
    let input = aoc2024::read_input(3);
    let l: Vec<Vec<i32>> = parse_input(&input);

    println!("answer to part 1 = {}", part1(&l));
    println!("answer to part 2 = {}", part2(&l));
}

fn parse_input(input: &str) -> Vec<Vec<i32>> 
{
    let mut l: Vec<Vec<i32>> = Vec::new();

    for (i, line) in input.lines().enumerate() 
    {
        l.push(Vec::new());

        for val in line.split_whitespace() 
        {
            l[i].push(val.parse().unwrap());
        }
    }

    l
}

fn check_vec(vec: &[i32]) -> bool 
{
    let (Some(first), Some(last)) = (vec.first(), vec.last()) else { return false };
    let pairs = match first.cmp(last) 
    {
        std::cmp::Ordering::Less    => Either::Left(vec.iter().zip(vec.iter().skip(1))),
        std::cmp::Ordering::Greater => Either::Right(vec.iter().rev().zip(vec.iter().rev().skip(1))),
        std::cmp::Ordering::Equal   => return false
    };

    for (&cur, &next) in pairs
    {
        if next < cur + 1 { return false }
        if next > cur + 3 { return false }
    }

    true
}

fn part1(l: &[Vec<i32>]) -> i32 
{
    let mut count = 0;

    l.iter().for_each(|vec| {
        if check_vec(vec) { count += 1; }
    });

    count
}

fn part2(l: &[Vec<i32>]) -> i32
{
    let mut count = 0;

    'outer: for vec in l
    {   
        for i in 0..vec.len()
        {
            let mut vec_clone = vec.clone();
            vec_clone.remove(i);

            if check_vec(&vec_clone) 
            {
                count += 1;
                continue 'outer;
            }
        }
    };

    count

}
