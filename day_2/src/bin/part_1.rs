use colored::Colorize;

const LIMIT: (i32, i32, i32) = (12, 13, 14);

fn main() {
    let input = include_str!("./input_1.txt");

    let output = part_1(input);
    dbg!(output);
}

fn part_1(input: &str) -> u32 {
    let mut output = Vec::<u32>::new();
    for line in input.lines() {
        let (game, is_ok) = resolve_line(line);
        output.push(if is_ok { game } else { 0 });
    }

    output.iter().sum()
}

fn resolve_line(input: &str) -> (u32, bool) {
    let game_data: Vec<&str> = input.split(":").collect();
    let game_id = game_data[0].strip_prefix("Game ").unwrap().parse().unwrap();

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

        if sums.0 > LIMIT.0 || sums.1 > LIMIT.1 || sums.2 > LIMIT.2 {
            println!(
                "Game Id: {}\n Sets: {}",
                game_id,
                format!("{:?}", sums).red()
            );
            return (game_id, false);
        } else {
            println!("{}", format!("{:?}", sums).green());
        }
    }
    (game_id, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_is_ok() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let res = part_1(input);

        assert_eq!(res, 8);
    }
}
