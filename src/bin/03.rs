use regex::RegexBuilder;

advent_of_code::solution!(3);


pub fn part_one(input: &str) -> Option<u32> {
    Some(RegexBuilder::new(r"mul\((\d{1,3}),(\d{1,3})\)").unicode(false).build().unwrap().captures_iter(input).map(|cap| {
        cap[1].parse::<u32>().unwrap() * cap[2].parse::<u32>().unwrap()
    }).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        RegexBuilder::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)")
        .unicode(false)
        .build()
        .unwrap()
        .captures_iter(input)
        .fold((0, true), |prev, cap| 
            if cap.get(0).unwrap().as_str() == "do()" {
                (prev.0, true)
            } else if cap.get(0).unwrap().as_str() == "don't()" {
                (prev.0, false)
            } else {
                if prev.1 {
                    (prev.0 + cap[1].parse::<u32>().unwrap() * cap[2].parse::<u32>().unwrap(), prev.1)
                } else {
                    (prev.0, prev.1)
                }
            })
        .0)
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
