use std::collections::{vec_deque, HashMap, HashSet, VecDeque};

advent_of_code::solution!(16);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction{
    N,
    E,
    S,
    W,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State{
    pos: (usize, usize),
    direction: Direction,
    points: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Path{
    states: Vec<State>,
    //points: u32,
}

impl Path {
    fn current_state(&self) -> &State {
        self.states.last().unwrap()
    }
    fn places_on_path(&self) -> HashSet<(usize,usize)> {
        self.states.iter().map(|state| state.pos).collect()
    }
    fn add_state(&self, state: State) -> Path {
        let mut new_states = self.states.clone();
        new_states.push(state);
        Path { states: new_states }
    }
    fn print(&self, grid: Vec<Vec<char>>) {
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if self.states.iter().any(|state| state.pos == (x, y)) {
                    print!("X");
                } else {
                    print!("{}", grid[y][x]);
                }
            }
            println!();
        }
    }
}

impl State {
    fn step(&self) -> State {
        let mut new_pos = self.pos;
        match self.direction {
            Direction::N => new_pos.1 -= 1,
            Direction::E => new_pos.0 += 1,
            Direction::S => new_pos.1 += 1,
            Direction::W => new_pos.0 -= 1,
        }
        State {
            pos: new_pos,
            direction: self.direction,
            points: self.points +1,
        }
    }
    fn rotate_clockwise(&self) -> State {
        let new_direction = match self.direction {
            Direction::N => Direction::E,
            Direction::E => Direction::S,
            Direction::S => Direction::W,
            Direction::W => Direction::N,
        };
        State {
            pos: self.pos,
            direction: new_direction,
            points: self.points + 1000,
        }
    }
    fn rotate_counter_clockwise(&self) -> State {
        let new_direction = match self.direction {
            Direction::N => Direction::W,
            Direction::E => Direction::N,
            Direction::S => Direction::E,
            Direction::W => Direction::S,
        };
        State {
            pos: self.pos,
            direction: new_direction,
            points: self.points +1000,
        }
    }
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().filter_map(|line| {
        if line.is_empty() {
            return None;
        }
        Some(line.chars().collect())
    }).collect()
}

fn add_state_to_queue(state: State, queue: &mut VecDeque<State>, visited: &mut HashSet<(usize, usize, Direction)>) {
    if visited.insert((state.pos.0, state.pos.1, state.direction)) {
        // add state to queue at correct position (sorted by points)
        let mut index = 0;
        while index < queue.len() && queue[index].points < state.points {
            index += 1;
        }
        queue.insert(index, state);
    }
}

fn pathfinding(grid: Vec<Vec<char>>) -> u32 {
    let start_pos = grid.iter().enumerate().find_map(|(y, row)| {
        row.iter().enumerate().find_map(|(x, &c)| {
            if c == 'S' {
                Some((x, y))
            } else {
                None
            }
        })
    }).unwrap();
    let mut states_queue: VecDeque<State> = VecDeque::new();
    let mut visited: HashSet<(usize, usize, Direction)> = HashSet::new();
    states_queue.push_back(State {
        pos: start_pos,
        direction: Direction::E,
        points: 0,
    });
    visited.insert((start_pos.0, start_pos.1, Direction::E));
    loop {
        let current_state = states_queue.pop_front().unwrap();
        if grid[current_state.pos.1][current_state.pos.0] == 'E' {
            return current_state.points;
        }
        // step 
        let next_state = current_state.step();
        if grid[next_state.pos.1][next_state.pos.0] != '#' {
            add_state_to_queue(next_state, &mut states_queue, &mut visited);
        }
        // rotate clockwise
        let next_state = current_state.rotate_clockwise();
        add_state_to_queue(next_state, &mut states_queue, &mut visited);
        // rotate counter clockwise
        let next_state = current_state.rotate_counter_clockwise();
        add_state_to_queue(next_state, &mut states_queue, &mut visited);
    }
} 

fn add_path_to_queue(path: Path, queue: &mut VecDeque<Path>, visited: &mut HashMap<(usize, usize, Direction), u32>) {
    let pos_points = visited.get(&(path.current_state().pos.0, path.current_state().pos.1, path.current_state().direction)).unwrap_or(&0);
    if *pos_points == 0 || path.current_state().points <= *pos_points {
        // add state to queue at correct position (sorted by points)
        let mut index = 0;
        while index < queue.len() && queue[index].current_state().points < path.current_state().points {
            index += 1;
        }
        queue.insert(index, path.clone());
        visited.insert((path.current_state().pos.0, path.current_state().pos.1, path.current_state().direction), path.current_state().points);
    }
}

fn find_best_paths(grid: Vec<Vec<char>>) -> Vec<Path> {
    let start_pos = grid.iter().enumerate().find_map(|(y, row)| {
        row.iter().enumerate().find_map(|(x, &c)| {
            if c == 'S' {
                Some((x, y))
            } else {
                None
            }
        })
    }).unwrap();
    let mut paths_queue: VecDeque<Path> = VecDeque::new();
    let mut visited: HashMap<(usize, usize, Direction), u32> = HashMap::new();
    paths_queue.push_back(Path {
        states: vec![State {
            pos: start_pos,
            direction: Direction::E,
            points: 0,
        }]
    });
    visited.insert((start_pos.0, start_pos.1, Direction::E),0);
    let mut all_paths = Vec::new();
    let mut best_path_length = 0;
    loop {
        if paths_queue.is_empty() {
            break;
        }
        let current_path = paths_queue.pop_front().unwrap();
        let current_state = current_path.current_state();
        if best_path_length == 0 && grid[current_state.pos.1][current_state.pos.0] == 'E' {
            best_path_length = current_state.points;
        }
        if best_path_length != 0 && current_state.points > best_path_length {
            continue;
        }
        if grid[current_state.pos.1][current_state.pos.0] == 'E' {
            all_paths.push(current_path);
            continue;
        }
        // step 
        let next_state = current_state.step();
        if grid[next_state.pos.1][next_state.pos.0] != '#' {
            let new_path = current_path.add_state(next_state); 
            add_path_to_queue(new_path, &mut paths_queue, &mut visited);
        }
        // rotate clockwise
        let next_state = current_state.rotate_clockwise();
        let new_path = current_path.add_state(next_state);
        add_path_to_queue(new_path, &mut paths_queue, &mut visited);
        // rotate counter clockwise
        let next_state = current_state.rotate_counter_clockwise();
        let new_path = current_path.add_state(next_state);
        add_path_to_queue(new_path, &mut paths_queue, &mut visited);
    }
    all_paths
} 

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse(input);
    Some(pathfinding(grid))
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse(input);
    let paths = find_best_paths(grid);
    // union of all pos in paths
    let all_places = paths.iter().map(|path| path.places_on_path()).fold(HashSet::new(), |acc, x| acc.union(&x).cloned().collect());
    Some(all_places.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
