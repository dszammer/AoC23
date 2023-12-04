use std::fs::read_to_string;

fn main() {
    let input = read_lines("./input.txt");

    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input));

    //println!("Part 2: {}", part2(input));
}

#[derive(Debug)]
struct Cube<'a> {
    color: &'a str,
    num: u32,
}

#[derive(Debug)]
struct Set<'a> {
    cubes: Vec<Cube<'a>>,
}

#[derive(Debug)]
struct Game<'a> {
    game_num: u32,
    game: Vec<Set<'a>>,
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

    input.iter().for_each(|line: &String| {
        let game: Game = get_game(line);
        if is_valid(&game) {
            sum += game.game_num;
        }
    });

    sum
}

fn part2(input: Vec<String>) -> u32 {
    let mut sum: u32 = 0;

    input.iter().for_each(|line: &String| {
        let game: Game = get_game(line);
        sum += power(&get_min_set(&game));
    });

    sum
}

// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
fn get_game(line: &str) -> Game {
    let mut game: Game = Game {
        game_num: get_game_number(line),
        game: vec![],
    };

    line.split(':').collect::<Vec<&str>>()[1]
        .split(';')
        .collect::<Vec<&str>>()
        .iter()
        .for_each(|s| {
            game.game.push(get_set(s));
        });
    game
}

fn is_valid(game: &Game) -> bool {
    let mut valid: bool = true;

    game.game.iter().for_each(|set: &Set| {
        set.cubes.iter().for_each(|cube: &Cube| {
            if cube.color == "red" && cube.num > 12 {
                valid = false;
            }
            if cube.color == "green" && cube.num > 13 {
                valid = false;
            }
            if cube.color == "blue" && cube.num > 14 {
                valid = false;
            }
        });
    });

    valid
}

// 3 blue, 4 red
fn get_set(line: &str) -> Set {
    let mut set: Set = Set { cubes: vec![] };

    line.split(',').collect::<Vec<&str>>().iter().for_each(|s| {
        set.cubes.push(get_cube(s));
    });
    set
}

// 3 blue
fn get_cube(line: &str) -> Cube {
    let split: Vec<&str> = line.split(' ').collect::<Vec<&str>>();

    let cube: Cube = Cube {
        color: split[2],
        num: split[1].parse::<u32>().unwrap(),
    };
    cube
}

fn get_game_number(line: &str) -> u32 {
    line.split(':').collect::<Vec<&str>>()[0]
        .split(' ')
        .collect::<Vec<&str>>()[1]
        .parse::<u32>()
        .unwrap()
}

fn get_min_set<'a>(g: &'a Game<'a>) -> Set<'a> {
    let mut min_set: Set = Set { cubes: vec![] };

    min_set.cubes.push(Cube {
        color: "red",
        num: 0,
    });
    min_set.cubes.push(Cube {
        color: "green",
        num: 0,
    });
    min_set.cubes.push(Cube {
        color: "blue",
        num: 0,
    });

    g.game.iter().for_each(|set: &Set| {
        set.cubes.iter().for_each(|cube: &Cube| {
            min_set.cubes.iter_mut().for_each(|min_cube: &mut Cube| {
                if cube.color == min_cube.color && cube.num > min_cube.num {
                    min_cube.num = cube.num;
                }
            });
        });
    });

    min_set
}

fn power(set: &Set) -> u32 {
    let mut power: u32 = 1;
    set.cubes.iter().for_each(|cube: &Cube| {
        power *= cube.num;
    });
    power
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {
        let input: Vec<String> = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string(),
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string(),
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string(),
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string(),
        ];
        assert_eq!(super::part1(input), 8);
    }

    fn test_part2() {
        let input: Vec<String> = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string(),
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string(),
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string(),
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string(),
        ];
        assert_eq!(super::part2(input), 2286);
    }

    #[test]
    fn test_cube() {
        let line: &str = " 3 blue";
        let cube: super::Cube = super::get_cube(line);
        assert_eq!(cube.num, 3);
    }

    #[test]
    fn test_game_number() {
        let line: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game_number = super::get_game_number(line);
        assert_eq!(game_number, 1);
    }

    #[test]
    fn test_set() {
        let line: &str = " 3 blue, 4 red";
        let set: super::Set = super::get_set(line);
        assert_eq!(set.cubes.len(), 2);
        assert_eq!(set.cubes[0].num, 3);
        assert_eq!(set.cubes[0].color, "blue");
        assert_eq!(set.cubes[1].num, 4);
        assert_eq!(set.cubes[1].color, "red");
    }

    #[test]
    fn test_game() {
        let line: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game = super::get_game(line);
        assert_eq!(game.game_num, 1);
        assert_eq!(game.game.len(), 3);
        assert_eq!(game.game[0].cubes.len(), 2);
        assert_eq!(game.game[0].cubes[0].num, 3);
        assert_eq!(game.game[0].cubes[0].color, "blue");
    }

    #[test]
    fn test_is_valid() {
        let line: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game = super::get_game(line);
        assert_eq!(super::is_valid(&game), true);

        let line: &str = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        let game = super::get_game(line);
        assert_eq!(super::is_valid(&game), false);
    }

    #[test]
    fn test_min_set() {
        let line: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game: super::Game = super::get_game(line);
        let min_set: super::Set = super::get_min_set(&game);
        assert_eq!(min_set.cubes.len(), 3);
        assert_eq!(min_set.cubes[0].num, 4);
        assert_eq!(min_set.cubes[0].color, "red");
        assert_eq!(min_set.cubes[1].num, 2);
        assert_eq!(min_set.cubes[1].color, "green");
        assert_eq!(min_set.cubes[2].num, 6);
        assert_eq!(min_set.cubes[2].color, "blue");
    }

    #[test]
    fn test_power() {
        let line: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game: super::Game = super::get_game(line);
        let min_set: super::Set = super::get_min_set(&game);
        assert_eq!(super::power(&min_set), 48);
    }
}
