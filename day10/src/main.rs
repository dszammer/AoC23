use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
} //origin top left

impl Copy for Point {}

#[derive(Debug)]
struct Pipe {
    connections: Vec<Point>,
}

fn main() {
    println!("Part 1: {}", part1(common::read_lines("./input1.txt")));
    println!("Part 2: {}", part2(common::read_lines("./input2.txt")));
}

fn part1(input: Vec<String>) -> u64 {
    let mut start_point = Point { x: 0, y: 0 };
    let mut map = get_map(input, &mut start_point);

    map.insert(
        Point {
            x: start_point.x,
            y: start_point.y,
        },
        get_start_pipe(&map, &start_point),
    );

    let mut steps = 1;
    let mut previous = start_point;
    let mut current = &map.get(&start_point).unwrap().connections[0];

    while current != &start_point {
        for connection in map.get(current).unwrap().connections.iter() {
            if connection != &previous {
                previous = *current;
                current = connection;
                break;
            }
        }

        steps += 1;
    }

    steps / 2
}

#[allow(unused_variables)]
fn part2(input: Vec<String>) -> u64 {
    0
}

fn get_map(input: Vec<String>, start: &mut Point) -> HashMap<Point, Pipe> {
    let mut map = HashMap::<Point, Pipe>::new();
    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let mut pipe = Pipe {
                connections: Vec::new(),
            };
            match c {
                '.' => continue,
                '|' => {
                    pipe.connections.push(Point {
                        x: x as i32,
                        y: y as i32 + 1,
                    }); //south
                    pipe.connections.push(Point {
                        x: x as i32,
                        y: y as i32 - 1,
                    }) //north
                }
                '-' => {
                    pipe.connections.push(Point {
                        x: x as i32 + 1,
                        y: y as i32,
                    }); //east
                    pipe.connections.push(Point {
                        x: x as i32 - 1,
                        y: y as i32,
                    }) //west
                }
                'L' => {
                    pipe.connections.push(Point {
                        x: x as i32 + 1,
                        y: y as i32,
                    }); //east
                    pipe.connections.push(Point {
                        x: x as i32,
                        y: y as i32 - 1,
                    }) //north
                }
                'J' => {
                    pipe.connections.push(Point {
                        x: x as i32 - 1,
                        y: y as i32,
                    }); //west
                    pipe.connections.push(Point {
                        x: x as i32,
                        y: y as i32 - 1,
                    }) //north
                }
                '7' => {
                    pipe.connections.push(Point {
                        x: x as i32 - 1,
                        y: y as i32,
                    }); //west
                    pipe.connections.push(Point {
                        x: x as i32,
                        y: y as i32 + 1,
                    }) //south
                }
                'F' => {
                    pipe.connections.push(Point {
                        x: x as i32 + 1,
                        y: y as i32,
                    }); //east
                    pipe.connections.push(Point {
                        x: x as i32,
                        y: y as i32 + 1,
                    }) //south
                }
                'S' => {
                    start.x = x as i32;
                    start.y = y as i32;
                }
                _ => {}
            }
            map.insert(
                Point {
                    x: x as i32,
                    y: y as i32,
                },
                pipe,
            );
        }
    }

    map
}

fn get_start_pipe(map: &HashMap<Point, Pipe>, start: &Point) -> Pipe {
    let mut start_pipe = Pipe {
        connections: Vec::new(),
    };

    for i in -1..=1 {
        for j in -1..=1 {
            if let Some(pipe) = map.get(&Point {
                x: start.x + i,
                y: start.y + j,
            }) {
                if pipe.connections.contains(&Point {
                    x: start.x,
                    y: start.y,
                }) {
                    start_pipe.connections.push(Point {
                        x: start.x + i,
                        y: start.y + j,
                    });
                }
            }
        }
    }

    start_pipe
}

#[cfg(test)]
mod tests {

    fn get_test_input() -> Vec<String> {
        vec![
            "L-|F7".to_string(),
            "7S-7|".to_string(),
            "L|7||".to_string(),
            "-L-J|".to_string(),
            "L|-JF".to_string(),
        ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(get_test_input()), 4);
    }

    #[test]
    fn test_get_start() {
        let mut start_point = super::Point { x: 0, y: 0 };
        let map = super::get_map(get_test_input(), &mut start_point);

        let start = super::get_start_pipe(&map, &start_point);

        print!("{:?}", start);

        assert_eq!(start_point.x, 1);
        assert_eq!(start_point.y, 1);
        assert_eq!(start.connections.len(), 2);
    }

    #[test]
    fn test_get_map() {
        let mut start_point = super::Point { x: 0, y: 0 };
        let map = super::get_map(get_test_input(), &mut start_point);

        print!("{:?}", map);

        assert_eq!(start_point.x, 1);
        assert_eq!(start_point.y, 1);
        assert_eq!(map.len(), 25);
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(get_test_input()), 0);
    }
}
