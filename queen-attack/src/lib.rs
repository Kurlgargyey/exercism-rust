use std::iter::successors;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        let valid_range = 0..8;
        (valid_range.contains(&rank) && valid_range.contains(&file)).then_some(ChessPosition {
            rank,
            file,
        })
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen {
            position,
        }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        self.position.rank == other.position.rank ||
            self.position.file == other.position.file ||
            (self.position.rank - other.position.rank).abs() ==
                (self.position.file - other.position.file).abs()
    }
}
