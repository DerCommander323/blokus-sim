pub struct GameBoard {
    squares: [[Square; 20]; 20],
}

#[derive(Debug, Clone, Default)]
pub enum Square {
    #[default]
    Empty,
    Blue,
    Green,
    Red,
    Yellow,
}

impl GameBoard {
    pub fn new() -> Self {
        Self {
            squares: Default::default(),
        }
    }

    pub fn get_squares(&self) -> &[[Square; 20]; 20] {
        &self.squares
    }

    pub fn set_square(&mut self, x: usize, y: usize, square: Square) {
        self.squares[x][y] = square;
    }
}
