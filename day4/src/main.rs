use num_traits::pow;
use std::fs::read_to_string;

fn main() {
    let input = read_lines("./input.txt");

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

    let input_clean = input
        .iter()
        .map(|line| line.replace("  ", " "))
        .collect::<Vec<String>>();

    for line in input_clean {
        dbg!(&line);
        let mut points: u32 = 0;
        let card: Card = Card {
            number: 0,
            winning: to_vec_u32(
                line.as_str().split(": ").collect::<Vec<&str>>()[1]
                    .split(" | ")
                    .collect::<Vec<&str>>()[0],
            ),
            having: to_vec_u32(
                line.as_str().split(": ").collect::<Vec<&str>>()[1]
                    .split(" | ")
                    .collect::<Vec<&str>>()[1],
            ),
        };

        card.winning.iter().for_each(|win| {
            card.having.iter().for_each(|have| {
                if win == have {
                    points += 1;
                }
            })
        });

        if points != 0 {
            sum += pow(2u32, (points - 1).try_into().unwrap());
        }
    }

    // let _ = input_clean
    //     .iter()
    //     .map(|line| {
    //         line.as_str().split(": ").collect::<Vec<&str>>()[1]
    //             .split(" | ")
    //             .collect::<Vec<&str>>()
    //     })
    //     .collect::<Vec<Vec<&str>>>()
    //     .iter()
    //     .map(|line_vec| {
    //         dbg!(line_vec);
    //         let mut point: u32 = 0;
    //         let _ = line_vec[0]
    //             .split(' ')
    //             .collect::<Vec<&str>>()
    //             .iter()
    //             .map(|&s| s.parse::<u32>().expect("Failed to parse u32"))
    //             .collect::<Vec<u32>>()
    //             .iter()
    //             .map(|winning| {
    //                 dbg!(winning);
    //                 dbg!(line_vec[1]);
    //                 let _ = line_vec[1]
    //                     .split(' ')
    //                     .collect::<Vec<&str>>()
    //                     .iter()
    //                     .map(|&s| s.parse::<u32>().expect("Failed to parse u32"))
    //                     .collect::<Vec<u32>>()
    //                     .iter()
    //                     .map(|having| {
    //                         dbg!(having);
    //                         if winning == having {
    //                             point += 1;
    //                         }
    //                     });
    //             })
    //             .collect::<Vec<_>>();
    //         if point != 0 {
    //              sum += pow(2u32, (point - 1).try_into().unwrap());
    //         }
    //         dbg!(sum);
    //     })
    //     .collect::<Vec<_>>();

    sum
}

fn part2(input: &Vec<String>) -> u32 {
    let mut sum = 0;

    sum
}

struct Card {
    number: u32,
    winning: Vec<u32>,
    having: Vec<u32>,
}

fn get_card_number(line: &str) -> u32 {
    line.split(' ').collect::<Vec<&str>>()[1]
        .parse::<u32>()
        .unwrap()
}

fn to_vec_u32(line: &str) -> Vec<u32> {
    line.split(' ')
        .collect::<Vec<&str>>()
        .iter()
        .map(|&s| s.parse::<u32>().expect("Failed to parse u32"))
        .collect::<Vec<u32>>()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {
        let input: Vec<String> = vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string(),
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19".to_string(),
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1".to_string(),
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83".to_string(),
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36".to_string(),
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string(),
        ];
        assert_eq!(super::part1(&input), 13);
    }
    #[test]
    fn test_card_number() {
        let line: &str = "Card 1";
        assert_eq!(super::get_card_number(&line), 1);
    }
    #[test]
    fn test_to_vec_u32() {
        let line: &str = "41 48 83 86 17";
        assert_eq!(super::to_vec_u32(&line), [41, 48, 83, 86, 17]);
    }
}
