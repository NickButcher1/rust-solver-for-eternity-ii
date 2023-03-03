use crate::celltype::{CellType, MID, MID_LEFT, MID_TOP, MID_TOP_LEFT};
use crate::ori::{Ori, ANY};
use crate::Backtracker;

pub const NUM_CELLS: usize = 256;
pub const NUM_TILES: usize = 196;
pub const NUM_ROWS: usize = 16;
pub const NUM_COLS: usize = 16;

// pub const NUM_CORNERS: u32 = 4;
// pub const NUM_EDGES: u32 = 56;
// pub const NUM_MIDS: usize = 196;

// pub const NUM_COLOURS: u32 = 22;
// pub const NUM_COLOURS_INC_GREY: u32 = 23;
// pub const NUM_EDGE_COLOURS: u32 = 5;
// pub const NUM_MID_COLOURS: u32 = 17;

#[cfg(feature = "backtracker-mids")]
pub const ANY_COLOUR: u8 = 17;

const INVALID_CELL_IDX: u8 = 255;

#[derive(Debug)]
pub struct Cell {
    pub north_idx: u8,
    pub west_idx: u8,
    pub cell_type: CellType,
    pub ori: Ori,
}

pub const FILL_ORDER: [Cell; NUM_TILES] = [
    Cell {
        north_idx: INVALID_CELL_IDX,
        west_idx: INVALID_CELL_IDX,
        cell_type: MID_TOP_LEFT,
        ori: ANY,
    }, // idx 0
    Cell {
        north_idx: INVALID_CELL_IDX,
        west_idx: 0,
        cell_type: MID_TOP,
        ori: ANY,
    }, // idx 1
    Cell {
        north_idx: INVALID_CELL_IDX,
        west_idx: 1,
        cell_type: MID_TOP,
        ori: ANY,
    }, // idx 2
    Cell {
        north_idx: INVALID_CELL_IDX,
        west_idx: 2,
        cell_type: MID_TOP,
        ori: ANY,
    }, // idx 3
    Cell {
        north_idx: INVALID_CELL_IDX,
        west_idx: 3,
        cell_type: MID_TOP,
        ori: ANY,
    }, // idx 4
    Cell {
        north_idx: INVALID_CELL_IDX,
        west_idx: 4,
        cell_type: MID_TOP,
        ori: ANY,
    }, // idx 5
    Cell {
        north_idx: INVALID_CELL_IDX,
        west_idx: 5,
        cell_type: MID_TOP,
        ori: ANY,
    }, // idx 6
    Cell {
        north_idx: INVALID_CELL_IDX,
        west_idx: 6,
        cell_type: MID_TOP,
        ori: ANY,
    }, // idx 7
    Cell {
        north_idx: INVALID_CELL_IDX,
        west_idx: 7,
        cell_type: MID_TOP,
        ori: ANY,
    }, // idx 8
    Cell {
        north_idx: INVALID_CELL_IDX,
        west_idx: 8,
        cell_type: MID_TOP,
        ori: ANY,
    }, // idx 9
    Cell {
        north_idx: INVALID_CELL_IDX,
        west_idx: 9,
        cell_type: MID_TOP,
        ori: ANY,
    }, // idx 10
    Cell {
        north_idx: INVALID_CELL_IDX,
        west_idx: 10,
        cell_type: MID_TOP,
        ori: ANY,
    }, // idx 11
    Cell {
        north_idx: INVALID_CELL_IDX,
        west_idx: 11,
        cell_type: MID_TOP,
        ori: ANY,
    }, // idx 12
    Cell {
        north_idx: INVALID_CELL_IDX,
        west_idx: 12,
        cell_type: MID_TOP,
        ori: ANY,
    }, // idx 13
    Cell {
        north_idx: 0,
        west_idx: INVALID_CELL_IDX,
        cell_type: MID_LEFT,
        ori: ANY,
    }, // idx 14
    Cell {
        north_idx: 1,
        west_idx: 14,
        cell_type: MID,
        ori: ANY,
    }, // idx 15
    Cell {
        north_idx: 2,
        west_idx: 15,
        cell_type: MID,
        ori: ANY,
    }, // idx 16
    Cell {
        north_idx: 3,
        west_idx: 16,
        cell_type: MID,
        ori: ANY,
    }, // idx 17
    Cell {
        north_idx: 4,
        west_idx: 17,
        cell_type: MID,
        ori: ANY,
    }, // idx 18
    Cell {
        north_idx: 5,
        west_idx: 18,
        cell_type: MID,
        ori: ANY,
    }, // idx 19
    Cell {
        north_idx: 6,
        west_idx: 19,
        cell_type: MID,
        ori: ANY,
    }, // idx 20
    Cell {
        north_idx: 7,
        west_idx: 20,
        cell_type: MID,
        ori: ANY,
    }, // idx 21
    Cell {
        north_idx: 8,
        west_idx: 21,
        cell_type: MID,
        ori: ANY,
    }, // idx 22
    Cell {
        north_idx: 9,
        west_idx: 22,
        cell_type: MID,
        ori: ANY,
    }, // idx 23
    Cell {
        north_idx: 10,
        west_idx: 23,
        cell_type: MID,
        ori: ANY,
    }, // idx 24
    Cell {
        north_idx: 11,
        west_idx: 24,
        cell_type: MID,
        ori: ANY,
    }, // idx 25
    Cell {
        north_idx: 12,
        west_idx: 25,
        cell_type: MID,
        ori: ANY,
    }, // idx 26
    Cell {
        north_idx: 13,
        west_idx: 26,
        cell_type: MID,
        ori: ANY,
    }, // idx 27
    Cell {
        north_idx: 14,
        west_idx: INVALID_CELL_IDX,
        cell_type: MID_LEFT,
        ori: ANY,
    }, // idx 28
    Cell {
        north_idx: 15,
        west_idx: 28,
        cell_type: MID,
        ori: ANY,
    }, // idx 29
    Cell {
        north_idx: 16,
        west_idx: 29,
        cell_type: MID,
        ori: ANY,
    }, // idx 30
    Cell {
        north_idx: 17,
        west_idx: 30,
        cell_type: MID,
        ori: ANY,
    }, // idx 31
    Cell {
        north_idx: 18,
        west_idx: 31,
        cell_type: MID,
        ori: ANY,
    }, // idx 32
    Cell {
        north_idx: 19,
        west_idx: 32,
        cell_type: MID,
        ori: ANY,
    }, // idx 33
    Cell {
        north_idx: 20,
        west_idx: 33,
        cell_type: MID,
        ori: ANY,
    }, // idx 34
    Cell {
        north_idx: 21,
        west_idx: 34,
        cell_type: MID,
        ori: ANY,
    }, // idx 35
    Cell {
        north_idx: 22,
        west_idx: 35,
        cell_type: MID,
        ori: ANY,
    }, // idx 36
    Cell {
        north_idx: 23,
        west_idx: 36,
        cell_type: MID,
        ori: ANY,
    }, // idx 37
    Cell {
        north_idx: 24,
        west_idx: 37,
        cell_type: MID,
        ori: ANY,
    }, // idx 38
    Cell {
        north_idx: 25,
        west_idx: 38,
        cell_type: MID,
        ori: ANY,
    }, // idx 39
    Cell {
        north_idx: 26,
        west_idx: 39,
        cell_type: MID,
        ori: ANY,
    }, // idx 40
    Cell {
        north_idx: 27,
        west_idx: 40,
        cell_type: MID,
        ori: ANY,
    }, // idx 41
    Cell {
        north_idx: 28,
        west_idx: INVALID_CELL_IDX,
        cell_type: MID_LEFT,
        ori: ANY,
    }, // idx 42
    Cell {
        north_idx: 29,
        west_idx: 42,
        cell_type: MID,
        ori: ANY,
    }, // idx 43
    Cell {
        north_idx: 30,
        west_idx: 43,
        cell_type: MID,
        ori: ANY,
    }, // idx 44
    Cell {
        north_idx: 31,
        west_idx: 44,
        cell_type: MID,
        ori: ANY,
    }, // idx 45
    Cell {
        north_idx: 32,
        west_idx: 45,
        cell_type: MID,
        ori: ANY,
    }, // idx 46
    Cell {
        north_idx: 33,
        west_idx: 46,
        cell_type: MID,
        ori: ANY,
    }, // idx 47
    Cell {
        north_idx: 34,
        west_idx: 47,
        cell_type: MID,
        ori: ANY,
    }, // idx 48
    Cell {
        north_idx: 35,
        west_idx: 48,
        cell_type: MID,
        ori: ANY,
    }, // idx 49
    Cell {
        north_idx: 36,
        west_idx: 49,
        cell_type: MID,
        ori: ANY,
    }, // idx 50
    Cell {
        north_idx: 37,
        west_idx: 50,
        cell_type: MID,
        ori: ANY,
    }, // idx 51
    Cell {
        north_idx: 38,
        west_idx: 51,
        cell_type: MID,
        ori: ANY,
    }, // idx 52
    Cell {
        north_idx: 39,
        west_idx: 52,
        cell_type: MID,
        ori: ANY,
    }, // idx 53
    Cell {
        north_idx: 40,
        west_idx: 53,
        cell_type: MID,
        ori: ANY,
    }, // idx 54
    Cell {
        north_idx: 41,
        west_idx: 54,
        cell_type: MID,
        ori: ANY,
    }, // idx 55
    Cell {
        north_idx: 42,
        west_idx: INVALID_CELL_IDX,
        cell_type: MID_LEFT,
        ori: ANY,
    }, // idx 56
    Cell {
        north_idx: 43,
        west_idx: 56,
        cell_type: MID,
        ori: ANY,
    }, // idx 57
    Cell {
        north_idx: 44,
        west_idx: 57,
        cell_type: MID,
        ori: ANY,
    }, // idx 58
    Cell {
        north_idx: 45,
        west_idx: 58,
        cell_type: MID,
        ori: ANY,
    }, // idx 59
    Cell {
        north_idx: 46,
        west_idx: 59,
        cell_type: MID,
        ori: ANY,
    }, // idx 60
    Cell {
        north_idx: 47,
        west_idx: 60,
        cell_type: MID,
        ori: ANY,
    }, // idx 61
    Cell {
        north_idx: 48,
        west_idx: 61,
        cell_type: MID,
        ori: ANY,
    }, // idx 62
    Cell {
        north_idx: 49,
        west_idx: 62,
        cell_type: MID,
        ori: ANY,
    }, // idx 63
    Cell {
        north_idx: 50,
        west_idx: 63,
        cell_type: MID,
        ori: ANY,
    }, // idx 64
    Cell {
        north_idx: 51,
        west_idx: 64,
        cell_type: MID,
        ori: ANY,
    }, // idx 65
    Cell {
        north_idx: 52,
        west_idx: 65,
        cell_type: MID,
        ori: ANY,
    }, // idx 66
    Cell {
        north_idx: 53,
        west_idx: 66,
        cell_type: MID,
        ori: ANY,
    }, // idx 67
    Cell {
        north_idx: 54,
        west_idx: 67,
        cell_type: MID,
        ori: ANY,
    }, // idx 68
    Cell {
        north_idx: 55,
        west_idx: 68,
        cell_type: MID,
        ori: ANY,
    }, // idx 69
    Cell {
        north_idx: 56,
        west_idx: INVALID_CELL_IDX,
        cell_type: MID_LEFT,
        ori: ANY,
    }, // idx 70
    Cell {
        north_idx: 57,
        west_idx: 70,
        cell_type: MID,
        ori: ANY,
    }, // idx 71
    Cell {
        north_idx: 58,
        west_idx: 71,
        cell_type: MID,
        ori: ANY,
    }, // idx 72
    Cell {
        north_idx: 59,
        west_idx: 72,
        cell_type: MID,
        ori: ANY,
    }, // idx 73
    Cell {
        north_idx: 60,
        west_idx: 73,
        cell_type: MID,
        ori: ANY,
    }, // idx 74
    Cell {
        north_idx: 61,
        west_idx: 74,
        cell_type: MID,
        ori: ANY,
    }, // idx 75
    Cell {
        north_idx: 62,
        west_idx: 75,
        cell_type: MID,
        ori: ANY,
    }, // idx 76
    Cell {
        north_idx: 63,
        west_idx: 76,
        cell_type: MID,
        ori: ANY,
    }, // idx 77
    Cell {
        north_idx: 64,
        west_idx: 77,
        cell_type: MID,
        ori: ANY,
    }, // idx 78
    Cell {
        north_idx: 65,
        west_idx: 78,
        cell_type: MID,
        ori: ANY,
    }, // idx 79
    Cell {
        north_idx: 66,
        west_idx: 79,
        cell_type: MID,
        ori: ANY,
    }, // idx 80
    Cell {
        north_idx: 67,
        west_idx: 80,
        cell_type: MID,
        ori: ANY,
    }, // idx 81
    Cell {
        north_idx: 68,
        west_idx: 81,
        cell_type: MID,
        ori: ANY,
    }, // idx 82
    Cell {
        north_idx: 69,
        west_idx: 82,
        cell_type: MID,
        ori: ANY,
    }, // idx 83
    Cell {
        north_idx: 70,
        west_idx: INVALID_CELL_IDX,
        cell_type: MID_LEFT,
        ori: ANY,
    }, // idx 84
    Cell {
        north_idx: 71,
        west_idx: 84,
        cell_type: MID,
        ori: ANY,
    }, // idx 85
    Cell {
        north_idx: 72,
        west_idx: 85,
        cell_type: MID,
        ori: ANY,
    }, // idx 86
    Cell {
        north_idx: 73,
        west_idx: 86,
        cell_type: MID,
        ori: ANY,
    }, // idx 87
    Cell {
        north_idx: 74,
        west_idx: 87,
        cell_type: MID,
        ori: ANY,
    }, // idx 88
    Cell {
        north_idx: 75,
        west_idx: 88,
        cell_type: MID,
        ori: ANY,
    }, // idx 89
    Cell {
        north_idx: 76,
        west_idx: 89,
        cell_type: MID,
        ori: ANY,
    }, // idx 90
    Cell {
        north_idx: 77,
        west_idx: 90,
        cell_type: MID,
        ori: ANY,
    }, // idx 91
    Cell {
        north_idx: 78,
        west_idx: 91,
        cell_type: MID,
        ori: ANY,
    }, // idx 92
    Cell {
        north_idx: 79,
        west_idx: 92,
        cell_type: MID,
        ori: ANY,
    }, // idx 93
    Cell {
        north_idx: 80,
        west_idx: 93,
        cell_type: MID,
        ori: ANY,
    }, // idx 94
    Cell {
        north_idx: 81,
        west_idx: 94,
        cell_type: MID,
        ori: ANY,
    }, // idx 95
    Cell {
        north_idx: 82,
        west_idx: 95,
        cell_type: MID,
        ori: ANY,
    }, // idx 96
    Cell {
        north_idx: 83,
        west_idx: 96,
        cell_type: MID,
        ori: ANY,
    }, // idx 97
    Cell {
        north_idx: 84,
        west_idx: INVALID_CELL_IDX,
        cell_type: MID_LEFT,
        ori: ANY,
    }, // idx 98
    Cell {
        north_idx: 85,
        west_idx: 98,
        cell_type: MID,
        ori: ANY,
    }, // idx 99
    Cell {
        north_idx: 86,
        west_idx: 99,
        cell_type: MID,
        ori: ANY,
    }, // idx 100
    Cell {
        north_idx: 87,
        west_idx: 100,
        cell_type: MID,
        ori: ANY,
    }, // idx 101
    Cell {
        north_idx: 88,
        west_idx: 101,
        cell_type: MID,
        ori: ANY,
    }, // idx 102
    Cell {
        north_idx: 89,
        west_idx: 102,
        cell_type: MID,
        ori: ANY,
    }, // idx 103
    Cell {
        north_idx: 90,
        west_idx: 103,
        cell_type: MID,
        ori: ANY,
    }, // idx 104
    Cell {
        north_idx: 91,
        west_idx: 104,
        cell_type: MID,
        ori: ANY,
    }, // idx 105
    Cell {
        north_idx: 92,
        west_idx: 105,
        cell_type: MID,
        ori: ANY,
    }, // idx 106
    Cell {
        north_idx: 93,
        west_idx: 106,
        cell_type: MID,
        ori: ANY,
    }, // idx 107
    Cell {
        north_idx: 94,
        west_idx: 107,
        cell_type: MID,
        ori: ANY,
    }, // idx 108
    Cell {
        north_idx: 95,
        west_idx: 108,
        cell_type: MID,
        ori: ANY,
    }, // idx 109
    Cell {
        north_idx: 96,
        west_idx: 109,
        cell_type: MID,
        ori: ANY,
    }, // idx 110
    Cell {
        north_idx: 97,
        west_idx: 110,
        cell_type: MID,
        ori: ANY,
    }, // idx 111
    Cell {
        north_idx: 98,
        west_idx: INVALID_CELL_IDX,
        cell_type: MID_LEFT,
        ori: ANY,
    }, // idx 112
    Cell {
        north_idx: 99,
        west_idx: 112,
        cell_type: MID,
        ori: ANY,
    }, // idx 113
    Cell {
        north_idx: 100,
        west_idx: 113,
        cell_type: MID,
        ori: ANY,
    }, // idx 114
    Cell {
        north_idx: 101,
        west_idx: 114,
        cell_type: MID,
        ori: ANY,
    }, // idx 115
    Cell {
        north_idx: 102,
        west_idx: 115,
        cell_type: MID,
        ori: ANY,
    }, // idx 116
    Cell {
        north_idx: 103,
        west_idx: 116,
        cell_type: MID,
        ori: ANY,
    }, // idx 117
    Cell {
        north_idx: 104,
        west_idx: 117,
        cell_type: MID,
        ori: ANY,
    }, // idx 118
    Cell {
        north_idx: 105,
        west_idx: 118,
        cell_type: MID,
        ori: ANY,
    }, // idx 119
    Cell {
        north_idx: 106,
        west_idx: 119,
        cell_type: MID,
        ori: ANY,
    }, // idx 120
    Cell {
        north_idx: 107,
        west_idx: 120,
        cell_type: MID,
        ori: ANY,
    }, // idx 121
    Cell {
        north_idx: 108,
        west_idx: 121,
        cell_type: MID,
        ori: ANY,
    }, // idx 122
    Cell {
        north_idx: 109,
        west_idx: 122,
        cell_type: MID,
        ori: ANY,
    }, // idx 123
    Cell {
        north_idx: 110,
        west_idx: 123,
        cell_type: MID,
        ori: ANY,
    }, // idx 124
    Cell {
        north_idx: 111,
        west_idx: 124,
        cell_type: MID,
        ori: ANY,
    }, // idx 125
    Cell {
        north_idx: 112,
        west_idx: INVALID_CELL_IDX,
        cell_type: MID_LEFT,
        ori: ANY,
    }, // idx 126
    Cell {
        north_idx: 113,
        west_idx: 126,
        cell_type: MID,
        ori: ANY,
    }, // idx 127
    Cell {
        north_idx: 114,
        west_idx: 127,
        cell_type: MID,
        ori: ANY,
    }, // idx 128
    Cell {
        north_idx: 115,
        west_idx: 128,
        cell_type: MID,
        ori: ANY,
    }, // idx 129
    Cell {
        north_idx: 116,
        west_idx: 129,
        cell_type: MID,
        ori: ANY,
    }, // idx 130
    Cell {
        north_idx: 117,
        west_idx: 130,
        cell_type: MID,
        ori: ANY,
    }, // idx 131
    Cell {
        north_idx: 118,
        west_idx: 131,
        cell_type: MID,
        ori: ANY,
    }, // idx 132
    Cell {
        north_idx: 119,
        west_idx: 132,
        cell_type: MID,
        ori: ANY,
    }, // idx 133
    Cell {
        north_idx: 120,
        west_idx: 133,
        cell_type: MID,
        ori: ANY,
    }, // idx 134
    Cell {
        north_idx: 121,
        west_idx: 134,
        cell_type: MID,
        ori: ANY,
    }, // idx 135
    Cell {
        north_idx: 122,
        west_idx: 135,
        cell_type: MID,
        ori: ANY,
    }, // idx 136
    Cell {
        north_idx: 123,
        west_idx: 136,
        cell_type: MID,
        ori: ANY,
    }, // idx 137
    Cell {
        north_idx: 124,
        west_idx: 137,
        cell_type: MID,
        ori: ANY,
    }, // idx 138
    Cell {
        north_idx: 125,
        west_idx: 138,
        cell_type: MID,
        ori: ANY,
    }, // idx 139
    Cell {
        north_idx: 126,
        west_idx: INVALID_CELL_IDX,
        cell_type: MID_LEFT,
        ori: ANY,
    }, // idx 140
    Cell {
        north_idx: 127,
        west_idx: 140,
        cell_type: MID,
        ori: ANY,
    }, // idx 141
    Cell {
        north_idx: 128,
        west_idx: 141,
        cell_type: MID,
        ori: ANY,
    }, // idx 142
    Cell {
        north_idx: 129,
        west_idx: 142,
        cell_type: MID,
        ori: ANY,
    }, // idx 143
    Cell {
        north_idx: 130,
        west_idx: 143,
        cell_type: MID,
        ori: ANY,
    }, // idx 144
    Cell {
        north_idx: 131,
        west_idx: 144,
        cell_type: MID,
        ori: ANY,
    }, // idx 145
    Cell {
        north_idx: 132,
        west_idx: 145,
        cell_type: MID,
        ori: ANY,
    }, // idx 146
    Cell {
        north_idx: 133,
        west_idx: 146,
        cell_type: MID,
        ori: ANY,
    }, // idx 147
    Cell {
        north_idx: 134,
        west_idx: 147,
        cell_type: MID,
        ori: ANY,
    }, // idx 148
    Cell {
        north_idx: 135,
        west_idx: 148,
        cell_type: MID,
        ori: ANY,
    }, // idx 149
    Cell {
        north_idx: 136,
        west_idx: 149,
        cell_type: MID,
        ori: ANY,
    }, // idx 150
    Cell {
        north_idx: 137,
        west_idx: 150,
        cell_type: MID,
        ori: ANY,
    }, // idx 151
    Cell {
        north_idx: 138,
        west_idx: 151,
        cell_type: MID,
        ori: ANY,
    }, // idx 152
    Cell {
        north_idx: 139,
        west_idx: 152,
        cell_type: MID,
        ori: ANY,
    }, // idx 153
    Cell {
        north_idx: 140,
        west_idx: INVALID_CELL_IDX,
        cell_type: MID_LEFT,
        ori: ANY,
    }, // idx 154
    Cell {
        north_idx: 141,
        west_idx: 154,
        cell_type: MID,
        ori: ANY,
    }, // idx 155
    Cell {
        north_idx: 142,
        west_idx: 155,
        cell_type: MID,
        ori: ANY,
    }, // idx 156
    Cell {
        north_idx: 143,
        west_idx: 156,
        cell_type: MID,
        ori: ANY,
    }, // idx 157
    Cell {
        north_idx: 144,
        west_idx: 157,
        cell_type: MID,
        ori: ANY,
    }, // idx 158
    Cell {
        north_idx: 145,
        west_idx: 158,
        cell_type: MID,
        ori: ANY,
    }, // idx 159
    Cell {
        north_idx: 146,
        west_idx: 159,
        cell_type: MID,
        ori: ANY,
    }, // idx 160
    Cell {
        north_idx: 147,
        west_idx: 160,
        cell_type: MID,
        ori: ANY,
    }, // idx 161
    Cell {
        north_idx: 148,
        west_idx: 161,
        cell_type: MID,
        ori: ANY,
    }, // idx 162
    Cell {
        north_idx: 149,
        west_idx: 162,
        cell_type: MID,
        ori: ANY,
    }, // idx 163
    Cell {
        north_idx: 150,
        west_idx: 163,
        cell_type: MID,
        ori: ANY,
    }, // idx 164
    Cell {
        north_idx: 151,
        west_idx: 164,
        cell_type: MID,
        ori: ANY,
    }, // idx 165
    Cell {
        north_idx: 152,
        west_idx: 165,
        cell_type: MID,
        ori: ANY,
    }, // idx 166
    Cell {
        north_idx: 153,
        west_idx: 166,
        cell_type: MID,
        ori: ANY,
    }, // idx 167
    Cell {
        north_idx: 154,
        west_idx: INVALID_CELL_IDX,
        cell_type: MID_LEFT,
        ori: ANY,
    }, // idx 168
    Cell {
        north_idx: 155,
        west_idx: 168,
        cell_type: MID,
        ori: ANY,
    }, // idx 169
    Cell {
        north_idx: 156,
        west_idx: 169,
        cell_type: MID,
        ori: ANY,
    }, // idx 170
    Cell {
        north_idx: 157,
        west_idx: 170,
        cell_type: MID,
        ori: ANY,
    }, // idx 171
    Cell {
        north_idx: 158,
        west_idx: 171,
        cell_type: MID,
        ori: ANY,
    }, // idx 172
    Cell {
        north_idx: 159,
        west_idx: 172,
        cell_type: MID,
        ori: ANY,
    }, // idx 173
    Cell {
        north_idx: 160,
        west_idx: 173,
        cell_type: MID,
        ori: ANY,
    }, // idx 174
    Cell {
        north_idx: 161,
        west_idx: 174,
        cell_type: MID,
        ori: ANY,
    }, // idx 175
    Cell {
        north_idx: 162,
        west_idx: 175,
        cell_type: MID,
        ori: ANY,
    }, // idx 176
    Cell {
        north_idx: 163,
        west_idx: 176,
        cell_type: MID,
        ori: ANY,
    }, // idx 177
    Cell {
        north_idx: 164,
        west_idx: 177,
        cell_type: MID,
        ori: ANY,
    }, // idx 178
    Cell {
        north_idx: 165,
        west_idx: 178,
        cell_type: MID,
        ori: ANY,
    }, // idx 179
    Cell {
        north_idx: 166,
        west_idx: 179,
        cell_type: MID,
        ori: ANY,
    }, // idx 180
    Cell {
        north_idx: 167,
        west_idx: 180,
        cell_type: MID,
        ori: ANY,
    }, // idx 181
    Cell {
        north_idx: 168,
        west_idx: INVALID_CELL_IDX,
        cell_type: MID_LEFT,
        ori: ANY,
    }, // idx 182
    Cell {
        north_idx: 169,
        west_idx: 182,
        cell_type: MID,
        ori: ANY,
    }, // idx 183
    Cell {
        north_idx: 170,
        west_idx: 183,
        cell_type: MID,
        ori: ANY,
    }, // idx 184
    Cell {
        north_idx: 171,
        west_idx: 184,
        cell_type: MID,
        ori: ANY,
    }, // idx 185
    Cell {
        north_idx: 172,
        west_idx: 185,
        cell_type: MID,
        ori: ANY,
    }, // idx 186
    Cell {
        north_idx: 173,
        west_idx: 186,
        cell_type: MID,
        ori: ANY,
    }, // idx 187
    Cell {
        north_idx: 174,
        west_idx: 187,
        cell_type: MID,
        ori: ANY,
    }, // idx 188
    Cell {
        north_idx: 175,
        west_idx: 188,
        cell_type: MID,
        ori: ANY,
    }, // idx 189
    Cell {
        north_idx: 176,
        west_idx: 189,
        cell_type: MID,
        ori: ANY,
    }, // idx 190
    Cell {
        north_idx: 177,
        west_idx: 190,
        cell_type: MID,
        ori: ANY,
    }, // idx 191
    Cell {
        north_idx: 178,
        west_idx: 191,
        cell_type: MID,
        ori: ANY,
    }, // idx 192
    Cell {
        north_idx: 179,
        west_idx: 192,
        cell_type: MID,
        ori: ANY,
    }, // idx 193
    Cell {
        north_idx: 180,
        west_idx: 193,
        cell_type: MID,
        ori: ANY,
    }, // idx 194
    Cell {
        north_idx: 181,
        west_idx: 194,
        cell_type: MID,
        ori: ANY,
    }, // idx 195
];

pub const DISPLAY_TO_FILL_ORDER: [i16; NUM_CELLS] = [
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8,
    9, 10, 11, 12, 13, -1, -1, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, -1, -1, 28,
    29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, -1, -1, 42, 43, 44, 45, 46, 47, 48, 49, 50,
    51, 52, 53, 54, 55, -1, -1, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, -1, -1, 70,
    71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, -1, -1, 84, 85, 86, 87, 88, 89, 90, 91, 92,
    93, 94, 95, 96, 97, -1, -1, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111,
    -1, -1, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, -1, -1, 126, 127,
    128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, -1, -1, 140, 141, 142, 143, 144,
    145, 146, 147, 148, 149, 150, 151, 152, 153, -1, -1, 154, 155, 156, 157, 158, 159, 160, 161,
    162, 163, 164, 165, 166, 167, -1, -1, 168, 169, 170, 171, 172, 173, 174, 175, 176, 177, 178,
    179, 180, 181, -1, -1, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192, 193, 194, 195,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
];

pub const TILES: [u8; 784] = [
    4, 13, 1, 1, // id: 60, tile_type: MID
    10, 5, 1, 1, // id: 61, tile_type: MID
    7, 2, 1, 7, // id: 62, tile_type: MID
    1, 6, 1, 13, // id: 63, tile_type: MID
    13, 17, 1, 13, // id: 64, tile_type: MID
    10, 10, 1, 13, // id: 65, tile_type: MID
    8, 7, 1, 13, // id: 66, tile_type: MID
    15, 4, 1, 13, // id: 67, tile_type: MID
    17, 6, 1, 13, // id: 68, tile_type: MID
    2, 5, 1, 2, // id: 69, tile_type: MID
    5, 9, 1, 2, // id: 70, tile_type: MID
    10, 13, 1, 8, // id: 71, tile_type: MID
    11, 3, 1, 8, // id: 72, tile_type: MID
    15, 11, 1, 8, // id: 73, tile_type: MID
    6, 2, 1, 8, // id: 74, tile_type: MID
    10, 11, 1, 14, // id: 75, tile_type: MID
    14, 11, 1, 14, // id: 76, tile_type: MID
    2, 12, 1, 5, // id: 77, tile_type: MID
    15, 2, 1, 5, // id: 78, tile_type: MID
    12, 16, 1, 5, // id: 79, tile_type: MID
    14, 13, 1, 11, // id: 80, tile_type: MID
    13, 13, 1, 3, // id: 81, tile_type: MID
    8, 3, 1, 3, // id: 82, tile_type: MID
    13, 14, 1, 9, // id: 83, tile_type: MID
    4, 10, 1, 9, // id: 84, tile_type: MID
    6, 9, 1, 9, // id: 85, tile_type: MID
    12, 15, 1, 9, // id: 86, tile_type: MID
    1, 16, 1, 15, // id: 87, tile_type: MID
    4, 17, 1, 15, // id: 88, tile_type: MID
    3, 12, 1, 15, // id: 89, tile_type: MID
    8, 9, 1, 6, // id: 90, tile_type: MID
    14, 11, 1, 6, // id: 91, tile_type: MID
    14, 3, 1, 6, // id: 92, tile_type: MID
    3, 16, 1, 6, // id: 93, tile_type: MID
    9, 10, 1, 6, // id: 94, tile_type: MID
    16, 2, 1, 16, // id: 95, tile_type: MID
    3, 14, 1, 17, // id: 96, tile_type: MID
    15, 6, 1, 17, // id: 97, tile_type: MID
    16, 4, 1, 17, // id: 98, tile_type: MID
    17, 16, 1, 17, // id: 99, tile_type: MID
    9, 11, 7, 7, // id: 100, tile_type: MID
    9, 12, 7, 7, // id: 101, tile_type: MID
    12, 14, 7, 7, // id: 102, tile_type: MID
    17, 4, 7, 7, // id: 103, tile_type: MID
    3, 11, 7, 13, // id: 104, tile_type: MID
    2, 6, 7, 4, // id: 105, tile_type: MID
    14, 6, 7, 4, // id: 106, tile_type: MID
    3, 11, 7, 4, // id: 107, tile_type: MID
    12, 8, 7, 4, // id: 108, tile_type: MID
    11, 9, 7, 10, // id: 109, tile_type: MID
    9, 11, 7, 10, // id: 110, tile_type: MID
    15, 14, 7, 2, // id: 111, tile_type: MID
    15, 12, 7, 2, // id: 112, tile_type: MID
    10, 3, 7, 8, // id: 113, tile_type: MID
    5, 9, 7, 8, // id: 114, tile_type: MID
    9, 8, 7, 8, // id: 115, tile_type: MID
    17, 12, 7, 8, // id: 116, tile_type: MID
    2, 16, 7, 14, // id: 117, tile_type: MID
    12, 9, 7, 5, // id: 118, tile_type: MID
    6, 17, 7, 11, // id: 119, tile_type: MID
    10, 17, 7, 3, // id: 120, tile_type: MID
    4, 12, 7, 15, // id: 121, tile_type: MID
    10, 14, 7, 15, // id: 122, tile_type: MID
    15, 11, 7, 15, // id: 123, tile_type: MID
    9, 17, 7, 6, // id: 124, tile_type: MID
    16, 11, 7, 6, // id: 125, tile_type: MID
    10, 4, 7, 12, // id: 126, tile_type: MID
    14, 16, 7, 12, // id: 127, tile_type: MID
    3, 2, 7, 12, // id: 128, tile_type: MID
    15, 6, 7, 12, // id: 129, tile_type: MID
    4, 15, 7, 16, // id: 130, tile_type: MID
    9, 10, 7, 16, // id: 131, tile_type: MID
    10, 12, 7, 17, // id: 132, tile_type: MID
    8, 3, 7, 17, // id: 133, tile_type: MID
    3, 2, 7, 17, // id: 134, tile_type: MID
    12, 15, 7, 17, // id: 135, tile_type: MID
    15, 5, 13, 13, // id: 136, tile_type: MID
    4, 2, 13, 4, // id: 137, tile_type: MID
    4, 8, 13, 4, // id: 138, tile_type: MID
    4, 9, 13, 4, // id: 139, tile_type: MID
    14, 16, 13, 4, // id: 140, tile_type: MID
    11, 4, 13, 4, // id: 141, tile_type: MID
    12, 9, 13, 4, // id: 142, tile_type: MID
    16, 16, 13, 4, // id: 143, tile_type: MID
    2, 3, 13, 10, // id: 144, tile_type: MID
    13, 9, 13, 2, // id: 145, tile_type: MID
    13, 17, 13, 2, // id: 146, tile_type: MID
    2, 10, 13, 2, // id: 147, tile_type: MID
    11, 9, 13, 2, // id: 148, tile_type: MID
    4, 8, 13, 14, // id: 149, tile_type: MID
    3, 17, 13, 14, // id: 150, tile_type: MID
    8, 8, 13, 5, // id: 151, tile_type: MID
    8, 14, 13, 5, // id: 152, tile_type: MID
    17, 12, 13, 5, // id: 153, tile_type: MID
    5, 5, 13, 3, // id: 154, tile_type: MID
    5, 9, 13, 3, // id: 155, tile_type: MID
    17, 5, 13, 3, // id: 156, tile_type: MID
    5, 12, 13, 15, // id: 157, tile_type: MID
    6, 4, 13, 15, // id: 158, tile_type: MID
    16, 16, 13, 6, // id: 159, tile_type: MID
    4, 15, 13, 12, // id: 160, tile_type: MID
    10, 15, 13, 12, // id: 161, tile_type: MID
    11, 8, 13, 17, // id: 162, tile_type: MID
    3, 12, 13, 17, // id: 163, tile_type: MID
    2, 3, 4, 10, // id: 164, tile_type: MID
    3, 6, 4, 10, // id: 165, tile_type: MID
    2, 2, 4, 8, // id: 166, tile_type: MID
    8, 11, 4, 14, // id: 167, tile_type: MID
    14, 16, 4, 14, // id: 168, tile_type: MID
    3, 6, 4, 5, // id: 169, tile_type: MID
    15, 12, 4, 5, // id: 170, tile_type: MID
    16, 8, 4, 5, // id: 171, tile_type: MID
    11, 11, 4, 11, // id: 172, tile_type: MID
    9, 15, 4, 11, // id: 173, tile_type: MID
    5, 16, 4, 3, // id: 174, tile_type: MID
    5, 14, 4, 9, // id: 175, tile_type: MID
    10, 3, 4, 15, // id: 176, tile_type: MID
    9, 12, 4, 6, // id: 177, tile_type: MID
    6, 11, 4, 6, // id: 178, tile_type: MID
    5, 12, 4, 12, // id: 179, tile_type: MID
    8, 12, 4, 16, // id: 180, tile_type: MID
    16, 5, 8, 4, // id: 181, tile_type: MID
    10, 14, 6, 10, // id: 182, tile_type: MID
    17, 5, 10, 2, // id: 183, tile_type: MID
    14, 9, 10, 8, // id: 184, tile_type: MID
    6, 6, 10, 8, // id: 185, tile_type: MID
    16, 5, 10, 14, // id: 186, tile_type: MID
    10, 16, 10, 5, // id: 187, tile_type: MID
    2, 14, 10, 5, // id: 188, tile_type: MID
    12, 14, 10, 5, // id: 189, tile_type: MID
    2, 2, 10, 11, // id: 190, tile_type: MID
    11, 15, 10, 11, // id: 191, tile_type: MID
    8, 5, 10, 3, // id: 192, tile_type: MID
    16, 8, 10, 9, // id: 193, tile_type: MID
    16, 17, 10, 9, // id: 194, tile_type: MID
    17, 11, 10, 9, // id: 195, tile_type: MID
    8, 17, 10, 15, // id: 196, tile_type: MID
    14, 6, 10, 15, // id: 197, tile_type: MID
    15, 15, 10, 15, // id: 198, tile_type: MID
    14, 2, 10, 6, // id: 199, tile_type: MID
    16, 6, 10, 12, // id: 200, tile_type: MID
    17, 5, 10, 12, // id: 201, tile_type: MID
    9, 14, 10, 16, // id: 202, tile_type: MID
    9, 6, 10, 16, // id: 203, tile_type: MID
    12, 2, 10, 16, // id: 204, tile_type: MID
    17, 16, 10, 16, // id: 205, tile_type: MID
    17, 5, 2, 2, // id: 206, tile_type: MID
    14, 17, 2, 8, // id: 207, tile_type: MID
    6, 11, 2, 8, // id: 208, tile_type: MID
    8, 14, 2, 5, // id: 209, tile_type: MID
    12, 11, 2, 5, // id: 210, tile_type: MID
    2, 12, 2, 11, // id: 211, tile_type: MID
    6, 6, 2, 3, // id: 212, tile_type: MID
    2, 15, 2, 9, // id: 213, tile_type: MID
    3, 17, 2, 9, // id: 214, tile_type: MID
    17, 16, 2, 9, // id: 215, tile_type: MID
    14, 11, 2, 15, // id: 216, tile_type: MID
    3, 9, 2, 12, // id: 217, tile_type: MID
    8, 3, 2, 16, // id: 218, tile_type: MID
    8, 12, 2, 17, // id: 219, tile_type: MID
    16, 16, 2, 17, // id: 220, tile_type: MID
    16, 17, 2, 17, // id: 221, tile_type: MID
    9, 17, 8, 8, // id: 222, tile_type: MID
    15, 5, 8, 8, // id: 223, tile_type: MID
    12, 11, 8, 8, // id: 224, tile_type: MID
    14, 11, 8, 5, // id: 225, tile_type: MID
    9, 9, 8, 5, // id: 226, tile_type: MID
    14, 6, 8, 3, // id: 227, tile_type: MID
    5, 17, 8, 15, // id: 228, tile_type: MID
    12, 6, 8, 15, // id: 229, tile_type: MID
    9, 15, 8, 6, // id: 230, tile_type: MID
    9, 16, 8, 6, // id: 231, tile_type: MID
    14, 15, 14, 14, // id: 232, tile_type: MID
    12, 3, 14, 5, // id: 233, tile_type: MID
    5, 3, 14, 3, // id: 234, tile_type: MID
    5, 15, 14, 3, // id: 235, tile_type: MID
    9, 11, 14, 3, // id: 236, tile_type: MID
    3, 12, 14, 9, // id: 237, tile_type: MID
    11, 17, 14, 15, // id: 238, tile_type: MID
    16, 11, 14, 15, // id: 239, tile_type: MID
    5, 16, 14, 6, // id: 240, tile_type: MID
    3, 16, 14, 6, // id: 241, tile_type: MID
    11, 9, 5, 11, // id: 242, tile_type: MID
    15, 6, 5, 11, // id: 243, tile_type: MID
    17, 15, 5, 3, // id: 244, tile_type: MID
    12, 17, 5, 16, // id: 245, tile_type: MID
    16, 17, 11, 11, // id: 246, tile_type: MID
    9, 3, 11, 3, // id: 247, tile_type: MID
    3, 16, 11, 9, // id: 248, tile_type: MID
    12, 16, 11, 15, // id: 249, tile_type: MID
    17, 6, 3, 3, // id: 250, tile_type: MID
    6, 9, 3, 9, // id: 251, tile_type: MID
    12, 15, 9, 6, // id: 252, tile_type: MID
    16, 12, 15, 12, // id: 253, tile_type: MID
    12, 17, 15, 17, // id: 254, tile_type: MID
    16, 17, 6, 17, // id: 255, tile_type: MID
];

impl Backtracker<'_> {
    pub const ADD_TILE_FUNCTIONS: [fn(&mut Self, usize) -> (); 196] = [
        Backtracker::add_tile_0,
        Backtracker::add_tile_top,
        Backtracker::add_tile_top,
        Backtracker::add_tile_top,
        Backtracker::add_tile_top,
        Backtracker::add_tile_top,
        Backtracker::add_tile_top,
        Backtracker::add_tile_top,
        Backtracker::add_tile_top,
        Backtracker::add_tile_top,
        Backtracker::add_tile_top,
        Backtracker::add_tile_top,
        Backtracker::add_tile_top,
        Backtracker::add_tile_top,
        Backtracker::add_tile_left,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_left,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_left,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_left,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_left,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_left,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_left,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_left,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_left,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_left,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_left,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_left,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_left,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_mid,
        Backtracker::add_tile_final,
    ];
}

pub static MIDS_BICOLOUR_ARRAY: [[u16; 18]; 18] = [
    [
        2, 12, 30, 44, 54, 64, 78, 0, 84, 98, 112, 130, 140, 162, 172, 182, 200, 210,
    ], // north 1
    [
        388, 398, 420, 434, 444, 458, 468, 482, 488, 498, 512, 534, 548, 566, 580, 590, 604, 618,
    ], // north 2
    [
        816, 826, 840, 846, 856, 870, 884, 890, 908, 930, 948, 958, 972, 990, 1012, 0, 1018, 1032,
    ], // north 3
    [
        1218, 1224, 1230, 1240, 0, 1254, 1268, 1286, 1296, 1302, 1308, 1326, 1348, 1378, 1388,
        1406, 1424, 1430,
    ], // north 4
    [
        1612, 1626, 1644, 1666, 1680, 1686, 1696, 1702, 1720, 1730, 0, 1748, 1754, 1768, 1774,
        1796, 1806, 1828,
    ], // north 5
    [
        2014, 2040, 2046, 2060, 0, 2070, 2084, 2094, 2116, 2134, 2140, 2146, 2152, 2158, 2184,
        2202, 2208, 2222,
    ], // north 6
    [
        2400, 2406, 2416, 2426, 0, 2436, 2450, 2472, 2486, 2500, 2506, 2532, 0, 2554, 2568, 2578,
        2588, 2602,
    ], // north 7
    [
        2776, 2794, 2804, 2814, 2828, 2850, 2864, 2882, 2900, 2910, 2920, 2934, 2940, 2946, 2952,
        2962, 2984, 3002,
    ], // north 8
    [
        3196, 3214, 3228, 3246, 3256, 3274, 3304, 3314, 3328, 3334, 3352, 3374, 3384, 3390, 3400,
        3406, 0, 3420,
    ], // north 9
    [
        3614, 3620, 3638, 3644, 3658, 3680, 3702, 3712, 3726, 3740, 3750, 3756, 3766, 3776, 3794,
        3812, 3822, 3836,
    ], // north 10
    [
        4026, 4032, 4042, 4056, 4070, 4080, 4090, 4096, 4106, 4120, 4138, 4160, 0, 4170, 4188,
        4202, 4220, 4234,
    ], // north 11
    [
        0, 4412, 4426, 4440, 4458, 4488, 4494, 4516, 4530, 4544, 0, 0, 4558, 0, 4568, 4590, 4604,
        4622,
    ], // north 12
    [
        4808, 4834, 4848, 4858, 4872, 0, 4886, 4892, 4910, 4936, 0, 4946, 4960, 4974, 4984, 4994,
        5008, 5018,
    ], // north 13
    [
        5200, 5210, 5216, 5234, 5252, 5262, 5276, 5282, 5300, 5306, 5320, 5334, 5352, 5366, 5384,
        5406, 5416, 5422,
    ], // north 14
    [
        5612, 5626, 0, 5644, 5658, 0, 5672, 5686, 5704, 5714, 5732, 5746, 5768, 5786, 5800, 0,
        5814, 5828,
    ], // north 15
    [
        6018, 6028, 6038, 6052, 6066, 6084, 6094, 0, 6104, 6118, 6140, 6146, 0, 6164, 6182, 6188,
        6206, 6236,
    ], // north 16
    [
        6426, 6444, 6466, 6488, 6494, 6504, 6514, 6536, 6546, 6564, 6570, 6576, 6590, 6612, 6618,
        6624, 6646, 6652,
    ], // north 17
    [
        6846, 7024, 7222, 7408, 7590, 7776, 7954, 8128, 8322, 8516, 8706, 8884, 9070, 9252, 9442,
        9632, 9822, 10016,
    ], // north ANY
];

pub static BICOLOUR_TILES: [u8; 13154] = [
    // unused
    0, 0,
    // topLeftCornersWithTwoColours
    // topRightCornersWithTwoColours
    // bottomLeftCornersWithTwoColours
    // bottomRightCornersWithTwoColours
    // topEdgesWithTwoColours
    // rightEdgesWithTwoColours
    // bottomEdgesWithTwoColours
    // leftEdgesWithTwoColours
    // midsWithTwoColours
    2, 0, // 257
    0, 3, 12, 3, 1, 3, 4, 9, 4, 0, // 258
    2, 2, 6, 6, 14, 2, 5, 7, 18, 2, 14, 4, 35, 2, 15, 15, 3, 0, // 259
    12, 2, 10, 7, 22, 2, 7, 2, 32, 2, 13, 5, 2, 0, // 260
    7, 2, 14, 12, 38, 2, 15, 16, 2, 0, // 261
    1, 2, 9, 0, 9, 2, 1, 1, 3, 0, // 262
    3, 2, 0, 12, 8, 2, 16, 12, 37, 2, 14, 16, 1, 0, // 263
    6, 2, 7, 12, 3, 0, // 265
    10, 2, 4, 1, 25, 2, 5, 8, 30, 2, 7, 5, 3, 0, // 266
    5, 2, 9, 12, 24, 2, 3, 8, 34, 2, 8, 5, 4, 0, // 267
    13, 2, 14, 7, 15, 2, 9, 13, 16, 2, 13, 13, 31, 2, 13, 5, 2, 0, // 268
    17, 2, 1, 4, 29, 2, 2, 14, 5, 0, // 269
    0, 2, 3, 0, 3, 0, 0, 5, 11, 2, 9, 7, 20, 2, 13, 10, 21, 2, 12, 2, 2, 0, // 270
    23, 2, 12, 8, 36, 2, 2, 16, 2, 0, // 271
    26, 2, 11, 8, 27, 0, 0, 15, 4, 0, // 272
    19, 2, 11, 4, 27, 2, 0, 14, 33, 2, 2, 5, 39, 2, 16, 16, 2, 0, // 273
    4, 2, 12, 12, 28, 2, 3, 14, 44, 0, // 274
    0, 3, 12, 3, 0, 2, 3, 0, 1, 3, 4, 9, 1, 2, 9, 0, 2, 2, 6, 6, 3, 0, 0, 5, 3, 2, 0, 12, 4, 2, 12,
    12, 5, 2, 9, 12, 6, 2, 7, 12, 7, 2, 14, 12, 8, 2, 16, 12, 9, 2, 1, 1, 10, 2, 4, 1, 11, 2, 9, 7,
    12, 2, 10, 7, 13, 2, 14, 7, 14, 2, 5, 7, 15, 2, 9, 13, 16, 2, 13, 13, 17, 2, 1, 4, 18, 2, 14,
    4, 19, 2, 11, 4, 20, 2, 13, 10, 21, 2, 12, 2, 22, 2, 7, 2, 23, 2, 12, 8, 24, 2, 3, 8, 25, 2, 5,
    8, 26, 2, 11, 8, 27, 0, 0, 15, 27, 2, 0, 14, 28, 2, 3, 14, 29, 2, 2, 14, 30, 2, 7, 5, 31, 2,
    13, 5, 32, 2, 13, 5, 33, 2, 2, 5, 34, 2, 8, 5, 35, 2, 15, 15, 36, 2, 2, 16, 37, 2, 14, 16, 38,
    2, 15, 16, 39, 2, 16, 16, 2, 0, // 513
    9, 3, 4, 1, 10, 3, 8, 4, 5, 0, // 514
    9, 0, 0, 4, 87, 0, 12, 9, 106, 1, 7, 3, 130, 1, 10, 9, 146, 3, 4, 16, 3, 0, // 515
    68, 1, 11, 6, 74, 1, 16, 6, 158, 2, 7, 15, 2, 0, // 516
    45, 0, 6, 5, 77, 1, 3, 12, 3, 0, // 517
    17, 0, 0, 11, 128, 0, 9, 13, 146, 2, 16, 1, 2, 0, // 518
    14, 1, 7, 0, 152, 2, 5, 2, 3, 0, // 519
    2, 1, 6, 0, 51, 3, 13, 14, 52, 3, 11, 14, 1, 0, // 520
    106, 0, 3, 1, 2, 0, // 521
    153, 0, 1, 14, 157, 2, 2, 11, 3, 0, // 522
    84, 0, 12, 2, 104, 0, 3, 2, 123, 3, 4, 16, 5, 0, // 523
    130, 0, 9, 1, 148, 2, 5, 7, 150, 2, 11, 4, 151, 0, 1, 11, 156, 2, 13, 14, 3, 0, // 524
    144, 1, 15, 9, 151, 2, 1, 10, 159, 2, 7, 16, 4, 0, // 525
    85, 3, 8, 12, 86, 3, 16, 12, 87, 3, 9, 1, 88, 3, 8, 10, 3, 0, // 526
    57, 0, 6, 15, 139, 1, 5, 9, 149, 2, 7, 4, 2, 0, // 527
    18, 1, 4, 0, 153, 2, 1, 8, 3, 0, // 528
    35, 1, 15, 0, 155, 2, 16, 8, 160, 2, 15, 16, 3, 0, // 529
    147, 2, 13, 7, 154, 2, 2, 8, 161, 2, 15, 16, 49, 0, // 530
    2, 1, 6, 0, 9, 0, 0, 4, 9, 3, 4, 1, 10, 3, 8, 4, 14, 1, 7, 0, 17, 0, 0, 11, 18, 1, 4, 0, 35, 1,
    15, 0, 45, 0, 6, 5, 51, 3, 13, 14, 52, 3, 11, 14, 57, 0, 6, 15, 68, 1, 11, 6, 74, 1, 16, 6, 77,
    1, 3, 12, 84, 0, 12, 2, 85, 3, 8, 12, 86, 3, 16, 12, 87, 0, 12, 9, 87, 3, 9, 1, 88, 3, 8, 10,
    104, 0, 3, 2, 106, 0, 3, 1, 106, 1, 7, 3, 123, 3, 4, 16, 128, 0, 9, 13, 130, 0, 9, 1, 130, 1,
    10, 9, 139, 1, 5, 9, 144, 1, 15, 9, 146, 3, 4, 16, 146, 2, 16, 1, 147, 2, 13, 7, 148, 2, 5, 7,
    149, 2, 7, 4, 150, 2, 11, 4, 151, 0, 1, 11, 151, 2, 1, 10, 152, 2, 5, 2, 153, 0, 1, 14, 153, 2,
    1, 8, 154, 2, 2, 8, 155, 2, 16, 8, 156, 2, 13, 14, 157, 2, 2, 11, 158, 2, 7, 15, 159, 2, 7, 16,
    160, 2, 15, 16, 161, 2, 15, 16, 2, 0, // 769
    21, 3, 12, 12, 22, 3, 2, 7, 3, 0, // 770
    84, 1, 9, 12, 104, 1, 9, 3, 152, 3, 5, 5, 1, 0, // 771
    190, 3, 5, 16, 2, 0, // 772
    47, 0, 6, 10, 114, 3, 15, 4, 3, 0, // 773
    109, 0, 3, 5, 174, 1, 2, 13, 184, 3, 14, 16, 3, 0, // 774
    33, 0, 0, 15, 181, 0, 13, 15, 190, 2, 16, 2, 1, 0, // 775
    60, 3, 16, 9, 4, 0, // 776
    22, 1, 2, 0, 73, 1, 16, 6, 158, 1, 15, 1, 167, 3, 5, 13, 5, 0, // 777
    154, 0, 1, 16, 177, 0, 13, 11, 187, 1, 2, 10, 188, 0, 10, 15, 191, 2, 5, 8, 4, 0, // 778
    53, 1, 7, 6, 105, 0, 3, 5, 116, 1, 14, 3, 132, 3, 4, 7, 2, 0, // 779
    12, 1, 7, 0, 187, 3, 2, 8, 3, 0, // 780
    68, 0, 6, 1, 157, 0, 1, 8, 173, 1, 4, 13, 4, 0, // 781
    44, 0, 6, 10, 94, 3, 4, 4, 95, 3, 8, 4, 96, 3, 4, 16, 5, 0, // 782
    32, 1, 5, 0, 90, 0, 12, 16, 174, 3, 2, 4, 175, 3, 14, 4, 176, 3, 10, 8, 1, 0, // 783
    29, 0, 0, 11, 3, 0, // 785
    36, 0, 0, 13, 74, 0, 6, 1, 103, 0, 12, 11, 46, 0, // 786
    12, 1, 7, 0, 21, 3, 12, 12, 22, 3, 2, 7, 22, 1, 2, 0, 29, 0, 0, 11, 32, 1, 5, 0, 33, 0, 0, 15,
    36, 0, 0, 13, 44, 0, 6, 10, 47, 0, 6, 10, 53, 1, 7, 6, 60, 3, 16, 9, 68, 0, 6, 1, 73, 1, 16, 6,
    74, 0, 6, 1, 84, 1, 9, 12, 90, 0, 12, 16, 94, 3, 4, 4, 95, 3, 8, 4, 96, 3, 4, 16, 103, 0, 12,
    11, 104, 1, 9, 3, 105, 0, 3, 5, 109, 0, 3, 5, 114, 3, 15, 4, 116, 1, 14, 3, 132, 3, 4, 7, 152,
    3, 5, 5, 154, 0, 1, 16, 157, 0, 1, 8, 158, 1, 15, 1, 167, 3, 5, 13, 173, 1, 4, 13, 174, 3, 2,
    4, 174, 1, 2, 13, 175, 3, 14, 4, 176, 3, 10, 8, 177, 0, 13, 11, 181, 0, 13, 15, 184, 3, 14, 16,
    187, 3, 2, 8, 187, 1, 2, 10, 188, 0, 10, 15, 190, 3, 5, 16, 190, 2, 16, 2, 191, 2, 5, 8, 1,
    0, // 1025
    0, 0, 0, 12, 1, 0, // 1026
    106, 2, 1, 7, 2, 0, // 1027
    104, 2, 1, 9, 116, 2, 9, 14, 3, 0, // 1028
    77, 0, 12, 1, 78, 0, 12, 7, 79, 0, 12, 8, 3, 0, // 1030
    98, 1, 14, 12, 105, 2, 2, 9, 109, 2, 2, 4, 4, 0, // 1031
    45, 3, 5, 1, 46, 3, 5, 13, 47, 3, 10, 2, 48, 3, 7, 11, 2, 0, // 1032
    111, 2, 15, 4, 121, 3, 4, 15, 1, 0, // 1033
    24, 0, 0, 9, 1, 0, // 1034
    66, 1, 11, 6, 4, 0, // 1035
    81, 1, 3, 12, 107, 2, 7, 13, 112, 2, 10, 10, 118, 2, 5, 5, 5, 0, // 1036
    100, 0, 12, 14, 110, 2, 14, 4, 117, 2, 8, 5, 119, 2, 4, 11, 120, 2, 7, 15, 7, 0, // 1037
    77, 3, 1, 3, 78, 3, 7, 3, 79, 3, 8, 3, 80, 3, 15, 13, 81, 3, 3, 10, 82, 3, 8, 11, 83, 3, 15,
    15, 2, 0, // 1038
    89, 0, 12, 7, 115, 2, 4, 8, 4, 0, // 1039
    7, 1, 12, 0, 28, 0, 0, 16, 61, 0, 6, 11, 113, 2, 8, 10, 4, 0, // 1040
    38, 1, 16, 0, 70, 0, 6, 14, 108, 2, 13, 13, 114, 2, 4, 2, 1, 0, // 1041
    43, 1, 6, 6, 45, 0, // 1042
    0, 0, 0, 12, 7, 1, 12, 0, 24, 0, 0, 9, 28, 0, 0, 16, 38, 1, 16, 0, 43, 1, 6, 6, 45, 3, 5, 1,
    46, 3, 5, 13, 47, 3, 10, 2, 48, 3, 7, 11, 61, 0, 6, 11, 66, 1, 11, 6, 70, 0, 6, 14, 77, 0, 12,
    1, 77, 3, 1, 3, 78, 0, 12, 7, 78, 3, 7, 3, 79, 0, 12, 8, 79, 3, 8, 3, 80, 3, 15, 13, 81, 3, 3,
    10, 81, 1, 3, 12, 82, 3, 8, 11, 83, 3, 15, 15, 89, 0, 12, 7, 98, 1, 14, 12, 100, 0, 12, 14,
    104, 2, 1, 9, 105, 2, 2, 9, 106, 2, 1, 7, 107, 2, 7, 13, 108, 2, 13, 13, 109, 2, 2, 4, 110, 2,
    14, 4, 111, 2, 15, 4, 112, 2, 10, 10, 113, 2, 8, 10, 114, 2, 4, 2, 115, 2, 4, 8, 116, 2, 9, 14,
    117, 2, 8, 5, 118, 2, 5, 5, 119, 2, 4, 11, 120, 2, 7, 15, 121, 3, 4, 15, 3, 0, // 1281
    17, 3, 11, 1, 18, 3, 1, 14, 19, 3, 15, 11, 4, 0, // 1282
    9, 1, 1, 0, 10, 0, 0, 8, 149, 3, 13, 7, 150, 3, 10, 11, 5, 0, // 1283
    94, 0, 12, 4, 95, 0, 12, 8, 114, 0, 3, 15, 174, 0, 13, 2, 175, 0, 13, 14, 3, 0, // 1284
    109, 3, 5, 2, 110, 3, 11, 14, 111, 3, 7, 15, 1, 0, // 1285
    94, 1, 2, 12, 2, 0, // 1286
    180, 0, 13, 15, 183, 2, 14, 10, 1, 0, // 1287
    58, 3, 8, 11, 4, 0, // 1288
    54, 0, 6, 8, 132, 1, 2, 9, 165, 3, 10, 13, 166, 3, 8, 8, 2, 0, // 1289
    115, 0, 3, 13, 182, 2, 10, 10, 4, 0, // 1290
    1, 1, 0, 0, 127, 3, 15, 9, 128, 3, 13, 1, 129, 3, 13, 11, 1, 0, // 1292
    119, 0, 3, 11, 3, 0, // 1293
    91, 3, 7, 7, 92, 3, 13, 7, 93, 3, 11, 16, 1, 0, // 1294
    173, 3, 2, 11, 5, 0, // 1295
    76, 1, 12, 12, 97, 0, 12, 11, 163, 1, 7, 7, 168, 0, 7, 16, 184, 2, 16, 2, 2, 0, // 1296
    121, 1, 3, 7, 126, 1, 13, 9, 5, 0, // 1297
    96, 1, 2, 12, 123, 1, 1, 9, 141, 1, 11, 9, 146, 1, 1, 1, 185, 2, 11, 15, 46, 0, // 1298
    1, 1, 0, 0, 9, 1, 1, 0, 10, 0, 0, 8, 17, 3, 11, 1, 18, 3, 1, 14, 19, 3, 15, 11, 54, 0, 6, 8,
    58, 3, 8, 11, 76, 1, 12, 12, 91, 3, 7, 7, 92, 3, 13, 7, 93, 3, 11, 16, 94, 0, 12, 4, 94, 1, 2,
    12, 95, 0, 12, 8, 96, 1, 2, 12, 97, 0, 12, 11, 109, 3, 5, 2, 110, 3, 11, 14, 111, 3, 7, 15,
    114, 0, 3, 15, 115, 0, 3, 13, 119, 0, 3, 11, 121, 1, 3, 7, 123, 1, 1, 9, 126, 1, 13, 9, 127, 3,
    15, 9, 128, 3, 13, 1, 129, 3, 13, 11, 132, 1, 2, 9, 141, 1, 11, 9, 146, 1, 1, 1, 149, 3, 13, 7,
    150, 3, 10, 11, 163, 1, 7, 7, 165, 3, 10, 13, 166, 3, 8, 8, 168, 0, 7, 16, 173, 3, 2, 11, 174,
    0, 13, 2, 175, 0, 13, 14, 180, 0, 13, 15, 182, 2, 10, 10, 183, 2, 14, 10, 184, 2, 16, 2, 185,
    2, 11, 15, 6, 0, // 1537
    3, 1, 12, 0, 30, 3, 8, 7, 31, 3, 10, 13, 32, 3, 2, 13, 33, 3, 15, 2, 34, 3, 9, 8, 1,
    0, // 1538
    45, 1, 3, 6, 3, 0, // 1539
    105, 1, 9, 3, 109, 1, 4, 3, 152, 0, 1, 5, 2, 0, // 1540
    117, 3, 11, 8, 118, 3, 10, 5, 3, 0, // 1542
    118, 0, 3, 10, 125, 1, 7, 9, 152, 1, 2, 1, 2, 0, // 1543
    64, 3, 16, 8, 65, 3, 10, 15, 5, 0, // 1544
    14, 0, 0, 1, 125, 0, 9, 5, 148, 0, 1, 10, 170, 3, 14, 8, 171, 3, 15, 8, 4, 0, // 1545
    25, 0, 0, 8, 143, 1, 15, 9, 191, 0, 2, 8, 192, 3, 14, 11, 1, 0, // 1546
    139, 3, 1, 13, 1, 0, // 1547
    59, 0, 6, 16, 1, 0, // 1548
    169, 1, 14, 7, 1, 0, // 1549
    99, 3, 15, 15, 6, 0, // 1550
    46, 1, 3, 6, 122, 2, 9, 9, 137, 1, 14, 9, 167, 1, 2, 7, 180, 3, 15, 4, 181, 3, 15, 2, 4,
    0, // 1551
    37, 1, 16, 0, 69, 1, 11, 6, 98, 0, 12, 3, 183, 1, 10, 4, 1, 0, // 1552
    140, 1, 11, 9, 3, 0, // 1553
    8, 1, 12, 0, 190, 1, 2, 2, 195, 2, 15, 16, 44, 0, // 1554
    3, 1, 12, 0, 8, 1, 12, 0, 14, 0, 0, 1, 25, 0, 0, 8, 30, 3, 8, 7, 31, 3, 10, 13, 32, 3, 2, 13,
    33, 3, 15, 2, 34, 3, 9, 8, 37, 1, 16, 0, 45, 1, 3, 6, 46, 1, 3, 6, 59, 0, 6, 16, 64, 3, 16, 8,
    65, 3, 10, 15, 69, 1, 11, 6, 98, 0, 12, 3, 99, 3, 15, 15, 105, 1, 9, 3, 109, 1, 4, 3, 117, 3,
    11, 8, 118, 0, 3, 10, 118, 3, 10, 5, 122, 2, 9, 9, 125, 0, 9, 5, 125, 1, 7, 9, 137, 1, 14, 9,
    139, 3, 1, 13, 140, 1, 11, 9, 143, 1, 15, 9, 148, 0, 1, 10, 152, 0, 1, 5, 152, 1, 2, 1, 167, 1,
    2, 7, 169, 1, 14, 7, 170, 3, 14, 8, 171, 3, 15, 8, 180, 3, 15, 4, 181, 3, 15, 2, 183, 1, 10, 4,
    190, 1, 2, 2, 191, 0, 2, 8, 192, 3, 14, 11, 195, 2, 15, 16, 1, 0, // 1793
    2, 3, 1, 6, 2, 0, // 1794
    68, 2, 2, 11, 74, 2, 2, 16, 2, 0, // 1795
    53, 2, 9, 7, 73, 2, 7, 16, 2, 0, // 1796
    43, 2, 16, 6, 66, 2, 9, 11, 3, 0, // 1798
    45, 2, 1, 3, 46, 2, 13, 3, 69, 2, 14, 11, 5, 0, // 1799
    2, 0, 0, 1, 40, 3, 10, 8, 41, 3, 11, 8, 42, 3, 13, 11, 43, 3, 3, 16, 3, 0, // 1800
    6, 1, 12, 0, 48, 2, 11, 3, 55, 2, 8, 7, 3, 0, // 1801
    49, 2, 10, 9, 54, 2, 4, 7, 58, 2, 11, 4, 1, 0, // 1802
    71, 2, 8, 15, 6, 0, // 1803
    40, 2, 8, 6, 44, 2, 2, 12, 47, 2, 2, 3, 50, 2, 8, 9, 63, 2, 14, 14, 65, 2, 15, 5, 5,
    0, // 1804
    41, 2, 8, 6, 52, 2, 14, 1, 56, 2, 16, 7, 61, 2, 3, 14, 72, 2, 9, 16, 3, 0, // 1806
    42, 2, 11, 6, 51, 2, 14, 1, 62, 2, 9, 14, 2, 0, // 1807
    70, 2, 3, 15, 75, 2, 11, 16, 2, 0, // 1808
    57, 2, 1, 13, 67, 2, 13, 11, 3, 0, // 1809
    59, 2, 5, 10, 60, 2, 9, 2, 64, 2, 8, 5, 43, 0, // 1810
    2, 0, 0, 1, 2, 3, 1, 6, 6, 1, 12, 0, 40, 3, 10, 8, 40, 2, 8, 6, 41, 3, 11, 8, 41, 2, 8, 6, 42,
    3, 13, 11, 42, 2, 11, 6, 43, 3, 3, 16, 43, 2, 16, 6, 44, 2, 2, 12, 45, 2, 1, 3, 46, 2, 13, 3,
    47, 2, 2, 3, 48, 2, 11, 3, 49, 2, 10, 9, 50, 2, 8, 9, 51, 2, 14, 1, 52, 2, 14, 1, 53, 2, 9, 7,
    54, 2, 4, 7, 55, 2, 8, 7, 56, 2, 16, 7, 57, 2, 1, 13, 58, 2, 11, 4, 59, 2, 5, 10, 60, 2, 9, 2,
    61, 2, 3, 14, 62, 2, 9, 14, 63, 2, 14, 14, 64, 2, 8, 5, 65, 2, 15, 5, 66, 2, 9, 11, 67, 2, 13,
    11, 68, 2, 2, 11, 69, 2, 14, 11, 70, 2, 3, 15, 71, 2, 8, 15, 72, 2, 9, 16, 73, 2, 7, 16, 74, 2,
    2, 16, 75, 2, 11, 16, 4, 0, // 2049
    11, 3, 12, 9, 12, 3, 2, 10, 13, 3, 10, 14, 14, 3, 1, 5, 2, 0, // 2050
    147, 3, 16, 13, 148, 3, 10, 5, 2, 0, // 2051
    22, 0, 0, 2, 132, 0, 9, 4, 3, 0, // 2052
    78, 1, 3, 12, 89, 1, 13, 12, 106, 3, 1, 1, 5, 0, // 2053
    91, 0, 12, 7, 92, 0, 12, 13, 121, 2, 15, 3, 149, 0, 1, 13, 163, 2, 14, 7, 3, 0, // 2054
    30, 0, 0, 8, 167, 2, 13, 2, 169, 2, 11, 14, 4, 0, // 2055
    53, 3, 2, 9, 54, 3, 8, 4, 55, 3, 7, 8, 56, 3, 11, 16, 4, 0, // 2056
    91, 1, 4, 12, 162, 3, 16, 8, 163, 3, 4, 14, 164, 3, 10, 11, 2, 0, // 2057
    55, 1, 7, 6, 166, 2, 8, 4, 2, 0, // 2058
    124, 3, 8, 13, 125, 3, 5, 5, 3, 0, // 2059
    102, 1, 16, 12, 164, 2, 11, 7, 165, 2, 13, 4, 1, 0, // 2060
    48, 1, 3, 6, 1, 0, // 2061
    6, 0, 0, 6, 1, 0, // 2062
    107, 0, 3, 10, 2, 0, // 2063
    136, 0, 9, 16, 170, 2, 8, 5, 5, 0, // 2064
    111, 1, 4, 3, 120, 0, 3, 11, 133, 1, 8, 9, 158, 0, 1, 2, 171, 2, 8, 5, 4, 0, // 2065
    73, 0, 6, 2, 159, 0, 1, 11, 162, 2, 8, 7, 168, 2, 4, 14, 48, 0, // 2066
    6, 0, 0, 6, 11, 3, 12, 9, 12, 3, 2, 10, 13, 3, 10, 14, 14, 3, 1, 5, 22, 0, 0, 2, 30, 0, 0, 8,
    48, 1, 3, 6, 53, 3, 2, 9, 54, 3, 8, 4, 55, 3, 7, 8, 55, 1, 7, 6, 56, 3, 11, 16, 73, 0, 6, 2,
    78, 1, 3, 12, 89, 1, 13, 12, 91, 0, 12, 7, 91, 1, 4, 12, 92, 0, 12, 13, 102, 1, 16, 12, 106, 3,
    1, 1, 107, 0, 3, 10, 111, 1, 4, 3, 120, 0, 3, 11, 121, 2, 15, 3, 124, 3, 8, 13, 125, 3, 5, 5,
    132, 0, 9, 4, 133, 1, 8, 9, 136, 0, 9, 16, 147, 3, 16, 13, 148, 3, 10, 5, 149, 0, 1, 13, 158,
    0, 1, 2, 159, 0, 1, 11, 162, 3, 16, 8, 162, 2, 8, 7, 163, 3, 4, 14, 163, 2, 14, 7, 164, 3, 10,
    11, 164, 2, 11, 7, 165, 2, 13, 4, 166, 2, 8, 4, 167, 2, 13, 2, 168, 2, 4, 14, 169, 2, 11, 14,
    170, 2, 8, 5, 171, 2, 8, 5, 4, 0, // 2305
    23, 3, 13, 12, 24, 3, 9, 3, 25, 3, 8, 5, 26, 3, 14, 11, 3, 0, // 2306
    153, 3, 14, 1, 154, 3, 16, 2, 155, 3, 15, 16, 4, 0, // 2307
    157, 1, 11, 1, 176, 0, 13, 10, 187, 0, 10, 2, 191, 3, 8, 5, 2, 0, // 2308
    79, 1, 3, 12, 115, 3, 13, 4, 4, 0, // 2309
    10, 1, 1, 0, 54, 1, 7, 6, 95, 1, 2, 12, 166, 0, 7, 8, 7, 0, // 2310
    25, 1, 8, 0, 34, 0, 0, 9, 64, 0, 6, 16, 117, 0, 3, 11, 170, 0, 7, 14, 171, 0, 7, 15, 191, 1, 8,
    2, 2, 0, // 2311
    40, 0, 6, 10, 41, 0, 6, 11, 3, 0, // 2312
    30, 1, 5, 0, 55, 0, 6, 7, 162, 0, 7, 16, 1, 0, // 2313
    166, 1, 4, 7, 4, 0, // 2314
    50, 0, 6, 10, 133, 3, 7, 15, 134, 3, 16, 15, 135, 3, 10, 16, 5, 0, // 2315
    49, 1, 9, 6, 88, 1, 1, 12, 113, 0, 3, 14, 182, 1, 10, 4, 188, 3, 15, 2, 2, 0, // 2316
    58, 1, 4, 6, 82, 1, 3, 12, 1, 0, // 2317
    85, 1, 1, 12, 2, 0, // 2318
    124, 1, 7, 9, 177, 3, 11, 2, 1, 0, // 2319
    192, 2, 11, 5, 3, 0, // 2320
    71, 0, 6, 9, 142, 0, 9, 13, 143, 0, 9, 5, 48, 0, // 2322
    10, 1, 1, 0, 23, 3, 13, 12, 24, 3, 9, 3, 25, 3, 8, 5, 25, 1, 8, 0, 26, 3, 14, 11, 30, 1, 5, 0,
    34, 0, 0, 9, 40, 0, 6, 10, 41, 0, 6, 11, 49, 1, 9, 6, 50, 0, 6, 10, 54, 1, 7, 6, 55, 0, 6, 7,
    58, 1, 4, 6, 64, 0, 6, 16, 71, 0, 6, 9, 79, 1, 3, 12, 82, 1, 3, 12, 85, 1, 1, 12, 88, 1, 1, 12,
    95, 1, 2, 12, 113, 0, 3, 14, 115, 3, 13, 4, 117, 0, 3, 11, 124, 1, 7, 9, 133, 3, 7, 15, 134, 3,
    16, 15, 135, 3, 10, 16, 142, 0, 9, 13, 143, 0, 9, 5, 153, 3, 14, 1, 154, 3, 16, 2, 155, 3, 15,
    16, 157, 1, 11, 1, 162, 0, 7, 16, 166, 0, 7, 8, 166, 1, 4, 7, 170, 0, 7, 14, 171, 0, 7, 15,
    176, 0, 13, 10, 177, 3, 11, 2, 182, 1, 10, 4, 187, 0, 10, 2, 188, 3, 15, 2, 191, 3, 8, 5, 191,
    1, 8, 2, 192, 2, 11, 5, 1, 0, // 2561
    1, 0, 0, 4, 4, 0, // 2562
    87, 1, 1, 12, 130, 2, 1, 10, 139, 2, 13, 5, 144, 2, 11, 15, 1, 0, // 2563
    60, 0, 6, 16, 3, 0, // 2564
    24, 1, 8, 0, 104, 3, 2, 1, 105, 3, 5, 2, 5, 0, // 2565
    123, 2, 16, 1, 126, 2, 15, 13, 127, 0, 9, 15, 132, 2, 7, 2, 141, 2, 16, 11, 5, 0, // 2566
    122, 3, 13, 9, 125, 2, 5, 7, 137, 2, 13, 14, 140, 2, 15, 11, 143, 2, 8, 15, 2, 0, // 2567
    49, 3, 8, 10, 50, 3, 10, 8, 3, 0, // 2568
    11, 0, 0, 12, 53, 0, 6, 2, 133, 2, 15, 8, 3, 0, // 2569
    34, 1, 5, 0, 71, 1, 15, 6, 124, 2, 13, 7, 2, 0, // 2570
    5, 1, 12, 0, 122, 0, 5, 13, 1, 0, // 2571
    135, 2, 16, 8, 2, 0, // 2572
    66, 0, 6, 3, 101, 0, 12, 14, 2, 0, // 2573
    5, 0, 0, 9, 84, 3, 2, 1, 4, 0, // 2574
    15, 0, 0, 10, 128, 2, 1, 4, 129, 2, 11, 4, 142, 2, 8, 15, 4, 0, // 2575
    62, 0, 6, 13, 116, 0, 3, 2, 131, 2, 10, 10, 138, 2, 14, 14, 2, 0, // 2576
    127, 2, 9, 4, 145, 2, 16, 15, 3, 0, // 2577
    72, 0, 6, 11, 134, 2, 15, 8, 136, 2, 7, 14, 47, 0, // 2578
    1, 0, 0, 4, 5, 0, 0, 9, 5, 1, 12, 0, 11, 0, 0, 12, 15, 0, 0, 10, 24, 1, 8, 0, 34, 1, 5, 0, 49,
    3, 8, 10, 50, 3, 10, 8, 53, 0, 6, 2, 60, 0, 6, 16, 62, 0, 6, 13, 66, 0, 6, 3, 71, 1, 15, 6, 72,
    0, 6, 11, 84, 3, 2, 1, 87, 1, 1, 12, 101, 0, 12, 14, 104, 3, 2, 1, 105, 3, 5, 2, 116, 0, 3, 2,
    122, 0, 5, 13, 122, 3, 13, 9, 123, 2, 16, 1, 124, 2, 13, 7, 125, 2, 5, 7, 126, 2, 15, 13, 127,
    0, 9, 15, 127, 2, 9, 4, 128, 2, 1, 4, 129, 2, 11, 4, 130, 2, 1, 10, 131, 2, 10, 10, 132, 2, 7,
    2, 133, 2, 15, 8, 134, 2, 15, 8, 135, 2, 16, 8, 136, 2, 7, 14, 137, 2, 13, 14, 138, 2, 14, 14,
    139, 2, 13, 5, 140, 2, 15, 11, 141, 2, 16, 11, 142, 2, 8, 15, 143, 2, 8, 15, 144, 2, 11, 15,
    145, 2, 16, 15, 1, 0, // 2817
    20, 3, 12, 13, 2, 0, // 2818
    88, 0, 12, 8, 151, 3, 11, 1, 3, 0, // 2819
    44, 1, 12, 6, 47, 1, 3, 6, 187, 2, 8, 2, 3, 0, // 2820
    81, 0, 12, 3, 112, 3, 10, 10, 113, 3, 14, 8, 2, 0, // 2821
    182, 3, 8, 10, 183, 3, 5, 14, 2, 0, // 2822
    118, 1, 5, 3, 148, 1, 7, 1, 1, 0, // 2823
    59, 3, 16, 5, 2, 0, // 2824
    12, 0, 0, 2, 107, 1, 13, 3, 3, 0, // 2825
    40, 1, 6, 6, 50, 1, 9, 6, 176, 1, 2, 13, 4, 0, // 2826
    15, 1, 13, 0, 49, 0, 6, 8, 130, 3, 1, 1, 131, 3, 14, 10, 5, 0, // 2827
    112, 0, 3, 10, 112, 1, 10, 3, 131, 0, 9, 14, 182, 0, 4, 8, 186, 3, 16, 15, 2, 0, // 2828
    150, 1, 4, 1, 164, 1, 7, 7, 4, 0, // 2830
    16, 1, 13, 0, 31, 1, 5, 0, 156, 1, 14, 1, 165, 1, 4, 7, 3, 0, // 2831
    13, 1, 7, 0, 63, 1, 14, 6, 178, 0, 13, 16, 4, 0, // 2832
    65, 1, 5, 6, 179, 1, 14, 13, 188, 2, 2, 8, 189, 2, 11, 14, 3, 0, // 2833
    102, 0, 12, 7, 135, 1, 8, 9, 186, 2, 15, 10, 44, 0, // 2834
    12, 0, 0, 2, 13, 1, 7, 0, 15, 1, 13, 0, 16, 1, 13, 0, 20, 3, 12, 13, 31, 1, 5, 0, 40, 1, 6, 6,
    44, 1, 12, 6, 47, 1, 3, 6, 49, 0, 6, 8, 50, 1, 9, 6, 59, 3, 16, 5, 63, 1, 14, 6, 65, 1, 5, 6,
    81, 0, 12, 3, 88, 0, 12, 8, 102, 0, 12, 7, 107, 1, 13, 3, 112, 0, 3, 10, 112, 3, 10, 10, 112,
    1, 10, 3, 113, 3, 14, 8, 118, 1, 5, 3, 130, 3, 1, 1, 131, 0, 9, 14, 131, 3, 14, 10, 135, 1, 8,
    9, 148, 1, 7, 1, 150, 1, 4, 1, 151, 3, 11, 1, 156, 1, 14, 1, 164, 1, 7, 7, 165, 1, 4, 7, 176,
    1, 2, 13, 178, 0, 13, 16, 179, 1, 14, 13, 182, 0, 4, 8, 182, 3, 8, 10, 183, 3, 5, 14, 186, 3,
    16, 15, 186, 2, 15, 10, 187, 2, 8, 2, 188, 2, 2, 8, 189, 2, 11, 14, 3, 0, // 3074
    17, 1, 4, 0, 151, 1, 10, 1, 157, 3, 8, 2, 3, 0, // 3075
    29, 1, 14, 0, 103, 1, 16, 12, 177, 1, 8, 13, 4, 0, // 3076
    48, 0, 6, 7, 61, 1, 14, 6, 82, 0, 12, 8, 119, 3, 11, 4, 7, 0, // 3077
    19, 0, 0, 15, 58, 0, 6, 8, 97, 1, 14, 12, 119, 1, 11, 3, 129, 0, 9, 13, 150, 0, 1, 10, 173, 0,
    13, 2, 1, 0, // 3078
    192, 0, 8, 14, 5, 0, // 3079
    42, 0, 6, 13, 66, 3, 3, 9, 67, 3, 15, 13, 68, 3, 1, 2, 69, 3, 5, 14, 3, 0, // 3080
    120, 1, 15, 3, 159, 1, 16, 1, 164, 0, 7, 10, 3, 0, // 3081
    26, 0, 0, 14, 41, 1, 6, 6, 117, 1, 5, 3, 3, 0, // 3082
    72, 1, 16, 6, 140, 3, 5, 15, 141, 3, 4, 16, 2, 0, // 3085
    100, 3, 14, 3, 101, 3, 14, 9, 5, 0, // 3087
    52, 1, 1, 6, 110, 1, 4, 3, 169, 0, 7, 5, 189, 0, 10, 15, 193, 3, 11, 15, 3, 0, // 3088
    144, 0, 9, 1, 185, 0, 4, 16, 193, 1, 11, 14, 4, 0, // 3089
    56, 1, 7, 6, 75, 0, 6, 14, 93, 1, 4, 12, 194, 0, 14, 16, 46, 0, // 3090
    17, 1, 4, 0, 19, 0, 0, 15, 26, 0, 0, 14, 29, 1, 14, 0, 41, 1, 6, 6, 42, 0, 6, 13, 48, 0, 6, 7,
    52, 1, 1, 6, 56, 1, 7, 6, 58, 0, 6, 8, 61, 1, 14, 6, 66, 3, 3, 9, 67, 3, 15, 13, 68, 3, 1, 2,
    69, 3, 5, 14, 72, 1, 16, 6, 75, 0, 6, 14, 82, 0, 12, 8, 93, 1, 4, 12, 97, 1, 14, 12, 100, 3,
    14, 3, 101, 3, 14, 9, 103, 1, 16, 12, 110, 1, 4, 3, 117, 1, 5, 3, 119, 3, 11, 4, 119, 1, 11, 3,
    120, 1, 15, 3, 129, 0, 9, 13, 140, 3, 5, 15, 141, 3, 4, 16, 144, 0, 9, 1, 150, 0, 1, 10, 151,
    1, 10, 1, 157, 3, 8, 2, 159, 1, 16, 1, 164, 0, 7, 10, 169, 0, 7, 5, 173, 0, 13, 2, 177, 1, 8,
    13, 185, 0, 4, 16, 189, 0, 10, 15, 192, 0, 8, 14, 193, 3, 11, 15, 193, 1, 11, 14, 194, 0, 14,
    16, 6, 0, // 3329
    3, 3, 5, 0, 4, 3, 16, 12, 5, 3, 9, 9, 6, 3, 6, 7, 7, 3, 3, 14, 8, 3, 5, 16, 3, 0, // 3330
    77, 2, 3, 3, 85, 0, 12, 8, 86, 0, 12, 16, 2, 0, // 3331
    21, 0, 0, 12, 84, 2, 1, 9, 3, 0, // 3332
    0, 1, 0, 0, 81, 2, 10, 3, 98, 2, 5, 14, 3, 0, // 3333
    76, 2, 14, 12, 94, 2, 4, 2, 96, 2, 16, 2, 1, 0, // 3335
    44, 3, 10, 2, 4, 0, // 3336
    78, 2, 3, 3, 89, 2, 3, 13, 91, 2, 7, 4, 102, 2, 10, 16, 6, 0, // 3337
    23, 0, 0, 13, 79, 2, 3, 3, 82, 2, 11, 3, 85, 2, 12, 1, 88, 2, 10, 1, 95, 2, 4, 2, 2,
    0, // 3338
    11, 1, 7, 0, 87, 2, 1, 1, 3, 0, // 3340
    93, 2, 16, 4, 97, 2, 4, 14, 103, 2, 2, 16, 3, 0, // 3341
    4, 0, 0, 16, 21, 1, 2, 0, 76, 3, 4, 14, 2, 0, // 3342
    20, 1, 10, 0, 92, 2, 7, 4, 2, 0, // 3343
    100, 2, 3, 11, 101, 2, 9, 11, 3, 0, // 3344
    80, 2, 13, 3, 83, 2, 15, 3, 99, 2, 15, 5, 2, 0, // 3345
    86, 2, 12, 1, 90, 2, 2, 13, 45, 0, // 3346
    0, 1, 0, 0, 3, 3, 5, 0, 4, 0, 0, 16, 4, 3, 16, 12, 5, 3, 9, 9, 6, 3, 6, 7, 7, 3, 3, 14, 8, 3,
    5, 16, 11, 1, 7, 0, 20, 1, 10, 0, 21, 0, 0, 12, 21, 1, 2, 0, 23, 0, 0, 13, 44, 3, 10, 2, 76, 3,
    4, 14, 76, 2, 14, 12, 77, 2, 3, 3, 78, 2, 3, 3, 79, 2, 3, 3, 80, 2, 13, 3, 81, 2, 10, 3, 82, 2,
    11, 3, 83, 2, 15, 3, 84, 2, 1, 9, 85, 0, 12, 8, 85, 2, 12, 1, 86, 0, 12, 16, 86, 2, 12, 1, 87,
    2, 1, 1, 88, 2, 10, 1, 89, 2, 3, 13, 90, 2, 2, 13, 91, 2, 7, 4, 92, 2, 7, 4, 93, 2, 16, 4, 94,
    2, 4, 2, 95, 2, 4, 2, 96, 2, 16, 2, 97, 2, 4, 14, 98, 2, 5, 14, 99, 2, 15, 5, 100, 2, 3, 11,
    101, 2, 9, 11, 102, 2, 10, 16, 103, 2, 2, 16, 2, 0, // 3585
    15, 3, 10, 9, 16, 3, 10, 13, 1, 0, // 3586
    128, 1, 4, 9, 4, 0, // 3587
    36, 1, 16, 0, 167, 0, 7, 5, 173, 2, 11, 4, 174, 2, 4, 2, 4, 0, // 3588
    46, 0, 6, 5, 80, 0, 12, 15, 107, 3, 10, 7, 108, 3, 15, 13, 2, 0, // 3589
    115, 1, 8, 3, 165, 0, 7, 10, 3, 0, // 3590
    31, 0, 0, 10, 32, 0, 0, 2, 139, 0, 9, 1, 1, 0, // 3591
    57, 3, 15, 1, 4, 0, // 3592
    92, 1, 4, 12, 124, 0, 9, 8, 147, 0, 1, 16, 149, 1, 4, 1, 1, 0, // 3593
    142, 1, 15, 9, 3, 0, // 3594
    62, 1, 14, 6, 122, 1, 9, 5, 126, 3, 4, 15, 3, 0, // 3595
    20, 0, 0, 12, 176, 2, 8, 2, 179, 2, 15, 14, 4, 0, // 3596
    42, 1, 6, 6, 67, 0, 6, 15, 129, 1, 4, 9, 177, 2, 2, 8, 3, 0, // 3597
    23, 1, 8, 0, 89, 3, 7, 3, 90, 3, 16, 2, 4, 0, // 3598
    16, 0, 0, 10, 108, 0, 3, 15, 172, 0, 13, 14, 172, 3, 14, 13, 5, 0, // 3599
    51, 1, 1, 6, 137, 0, 9, 5, 156, 0, 1, 10, 172, 2, 13, 13, 175, 2, 4, 2, 2, 0, // 3600
    180, 2, 4, 5, 181, 2, 2, 5, 1, 0, // 3601
    178, 2, 10, 14, 47, 0, // 3602
    15, 3, 10, 9, 16, 0, 0, 10, 16, 3, 10, 13, 20, 0, 0, 12, 23, 1, 8, 0, 31, 0, 0, 10, 32, 0, 0,
    2, 36, 1, 16, 0, 42, 1, 6, 6, 46, 0, 6, 5, 51, 1, 1, 6, 57, 3, 15, 1, 62, 1, 14, 6, 67, 0, 6,
    15, 80, 0, 12, 15, 89, 3, 7, 3, 90, 3, 16, 2, 92, 1, 4, 12, 107, 3, 10, 7, 108, 0, 3, 15, 108,
    3, 15, 13, 115, 1, 8, 3, 122, 1, 9, 5, 124, 0, 9, 8, 126, 3, 4, 15, 128, 1, 4, 9, 129, 1, 4, 9,
    137, 0, 9, 5, 139, 0, 9, 1, 142, 1, 15, 9, 147, 0, 1, 16, 149, 1, 4, 1, 156, 0, 1, 10, 165, 0,
    7, 10, 167, 0, 7, 5, 172, 0, 13, 14, 172, 3, 14, 13, 172, 2, 13, 13, 173, 2, 11, 4, 174, 2, 4,
    2, 175, 2, 4, 2, 176, 2, 8, 2, 177, 2, 2, 8, 178, 2, 10, 14, 179, 2, 15, 14, 180, 2, 4, 5, 181,
    2, 2, 5, 3, 0, // 3841
    27, 3, 15, 0, 28, 3, 16, 3, 29, 3, 11, 2, 4, 0, // 3842
    51, 0, 6, 13, 52, 0, 6, 11, 153, 1, 8, 1, 156, 3, 10, 13, 3, 0, // 3844
    70, 1, 15, 6, 100, 1, 11, 12, 116, 3, 2, 9, 3, 0, // 3845
    18, 0, 0, 1, 110, 0, 3, 11, 175, 1, 2, 13, 3, 0, // 3847
    61, 3, 11, 3, 62, 3, 13, 9, 63, 3, 10, 14, 4, 0, // 3848
    13, 0, 0, 10, 163, 0, 7, 4, 168, 3, 16, 4, 169, 3, 5, 11, 2, 0, // 3849
    113, 1, 10, 3, 170, 1, 5, 7, 4, 0, // 3850
    101, 1, 11, 12, 136, 3, 16, 7, 137, 3, 5, 13, 138, 3, 14, 14, 3, 0, // 3851
    131, 1, 10, 9, 183, 0, 4, 5, 189, 3, 15, 11, 5, 0, // 3852
    26, 1, 8, 0, 69, 0, 6, 5, 75, 1, 16, 6, 192, 1, 5, 8, 193, 2, 15, 11, 4, 0, // 3853
    7, 0, 0, 3, 76, 0, 12, 4, 97, 3, 11, 4, 98, 3, 3, 5, 3, 0, // 3854
    172, 1, 13, 13, 178, 3, 16, 10, 179, 3, 10, 15, 3, 0, // 3855
    63, 0, 6, 10, 138, 0, 9, 14, 138, 1, 14, 9, 3, 0, // 3857
    37, 0, 0, 5, 184, 1, 2, 4, 194, 2, 11, 16, 47, 0, // 3858
    7, 0, 0, 3, 13, 0, 0, 10, 18, 0, 0, 1, 26, 1, 8, 0, 27, 3, 15, 0, 28, 3, 16, 3, 29, 3, 11, 2,
    37, 0, 0, 5, 51, 0, 6, 13, 52, 0, 6, 11, 61, 3, 11, 3, 62, 3, 13, 9, 63, 0, 6, 10, 63, 3, 10,
    14, 69, 0, 6, 5, 70, 1, 15, 6, 75, 1, 16, 6, 76, 0, 12, 4, 97, 3, 11, 4, 98, 3, 3, 5, 100, 1,
    11, 12, 101, 1, 11, 12, 110, 0, 3, 11, 113, 1, 10, 3, 116, 3, 2, 9, 131, 1, 10, 9, 136, 3, 16,
    7, 137, 3, 5, 13, 138, 0, 9, 14, 138, 3, 14, 14, 138, 1, 14, 9, 153, 1, 8, 1, 156, 3, 10, 13,
    163, 0, 7, 4, 168, 3, 16, 4, 169, 3, 5, 11, 170, 1, 5, 7, 172, 1, 13, 13, 175, 1, 2, 13, 178,
    3, 16, 10, 179, 3, 10, 15, 183, 0, 4, 5, 184, 1, 2, 4, 189, 3, 15, 11, 192, 1, 5, 8, 193, 2,
    15, 11, 194, 2, 11, 16, 2, 0, // 4097
    27, 1, 14, 0, 35, 3, 1, 15, 2, 0, // 4098
    57, 1, 13, 6, 158, 3, 2, 7, 3, 0, // 4099
    33, 1, 5, 0, 181, 1, 5, 13, 188, 1, 8, 10, 3, 0, // 4100
    83, 0, 12, 15, 120, 3, 11, 7, 121, 0, 7, 4, 4, 0, // 4101
    111, 0, 3, 7, 114, 1, 2, 3, 180, 1, 5, 13, 185, 3, 16, 11, 2, 0, // 4102
    65, 0, 6, 10, 99, 0, 12, 15, 2, 0, // 4103
    70, 3, 14, 3, 71, 3, 9, 8, 3, 0, // 4105
    133, 0, 9, 7, 134, 0, 9, 16, 171, 1, 5, 7, 5, 0, // 4106
    127, 1, 4, 9, 142, 3, 13, 8, 143, 3, 5, 8, 144, 3, 1, 11, 145, 3, 15, 16, 1, 0, // 4107
    186, 0, 10, 16, 4, 0, // 4108
    19, 1, 4, 0, 140, 0, 9, 5, 189, 1, 14, 10, 193, 0, 14, 11, 4, 0, // 4110
    67, 1, 11, 6, 80, 1, 3, 12, 108, 1, 13, 3, 126, 0, 9, 4, 1, 0, // 4111
    179, 0, 13, 10, 4, 0, // 4112
    35, 0, 0, 1, 83, 1, 3, 12, 99, 1, 5, 12, 160, 1, 16, 1, 7, 0, // 4113
    38, 0, 0, 3, 39, 1, 16, 0, 145, 1, 15, 9, 155, 1, 8, 1, 160, 0, 1, 15, 161, 0, 1, 16, 195, 0,
    5, 16, 47, 0, // 4114
    19, 1, 4, 0, 27, 1, 14, 0, 33, 1, 5, 0, 35, 0, 0, 1, 35, 3, 1, 15, 38, 0, 0, 3, 39, 1, 16, 0,
    57, 1, 13, 6, 65, 0, 6, 10, 67, 1, 11, 6, 70, 3, 14, 3, 71, 3, 9, 8, 80, 1, 3, 12, 83, 0, 12,
    15, 83, 1, 3, 12, 99, 0, 12, 15, 99, 1, 5, 12, 108, 1, 13, 3, 111, 0, 3, 7, 114, 1, 2, 3, 120,
    3, 11, 7, 121, 0, 7, 4, 126, 0, 9, 4, 127, 1, 4, 9, 133, 0, 9, 7, 134, 0, 9, 16, 140, 0, 9, 5,
    142, 3, 13, 8, 143, 3, 5, 8, 144, 3, 1, 11, 145, 3, 15, 16, 145, 1, 15, 9, 155, 1, 8, 1, 158,
    3, 2, 7, 160, 0, 1, 15, 160, 1, 16, 1, 161, 0, 1, 16, 171, 1, 5, 7, 179, 0, 13, 10, 180, 1, 5,
    13, 181, 1, 5, 13, 185, 3, 16, 11, 186, 0, 10, 16, 188, 1, 8, 10, 189, 1, 14, 10, 193, 0, 14,
    11, 195, 0, 5, 16, 4, 0, // 4353
    36, 3, 13, 2, 37, 3, 5, 14, 38, 3, 3, 15, 39, 3, 15, 16, 5, 0, // 4354
    123, 0, 9, 4, 146, 0, 1, 4, 159, 3, 11, 7, 160, 3, 15, 15, 161, 3, 16, 15, 5, 0, // 4355
    90, 1, 13, 12, 96, 0, 12, 4, 154, 1, 8, 1, 184, 0, 4, 14, 190, 0, 2, 5, 1, 0, // 4356
    28, 1, 14, 0, 2, 0, // 4357
    93, 0, 12, 11, 168, 1, 14, 7, 2, 0, // 4358
    59, 1, 10, 6, 195, 3, 16, 15, 5, 0, // 4359
    43, 0, 6, 3, 72, 3, 11, 9, 73, 3, 2, 7, 74, 3, 1, 2, 75, 3, 14, 11, 2, 0, // 4360
    56, 0, 6, 11, 136, 1, 14, 9, 4, 0, // 4361
    64, 1, 5, 6, 135, 0, 9, 10, 155, 0, 1, 15, 162, 1, 7, 7, 1, 0, // 4362
    60, 1, 2, 6, 1, 0, // 4363
    178, 1, 14, 13, 3, 0, // 4364
    141, 0, 9, 4, 185, 1, 15, 4, 194, 1, 16, 14, 5, 0, // 4365
    4, 1, 12, 0, 8, 0, 0, 5, 86, 1, 1, 12, 102, 3, 7, 10, 103, 3, 11, 2, 1, 0, // 4366
    147, 1, 7, 1, 1, 0, // 4367
    194, 3, 16, 11, 5, 0, // 4368
    134, 1, 8, 9, 145, 0, 9, 15, 161, 1, 16, 1, 186, 1, 10, 10, 195, 1, 16, 5, 1, 0, // 4369
    39, 0, 0, 15, 48, 0, // 4370
    4, 1, 12, 0, 8, 0, 0, 5, 28, 1, 14, 0, 36, 3, 13, 2, 37, 3, 5, 14, 38, 3, 3, 15, 39, 0, 0, 15,
    39, 3, 15, 16, 43, 0, 6, 3, 56, 0, 6, 11, 59, 1, 10, 6, 60, 1, 2, 6, 64, 1, 5, 6, 72, 3, 11, 9,
    73, 3, 2, 7, 74, 3, 1, 2, 75, 3, 14, 11, 86, 1, 1, 12, 90, 1, 13, 12, 93, 0, 12, 11, 96, 0, 12,
    4, 102, 3, 7, 10, 103, 3, 11, 2, 123, 0, 9, 4, 134, 1, 8, 9, 135, 0, 9, 10, 136, 1, 14, 9, 141,
    0, 9, 4, 145, 0, 9, 15, 146, 0, 1, 4, 147, 1, 7, 1, 154, 1, 8, 1, 155, 0, 1, 15, 159, 3, 11, 7,
    160, 3, 15, 15, 161, 3, 16, 15, 161, 1, 16, 1, 162, 1, 7, 7, 168, 1, 14, 7, 178, 1, 14, 13,
    184, 0, 4, 14, 185, 1, 15, 4, 186, 1, 10, 10, 190, 0, 2, 5, 194, 3, 16, 11, 194, 1, 16, 14,
    195, 3, 16, 15, 195, 1, 16, 5, 44, 0, // 4609
    0, 0, 0, 12, 0, 3, 12, 3, 1, 0, 0, 4, 1, 3, 4, 9, 2, 3, 1, 6, 3, 3, 5, 0, 3, 1, 12, 0, 4, 3,
    16, 12, 5, 3, 9, 9, 6, 3, 6, 7, 7, 3, 3, 14, 8, 3, 5, 16, 9, 3, 4, 1, 10, 3, 8, 4, 11, 3, 12,
    9, 12, 3, 2, 10, 13, 3, 10, 14, 14, 3, 1, 5, 15, 3, 10, 9, 16, 3, 10, 13, 17, 3, 11, 1, 18, 3,
    1, 14, 19, 3, 15, 11, 20, 3, 12, 13, 21, 3, 12, 12, 22, 3, 2, 7, 23, 3, 13, 12, 24, 3, 9, 3,
    25, 3, 8, 5, 26, 3, 14, 11, 27, 3, 15, 0, 27, 1, 14, 0, 28, 3, 16, 3, 29, 3, 11, 2, 30, 3, 8,
    7, 31, 3, 10, 13, 32, 3, 2, 13, 33, 3, 15, 2, 34, 3, 9, 8, 35, 3, 1, 15, 36, 3, 13, 2, 37, 3,
    5, 14, 38, 3, 3, 15, 39, 3, 15, 16, 49, 0, // 4610
    2, 2, 6, 6, 9, 0, 0, 4, 9, 1, 1, 0, 10, 0, 0, 8, 14, 2, 5, 7, 17, 1, 4, 0, 18, 2, 14, 4, 35, 2,
    15, 15, 45, 1, 3, 6, 51, 0, 6, 13, 52, 0, 6, 11, 57, 1, 13, 6, 68, 2, 2, 11, 74, 2, 2, 16, 77,
    2, 3, 3, 84, 1, 9, 12, 85, 0, 12, 8, 86, 0, 12, 16, 87, 0, 12, 9, 87, 1, 1, 12, 88, 0, 12, 8,
    104, 1, 9, 3, 106, 2, 1, 7, 106, 1, 7, 3, 123, 0, 9, 4, 128, 1, 4, 9, 130, 2, 1, 10, 130, 1,
    10, 9, 139, 2, 13, 5, 144, 2, 11, 15, 146, 0, 1, 4, 146, 3, 4, 16, 147, 3, 16, 13, 148, 3, 10,
    5, 149, 3, 13, 7, 150, 3, 10, 11, 151, 3, 11, 1, 151, 1, 10, 1, 152, 3, 5, 5, 153, 3, 14, 1,
    153, 1, 8, 1, 154, 3, 16, 2, 155, 3, 15, 16, 156, 3, 10, 13, 157, 3, 8, 2, 158, 3, 2, 7, 159,
    3, 11, 7, 160, 3, 15, 15, 161, 3, 16, 15, 46, 0, // 4611
    12, 2, 10, 7, 21, 0, 0, 12, 22, 0, 0, 2, 22, 2, 7, 2, 29, 1, 14, 0, 32, 2, 13, 5, 33, 1, 5, 0,
    36, 1, 16, 0, 44, 1, 12, 6, 47, 1, 3, 6, 53, 2, 9, 7, 60, 0, 6, 16, 68, 1, 11, 6, 73, 2, 7, 16,
    74, 1, 16, 6, 84, 2, 1, 9, 90, 1, 13, 12, 94, 0, 12, 4, 95, 0, 12, 8, 96, 0, 12, 4, 103, 1, 16,
    12, 104, 2, 1, 9, 105, 1, 9, 3, 109, 1, 4, 3, 114, 0, 3, 15, 116, 2, 9, 14, 132, 0, 9, 4, 152,
    0, 1, 5, 154, 1, 8, 1, 157, 1, 11, 1, 158, 2, 7, 15, 167, 0, 7, 5, 173, 2, 11, 4, 174, 0, 13,
    2, 174, 2, 4, 2, 175, 0, 13, 14, 176, 0, 13, 10, 177, 1, 8, 13, 181, 1, 5, 13, 184, 0, 4, 14,
    187, 0, 10, 2, 187, 2, 8, 2, 188, 1, 8, 10, 190, 0, 2, 5, 190, 3, 5, 16, 191, 3, 8, 5, 45,
    0, // 4612
    0, 1, 0, 0, 7, 2, 14, 12, 24, 1, 8, 0, 28, 1, 14, 0, 38, 2, 15, 16, 43, 2, 16, 6, 45, 0, 6, 5,
    46, 0, 6, 5, 47, 0, 6, 10, 48, 0, 6, 7, 61, 1, 14, 6, 66, 2, 9, 11, 70, 1, 15, 6, 77, 0, 12, 1,
    77, 1, 3, 12, 78, 0, 12, 7, 78, 1, 3, 12, 79, 0, 12, 8, 79, 1, 3, 12, 80, 0, 12, 15, 81, 0, 12,
    3, 81, 2, 10, 3, 82, 0, 12, 8, 83, 0, 12, 15, 89, 1, 13, 12, 98, 2, 5, 14, 100, 1, 11, 12, 104,
    3, 2, 1, 105, 3, 5, 2, 106, 3, 1, 1, 107, 3, 10, 7, 108, 3, 15, 13, 109, 3, 5, 2, 110, 3, 11,
    14, 111, 3, 7, 15, 112, 3, 10, 10, 113, 3, 14, 8, 114, 3, 15, 4, 115, 3, 13, 4, 116, 3, 2, 9,
    117, 3, 11, 8, 118, 3, 10, 5, 119, 3, 11, 4, 120, 3, 11, 7, 121, 0, 7, 4, 46, 0, // 4613
    1, 2, 9, 0, 9, 2, 1, 1, 10, 1, 1, 0, 17, 0, 0, 11, 18, 0, 0, 1, 19, 0, 0, 15, 54, 1, 7, 6, 58,
    0, 6, 8, 76, 2, 14, 12, 91, 0, 12, 7, 92, 0, 12, 13, 93, 0, 12, 11, 94, 2, 4, 2, 94, 1, 2, 12,
    95, 1, 2, 12, 96, 2, 16, 2, 97, 1, 14, 12, 109, 0, 3, 5, 110, 0, 3, 11, 111, 0, 3, 7, 114, 1,
    2, 3, 115, 1, 8, 3, 119, 1, 11, 3, 121, 2, 15, 3, 123, 2, 16, 1, 126, 2, 15, 13, 127, 0, 9, 15,
    128, 0, 9, 13, 129, 0, 9, 13, 132, 2, 7, 2, 141, 2, 16, 11, 146, 2, 16, 1, 149, 0, 1, 13, 150,
    0, 1, 10, 163, 2, 14, 7, 165, 0, 7, 10, 166, 0, 7, 8, 168, 1, 14, 7, 173, 0, 13, 2, 174, 1, 2,
    13, 175, 1, 2, 13, 180, 1, 5, 13, 182, 3, 8, 10, 183, 3, 5, 14, 184, 3, 14, 16, 185, 3, 16, 11,
    44, 0, // 4614
    3, 2, 0, 12, 8, 2, 16, 12, 14, 1, 7, 0, 25, 1, 8, 0, 30, 0, 0, 8, 31, 0, 0, 10, 32, 0, 0, 2,
    33, 0, 0, 15, 34, 0, 0, 9, 37, 2, 14, 16, 45, 2, 1, 3, 46, 2, 13, 3, 59, 1, 10, 6, 64, 0, 6,
    16, 65, 0, 6, 10, 69, 2, 14, 11, 98, 1, 14, 12, 99, 0, 12, 15, 105, 2, 2, 9, 109, 2, 2, 4, 117,
    0, 3, 11, 118, 0, 3, 10, 118, 1, 5, 3, 122, 3, 13, 9, 125, 2, 5, 7, 125, 1, 7, 9, 137, 2, 13,
    14, 139, 0, 9, 1, 140, 2, 15, 11, 143, 2, 8, 15, 148, 1, 7, 1, 152, 2, 5, 2, 152, 1, 2, 1, 167,
    2, 13, 2, 169, 2, 11, 14, 170, 0, 7, 14, 171, 0, 7, 15, 180, 0, 13, 15, 181, 0, 13, 15, 183, 2,
    14, 10, 190, 2, 16, 2, 191, 1, 8, 2, 192, 0, 8, 14, 195, 3, 16, 15, 43, 0, // 4615
    2, 0, 0, 1, 2, 1, 6, 0, 6, 2, 7, 12, 40, 0, 6, 10, 40, 3, 10, 8, 41, 0, 6, 11, 41, 3, 11, 8,
    42, 0, 6, 13, 42, 3, 13, 11, 43, 0, 6, 3, 43, 3, 3, 16, 44, 3, 10, 2, 45, 3, 5, 1, 46, 3, 5,
    13, 47, 3, 10, 2, 48, 3, 7, 11, 49, 3, 8, 10, 50, 3, 10, 8, 51, 3, 13, 14, 52, 3, 11, 14, 53,
    3, 2, 9, 54, 3, 8, 4, 55, 3, 7, 8, 56, 3, 11, 16, 57, 3, 15, 1, 58, 3, 8, 11, 59, 3, 16, 5, 60,
    3, 16, 9, 61, 3, 11, 3, 62, 3, 13, 9, 63, 3, 10, 14, 64, 3, 16, 8, 65, 3, 10, 15, 66, 3, 3, 9,
    67, 3, 15, 13, 68, 3, 1, 2, 69, 3, 5, 14, 70, 3, 14, 3, 71, 3, 9, 8, 72, 3, 11, 9, 73, 3, 2, 7,
    74, 3, 1, 2, 75, 3, 14, 11, 48, 0, // 4616
    6, 1, 12, 0, 11, 0, 0, 12, 12, 0, 0, 2, 13, 0, 0, 10, 14, 0, 0, 1, 22, 1, 2, 0, 30, 1, 5, 0,
    48, 2, 11, 3, 53, 0, 6, 2, 54, 0, 6, 8, 55, 0, 6, 7, 55, 2, 8, 7, 56, 0, 6, 11, 73, 1, 16, 6,
    78, 2, 3, 3, 89, 2, 3, 13, 91, 2, 7, 4, 91, 1, 4, 12, 92, 1, 4, 12, 102, 2, 10, 16, 106, 0, 3,
    1, 107, 1, 13, 3, 111, 2, 15, 4, 120, 1, 15, 3, 121, 3, 4, 15, 124, 0, 9, 8, 125, 0, 9, 5, 132,
    1, 2, 9, 133, 2, 15, 8, 136, 1, 14, 9, 147, 0, 1, 16, 148, 0, 1, 10, 149, 1, 4, 1, 158, 1, 15,
    1, 159, 1, 16, 1, 162, 0, 7, 16, 162, 3, 16, 8, 163, 0, 7, 4, 163, 3, 4, 14, 164, 0, 7, 10,
    164, 3, 10, 11, 165, 3, 10, 13, 166, 3, 8, 8, 167, 3, 5, 13, 168, 3, 16, 4, 169, 3, 5, 11, 170,
    3, 14, 8, 171, 3, 15, 8, 48, 0, // 4617
    10, 2, 4, 1, 23, 0, 0, 13, 24, 0, 0, 9, 25, 0, 0, 8, 25, 2, 5, 8, 26, 0, 0, 14, 30, 2, 7, 5,
    34, 1, 5, 0, 40, 1, 6, 6, 41, 1, 6, 6, 49, 2, 10, 9, 50, 1, 9, 6, 54, 2, 4, 7, 55, 1, 7, 6, 58,
    2, 11, 4, 64, 1, 5, 6, 71, 1, 15, 6, 79, 2, 3, 3, 82, 2, 11, 3, 85, 2, 12, 1, 88, 2, 10, 1, 95,
    2, 4, 2, 113, 1, 10, 3, 115, 0, 3, 13, 117, 1, 5, 3, 124, 2, 13, 7, 133, 0, 9, 7, 134, 0, 9,
    16, 135, 0, 9, 10, 142, 1, 15, 9, 143, 1, 15, 9, 153, 0, 1, 14, 154, 0, 1, 16, 155, 0, 1, 15,
    157, 2, 2, 11, 162, 1, 7, 7, 166, 2, 8, 4, 166, 1, 4, 7, 170, 1, 5, 7, 171, 1, 5, 7, 176, 1, 2,
    13, 177, 0, 13, 11, 182, 2, 10, 10, 187, 1, 2, 10, 188, 0, 10, 15, 191, 0, 2, 8, 191, 2, 5, 8,
    192, 3, 14, 11, 47, 0, // 4618
    1, 1, 0, 0, 5, 2, 9, 12, 5, 1, 12, 0, 11, 1, 7, 0, 15, 1, 13, 0, 24, 2, 3, 8, 34, 2, 8, 5, 49,
    0, 6, 8, 50, 0, 6, 10, 53, 1, 7, 6, 60, 1, 2, 6, 62, 1, 14, 6, 66, 1, 11, 6, 71, 2, 8, 15, 72,
    1, 16, 6, 84, 0, 12, 2, 87, 2, 1, 1, 101, 1, 11, 12, 104, 0, 3, 2, 105, 0, 3, 5, 116, 1, 14, 3,
    122, 0, 5, 13, 122, 1, 9, 5, 123, 3, 4, 16, 124, 3, 8, 13, 125, 3, 5, 5, 126, 3, 4, 15, 127, 3,
    15, 9, 127, 1, 4, 9, 128, 3, 13, 1, 129, 3, 13, 11, 130, 3, 1, 1, 131, 3, 14, 10, 132, 3, 4, 7,
    133, 3, 7, 15, 134, 3, 16, 15, 135, 3, 10, 16, 136, 3, 16, 7, 137, 3, 5, 13, 138, 3, 14, 14,
    139, 3, 1, 13, 140, 3, 5, 15, 141, 3, 4, 16, 142, 3, 13, 8, 143, 3, 5, 8, 144, 3, 1, 11, 145,
    3, 15, 16, 44, 0, // 4619
    12, 1, 7, 0, 13, 2, 14, 7, 15, 2, 9, 13, 16, 2, 13, 13, 20, 0, 0, 12, 31, 2, 13, 5, 40, 2, 8,
    6, 44, 2, 2, 12, 47, 2, 2, 3, 49, 1, 9, 6, 50, 2, 8, 9, 59, 0, 6, 16, 63, 2, 14, 14, 65, 2, 15,
    5, 81, 1, 3, 12, 88, 1, 1, 12, 102, 1, 16, 12, 107, 2, 7, 13, 112, 0, 3, 10, 112, 2, 10, 10,
    112, 1, 10, 3, 113, 0, 3, 14, 118, 2, 5, 5, 130, 0, 9, 1, 131, 0, 9, 14, 131, 1, 10, 9, 135, 2,
    16, 8, 148, 2, 5, 7, 150, 2, 11, 4, 151, 0, 1, 11, 156, 2, 13, 14, 164, 2, 11, 7, 165, 2, 13,
    4, 176, 2, 8, 2, 178, 1, 14, 13, 179, 2, 15, 14, 182, 0, 4, 8, 182, 1, 10, 4, 183, 0, 4, 5,
    186, 0, 10, 16, 186, 3, 16, 15, 187, 3, 2, 8, 188, 3, 15, 2, 189, 3, 15, 11, 46,
    0, // 4620
    17, 2, 1, 4, 19, 1, 4, 0, 26, 1, 8, 0, 29, 2, 2, 14, 41, 2, 8, 6, 42, 1, 6, 6, 48, 1, 3, 6, 52,
    2, 14, 1, 56, 2, 16, 7, 58, 1, 4, 6, 61, 2, 3, 14, 66, 0, 6, 3, 67, 0, 6, 15, 68, 0, 6, 1, 69,
    0, 6, 5, 72, 2, 9, 16, 75, 1, 16, 6, 82, 1, 3, 12, 93, 2, 16, 4, 97, 2, 4, 14, 100, 0, 12, 14,
    101, 0, 12, 14, 103, 2, 2, 16, 110, 2, 14, 4, 117, 2, 8, 5, 119, 0, 3, 11, 119, 2, 4, 11, 120,
    2, 7, 15, 129, 1, 4, 9, 140, 0, 9, 5, 141, 0, 9, 4, 144, 1, 15, 9, 150, 1, 4, 1, 151, 2, 1, 10,
    157, 0, 1, 8, 159, 2, 7, 16, 164, 1, 7, 7, 169, 1, 14, 7, 173, 1, 4, 13, 177, 2, 2, 8, 185, 1,
    15, 4, 189, 1, 14, 10, 192, 1, 5, 8, 193, 0, 14, 11, 193, 2, 15, 11, 194, 1, 16, 14, 45,
    0, // 4621
    0, 2, 3, 0, 3, 0, 0, 5, 4, 0, 0, 16, 4, 1, 12, 0, 5, 0, 0, 9, 6, 0, 0, 6, 7, 0, 0, 3, 8, 0, 0,
    5, 11, 2, 9, 7, 20, 2, 13, 10, 21, 2, 12, 2, 21, 1, 2, 0, 23, 1, 8, 0, 44, 0, 6, 10, 76, 0, 12,
    4, 76, 3, 4, 14, 77, 3, 1, 3, 78, 3, 7, 3, 79, 3, 8, 3, 80, 3, 15, 13, 81, 3, 3, 10, 82, 3, 8,
    11, 83, 3, 15, 15, 84, 3, 2, 1, 85, 3, 8, 12, 85, 1, 1, 12, 86, 3, 16, 12, 86, 1, 1, 12, 87, 3,
    9, 1, 88, 3, 8, 10, 89, 3, 7, 3, 90, 3, 16, 2, 91, 3, 7, 7, 92, 3, 13, 7, 93, 3, 11, 16, 94, 3,
    4, 4, 95, 3, 8, 4, 96, 3, 4, 16, 97, 3, 11, 4, 98, 3, 3, 5, 99, 3, 15, 15, 100, 3, 14, 3, 101,
    3, 14, 9, 102, 3, 7, 10, 103, 3, 11, 2, 47, 0, // 4622
    15, 0, 0, 10, 16, 0, 0, 10, 16, 1, 13, 0, 20, 1, 10, 0, 23, 2, 12, 8, 31, 1, 5, 0, 32, 1, 5, 0,
    36, 2, 2, 16, 42, 2, 11, 6, 46, 1, 3, 6, 51, 2, 14, 1, 57, 0, 6, 15, 62, 2, 9, 14, 67, 1, 11,
    6, 80, 1, 3, 12, 89, 0, 12, 7, 90, 0, 12, 16, 92, 2, 7, 4, 107, 0, 3, 10, 108, 0, 3, 15, 108,
    1, 13, 3, 115, 2, 4, 8, 122, 2, 9, 9, 124, 1, 7, 9, 126, 0, 9, 4, 128, 2, 1, 4, 129, 2, 11, 4,
    137, 1, 14, 9, 139, 1, 5, 9, 142, 2, 8, 15, 147, 1, 7, 1, 149, 2, 7, 4, 156, 1, 14, 1, 165, 1,
    4, 7, 167, 1, 2, 7, 172, 0, 13, 14, 172, 3, 14, 13, 172, 1, 13, 13, 173, 3, 2, 11, 174, 3, 2,
    4, 175, 3, 14, 4, 176, 3, 10, 8, 177, 3, 11, 2, 178, 3, 16, 10, 179, 3, 10, 15, 180, 3, 15, 4,
    181, 3, 15, 2, 47, 0, // 4623
    7, 1, 12, 0, 13, 1, 7, 0, 18, 1, 4, 0, 26, 2, 11, 8, 27, 0, 0, 15, 28, 0, 0, 16, 29, 0, 0, 11,
    37, 1, 16, 0, 51, 1, 1, 6, 52, 1, 1, 6, 61, 0, 6, 11, 62, 0, 6, 13, 63, 0, 6, 10, 63, 1, 14, 6,
    69, 1, 11, 6, 70, 2, 3, 15, 75, 2, 11, 16, 76, 1, 12, 12, 97, 0, 12, 11, 98, 0, 12, 3, 100, 2,
    3, 11, 101, 2, 9, 11, 110, 1, 4, 3, 113, 2, 8, 10, 116, 0, 3, 2, 131, 2, 10, 10, 136, 0, 9, 16,
    137, 0, 9, 5, 138, 0, 9, 14, 138, 2, 14, 14, 138, 1, 14, 9, 153, 2, 1, 8, 156, 0, 1, 10, 163,
    1, 7, 7, 168, 0, 7, 16, 169, 0, 7, 5, 170, 2, 8, 5, 172, 2, 13, 13, 175, 2, 4, 2, 178, 0, 13,
    16, 179, 0, 13, 10, 183, 1, 10, 4, 184, 2, 16, 2, 189, 0, 10, 15, 192, 2, 11, 5, 193, 3, 11,
    15, 194, 3, 16, 11, 47, 0, // 4624
    19, 2, 11, 4, 27, 2, 0, 14, 33, 2, 2, 5, 35, 0, 0, 1, 35, 1, 15, 0, 38, 1, 16, 0, 39, 2, 16,
    16, 57, 2, 1, 13, 65, 1, 5, 6, 67, 2, 13, 11, 70, 0, 6, 14, 71, 0, 6, 9, 80, 2, 13, 3, 83, 2,
    15, 3, 83, 1, 3, 12, 99, 2, 15, 5, 99, 1, 5, 12, 108, 2, 13, 13, 111, 1, 4, 3, 114, 2, 4, 2,
    120, 0, 3, 11, 121, 1, 3, 7, 126, 1, 13, 9, 127, 2, 9, 4, 133, 1, 8, 9, 134, 1, 8, 9, 140, 1,
    11, 9, 142, 0, 9, 13, 143, 0, 9, 5, 144, 0, 9, 1, 145, 0, 9, 15, 145, 2, 16, 15, 155, 2, 16, 8,
    158, 0, 1, 2, 160, 2, 15, 16, 160, 1, 16, 1, 161, 1, 16, 1, 171, 2, 8, 5, 179, 1, 14, 13, 180,
    2, 4, 5, 181, 2, 2, 5, 185, 0, 4, 16, 186, 1, 10, 10, 188, 2, 2, 8, 189, 2, 11, 14, 193, 1, 11,
    14, 195, 1, 16, 5, 48, 0, // 4625
    4, 2, 12, 12, 8, 1, 12, 0, 28, 2, 3, 14, 36, 0, 0, 13, 37, 0, 0, 5, 38, 0, 0, 3, 39, 0, 0, 15,
    39, 1, 16, 0, 43, 1, 6, 6, 56, 1, 7, 6, 59, 2, 5, 10, 60, 2, 9, 2, 64, 2, 8, 5, 72, 0, 6, 11,
    73, 0, 6, 2, 74, 0, 6, 1, 75, 0, 6, 14, 86, 2, 12, 1, 90, 2, 2, 13, 93, 1, 4, 12, 96, 1, 2, 12,
    102, 0, 12, 7, 103, 0, 12, 11, 123, 1, 1, 9, 134, 2, 15, 8, 135, 1, 8, 9, 136, 2, 7, 14, 141,
    1, 11, 9, 145, 1, 15, 9, 146, 1, 1, 1, 147, 2, 13, 7, 154, 2, 2, 8, 155, 1, 8, 1, 159, 0, 1,
    11, 160, 0, 1, 15, 161, 0, 1, 16, 161, 2, 15, 16, 162, 2, 8, 7, 168, 2, 4, 14, 178, 2, 10, 14,
    184, 1, 2, 4, 185, 2, 11, 15, 186, 2, 15, 10, 190, 1, 2, 2, 194, 0, 14, 16, 194, 2, 11, 16,
    195, 0, 5, 16, 195, 2, 15, 16, 0, 0, // 4626
    0, 0, 0, 12, 0, 3, 12, 3, 0, 2, 3, 0, 0, 1, 0, 0, 1, 0, 0, 4, 1, 3, 4, 9, 1, 2, 9, 0, 1, 1, 0,
    0, 2, 0, 0, 1, 2, 3, 1, 6, 2, 2, 6, 6, 2, 1, 6, 0, 3, 0, 0, 5, 3, 3, 5, 0, 3, 2, 0, 12, 3, 1,
    12, 0, 4, 0, 0, 16, 4, 3, 16, 12, 4, 2, 12, 12, 4, 1, 12, 0, 5, 0, 0, 9, 5, 3, 9, 9, 5, 2, 9,
    12, 5, 1, 12, 0, 6, 0, 0, 6, 6, 3, 6, 7, 6, 2, 7, 12, 6, 1, 12, 0, 7, 0, 0, 3, 7, 3, 3, 14, 7,
    2, 14, 12, 7, 1, 12, 0, 8, 0, 0, 5, 8, 3, 5, 16, 8, 2, 16, 12, 8, 1, 12, 0, 9, 0, 0, 4, 9, 3,
    4, 1, 9, 2, 1, 1, 9, 1, 1, 0, 10, 0, 0, 8, 10, 3, 8, 4, 10, 2, 4, 1, 10, 1, 1, 0, 11, 0, 0, 12,
    11, 3, 12, 9, 11, 2, 9, 7, 11, 1, 7, 0, 12, 0, 0, 2, 12, 3, 2, 10, 12, 2, 10, 7, 12, 1, 7, 0,
    13, 0, 0, 10, 13, 3, 10, 14, 13, 2, 14, 7, 13, 1, 7, 0, 14, 0, 0, 1, 14, 3, 1, 5, 14, 2, 5, 7,
    14, 1, 7, 0, 15, 0, 0, 10, 15, 3, 10, 9, 15, 2, 9, 13, 15, 1, 13, 0, 16, 0, 0, 10, 16, 3, 10,
    13, 16, 2, 13, 13, 16, 1, 13, 0, 17, 0, 0, 11, 17, 3, 11, 1, 17, 2, 1, 4, 17, 1, 4, 0, 18, 0,
    0, 1, 18, 3, 1, 14, 18, 2, 14, 4, 18, 1, 4, 0, 19, 0, 0, 15, 19, 3, 15, 11, 19, 2, 11, 4, 19,
    1, 4, 0, 20, 0, 0, 12, 20, 3, 12, 13, 20, 2, 13, 10, 20, 1, 10, 0, 21, 0, 0, 12, 21, 3, 12, 12,
    21, 2, 12, 2, 21, 1, 2, 0, 22, 0, 0, 2, 22, 3, 2, 7, 22, 2, 7, 2, 22, 1, 2, 0, 23, 0, 0, 13,
    23, 3, 13, 12, 23, 2, 12, 8, 23, 1, 8, 0, 24, 0, 0, 9, 24, 3, 9, 3, 24, 2, 3, 8, 24, 1, 8, 0,
    25, 0, 0, 8, 25, 3, 8, 5, 25, 2, 5, 8, 25, 1, 8, 0, 26, 0, 0, 14, 26, 3, 14, 11, 26, 2, 11, 8,
    26, 1, 8, 0, 27, 0, 0, 15, 27, 3, 15, 0, 27, 2, 0, 14, 27, 1, 14, 0, 28, 0, 0, 16, 28, 3, 16,
    3, 28, 2, 3, 14, 28, 1, 14, 0, 29, 0, 0, 11, 29, 3, 11, 2, 29, 2, 2, 14, 29, 1, 14, 0, 30, 0,
    0, 8, 30, 3, 8, 7, 30, 2, 7, 5, 30, 1, 5, 0, 31, 0, 0, 10, 31, 3, 10, 13, 31, 2, 13, 5, 31, 1,
    5, 0, 32, 0, 0, 2, 32, 3, 2, 13, 32, 2, 13, 5, 32, 1, 5, 0, 33, 0, 0, 15, 33, 3, 15, 2, 33, 2,
    2, 5, 33, 1, 5, 0, 34, 0, 0, 9, 34, 3, 9, 8, 34, 2, 8, 5, 34, 1, 5, 0, 35, 0, 0, 1, 35, 3, 1,
    15, 35, 2, 15, 15, 35, 1, 15, 0, 36, 0, 0, 13, 36, 3, 13, 2, 36, 2, 2, 16, 36, 1, 16, 0, 37, 0,
    0, 5, 37, 3, 5, 14, 37, 2, 14, 16, 37, 1, 16, 0, 38, 0, 0, 3, 38, 3, 3, 15, 38, 2, 15, 16, 38,
    1, 16, 0, 39, 0, 0, 15, 39, 3, 15, 16, 39, 2, 16, 16, 39, 1, 16, 0, 40, 0, 6, 10, 40, 3, 10, 8,
    40, 2, 8, 6, 40, 1, 6, 6, 41, 0, 6, 11, 41, 3, 11, 8, 41, 2, 8, 6, 41, 1, 6, 6, 42, 0, 6, 13,
    42, 3, 13, 11, 42, 2, 11, 6, 42, 1, 6, 6, 43, 0, 6, 3, 43, 3, 3, 16, 43, 2, 16, 6, 43, 1, 6, 6,
    44, 0, 6, 10, 44, 3, 10, 2, 44, 2, 2, 12, 44, 1, 12, 6, 45, 0, 6, 5, 45, 3, 5, 1, 45, 2, 1, 3,
    45, 1, 3, 6, 46, 0, 6, 5, 46, 3, 5, 13, 46, 2, 13, 3, 46, 1, 3, 6, 47, 0, 6, 10, 47, 3, 10, 2,
    47, 2, 2, 3, 47, 1, 3, 6, 48, 0, 6, 7, 48, 3, 7, 11, 48, 2, 11, 3, 48, 1, 3, 6, 49, 0, 6, 8,
    49, 3, 8, 10, 49, 2, 10, 9, 49, 1, 9, 6, 50, 0, 6, 10, 50, 3, 10, 8, 50, 2, 8, 9, 50, 1, 9, 6,
    51, 0, 6, 13, 51, 3, 13, 14, 51, 2, 14, 1, 51, 1, 1, 6, 52, 0, 6, 11, 52, 3, 11, 14, 52, 2, 14,
    1, 52, 1, 1, 6, 53, 0, 6, 2, 53, 3, 2, 9, 53, 2, 9, 7, 53, 1, 7, 6, 54, 0, 6, 8, 54, 3, 8, 4,
    54, 2, 4, 7, 54, 1, 7, 6, 55, 0, 6, 7, 55, 3, 7, 8, 55, 2, 8, 7, 55, 1, 7, 6, 56, 0, 6, 11, 56,
    3, 11, 16, 56, 2, 16, 7, 56, 1, 7, 6, 57, 0, 6, 15, 57, 3, 15, 1, 57, 2, 1, 13, 57, 1, 13, 6,
    58, 0, 6, 8, 58, 3, 8, 11, 58, 2, 11, 4, 58, 1, 4, 6, 59, 0, 6, 16, 59, 3, 16, 5, 59, 2, 5, 10,
    59, 1, 10, 6, 60, 0, 6, 16, 60, 3, 16, 9, 60, 2, 9, 2, 60, 1, 2, 6, 61, 0, 6, 11, 61, 3, 11, 3,
    61, 2, 3, 14, 61, 1, 14, 6, 62, 0, 6, 13, 62, 3, 13, 9, 62, 2, 9, 14, 62, 1, 14, 6, 63, 0, 6,
    10, 63, 3, 10, 14, 63, 2, 14, 14, 63, 1, 14, 6, 64, 0, 6, 16, 64, 3, 16, 8, 64, 2, 8, 5, 64, 1,
    5, 6, 65, 0, 6, 10, 65, 3, 10, 15, 65, 2, 15, 5, 65, 1, 5, 6, 66, 0, 6, 3, 66, 3, 3, 9, 66, 2,
    9, 11, 66, 1, 11, 6, 67, 0, 6, 15, 67, 3, 15, 13, 67, 2, 13, 11, 67, 1, 11, 6, 68, 0, 6, 1, 68,
    3, 1, 2, 68, 2, 2, 11, 68, 1, 11, 6, 69, 0, 6, 5, 69, 3, 5, 14, 69, 2, 14, 11, 69, 1, 11, 6,
    70, 0, 6, 14, 70, 3, 14, 3, 70, 2, 3, 15, 70, 1, 15, 6, 71, 0, 6, 9, 71, 3, 9, 8, 71, 2, 8, 15,
    71, 1, 15, 6, 72, 0, 6, 11, 72, 3, 11, 9, 72, 2, 9, 16, 72, 1, 16, 6, 73, 0, 6, 2, 73, 3, 2, 7,
    73, 2, 7, 16, 73, 1, 16, 6, 74, 0, 6, 1, 74, 3, 1, 2, 74, 2, 2, 16, 74, 1, 16, 6, 75, 0, 6, 14,
    75, 3, 14, 11, 75, 2, 11, 16, 75, 1, 16, 6, 76, 0, 12, 4, 76, 3, 4, 14, 76, 2, 14, 12, 76, 1,
    12, 12, 77, 0, 12, 1, 77, 3, 1, 3, 77, 2, 3, 3, 77, 1, 3, 12, 78, 0, 12, 7, 78, 3, 7, 3, 78, 2,
    3, 3, 78, 1, 3, 12, 79, 0, 12, 8, 79, 3, 8, 3, 79, 2, 3, 3, 79, 1, 3, 12, 80, 0, 12, 15, 80, 3,
    15, 13, 80, 2, 13, 3, 80, 1, 3, 12, 81, 0, 12, 3, 81, 3, 3, 10, 81, 2, 10, 3, 81, 1, 3, 12, 82,
    0, 12, 8, 82, 3, 8, 11, 82, 2, 11, 3, 82, 1, 3, 12, 83, 0, 12, 15, 83, 3, 15, 15, 83, 2, 15, 3,
    83, 1, 3, 12, 84, 0, 12, 2, 84, 3, 2, 1, 84, 2, 1, 9, 84, 1, 9, 12, 85, 0, 12, 8, 85, 3, 8, 12,
    85, 2, 12, 1, 85, 1, 1, 12, 86, 0, 12, 16, 86, 3, 16, 12, 86, 2, 12, 1, 86, 1, 1, 12, 87, 0,
    12, 9, 87, 3, 9, 1, 87, 2, 1, 1, 87, 1, 1, 12, 88, 0, 12, 8, 88, 3, 8, 10, 88, 2, 10, 1, 88, 1,
    1, 12, 89, 0, 12, 7, 89, 3, 7, 3, 89, 2, 3, 13, 89, 1, 13, 12, 90, 0, 12, 16, 90, 3, 16, 2, 90,
    2, 2, 13, 90, 1, 13, 12, 91, 0, 12, 7, 91, 3, 7, 7, 91, 2, 7, 4, 91, 1, 4, 12, 92, 0, 12, 13,
    92, 3, 13, 7, 92, 2, 7, 4, 92, 1, 4, 12, 93, 0, 12, 11, 93, 3, 11, 16, 93, 2, 16, 4, 93, 1, 4,
    12, 94, 0, 12, 4, 94, 3, 4, 4, 94, 2, 4, 2, 94, 1, 2, 12, 95, 0, 12, 8, 95, 3, 8, 4, 95, 2, 4,
    2, 95, 1, 2, 12, 96, 0, 12, 4, 96, 3, 4, 16, 96, 2, 16, 2, 96, 1, 2, 12, 97, 0, 12, 11, 97, 3,
    11, 4, 97, 2, 4, 14, 97, 1, 14, 12, 98, 0, 12, 3, 98, 3, 3, 5, 98, 2, 5, 14, 98, 1, 14, 12, 99,
    0, 12, 15, 99, 3, 15, 15, 99, 2, 15, 5, 99, 1, 5, 12, 100, 0, 12, 14, 100, 3, 14, 3, 100, 2, 3,
    11, 100, 1, 11, 12, 101, 0, 12, 14, 101, 3, 14, 9, 101, 2, 9, 11, 101, 1, 11, 12, 102, 0, 12,
    7, 102, 3, 7, 10, 102, 2, 10, 16, 102, 1, 16, 12, 103, 0, 12, 11, 103, 3, 11, 2, 103, 2, 2, 16,
    103, 1, 16, 12, 104, 0, 3, 2, 104, 3, 2, 1, 104, 2, 1, 9, 104, 1, 9, 3, 105, 0, 3, 5, 105, 3,
    5, 2, 105, 2, 2, 9, 105, 1, 9, 3, 106, 0, 3, 1, 106, 3, 1, 1, 106, 2, 1, 7, 106, 1, 7, 3, 107,
    0, 3, 10, 107, 3, 10, 7, 107, 2, 7, 13, 107, 1, 13, 3, 108, 0, 3, 15, 108, 3, 15, 13, 108, 2,
    13, 13, 108, 1, 13, 3, 109, 0, 3, 5, 109, 3, 5, 2, 109, 2, 2, 4, 109, 1, 4, 3, 110, 0, 3, 11,
    110, 3, 11, 14, 110, 2, 14, 4, 110, 1, 4, 3, 111, 0, 3, 7, 111, 3, 7, 15, 111, 2, 15, 4, 111,
    1, 4, 3, 112, 0, 3, 10, 112, 3, 10, 10, 112, 2, 10, 10, 112, 1, 10, 3, 113, 0, 3, 14, 113, 3,
    14, 8, 113, 2, 8, 10, 113, 1, 10, 3, 114, 0, 3, 15, 114, 3, 15, 4, 114, 2, 4, 2, 114, 1, 2, 3,
    115, 0, 3, 13, 115, 3, 13, 4, 115, 2, 4, 8, 115, 1, 8, 3, 116, 0, 3, 2, 116, 3, 2, 9, 116, 2,
    9, 14, 116, 1, 14, 3, 117, 0, 3, 11, 117, 3, 11, 8, 117, 2, 8, 5, 117, 1, 5, 3, 118, 0, 3, 10,
    118, 3, 10, 5, 118, 2, 5, 5, 118, 1, 5, 3, 119, 0, 3, 11, 119, 3, 11, 4, 119, 2, 4, 11, 119, 1,
    11, 3, 120, 0, 3, 11, 120, 3, 11, 7, 120, 2, 7, 15, 120, 1, 15, 3, 121, 0, 7, 4, 121, 3, 4, 15,
    121, 2, 15, 3, 121, 1, 3, 7, 122, 0, 5, 13, 122, 3, 13, 9, 122, 2, 9, 9, 122, 1, 9, 5, 123, 0,
    9, 4, 123, 3, 4, 16, 123, 2, 16, 1, 123, 1, 1, 9, 124, 0, 9, 8, 124, 3, 8, 13, 124, 2, 13, 7,
    124, 1, 7, 9, 125, 0, 9, 5, 125, 3, 5, 5, 125, 2, 5, 7, 125, 1, 7, 9, 126, 0, 9, 4, 126, 3, 4,
    15, 126, 2, 15, 13, 126, 1, 13, 9, 127, 0, 9, 15, 127, 3, 15, 9, 127, 2, 9, 4, 127, 1, 4, 9,
    128, 0, 9, 13, 128, 3, 13, 1, 128, 2, 1, 4, 128, 1, 4, 9, 129, 0, 9, 13, 129, 3, 13, 11, 129,
    2, 11, 4, 129, 1, 4, 9, 130, 0, 9, 1, 130, 3, 1, 1, 130, 2, 1, 10, 130, 1, 10, 9, 131, 0, 9,
    14, 131, 3, 14, 10, 131, 2, 10, 10, 131, 1, 10, 9, 132, 0, 9, 4, 132, 3, 4, 7, 132, 2, 7, 2,
    132, 1, 2, 9, 133, 0, 9, 7, 133, 3, 7, 15, 133, 2, 15, 8, 133, 1, 8, 9, 134, 0, 9, 16, 134, 3,
    16, 15, 134, 2, 15, 8, 134, 1, 8, 9, 135, 0, 9, 10, 135, 3, 10, 16, 135, 2, 16, 8, 135, 1, 8,
    9, 136, 0, 9, 16, 136, 3, 16, 7, 136, 2, 7, 14, 136, 1, 14, 9, 137, 0, 9, 5, 137, 3, 5, 13,
    137, 2, 13, 14, 137, 1, 14, 9, 138, 0, 9, 14, 138, 3, 14, 14, 138, 2, 14, 14, 138, 1, 14, 9,
    139, 0, 9, 1, 139, 3, 1, 13, 139, 2, 13, 5, 139, 1, 5, 9, 140, 0, 9, 5, 140, 3, 5, 15, 140, 2,
    15, 11, 140, 1, 11, 9, 141, 0, 9, 4, 141, 3, 4, 16, 141, 2, 16, 11, 141, 1, 11, 9, 142, 0, 9,
    13, 142, 3, 13, 8, 142, 2, 8, 15, 142, 1, 15, 9, 143, 0, 9, 5, 143, 3, 5, 8, 143, 2, 8, 15,
    143, 1, 15, 9, 144, 0, 9, 1, 144, 3, 1, 11, 144, 2, 11, 15, 144, 1, 15, 9, 145, 0, 9, 15, 145,
    3, 15, 16, 145, 2, 16, 15, 145, 1, 15, 9, 146, 0, 1, 4, 146, 3, 4, 16, 146, 2, 16, 1, 146, 1,
    1, 1, 147, 0, 1, 16, 147, 3, 16, 13, 147, 2, 13, 7, 147, 1, 7, 1, 148, 0, 1, 10, 148, 3, 10, 5,
    148, 2, 5, 7, 148, 1, 7, 1, 149, 0, 1, 13, 149, 3, 13, 7, 149, 2, 7, 4, 149, 1, 4, 1, 150, 0,
    1, 10, 150, 3, 10, 11, 150, 2, 11, 4, 150, 1, 4, 1, 151, 0, 1, 11, 151, 3, 11, 1, 151, 2, 1,
    10, 151, 1, 10, 1, 152, 0, 1, 5, 152, 3, 5, 5, 152, 2, 5, 2, 152, 1, 2, 1, 153, 0, 1, 14, 153,
    3, 14, 1, 153, 2, 1, 8, 153, 1, 8, 1, 154, 0, 1, 16, 154, 3, 16, 2, 154, 2, 2, 8, 154, 1, 8, 1,
    155, 0, 1, 15, 155, 3, 15, 16, 155, 2, 16, 8, 155, 1, 8, 1, 156, 0, 1, 10, 156, 3, 10, 13, 156,
    2, 13, 14, 156, 1, 14, 1, 157, 0, 1, 8, 157, 3, 8, 2, 157, 2, 2, 11, 157, 1, 11, 1, 158, 0, 1,
    2, 158, 3, 2, 7, 158, 2, 7, 15, 158, 1, 15, 1, 159, 0, 1, 11, 159, 3, 11, 7, 159, 2, 7, 16,
    159, 1, 16, 1, 160, 0, 1, 15, 160, 3, 15, 15, 160, 2, 15, 16, 160, 1, 16, 1, 161, 0, 1, 16,
    161, 3, 16, 15, 161, 2, 15, 16, 161, 1, 16, 1, 162, 0, 7, 16, 162, 3, 16, 8, 162, 2, 8, 7, 162,
    1, 7, 7, 163, 0, 7, 4, 163, 3, 4, 14, 163, 2, 14, 7, 163, 1, 7, 7, 164, 0, 7, 10, 164, 3, 10,
    11, 164, 2, 11, 7, 164, 1, 7, 7, 165, 0, 7, 10, 165, 3, 10, 13, 165, 2, 13, 4, 165, 1, 4, 7,
    166, 0, 7, 8, 166, 3, 8, 8, 166, 2, 8, 4, 166, 1, 4, 7, 167, 0, 7, 5, 167, 3, 5, 13, 167, 2,
    13, 2, 167, 1, 2, 7, 168, 0, 7, 16, 168, 3, 16, 4, 168, 2, 4, 14, 168, 1, 14, 7, 169, 0, 7, 5,
    169, 3, 5, 11, 169, 2, 11, 14, 169, 1, 14, 7, 170, 0, 7, 14, 170, 3, 14, 8, 170, 2, 8, 5, 170,
    1, 5, 7, 171, 0, 7, 15, 171, 3, 15, 8, 171, 2, 8, 5, 171, 1, 5, 7, 172, 0, 13, 14, 172, 3, 14,
    13, 172, 2, 13, 13, 172, 1, 13, 13, 173, 0, 13, 2, 173, 3, 2, 11, 173, 2, 11, 4, 173, 1, 4, 13,
    174, 0, 13, 2, 174, 3, 2, 4, 174, 2, 4, 2, 174, 1, 2, 13, 175, 0, 13, 14, 175, 3, 14, 4, 175,
    2, 4, 2, 175, 1, 2, 13, 176, 0, 13, 10, 176, 3, 10, 8, 176, 2, 8, 2, 176, 1, 2, 13, 177, 0, 13,
    11, 177, 3, 11, 2, 177, 2, 2, 8, 177, 1, 8, 13, 178, 0, 13, 16, 178, 3, 16, 10, 178, 2, 10, 14,
    178, 1, 14, 13, 179, 0, 13, 10, 179, 3, 10, 15, 179, 2, 15, 14, 179, 1, 14, 13, 180, 0, 13, 15,
    180, 3, 15, 4, 180, 2, 4, 5, 180, 1, 5, 13, 181, 0, 13, 15, 181, 3, 15, 2, 181, 2, 2, 5, 181,
    1, 5, 13, 182, 0, 4, 8, 182, 3, 8, 10, 182, 2, 10, 10, 182, 1, 10, 4, 183, 0, 4, 5, 183, 3, 5,
    14, 183, 2, 14, 10, 183, 1, 10, 4, 184, 0, 4, 14, 184, 3, 14, 16, 184, 2, 16, 2, 184, 1, 2, 4,
    185, 0, 4, 16, 185, 3, 16, 11, 185, 2, 11, 15, 185, 1, 15, 4, 186, 0, 10, 16, 186, 3, 16, 15,
    186, 2, 15, 10, 186, 1, 10, 10, 187, 0, 10, 2, 187, 3, 2, 8, 187, 2, 8, 2, 187, 1, 2, 10, 188,
    0, 10, 15, 188, 3, 15, 2, 188, 2, 2, 8, 188, 1, 8, 10, 189, 0, 10, 15, 189, 3, 15, 11, 189, 2,
    11, 14, 189, 1, 14, 10, 190, 0, 2, 5, 190, 3, 5, 16, 190, 2, 16, 2, 190, 1, 2, 2, 191, 0, 2, 8,
    191, 3, 8, 5, 191, 2, 5, 8, 191, 1, 8, 2, 192, 0, 8, 14, 192, 3, 14, 11, 192, 2, 11, 5, 192, 1,
    5, 8, 193, 0, 14, 11, 193, 3, 11, 15, 193, 2, 15, 11, 193, 1, 11, 14, 194, 0, 14, 16, 194, 3,
    16, 11, 194, 2, 11, 16, 194, 1, 16, 14, 195, 0, 5, 16, 195, 3, 16, 15, 195, 2, 15, 16, 195, 1,
    16, 5,
];
