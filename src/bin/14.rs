advent_of_code::solution!(14);



struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
}

struct Grid {
    robots: Vec<Robot>,
    width: usize,
    height: usize,
}

impl Grid{
    fn move_robots(&mut self){
        for robot in self.robots.iter_mut(){
            let new_x = robot.position.0 + robot.velocity.0;
            let new_y = robot.position.1 + robot.velocity.1;
            // wrap around
            match new_x {
                x if x < 0 => robot.position.0 = self.width as i32 + x,
                x if x >= self.width as i32 => robot.position.0 = x - self.width as i32,
                _ => robot.position.0 = new_x,
            }
            match new_y {
                y if y < 0 => robot.position.1 = self.height as i32 + y,
                y if y >= self.height as i32 => robot.position.1 = y - self.height as i32,
                _ => robot.position.1 = new_y,
            }
        }
    }
    fn safety_factor(&self) -> i32 {
        let mut lower_left_quadrant =0;
        let mut upper_right_quadrant =0;
        let mut upper_left_quadrant =0;
        let mut lower_right_quadrant =0;
        for robot in self.robots.iter(){
            match robot.position {
                (x, y) if x < self.width as i32 / 2 && y < self.height as i32 / 2 => lower_left_quadrant += 1,
                (x, y) if x > self.width as i32 / 2 && y > self.height as i32 / 2 => upper_right_quadrant += 1,
                (x, y) if x < self.width as i32 / 2 && y > self.height as i32 / 2 => lower_right_quadrant += 1,
                (x, y) if x > self.width as i32 / 2 && y < self.height as i32 / 2 => upper_left_quadrant += 1,
                _ => (),
            }
        }
        lower_left_quadrant * upper_right_quadrant * upper_left_quadrant * lower_right_quadrant
    }
    fn print_grid(&self){
        let mut grid = vec![vec!['.'; self.width]; self.height];
        for robot in self.robots.iter(){
            grid[robot.position.1 as usize][robot.position.0 as usize] = '#';
        }
        for row in grid.iter(){
            println!("{}", row.iter().collect::<String>());
        }
    }
    fn no_overlap(&self) -> bool {
        let mut grid = vec![vec![0; self.width]; self.height];
        for robot in self.robots.iter(){
            grid[robot.position.1 as usize][robot.position.0 as usize] += 1;
        }
        grid.iter().flatten().all(|&x| x < 2)
    }
}


fn parse(input: &str) -> Vec<Robot> {
    // "p=0,4 v=3,-3" to position: (0, 4), velocity: (3, -3)
    input.lines().map(|line| {
        let position = (
            line.split_ascii_whitespace().nth(0).unwrap().split("=").nth(1).unwrap().split(",").nth(0).unwrap().parse().unwrap(),
            line.split_ascii_whitespace().nth(0).unwrap().split("=").nth(1).unwrap().split(",").nth(1).unwrap().parse().unwrap(),
        );
        let velocity = (
            line.split_ascii_whitespace().nth(1).unwrap().split("=").nth(1).unwrap().split(",").nth(0).unwrap().parse().unwrap(),
            line.split_ascii_whitespace().nth(1).unwrap().split("=").nth(1).unwrap().split(",").nth(1).unwrap().parse().unwrap(),
        );
        Robot { position, velocity }
    }).collect()
}

pub fn run_part_one(input: &str, n_cols: usize, n_rows: usize) -> Option<u32> {
    let robots = parse(input);
    let mut grid = Grid { robots, width: n_cols, height: n_rows };
    for _ in 0..100 {
        grid.move_robots();
    }
    Some(grid.safety_factor() as u32)
}

pub fn part_one_test(input: &str) -> Option<u32> {
    run_part_one(input, 11, 7)
}

pub fn part_one(input: &str) -> Option<u32> {
    run_part_one(input, 101, 103)
}

pub fn run_part_two(input: &str, n_cols: usize, n_rows: usize) -> Option<u32> {
    let robots = parse(input);
    let mut grid = Grid { robots, width: n_cols, height: n_rows };
    let mut steps = 0;
    loop {
        grid.move_robots();
        steps += 1;
        // looking for no overlap was a guess based on a discord hint - No intend to solve this one properly
        if grid.no_overlap() {
            grid.print_grid();
            break;
        }
    }
    Some(steps)
}

pub fn part_two(input: &str) -> Option<u32> {
    run_part_two(input, 101, 103)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_test(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }
}
