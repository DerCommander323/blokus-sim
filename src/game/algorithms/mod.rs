use crate::game::{
    board::GameBoard,
    pieces::{Colors, Pieces, PlacedPiece},
};

pub mod random;

#[derive(Debug)]
#[enum_dispatch::enum_dispatch(Algorithm)]
pub enum Algorithms {
    Random(random::RandomAlgorithm),
}

#[enum_dispatch::enum_dispatch]
pub trait Algorithm {
    fn get_next_piece(
        &mut self,
        available_pieces: &mut Vec<Pieces>,
        placed_pieces: &Vec<Pieces>,
        open_corners: &mut Vec<(i32, i32)>,
        color: &Colors,
        board: &mut GameBoard,
        rng: &mut fastrand::Rng,
    ) -> Option<PlacedPiece>;
}
