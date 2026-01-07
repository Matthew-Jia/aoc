use aoc2024;

fn main() 
{
    let input: String = aoc2024::read_input(6);
    let mut grid = parse_input(&input);
    println!("answer to part 1 = {}", part1(&grid));
    println!("answer to part 2 = {}", part2(&mut grid));
}

fn parse_input(input: &str) -> Vec<Vec<u8>>
{
    input.lines().map(|l| l.as_bytes().to_vec()).collect()
}

fn find_start(grid: &[Vec<u8>]) -> (usize, usize, usize) {
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            let sdi = match grid[r][c] {
                b'^' => 0,
                b'>' => 1,
                b'v' => 2,
                b'<' => 3,
                _ => continue,
            };
            return (sdi, r, c);
        }
    }

    unreachable!("grid must contain a start marker");
}



fn traverse_grid(sdi: usize, sr: usize, sc: usize, grid: &[Vec<u8>]) -> (bool, Vec<Vec<u8>>)
{
    const DIR: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)]; // up, right, down, left
    let escaped: bool;
    let mut di = sdi;
    let mut cr = sr;
    let mut cc = sc;
    let mut memo = vec![vec![0u8; grid[0].len()]; grid.len()];

    loop {
        if memo[cr][cc] & 0b1 << di != 0 { escaped = false; break; }

        memo[cr][cc] |= 0b1 << di;

        let nr: i32 = cr as i32 + DIR[di].0;
        let nc: i32 = cc as i32 + DIR[di].1;

        if nr < 0 || nc < 0 { escaped = true; break; }
        
        let nr: usize = nr as usize;
        let nc: usize = nc as usize;

        if nr >= grid.len() || nc >= grid[0].len() { escaped = true; break; }
        
        if grid[nr][nc] == b'#' { di = (di + 1) % 4; }
        else 
        { 
            cr = nr;
            cc = nc;
        }
    }

    (escaped, memo)
}

fn part1(grid: &[Vec<u8>]) -> usize
{
    let (sdi, sr, sc) = find_start(grid);
    let (escaped, memo) = traverse_grid(sdi, sr, sc, grid);
    assert_eq!(escaped, true);
    memo.iter().flatten().filter(|&&m| m & 0b1111 != 0).count()
}

fn part2(grid: &mut [Vec<u8>]) -> usize 
{
    let mut res = 0;
    let (sdi, sr, sc) = find_start(grid);
    let (_, memo) = traverse_grid(sdi, sr, sc, grid);

    for r in 0..memo.len()
    {
        for c in 0..memo[0].len()
        {
            if r == sr && c == sc { continue; }
            
            if memo[r][c] & 0b1111 != 0
            {
                let orig = grid[r][c];
                grid[r][c] = b'#';
                let (escaped, _) = traverse_grid(sdi, sr, sc, grid);
                if !escaped { res += 1; }
                grid[r][c] = orig;
            }
        }
    }
    
    res
}
