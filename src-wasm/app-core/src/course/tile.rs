use bevy::prelude::*;
use enum_iterator::Sequence;

#[derive(Component)]
pub struct TilePreview;

pub struct TilePlacePreview(pub Option<(Entity, [i32; 2])>);

#[derive(Clone, Debug)]
pub struct Tile {
    pub entity: Entity,
    pub variant: TileVariant,
}

#[derive(Clone, Component, Debug, Eq, Hash, PartialEq, Sequence)]
pub enum TileVariant {
    Ground(GroundVariant),
    HardBlock,
    RotatingBlock,
    DonutBlock,
    CloudBlock,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Sequence)]
pub enum GroundVariant {
    TopLeft0,
    TopRight0,
    Left,
    Right,
    BottomLeft0,
    BottomRight0,
    TopBottom0,
    TopBottom1,
    TopBottom2,
    Top0,
    Top1,
    Top2,
    Full0,
    Full1,
    Full2,
    Bottom0,
    Bottom1,
    Bottom2,
    Left0,
    Left1,
    Left2,
    Right0,
    Right1,
    Right2,
    All,
    TopBottomLeft,
    TopBottom3,
    TopRightBottom,
    TopRightLeft,
    RightLeft,
    RightBottomLeft,
    RightBottomCornerTL,
    BottomLeftCornerTR,
    TopRightCornerBL,
    TopLeftCornerBR,
    BottomCornerTRTL,
    TopCornerBRBL,
    RightCornerTLBL,
    LeftCornerTRBR,
    CornerAll,
    LeftCornerTR,
    RightCornerTL,
    LeftCornerBR,
    RightCornerBL,
    TopCornerBL,
    TopCornerBR,
    BottomCornerTL,
    BottomCornerTR,
    CornerTRTLBL,
    CornerTRTLBR,
    CornerTLBRBL,
    CornerTRBRBL,
    CornerTRTL,
    CornerBRBL,
    CornerTLBL,
    CornerTRBR,
    CornerTRBL,
    CornerTLBR,
    TopLeft1,
    Top4,
    TopRight1,
    Left4,
    Full4,
    Right4,
    BottomLeft1,
    Bottom4,
    BottomRight1,
    CornerTL,
    CornerTR,
    CornerBL,
    CornerBR,
}

#[derive(Component)]
pub struct GroundSurroundingMatrix(pub [[bool; 3]; 3]);

#[derive(Debug, Default)]
pub struct SelectedTile(pub Option<TileVariant>);

impl TileVariant {
    pub fn get_sprite_sheet_index(&self) -> usize {
        match self {
            Self::Ground(variant) => variant.get_sprite_sheet_index(),
            Self::HardBlock => 6,
            Self::RotatingBlock => 1,
            Self::DonutBlock => 64,
            Self::CloudBlock => 102,
        }
    }
}

impl Default for GroundVariant {
    fn default() -> Self {
        Self::Top0
    }
}

impl GroundVariant {
    pub fn get_sprite_sheet_index(&self) -> usize {
        match self {
            Self::TopLeft0 => 184,
            Self::TopRight0 => 185,
            Self::Left => 186,
            Self::Right => 187,
            Self::BottomLeft0 => 188,
            Self::BottomRight0 => 189,
            Self::TopBottom0 => 190,
            Self::TopBottom1 => 191,
            Self::TopBottom2 => 192,
            Self::Top0 => 193,
            Self::Top1 => 194,
            Self::Top2 => 195,
            Self::Full0 => 196,
            Self::Full1 => 197,
            Self::Full2 => 198,
            Self::Bottom0 => 199,
            Self::Bottom1 => 200,
            Self::Bottom2 => 201,
            Self::Left0 => 202,
            Self::Left1 => 203,
            Self::Left2 => 204,
            Self::Right0 => 205,
            Self::Right1 => 206,
            Self::Right2 => 207,
            Self::All => 208,
            Self::TopBottomLeft => 209,
            Self::TopBottom3 => 210,
            Self::TopRightBottom => 211,
            Self::TopRightLeft => 212,
            Self::RightLeft => 213,
            Self::RightBottomLeft => 214,
            Self::RightBottomCornerTL => 215,
            Self::BottomLeftCornerTR => 216,
            Self::TopRightCornerBL => 217,
            Self::TopLeftCornerBR => 218,
            Self::BottomCornerTRTL => 219,
            Self::TopCornerBRBL => 220,
            Self::RightCornerTLBL => 221,
            Self::LeftCornerTRBR => 222,
            Self::CornerAll => 223,
            Self::LeftCornerTR => 224,
            Self::RightCornerTL => 225,
            Self::LeftCornerBR => 226,
            Self::RightCornerBL => 227,
            Self::TopCornerBL => 228,
            Self::TopCornerBR => 229,
            Self::BottomCornerTL => 230,
            Self::BottomCornerTR => 231,
            Self::CornerTRTLBL => 232,
            Self::CornerTRTLBR => 233,
            Self::CornerTLBRBL => 234,
            Self::CornerTRBRBL => 235,
            Self::CornerTRTL => 236,
            Self::CornerBRBL => 237,
            Self::CornerTLBL => 238,
            Self::CornerTRBR => 239,
            Self::CornerTRBL => 240,
            Self::CornerTLBR => 241,
            Self::TopLeft1 => 242,
            Self::Top4 => 243,
            Self::TopRight1 => 244,
            Self::Left4 => 245,
            Self::Full4 => 246,
            Self::Right4 => 247,
            Self::BottomLeft1 => 248,
            Self::Bottom4 => 249,
            Self::BottomRight1 => 250,
            Self::CornerTL => 251,
            Self::CornerTR => 252,
            Self::CornerBL => 253,
            Self::CornerBR => 254,
        }
    }

    pub fn from_surrounding_matrix(matrix: &[[bool; 3]; 3]) -> Self {
        match matrix {
            [[_, false, _], [false, _, true], [_, true, true]] => GroundVariant::TopLeft0,
            [[_, false, _], [true, _, false], [true, true, _]] => GroundVariant::TopRight0,
            [[_, true, true], [false, _, true], [_, true, true]] => GroundVariant::Left,
            [[true, true, _], [true, _, false], [true, true, _]] => GroundVariant::Right,
            [[_, true, true], [false, _, true], [_, false, _]] => GroundVariant::BottomLeft0,
            [[true, true, _], [true, _, false], [_, false, _]] => GroundVariant::BottomRight0,
            [[_, false, _], [true, _, true], [_, false, _]] => GroundVariant::TopBottom0,
            [[_, false, _], [true, _, true], [true, true, true]] => GroundVariant::Top0,
            [[true, true, true], [true, _, true], [true, true, true]] => GroundVariant::Full0,
            [[true, true, true], [true, _, true], [_, false, _]] => GroundVariant::Bottom0,
            [[_, false, _], [false, _, false], [_, false, _]] => GroundVariant::All,
            [[_, false, _], [false, _, true], [_, false, _]] => GroundVariant::TopBottomLeft,
            [[_, false, _], [true, _, false], [_, false, _]] => GroundVariant::TopRightBottom,
            [[_, false, _], [false, _, false], [_, true, _]] => GroundVariant::TopRightLeft,
            [[_, true, _], [false, _, false], [_, true, _]] => GroundVariant::RightLeft,
            [[_, true, _], [false, _, false], [_, false, _]] => GroundVariant::RightBottomLeft,
            [[false, true, _], [true, _, false], [_, false, _]] => {
                GroundVariant::RightBottomCornerTL
            }
            [[_, true, false], [false, _, true], [_, false, _]] => {
                GroundVariant::BottomLeftCornerTR
            }
            [[_, false, _], [true, _, false], [false, true, _]] => GroundVariant::TopRightCornerBL,
            [[_, false, _], [false, _, true], [_, true, false]] => GroundVariant::TopLeftCornerBR,
            [[false, true, false], [true, _, true], [_, false, _]] => {
                GroundVariant::BottomCornerTRTL
            }
            [[_, false, _], [true, _, true], [false, true, false]] => GroundVariant::TopCornerBRBL,
            [[false, true, _], [true, _, false], [false, true, _]] => {
                GroundVariant::RightCornerTLBL
            }
            [[_, true, false], [false, _, true], [_, true, false]] => GroundVariant::LeftCornerTRBR,
            [[false, true, false], [true, _, true], [false, true, false]] => {
                GroundVariant::CornerAll
            }
            [[_, true, false], [false, _, true], [_, _, _]] => GroundVariant::LeftCornerTR,
            [[false, true, _], [true, _, false], [_, _, _]] => GroundVariant::RightCornerTL,
            [[_, _, _], [false, _, true], [_, true, false]] => GroundVariant::LeftCornerBR,
            [[_, _, _], [true, _, false], [false, true, _]] => GroundVariant::RightCornerBL,
            [[_, false, _], [true, _, _], [false, true, _]] => GroundVariant::TopCornerBL,
            [[_, false, _], [_, _, true], [_, true, false]] => GroundVariant::TopCornerBR,
            [[false, true, _], [true, _, _], [_, false, _]] => GroundVariant::BottomCornerTL,
            [[_, true, false], [_, _, true], [_, false, _]] => GroundVariant::BottomCornerTR,
            [[false, true, false], [true, _, true], [false, true, true]] => {
                GroundVariant::CornerTRTLBL
            }
            [[false, true, false], [true, _, true], [true, true, false]] => {
                GroundVariant::CornerTRTLBR
            }
            [[false, true, true], [true, _, true], [false, true, false]] => {
                GroundVariant::CornerTLBRBL
            }
            [[true, true, false], [true, _, true], [false, true, false]] => {
                GroundVariant::CornerTRBRBL
            }
            [[false, true, false], [true, _, true], [true, true, true]] => {
                GroundVariant::CornerTRTL
            }
            [[true, true, true], [true, _, true], [false, true, false]] => {
                GroundVariant::CornerBRBL
            }
            [[false, true, true], [true, _, true], [false, true, true]] => {
                GroundVariant::CornerTLBL
            }
            [[true, true, false], [true, _, true], [true, true, false]] => {
                GroundVariant::CornerTRBR
            }
            [[true, true, false], [true, _, true], [false, true, true]] => {
                GroundVariant::CornerTRBL
            }
            [[false, true, true], [true, _, true], [true, true, false]] => {
                GroundVariant::CornerTLBR
            }
            [[false, true, true], [true, _, true], [true, true, true]] => GroundVariant::CornerTL,
            [[true, true, false], [true, _, true], [true, true, true]] => GroundVariant::CornerTR,
            [[true, true, true], [true, _, true], [false, true, true]] => GroundVariant::CornerBL,
            [[true, true, true], [true, _, true], [true, true, false]] => GroundVariant::CornerBR,
        }
    }

    pub fn get_surrounding_matrix(&self) -> [[Option<bool>; 3]; 3] {
        match self {
            GroundVariant::TopLeft0 => [
                [None, Some(false), None],
                [Some(false), None, Some(true)],
                [None, Some(true), Some(true)],
            ],
            GroundVariant::TopRight0 => [
                [None, Some(false), None],
                [Some(true), None, Some(false)],
                [Some(true), Some(true), None],
            ],
            GroundVariant::Left => [
                [None, Some(true), Some(true)],
                [Some(false), None, Some(true)],
                [None, Some(true), Some(true)],
            ],
            GroundVariant::Right => [
                [Some(true), Some(true), None],
                [Some(true), None, Some(false)],
                [Some(true), Some(true), None],
            ],
            GroundVariant::BottomLeft0 => [
                [None, Some(true), Some(true)],
                [Some(false), None, Some(true)],
                [None, Some(false), None],
            ],
            GroundVariant::BottomRight0 => [
                [Some(true), Some(true), None],
                [Some(true), None, Some(false)],
                [None, Some(false), None],
            ],
            GroundVariant::TopBottom0 => [
                [None, Some(false), None],
                [Some(true), None, Some(true)],
                [None, Some(false), None],
            ],
            GroundVariant::TopBottom1 => [
                [None, Some(false), None],
                [Some(true), None, Some(true)],
                [None, Some(false), None],
            ],
            GroundVariant::TopBottom2 => [
                [None, Some(false), None],
                [Some(true), None, Some(true)],
                [None, Some(false), None],
            ],
            GroundVariant::Top0 => [
                [None, Some(false), None],
                [Some(true), None, Some(true)],
                [Some(true), Some(true), Some(true)],
            ],
            GroundVariant::Top1 => [
                [None, Some(false), None],
                [Some(true), None, Some(true)],
                [Some(true), Some(true), Some(true)],
            ],
            GroundVariant::Top2 => [
                [None, Some(false), None],
                [Some(true), None, Some(true)],
                [Some(true), Some(true), Some(true)],
            ],
            GroundVariant::Full0 => [
                [Some(true), Some(true), Some(true)],
                [Some(true), None, Some(true)],
                [Some(true), Some(true), Some(true)],
            ],
            GroundVariant::Full1 => [
                [Some(true), Some(true), Some(true)],
                [Some(true), None, Some(true)],
                [Some(true), Some(true), Some(true)],
            ],
            GroundVariant::Full2 => [
                [Some(true), Some(true), Some(true)],
                [Some(true), None, Some(true)],
                [Some(true), Some(true), Some(true)],
            ],
            GroundVariant::Bottom0 => [
                [Some(true), Some(true), Some(true)],
                [Some(true), None, Some(true)],
                [None, Some(false), None],
            ],
            GroundVariant::Bottom1 => [
                [Some(true), Some(true), Some(true)],
                [Some(true), None, Some(true)],
                [None, Some(false), None],
            ],
            GroundVariant::Bottom2 => [
                [Some(true), Some(true), Some(true)],
                [Some(true), None, Some(true)],
                [None, Some(false), None],
            ],
            GroundVariant::Left0 => [
                [None, Some(true), Some(true)],
                [Some(false), None, Some(true)],
                [None, Some(true), Some(true)],
            ],
            GroundVariant::Left1 => [
                [None, Some(true), Some(true)],
                [Some(false), None, Some(true)],
                [None, Some(true), Some(true)],
            ],
            GroundVariant::Left2 => [
                [None, Some(true), Some(true)],
                [Some(false), None, Some(true)],
                [None, Some(true), Some(true)],
            ],
            GroundVariant::Right0 => [
                [Some(true), Some(true), None],
                [Some(true), None, Some(false)],
                [Some(true), Some(true), None],
            ],
            GroundVariant::Right1 => [
                [Some(true), Some(true), None],
                [Some(true), None, Some(false)],
                [Some(true), Some(true), None],
            ],
            GroundVariant::Right2 => [
                [Some(true), Some(true), None],
                [Some(true), None, Some(false)],
                [Some(true), Some(true), None],
            ],
            GroundVariant::All => [
                [None, Some(false), None],
                [Some(false), None, Some(false)],
                [None, Some(false), None],
            ],
            GroundVariant::TopBottomLeft => [
                [None, Some(false), None],
                [Some(false), None, Some(true)],
                [None, Some(false), None],
            ],
            GroundVariant::TopBottom3 => [
                [None, Some(false), None],
                [Some(true), None, Some(true)],
                [None, Some(false), None],
            ],
            GroundVariant::TopRightBottom => [
                [None, Some(false), None],
                [Some(true), None, Some(false)],
                [None, Some(false), None],
            ],
            GroundVariant::TopRightLeft => [
                [None, Some(false), None],
                [Some(false), None, Some(false)],
                [None, Some(true), None],
            ],
            GroundVariant::RightLeft => [
                [None, Some(true), None],
                [Some(false), None, Some(false)],
                [None, Some(true), None],
            ],
            GroundVariant::RightBottomLeft => [
                [None, Some(true), None],
                [Some(false), None, Some(false)],
                [None, Some(false), None],
            ],
            GroundVariant::RightBottomCornerTL => [
                [Some(false), Some(true), None],
                [Some(true), None, Some(false)],
                [None, Some(false), None],
            ],
            GroundVariant::BottomLeftCornerTR => [
                [None, Some(true), Some(false)],
                [Some(false), None, Some(true)],
                [None, Some(false), None],
            ],
            GroundVariant::TopRightCornerBL => [
                [None, Some(false), None],
                [Some(true), None, Some(false)],
                [Some(false), Some(true), None],
            ],
            GroundVariant::TopLeftCornerBR => [
                [None, Some(false), None],
                [Some(false), None, Some(true)],
                [None, Some(true), Some(false)],
            ],
            GroundVariant::BottomCornerTRTL => [
                [Some(false), Some(true), Some(false)],
                [Some(true), None, Some(true)],
                [None, Some(false), None],
            ],
            GroundVariant::TopCornerBRBL => [
                [None, Some(false), None],
                [Some(true), None, Some(true)],
                [Some(false), Some(true), Some(false)],
            ],
            GroundVariant::RightCornerTLBL => [
                [Some(false), Some(true), None],
                [Some(true), None, Some(false)],
                [Some(false), Some(true), None],
            ],
            GroundVariant::LeftCornerTRBR => [
                [None, Some(true), Some(false)],
                [Some(false), None, Some(true)],
                [None, Some(true), Some(false)],
            ],
            GroundVariant::CornerAll => [
                [Some(false), Some(true), Some(false)],
                [Some(true), None, Some(true)],
                [Some(false), Some(true), Some(false)],
            ],
            GroundVariant::LeftCornerTR => [
                [None, Some(true), Some(false)],
                [Some(false), None, Some(true)],
                [None, None, None],
            ],
            GroundVariant::RightCornerTL => [
                [Some(false), Some(true), None],
                [Some(true), None, Some(false)],
                [None, None, None],
            ],
            GroundVariant::LeftCornerBR => [
                [None, None, None],
                [Some(false), None, Some(true)],
                [None, Some(true), Some(false)],
            ],
            GroundVariant::RightCornerBL => [
                [None, None, None],
                [Some(true), None, Some(false)],
                [Some(false), Some(true), None],
            ],
            GroundVariant::TopCornerBL => [
                [None, Some(false), None],
                [Some(true), None, None],
                [Some(false), Some(true), None],
            ],
            GroundVariant::TopCornerBR => [
                [None, Some(false), None],
                [None, None, Some(true)],
                [None, Some(true), Some(false)],
            ],
            GroundVariant::BottomCornerTL => [
                [Some(false), Some(true), None],
                [Some(true), None, None],
                [None, Some(false), None],
            ],
            GroundVariant::BottomCornerTR => [
                [None, Some(true), Some(false)],
                [None, None, Some(true)],
                [None, Some(false), None],
            ],
            GroundVariant::CornerTRTLBL => [
                [Some(false), Some(true), Some(false)],
                [Some(true), None, Some(true)],
                [Some(false), Some(true), Some(true)],
            ],
            GroundVariant::CornerTRTLBR => [
                [Some(false), Some(true), Some(false)],
                [Some(true), None, Some(true)],
                [Some(true), Some(true), Some(false)],
            ],
            GroundVariant::CornerTLBRBL => [
                [Some(false), Some(true), Some(true)],
                [Some(true), None, Some(true)],
                [Some(false), Some(true), Some(false)],
            ],
            GroundVariant::CornerTRBRBL => [
                [Some(true), Some(true), Some(false)],
                [Some(true), None, Some(true)],
                [Some(false), Some(true), Some(false)],
            ],
            GroundVariant::CornerTRTL => [
                [Some(false), Some(true), Some(false)],
                [Some(true), None, Some(true)],
                [Some(true), Some(true), Some(true)],
            ],
            GroundVariant::CornerBRBL => [
                [Some(true), Some(true), Some(true)],
                [Some(true), None, Some(true)],
                [Some(false), Some(true), Some(false)],
            ],
            GroundVariant::CornerTLBL => [
                [Some(false), Some(true), Some(true)],
                [Some(true), None, Some(true)],
                [Some(false), Some(true), Some(true)],
            ],
            GroundVariant::CornerTRBR => [
                [Some(true), Some(true), Some(false)],
                [Some(true), None, Some(true)],
                [Some(true), Some(true), Some(false)],
            ],
            GroundVariant::CornerTRBL => [
                [Some(true), Some(true), Some(false)],
                [Some(true), None, Some(true)],
                [Some(false), Some(true), Some(true)],
            ],
            GroundVariant::CornerTLBR => [
                [Some(false), Some(true), Some(true)],
                [Some(true), None, Some(true)],
                [Some(true), Some(true), Some(false)],
            ],
            GroundVariant::TopLeft1 => [
                [None, Some(false), None],
                [Some(false), None, Some(true)],
                [None, Some(true), Some(true)],
            ],
            GroundVariant::Top4 => [
                [None, Some(false), None],
                [Some(true), None, Some(true)],
                [Some(true), Some(true), Some(true)],
            ],
            GroundVariant::TopRight1 => [
                [None, Some(false), None],
                [Some(true), None, Some(false)],
                [Some(true), Some(true), None],
            ],
            GroundVariant::Left4 => [
                [None, Some(true), Some(true)],
                [Some(false), None, Some(true)],
                [None, Some(true), Some(true)],
            ],
            GroundVariant::Full4 => [
                [Some(true), Some(true), Some(true)],
                [Some(true), None, Some(true)],
                [Some(true), Some(true), Some(true)],
            ],
            GroundVariant::Right4 => [
                [Some(true), Some(true), None],
                [Some(true), None, Some(false)],
                [Some(true), Some(true), None],
            ],
            GroundVariant::BottomLeft1 => [
                [None, Some(true), Some(true)],
                [Some(false), None, Some(true)],
                [None, Some(false), None],
            ],
            GroundVariant::Bottom4 => [
                [Some(true), Some(true), Some(true)],
                [Some(true), None, Some(true)],
                [None, Some(false), None],
            ],
            GroundVariant::BottomRight1 => [
                [Some(true), Some(true), None],
                [Some(true), None, Some(false)],
                [None, Some(false), None],
            ],
            GroundVariant::CornerTL => [
                [Some(false), Some(true), Some(true)],
                [Some(true), None, Some(true)],
                [Some(true), Some(true), Some(true)],
            ],
            GroundVariant::CornerTR => [
                [Some(true), Some(true), Some(false)],
                [Some(true), None, Some(true)],
                [Some(true), Some(true), Some(true)],
            ],
            GroundVariant::CornerBL => [
                [Some(true), Some(true), Some(true)],
                [Some(true), None, Some(true)],
                [Some(false), Some(true), Some(true)],
            ],
            GroundVariant::CornerBR => [
                [Some(true), Some(true), Some(true)],
                [Some(true), None, Some(true)],
                [Some(true), Some(true), Some(false)],
            ],
        }
    }
}
