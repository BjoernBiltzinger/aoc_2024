advent_of_code::solution!(7);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Equation {
    parts: Vec<u64>,
    total: u64,
}

impl Equation {
    fn is_valid(&self) -> bool {
        let mut potential_total = vec![self.parts[0]];
        for part in self.parts.iter().skip(1) {
            let mut new_values = Vec::new();
            for value in potential_total.iter() {
                // either + or *
                let plus_value = value + part;
                let times_value = value * part;

                if plus_value == self.total || times_value == self.total {
                    return true;
                }
                if plus_value < self.total {
                    new_values.push(plus_value);
                }
                if times_value < self.total {
                    new_values.push(times_value);
                }
            }
            if new_values.is_empty() {
                return false;
            }
            potential_total = new_values;
        }
        false
    }
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

fn is_valid_2(parts: &Vec<u64>, total: u64, current: u64, idx: usize) -> bool {
    if idx == parts.len() {
        return current == total;
    }
    let next_part = parts[idx];

    // three options - either +, *, or concat next two values
    let plus_value = current + next_part;
    if plus_value <= total {
        if is_valid_2(parts, total, plus_value, idx+1){
            return true;
        }
    }
    let times_value = current * next_part;
    if times_value <= total {
        if is_valid_2(parts, total, times_value, idx+1) {
            return true;
        }
    }
    let concat_value = current * 10 * order_of_magnitude(next_part) + next_part;
    if is_valid_2(parts, total, concat_value, idx+1) {
        return true;
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
            if eq.is_valid() {
                Some(eq.total)
            } else {
                None
            }
        })
        .sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let equations = parse_input(input);
    Some(equations
        .iter()
        .filter_map(|eq| {
            if is_valid_2(&eq.parts, eq.total, eq.parts[0], 1) {
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