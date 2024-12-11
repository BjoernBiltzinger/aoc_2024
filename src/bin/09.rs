use std::collections::HashSet;


advent_of_code::solution!(9);


fn parse_input(input: &str) -> Vec<i64> {
    //let mut sizes = Vec::new();
    let mut blocks = Vec::new();
    let mut file_id = 0;
    input.chars().enumerate().for_each(|(idx, char)| {
        if !char.is_digit(10) {
            return;
        }
        let n = char.to_digit(10).unwrap();
        if idx % 2 == 0 {
            for _ in 0..n {
                blocks.push(file_id);
            }
            file_id += 1;
        } else {
            for _ in 0..n {
                blocks.push(-1);
            }
        }
        //sizes.push(char.to_digit(10).unwrap());
    });
    //sizes
    blocks
}

#[derive(Debug)]
struct Allocation{
    start: usize,
    size: usize,
    file_id: u32,
    is_free: bool,
}

impl Allocation {
    fn fits(&self, other: &Allocation) -> bool {
        self.size >= other.size
    }
    fn move_into(&self, other: &Allocation) -> (Allocation, Allocation) {
        (Allocation{start: self.start, size: other.size, file_id: other.file_id, is_free: other.is_free}, Allocation{start: self.start + other.size, size: self.size - other.size, file_id: self.file_id, is_free: self.is_free})
    }
    fn free(&self) -> Allocation {
        Allocation{start: self.start, size: self.size, file_id: 0, is_free: true}
    }
    fn score(&self) -> u64 {
        if self.is_free {
            return 0;
        }
        (self.start..self.start+self.size).fold(0, |acc, idx| {
            acc + (idx as u64)*(self.file_id as u64)
        })
    }
}

fn parse_input2(input: &str) -> Vec<Allocation> {
    //let mut sizes = Vec::new();
    let mut allocations: Vec<Allocation> = Vec::new();
    let mut file_id = 0;
    let mut start = 0;
    input.chars().enumerate().for_each(|(idx, char)| {
        if !char.is_digit(10) {
            return;
        }
        let n = char.to_digit(10).unwrap();
        if idx % 2 == 0 {
            allocations.push(Allocation{start, size: n as usize, file_id, is_free: false});

            file_id += 1;
            start += n as usize;
        } else {
            allocations.push(Allocation{start, size: n as usize, file_id:0, is_free: true});
            start += n as usize;
        }
    });
    allocations
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut blocks = parse_input(input);
    
    let mut low_idx = 0;
    let mut high_idx = blocks.len() - 1;

    while low_idx < high_idx {
        if blocks[high_idx] == -1 {
            high_idx -= 1;
        } else if blocks[low_idx] != -1 {
            low_idx += 1;
        } else {
            blocks[low_idx] = blocks[high_idx];
            blocks[high_idx] = -1;
            low_idx += 1;
            high_idx -= 1;
        }
    }

    Some(blocks.iter().enumerate().fold(0, |acc, (idx, block)| {
        if *block != -1 {
            acc + (idx as i64)*block
        } else {
            acc
        }
    }) as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut allocations = parse_input2(input);
    let mut already_moved = HashSet::new();
    let mut high_idx = allocations.len() - 1;
    while high_idx > 0{
        if !allocations[high_idx].is_free && !already_moved.contains(&allocations[high_idx].file_id) {
            for low_idx in 0..high_idx {
                if !allocations[low_idx].is_free {
                    continue;
                }
                if allocations[low_idx].size >= allocations[high_idx].size {
                    let (new_allocation1, new_allocation2) = allocations[low_idx].move_into(&allocations[high_idx]);
                    already_moved.insert( new_allocation1.file_id);
                    allocations[high_idx] = allocations[high_idx].free();
                    allocations[low_idx] = new_allocation1;
                    if new_allocation2.size > 0 {
                        allocations.insert(low_idx+1, new_allocation2);
                    }
                    break;
                }
            }
        }
        high_idx -= 1;
    }

    Some(allocations.iter().fold(0, |acc, allocation| {
        acc + allocation.score()
    }) as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}

// 6265268809555