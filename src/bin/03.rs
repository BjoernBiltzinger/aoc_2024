use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    Some(re.captures_iter(input).map(|cap| {
        cap[1].parse::<u32>().unwrap() * cap[2].parse::<u32>().unwrap()
    }).sum())
}

fn is_active(pos: usize, do_pos: &Vec<usize>, dont_pos: &Vec<usize>) -> bool {
    // check if the closest do is closer than the closest dont only consider the ones before the current position
    match (do_pos.iter().filter(|&&x| x < pos).max(), dont_pos.iter().filter(|&&x| x < pos).max()) {
        (Some(do_val), Some(dont_val)) => do_val > dont_val,
        (Some(_), None) => true,
        (None, None) => true,
        (None, Some(_)) => false,
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let do_pos = Regex::new(r"do\(\)").unwrap().find_iter(input).map(|m| m.start()).collect::<Vec<_>>();
    let dont_pos = Regex::new(r"don't\(\)").unwrap().find_iter(input).map(|m| m.start()).collect::<Vec<_>>();
    Some(Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap().captures_iter(input).map(|cap| {
        if is_active(cap.get(0).unwrap().start(), &do_pos, &dont_pos) {
            cap[1].parse::<u32>().unwrap() * cap[2].parse::<u32>().unwrap()
        } else {
            0
        }
    }).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY,1));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY,2));
        assert_eq!(result, Some(48));
    }
}
