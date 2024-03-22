use std::iter::successors;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
    diagonals: Vec<ChessPosition>,
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
        let mut diagonals = Vec::<ChessPosition>::new();
        for position in successors(Some(position.clone()), |pos| {
            ChessPosition::new(pos.rank - 1, pos.file + 1)
        }) {
            diagonals.push(position);
        }
        for position in successors(Some(position.clone()), |pos| {
            ChessPosition::new(pos.rank + 1, pos.file + 1)
        }) {
            diagonals.push(position);
        }
        for position in successors(Some(position.clone()), |pos| {
            ChessPosition::new(pos.rank + 1, pos.file - 1)
        }) {
            diagonals.push(position);
        }
        for position in successors(Some(position.clone()), |pos| {
            ChessPosition::new(pos.rank - 1, pos.file - 1)
        }) {
            diagonals.push(position);
        }
        Queen { position, diagonals }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        if self.position.rank == other.position.rank {
            return true;
        } else if self.position.file == other.position.file {
            return true;
        } else if self.diagonals.contains(&other.position) {
            return true;
        }
        false
    }
}
