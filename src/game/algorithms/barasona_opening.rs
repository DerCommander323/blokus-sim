use std::collections::HashMap;

use crate::game::{
    algorithms::{Algorithm, Algorithms},
    board::GameBoard,
    pieces::{Colors, Pieces, PlacedPiece, SPECIAL_SHAPES_BY_PIECE, SpecialShape},
};

#[derive(Debug)]
pub struct BarasonaOpeningAlgorithm {
    shapes_by_piece: HashMap<Pieces, Vec<SpecialShape>>,
    turn_count: i32,
    starting_corner: (i32, i32),
}

impl BarasonaOpeningAlgorithm {
    pub fn new() -> Algorithms {
        Algorithms::BarasonaOpening(BarasonaOpeningAlgorithm {
            shapes_by_piece: SPECIAL_SHAPES_BY_PIECE.clone(),
            turn_count: 1,
            starting_corner: (-1, -1),
        })
    }
}

impl Algorithm for BarasonaOpeningAlgorithm {
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

        match self.turn_count {
            1 => {
                self.starting_corner = open_corners[0];
                let piece = &Pieces::Thingy5;

                Some(self.fit_piece(board, piece, self.starting_corner, color))
            }
            2 => {
                let piece = &Pieces::Plus5;

                let position = match self.starting_corner {
                    (0, 0) => (2, 3),
                    (0, 19) => (2, 16),
                    (19, 19) => (16, 17),
                    (19, 0) => (16, 2),
                    _ => panic!(),
                };

                Some(self.fit_piece(board, piece, position, color))
            }

            3 => {
                let piece = Pieces::W5;
                let placed = match self.starting_corner {
                    (0, 0) => PlacedPiece {
                        shape: &SPECIAL_SHAPES_BY_PIECE.get(&piece).unwrap()[3],
                        piece,
                        used_corner: (-1, -1),
                        x: 3,
                        y: 3,
                    },
                    (0, 19) => PlacedPiece {
                        shape: &SPECIAL_SHAPES_BY_PIECE.get(&piece).unwrap()[2],
                        piece,
                        used_corner: (-1, -1),
                        x: 3,
                        y: 12,
                    },
                    (19, 19) => PlacedPiece {
                        shape: &SPECIAL_SHAPES_BY_PIECE.get(&piece).unwrap()[1],
                        piece,
                        used_corner: (-1, -1),
                        x: 12,
                        y: 12,
                    },
                    (19, 0) => PlacedPiece {
                        shape: &SPECIAL_SHAPES_BY_PIECE.get(&piece).unwrap()[0],
                        piece,
                        used_corner: (-1, -1),
                        x: 12,
                        y: 3,
                    },
                    _ => panic!("unknown startin corner {:?}", self.starting_corner),
                };

                self.turn_count += 1;

                Some(placed)
            }
            _ => {
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
                                    self.turn_count += 1;

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
        }
    }

    fn reset(&mut self) {
        self.shapes_by_piece = SPECIAL_SHAPES_BY_PIECE.clone();
        self.turn_count = 1;
    }
}

impl BarasonaOpeningAlgorithm {
    fn fit_piece<'a>(
        &'a mut self,
        board: &GameBoard,
        piece: &Pieces,
        position: (i32, i32),
        color: &Colors,
    ) -> PlacedPiece<'a> {
        let variations = self.shapes_by_piece.get(piece).unwrap();

        for shape in variations {
            for inner_corner in &shape.inner_corners {
                let x = position.0 - inner_corner.0;
                let y = position.1 - inner_corner.1;

                if board.can_place_piece(&shape, color, x, y) {
                    self.turn_count += 1;

                    return PlacedPiece {
                        piece: piece.clone(),
                        shape: shape,
                        used_corner: position,
                        x: x,
                        y: y,
                    };
                }
            }
        }
        panic!("Couldnt fit piece {piece}");
    }
}
