use std::fmt::Debug;

use common;

#[derive(Debug)]
struct hand {
    cards: [char; 5],
    bid: u64,
    value: u64,
    rank: u64,
}

fn main() {
    println!("Part 1: {}", part1(common::read_lines("./input1.txt")));
    println!("Part 2: {}", part2(common::read_lines("./input2.txt")));
}

fn part1(input: Vec<String>) -> u64 {
    let mut hands = get_hands(input);
    let mut points: u64 = 0;
    let mut multiplier: u64 = 1;

    // for hand in hands.iter() {
    //     println!("{:?}", hand);
    // }

    hands.sort_by_key(|x| x.value);

    // for hand in hands.iter() {
    //     println!("{:?}", hand);
    // }

    for hand in hands.iter_mut() {
        points += hand.bid * multiplier;
        multiplier += 1;
        hand.rank = multiplier;
    }

    points
}

fn part2(input: Vec<String>) -> u64 {
    let mut hands = get_hands2(input);
    let mut points: u64 = 0;
    let mut multiplier: u64 = 1;

    // for hand in hands.iter() {
    //     println!("{:?}", hand);
    // }

    hands.sort_by_key(|x| x.value);

    // for hand in hands.iter() {
    //     println!("{:?}", hand);
    // }

    for hand in hands.iter_mut() {
        points += hand.bid * multiplier;
        multiplier += 1;
        hand.rank = multiplier;
    }

    points
}

fn get_hands(mut input: Vec<String>) -> Vec<hand> {
    let mut hands: Vec<hand> = Vec::new();
    for line in input {
        hands.push(get_hand(line));
    }

    hands
}

fn get_hands2(mut input: Vec<String>) -> Vec<hand> {
    let mut hands: Vec<hand> = Vec::new();

    input.iter_mut().for_each(|mut line| {
        *line = line.replace("J", "1");
        hands.push(get_hand(line.to_string()));
    });

    hands
}

fn get_hand(line: String) -> hand {
    let mut cards: [char; 5] = ['0'; 5];
    let line_split = line.split_whitespace().collect::<Vec<&str>>();

    let mut i: usize = 0;
    for c in line_split[0].chars() {
        cards[i] = c;
        i += 1;
    }

    hand {
        cards: cards,
        bid: line_split[1].parse::<u64>().unwrap(),
        value: get_hand_value(cards),
        rank: 0,
    }
}

fn get_hand_value(cards: [char; 5]) -> u64 {
    let mut value: u64 = get_type_value(cards);

    for c in cards.iter() {
        value *= 100;
        value += get_card_value(*c);
    }

    value
}

fn get_type_value(cards: [char; 5]) -> u64 {
    let mut value: u64 = 0;
    let mut occ_biggest: u64 = 0;
    let mut occ_second_biggest: u64 = 0;
    let mut checked = vec![];
    //println!("cards= {:?}", cards);
    for c in cards.iter() {
        if checked.contains(c) || *c == '1' {
            continue;
        }
        checked.push(*c);
        let mut occ: u64 = cards.iter().filter(|&x| x == c).count() as u64;

        if occ > occ_biggest {
            occ_second_biggest = occ_biggest;
            occ_biggest = occ;
        } else if occ > occ_second_biggest {
            occ_second_biggest = occ;
        }
    }

    let wild = cards.iter().filter(|&x| x == &'1').count() as u64;
    occ_biggest += wild;

    if occ_biggest >= 4 {
        value = occ_biggest + 2;
        //println!("Five or four: {:?}  -> {}", cards, value);
    } else if occ_biggest == 3 && occ_second_biggest >= 2 {
        value = 5;
        //println!("Full: {:?}  -> {}", cards, value);
    } else if occ_biggest == 2 && occ_second_biggest == 2 {
        value = 3;
        //println!("Two pair: {:?}  -> {}", cards, value);
    } else if occ_biggest == 3 {
        value = 4;
        //println!("Triple: {:?}  -> {}", cards, value);
    } else {
        value = occ_biggest;
        //println!("Pair or high: {:?}  -> {}", cards, value);
    }

    //println!("{:?} -> {}", cards, value);
    //println!("{} > {}", occ_biggest, occ_second_biggest);

    value
}

fn get_card_value(c: char) -> u64 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => c.to_digit(10).unwrap() as u64,
    }
}

#[cfg(test)]
mod tests {

    fn get_part1_test_input() -> Vec<String> {
        vec![
            "32T3K 765".to_string(),
            "T55J5 684".to_string(),
            "KK677 28".to_string(),
            "KTJJT 220".to_string(),
            "QQQJA 483".to_string(),
        ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(get_part1_test_input()), 6440);
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(get_part1_test_input()), 5905);
    }

    #[test]
    fn test_get_type_value() {
        assert_eq!(super::get_type_value(['A', 'A', 'A', 'A', 'A']), 7);
        assert_eq!(super::get_type_value(['A', 'A', '8', 'A', 'A']), 6);
        assert_eq!(super::get_type_value(['2', '3', '3', '3', '2']), 5);
        assert_eq!(super::get_type_value(['T', 'T', 'T', '9', '8']), 4);
        assert_eq!(super::get_type_value(['2', '3', '4', '3', '2']), 3);
        assert_eq!(super::get_type_value(['A', '2', '3', 'A', '4']), 2);
        assert_eq!(super::get_type_value(['2', '3', '4', '5', '6']), 1);
    }

    #[test]
    fn test_get_card_value() {
        assert_eq!(super::get_card_value('A'), 14);
        assert_eq!(super::get_card_value('K'), 13);
        assert_eq!(super::get_card_value('Q'), 12);
        assert_eq!(super::get_card_value('J'), 11);
        assert_eq!(super::get_card_value('T'), 10);
        assert_eq!(super::get_card_value('9'), 9);
        assert_eq!(super::get_card_value('8'), 8);
        assert_eq!(super::get_card_value('7'), 7);
        assert_eq!(super::get_card_value('6'), 6);
        assert_eq!(super::get_card_value('5'), 5);
        assert_eq!(super::get_card_value('4'), 4);
        assert_eq!(super::get_card_value('3'), 3);
        assert_eq!(super::get_card_value('2'), 2);
    }

    #[test]
    fn test_get_hand_value() {
        assert_eq!(
            super::get_hand_value(['A', 'A', 'A', 'A', 'A']),
            71414141414
        );
        assert_eq!(
            super::get_hand_value(['A', 'A', '8', 'A', 'A']),
            61414081414
        );
        assert_eq!(
            super::get_hand_value(['2', '3', '3', '3', '2']),
            50203030302
        );
        assert_eq!(
            super::get_hand_value(['T', 'T', 'T', '9', '8']),
            41010100908
        );
        assert_eq!(
            super::get_hand_value(['2', '3', '4', '3', '2']),
            30203040302
        );
        assert_eq!(
            super::get_hand_value(['A', '2', '3', 'A', '4']),
            21402031404
        );
        assert_eq!(
            super::get_hand_value(['2', '3', '4', '5', '6']),
            10203040506
        );
    }
}
