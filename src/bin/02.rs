advent_of_code::solution!(2);

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            let values = line.split_ascii_whitespace().map(|x| x.parse().ok().unwrap()).collect();
            values
        })
        .collect()
}

fn check_valid(vec: &Vec<u32>, skip: usize) -> bool {
    let increase: bool;

    let start = if skip == 0 { 1 } else { 0 };
    let second = if skip == 1 || skip == 0 { 2 } else { 1 };  

    if vec[second] < vec[start] {
        increase = false;
    } else if vec[second] > vec[start] {
        increase = true;    
    } else {
        return false;
    }

    let mut current_value = vec[start];

    for (i,value) in vec.iter().enumerate() {
        if i == skip || i == start {
            continue;
        }

        if increase && (*value <= current_value || value - current_value > 3) {
            return false;
        }
        if !increase && (*value >= current_value || current_value - value > 3) {
            return false;
        }
        current_value = *value;
    }
    true
}

fn check_valid_part2(vec: &Vec<u32>) -> bool {
    // the 10000 is a hack to avoid skipping an element - was too lazy to write this properly
    if check_valid(vec, 10000) {
        return true;
    }
    for i in 0..vec.len() {
        // drop i-th element

        // first version with copy - new version avoids copying 
        //let mut vec_copy = vec.clone();
        //vec_copy.remove(i);
        if check_valid(vec, i) {
            return true;
        }
    }
    false
}

pub fn part_one(input: &str) -> Option<u32> {
    let vecs: Vec<Vec<u32>> = parse_input(input);

    Some(vecs.iter().filter(|vec| check_valid(vec, 10000)).count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let vecs: Vec<Vec<u32>> = parse_input(input);

    Some(vecs.iter().filter(|vec| check_valid_part2(vec)).count() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
