// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: i32,
    y: i32,
    d: Direction,
}

use self::Direction::*;
impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot { x, y, d }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        Robot {
            x: self.x,
            y: self.y,
            d: {
                match self.d {
                    North => East,
                    East => South,
                    South => West,
                    West => North,
                }
            },
        }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        Robot {
            x: self.x,
            y: self.y,
            d: {
                match self.d {
                    North => West,
                    East => North,
                    South => East,
                    West => South,
                }
            },
        }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        Robot {
            x: {
                match self.direction() {
                    East => self.x + 1,
                    West => self.x - 1,
                    _ => self.x,
                }
            },
            y: {
                match self.direction() {
                    North => self.y + 1,
                    South => self.y - 1,
                    _ => self.y,
                }
            },
            d: self.d,
        }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |robot, instruction| {
            match instruction {
                'R' => { robot.turn_right() }
                'L' => { robot.turn_left() }
                'A' => { robot.advance() }
                _ => {
                    panic!("I don't know this instruction!");
                }
            }
        })
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
