fn main() {
    let input = include_str!("./input_1.txt");

    let output = part_2(input);
    dbg!(output);
}

fn part_2(input: &str) -> u32 {
    let mut output = Vec::<String>::new();
    for line in input.lines() {
        output.push(resolve_line(line));
    }

    output
        .into_iter()
        .map(|ele| ele.parse::<u32>().unwrap())
        .sum()
}

fn resolve_line(input: &str) -> String {
    let mut digits = Vec::<u32>::new();

    let str_lenth = input.len();
    for start in 0..str_lenth {
        for end in start..str_lenth+1 {
            let var_name = &input[start..end];
            match var_name {
                "1" | "one" => { digits.push(1); break;},
                "2" | "two" => { digits.push(2); break; },
                "3" | "three" => { digits.push(3); break; },
                "4" | "four" => { digits.push(4); break; },
                "5" | "five" => { digits.push(5); break; },
                "6" | "six" => { digits.push(6); break; },
                "7" | "seven" => { digits.push(7); break; },
                "8" | "eight" => { digits.push(8); break; },
                "9" | "nine" => { digits.push(9); break; },
                &_ => (),
            };
        }
    }

    println!("Digits for {} are {:?}", input, digits);
    format!("{}{}", digits.first().unwrap(), digits.last().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_2_is_ok() {
        let input = 
"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        let res = part_2(input);
        assert_eq!(res, 281);
    }
}
