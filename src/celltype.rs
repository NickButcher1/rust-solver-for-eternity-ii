#[cfg(feature = "backtracker-full")]
use crate::ori::{ANTICLOCKWISE_90, BASE, CLOCKWISE_90, HALF};

pub type CellType = u8;

pub const MID: CellType = 0;

#[cfg(feature = "backtracker-full")]
pub const EDGE_TOP: CellType = 10 + HALF;
#[cfg(feature = "backtracker-full")]
pub const EDGE_RIGHT: CellType = 10 + ANTICLOCKWISE_90;
#[cfg(feature = "backtracker-full")]
pub const EDGE_BOTTOM: CellType = 10 + BASE;
#[cfg(feature = "backtracker-full")]
pub const EDGE_LEFT: CellType = 10 + CLOCKWISE_90;
#[cfg(feature = "backtracker-full")]
pub const CORNER_TOP_LEFT: CellType = 20 + CLOCKWISE_90;
#[cfg(feature = "backtracker-full")]
pub const CORNER_TOP_RIGHT: CellType = 20 + HALF;
#[cfg(feature = "backtracker-full")]
pub const CORNER_BOTTOM_LEFT: CellType = 20 + BASE;
#[cfg(feature = "backtracker-full")]
pub const CORNER_BOTTOM_RIGHT: CellType = 20 + ANTICLOCKWISE_90;

#[cfg(feature = "backtracker-mids")]
pub const MID_TOP_LEFT: CellType = 1;
#[cfg(feature = "backtracker-mids")]
pub const MID_TOP: CellType = 2;
#[cfg(feature = "backtracker-mids")]
pub const MID_LEFT: CellType = 3;
#[cfg(feature = "backtracker-mids")]
pub const MID_RIGHT: CellType = 4;
