
fn main() {
    let input = include_str!("./input_1.txt");

    let output = part_1(input);
    dbg!(output);
}

fn part_1(input: &str) -> u32 {
    let mut output = Vec::<String>::new();
    for line in input.lines() {
        output.push(resolve_line(line));
    }

    output.into_iter().map(|ele| ele.parse::<u32>().unwrap()).sum()
}

fn resolve_line(input: &str) -> String {
    use regex::Regex;

    let re = Regex::new(r"[[:digit:]]")
        .expect("RegEx could not be created.");

    let digits : Vec<u32> = re.find_iter(input)
        .map(|m| m.as_str().parse::<u32>().unwrap())
        .collect();

    format!("{}{}", digits.first().unwrap(), digits.last().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_is_ok() {
        let input = 
            "1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet";

        let res = part_1(input);

        assert_eq!(res, 142);        
    }
}