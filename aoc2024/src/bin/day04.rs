use aoc2024;

fn main() 
{
    let input: String   = aoc2024::read_input(4);
    let lines: Vec<Vec<u8>> = input.lines().map(|l| l.as_bytes().to_vec()).collect();

    println!("answer to part 1 = {}", part1(&lines));
    println!("answer to part 2 = {}", part2(&lines));
}

fn part1(lines: &[Vec<u8>]) -> i32 
{
    let mut res: i32 = 0;

    let mut find_occurences = |start_row: usize, start_col: usize|
    {
        let directions = vec![(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (1, -1), (-1, 1), (-1, -1)];
        let target: Vec<u8> = vec![b'X', b'M', b'A', b'S'];

        'outer: for (dr, dc) in directions.iter()
        {
            let (mut r, mut c) = (start_row as i32, start_col as i32);

            for i in 0..4
            {
                if r < 0 || r >= lines.len() as i32             { continue 'outer; }
                if c < 0 || c >= lines[0].len() as i32          { continue 'outer; }
                if lines[r as usize][c as usize] != target[i]   { continue 'outer; }

                r += dr;
                c += dc;
            }

            res += 1;
        }
    };

    for r in 0..lines.len()
    {
        for c in 0..lines[0].len()
        {
            find_occurences(r, c);
        }
    }

    res
}

fn part2(lines: &[Vec<u8>]) -> i32
{
    let mut res: i32 = 0;

    let mut find_occurences = |start_row: usize, start_col: usize|
    {
        if lines[start_row][start_col] != b'A' { return; }

        let directions = vec![(1, 1), (1, -1)];
        for (dr, dc) in directions.iter()
        {
            let (r1, c1) = (start_row as i32 + dr, start_col as i32 + dc);
            let (r2, c2) = (start_row as i32 - dr, start_col as i32 - dc);

            if r1 < 0 || c1 < 0 { return; }
            if r2 < 0 || c2 < 0 { return; }

            let (r1, c1) = (r1 as usize, c1 as usize);
            let (r2, c2) = (r2 as usize, c2 as usize);

            if r1 >= lines.len() || c1 >= lines[r1].len() { return; }
            if r2 >= lines.len() || c2 >= lines[r2].len() { return; }

            let mut diag: Vec<u8> = vec![lines[r1][c1], lines[r2][c2]];
            diag.sort();

            if diag != b"MS" { return; }
        }
        res += 1;
    };
    
    for r in 1..lines.len()-1
    {
        for c in 1..lines[0].len()-1
        {
            find_occurences(r, c);
        }
    }

    res
}

