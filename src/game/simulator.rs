use std::sync::{Arc, Mutex};

use strum::IntoEnumIterator;

use crate::{
    bounds_check,
    game::{
        algorithms::{Algorithm, Algorithms, random::RandomAlgorithm},
        board::GameBoard,
        pieces::{Colors, Pieces, PlacedPiece},
        statistics::Statistics,
    },
};

pub struct Simulator {
    board: GameBoard,
    overlay_board: GameBoard,
    rng: fastrand::Rng,
    player_blue: Player,
    player_green: Player,
    player_red: Player,
    player_yellow: Player,
    current_turn: Colors,
}

impl Simulator {
    pub fn new(rng: fastrand::Rng) -> Self {
        Self {
            board: GameBoard::new(),
            overlay_board: GameBoard::new(),
            rng,
            player_blue: Player::new(Colors::Blue, (0, 0)),
            player_green: Player::new(Colors::Green, (0, 19)),
            player_red: Player::new(Colors::Red, (19, 19)),
            player_yellow: Player::new(Colors::Yellow, (19, 0)),
            current_turn: Colors::Blue,
        }
    }

    pub fn run_full(&mut self, statistics: &Arc<Mutex<Statistics>>) {
        let thread_count = std::thread::available_parallelism().unwrap().get() - 1;
        println!("Running simulations on {thread_count} threads");

        for i in 0..thread_count {
            let stat_clone = statistics.clone();

            std::thread::Builder::new()
                .name(format!("sim thread {i}"))
                .spawn(move || {
                    let mut simulator = Simulator::new(fastrand::Rng::with_seed(i as u64));

                    loop {
                        simulator.step(&stat_clone);

                        #[cfg(feature = "tracy")]
                        tracy_client::frame_mark();
                    }
                })
                .unwrap();
        }

        let stat_clone = statistics.clone();
        let target_games = 1_000_000;

        std::thread::Builder::new()
            .name("Sim watcher thread".to_string())
            .spawn(move || {
                let mut gps_accumulated = Vec::new();

                loop {
                    let prev_games = stat_clone.lock().unwrap().finished_games;

                    std::thread::sleep(std::time::Duration::from_secs(1));

                    let new_games = stat_clone.lock().unwrap().finished_games;
                    let delta = new_games - prev_games;
                    println!("Finished {} games per second", delta);

                    gps_accumulated.push(delta);
                    let average =
                        gps_accumulated.iter().sum::<i32>() as f64 / gps_accumulated.len() as f64;
                    println!("Average gps: {average:.1}");

                    let percentage = new_games as f64 / target_games as f64;
                    println!(
                        "{new_games} out of {target_games} finished: {:.1}% complete",
                        percentage * 100.
                    );

                    if new_games > target_games {
                        println!("Finished running {new_games} games, exiting...");
                        stat_clone.lock().unwrap().print();
                        break;
                    }
                }
            })
            .unwrap();
    }

    pub fn step(&mut self, statistics: &Arc<Mutex<Statistics>>) {
        #[cfg(feature = "tracy")]
        let _zone = tracy_client::span!("Simulator step");

        match self.current_turn {
            Colors::Blue => {
                self.player_blue
                    .place_next_piece(&mut self.board, &mut self.rng);
                self.current_turn = Colors::Yellow
            }
            Colors::Yellow => {
                self.player_yellow
                    .place_next_piece(&mut self.board, &mut self.rng);
                self.current_turn = Colors::Red
            }
            Colors::Red => {
                self.player_red
                    .place_next_piece(&mut self.board, &mut self.rng);
                self.current_turn = Colors::Green
            }
            Colors::Green => {
                self.player_green
                    .place_next_piece(&mut self.board, &mut self.rng);
                self.current_turn = Colors::Blue
            }
        };
        if self.player_blue.is_finished
            && self.player_yellow.is_finished
            && self.player_red.is_finished
            && self.player_green.is_finished
        {
            self.end_game(statistics);
            self.reset();
        }
    }

    pub fn end_game(&self, statistics: &Arc<Mutex<Statistics>>) {
        let mut all_players = [
            (&self.player_blue, self.player_blue.calculate_score()),
            (&self.player_yellow, self.player_yellow.calculate_score()),
            (&self.player_red, self.player_red.calculate_score()),
            (&self.player_green, self.player_green.calculate_score()),
        ];

        all_players.sort_by(|(_, score_a), (_, score_b)| score_a.cmp(score_b));

        let (winner, _winning_score) = all_players[3];

        let mut stat_lock = statistics.lock().unwrap();
        stat_lock.finished_games += 1;

        match winner.color {
            Colors::Blue => {
                stat_lock.amount_wins.blue += 1;
            }
            Colors::Yellow => {
                stat_lock.amount_wins.yellow += 1;
            }
            Colors::Red => {
                stat_lock.amount_wins.red += 1;
            }
            Colors::Green => {
                stat_lock.amount_wins.green += 1;
            }
        }
    }

    pub fn reset(&mut self) {
        let new_rng = self.rng.fork();
        *self = Self::new(new_rng);
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
    pub is_finished: bool,
}

impl Player {
    pub fn new(color: Colors, starting_position: (i32, i32)) -> Self {
        Self {
            color,
            available_pieces: Pieces::iter().collect(),
            placed_pieces: Vec::new(),
            open_corners: vec![starting_position],
            algorithm: RandomAlgorithm::new(),
            is_finished: false,
        }
    }

    pub fn get_open_corners(&self) -> &Vec<(i32, i32)> {
        &self.open_corners
    }

    pub fn calculate_score(&self) -> i32 {
        debug_assert!(
            self.is_finished,
            "Player {} was not finished when calculating score!",
            self.color
        );

        let mut score = 0;

        if self.available_pieces.is_empty() {
            score += 15;
            if *self.placed_pieces.last().unwrap() == Pieces::Single1 {
                score += 20;
            }
        } else {
            for piece in &self.available_pieces {
                score -= piece.count_squares();
            }
        }

        // if score > 15 {
        //     dbg!(&self.available_pieces, score);
        //     std::thread::sleep(std::time::Duration::from_secs(10));
        // }

        score
    }

    pub fn place_next_piece(&mut self, board: &mut GameBoard, rng: &mut fastrand::Rng) {
        #[cfg(feature = "tracy")]
        let _zone = tracy_client::span!("player step");

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
            rng,
        )
        else {
            self.is_finished = true;
            return;
        };

        #[cfg(feature = "tracy")]
        let _zone2 = tracy_client::span!("piece placement");

        // println!("Placing {}", piece.to_string());
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
