use crate::game::{
    board::GameBoard,
    pieces::{Pieces, PlacedPiece, Shape},
    simulator::Player,
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
        placed_pieces: &Vec<Pieces>,
        open_corners: &mut Vec<(usize, usize)>,
        board: &mut GameBoard,
    ) -> PlacedPiece;
}
