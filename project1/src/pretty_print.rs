//! Pretty-printing functions to make debugging more pleasant.

use super::*;

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..PUZZLE_SIZE {
            for col in 0..PUZZLE_SIZE {
                write!(f, "{:?} ", self.board[row][col])?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
