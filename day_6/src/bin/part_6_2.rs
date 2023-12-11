use itertools::Itertools;

fn main() {
    let input = include_str!("./input_1.txt");

    let output = part_2(input);
    dbg!(output);
}

fn part_2(input: &str) -> u64 {
    let (time, distance) = parse_input(input);
    
    solve_for_duration(time, distance)
        .len() as u64
}

fn solve_for_duration(time: u64, distance: u64) -> Vec<u64> {
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

    (lower_bound.ceil()as u64..=upper_bound.floor() as u64).collect_vec()
}

fn parse_input(input: &str) -> (u64, u64) {
    let mut input = input.lines();

    let time: u64 = input
        .next()
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .trim()
        .replace(" ", "")
        .parse()
        .unwrap();

    let distance: u64 = input
        .next()
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .trim()
        .replace(" ", "")
        .parse()
        .unwrap();

    (time, distance)
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

        assert_eq!(res, 71503);
    }

    #[test]
    fn parse_input_returns_expected_values() {
        let input = 
"Time:      7  15   30
Distance:  9  40  200";

        let expected = (71530, 940200);
        let result = parse_input(input);

        assert_eq!(result, expected);
    }
    
    #[test]
    fn solve_for_duration_returns_expected_values() {
        let time = 71530;
        let distance = 940200;
        let expected = (14..=71516).collect_vec();
        let result = solve_for_duration(time, distance);
        assert_eq!(result, expected);
    }
}
