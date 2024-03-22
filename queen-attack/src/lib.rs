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
        (matches!(rank, 0..=7) && matches!(file, 0..=7)).then_some(ChessPosition {
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
