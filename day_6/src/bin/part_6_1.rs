
fn main() {
    let input = include_str!("./input_1.txt");

    let output = part_2(input);
    dbg!(output);
}

fn part_2(input: &str) -> u32 {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_2_is_ok() {
        let input = 
            "1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet";

        let res = part_2(input);

        assert_eq!(res, 142);        
    }
}