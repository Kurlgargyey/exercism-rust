use std::ops;

#[derive(Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy)]
struct Direction {
    x: isize,
    y: isize,
}

impl ops::Add<Direction> for Position {
    type Output = Position;
    fn add(self, rhs: Direction) -> Position {
        Position {
            x: ((self.x as isize) + rhs.x) as usize,
            y: ((self.y as isize) + rhs.y) as usize,
        }
    }
}
impl ops::AddAssign<Direction> for Position {
    fn add_assign(&mut self, rhs: Direction) {
        self.x = ((self.x as isize) + rhs.x) as usize;
        self.y = ((self.y as isize) + rhs.y) as usize;
    }
}

pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut matrix = vec![vec![0; size as usize]; size as usize];

    let mut pos = Position { x: 0, y: 0 };

    let right = Direction { x: 0, y: 1 };
    let down = Direction { x: 1, y: 0 };
    let left = Direction { x: 0, y: -1 };
    let up = Direction { x: -1, y: 0 };

    let directions = vec![right, down, left, up];

    let mut turns = 0;
    let mut direction = directions[turns % 4];

    for val in 1..=size.pow(2) {
        set_cell(&mut matrix, pos, val);
        if !cell_ahead_is_empty_and_valid(&matrix, pos + direction) {
            turns += 1;
            direction = directions[turns % 4];
        }
        pos += direction;
    }

    matrix
}

fn cell_ahead_is_empty_and_valid(matrix: &Vec<Vec<u32>>, pos: Position) -> bool {
    if let Some(row) = matrix.get(pos.x) {
        if let Some(val) = row.get(pos.y) {
            return *val == 0;
        }
    }
    false
}

fn set_cell(matrix: &mut Vec<Vec<u32>>, pos: Position, val: u32) {
    matrix[pos.x][pos.y] = val;
}
