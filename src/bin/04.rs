advent_of_code::solution!(4);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    N,
    E,
    S,
    W,
    NE,
    SE,
    SW,
    NW,
}

impl Direction {
    pub fn all() -> Vec<Direction> {
        vec![
            Direction::N,
            Direction::E,
            Direction::S,
            Direction::W,
            Direction::NE,
            Direction::SE,
            Direction::SW,
            Direction::NW,
        ]
    }
}

static ALL_DIRECTIONS: [Direction; 8] = [
    Direction::N,
    Direction::E,
    Direction::S,
    Direction::W,
    Direction::NE,
    Direction::SE,
    Direction::SW,
    Direction::NW,
];

#[derive(Clone, Copy, Debug)]
pub struct Point {
    x: usize,
    y: usize,
    content: char,
}


pub struct Matrix {
    pub cells: Vec<Vec<Point>>,
    pub cols: usize,
    pub rows: usize
}

impl Matrix {
    fn get(&self, x: usize, y: usize) -> &Point {
        &self.cells[y][x]
    }

    fn neighbor(&self, point: &Point, direction: Direction)  -> Option<&Point> {
        match direction {
            Direction::N => {
                let x = point.x;
                let y= point.y.checked_sub(1)?;
                Some(self.get(x, y))
                //Some(Point { x, y })
            },
            Direction::E => {
                let x = if point.x + 1 < self.cols {
                    point.x + 1
                } else {
                    return None;
                };
                Some(self.get(x, point.y))
                //Some(Point { x, y: point.y })
            },
            Direction::S  => {
                let y = if point.y + 1 < self.rows {
                    point.y + 1
                } else {
                    return None;
                };
                Some(self.get(point.x, y))
            },
            Direction::W => {
                let x = point.x.checked_sub(1)?;
                let y= point.y;
                Some(self.get(x, y))
            },
            Direction::NE => {
                let point_east = self.neighbor(point, Direction::E)?;
                self.neighbor(&point_east, Direction::N)
            },
            Direction::SE => {
                let point_east = self.neighbor(point, Direction::E)?;
                self.neighbor(&point_east, Direction::S)
            },
            Direction::SW => {
                let point_west =  self.neighbor(point, Direction::W)?;
                self.neighbor(&point_west, Direction::S)
            },
            Direction::NW => {
                let point_west =  self.neighbor(point, Direction::W)?;
                self.neighbor(&point_west, Direction::N)
            },
        }
    }
    fn check(&self, point: &Point, direction: Direction, chars: [char; 3], index: usize) -> bool{
        // check that this point is the start of the word
        if chars[index] != point.content {
            return false;
        }
        if index == chars.len() - 1 {
            return true;
        }
        let next_point = self.neighbor(point, direction);
        if next_point.is_none() {
            return false;
        }
        self.check(&next_point.unwrap(), direction, chars, index+1)
    }
    fn check_2(&self, point: &Point) -> bool{
        let sw_point = self.neighbor(point, Direction::SW);
        let ne_point = self.neighbor(point, Direction::NE);
        if !sw_point.is_none() && !ne_point.is_none() {
            let sw = sw_point.unwrap();
            let ne = ne_point.unwrap();
            if sw.content == 'M' && ne.content == 'S' || sw.content == 'S' && ne.content == 'M' {
                let se_point = self.neighbor(point, Direction::SE);
                let nw_point = self.neighbor(point, Direction::NW);
                if !se_point.is_none() && !nw_point.is_none() {
                    let se = se_point.unwrap();
                    let nw = nw_point.unwrap();
                    if se.content == 'M' && nw.content == 'S' || se.content == 'S' && nw.content == 'M' {
                        return true;
                    }
                }
            }
        }
        false
    }
}

fn parse_input(input: &str) -> Matrix {
    let cells: Vec<Vec<Point>> = input
        .lines()
        .enumerate()
        .filter_map(|(idy, line)| {
            if line.is_empty() {
                None
            } else {
                Some(
                    line.chars()
                        .enumerate()
                        .map(|(idx, c)| Point { x: idx, y: idy, content: c })
                        .collect(),
                )


                //Some(line.chars().collect())
            }
        })
        .collect();

    let rows = cells.len();
    let cols = cells[0].len();

    Matrix {
        cells,
        rows,
        cols,
    }
}

const CHARS: [char; 3] = ['M', 'A', 'S'];

fn count_xmas(matrix: &Matrix, point: &Point) -> usize {

    ALL_DIRECTIONS.iter().fold(0, |acc, direction| {
        let next_point = matrix.neighbor(point, *direction);
        if !next_point.is_none() && matrix.check(&next_point.unwrap(), *direction, CHARS, 0) {
            return acc + 1;
        }
        acc
    })
}

pub fn part_one(input: &str) -> Option<usize> {
    let matrix = parse_input(input);

    Some(matrix.cells.iter().fold(0,|acc, row| {
        acc + row.iter().fold(0, |acc,cell| {
            if cell.content == 'X' {
                return acc + count_xmas(&matrix, cell)
            }
            acc
        })
    }))
}

pub fn part_two(input: &str) -> Option<usize> {
    let matrix = parse_input(input);

    Some(matrix.cells.iter().fold(0,|acc, row| {
        acc + row.iter().fold(0, |acc,cell| {
            if cell.content == 'A' && matrix.check_2(cell){
                    return acc + 1;
            }    
            acc
        })
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
