use itertools::Itertools;

fn main() {
    let input = include_str!("./input_1.txt");

    let output = part_2(input);
    dbg!(output);
}

fn part_2(input: &str) -> u32 {
    parse_input(input)
        .into_iter()
        .map(|(time, distance)| solve_for_duration(time, distance))
        .map(|solutions| solutions.len() as u32)
        .fold(1, |acc, nxt| acc * nxt)
}

fn solve_for_duration(time: u32, distance: u32) -> Vec<u32> {
    let time = time as f64;
    let distance = distance as f64;

    let mut lower_bound = (time - (time.powf(2.0) - 4.0 * distance).sqrt()) / 2.0;
    let mut upper_bound = (time + (time.powf(2.0) - 4.0 * distance).sqrt()) / 2.0;

    if lower_bound.fract() == 0.0 {
        lower_bound += 1.0;
    }
    if upper_bound.fract() == 0.0 {
        upper_bound -= 1.0;
    }

    (lower_bound.ceil()as u32..=upper_bound.floor() as u32).collect_vec()
}

fn parse_input(input: &str) -> Vec<(u32, u32)> {
    let mut input = input.lines();

    let time: Vec<u32> = input
        .next()
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let distance: Vec<u32> = input
        .next()
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    time.into_iter().zip(distance.into_iter()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_2_is_ok() {
        let input = 
"Time:      7  15   30
Distance:  9  40  200";

        let res = part_2(input);

        assert_eq!(res, 288);
    }

    #[test]
    fn parse_input_returns_expected_values() {
        let input = 
"Time:      7  15   30
Distance:  9  40  200";

        let expected = vec![(7, 9), (15, 40), (30, 200)];
        let result = parse_input(input);

        assert_eq!(result, expected);
    }
    
    #[test]
    fn solve_for_duration_returns_expected_values() {
        let time = 7;
        let distance = 9;
        let expected = vec![2,3,4,5];
        let result = solve_for_duration(time, distance);
        assert_eq!(result, expected);
    }
}
