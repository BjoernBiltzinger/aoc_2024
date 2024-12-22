use std::collections::HashSet;

advent_of_code::solution!(12);

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

#[derive(Debug, Clone, PartialEq, Eq)]
struct Region{
    plots_positions: HashSet<(usize, usize)>,
    value: char,
}

impl Region{
    fn area(&self) -> u32{
        self.plots_positions.len() as u32
    }
    fn perimeter(&self, matrix: &Matrix) -> u32{
        let mut perimeter = 0;
        for (x, y) in self.plots_positions.iter(){
            perimeter += 4 - matrix.get_same_char_neighbours(*x, *y).len() as u32;
        }
        perimeter
    }
    fn sides(&self, matrix: &Matrix) -> u32{
        let mut sides = 0;
        for (x, y) in self.plots_positions.iter(){
            for direction in ALL_DIRECTIONS.iter(){
                match direction {
                    Direction::S => {
                        let south_point = matrix.get_neighbour(*x, *y, Direction::S);
                        if south_point.is_none(){
                            let east_point = matrix.get_neighbour(*x, *y, Direction::E);
                            if east_point.is_none() || matrix.values[east_point.unwrap().1][east_point.unwrap().0] != self.value{
                                sides += 1;
                                continue;
                            }
                        } else if matrix.values[south_point.unwrap().1][south_point.unwrap().0] == self.value{
                                continue;
                        } else {
                            let east_point = matrix.get_neighbour(*x, *y, Direction::E);
                            if east_point.is_none() || matrix.values[east_point.unwrap().1][east_point.unwrap().0] != self.value{
                                sides += 1;
                                continue;
                            }
                            let south_east_point = matrix.get_neighbour(south_point.unwrap().0, south_point.unwrap().1, Direction::E);
                            if south_east_point.is_none() || matrix.values[south_east_point.unwrap().1][south_east_point.unwrap().0] == self.value{
                                sides += 1;
                                continue;
                            }
                        }
                    }
                    Direction::N => {
                        let north_point = matrix.get_neighbour(*x, *y, Direction::N);
                        if north_point.is_none(){
                            let east_point = matrix.get_neighbour(*x, *y, Direction::E);
                            if east_point.is_none() || matrix.values[east_point.unwrap().1][east_point.unwrap().0] != self.value{
                                sides += 1;
                                continue;
                            }
                        } else if matrix.values[north_point.unwrap().1][north_point.unwrap().0] == self.value{
                                continue;
                        } else {
                            let east_point = matrix.get_neighbour(*x, *y, Direction::E);
                            if east_point.is_none() || matrix.values[east_point.unwrap().1][east_point.unwrap().0] != self.value{
                                sides += 1;
                                continue;
                            }
                            let north_east_point = matrix.get_neighbour(north_point.unwrap().0, north_point.unwrap().1, Direction::E);
                            if north_east_point.is_none() || matrix.values[north_east_point.unwrap().1][north_east_point.unwrap().0] == self.value{
                                sides += 1;
                                continue;
                            }
                        }
                    }
                    Direction::E => {
                        let east_point = matrix.get_neighbour(*x, *y, Direction::E);
                        if east_point.is_none(){
                            let south_point = matrix.get_neighbour(*x, *y, Direction::S);
                            if south_point.is_none() || matrix.values[south_point.unwrap().1][south_point.unwrap().0] != self.value{
                                sides += 1;
                                continue;
                            }
                        } else if matrix.values[east_point.unwrap().1][east_point.unwrap().0] == self.value{
                            continue;
                        } else {
                            let south_point = matrix.get_neighbour(*x, *y, Direction::S);
                            if south_point.is_none() || matrix.values[south_point.unwrap().1][south_point.unwrap().0] != self.value{
                                sides += 1;
                                continue;
                            }
                            let south_east_point = matrix.get_neighbour(south_point.unwrap().0, south_point.unwrap().1, Direction::E);
                            if south_east_point.is_none() || matrix.values[south_east_point.unwrap().1][south_east_point.unwrap().0] == self.value{
                                sides += 1;
                                continue;
                            }
                        }
                    }
                    Direction::W => {
                        let west_point = matrix.get_neighbour(*x, *y, Direction::W);
                        if west_point.is_none(){
                            let south_point = matrix.get_neighbour(*x, *y, Direction::S);
                            if south_point.is_none() || matrix.values[south_point.unwrap().1][south_point.unwrap().0] != self.value{
                                sides += 1;
                                continue;
                            }
                        } else if matrix.values[west_point.unwrap().1][west_point.unwrap().0] == self.value{
                            continue;
                        } else {
                            let south_point = matrix.get_neighbour(*x, *y, Direction::S);
                            if south_point.is_none() || matrix.values[south_point.unwrap().1][south_point.unwrap().0] != self.value{
                                sides += 1;
                                continue;
                            }
                            let south_west_point = matrix.get_neighbour(south_point.unwrap().0, south_point.unwrap().1, Direction::W);
                            if south_west_point.is_none() || matrix.values[south_west_point.unwrap().1][south_west_point.unwrap().0] == self.value{
                                sides += 1;
                                continue;
                            }
                        }
                    }
                }
            }
        }
        sides
    }
}

struct Matrix{
    values: Vec<Vec<char>>,
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
    fn get_same_char_neighbours(&self, x: usize, y: usize) -> Vec<(usize, usize)>{
        let value = self.values[y][x];
        let mut vec = Vec::new();
        for direction in ALL_DIRECTIONS.iter(){
            if let Some((xnew, ynew)) = self.get_neighbour(x, y, *direction){
                if self.values[ynew][xnew] == value{
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
                vec2.push(char);
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

fn get_region(matrix: &Matrix, x_start: usize, y_start: usize) -> Region{
    // for a given point, get the region it belongs to
    let mut places_assigned = HashSet::new();
    let mut places_to_visit = HashSet::new();
    places_to_visit.insert((x_start, y_start));
    let value = matrix.values[y_start][x_start];
    while !places_to_visit.is_empty(){
        let (x, y) = places_to_visit.iter().next().unwrap().clone();
        places_assigned.insert((x, y));
        places_to_visit.remove(&(x, y));
        for direction in ALL_DIRECTIONS.iter(){
            if let Some((xnew, ynew)) = matrix.get_neighbour(x, y, *direction){
                if matrix.values[ynew][xnew] == value && !places_assigned.contains(&(xnew, ynew)){
                    places_to_visit.insert((xnew, ynew));
                }
            }
        }
    }
    Region{plots_positions: places_assigned, value}   
}

fn map_out_regions(matrix: &Matrix) -> Vec<Region>{
    let mut places_assigned = HashSet::new();
    let mut regions = Vec::new();
    for y in 0..matrix.n_rows{
        for x in 0..matrix.n_cols{
            if places_assigned.contains(&(x, y)){
                continue;
            }
            let region = get_region(&matrix, x, y);
            places_assigned = places_assigned.union(&region.plots_positions).cloned().collect();
            regions.push(region);
        }
    }
    regions
}

pub fn part_one(input: &str) -> Option<u32> {
    let matrix = parse_input(input);
    let regions = map_out_regions(&matrix);
    Some(regions.iter().map(|region| region.area() * region.perimeter(&matrix)).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let matrix = parse_input(input);
    let regions = map_out_regions(&matrix);
    Some(regions.iter().map(|region| region.area() * region.sides(&matrix)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
