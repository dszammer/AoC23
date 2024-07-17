fn main() {
    println!("Part 1: {}", part1(common::read_lines("./input1.txt")));
    println!("Part 2: {}", part2(common::read_lines("./input2.txt")));
}

fn part1(input: Vec<String>) -> u64 {
    let mut result: i64 = 0;

    input.iter().for_each(|line| {
        let numbers = line
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        for (i, number) in numbers.iter().enumerate() {
            result += number
                * comb(numbers.len() as i64, i as i64)
                * (-1_i32).pow(numbers.len() as u32 - 1 - i as u32) as i64;
        }
    });

    result as u64
}

fn part2(input: Vec<String>) -> u64 {
    let mut result: i64 = 0;

    input.iter().for_each(|line| {
        let numbers = line
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        for (i, number) in numbers.iter().enumerate() {
            result +=
                number * comb(numbers.len() as i64, i as i64 + 1) * (-1_i32).pow(i as u32) as i64;
        }
    });

    result as u64
}

pub fn comb(n: i64, k: i64) -> i64 {
    if k > n {
        0
    } else {
        (1..=k).fold(1, |acc, val| acc * (n - val + 1) / val)
    }
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
        assert_eq!(super::part1(get_test_input()), 114);
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(get_test_input()), 2);
    }

    #[test]
    fn test_comb() {
        assert_eq!(super::comb(5, 2), 10);
        assert_eq!(super::comb(5, 3), 10);
        assert_eq!(super::comb(1, 1), 1);
        assert_eq!(super::comb(0, 0), 1);
        assert_eq!(super::comb(10, 1), 10);
        assert_eq!(super::comb(10, 2), 45);
        assert_eq!(super::comb(10, 3), 120);
    }
}
