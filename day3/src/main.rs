use std::fs::read_to_string;

fn main() {
    let input = read_lines("./input.txt");

    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input.clone()));
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn part1(input: Vec<String>) -> u32 {
    let mut sum: u32 = 0;

    let parts: Vec<Part> = get_parts(input);

    parts.iter().for_each(|part: &Part| {
        if part.is_part {
            sum += part.num;
        }
    });

    sum
}

fn part2(input: Vec<String>) -> u32 {
    let mut sum: u32 = 0;

    let gears = get_gears(input);

    gears.iter().for_each(|gear| {
        if gear.is_gear {
            sum += gear.num;
        }
    });

    sum
}

#[derive(Debug)]
struct Part {
    num: u32,
    is_part: bool,
}

#[derive(Debug)]
struct Gear {
    num: u32,
    is_gear: bool,
    i: usize,
    j: usize,
}

fn is_part_char(c: char) -> bool {
    !c.is_ascii_alphanumeric() && c != '.' && c != ' '
}

fn is_gear_char(c: char) -> bool {
    c == '*'
}

fn add_gear(gear: Gear, gears: &mut Vec<Gear>) {
    for g in gears.iter_mut() {
        if g.i == gear.i && g.j == gear.j {
            g.is_gear = true;
            g.num *= gear.num;
            return;
        }
    }

    gears.push(gear);
}

fn get_gears(input: Vec<String>) -> Vec<Gear> {
    let mut gears: Vec<Gear> = Vec::new();
    let mut num: u32 = 0;
    let mut is_gear: bool = false;
    let mut gear_i: usize = 0;
    let mut gear_j: usize = 0;

    input.iter().enumerate().for_each(|(i, line)| {
        num = 0;

        line.chars().enumerate().for_each(|(j, c)| {
            if c.is_ascii_digit() {
                num *= 10;
                num += c.to_digit(10).unwrap();

                // todo figure out for loops
                for x in 0..=2 {
                    for y in 0..=2 {
                        if i + x < input.len() + 1
                            && j + y < line.len() + 1
                            && i + x > 0
                            && j + y > 0
                            && is_gear_char(input[i + x - 1].chars().nth(j + y - 1).unwrap())
                        {
                            gear_i = i + x - 1;
                            gear_j = j + y - 1;
                            is_gear |= true;
                        }
                    }
                }
            } else if num != 0 && is_gear {
                add_gear(
                    Gear {
                        num,
                        is_gear: false,
                        i: gear_i,
                        j: gear_j,
                    },
                    &mut gears,
                );
                is_gear = false;
                num = 0;
            } else {
                num = 0;
            }
        });

        if num != 0 && is_gear {
            add_gear(
                Gear {
                    num,
                    is_gear: false,
                    i: gear_i,
                    j: gear_j,
                },
                &mut gears,
            );
            is_gear = false;
            num = 0;
        }
    });
    gears
}

fn get_parts(input: Vec<String>) -> Vec<Part> {
    let mut parts: Vec<Part> = Vec::new();
    let mut num: u32 = 0;
    let mut is_part: bool = false;

    input.iter().enumerate().for_each(|(i, line)| {
        num = 0;
        is_part = false;

        line.chars().enumerate().for_each(|(j, c)| {
            if c.is_ascii_digit() {
                num *= 10;
                num += c.to_digit(10).unwrap();

                // maybe refactor this later
                if j < line.len() - 1 {
                    is_part |= is_part_char(line.chars().nth(j + 1).unwrap());
                }
                if j > 0 {
                    is_part |= is_part_char(line.chars().nth(j - 1).unwrap());
                }
                if i < input.len() - 1 {
                    is_part |= is_part_char(input[i + 1].chars().nth(j).unwrap());
                }
                if i > 0 {
                    is_part |= is_part_char(input[i - 1].chars().nth(j).unwrap());
                }
                if i < input.len() - 1 && j < line.len() - 1 {
                    is_part |= is_part_char(input[i + 1].chars().nth(j + 1).unwrap());
                }
                if i > 0 && j > 0 {
                    is_part |= is_part_char(input[i - 1].chars().nth(j - 1).unwrap());
                }
                if i < input.len() - 1 && j > 0 {
                    is_part |= is_part_char(input[i + 1].chars().nth(j - 1).unwrap());
                }
                if i > 0 && j < line.len() - 1 {
                    is_part |= is_part_char(input[i - 1].chars().nth(j + 1).unwrap());
                }
            } else if num != 0 {
                parts.push(Part { num, is_part });
                is_part = false;
                num = 0;
            }
        });

        if num != 0 {
            parts.push(Part { num, is_part });
            is_part = false;
            num = 0;
        }
    });
    parts
}

#[cfg(test)]
mod tests {
    use crate::get_parts;

    #[test]
    fn test_part1() {
        let input: Vec<String> = vec![
            "467..114..".to_string(),
            "...*......".to_string(),
            "..35..633.".to_string(),
            "......#...".to_string(),
            "617*......".to_string(),
            ".....+.58.".to_string(),
            "..592.....".to_string(),
            "......755.".to_string(),
            "...$.*....".to_string(),
            ".664.598..".to_string(),
        ];
        assert_eq!(super::part1(input), 4361);
    }

    #[test]
    fn test_is_part_char() {
        assert_eq!(super::is_part_char('.'), false);
        assert_eq!(super::is_part_char('*'), true);
        assert_eq!(super::is_part_char('$'), true);
        assert_eq!(super::is_part_char('+'), true);
        assert_eq!(super::is_part_char('#'), true);
        assert_eq!(super::is_part_char('a'), false);
        assert_eq!(super::is_part_char('A'), false);
        assert_eq!(super::is_part_char('1'), false);
        assert_eq!(super::is_part_char(' '), false);
    }

    #[test]
    fn test_get_parts() {
        let input: Vec<String> = vec![
            "467..114..".to_string(),
            "...*......".to_string(),
            "..35..633.".to_string(),
            "......#...".to_string(),
            "617*......".to_string(),
            ".....+.58.".to_string(),
            "..592.....".to_string(),
            "......755.".to_string(),
            "...$.*....".to_string(),
            ".664.598..".to_string(),
        ];
        let parts: Vec<super::Part> = get_parts(input);

        dbg!(&parts);
        assert_eq!(parts[0].num, 467);
        assert_eq!(parts[0].is_part, true);
        assert_eq!(parts[1].num, 114);
        assert_eq!(parts[1].is_part, false);
        assert_eq!(parts[2].num, 35);
        assert_eq!(parts[2].is_part, true);
        assert_eq!(parts[3].num, 633);
        assert_eq!(parts[3].is_part, true);
        assert_eq!(parts[4].num, 617);
        assert_eq!(parts[4].is_part, true);
        assert_eq!(parts[5].num, 58);
        assert_eq!(parts[5].is_part, false);
        assert_eq!(parts[6].num, 592);
        assert_eq!(parts[6].is_part, true);
        assert_eq!(parts[7].num, 755);
        assert_eq!(parts[7].is_part, true);
        assert_eq!(parts[8].num, 664);
        assert_eq!(parts[8].is_part, true);
        assert_eq!(parts[9].num, 598);
        assert_eq!(parts[9].is_part, true);
    }

    #[test]
    fn test_part2() {
        let input: Vec<String> = vec![
            "467..114..".to_string(),
            "...*......".to_string(),
            "..35..633.".to_string(),
            "......#...".to_string(),
            "617*......".to_string(),
            ".....+.58.".to_string(),
            "..592.....".to_string(),
            "......755.".to_string(),
            "...$.*....".to_string(),
            ".664.598..".to_string(),
        ];
        assert_eq!(super::part2(input), 467835);
    }

    #[test]
    fn test_is_gear_char() {
        assert_eq!(super::is_gear_char('.'), false);
        assert_eq!(super::is_gear_char('*'), true);
        assert_eq!(super::is_gear_char('$'), false);
        assert_eq!(super::is_gear_char('+'), false);
        assert_eq!(super::is_gear_char('#'), false);
        assert_eq!(super::is_gear_char('a'), false);
        assert_eq!(super::is_gear_char('A'), false);
        assert_eq!(super::is_gear_char('1'), false);
        assert_eq!(super::is_gear_char(' '), false);
    }

    #[test]
    fn test_add_gear() {
        let mut gears: Vec<super::Gear> = Vec::new();
        let gear: super::Gear = super::Gear {
            num: 2,
            is_gear: false,
            i: 0,
            j: 0,
        };
        super::add_gear(gear, &mut gears);

        dbg!(&gears);

        assert_eq!(gears.len(), 1);
        assert_eq!(gears[0].num, 2);
        assert_eq!(gears[0].is_gear, false);
        assert_eq!(gears[0].i, 0);
        assert_eq!(gears[0].j, 0);

        let gear: super::Gear = super::Gear {
            num: 3,
            is_gear: false,
            i: 0,
            j: 0,
        };
        super::add_gear(gear, &mut gears);

        dbg!(&gears);

        assert_eq!(gears.len(), 1);
        assert_eq!(gears[0].num, 6);
        assert_eq!(gears[0].is_gear, true);
        assert_eq!(gears[0].i, 0);
        assert_eq!(gears[0].j, 0);

        // let mut gear: super::Gear = super::Gear {
        //     num: 1,
        //     is_gear: false,
        //     i: 0,
        //     j: 1,
        // };
        // super::add_gear(&mut gear, &mut gears);
        // assert_eq!(gears.len(), 2);
        // assert_eq!(gears[1].num, 1);
        // assert_eq!(gears[1].is_gear, false);
        // assert_eq!(gears[1].i, 0);
        // assert_eq!(gears[1].j, 1);

        // let mut gear: super::Gear = super::Gear {
        //     num: 1,
        //     is_gear: false,
        //     i: 0,
        //     j: 1,
        // };
        // super::add_gear(&mut gear, &mut gears);
        // assert_eq!(gears.len(), 2);
        // assert_eq!(gears[1].num, 1);
        // assert_eq!(gears[1].is_gear, true);
        // assert_eq!(gears[1].i, 0);
        // assert_eq!(gears[1].j, 1);
    }
}
