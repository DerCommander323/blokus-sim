use ply_engine::prelude::rand::ChooseRandom;
use strum::IntoEnumIterator;

use crate::game::{
    algorithms::{Algorithm, Algorithms},
    board::GameBoard,
    pieces::{Colors, Pieces, PlacedPiece, SHAPES_BY_ROTATION, SPECIAL_SHAPES_BY_PIECE},
    simulator::Player,
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
        placed_pieces: &Vec<Pieces>,
        open_corners: &mut Vec<(usize, usize)>,
        color: &Colors,
        board: &mut GameBoard,
    ) -> PlacedPiece {
        // Select a random piece, a random open corner and a random rotation for the piece and try until the piece fits

        self.available_pieces.shuffle();
        open_corners.shuffle();

        let mut pieces = self.available_pieces.clone();
        let mut corners = open_corners.clone();

        for piece in &pieces {
            for outer_corner in &corners {
                for shape in SPECIAL_SHAPES_BY_PIECE.get(piece).unwrap() {
                    for inner_corner in &shape.inner_corners {
                        let x = outer_corner.0 as isize - inner_corner.0 as isize;
                        let y = outer_corner.1 as isize - inner_corner.1 as isize;

                        dbg!(x, y);

                        if board.can_place_piece(&shape, color, x - 1, y - 1) {
                            return PlacedPiece {
                                piece: piece.clone(),
                                shape: shape.clone(),
                                x: x,
                                y: y,
                            };
                        }
                    }
                }
            }
        }
        panic!("out of pieces")
    }
}
