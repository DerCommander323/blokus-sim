use crate::{
    bounds_check,
    game::pieces::{Colors, Shape, SpecialShape, SpecialShapePart},
    hard_bounds_check,
};

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

    pub fn get_square(&self, x: i32, y: i32) -> &Square {
        hard_bounds_check(x, y);
        &self.get_squares()[x as usize][y as usize]
    }

    pub fn set_square(&mut self, x: i32, y: i32, square: Square) {
        hard_bounds_check(x, y);
        self.squares[x as usize][y as usize] = square;
    }

    pub fn can_place_piece(&self, shape: &SpecialShape, color: &Colors, x: i32, y: i32) -> bool {
        #[cfg(feature = "tracy")]
        let _zone = tracy_client::span!("piece placement check");

        #[cfg(feature = "tracy")]
        let _zone2 = tracy_client::span!("inbounds check");
        // 22 because the SpecialShapes are extended by 2 in both directions
        if x <= -2 || x + shape.width > 22 || y <= -2 || y + shape.height > 22 {
            return false;
        }
        #[cfg(feature = "tracy")]
        drop(_zone2);

        let mut ret = false;

        #[cfg(feature = "tracy")]
        let _zone2 = tracy_client::span!("square check");
        for dx in 0..shape.width {
            for dy in 0..shape.height {
                let board_x = dx + x;
                let board_y = dy + y;

                match shape.value_at_pos(dx, dy) {
                    SpecialShapePart::Adjacent___ => {
                        if bounds_check(board_x, board_y) {
                            if self.get_square(board_x, board_y) == &color.to_square() {
                                return false;
                            }
                        }
                    }
                    SpecialShapePart::InnerCorner | SpecialShapePart::Normal_____ => {
                        if bounds_check(board_x, board_y) {
                            if self.get_square(board_x, board_y) != &Square::Empty {
                                return false;
                            }
                        } else {
                            return false;
                        }
                    }
                    _ => {
                        ret = true;
                    }
                }
            }
        }

        ret
    }

    pub fn place_piece(&mut self, shape: &Shape, color: &Colors, x: i32, y: i32) {
        let square = color.to_square();

        for dx in 0..shape.width {
            for dy in 0..shape.height {
                if shape.occupied_at_pos(dx, dy) {
                    if x + dx >= 20 || y + dy >= 20 {
                        continue;
                    }

                    self.set_square(x + dx, y + dy, square.clone());
                }
            }
        }
    }

    pub fn place_special_piece(&mut self, shape: &SpecialShape, color: &Colors, x: i32, y: i32) {
        let square = color.to_square();

        for dx in 0..shape.width {
            for dy in 0..shape.height {
                if shape.occupied_at_pos(dx, dy) {
                    if x + dx >= 20 || y + dy >= 20 {
                        continue;
                    }

                    self.set_square(x + dx, y + dy, square.clone());
                }
            }
        }
    }
}
