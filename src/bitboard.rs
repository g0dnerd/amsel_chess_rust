#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct BitBoard(pub u64);

impl BitBoard {
    // Returns an empty BitBoard
    pub fn new() -> Self {
        Self(0)
    }

    // Returns whether the BitBoard is empty
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }
} 