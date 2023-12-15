use std::fs::read_to_string;

fn main() {
    let input = read_lines("./part1.txt");

    println!("Part 1: {}", part1(&input));

    println!("Part 2: {}", part2(&input));
}
fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn part1(input: &Vec<String>) -> u32 {
    let mut sum = 0;

    sum
}

fn part2(input: &Vec<String>) -> u32 {
    let mut sum = 0;

    sum
}
