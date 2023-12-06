use std::collections::HashMap;
fn main() {
    let input = include_str!("./input_1.txt");

    let output = part_1(input);
    dbg!(output);
}

fn split_by_empty_lines(input: &str) -> Vec<&str> {
    input.split("\n\n").collect()
}

fn parse_input(input: &str) -> HashMap<i64, i64> {
    let mut map: HashMap<i64, i64> = HashMap::new();

    let parts: Vec<&str> = input.split(":").collect();
    for line in parts[1].trim().lines() {
        let numbers: Vec<i64> = line
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();

        for i in 0..numbers[2] {
            map.insert(numbers[1] + i, numbers[0] + i);
        }
    }
    map
}

fn check_key(map: &HashMap<i64, i64>, x: i64) -> i64 {
    if let Some(&value) = map.get(&x) {
        value
    } else {
        x
    }
}

fn part_1(input: &str) -> u32 {
    let mut maps_as_string = split_by_empty_lines(input);

    let seeds = maps_as_string.remove(0);
    let maps = maps_as_string
        .into_iter()
        .map(|map| parse_input(map))
        .collect::<Vec<HashMap<i64, i64>>>();

    let converted_seed_numbers: Vec<i64> = seeds
        .strip_prefix("seeds:")
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|seed| seed.parse::<i64>().unwrap())
        .map(|seed_num| {
            maps.clone()
                .into_iter()
                .fold(seed_num, |acc, nxt| check_key(&nxt, acc))
        })
        .collect();

    converted_seed_numbers.into_iter().min().unwrap() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "seeds: 79 14 55 13\n\nseed-to-soil map:\n50 98 2\n52 50 48\n\nsoil-to-fertilizer map:\n0 15 37\n37 52 2\n39 0 15\n\nfertilizer-to-water map:\n49 53 8\n0 11 42\n42 0 7\n57 7 4\n\nwater-to-light map:\n88 18 7\n18 25 70\n\nlight-to-temperature map:\n45 77 23\n81 45 19\n68 64 13\n\ntemperature-to-humidity map:\n0 69 1\n1 0 69\n\nhumidity-to-location map:\n60 56 37\n56 93 4";

    #[test]
    fn test_split_by_empty_lines() {
        let expected_output = vec![
            "seeds: 79 14 55 13",
            "seed-to-soil map:\n50 98 2\n52 50 48",
            "soil-to-fertilizer map:\n0 15 37\n37 52 2\n39 0 15",
            "fertilizer-to-water map:\n49 53 8\n0 11 42\n42 0 7\n57 7 4",
            "water-to-light map:\n88 18 7\n18 25 70",
            "light-to-temperature map:\n45 77 23\n81 45 19\n68 64 13",
            "temperature-to-humidity map:\n0 69 1\n1 0 69",
            "humidity-to-location map:\n60 56 37\n56 93 4",
        ];
        assert_eq!(split_by_empty_lines(INPUT), expected_output);
    }

    #[test]
    fn test_parse_input() {
        let input = "seed-to-soil map:\n50 98 2\n52 50 48";
        let expected_output: HashMap<i64, i64> = [
            (98, 50),
            (99, 51),
            (50, 52),
            (51, 53),
            (52, 54),
            (53, 55),
            (54, 56),
            (55, 57),
            (56, 58),
            (57, 59),
            (58, 60),
            (59, 61),
            (60, 62),
            (61, 63),
            (62, 64),
            (63, 65),
            (64, 66),
            (65, 67),
            (66, 68),
            (67, 69),
            (68, 70),
            (69, 71),
            (70, 72),
            (71, 73),
            (72, 74),
            (73, 75),
            (74, 76),
            (75, 77),
            (76, 78),
            (77, 79),
            (78, 80),
            (79, 81),
            (80, 82),
            (81, 83),
            (82, 84),
            (83, 85),
            (84, 86),
            (85, 87),
            (86, 88),
            (87, 89),
            (88, 90),
            (89, 91),
            (90, 92),
            (91, 93),
            (92, 94),
            (93, 95),
            (94, 96),
            (95, 97),
            (96, 98),
            (97, 99),
        ]
        .into();
        assert_eq!(parse_input(&input), expected_output);
    }

    #[test]
    fn test_check_key_existing() {
        let map: HashMap<i64, i64> = [
            (98, 50),
            (99, 51),
            (50, 52),
            (51, 53),
            (52, 54),
            (53, 55),
            (54, 56),
            (55, 57),
            (56, 58),
            (57, 59),
            (58, 60),
            (59, 61),
            (60, 62),
            (61, 63),
            (62, 64),
            (63, 65),
            (64, 66),
            (65, 67),
            (66, 68),
            (67, 69),
            (68, 70),
            (69, 71),
            (70, 72),
            (71, 73),
            (72, 74),
            (73, 75),
            (74, 76),
            (75, 77),
            (76, 78),
            (77, 79),
            (78, 80),
            (79, 81),
            (80, 82),
            (81, 83),
            (82, 84),
            (83, 85),
            (84, 86),
            (85, 87),
            (86, 88),
            (87, 89),
            (88, 90),
            (89, 91),
            (90, 92),
            (91, 93),
            (92, 94),
            (93, 95),
            (94, 96),
            (95, 97),
            (96, 98),
            (97, 99),
        ]
        .into();
        let x = 79;
        let expected_output = 81;
        assert_eq!(check_key(&map, x), expected_output);
    }

    #[test]
    fn test_check_key_non_existing() {
        let map: HashMap<i64, i64> = [(1, 10), (2, 20), (3, 30)].iter().cloned().collect();
        let x = 4;
        let expected_output = 4;
        assert_eq!(check_key(&map, x), expected_output);
    }

    #[test]
    fn test_part_1() {
        let expected_output = 35;
        assert_eq!(part_1(INPUT), expected_output);
    }
}
