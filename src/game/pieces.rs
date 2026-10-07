use crate::game::pieces::Pieces::*;

/// See https://www.spiele4us.de/wp-content/uploads/2022/10/blokus-n002700060.pdf
#[derive(Debug, strum::Display, strum::EnumIter)]
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

#[rustfmt::skip]
pub fn get_occupation(piece: &Pieces) -> Occupation {
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

pub struct Occupation {
    pub width: u8,
    pub height: u8,
    pub map: Vec<bool>,
}

impl Occupation {
    pub fn new(width: u8, height: u8, occupied: &[bool]) -> Occupation {
        assert_eq!(occupied.len() as u8, width * height);

        Self {
            width,
            height,
            map: occupied.to_vec(),
        }
    }

    pub fn new_1x1(occupied: bool) -> Self {
        Self {
            width: 1,
            height: 1,
            map: vec![occupied],
        }
    }

    pub fn new_2x2(occupied: [bool; 4]) -> Self {
        Self {
            width: 2,
            height: 2,
            map: occupied.to_vec(),
        }
    }

    pub fn new_3x3(occupied: [bool; 9]) -> Self {
        Self {
            width: 3,
            height: 3,
            map: occupied.to_vec(),
        }
    }
}
