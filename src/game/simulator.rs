use crate::game::board::GameBoard;

pub struct Simulator {
    board: GameBoard,
    overlay_board: GameBoard,
}

impl Simulator {
    pub fn new() -> Self {
        Self {
            board: GameBoard::new(),
            overlay_board: GameBoard::new(),
        }
    }

    pub fn board(&self) -> &GameBoard {
        &self.board
    }

    pub fn board_mut(&mut self) -> &mut GameBoard {
        &mut self.board
    }

    pub fn overlay_board_mut(&mut self) -> &mut GameBoard {
        &mut self.overlay_board
    }
}
