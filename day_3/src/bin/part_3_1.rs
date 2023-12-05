fn is_symbol(chr: &char) -> bool {
    if chr.is_numeric() {
        false
    } else if chr == &'.' {
        false
    } else {
        true
    }
}

fn get_neighbor_coord(pos: usize, line_width: usize) -> Vec<usize> {
    // Map 3D to 2D
    vec![pos-line_width-1, pos-line_width, pos-line_width+1,
         pos-1,                            pos+1,
         pos+line_width-1, pos+line_width, pos+line_width+1]
}

fn complete_number(chars: &Vec<char>, pos: usize, line_width: usize) -> u32 {
    let cur_line = pos / line_width;
    let mut number: Vec<char> = Vec::new();
    
    // Left arm
    for left_pos in (cur_line*line_width..pos).rev() {
        let cur_chr = chars[left_pos as usize];

        if cur_chr.is_numeric() {
            number.push(cur_chr);
        } else {
            break;
        }
    }
    number.reverse();
    // Middle has to be number, because it was found as a neighbor of a symbol
    number.push(chars[pos as usize]);
    // Right arm
    for right_pos in pos+1..(cur_line + 1)*line_width {
        let cur_chr = chars[right_pos as usize];

        if cur_chr.is_numeric() {
            number.push(cur_chr);
        } else {
            break;
        }
    }

    number.iter().collect::<String>().parse().unwrap()
}

fn is_neighbor_number(neighbor_chars: Vec<(usize, char)>) -> Vec<(usize, char)> {
    // [0 1 2
    //  3 _ 4
    //  5 6 7]
    let mut output: Vec<(usize, char)> = Vec::new();

    if neighbor_chars[0].1.is_numeric() && !neighbor_chars[1].1.is_numeric() && !neighbor_chars[2].1.is_numeric() {
        output.push(neighbor_chars[0]);
    } else if neighbor_chars[0].1.is_numeric() && neighbor_chars[1].1.is_numeric() && !neighbor_chars[2].1.is_numeric() {
        output.push(neighbor_chars[0]);
    } else if neighbor_chars[0].1.is_numeric() && neighbor_chars[1].1.is_numeric() && neighbor_chars[2].1.is_numeric(){
        output.push(neighbor_chars[0]);
    } else if !neighbor_chars[0].1.is_numeric() && neighbor_chars[1].1.is_numeric() && neighbor_chars[2].1.is_numeric() {
        output.push(neighbor_chars[1]);
    } else if !neighbor_chars[0].1.is_numeric() && neighbor_chars[1].1.is_numeric() && !neighbor_chars[2].1.is_numeric() {
        output.push(neighbor_chars[1]);
    } else if !neighbor_chars[0].1.is_numeric() && !neighbor_chars[1].1.is_numeric() && neighbor_chars[2].1.is_numeric() {
        output.push(neighbor_chars[2]);
    } else if neighbor_chars[0].1.is_numeric() && !neighbor_chars[1].1.is_numeric() && neighbor_chars[2].1.is_numeric() {
        output.push(neighbor_chars[0]);
        output.push(neighbor_chars[2]);
    }

    if neighbor_chars[3].1.is_numeric() {
        output.push(neighbor_chars[3]);
    }
    if neighbor_chars[4].1.is_numeric() {
        output.push(neighbor_chars[4]);
    }

    if neighbor_chars[5].1.is_numeric() && !neighbor_chars[6].1.is_numeric() && !neighbor_chars[7].1.is_numeric() {
        output.push(neighbor_chars[5]);
    } else if neighbor_chars[5].1.is_numeric() && neighbor_chars[6].1.is_numeric() && !neighbor_chars[7].1.is_numeric() {
        output.push(neighbor_chars[5]);
    } else if neighbor_chars[5].1.is_numeric() && neighbor_chars[6].1.is_numeric() && neighbor_chars[7].1.is_numeric() {
        output.push(neighbor_chars[5]);
    } else if !neighbor_chars[5].1.is_numeric() && neighbor_chars[6].1.is_numeric() && neighbor_chars[7].1.is_numeric() {
        output.push(neighbor_chars[6]);
    } else if !neighbor_chars[5].1.is_numeric() && neighbor_chars[6].1.is_numeric() && !neighbor_chars[7].1.is_numeric() {
        output.push(neighbor_chars[6]);
    } else if !neighbor_chars[5].1.is_numeric() && !neighbor_chars[6].1.is_numeric() && neighbor_chars[7].1.is_numeric() {
        output.push(neighbor_chars[7]);
    } else if neighbor_chars[5].1.is_numeric() && !neighbor_chars[6].1.is_numeric() && neighbor_chars[7].1.is_numeric() {
        output.push(neighbor_chars[5]);
        output.push(neighbor_chars[7]);
    }  

    output
}

fn main() {
    let input = include_str!("./input_1.txt");

    let output = part_1(input);
    dbg!(output);
}

fn part_1(input: &str) -> u32 {
    let line_width = input.find("\n").unwrap();
    println!("The line is {} long.", line_width);

    let chars: Vec<char> = input.chars().filter(|chr| chr != &'\n').collect();

    let symbol_points: u32 = chars.clone().into_iter()
        .enumerate()
        .filter(|(_pos, chr)| is_symbol(chr))
        .map(|(pos, _chr)| get_neighbor_coord(pos, line_width))
        .map(|neighbor_cords| {
            neighbor_cords.into_iter()
                .map(|pos| (pos, chars[pos]))
        })
        .map(|itr| is_neighbor_number(itr.collect()))
        .map(|vec| vec.into_iter().map(|(pos, _chr)| complete_number(&chars, pos, line_width)).collect::<Vec<u32>>())
        .flatten()    
        .sum();

    symbol_points
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_is_ok() {
        let input = 
"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let outout = part_1(input);

        assert_eq!(outout, 4361);
    }
}