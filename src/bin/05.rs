use std::collections::HashMap;

advent_of_code::solution!(5);
fn parse_input(input: &str) -> (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>) {
    // split input on empty line
    let string_matrix: Vec<&str> = input.split("\n\n").collect();
    if string_matrix.len() != 2 {
        panic!("Invalid input");
    }
    let mut rule_map: HashMap<u32, Vec<u32>> = HashMap::new();
    for line in string_matrix[0].lines() {
        let mut parts = line.split("|").map(|x| x.parse().ok().unwrap());
        let first = parts.next().unwrap();
        let second = parts.next().unwrap();
        rule_map.entry(second).or_insert(vec![]).push(first);
    }
    let messages = string_matrix[1].lines().map(|line| {
        line.split(",").map(|x| x.parse().ok().unwrap()).collect()
    }).collect();
    (rule_map, messages)
}

fn get_middle_value(input: &Vec<u32>) -> u32 {
    input[input.len()/2]
}

fn check_if_valid(input: &Vec<u32>, rule_map: &HashMap<u32, Vec<u32>>) -> bool {
    for (idx, value) in input.iter().enumerate() {
        let rules = rule_map.get(value);
        if rules.is_none() {
            continue;
        }

        for j in idx..input.len() {
            if rules.unwrap().contains(&input[j]) {
                return false;
            }
        }
    }
    true
}

fn sort(input: &Vec<u32>, rule_map: &HashMap<u32, Vec<u32>>) -> bool {
    let value;
    for (idx, value) in input.iter().enumerate() {
        let rules = rule_map.get(value);
        if rules.is_none() {
            continue;
        }

        for j in idx..input.len() {
            if rules.unwrap().contains(&input[j]) {
                return false;
            }
        }
    }
    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rule_map, messages) = parse_input(input);
    Some(messages.iter().fold(0, |acc, message| {
        if check_if_valid(message, &rule_map) {
            acc + get_middle_value(message)
        } else {
            acc
        }
    }))
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
