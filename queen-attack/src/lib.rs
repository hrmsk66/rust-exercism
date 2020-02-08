#[derive(Debug)]
pub struct ChessPosition (i32, i32);

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        match (rank, file) {
            (0..=7, 0..=7) => Some(ChessPosition(rank, file)),
            _ => None,
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let x = self.position.0 - other.position.0;
        let y = self.position.1 - other.position.1;
        x == 0 || y == 0 || i32::abs(x) == i32::abs(y) 
    }
}
