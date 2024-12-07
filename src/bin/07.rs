advent_of_code::solution!(7);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Equation {
    parts: Vec<u64>,
    total: u64,
}

pub enum Operation {
    Mult,
    Add,
    Concat,
}

fn order_of_magnitude(n: u64) -> u64 {
    let mut n = n;
    let mut result = 1;
    while n >= 10 {
        n /= 10;
        result *= 10;
    }
    result
}

fn is_valid(parts: &Vec<u64>, total: u64, current: u64, idx: usize, operations: &[Operation]) -> bool {
    if idx == parts.len() {
        return current == total;
    }
    let next_part = parts[idx];


    for operation in operations {
        let new_val = match operation {
            Operation::Add => {
                current + next_part
            }
            Operation::Mult => {
                current * next_part
            }
            Operation::Concat => {
                current * 10 * order_of_magnitude(next_part) + next_part
            }
        };
        if new_val <= total {
            if is_valid(parts, total, new_val, idx+1, operations) {
                return true;
            }
        }
    }
    false
}

fn parse_input(input: &str) -> Vec<Equation> {
    input.lines().filter_map(|line| {
        if line.is_empty() {
            return None;
        }
        let (total_str, parts_str) = line.split_once(": ").unwrap();
        Some(Equation {
            total: total_str.parse().unwrap(),
            parts: parts_str.split(' ').map(|x| x.parse().unwrap()).collect(),
        })
    }).collect()
}


pub fn part_one(input: &str) -> Option<u64> {
    let equations = parse_input(input);
    Some(equations
        .iter()
        .filter_map(|eq| {
            if is_valid(&eq.parts, eq.total, eq.parts[0], 1, &[Operation::Add, Operation::Mult]) {
                Some(eq.total)
            } else {
                None
            }
        })
        .sum()
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let equations = parse_input(input);
    Some(equations
        .iter()
        .filter_map(|eq| {
            if is_valid(&eq.parts, eq.total, eq.parts[0], 1, &[Operation::Add, Operation::Mult, Operation::Concat]) {
                Some(eq.total)
            } else {
                None
            }
        })
        .sum()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}