use std::fmt::Debug;

fn main() {
    println!("Part 1: {}", part1(common::read_lines("./input1.txt")));
    println!("Part 2: {}", part2(common::read_lines("./input2.txt")));
}

fn part1(mut input: Vec<String>) -> u64 {
    0
}

fn part2(mut input: Vec<String>) -> u64 {
    0
}

#[cfg(test)]
mod tests {

    fn get_test_input() -> Vec<String> {
        vec![
            "0 3 6 9 12 15".to_string(),
            "1 3 6 10 15 21".to_string(),
            "10 13 16 21 30 45".to_string(),
        ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(get_test_input()), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(get_test_input()), 0);
    }
}
