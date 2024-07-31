fn main() {
    println!("Part 1: {}", part1(common::read_lines("./input.txt")));
    println!("Part 2: {}", part2(common::read_lines("./input.txt")));
}

fn part1(input: Vec<String>) -> u64 {
    let mut result = 0;

    input.split(|line| line.is_empty()).for_each(|group| {
        result += find_symetry_horizontal(group, 0) * 100_u64;
        result += find_symetry_vertical(group, 0);
    });

    result
}

fn part2(input: Vec<String>) -> u64 {
    let mut result = 0;

    input.split(|line| line.is_empty()).for_each(|group| {
        result += find_symetry_horizontal(group, 1) * 100_u64;
        result += find_symetry_vertical(group, 1);
    });

    result
}

fn find_symetry_horizontal(group: &[String], allowed_smudges: u64) -> u64 {
    for (i, line) in group.iter().enumerate() {
        if i < group.len() - 1 {
            if check_symetry_horizontal(group, i) == allowed_smudges {
                return i as u64 + 1 as u64;
            } else {
                continue;
            }
        }
    }

    0
}

fn check_symetry_horizontal(group: &[String], i: usize) -> u64 {
    let mut diff = 0_u64;
    for j in 0..group.len() / 2 {
        if (i as i64 - j as i64) >= 0 && (i + j + 1) < group.len() {
            for k in 0..group[0].len() {
                if group[i - j].chars().nth(k).unwrap() != group[i + j + 1].chars().nth(k).unwrap() {
                    diff += 1;
                }
            }
        }
    }
    diff
}

fn find_symetry_vertical(group: &[String], allowed_smudges: u64) -> u64 {
    for i in 0..(group[0].len() - 1) {
        if check_symetry_vertically(group, i) == allowed_smudges {
            return i as u64 + 1 as u64;
        }
    }
    0
}

fn check_symetry_vertically(group: &[String], i: usize) -> u64 {
    let mut diff = 0_u64;
    for j in 0..group[0].len() / 2 {
        if (i as i64 - j as i64) >= 0 && (i + j + 1) < group[0].len() {
            for line in 0..group.len() {
                if group[line].chars().nth(i - j).unwrap()
                    != group[line].chars().nth(i + j + 1).unwrap()
                {
                    diff += 1;
                }
            }
        }
    }

    diff
}

#[cfg(test)]
mod tests {
    fn get_test_input() -> Vec<String> {
        vec![
            "#.##..##.".to_string(),
            "..#.##.#.".to_string(),
            "##......#".to_string(),
            "##......#".to_string(),
            "..#.##.#.".to_string(),
            "..##..##.".to_string(),
            "#.#.##.#.".to_string(),
            "".to_string(),
            "#...##..#".to_string(),
            "#....#..#".to_string(),
            "..##..###".to_string(),
            "#####.##.".to_string(),
            "#####.##.".to_string(),
            "..##..###".to_string(),
            "#....#..#".to_string(),
        ]
    }

    fn get_test_group1() -> Vec<String> {
        vec![
            "#.##..##.".to_string(),
            "..#.##.#.".to_string(),
            "##......#".to_string(),
            "##......#".to_string(),
            "..#.##.#.".to_string(),
            "..##..##.".to_string(),
            "#.#.##.#.".to_string(),
        ]
    }

    fn get_test_group2() -> Vec<String> {
        vec![
            "#...##..#".to_string(),
            "#....#..#".to_string(),
            "..##..###".to_string(),
            "#####.##.".to_string(),
            "#####.##.".to_string(),
            "..##..###".to_string(),
            "#....#..#".to_string(),
        ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(get_test_input()), 405);
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(get_test_input()), 400);
    }

    #[test]
    fn test_find_symetry_horizontal() {
        assert_eq!(super::find_symetry_horizontal(&get_test_group1(), 0), 0);
        assert_eq!(super::find_symetry_horizontal(&get_test_group2(), 0), 4);
        assert_eq!(super::find_symetry_horizontal(&get_test_group1(), 1), 3);
        assert_eq!(super::find_symetry_horizontal(&get_test_group2(), 1), 1);
    }

    #[test]
    fn test_find_symetry_vertical() {
        assert_eq!(super::find_symetry_vertical(&get_test_group1(), 0), 5);
        assert_eq!(super::find_symetry_vertical(&get_test_group2(), 0), 0);
        assert_eq!(super::find_symetry_vertical(&get_test_group1(), 1), 0);
        assert_eq!(super::find_symetry_vertical(&get_test_group2(), 1), 0);
    }
}
