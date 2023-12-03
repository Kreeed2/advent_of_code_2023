use colored::Colorize;

fn main() {
    let input = include_str!("./input_1.txt");

    let output = part_2(input);
    dbg!(output);
}

fn part_2(input: &str) -> i32 {
    let mut output = Vec::<i32>::new();
    for line in input.lines() {
        let (_game, minimal_amount) = resolve_line(line);
        let power = minimal_amount.0 * minimal_amount.1 * minimal_amount.2;
        println!("POWER: {}", power);
        output.push(power);
    }

    output.iter().sum()
}

fn resolve_line(input: &str) -> (u32, (i32, i32, i32)) {
    let game_data: Vec<&str> = input.split(":").collect();
    let game_id = game_data[0].strip_prefix("Game ").unwrap().parse().unwrap();

    let mut min_amount = (i32::MIN, i32::MIN, i32::MIN);

    println!("Game Id: {}\nSets:", game_id);
    for set in game_data[1].split(";") {
        let sums = set
            .split(',')
            .map(|ele| ele.trim().split_once(" ").unwrap())
            .fold((0, 0, 0), |acc, nex| match nex.1 {
                "red" => (acc.0 + nex.0.parse::<i32>().unwrap(), acc.1, acc.2),
                "green" => (acc.0, acc.1 + nex.0.parse::<i32>().unwrap(), acc.2),
                "blue" => (acc.0, acc.1, acc.2 + nex.0.parse::<i32>().unwrap()),
                &_ => panic!("not a valid color."),
            });

        println!("{}", format!("{:?}", sums).green());
        if sums.0 > min_amount.0 {
            min_amount.0 = sums.0;
        } 
        if sums.1 > min_amount.1 {
            min_amount.1 = sums.1;
        } 
        if sums.2 > min_amount.2 {
            min_amount.2 = sums.2;
        }
    }
    println!("Minimal set: {:?}", min_amount);
    (game_id, min_amount)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_2_is_ok() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let res = part_2(input);

        assert_eq!(res, 2286);
    }
}
