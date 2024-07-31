fn main() {
    println!("Part 1: {}", part1(common::read_lines("./input1.txt")));
    println!("Part 2: {}", part2(common::read_lines("./input2.txt")));
}

fn part1(input: Vec<String>) -> u64 {
    let mut galaxies = get_galaxies(&input);
    let expanding_cols = get_expanding_cols(&input);
    let expanding_rows = get_expanding_rows(&input);
    expand_universe(&mut galaxies, &expanding_cols, &expanding_rows, 2);
    fast_manhattan_distance_sum(&galaxies)
}

fn part2(input: Vec<String>) -> u64 {
    let mut galaxies = get_galaxies(&input);
    let expanding_cols = get_expanding_cols(&input);
    let expanding_rows = get_expanding_rows(&input);
    expand_universe(&mut galaxies, &expanding_cols, &expanding_rows, 1000000);
    fast_manhattan_distance_sum(&galaxies)
}

fn get_galaxies(input: &[String]) -> Vec<common::Point> {
    let mut galaxies: Vec<common::Point> = Vec::new();

    for (i, line) in input.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push(common::Point {
                    x: j.try_into().unwrap(),
                    y: i.try_into().unwrap(),
                });
            }
        }
    }

    galaxies
}

fn get_expanding_rows(input: &[String]) -> Vec<u64> {
    let mut rows: Vec<u64> = Vec::new();

    for (i, line) in input.iter().enumerate() {
        if line.find('#').is_some() {
            continue;
        }
        rows.push(i.try_into().unwrap());
    }
    rows
}

fn get_expanding_cols(input: &[String]) -> Vec<u64> {
    let mut cols: Vec<u64> = Vec::new();

    for i in 0..input[0].len() {
        let mut found = false;
        for line in input.iter() {
            if line.chars().nth(i).unwrap() == '#' {
                found = true;
                break;
            }
        }
        if !found {
            cols.push(i.try_into().unwrap());
        }
    }

    cols
}

fn expand_universe(
    galaxies: &mut [common::Point],
    expanding_cols: &[u64],
    expanding_rows: &[u64],
    expansion_factor: u64,
) {
    galaxies.iter_mut().for_each(|galaxy| {
        let mut expansion_x = 0_u64;
        let mut expansion_y = 0_u64;

        for col in expanding_cols.iter() {
            if galaxy.x > *col as i64 {
                expansion_x += expansion_factor - 1;
            } else {
                break;
            }
        }

        for row in expanding_rows.iter() {
            if galaxy.y > *row as i64 {
                expansion_y += expansion_factor - 1;
            } else {
                break;
            }
        }

        galaxy.x += expansion_x as i64;
        galaxy.y += expansion_y as i64;
    });
}

#[allow(dead_code)] //fast_manhattan_distance_sum is used instead
fn manhattan_distance_sum(points: &Vec<common::Point>) -> u64 {
    let mut sum = 0;

    for i in 0..points.len() {
        for j in i + 1..points.len() {
            sum += (points[i].x - points[j].x).unsigned_abs();
            sum += (points[i].y - points[j].y).unsigned_abs();
        }
    }

    sum
}

fn fast_manhattan_distance_sum(points: &Vec<common::Point>) -> u64 {
    let mut sum = 0;
    let mut x_coords: Vec<i64> = points.iter().map(|point| point.x).collect::<Vec<_>>();
    let mut y_coords: Vec<i64> = points.iter().map(|point| point.y).collect::<Vec<_>>();

    x_coords.sort();
    y_coords.sort();

    for i in 1..points.len() {
        sum += i as i64 * x_coords[i] - (points.len() - i) as i64 * x_coords[i - 1];
        sum += i as i64 * y_coords[i] - (points.len() - i) as i64 * y_coords[i - 1];
    }

    sum.unsigned_abs()
}

#[cfg(test)]
mod tests {

    fn get_test_input() -> Vec<String> {
        vec![
            ".......#..".to_string(),
            "#.........".to_string(),
            "......#...".to_string(),
            "...#......".to_string(),
            ".#........".to_string(),
            "..........".to_string(),
            ".........#".to_string(),
            "..........".to_string(),
            ".......#..".to_string(),
            "#...#.....".to_string(),
        ]
    }

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(get_test_input()), 374);
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(get_test_input()), 82000210);
    }

    #[test]
    fn test_get_expanding_rows() {
        let rows = super::get_expanding_rows(&get_test_input());
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0], 3);
        assert_eq!(rows[1], 7);
    }

    #[test]
    fn test_get_expanding_cols() {
        let cols = super::get_expanding_cols(&get_test_input());
        assert_eq!(cols.len(), 3);
        assert_eq!(cols[0], 2);
        assert_eq!(cols[1], 5);
        assert_eq!(cols[2], 8);
    }

    #[test]
    fn test_get_galaxies() {
        let galaxies = super::get_galaxies(&get_test_input());
        assert_eq!(galaxies.len(), 9);
        assert_eq!(galaxies[0], common::Point { x: 3, y: 0 });
        assert_eq!(galaxies[1], common::Point { x: 7, y: 1 });
        assert_eq!(galaxies[2], common::Point { x: 0, y: 2 });
        assert_eq!(galaxies[3], common::Point { x: 6, y: 4 });
    }

    #[test]
    fn test_expand_universe() {
        let mut galaxies = super::get_galaxies(&get_test_input());
        let expanding_cols = super::get_expanding_cols(&get_test_input());
        let expanding_rows = super::get_expanding_rows(&get_test_input());
        super::expand_universe(&mut galaxies, &expanding_cols, &expanding_rows, 2);
        assert_eq!(galaxies.len(), 9);
        assert_eq!(galaxies[0], common::Point { x: 4, y: 0 });
        assert_eq!(galaxies[1], common::Point { x: 9, y: 1 });
        assert_eq!(galaxies[2], common::Point { x: 0, y: 2 });
        assert_eq!(galaxies[3], common::Point { x: 8, y: 5 });
    }

    #[test]
    fn test_manhattan_distance_sum() {
        let mut galaxies = super::get_galaxies(&get_test_input());
        let expanding_cols = super::get_expanding_cols(&get_test_input());
        let expanding_rows = super::get_expanding_rows(&get_test_input());
        super::expand_universe(&mut galaxies, &expanding_cols, &expanding_rows, 2);
        assert_eq!(super::manhattan_distance_sum(&galaxies), 374);
    }

    #[test]
    fn test_fast_manhattan_distance_sum() {
        let mut galaxies = super::get_galaxies(&get_test_input());
        let expanding_cols = super::get_expanding_cols(&get_test_input());
        let expanding_rows = super::get_expanding_rows(&get_test_input());
        super::expand_universe(&mut galaxies, &expanding_cols, &expanding_rows, 2);
        assert_eq!(super::fast_manhattan_distance_sum(&galaxies), 374);
    }
}
