use ply_engine::prelude::rand::ChooseRandom;
use strum::IntoEnumIterator;

use crate::game::{
    algorithms::{Algorithm, Algorithms},
    board::GameBoard,
    pieces::{Pieces, PlacedPiece},
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
        board: &mut GameBoard,
    ) -> PlacedPiece {
        // Select a random piece, a random open corner and a random rotation for the piece and try until the piece fits

        self.available_pieces.shuffle();
        open_corners.shuffle();

        let mut pieces = self.available_pieces.clone();
        let mut corners = open_corners.clone();

        while let Some(piece) = pieces.pop() {
            for open_corner in &corners {}
        }
        panic!("out of pieces")
    }
}
