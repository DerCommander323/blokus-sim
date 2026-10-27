use crate::game::pieces::{Colors, Occupation};

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

    pub fn reset(&mut self) {
        *self = Self::new()
    }

    pub fn get_squares(&self) -> &[[Square; 20]; 20] {
        &self.squares
    }

    pub fn set_square(&mut self, x: usize, y: usize, square: Square) {
        self.squares[x][y] = square;
    }

    pub fn place_piece(&mut self, occupation: &Occupation, color: Colors, x: usize, y: usize) {
        let square = color.to_square();

        for dx in 0..occupation.width {
            for dy in 0..occupation.height {
                if occupation.occupied_at_pos(dx, dy) {
                    if x + dx as usize >= 20 || y + dy as usize >= 20 {
                        continue;
                    }

                    self.set_square(x + dx as usize, y + dy as usize, square.clone());
                }
            }
        }
    }
}
