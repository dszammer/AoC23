use std::fmt::Debug;

use common;

fn main() {
    println!("Part 1: {}", part1(common::read_lines("./input1.txt")));
    println!("Part 2: {}", part2(common::read_lines("./input2.txt")));
}

fn part1(input: Vec<String>) -> i64 {
    let input_clean = input
        .iter()
        .map(|line| line.split_whitespace().skip(1).collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let mut result: i64 = 1;

    input_clean[0]
        .iter()
        .zip(input_clean[1].iter())
        .for_each(|(time, distance)| {
            result *= calc_winning(
                time.parse::<u64>().unwrap(),
                distance.parse::<u64>().unwrap(),
            );
        });

    result
}

fn part2(input: Vec<String>) -> i64 {
    let input_clean = input
        .iter()
        .map(|line| line.replace(" ", ""))
        .collect::<Vec<String>>();

    let input_split = input_clean
        .iter()
        .map(|line| line.split(":").skip(1).collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    calc_winning(
        input_split[0][0].parse::<u64>().unwrap(),
        input_split[1][0].parse::<u64>().unwrap(),
    )
}

fn calc_winning(time: u64, distance: u64) -> i64 {
    let isqrt: f64 = (((time.pow(2) as f64) / 4.0) - distance as f64).sqrt();
    let winning_lower: i64 = ((time as f64) / 2.0 - isqrt).ceil() as i64;
    let winning_higher: i64 = ((time as f64) / 2.0 + isqrt).floor() as i64;

    winning_higher - winning_lower + 1
}

#[cfg(test)]
mod tests {

    fn get_part1_test_input() -> Vec<String> {
        vec![
            "Time:      7  15   30".to_string(),
            "Distance:  9  40  200".to_string(),
        ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(get_part1_test_input()), 352);
    }

    #[test]
    fn test_calc_winning() {
        assert_eq!(super::calc_winning(7, 9), 4);
        assert_eq!(super::calc_winning(15, 40), 8);
        assert_eq!(super::calc_winning(30, 200), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(get_part1_test_input()), 71503);
    }
}
