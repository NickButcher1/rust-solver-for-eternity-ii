pub type Ori = u8;

#[cfg(feature = "backtracker-full")]
pub const BASE: Ori = 0;
#[cfg(feature = "backtracker-full")]
pub const ANTICLOCKWISE_90: Ori = 1;
#[cfg(feature = "backtracker-full")]
pub const HALF: Ori = 2;
#[cfg(feature = "backtracker-full")]
pub const CLOCKWISE_90: Ori = 3;

pub const ANY: Ori = 99;
