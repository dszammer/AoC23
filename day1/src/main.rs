use std::fs::read_to_string;

fn main() {
    let input1 = read_lines("./part1.txt");

    // for line in input.iter() {
    //     println!("{}", line);
    // }

    println!("Part 1: {}", part1(input1));

    let input2 = read_lines("./part2.txt");

    println!("Part 2: {}", part2(input2));
}
fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn part1(input: Vec<String>) -> u32 {
    let mut sum = 0;
    for line in input {
        sum += line
            .chars()
            .find(|c| c.is_ascii_digit())
            .map_or(0, |d| d.to_digit(10).unwrap_or(0))
            * 10;
        sum += line
            .chars()
            .rev()
            .find(|c| c.is_ascii_digit())
            .map_or(0, |d| d.to_digit(10).unwrap_or(0));
    }
    sum
}

fn part2(mut input: Vec<String>) -> u32 {
    for line in input.iter_mut() {
        *line = line.replace("one", "o1e");
        *line = line.replace("two", "t2o");
        *line = line.replace("three", "t3e");
        *line = line.replace("four", "f4r");
        *line = line.replace("five", "f5e");
        *line = line.replace("six", "s6x");
        *line = line.replace("seven", "s7n");
        *line = line.replace("eight", "e8t");
        *line = line.replace("nine", "n9e");
    }

    // for line in input.iter() {
    //     println!("{}", line);
    // }

    part1(input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {
        let input: Vec<String> = vec![
            "1abc2".to_string(),
            "pqr3stu8vwx".to_string(),
            "a1b2c3d4e5f".to_string(),
            "treb7uchet".to_string(),
        ];
        assert_eq!(super::part1(input), 142);
    }

    #[test]
    fn test_part2() {
        let input: Vec<String> = vec![
            "two1nine".to_string(),
            "eightwothree".to_string(),
            "abcone2threexyz".to_string(),
            "xtwone3four".to_string(),
            "4nineeightseven2".to_string(),
            "zoneight234".to_string(),
            "7pqrstsixteen".to_string(),
        ];
        assert_eq!(super::part2(input), 281);
    }
}
