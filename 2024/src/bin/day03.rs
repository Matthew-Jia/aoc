use aoc2024;
use std::cmp;

fn main() 
{
    let input = aoc2024::read_input(3);

    println!("answer to part 1 = {}", part1(&input));
    println!("answer to part 2 = {}", part2(&input));
}

fn part1(l: &str) -> i32 
{
    let mut res = 0;
    let max_substr_len = 3 + 2 + 3 + 1 + 3; // 3 for mut, 2 for (), 3 for first int, 1 for
                                                // comma, 3 for second int

    for idx in 0..l.len()-7
    {
        let substr_len = cmp::min(max_substr_len, l.len()-idx); // get the length of the maximum possible
                                                                // mul substr starting at idx

        let mul = &l[idx..idx+substr_len];
        
        let nums: Vec<i32> = mul
            .split(|c: char| !c.is_ascii_digit())
            .filter(|t| !t.is_empty())
            .map(|t| t.parse().unwrap())
            .collect();

        if nums.len() < 2 { continue }

        let final_string = format!("mul({},{})", nums[0], nums[1]);
        if mul[0..final_string.len()] == final_string { res += nums[0] * nums[1]; }
    }
    
    res
}

fn part2(l: &str) -> i32
{
    let mut res = 0;
    let max_substr_len = 3 + 2 + 3 + 1 + 3; // 3 for mut, 2 for (), 3 for first int, 1 for
                                                // comma, 3 for second int
    
    let mut do_mul: bool = true;
    let do_len = 4;
    let dont_len = 7;

    for idx in 0..l.len()-7
    {
        let substr_len = cmp::min(max_substr_len, l.len()-idx); // get the length of the maximum possible
                                                                // mul substr starting at idx

        let s = &l[idx..idx+substr_len];

        if s[0..do_len] == *"do()"       { do_mul = true; continue; }
        if s[0..dont_len] == *"don't()"  { do_mul = false; continue; }

        if !do_mul { continue; }
        
        let nums: Vec<i32> = s
            .split(|c: char| !c.is_ascii_digit())
            .filter(|t| !t.is_empty())
            .map(|t| t.parse().unwrap())
            .collect();

        if nums.len() < 2 { continue }

        let final_string = format!("mul({},{})", nums[0], nums[1]);
        if s[0..final_string.len()] == final_string { res += nums[0] * nums[1]; }
    }
    
    res
}

