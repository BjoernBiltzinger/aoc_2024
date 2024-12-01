use std::collections::HashMap;

advent_of_code::solution!(1);

fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(|line| {
            let mut values = line.split_ascii_whitespace().map(|x| x.parse().ok());
            let x: u32 = values.next().unwrap().unwrap();
            let y: u32 = values.next().unwrap().unwrap();
            (x, y)
        })
        .unzip()
}

pub fn part_one(input: &str) -> Option<u32> {
    //let mut vec1 = Vec::new();
    //let mut vec2 = Vec::new();

    let (mut vec1, mut vec2): (Vec<u32>, Vec<u32>) = parse_input(input);

    vec1.sort_unstable();
    vec2.sort_unstable();
    Some(
        vec1.into_iter()
            .zip(vec2)
            .fold(0, |acc, (a, b)| acc + a.abs_diff(b)),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (vec1, vec2): (Vec<u32>, Vec<u32>) = parse_input(input);

    let freq_map = vec2.into_iter().fold(HashMap::new(), |mut map, num| {
        *map.entry(num).or_insert(0) += 1;
        map
    });

    Some(vec1.into_iter().fold(0, |acc, num| acc + num * freq_map.get(&num).unwrap_or(&0)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
