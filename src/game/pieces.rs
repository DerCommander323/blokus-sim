use std::sync::LazyLock;

use strum::IntoEnumIterator;

use crate::game::{board, pieces::Pieces::*};

/// See https://www.spiele4us.de/wp-content/uploads/2022/10/blokus-n002700060.pdf
#[derive(Debug, Clone, strum::Display, strum::EnumIter, strum::FromRepr)]
pub enum Pieces {
    /// 1 Square
    Single1 = 1,

    /// 2 Squares
    Bar2 = 2,

    /// 3 Squares
    Bar3 = 3,
    Corner3 = 4,

    /// 4 Squares
    Bar4 = 5,
    L4 = 6,
    T4 = 7,
    Square4 = 8,
    Z4 = 9,

    /// 5 Squares
    Bar5 = 10,
    L5 = 11,
    Z5 = 12,
    P5 = 13,
    Bracket5 = 14,
    WeirdBar5 = 15,
    T5 = 16,
    Corner5 = 17,
    W5 = 18,
    LongS5 = 19,
    Thingy5 = 20,
    Plus5 = 21,
}

pub const PIECE_OCCUPATIONS: LazyLock<Vec<Occupation>> = LazyLock::new(|| {
    Pieces::iter()
        .map(|p| calculate_piece_occupation(&p))
        .collect::<Vec<Occupation>>()
});

impl Pieces {
    pub fn get_id(&self) -> usize {
        self.clone() as usize
    }
}

#[rustfmt::skip]
pub fn calculate_piece_occupation(piece: &Pieces) -> Occupation {
    match piece {
        Single1 => Occupation::new_1x1(true),
        Bar2 => Occupation::new(1, 2, &[
            true,
            true
        ]),
        Bar3 => Occupation::new(1, 3, &[
            true,
            true,
            true
        ]),
        Corner3 => Occupation::new_2x2([
            true, false,
            true, true
        ]),
        Bar4 => Occupation::new(1, 4, &[
            true,
            true,
            true,
            true
        ]),
        L4 => Occupation::new(2, 3, &[
            true, false,
            true, false,
            true, true
        ]),
        T4 => Occupation::new(3, 2, &[
            true, true, true,
            false, true, false
        ]),
        Square4 => Occupation::new_2x2([
            true, true,
            true, true
        ]),
        Z4 => Occupation::new(3, 2, &[
            true, true, false,
            false, true, true
        ]),
        Bar5 => Occupation::new(1, 5, &[
            true,
            true,
            true,
            true,
            true
        ]),
        L5 => Occupation::new(2, 4, &[
            true, false,
            true, false,
            true, false,
            true, true,
        ]),
        Z5 => Occupation::new(2, 4, &[
            false, true,
            false, true,
            true, true,
            true, false
        ]),
        P5 => Occupation::new(2, 3, &[
            false, true,
            true, true,
            true, true
        ]),
        Bracket5 => Occupation::new(2, 3, &[
            true, true,
            false, true,
            true, true
        ]),
        WeirdBar5 => Occupation::new(2, 4, &[
            true, false,
            true, false,
            true, true,
            true, false
        ]),
        T5 => Occupation::new_3x3([
            true, true, true,
            false, true, false,
            false, true, false
        ]),
        Corner5 => Occupation::new_3x3([
            true, false, false,
            true, false, false,
            true, true, true
        ]),
        W5 => Occupation::new_3x3([
            true, true, false,
            false, true, true,
            false, false, true
        ]),
        LongS5 => Occupation::new_3x3([
            true, false, false,
            true, true, true,
            false, false, true
        ]),
        Thingy5 => Occupation::new_3x3([
            true, false, false,
            true, true, true,
            false, true, false,
        ]),
        Plus5 => Occupation::new_3x3([
            false, true, false,
            true, true, true,
            false, true, false
        ]),
    }
}

#[derive(Debug, Clone)]
pub struct Occupation {
    pub width: u8,
    pub height: u8,
    pub map: Vec<bool>,
    pub rotation: Rotation,
}

impl Occupation {
    pub fn new(width: u8, height: u8, occupied: &[bool]) -> Occupation {
        assert_eq!(occupied.len() as u8, width * height);

        Self {
            width,
            height,
            map: occupied.to_vec(),
            rotation: Rotation::Normal,
        }
    }

    pub fn rotate_right(&mut self) {
        let count = self.map.len();
        let original_map = self.map.clone();

        let mut walking_index = 0;
        for dx in 0..self.width as usize {
            for dy in 0..self.height as usize {
                let temp = count - ((dy + 1) * self.width as usize);
                let original_index = temp + dx;

                self.map[walking_index] = original_map[original_index];

                walking_index += 1;
            }
        }

        let old_h = self.height;
        self.height = self.width;
        self.width = old_h;
        self.rotation = match self.rotation {
            Rotation::Normal => Rotation::Cw90,
            Rotation::Cw90 => Rotation::Cw180,
            Rotation::Cw180 => Rotation::Cw270,
            Rotation::Cw270 => Rotation::Normal,
        }

        // Rotating the indices:
        // 0 1 2 3
        // 4 5 6 7

        // 4 0
        // 5 1
        // 6 2
        // 7 3
        // = 40516273

        // 7 6 5 4
        // 3 2 1 0
        // = 76543210

        // 0 1 2
        // 3 4 5
        // 6 7 8

        // = 630741852

        // 0 1 2 3
        // 4 5 6 7
        // 8 9 10 11

        // = 8 4 0 9 5 1 10 6 2 11 7 3
    }

    pub fn mirror_vertically(&mut self) {
        let original_map = self.map.clone();

        let mut walking_index = 0;
        for dy in 0..self.height as usize {
            for dx in 0..self.width as usize {
                let original_index = (self.width as usize * (dy + 1)) - dx - 1;

                self.map[walking_index] = original_map[original_index];

                walking_index += 1;
            }
        }

        // 0 1 2
        // 3 4 5
        // 6 7 8
        // = 210 543 876

        // 0 1 2 3
        // 4 5 6 7
        // = 3210 7654
    }

    /// Same effect as horizontal mirroring but implemented the lazy way with one vertical mirror and 2 rotates
    pub fn mirror_horizontally_fake(&mut self) {
        self.mirror_vertically();
        self.rotate_right();
        self.rotate_right();
    }

    pub fn map_index(&self, x: u8, y: u8) -> u8 {
        let index = x + y * self.width;

        debug_assert!((index as usize) < self.map.len());

        index
    }

    pub fn occupied_at_pos(&self, x: u8, y: u8) -> bool {
        debug_assert!(x <= self.width);
        debug_assert!(y <= self.height);

        self.map[self.map_index(x, y) as usize]
    }

    pub fn new_1x1(occupied: bool) -> Self {
        Self {
            width: 1,
            height: 1,
            map: vec![occupied],
            rotation: Rotation::Normal,
        }
    }

    pub fn new_2x2(occupied: [bool; 4]) -> Self {
        Self {
            width: 2,
            height: 2,
            map: occupied.to_vec(),
            rotation: Rotation::Normal,
        }
    }

    pub fn new_3x3(occupied: [bool; 9]) -> Self {
        Self {
            width: 3,
            height: 3,
            map: occupied.to_vec(),
            rotation: Rotation::Normal,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Rotation {
    Normal,
    Cw90,
    Cw180,
    Cw270,
}

#[derive(Debug, Clone)]
pub enum Colors {
    Blue,
    Green,
    Red,
    Yellow,
}

impl Colors {
    pub fn to_rgb(&self) -> u32 {
        match self {
            Colors::Blue => 0x0000FF,
            Colors::Green => 0x00FF00,
            Colors::Red => 0xFF0000,
            Colors::Yellow => 0xFFFF00,
        }
    }

    pub fn to_square(&self) -> board::Square {
        match self {
            Colors::Blue => board::Square::Blue,
            Colors::Green => board::Square::Green,
            Colors::Red => board::Square::Red,
            Colors::Yellow => board::Square::Yellow,
        }
    }
}
