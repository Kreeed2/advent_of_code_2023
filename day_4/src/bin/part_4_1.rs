
fn main() {
    let input = include_str!("./input_1.txt");

    let output = part_1(input);
    dbg!(output);
}

fn part_1(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        sum += resolve_line(line);
    }
    sum
}

fn resolve_line(input: &str) -> u32 {
    let game_data: Vec<&str> = input.split(":").collect();
    let _game_id: u32 = game_data[0].strip_prefix("Card").unwrap().trim().parse().unwrap();

    let numbers: Vec<Vec<u32>> = game_data[1].split("|")
        .map(|half| {
            half.split_ascii_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        }).collect();

    let numbers_won: Vec<u32> = numbers[1].clone().into_iter().filter(|num| numbers[0].contains(num)).collect();

    if numbers_won.len() == 0 {
        0
    } else {
        2_u32.pow(numbers_won.len() as u32 - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_is_ok() {
        let input = 
"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let outout = part_1(input);

        assert_eq!(outout, 13);
    }
}