use itertools::Itertools;

fn main() {
    let input = include_str!("./input_1.txt");

    let output = part_2(input);
    dbg!(output);
}

fn split_by_empty_lines(input: &str) -> Vec<&str> {
    input.split("\n\n").collect()
}

fn parse_input(input: &str) -> Vec<(u32, u32, u32)> {
    let mut map: Vec<(u32, u32, u32)> = Vec::new();

    let parts: Vec<&str> = input.split(":").collect();
    for line in parts[1].trim().lines() {
        let numbers: Vec<i64> = line
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();

        // Given the input, the first number is the destination, the second is the source and the third is the length
        map.push((numbers[1] as u32, numbers[0] as u32, numbers[2] as u32));
    }
    map.sort_by_key(|tupl| tupl.0);

    map
}

fn check_key(map: &Vec<(u32, u32, u32)>, x: u32) -> u32 {
    for (src, dest, len) in map {
        if x >= *src && x <= (*src + *len - 1) {
            return *dest + (x - *src);
        }
    }
    x
}

fn part_2(input: &str) -> u32 {
    let mut maps_as_string = split_by_empty_lines(input);

    let seeds = maps_as_string
        .remove(0)
        .strip_prefix("seeds:")
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|seed| seed.parse::<u32>().unwrap());

    let maps = maps_as_string
        .into_iter()
        .map(|map| parse_input(map))
        .collect::<Vec<Vec<(u32, u32, u32)>>>();

    let converted_seed_numbers: Vec<u32> = seeds.clone()
        .tuples()
        .map(|(seed_num, len)| (seed_num..seed_num + len).collect::<Vec<u32>>())
        .flatten()
        .map(|seed_num| {
            maps.clone()
                .into_iter()
                .fold(seed_num, |acc, nxt| check_key(&nxt, acc))
        })
        .collect();

    converted_seed_numbers.into_iter().min().unwrap()
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
        let expected_output: Vec<(u32, u32, u32)> = vec![(50, 52, 48), (98, 50, 2)];
        assert_eq!(parse_input(&input), expected_output);
    }

    #[test]
    fn test_check_key_existing() {
        let map: Vec<(u32, u32, u32)> = vec![(50, 52, 48), (98, 50, 2)];
        let x = 79;
        let expected_output = 81;
        assert_eq!(check_key(&map, x), expected_output);
    }

    #[test]
    fn test_check_key_non_existing() {
        let map: Vec<(u32, u32, u32)> = vec![(50, 52, 48), (98, 50, 2)];
        let x = 4;
        let expected_output = 4;
        assert_eq!(check_key(&map, x), expected_output);
    }

    #[test]
    fn test_part_2() {
        let expected_output = 46;
        assert_eq!(part_2(INPUT), expected_output);
    }
}
