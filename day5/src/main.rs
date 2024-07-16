use std::fmt::Debug;

use common;

fn main() {
    let input = common::read_lines("./input.txt");

    println!("Part 1: {}", part1(&input));

    let input = common::read_lines("./input.txt");
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &[String]) -> u64 {
    let almanac = get_almanac(input);
    let mut locations: Vec<u64> = Vec::new();
    almanac.seeds.iter().for_each(|seed| {
        locations.push(traverse_almanac(&almanac, *seed));
    });
    let location = locations.iter().min().unwrap();
    *location
}

fn part2(input: &[String]) -> u64 {
    let almanac = get_almanac(input);

    let starts: Vec<u64> = almanac.seeds.iter().skip(0).step_by(2).copied().collect();
    let ranges: Vec<u64> = almanac.seeds.iter().skip(1).step_by(2).copied().collect();

    let mut location = u64::MAX;

    //super inefficient brute force. takes hours.
    starts.iter().zip(ranges.iter()).for_each(|(start, range)| {
        let mut seed = *start;
        println!("start = {}, range = {}", start, range);
        while seed <= *start + *range {
            let new_loc = traverse_almanac(&almanac, seed);
            if new_loc < location {
                location = new_loc;
            }
            seed += 1;
        }
    });

    //let location = locations.iter().min().unwrap();
    location
}

#[derive(Debug, Default)]
struct MapEntry {
    destination: u64,
    source: u64,
    range: u64,
}

type Map = Vec<MapEntry>;

#[derive(Debug, Default)]
struct Almanac {
    seeds: Vec<u64>,
    seed_to_soil: Map,
    soil_to_fertilizer: Map,
    fertilizer_to_water: Map,
    water_to_light: Map,
    light_to_temperature: Map,
    temperature_to_humidity: Map,
    humidity_to_location: Map,
}

fn get_almanac(input: &[String]) -> Almanac {
    let mut almanac: Almanac = Default::default();

    let chunks = input.split(|line| line == "");

    for chunk in chunks {
        if chunk[0].starts_with("seeds:") {
            almanac.seeds = get_seeds(&chunk[0]);
        } else if chunk[0].starts_with("seed-to-soil map:") {
            parse_map(chunk, &mut almanac.seed_to_soil);
        } else if chunk[0].starts_with("soil-to-fertilizer map:") {
            parse_map(chunk, &mut almanac.soil_to_fertilizer);
        } else if chunk[0].starts_with("fertilizer-to-water map:") {
            parse_map(chunk, &mut almanac.fertilizer_to_water);
        } else if chunk[0].starts_with("water-to-light map:") {
            parse_map(chunk, &mut almanac.water_to_light);
        } else if chunk[0].starts_with("light-to-temperature map:") {
            parse_map(chunk, &mut almanac.light_to_temperature);
        } else if chunk[0].starts_with("temperature-to-humidity map:") {
            parse_map(chunk, &mut almanac.temperature_to_humidity);
        } else if chunk[0].starts_with("humidity-to-location map:") {
            parse_map(chunk, &mut almanac.humidity_to_location);
        }
    }

    //println!("{:?}\n", almanac);

    almanac
}

fn parse_map(chunk: &[String], current_map: &mut Map) {
    chunk
        .iter()
        .skip(1)
        .for_each(|line| current_map.push(get_map_entrie(line)));
}

fn get_seeds(line: &str) -> Vec<u64> {
    let mut seeds: Vec<u64> = Vec::new();

    line.split(": ").collect::<Vec<&str>>()[1]
        .split(' ')
        .collect::<Vec<&str>>()
        .iter()
        .for_each(|numberstr| seeds.push(numberstr.parse::<u64>().unwrap()));

    seeds
}

fn get_corresponding(map: &Map, source: u64) -> u64 {
    let mut destination: u64 = source;

    map.iter().for_each(|entry| {
        if (source >= entry.source)
            && (source <= (entry.source + entry.range))
            && (source == destination)
        {
            destination = entry.destination + (source - entry.source);
            // println!(
            //     "source = {} -> destination = {}; entrie source = {}; entrie dest = {}",
            //     source, destination, entry.source, entry.destination
            // );
            return;
        }
    });
    //println!("{} -> {}", source, destination);
    destination
}

fn get_map_entrie(line: &str) -> MapEntry {
    let vec = line.split(' ').collect::<Vec<&str>>();

    MapEntry {
        destination: vec[0].parse::<u64>().unwrap(),
        source: vec[1].parse::<u64>().unwrap(),
        range: vec[2].parse::<u64>().unwrap(),
    }
}

fn traverse_almanac(almanac: &Almanac, seed: u64) -> u64 {
    let soil: u64 = get_corresponding(&almanac.seed_to_soil, seed);
    let fertilizer: u64 = get_corresponding(&almanac.soil_to_fertilizer, soil);
    let water: u64 = get_corresponding(&almanac.fertilizer_to_water, fertilizer);
    let light: u64 = get_corresponding(&almanac.water_to_light, water);
    let temperature: u64 = get_corresponding(&almanac.light_to_temperature, light);
    let humidity: u64 = get_corresponding(&almanac.temperature_to_humidity, temperature);
    let location: u64 = get_corresponding(&almanac.humidity_to_location, humidity);
    location
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {
        let input = get_test_input();
        assert_eq!(super::part1(&input), 35);
    }

    #[test]
    fn test_part2() {
        let input = get_test_input();
        assert_eq!(super::part2(&input), 46);
    }

    #[test]
    fn test_get_seeds() {
        let line: &str = "seeds: 79 14 55 13";
        assert_eq!(super::get_seeds(line)[0], 79);
        assert_eq!(super::get_seeds(line)[1], 14);
        assert_eq!(super::get_seeds(line)[2], 55);
        assert_eq!(super::get_seeds(line)[3], 13);
    }

    #[test]
    fn test_get_map_entrie() {
        let line: &str = "50 98 2";
        assert_eq!(super::get_map_entrie(line).destination, 50);
        assert_eq!(super::get_map_entrie(line).source, 98);
        assert_eq!(super::get_map_entrie(line).range, 2);
    }

    #[test]
    fn test_get_corresponding() {
        let map: Vec<super::MapEntry> = vec![
            super::MapEntry {
                destination: 50,
                source: 98,
                range: 2,
            },
            super::MapEntry {
                destination: 52,
                source: 50,
                range: 48,
            },
        ];
        assert_eq!(super::get_corresponding(&map, 10), 10); //because there is no entry for 10
        assert_eq!(super::get_corresponding(&map, 100), 52); //first entry that matches
        assert_eq!(super::get_corresponding(&map, 60), 62);

        let f2w: Vec<super::MapEntry> = vec![
            super::MapEntry {
                destination: 49,
                source: 53,
                range: 8,
            },
            super::MapEntry {
                destination: 0,
                source: 11,
                range: 42,
            },
            super::MapEntry {
                destination: 42,
                source: 0,
                range: 7,
            },
            super::MapEntry {
                destination: 57,
                source: 7,
                range: 4,
            },
        ];

        assert_eq!(super::get_corresponding(&f2w, 81), 81); //because there is no entry for 10
        assert_eq!(super::get_corresponding(&f2w, 53), 49);
        assert_eq!(super::get_corresponding(&f2w, 57), 53);
        assert_eq!(super::get_corresponding(&f2w, 52), 41);
    }

    #[test]
    fn test_traverse_almanac() {
        let input = get_test_input();
        let almanac = super::get_almanac(&input);
        assert_eq!(super::traverse_almanac(&almanac, 79), 82);
        assert_eq!(super::traverse_almanac(&almanac, 14), 43);
    }

    fn get_test_input() -> Vec<String> {
        vec![
            "seeds: 79 14 55 13".to_string(),
            "".to_string(),
            "seed-to-soil map:".to_string(),
            "50 98 2".to_string(),
            "52 50 48".to_string(),
            "".to_string(),
            "soil-to-fertilizer map:".to_string(),
            "0 15 37".to_string(),
            "37 52 2".to_string(),
            "39 0 15".to_string(),
            "".to_string(),
            "fertilizer-to-water map:".to_string(),
            "49 53 8".to_string(),
            "0 11 42".to_string(),
            "42 0 7".to_string(),
            "57 7 4".to_string(),
            "".to_string(),
            "water-to-light map:".to_string(),
            "88 18 7".to_string(),
            "18 25 70".to_string(),
            "".to_string(),
            "light-to-temperature map:".to_string(),
            "45 77 23".to_string(),
            "81 45 19".to_string(),
            "68 64 13".to_string(),
            "".to_string(),
            "temperature-to-humidity map:".to_string(),
            "0 69 1".to_string(),
            "1 0 69".to_string(),
            "".to_string(),
            "humidity-to-location map:".to_string(),
            "60 56 37".to_string(),
            "56 93 4".to_string(),
        ]
    }
}
