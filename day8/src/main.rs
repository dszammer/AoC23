use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug)]
struct Direction {
    key: String,
    left: String,
    right: String,
}

fn main() {
    println!("Part 1: {}", part1(common::read_lines("./input1.txt")));
    println!("Part 2: {}", part2(common::read_lines("./input2.txt")));
}

fn part1(mut input: Vec<String>) -> u64 {
    let mut current_key = "AAA".to_string();
    let mut step: usize = 0;

    let instructions = input[0].chars().collect::<Vec<char>>();

    input.remove(0);
    input.remove(0);

    let directions = get_directions(input);

    while current_key != "ZZZ" {
        let instruction = instructions[step % instructions.len() as usize];
        let direction = directions.get(&current_key).unwrap();

        if instruction == 'R' {
            current_key = direction.right.to_string();
        } else {
            current_key = direction.left.to_string();
        }

        step += 1;
    }

    step as u64
}

fn part2(mut input: Vec<String>) -> u64 {
    let mut current_keys = Vec::<String>::new();
    let mut step = Vec::<u64>::new();

    let instructions = input[0].chars().collect::<Vec<char>>();

    input.remove(0);
    input.remove(0);

    let directions = get_directions(input);

    directions.iter().for_each(|(key, _)| {
        if key.ends_with("A") {
            current_keys.push(key.to_string());
            step.push(0);
        }
    });

    for (i, key) in current_keys.iter_mut().enumerate() {
        while !key.ends_with("Z") {
            let instruction = instructions[step[i] as usize % instructions.len() as usize];
            let direction = directions.get(key).unwrap();

            if instruction == 'R' {
                *key = direction.right.to_string();
            } else {
                *key = direction.left.to_string();
            }

            step[i] += 1;
        }
    }

    lcm(step)
}

fn lcm(steps: Vec<u64>) -> u64 {
    let mut result = steps[0];
    for i in 1..steps.len() {
        result = result * steps[i] / gcd(result, steps[i]);
    }
    result
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    if a == b {
        return a;
    }
    if b > a {
        let temp = a;
        a = b;
        b = temp;
    }
    while b > 0 {
        let temp = a;
        a = b;
        b = temp % b;
    }
    return a;
}

fn get_direction(line: String) -> Direction {
    let line_clean = line.replace("(", "").replace(")", "").replace(" ", "");
    let parts: Vec<&str> = line_clean.split("=").collect();
    let key = parts[0].to_string();
    let value = parts[1].to_string();
    let parts: Vec<&str> = value.split(",").collect();
    let left = parts[0].to_string();
    let right = parts[1].to_string();

    Direction { key, left, right }
}

fn get_directions(mut input: Vec<String>) -> HashMap<String, Direction> {
    let mut directions: HashMap<String, Direction> = HashMap::new();

    input.iter_mut().for_each(|line| {
        let direction = get_direction(line.to_string());
        directions.insert(direction.key.to_string(), direction);
    });

    directions
}

#[cfg(test)]
mod tests {

    fn get_part1_test_input1() -> Vec<String> {
        vec![
            "RL".to_string(),
            "".to_string(),
            "AAA = (BBB, CCC)".to_string(),
            "BBB = (DDD, EEE)".to_string(),
            "CCC = (ZZZ, GGG)".to_string(),
            "DDD = (DDD, DDD)".to_string(),
            "EEE = (EEE, EEE)".to_string(),
            "GGG = (GGG, GGG)".to_string(),
            "ZZZ = (ZZZ, ZZZ)".to_string(),
        ]
    }

    fn get_part2_test_input() -> Vec<String> {
        vec![
            "LR".to_string(),
            "".to_string(),
            "11A = (11B, XXX)".to_string(),
            "11B = (XXX, 11Z)".to_string(),
            "11Z = (11B, XXX)".to_string(),
            "22A = (22B, XXX)".to_string(),
            "22B = (22C, 22C)".to_string(),
            "22C = (22Z, 22Z)".to_string(),
            "22Z = (22B, 22B)".to_string(),
            "XXX = (XXX, XXX)".to_string(),
        ]
    }

    fn get_part1_test_input1_clean() -> Vec<String> {
        let mut input = get_part1_test_input1();
        input.remove(0);
        input.remove(0);
        input
    }

    fn get_part1_test_input2() -> Vec<String> {
        vec![
            "LLR".to_string(),
            "".to_string(),
            "AAA = (BBB, BBB)".to_string(),
            "BBB = (AAA, ZZZ)".to_string(),
            "ZZZ = (ZZZ, ZZZ)".to_string(),
        ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(get_part1_test_input1()), 2);
        assert_eq!(super::part1(get_part1_test_input2()), 6);
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(get_part2_test_input()), 6);
    }

    #[test]
    fn test_get_direction() {
        let line = "AAA = (BBB, CCC)".to_string();
        let direction = super::get_direction(line);

        println!("{:?}", direction);

        assert_eq!(direction.key, "AAA");
        assert_eq!(direction.left, "BBB");
        assert_eq!(direction.right, "CCC");
    }

    #[test]
    fn test_get_directions() {
        let mut input = get_part1_test_input1_clean();

        let directions = super::get_directions(input);

        let direction = directions.get("AAA").unwrap();

        println!("{:?}", direction);

        assert_eq!(direction.key, "AAA");
        assert_eq!(direction.left, "BBB");
        assert_eq!(direction.right, "CCC");
    }
}
