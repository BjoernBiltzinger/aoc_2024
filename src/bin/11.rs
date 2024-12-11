use cached::{proc_macro::cached, Cached};

advent_of_code::solution!(11);



// impl<'a> Stone<'a> {
//     fn blink(&mut self) {
//         if self.value == 0{
//             self.value = 1;
//         } else if self.value.to_string().len() % 2 == 0{
//             let string_number = self.value.to_string();
//             let split_idx = string_number.len() / 2;
//             let (left, right) = string_number.split_at(split_idx);
//             let left = left.parse::<u64>().unwrap();
//             let right = right.parse::<u64>().unwrap();
//             let new_stone = Stone{value: right, left_neighbour: Some(&Box::new(self)), right_neighbour: self.right_neighbour};
//             self.value = left;
//             self.left_neighbour = Some(&Box::new(&new_stone));
//         } else {
//             self.value *= 2024;
//         }
//     }
// }

#[cached(name = "MY_CUSTOM_CACHE")]
fn blink (value: u64, cycle: usize, num_cycles: usize) -> u64 {
    if cycle == num_cycles {
        return 1;
    }

    let mut final_states= 0;

    if value == 0 {
        final_states += blink(1, cycle+1, num_cycles);
    } else if value.to_string().len() % 2 == 0 {
        let string_number = value.to_string();
        let split_idx = string_number.len() / 2;
        let (left, right) = string_number.split_at(split_idx);
        let left = left.parse::<u64>().unwrap();
        let right = right.parse::<u64>().unwrap();
        final_states += blink(left, cycle+1, num_cycles);
        final_states += blink(right, cycle+1, num_cycles);
    } else {
        final_states += blink(value*2024, cycle+1, num_cycles);
    }
    final_states
}


fn parse_input(input: &str) -> Vec<u64> {
    input.split_ascii_whitespace().map(|str_number| {
        str_number.parse::<u64>().unwrap()
    }).collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let stones = parse_input(input);
    MY_CUSTOM_CACHE.lock().unwrap().cache_reset();
    Some(stones.iter().fold(0, |acc, stone| {
        acc + blink(*stone, 0, 25)
    }))
}

pub fn part_two(input: &str) -> Option<u64> {
    let stones = parse_input(input);
    MY_CUSTOM_CACHE.lock().unwrap().cache_reset();
    Some(stones.iter().fold(0, |acc, stone| {
        acc + blink(*stone, 0, 75)
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
