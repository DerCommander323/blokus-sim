use std::{collections::HashMap, sync::LazyLock};

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

pub static PIECE_SHAPES: LazyLock<Vec<Shape>> = LazyLock::new(|| {
    Pieces::iter()
        .map(|p| calculate_piece_shape(&p))
        .collect::<Vec<Shape>>()
});

pub static PIECE_SPECIAL_SHAPES: LazyLock<Vec<SpecialShape>> = LazyLock::new(|| {
    Pieces::iter()
        .map(|p| calculate_special_piece_shape(&p))
        .collect::<Vec<SpecialShape>>()
});

pub static SHAPES_BY_ROTATION: LazyLock<HashMap<Rotation, Vec<SpecialShape>>> =
    LazyLock::new(|| {
        let mut map = HashMap::new();
        for rotation in Rotation::iter() {
            let closure: fn(SpecialShape) -> SpecialShape = match rotation {
                Rotation::Normal => |s| s,
                Rotation::Cw90 => |mut s| {
                    s.rotate_right();
                    s
                },
                Rotation::Cw180 => |mut s| {
                    s.rotate_right();
                    s.rotate_right();
                    s
                },
                Rotation::Cw270 => |mut s| {
                    s.rotate_right();
                    s.rotate_right();
                    s.rotate_right();
                    s
                },
                Rotation::MirroredNormal => |mut s| {
                    s.mirror_vertically();
                    s
                },
                Rotation::MirroredCw90 => |mut s| {
                    s.mirror_vertically();
                    s.rotate_right();
                    s
                },
                Rotation::MirroredCw180 => |mut s| {
                    s.mirror_vertically();
                    s.rotate_right();
                    s.rotate_right();
                    s
                },
                Rotation::MirroredCw270 => |mut s| {
                    s.mirror_vertically();
                    s.rotate_right();
                    s.rotate_right();
                    s.rotate_right();
                    s
                },
            };

            let iter = PIECE_SPECIAL_SHAPES.iter().cloned().map(closure);
            map.insert(rotation.clone(), iter.collect());
        }

        map
    });

impl Pieces {
    pub fn get_id(&self) -> usize {
        self.clone() as usize
    }
}

#[rustfmt::skip]
pub fn calculate_piece_shape(piece: &Pieces) -> Shape {
    match piece {
        Single1 => Shape::new_1x1(true),
        Bar2 => Shape::new(1, 2, &[
            true,
            true
        ]),
        Bar3 => Shape::new(1, 3, &[
            true,
            true,
            true
        ]),
        Corner3 => Shape::new_2x2([
            true, false,
            true, true
        ]),
        Bar4 => Shape::new(1, 4, &[
            true,
            true,
            true,
            true
        ]),
        L4 => Shape::new(2, 3, &[
            true, false,
            true, false,
            true, true
        ]),
        T4 => Shape::new(3, 2, &[
            true, true, true,
            false, true, false
        ]),
        Square4 => Shape::new_2x2([
            true, true,
            true, true
        ]),
        Z4 => Shape::new(3, 2, &[
            true, true, false,
            false, true, true
        ]),
        Bar5 => Shape::new(1, 5, &[
            true,
            true,
            true,
            true,
            true
        ]),
        L5 => Shape::new(2, 4, &[
            true, false,
            true, false,
            true, false,
            true, true,
        ]),
        Z5 => Shape::new(2, 4, &[
            false, true,
            false, true,
            true, true,
            true, false
        ]),
        P5 => Shape::new(2, 3, &[
            false, true,
            true, true,
            true, true
        ]),
        Bracket5 => Shape::new(2, 3, &[
            true, true,
            false, true,
            true, true
        ]),
        WeirdBar5 => Shape::new(2, 4, &[
            true, false,
            true, false,
            true, true,
            true, false
        ]),
        T5 => Shape::new_3x3([
            true, true, true,
            false, true, false,
            false, true, false
        ]),
        Corner5 => Shape::new_3x3([
            true, false, false,
            true, false, false,
            true, true, true
        ]),
        W5 => Shape::new_3x3([
            true, true, false,
            false, true, true,
            false, false, true
        ]),
        LongS5 => Shape::new_3x3([
            true, false, false,
            true, true, true,
            false, false, true
        ]),
        Thingy5 => Shape::new_3x3([
            true, false, false,
            true, true, true,
            false, true, false,
        ]),
        Plus5 => Shape::new_3x3([
            false, true, false,
            true, true, true,
            false, true, false
        ]),
    }
}

#[derive(Debug, Clone)]
pub struct Shape {
    pub width: u8,
    pub height: u8,
    pub map: Vec<bool>,
    pub rotation: Rotation,
}

impl Shape {
    pub fn new(width: u8, height: u8, occupied: &[bool]) -> Shape {
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

        self.rotation = self.rotation.rotated_right()

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

        // Toggle mirrored state
        self.rotation = self.rotation.mirrored_vertically()

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

#[derive(Debug, Clone, Hash, PartialEq, Eq, strum::EnumIter)]
pub enum Rotation {
    Normal,
    Cw90,
    Cw180,
    Cw270,
    MirroredNormal,
    MirroredCw90,
    MirroredCw180,
    MirroredCw270,
}

impl Rotation {
    /// Returns the Rotation in 90° clockwise
    pub fn rotated_right(&self) -> Self {
        match self {
            Rotation::Normal => Rotation::Cw90,
            Rotation::Cw90 => Rotation::Cw180,
            Rotation::Cw180 => Rotation::Cw270,
            Rotation::Cw270 => Rotation::Normal,
            Rotation::MirroredNormal => Rotation::MirroredCw90,
            Rotation::MirroredCw90 => Rotation::MirroredCw180,
            Rotation::MirroredCw180 => Rotation::MirroredCw270,
            Rotation::MirroredCw270 => Rotation::MirroredNormal,
        }
    }

    /// Returns the mirrored version of this rotation
    pub fn mirrored_vertically(&self) -> Self {
        match self {
            Rotation::Normal => Rotation::MirroredNormal,
            Rotation::Cw90 => Rotation::MirroredCw90,
            Rotation::Cw180 => Rotation::MirroredCw180,
            Rotation::Cw270 => Rotation::MirroredCw270,
            Rotation::MirroredNormal => Rotation::Normal,
            Rotation::MirroredCw90 => Rotation::Cw90,
            Rotation::MirroredCw180 => Rotation::Cw180,
            Rotation::MirroredCw270 => Rotation::Cw270,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Colors {
    Blue,
    Yellow,
    Red,
    Green,
}

impl Colors {
    pub fn to_rgb(&self) -> u32 {
        match self {
            Colors::Blue => 0x0000FF,
            Colors::Yellow => 0xFFFF00,
            Colors::Red => 0xFF0000,
            Colors::Green => 0x00FF00,
        }
    }

    pub fn to_square(&self) -> board::Square {
        match self {
            Colors::Blue => board::Square::Blue,
            Colors::Yellow => board::Square::Yellow,
            Colors::Red => board::Square::Red,
            Colors::Green => board::Square::Green,
        }
    }
}

pub struct PlacedPiece {
    pub piece: Pieces,
    pub shape: Shape,
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone)]
pub struct SpecialShape {
    pub width: u8,
    pub height: u8,
    pub map: Vec<SpecialShapePart>,
    pub rotation: Rotation,
    pub inner_corners: Vec<(u8, u8)>,
    pub outer_corners: Vec<(u8, u8)>,
}

#[derive(Debug, Clone, Copy)]
pub enum SpecialShapePart {
    /// Blank space
    Blank______,
    /// Occupied square with no possible corner connections
    Normal_____,
    /// A square of the piece that is in a corner
    InnerCorner,
    /// A square outside the piece touching the corner of an InnerCorner
    OuterCorner,
}

impl SpecialShape {
    pub fn new(width: u8, height: u8, occupied: &[SpecialShapePart]) -> Self {
        assert_eq!(occupied.len() as u8, width * height);

        let mut result = Self {
            width,
            height,
            map: occupied.to_vec(),
            rotation: Rotation::Normal,
            inner_corners: Vec::new(),
            outer_corners: Vec::new(),
        };

        result.build_lists();

        result
    }

    fn build_lists(&mut self) {
        self.inner_corners.clear();
        self.outer_corners.clear();

        for x in 0..self.width {
            for y in 0..self.height {
                let index = y * self.width + x;

                match self.map[index as usize] {
                    SpecialShapePart::InnerCorner => self.inner_corners.push((x, y)),
                    SpecialShapePart::OuterCorner => self.outer_corners.push((x, y)),
                    _ => {}
                }
            }
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

        // Increase rotation by 90° clockwise
        self.rotation = self.rotation.rotated_right();

        self.build_lists();
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

        // Toggle mirrored state
        self.rotation = self.rotation.mirrored_vertically();

        self.build_lists();
    }

    /// Same effect as horizontal mirroring but implemented the lazy way with one vertical mirror and 2 rotates
    pub fn mirror_horizontally_fake(&mut self) {
        self.mirror_vertically();
        self.rotate_right();
        self.rotate_right();
    }

    pub fn value_at_pos(&self, x: u8, y: u8) -> SpecialShapePart {
        debug_assert!(x <= self.width);
        debug_assert!(y <= self.height);

        self.map[self.map_index(x, y) as usize]
    }

    pub fn map_index(&self, x: u8, y: u8) -> u8 {
        let index = x + y * self.width;

        debug_assert!((index as usize) < self.map.len());

        index
    }

    pub fn new_3x3(occupied: [SpecialShapePart; 9]) -> Self {
        Self::new(3, 3, &occupied)
    }

    pub fn new_4x4(occupied: [SpecialShapePart; 16]) -> Self {
        Self::new(4, 4, &occupied)
    }

    pub fn new_5x5(occupied: [SpecialShapePart; 25]) -> Self {
        Self::new(5, 5, &occupied)
    }
}

#[rustfmt::skip]
pub fn calculate_special_piece_shape(piece: &Pieces) -> SpecialShape {
    use SpecialShapePart::{*};

    match piece {
        Single1 => SpecialShape::new_3x3([
            OuterCorner, Blank______, OuterCorner,
            Blank______, InnerCorner, Blank______,
            OuterCorner, Blank______, OuterCorner
        ]),
        Bar2 => SpecialShape::new(3, 4, &[
            OuterCorner, Blank______, OuterCorner,
            Blank______, InnerCorner, Blank______,
            Blank______, InnerCorner, Blank______,
            OuterCorner, Blank______, OuterCorner
        ]),
        Bar3 => SpecialShape::new(3, 5, &[
            OuterCorner, Blank______, OuterCorner,
            Blank______, InnerCorner, Blank______,
            Blank______, Normal_____, Blank______,
            Blank______, InnerCorner, Blank______,
            OuterCorner, Blank______, OuterCorner
        ]),
        Corner3 => SpecialShape::new_4x4([
            OuterCorner, Blank______, OuterCorner, Blank______,
            Blank______, InnerCorner, Blank______, OuterCorner,
            Blank______, InnerCorner, InnerCorner, Blank______,
            OuterCorner, Blank______, Blank______, OuterCorner
        ]),
        Bar4 => SpecialShape::new(3, 6, &[
            OuterCorner, Blank______, OuterCorner,
            Blank______, InnerCorner, Blank______,
            Blank______, Normal_____, Blank______,
            Blank______, Normal_____, Blank______,
            Blank______, InnerCorner, Blank______,
            OuterCorner, Blank______, OuterCorner
        ]),
        L4 => SpecialShape::new(4, 5, &[
            OuterCorner, Blank______, Blank______, OuterCorner,
            Blank______, InnerCorner, InnerCorner, Blank______,
            Blank______, Normal_____, Blank______, OuterCorner,
            Blank______, InnerCorner, Blank______, Blank______,
            OuterCorner, Blank______, OuterCorner, Blank______,
        ]),
        T4 => SpecialShape::new(5, 4, &[
            OuterCorner, Blank______, Blank______, Blank______, OuterCorner,
            Blank______, InnerCorner, Normal_____, InnerCorner, Blank______,
            OuterCorner, Blank______, InnerCorner, Blank______, OuterCorner,
            Blank______, OuterCorner, Blank______, OuterCorner, Blank______
        ]),
        Square4 => SpecialShape::new_4x4([
            OuterCorner, Blank______, Blank______, OuterCorner,
            Blank______, InnerCorner, InnerCorner, Blank______,
            Blank______, InnerCorner, InnerCorner, Blank______,
            OuterCorner, Blank______, Blank______, OuterCorner
        ]),
        Z4 => SpecialShape::new(5, 4, &[
            OuterCorner, Blank______, Blank______, OuterCorner, Blank______,
            Blank______, InnerCorner, InnerCorner, Blank______, OuterCorner,
            OuterCorner, Blank______, InnerCorner, InnerCorner, Blank______,
            Blank______, OuterCorner, Blank______, Blank______, OuterCorner
        ]),
        Bar5 => SpecialShape::new(3, 7, &[
            OuterCorner, Blank______, OuterCorner,
            Blank______, InnerCorner, Blank______,
            Blank______, Normal_____, Blank______,
            Blank______, Normal_____, Blank______,
            Blank______, Normal_____, Blank______,
            Blank______, InnerCorner, Blank______,
            OuterCorner, Blank______, OuterCorner
        ]),
        L5 => SpecialShape::new(4, 6, &[
            OuterCorner, Blank______, Blank______, OuterCorner,
            Blank______, InnerCorner, InnerCorner, Blank______,
            Blank______, Normal_____, Blank______, Blank______,
            Blank______, Normal_____, Blank______, Blank______,
            Blank______, InnerCorner, Blank______, Blank______,
            OuterCorner, Blank______, OuterCorner, Blank______
        ]),
        Z5 => SpecialShape::new(4, 6, &[
            Blank______, OuterCorner, Blank______, OuterCorner,
            Blank______, Blank______, InnerCorner, Blank______,
            OuterCorner, Blank______, Normal_____, Blank______,
            Blank______, InnerCorner, InnerCorner, Blank______,
            Blank______, InnerCorner, Blank______, OuterCorner,
            OuterCorner, Blank______, OuterCorner, Blank______
        ]),
        P5 => SpecialShape::new(4, 5, &[
            Blank______, OuterCorner, Blank______, OuterCorner,
            OuterCorner, Blank______, InnerCorner, Blank______,
            Blank______, InnerCorner, Normal_____, Blank______,
            Blank______, InnerCorner, InnerCorner, Blank______,
            OuterCorner, Blank______, Blank______, OuterCorner
        ]),
        Bracket5 => SpecialShape::new(4, 5, &[
            OuterCorner, Blank______, Blank______, OuterCorner,
            Blank______, InnerCorner, InnerCorner, Blank______,
            OuterCorner, Blank______, Normal_____, Blank______,
            Blank______, InnerCorner, InnerCorner, Blank______,
            OuterCorner, Blank______, Blank______, OuterCorner
        ]),
        WeirdBar5 => SpecialShape::new(4, 6, &[
            OuterCorner, Blank______, OuterCorner, Blank______,
            Blank______, InnerCorner, Blank______, Blank______,
            Blank______, Normal_____, Blank______, OuterCorner,
            Blank______, Normal_____, InnerCorner, Blank______,
            Blank______, InnerCorner, Blank______, OuterCorner,
            OuterCorner, Blank______, OuterCorner, Blank______,
        ]),
        T5 => SpecialShape::new_5x5([
            OuterCorner, Blank______, Blank______, Blank______, OuterCorner,
            Blank______, InnerCorner, Normal_____, InnerCorner, Blank______,
            OuterCorner, Blank______, Normal_____, Blank______, OuterCorner,
            Blank______, Blank______, InnerCorner, Blank______, Blank______,
            Blank______, OuterCorner, Blank______, OuterCorner, Blank______
        ]),
        Corner5 => SpecialShape::new_5x5([
            OuterCorner, Blank______, OuterCorner, Blank______, Blank______,
            Blank______, InnerCorner, Blank______, Blank______, Blank______,
            Blank______, Normal_____, Blank______, Blank______, OuterCorner,
            Blank______, InnerCorner, Normal_____, InnerCorner, Blank______,
            OuterCorner, Blank______, Blank______, Blank______, OuterCorner
        ]),
        W5 => SpecialShape::new_5x5([
            OuterCorner, Blank______, Blank______, OuterCorner, Blank______,
            Blank______, InnerCorner, InnerCorner, Blank______, OuterCorner,
            OuterCorner, Blank______, InnerCorner, InnerCorner, Blank______,
            Blank______, OuterCorner, Blank______, InnerCorner, Blank______,
            Blank______, Blank______, OuterCorner, Blank______, OuterCorner
        ]),
        LongS5 => SpecialShape::new_5x5([
            OuterCorner, Blank______, Blank______, Blank______, Blank______,
            Blank______, InnerCorner, Blank______, Blank______, OuterCorner,
            Blank______, InnerCorner, Normal_____, InnerCorner, Blank______,
            OuterCorner, Blank______, Blank______, InnerCorner, Blank______,
            Blank______, Blank______, Blank______, Blank______, OuterCorner
        ]),
        Thingy5 => SpecialShape::new_5x5([
            OuterCorner, Blank______, OuterCorner, Blank______, Blank______,
            Blank______, InnerCorner, Blank______, Blank______, OuterCorner,
            Blank______, InnerCorner, Normal_____, InnerCorner, Blank______,
            OuterCorner, Blank______, InnerCorner, Blank______, OuterCorner,
            Blank______, OuterCorner, Blank______, OuterCorner, Blank______
        ]),
        Plus5 => SpecialShape::new_5x5([
            Blank______, OuterCorner, Blank______, OuterCorner, Blank______,
            OuterCorner, Blank______, InnerCorner, Blank______, OuterCorner,
            Blank______, InnerCorner, Normal_____, InnerCorner, Blank______,
            OuterCorner, Blank______, InnerCorner, Blank______, OuterCorner,
            Blank______, OuterCorner, Blank______, OuterCorner, Blank______
        ]),
    }
}
