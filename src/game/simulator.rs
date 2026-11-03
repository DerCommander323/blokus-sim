use strum::IntoEnumIterator;

use crate::{
    bounds_check,
    game::{
        algorithms::{Algorithm, Algorithms, random::RandomAlgorithm},
        board::GameBoard,
        pieces::{Colors, Pieces, PlacedPiece, Shape},
    },
};

pub struct Simulator {
    board: GameBoard,
    overlay_board: GameBoard,
    player_blue: Player,
    player_green: Player,
    player_red: Player,
    player_yellow: Player,
    current_turn: Colors,
}

impl Simulator {
    pub fn new() -> Self {
        Self {
            board: GameBoard::new(),
            overlay_board: GameBoard::new(),
            player_blue: Player::new(Colors::Blue, (0, 0)),
            player_green: Player::new(Colors::Green, (0, 19)),
            player_red: Player::new(Colors::Red, (19, 19)),
            player_yellow: Player::new(Colors::Yellow, (19, 0)),
            current_turn: Colors::Blue,
        }
    }

    pub fn step(&mut self) {
        match self.current_turn {
            Colors::Blue => {
                self.player_blue.place_next_piece(&mut self.board);
                self.current_turn = Colors::Yellow
            }
            Colors::Yellow => {
                self.player_yellow.place_next_piece(&mut self.board);
                self.current_turn = Colors::Red
            }
            Colors::Red => {
                self.player_red.place_next_piece(&mut self.board);
                self.current_turn = Colors::Green
            }
            Colors::Green => {
                self.player_green.place_next_piece(&mut self.board);
                self.current_turn = Colors::Blue
            }
        };
    }

    pub fn place_open_corner_overlay(&mut self) {
        let all = self.all_open_corners();

        for (color, corners) in all {
            for (x, y) in corners.clone() {
                self.overlay_board.set_square(x, y, color.to_square());
            }
        }
    }

    pub fn all_open_corners(&self) -> [(Colors, Vec<(i32, i32)>); 4] {
        [
            (Colors::Blue, self.player_blue.get_open_corners().clone()),
            (
                Colors::Yellow,
                self.player_yellow.get_open_corners().clone(),
            ),
            (Colors::Red, self.player_red.get_open_corners().clone()),
            (Colors::Green, self.player_green.get_open_corners().clone()),
        ]
    }

    pub fn board(&self) -> &GameBoard {
        &self.board
    }

    pub fn board_mut(&mut self) -> &mut GameBoard {
        &mut self.board
    }

    pub fn overlay_board_mut(&mut self) -> &mut GameBoard {
        &mut self.overlay_board
    }
}

pub struct Player {
    pub color: Colors,
    pub available_pieces: Vec<Pieces>,
    pub placed_pieces: Vec<Pieces>,
    pub open_corners: Vec<(i32, i32)>,
    pub algorithm: Algorithms,
}

impl Player {
    pub fn new(color: Colors, starting_position: (i32, i32)) -> Self {
        Self {
            color,
            available_pieces: Pieces::iter().collect(),
            placed_pieces: Vec::new(),
            open_corners: vec![starting_position],
            algorithm: RandomAlgorithm::new(),
        }
    }

    pub fn get_open_corners(&self) -> &Vec<(i32, i32)> {
        &self.open_corners
    }

    pub fn place_next_piece(&mut self, board: &mut GameBoard) {
        let Some(PlacedPiece {
            piece,
            shape,
            used_corner,
            x,
            y,
        }) = self.algorithm.get_next_piece(
            &mut self.available_pieces,
            &self.placed_pieces,
            &mut self.open_corners,
            &self.color,
            board,
        )
        else {
            return;
        };

        println!("Placing {}", piece.to_string());
        board.place_special_piece(&shape, &self.color, x, y);
        self.open_corners
            .extract_if(.., |crnr| *crnr == used_corner)
            .for_each(drop);
        for new_corner in shape.outer_corners {
            let nx = new_corner.0 + x;
            let ny = new_corner.1 + y;

            if bounds_check(nx, ny) {
                self.open_corners.push((nx, ny));
            }
        }
        self.available_pieces
            .extract_if(.., |pc| *pc == piece)
            .for_each(drop);
        self.placed_pieces.push(piece);
    }
}
