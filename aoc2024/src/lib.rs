pub fn read_input(day: u32) -> String {
    let filename = format!("inputs/day{:02}.txt", day);
    std::fs::read_to_string(filename).expect("failed to read input")
}

