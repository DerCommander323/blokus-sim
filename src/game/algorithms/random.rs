use ply_engine::prelude::rand::ChooseRandom;
use strum::IntoEnumIterator;

use crate::game::{
    algorithms::{Algorithm, Algorithms},
    board::GameBoard,
    pieces::{Colors, Pieces, PlacedPiece, SPECIAL_SHAPES_BY_PIECE},
};

#[derive(Debug)]
pub struct RandomAlgorithm {
    available_pieces: Vec<Pieces>,
}

impl RandomAlgorithm {
    pub fn new() -> Algorithms {
        Algorithms::Random(RandomAlgorithm {
            available_pieces: Pieces::iter().collect(),
        })
    }
}

impl Algorithm for RandomAlgorithm {
    fn get_next_piece(
        &mut self,
        available_pieces: &mut Vec<Pieces>,
        placed_pieces: &Vec<Pieces>,
        open_corners: &mut Vec<(i32, i32)>,
        color: &Colors,
        board: &mut GameBoard,
    ) -> Option<PlacedPiece> {
        // Select a random piece, a random open corner and a random rotation for the piece and try until the piece fits

        fastrand::shuffle(available_pieces);
        fastrand::shuffle(open_corners);

        for piece in available_pieces {
            for outer_corner in &*open_corners {
                for shape in SPECIAL_SHAPES_BY_PIECE.get(piece).unwrap() {
                    for inner_corner in &shape.inner_corners {
                        let x = outer_corner.0 - inner_corner.0;
                        let y = outer_corner.1 - inner_corner.1;

                        if board.can_place_piece(&shape, color, x, y) {
                            return Some(PlacedPiece {
                                piece: piece.clone(),
                                shape: shape.clone(),
                                used_corner: *outer_corner,
                                x: x,
                                y: y,
                            });
                        }
                    }
                }
            }
        }

        println!("{color} is out of pieces",);
        None
    }
}
