use crate::game::board::GameBoard;

pub struct Simulator {
    board: GameBoard,
}

impl Simulator {
    pub fn new() -> Self {
        Self {
            board: GameBoard::new(),
        }
    }

    pub fn board(&self) -> &GameBoard {
        &self.board
    }

    pub fn board_mut(&mut self) -> &mut GameBoard {
        &mut self.board
    }
}
