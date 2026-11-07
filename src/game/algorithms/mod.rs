use crate::game::{
    board::GameBoard,
    pieces::{Colors, Pieces, PlacedPiece},
};

pub mod barasona_opening;
pub mod largest_first;
pub mod random;

#[derive(Debug)]
#[enum_dispatch::enum_dispatch(Algorithm)]
pub enum Algorithms {
    Random(random::RandomAlgorithm),
    LargestFirst(largest_first::LargestFirstAlgorithm),
    BarasonaOpening(barasona_opening::BarasonaOpeningAlgorithm),
}

#[enum_dispatch::enum_dispatch]
pub trait Algorithm {
    fn get_next_piece<'a>(
        &'a mut self,
        available_pieces: &mut Vec<Pieces>,
        placed_pieces: &Vec<Pieces>,
        open_corners: &mut Vec<(i32, i32)>,
        color: &Colors,
        board: &mut GameBoard,
        rng: &mut fastrand::Rng,
    ) -> Option<PlacedPiece<'a>>;

    fn reset(&mut self);
}
