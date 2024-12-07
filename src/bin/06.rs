use std::collections::HashSet;

advent_of_code::solution!(6);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    N,
    E,
    S,
    W,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub struct State{
    pub pos: (i32, i32),
    pub direction: Direction,
}

impl State {
    pub fn turn_right(&self) -> State {
        match self.direction {
            Direction::N => State { pos: self.pos, direction: Direction::E },
            Direction::E => State { pos: self.pos, direction: Direction::S },
            Direction::S => State { pos: self.pos, direction: Direction::W },
            Direction::W => State { pos: self.pos, direction: Direction::N },
        }
    }
    pub fn walk(&self) -> State {
        match self.direction {
            Direction::N => State { pos: (self.pos.0, self.pos.1 - 1), direction: self.direction },
            Direction::E => State { pos: (self.pos.0 + 1, self.pos.1), direction: self.direction },
            Direction::S => State { pos: (self.pos.0, self.pos.1 + 1), direction: self.direction },
            Direction::W => State { pos: (self.pos.0 - 1, self.pos.1), direction: self.direction },
        }
    }
}

pub struct Matrix {
    pub obstructions: HashSet<(i32, i32)>,
    pub cols: i32,
    pub rows: i32
}

fn parse_input(input: &str) -> (Matrix, State) {
    let mut obstructions: HashSet<(i32, i32)> = HashSet::new();

    let mut num_rows = 0;
    let mut num_cols = 0;
    let mut start_pos = State { pos: (0, 0), direction: Direction::N};

    for (idy, line) in input.lines().enumerate() {
        if !line.is_empty() {
            for (idx, c) in line.chars().enumerate() {
                if num_rows == 0 {
                    num_cols += 1;
                }
                match c {
                    '#' => {obstructions.insert((idx.try_into().unwrap(), idy.try_into().unwrap()));},
                    '^' => {
                        start_pos = State { pos: (idx.try_into().unwrap(), idy.try_into().unwrap()), direction: Direction::N};},
                    '>' => {
                        start_pos = State { pos: (idx.try_into().unwrap(), idy.try_into().unwrap()), direction: Direction::E};},
                    'v' => {
                        start_pos = State { pos: (idx.try_into().unwrap(), idy.try_into().unwrap()), direction: Direction::S};},
                    '<' => {
                        start_pos = State { pos: (idx.try_into().unwrap(), idy.try_into().unwrap()), direction: Direction::W};},
                    '.' => {continue;},
                    _ => panic!("Invalid input")
                }
            }
            num_rows += 1;
        }
    }

    (Matrix {
        obstructions,
        cols: num_cols,
        rows: num_rows
    }, start_pos)
}

impl Matrix {
    fn walk_to_next_obstruction(&self, state: &State, mut places_visisted: HashSet<(i32,i32)>, additional_obs: Option<(i32,i32)>) -> (Option<State>, HashSet<(i32,i32)>) {
        // find the position of the next obstruction in the direction of the state
        let mut current_state = *state; 
        places_visisted.insert(current_state.pos);
        let mut next_state: State;
        loop {
            next_state = current_state.walk();
            if self.obstructions.contains(&next_state.pos) || additional_obs == Some(next_state.pos) {
                // found the next obstruction - turn 90 degrees to the right and return 
                next_state = current_state.turn_right();
                return (Some(next_state), places_visisted);
            }
            if !self.check_in_grid(&next_state) {
                return (None, places_visisted);
            }
            places_visisted.insert(next_state.pos);
            current_state = next_state;
        }

    }
    fn check_in_grid(&self, state: &State) -> bool {
        state.pos.0 >= 0 && state.pos.0 < self.rows && state.pos.1 >= 0 && state.pos.1 < self.cols
    }
    fn results_in_loop(&self, start_state: &State, additional_obs: (i32,i32)) -> bool {
        let mut states = HashSet::new();
        let mut current_state = *start_state;
        loop {
            if states.contains(&current_state) {
                return true;
            }
            states.insert(current_state);
            let (res, _) = self.walk_to_next_obstruction(&current_state, HashSet::new(), Some(additional_obs));
            current_state = match res {
                Some(s) => s,
                None => return false,
            };
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (matrix, start_pos) = parse_input(input);
    let mut state = start_pos;
    let mut places_visited = HashSet::new();
    while matrix.check_in_grid(&state) {
        match matrix.walk_to_next_obstruction(&state, places_visited, None) {
            (Some(s), p) => {
                state = s;
                places_visited = p;
            },
            (None, p) => {
                places_visited = p;
                break;
            }
        }
    }

    Some(places_visited.len().try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (matrix, start_pos) = parse_input(input);
    let mut state = start_pos;
    let mut places_visited = HashSet::new();
    while matrix.check_in_grid(&state) {
        match matrix.walk_to_next_obstruction(&state, places_visited, None) {
            (Some(s), p) => {
                state = s;
                places_visited = p;
            },
            (None, p) => {
                places_visited = p;
                break;
            }
        }
    }

    places_visited.into_iter().filter(|pos_add| {
        matrix.results_in_loop(&start_pos, *pos_add)
    }).count().try_into().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
