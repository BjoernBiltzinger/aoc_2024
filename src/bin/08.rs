use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

type Position = (i32, i32);

fn parse_input(input: &str) -> (HashMap<char, Vec<Position>>, usize, usize) {
    let mut positions: HashMap<char, Vec<Position>> = HashMap::new();
    let x_len = input.lines().next().unwrap().len();
    let y_len = input.lines().count();
    input
    .lines()
    .enumerate()
    .for_each(|(idy, line)| {
        if line.is_empty() {
            return;
        }
        line.chars().enumerate().for_each(|(idx, char)| {
            match char {
                '.' => {},
                _ => {
                    let vec = positions.entry(char).or_insert(Vec::new());
                    vec.push((idx as i32, idy as i32));
                }
            }
        });
    });
    (positions, x_len, y_len)
}

fn simplify_ratio(a: i32, b: i32) -> (i32, i32) {
    let gcd = {
        let mut x = a.abs();
        let mut y = b.abs();
        while y != 0 {
            let temp = y;
            y = x % y;
            x = temp;
        }
        x
    };
    
    (a / gcd, b / gcd)
}

fn get_antinodes(positions: &HashMap<char, Vec<Position>>, x_len: usize, y_len: usize, part_2: bool) -> HashSet<Position> {
    let mut result = HashSet::new();
    for (_, vec) in positions {
        // get all the pairs of positions
        for first_index in 0..vec.len() {
            for second_index in first_index+1..vec.len() {
                // get vector from first to second
                let first = vec[first_index];
                let second = vec[second_index];
                let x = second.0 - first.0;
                let y = second.1 - first.1;
                if !part_2{
                    let antidote_1 = (first.0 - x, first.1 - y);
                    let antidote_2 = (second.0 + x, second.1 + y);
                    if antidote_1.0 >= 0 && antidote_1.0 < x_len as i32 && antidote_1.1 >= 0 && antidote_1.1 < y_len as i32 {
                        result.insert(antidote_1);
                    }
                    if antidote_2.0 >= 0 && antidote_2.0 < x_len as i32 && antidote_2.1 >= 0 && antidote_2.1 < y_len as i32 {
                        result.insert(antidote_2);
                    }
                } else {
                    let (x, y) = simplify_ratio(x, y);
                    let mut i = 0;
                    loop {
                        let antidote = (first.0 + x*i, first.1 + y*i);
                        if antidote.0 >= 0 && antidote.0 < x_len as i32 && antidote.1 >= 0 && antidote.1 < y_len as i32 {
                            result.insert(antidote);
                        } else {
                            break;
                        }
                        i += 1;
                    }
                    let mut i = 0;
                    loop {
                        let antidote = (first.0 - x*i, first.1 - y*i);
                        if antidote.0 >= 0 && antidote.0 < x_len as i32 && antidote.1 >= 0 && antidote.1 < y_len as i32 {
                            result.insert(antidote);
                        } else {
                            break;
                        }
                        i += 1;
                    }
                }
            }
        }
    }
    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let (positions, x_len, y_len) = parse_input(input);
    let antinodes = get_antinodes(&positions, x_len, y_len, false);
    Some(antinodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (positions, x_len, y_len) = parse_input(input);
    let antinodes = get_antinodes(&positions, x_len, y_len, true);
    Some(antinodes.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
