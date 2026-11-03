use crate::game::pieces::{Colors, Shape, SpecialShape, SpecialShapePart};

pub struct GameBoard {
    squares: [[Square; 20]; 20],
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum Square {
    #[default]
    Empty,
    Blue,
    Yellow,
    Red,
    Green,
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

    pub fn get_square(&self, x: usize, y: usize) -> &Square {
        &self.get_squares()[x][y]
    }

    pub fn set_square(&mut self, x: usize, y: usize, square: Square) {
        self.squares[x][y] = square;
    }

    pub fn can_place_piece(
        &self,
        shape: &SpecialShape,
        color: &Colors,
        x: isize,
        y: isize,
    ) -> bool {
        // 22 because the SpecialShapes are extended by 2 in both directions
        if x + shape.width as isize > 22 || y + shape.height as isize > 22 {
            return false;
        }

        dbg!(x, y);

        for dx in 0..shape.width {
            for dy in 0..shape.height {
                let Some(x) = (dx as isize + x).checked_sub(1) else {
                    continue;
                };
                let Some(y) = (dy as isize + y).checked_sub(1) else {
                    continue;
                };

                if x <= 0 || x >= 20 || y <= 0 || y >= 20 {
                    continue;
                }

                match shape.value_at_pos(dx, dy) {
                    SpecialShapePart::Adjacent___ => {
                        if self.get_square(x as usize, y as usize) == &color.to_square() {
                            return false;
                        }
                    }
                    SpecialShapePart::InnerCorner | SpecialShapePart::Normal_____ => {
                        if self.get_square(x as usize, y as usize) != &Square::Empty {
                            return false;
                        }
                    }
                    _ => {}
                }
            }
        }

        true
    }

    pub fn place_piece(&mut self, shape: &Shape, color: &Colors, x: usize, y: usize) {
        let square = color.to_square();

        for dx in 0..shape.width {
            for dy in 0..shape.height {
                if shape.occupied_at_pos(dx, dy) {
                    if x + dx as usize >= 20 || y + dy as usize >= 20 {
                        continue;
                    }

                    self.set_square(x + dx as usize, y + dy as usize, square.clone());
                }
            }
        }
    }

    pub fn place_special_piece(
        &mut self,
        shape: &SpecialShape,
        color: &Colors,
        x: isize,
        y: isize,
    ) {
        let square = color.to_square();

        for dx in 0..shape.width {
            for dy in 0..shape.height {
                if shape.occupied_at_pos(dx, dy) {
                    if x + dx as isize >= 20 || y + dy as isize >= 20 {
                        continue;
                    }

                    self.set_square(
                        (x + dx as isize) as usize,
                        (y + dy as isize) as usize,
                        square.clone(),
                    );
                }
            }
        }
    }
}
