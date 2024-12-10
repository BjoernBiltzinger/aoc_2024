use std::collections::HashMap;

advent_of_code::solution!(10);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction{
    N,
    E,
    S,
    W,
}

static ALL_DIRECTIONS: [Direction; 4] = [
    Direction::N,
    Direction::E,
    Direction::S,
    Direction::W
];

struct Matrix{
    values: Vec<Vec<u32>>,
    n_cols: usize,
    n_rows: usize,
}

impl Matrix{
    fn get_neighbour(&self, x: usize, y: usize, direction: Direction) -> Option<(usize, usize)>{
        let xnew: i32 = if direction == Direction::E {x as i32+1} else if direction == Direction::W {x as i32-1} else {x as i32};
        let ynew: i32 = if direction == Direction::N {y as i32-1} else if direction == Direction::S {y as i32+1} else {y as i32};
        if xnew < 0 || xnew >= self.n_cols as i32 || ynew < 0 || ynew >= self.n_rows as i32{
            return None;
        }
        Some((xnew as usize, ynew as usize)) 
    }
    fn get_neighbours(&self, x: usize, y: usize, delta: u32) -> Vec<(usize, usize)>{
        let mut vec = Vec::new();
        for direction in ALL_DIRECTIONS.iter(){
            if let Some((xnew, ynew)) = self.get_neighbour(x, y, *direction){
                if self.values[ynew][xnew] as i32 - self.values[y][x] as i32 == delta as i32{
                    vec.push((xnew, ynew));
                }
            }
        }
        vec
    }
}

fn parse_input(input: &str) -> Matrix {
    let values = input.lines().fold(Vec::new(),  |mut vec, line| {
            if line.is_empty(){
                return vec;
            }
            let vec_inner = line.chars().fold(Vec::new(), |mut vec2, char|{
                vec2.push(char.to_digit(10).unwrap());
                vec2
                }
            );
            vec.push(vec_inner);
            return vec;
        }   
    );
    let n_cols = values[0].len();
    let n_rows = values.len();
    Matrix{values, n_cols, n_rows}
}

fn check_hikes(matrix: &Matrix, x: usize, y: usize, delta: usize, mut reachable: HashMap<(usize,usize), u32>) -> HashMap<(usize,usize), u32>{
    if matrix.values[y][x] == 9{
        // add entry to reachable if not already present and count up by one 
        reachable.entry((x,y)).and_modify(|e| *e += 1).or_insert(1);
    }
    matrix.get_neighbours(x, y, delta as u32).iter().fold(reachable, |acc, (neighbour_x, neighbour_y)| {
        check_hikes(matrix, *neighbour_x, *neighbour_y, delta, acc)
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    let matrix = parse_input(input);
    Some(matrix.values.iter().enumerate().fold(0, |acc, (idy, vec)| {
        acc + vec.iter().enumerate().fold(0, |acc2, (idx, val)| {
            if *val != 0{
                return acc2;
            }
            let mut reachable = HashMap::new();
            reachable = check_hikes(&matrix, idx, idy, 1, reachable);
            acc2 + reachable.len() as u32
            })
        })
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let matrix = parse_input(input);
    Some(matrix.values.iter().enumerate().fold(0, |acc, (idy, vec)| {
        acc + vec.iter().enumerate().fold(0, |acc2, (idx, val)| {
            if *val != 0{
                return acc2;
            }
            let mut reachable = HashMap::new();
            reachable = check_hikes(&matrix, idx, idy, 1, reachable);
            acc2 + reachable
            .into_iter()    
            .fold(0,|acc, (_, n)| {
                        acc+n
                    })
            })
        })
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
