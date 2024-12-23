use std::collections::HashSet;

advent_of_code::solution!(15);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction{
    N,
    E,
    S,
    W,
}

struct Grid {
    matrix: Vec<Vec<char>>,
    robot_position: (usize, usize),
}

impl Grid {
    fn move_robot(&mut self, direction: Direction) {
        let delta_y: i32;
        let delta_x: i32;
        match direction {
            Direction::N => {
                delta_y = -1;
                delta_x = 0;
            }
            Direction::E => {
                delta_y = 0;
                delta_x = 1;
            }
            Direction::S => {
                delta_y = 1;
                delta_x = 0;
            }
            Direction::W => {
                delta_y = 0;
                delta_x = -1;
            }
        }

        let mut new_y = self.robot_position.0 as i32;
        let mut new_x = self.robot_position.1 as i32;
        let mut hit_wall = false;
        let mut box_positions = Vec::new();
        loop {
            new_x += delta_x;
            new_y += delta_y;
            // if char is O we have a box and continue 
            match self.matrix[new_y as usize][new_x as usize] {
                'O' => {
                    box_positions.push((new_y as usize, new_x as usize));
                    continue;
                }
                '#' => {
                    hit_wall = true;
                    break;
                }
                '.' => break,
                _ => panic!("Invalid character in matrix"),
            }
        }
        if !hit_wall {
            // move all boxes and robot
            for box_position in box_positions.iter().rev() {
                self.matrix[box_position.0][box_position.1] = '.';
                self.matrix[(box_position.0 as i32 + delta_y) as usize][(box_position.1 as i32 + delta_x) as usize] = 'O';
            }
            self.matrix[self.robot_position.0][self.robot_position.1] = '.';
            self.robot_position = ((self.robot_position.0 as i32 + delta_y) as usize, (self.robot_position.1 as i32 + delta_x) as usize);
        }

    }
    fn move_robot2(&mut self, direction: Direction){
        let mut contact_points = HashSet::new();
        let delta_y: i32;
        let delta_x: i32;
        match direction {
            Direction::N => {
                delta_y = -1;
                delta_x = 0;
            }
            Direction::E => {
                delta_y = 0;
                delta_x = 1;
            }
            Direction::S => {
                delta_y = 1;
                delta_x = 0;
            }
            Direction::W => {
                delta_y = 0;
                delta_x = -1;
            }
        }
        contact_points.insert((self.robot_position.0, self.robot_position.1));
        let mut box_positions = HashSet::new();
        let mut hit_wall = false;
        loop {
            let mut new_contact_points = HashSet::new();
            for contact_point in contact_points.iter(){
                let new_y = contact_point.0 as i32 + delta_y;
                let new_x = contact_point.1 as i32 + delta_x;
                match self.matrix[new_y as usize][new_x as usize] {
                    '[' => {
                        box_positions.insert((new_y, new_x));
                        new_contact_points.insert((new_y as usize, new_x as usize));
                        if delta_x== 0{
                            new_contact_points.insert((new_y as usize, new_x as usize +1));
                        }
                    }
                    ']'=> {
                        box_positions.insert((new_y, new_x-1));
                        new_contact_points.insert((new_y as usize, new_x  as usize));
                        if delta_x == 0{
                            new_contact_points.insert((new_y as usize, new_x  as usize -1));
                        }
                    }
                    '.' => {
                        continue;
                    }
                    '#' => {
                        hit_wall = true;
                        break;
                    }
                    _ => panic!("Invalid character in matrix"),
                }
            }
            if hit_wall || new_contact_points.is_empty(){
                break;
            }
            contact_points = new_contact_points;
        }
        if !hit_wall {
            for box_position in box_positions.iter(){
                self.matrix[box_position.0 as usize][box_position.1 as usize] = '.';
                self.matrix[box_position.0 as usize][box_position.1 as usize+1] = '.';
            }
            for box_position in box_positions.iter(){
                self.matrix[(box_position.0 as i32 + delta_y) as usize][(box_position.1 as i32 + delta_x) as usize] = '[';
                self.matrix[(box_position.0 as i32 + delta_y) as usize][(box_position.1 as i32 + delta_x) as usize+1] = ']';
            }
            self.robot_position = ((self.robot_position.0 as i32 + delta_y) as usize, (self.robot_position.1 as i32 + delta_x) as usize);
        }

        
    }
    fn score(&self) -> u32 {
        let mut score = 0;
        for (y, row) in self.matrix.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c == 'O' {
                    score += y*100 + x;
                }
            }
        }
        score as u32
    }
    fn score2(&self) -> u32 {
        let mut score = 0;
        for (y, row) in self.matrix.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c == '[' {
                    score += y*100 + x;
                }
            }
        }
        score as u32
    }
    fn expand(&mut self) {
        let mut new_matrix: Vec<Vec<char>> = Vec::new();
        for row in self.matrix.iter() {
            let mut new_row = Vec::new();
            for c in row.iter() {
                match c {
                    'O' => {
                        new_row.push('[');
                        new_row.push(']');
                    }
                    '#' => {
                        new_row.push('#');
                        new_row.push('#');
                    }
                    '.' => {
                        new_row.push('.');
                        new_row.push('.');
                    }
                    _ => panic!("Invalid character in matrix"),
                }
            }
            new_matrix.push(new_row);
        }
        self.matrix = new_matrix;
        self.robot_position = (self.robot_position.0, self.robot_position.1*2);
    }
    fn print(&self) {
        for (y,row) in self.matrix.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if (y, x) == self.robot_position {
                    print!("@");
                } else {
                    print!("{}", c);
                }
            }
            println!();
        }
        println!();
    }
    
}

fn parse(input: &str) -> (Grid, Vec<Direction>) {
    let mut matrix = Vec::new();
    let mut robot_position = (0, 0);
    let (grid_input, instructions) = input.split_once("\n\n").unwrap();

    for (y, line) in grid_input.lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            match c {
                '@' => {
                    robot_position = (y, x);
                    row.push('.');
                },
                _ => row.push(c),
            }
        }
        matrix.push(row);
    }

    let mut directions = Vec::new();
    instructions.lines().for_each(|line| {
        line.chars().for_each(|c| {
            match c {
                '^' => directions.push(Direction::N),
                '>' => directions.push(Direction::E),
                'v' => directions.push(Direction::S),
                '<' => directions.push(Direction::W),
                _ => panic!("Invalid direction {}", c),
         }
        }
        );
    });

    (Grid { matrix, robot_position }, directions)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut grid, directions) = parse(input);
    for direction in directions.iter() {
        grid.move_robot(*direction);
    }
    Some(grid.score())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut grid, directions) = parse(input);
    grid.expand();
    for direction in directions.iter() {
        grid.move_robot2(*direction);
    }
    Some(grid.score2())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
