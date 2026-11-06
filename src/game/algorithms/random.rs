use crate::game::{
    algorithms::{Algorithm, Algorithms},
    board::GameBoard,
    pieces::{Colors, Pieces, PlacedPiece, SPECIAL_SHAPES_BY_PIECE},
};

#[derive(Debug)]
pub struct RandomAlgorithm {}

impl RandomAlgorithm {
    pub fn new() -> Algorithms {
        Algorithms::Random(RandomAlgorithm {})
    }
}

impl Algorithm for RandomAlgorithm {
    fn get_next_piece<'a>(
        &mut self,
        available_pieces: &mut Vec<Pieces>,
        _placed_pieces: &Vec<Pieces>,
        open_corners: &mut Vec<(i32, i32)>,
        color: &Colors,
        board: &mut GameBoard,
        rng: &mut fastrand::Rng,
    ) -> Option<PlacedPiece<'a>> {
        // Select a random piece, a random open corner and a random rotation for the piece and try until the piece fits
        #[cfg(feature = "tracy")]
        let _zone = tracy_client::span!("RandomAlgorithm step");

        #[cfg(feature = "tracy")]
        let _zone2 = tracy_client::span!("array shuffling");
        rng.shuffle(available_pieces);
        rng.shuffle(open_corners);
        #[cfg(feature = "tracy")]
        drop(_zone2);

        #[cfg(feature = "tracy")]
        let _zone3 = tracy_client::span!("available piece iteration");
        #[cfg(feature = "tracy")]
        _zone3.emit_value(available_pieces.len() as u64);

        for piece in available_pieces {
            #[cfg(feature = "tracy")]
            let _zone4 = tracy_client::span!("outer corner iteration");
            #[cfg(feature = "tracy")]
            _zone4.emit_value(open_corners.len() as u64);

            for outer_corner in open_corners.iter() {
                let variations = SPECIAL_SHAPES_BY_PIECE.get(piece).unwrap();

                #[cfg(feature = "tracy")]
                let _zone5 = tracy_client::span!("variant iteration");
                #[cfg(feature = "tracy")]
                _zone5.emit_value(variations.len() as u64);

                for shape in variations {
                    #[cfg(feature = "tracy")]
                    let _zone6 = tracy_client::span!("inner corner iteration");
                    #[cfg(feature = "tracy")]
                    _zone6.emit_value(shape.inner_corners.len() as u64);

                    for inner_corner in &shape.inner_corners {
                        #[cfg(feature = "tracy")]
                        let _zone6 = tracy_client::span!("attempted piece placement");

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
                    #[cfg(feature = "tracy")]
                    drop(_zone6);
                }
                #[cfg(feature = "tracy")]
                drop(_zone5);
            }
            #[cfg(feature = "tracy")]
            drop(_zone4);
        }
        #[cfg(feature = "tracy")]
        drop(_zone3);

        // println!("{color} is out of pieces",);
        None
    }

    fn reset(&mut self) {}
}
