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
        let diagonals = vec![(-1, 1), (1, 1), (1, -1), (-1, -1)]
            .into_iter()
            .flat_map(|(rank_mv, file_mv)|
                successors(Some(position.clone()), |pos|
                    ChessPosition::new(pos.rank + rank_mv, pos.file + file_mv)
                ).collect::<Vec<ChessPosition>>()
            )
            .collect();

        Queen { position, diagonals }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        self.position.rank == other.position.rank ||
            self.position.file == other.position.file ||
            self.diagonals.contains(&other.position)
    }
}
