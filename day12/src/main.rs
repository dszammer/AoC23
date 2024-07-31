use itertools::Itertools;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum State {
    Broken,
    Ok,
    Unknown,
}

fn main() {
    println!("Part 1: {}", part1(common::read_lines("./input.txt")));
    println!("Part 2: {}", part2(common::read_lines("./input.txt")));
}

fn part1(input: Vec<String>) -> u64 {
    input
        .iter()
        .map(|line| {
            let (states, configs) = parse_input(line);
            calculate_arrangements(&states, &configs)
        })
        .sum()
}

fn part2(input: Vec<String>) -> u64 {
    input
        .iter()
        .map(|line| {
            let (states, configs) = unfold_input(line);
            calculate_arrangements(&states, &configs)
        })
        .sum()
}

fn parse_input(line: &String) -> (Vec<State>, Vec<u64>) {
    let mut states = Vec::new();
    let mut configs = Vec::new();
    let mut i = 0;
    let split = line.split(" ").collect::<Vec<&str>>();

    split[0].chars().for_each(|c| {
        states.push(match c {
            '#' => State::Broken,
            '.' => State::Ok,
            '?' => State::Unknown,
            _ => panic!("Invalid state"),
        });
    });

    split[1].split(",").for_each(|c| {
        configs.push(c.parse::<u64>().unwrap());
    });

    (states, configs)
}

fn unfold_input(line: &String) -> (Vec<State>, Vec<u64>) {
    let mut states = Vec::new();
    let mut configs = Vec::new();
    let mut i = 0;
    let split = line.split(" ").collect::<Vec<&str>>();
    for i in 0..5 {
        if i != 0 {
            states.push(State::Unknown);
        }

        split[0].chars().for_each(|c| {
            states.push(match c {
                '#' => State::Broken,
                '.' => State::Ok,
                '?' => State::Unknown,
                _ => panic!("Invalid state"),
            });
        });

        split[1].split(",").for_each(|c| {
            configs.push(c.parse::<u64>().unwrap());
        });
    }

    (states, configs)
}

fn calculate_arrangements(states: &Vec<State>, configs: &Vec<u64>) -> u64 {
    let mut count = 0;

    let total_spring_count = states.len();
    let damaged_sequence_count = configs.len();

    let mut prev_counts = vec![0u64; total_spring_count + 2];
    prev_counts[total_spring_count + 1] = 1;

    // Consider each damaged-length from last to first.
    for (d, damaged_len) in configs.iter().copied().enumerate().rev() {
        let mut possibly_damaged_run_len = 0;

        let mut nways = 0;

        let mut curr_counts = vec![0u64; total_spring_count + 2];

        // Attempt to place a damaged-length at each possible location from end of spring sequence to start.
        for s in (0..total_spring_count).rev() {
            nways = if let Some(State::Broken) = states.get(s + damaged_len as usize) {
                0
            } else {
                match prev_counts.get(s + damaged_len as usize + 1) {
                    Some(ways) => nways + *ways,
                    None => 0,
                }
            };

            curr_counts[s] = match states[s] {
                State::Ok => {
                    possibly_damaged_run_len = 0;
                    0
                }
                State::Unknown | State::Broken => {
                    possibly_damaged_run_len += 1;

                    if possibly_damaged_run_len >= damaged_len
                        && (s == 0 || states[s - 1] != State::Broken)
                        && {
                            let limit = s + damaged_len as usize;
                            limit == total_spring_count || states[limit] != State::Broken
                        }
                    {
                        nways
                    } else {
                        0
                    }
                }
            };
        }

        prev_counts = curr_counts;
    }

    if false {
        println!("final counts:");
        println!("{prev_counts:?}");
    }

    let ans = Itertools::take_while_inclusive(
        prev_counts.iter().take(total_spring_count).enumerate(),
        |(s, _)| states[*s] != State::Broken,
    )
    .map(|(_s, ways)| *ways)
    .sum();

    if false {
        println!("Computed answer: {ans}");
    }

    ans
}

fn test_arrangemnt(states: &Vec<State>, configs: &Vec<u64>) -> bool {
    let mut blocksize = 0;
    let mut config = 0;
    for state in states {
        if let State::Broken = state {
            if blocksize < configs[config] {
                blocksize += 1;
            } else {
                return false;
            };
        } else if (blocksize > 0) && (blocksize != configs[config]) {
            return false;
        } else if blocksize > 0 {
            blocksize = 0;
            config += 1;
        }
    }
    true
}

#[cfg(test)]
mod tests {

    fn get_test_input() -> Vec<String> {
        vec![
            "???.### 1,1,3".to_string(),
            ".??..??...?##. 1,1,3".to_string(),
            "?#?#?#?#?#?#?#? 1,3,1,6".to_string(),
            "????.#...#... 4,1,1".to_string(),
            "????.######..#####. 1,6,5".to_string(),
            "?###???????? 3,2,1".to_string(),
        ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(get_test_input()), 21);
    }

    #[test]
    fn test_calculate_arrangements() {
        {
            let (states, configs) = super::parse_input(&"???.### 1,1,3".to_string());
            assert_eq!(super::calculate_arrangements(&states, &configs), 1);
        }

        {
            let (states, configs) = super::parse_input(&".??..??...?##. 1,1,3".to_string());
            assert_eq!(super::calculate_arrangements(&states, &configs), 4);
        }

        {
            let (states, configs) = super::parse_input(&"?#?#?#?#?#?#?#? 1,3,1,6".to_string());
            assert_eq!(super::calculate_arrangements(&states, &configs), 1);
        }

        {
            let (states, configs) = super::parse_input(&"????.#...#... 4,1,1".to_string());
            assert_eq!(super::calculate_arrangements(&states, &configs), 1);
        }

        {
            let (states, configs) = super::parse_input(&"????.######..#####. 1,6,5".to_string());
            assert_eq!(super::calculate_arrangements(&states, &configs), 4);
        }

        {
            let (states, configs) = super::parse_input(&"?###???????? 3,2,1".to_string());
            assert_eq!(super::calculate_arrangements(&states, &configs), 10);
        }
    }

    #[test]
    fn test_test_arrangement() {
        {
            let (states, configs) = super::parse_input(&"#.#.### 1,1,3".to_string());
            assert_eq!(super::test_arrangemnt(&states, &configs), true);
        }
        {
            let (states, configs) = super::parse_input(&"#.#.### 1,1,2".to_string());
            assert_eq!(super::test_arrangemnt(&states, &configs), false);
        }
    }

    #[test]
    fn test_unfold_input() {
        let (unfold, config) = super::unfold_input(&"???.### 1,1,3".to_string());
        assert_eq!(unfold.len(), 39);
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(get_test_input()), 525152);
    }

    #[test]
    fn test_calculate_arrangements_part2() {
        {
            let (states, configs) = super::unfold_input(&"???.### 1,1,3".to_string());
            assert_eq!(super::calculate_arrangements(&states, &configs), 1);
        }

        {
            let (states, configs) = super::unfold_input(&".??..??...?##. 1,1,3".to_string());
            assert_eq!(super::calculate_arrangements(&states, &configs), 16384);
        }

        {
            let (states, configs) = super::unfold_input(&"?#?#?#?#?#?#?#? 1,3,1,6".to_string());
            assert_eq!(super::calculate_arrangements(&states, &configs), 1);
        }

        {
            let (states, configs) = super::unfold_input(&"????.#...#... 4,1,1".to_string());
            assert_eq!(super::calculate_arrangements(&states, &configs), 16);
        }

        {
            let (states, configs) = super::unfold_input(&"????.######..#####. 1,6,5".to_string());
            assert_eq!(super::calculate_arrangements(&states, &configs), 2500);
        }

        {
            let (states, configs) = super::unfold_input(&"?###???????? 3,2,1".to_string());
            assert_eq!(super::calculate_arrangements(&states, &configs), 506250);
        }
    }
}
