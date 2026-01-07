use aoc2024;
use std::collections::HashMap;
use std::collections::HashSet;

type AdjList = HashMap<i32, Vec<i32>>;

fn main() 
{
    let input: String = aoc2024::read_input(5);
    let (rules, mut updates) = parse_input(&input); // rules is a vector of (first, second) pages

    println!("answer to part 1 = {}", part1(&rules, &updates));
    println!("answer to part 2 = {}", part2(&rules, &mut updates));
}

fn parse_input(input: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>)
{
    let mut rules: Vec<(i32, i32)> = Vec::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();

    let mut iter = input.lines();

    while let Some(rule) = iter.next() 
    {
        if rule.is_empty() { break; }

        let (a, b) = rule.split_once('|').unwrap();
        rules.push((a.parse().unwrap(), b.parse().unwrap()));
    }

    while let Some(update) = iter.next() 
    {
        let update: Vec<i32> = update.split(',').map(|c| c.parse::<i32>().unwrap()).collect();
        updates.push(update);
    }

    (rules, updates)
}

fn process_update_1(update: &[i32], pre: &AdjList) -> bool
{
    let mut restricted: HashSet<i32> = HashSet::new();

    for page in update
    {
        if restricted.contains(page) { return false; }

        if let Some(prereqs) = pre.get(&page)
        {
            for prereq in prereqs
            {
                restricted.insert(*prereq);
            }
        }
    }
    true
}

// rules is a vector of (first, second) pages
fn part1(rules: &[(i32, i32)], updates: &[Vec<i32>]) -> i32 
{
    let mut res = 0;

    // make adj list
    let mut pre: AdjList = HashMap::new();
    for (u, v) in rules
    {
        pre.entry(*v).or_default().push(*u);
    }

    // process updates
    for update in updates
    {
        if !process_update_1(update, &pre) { continue; }
        res += update[update.len()/2];
    }

    res
}

// fixes an index of an update, returns true if a page was moved
fn fix_page(ind: usize, update: &mut [i32], to_index: &mut HashMap<i32, usize>, dep: &AdjList) -> bool
{
    let mut page_fixed: bool = false;
    to_index.insert(update[ind], ind);

    // loop until the index has been swapped with a page that doesn't have any earlier dependencies
    loop {
        let page = update[ind];

        if let Some(dep_pages) = dep.get(&page)
        {
            if let Some(earliest_dep_ind) = dep_pages
                .iter()
                    .filter_map(|dep_page| to_index.get(dep_page).copied())
                    .min()
            {
                page_fixed = true;
                let earliest_dep_page: i32 = update[earliest_dep_ind];

                update.swap(ind, earliest_dep_ind);
                to_index.insert(page, earliest_dep_ind);
                to_index.insert(earliest_dep_page, ind);
            }
            else { return page_fixed; }
        }
        else { return page_fixed; }
    }
}


// processes update, returns true if the update was correctly ordered
fn process_update_2(update: &mut Vec<i32>, dep: &AdjList) -> bool
{
    let mut is_ordered: bool = true;
    let mut to_index: HashMap<i32, usize> = HashMap::new();

    for i in 0..update.len()
    {
        is_ordered &= !fix_page(i, update, &mut to_index, dep);
    }
    is_ordered
}

fn part2(rules: &[(i32, i32)], updates: &mut [Vec<i32>]) -> i32 
{
    let mut res = 0;

    // make adj list
    let mut dep: AdjList = HashMap::new();
    for (u, v) in rules
    {
        dep.entry(*u).or_default().push(*v);
    }

    // process updates
    for update in updates
    {
        if process_update_2(update, &dep) { continue; }
        res += update[update.len()/2];
    }

    res
}
