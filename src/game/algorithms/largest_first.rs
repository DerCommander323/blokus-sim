use std::collections::HashMap;

use crate::game::{
    algorithms::{Algorithm, Algorithms},
    board::GameBoard,
    pieces::{Colors, Pieces, PlacedPiece, SPECIAL_SHAPES_BY_PIECE, SpecialShape},
};

#[derive(Debug)]
pub struct LargestFirstAlgorithm {
    shapes_by_piece: HashMap<Pieces, Vec<SpecialShape>>,
}

impl LargestFirstAlgorithm {
    pub fn new() -> Algorithms {
        Algorithms::LargestFirst(LargestFirstAlgorithm {
            shapes_by_piece: SPECIAL_SHAPES_BY_PIECE.clone(),
        })
    }
}

impl Algorithm for LargestFirstAlgorithm {
    fn get_next_piece<'a>(
        &'a mut self,
        available_pieces: &mut Vec<Pieces>,
        _placed_pieces: &Vec<Pieces>,
        open_corners: &mut Vec<(i32, i32)>,
        color: &Colors,
        board: &mut GameBoard,
        rng: &mut fastrand::Rng,
    ) -> Option<PlacedPiece<'a>> {
        available_pieces.sort_by_cached_key(|p| 5 - p.count_squares());

        rng.shuffle(open_corners);
        for values in self.shapes_by_piece.values_mut() {
            rng.shuffle(values);
            for v in values {
                rng.shuffle(&mut v.inner_corners);
            }
        }

        for piece in available_pieces {
            for outer_corner in open_corners.iter() {
                let variations = self.shapes_by_piece.get(piece).unwrap();

                for shape in variations {
                    for inner_corner in &shape.inner_corners {
                        let x = outer_corner.0 - inner_corner.0;
                        let y = outer_corner.1 - inner_corner.1;

                        if board.can_place_piece(&shape, color, x, y) {
                            return Some(PlacedPiece {
                                piece: piece.clone(),
                                shape: shape,
                                used_corner: *outer_corner,
                                x: x,
                                y: y,
                            });
                        }
                    }
                }
            }
        }

        None
    }

    fn reset(&mut self) {}
}
