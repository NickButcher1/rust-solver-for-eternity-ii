use crate::celltype::{
    CellType, CORNER_BOTTOM_LEFT, CORNER_BOTTOM_RIGHT, CORNER_TOP_LEFT, CORNER_TOP_RIGHT,
    EDGE_BOTTOM, EDGE_LEFT, EDGE_RIGHT, EDGE_TOP, MID,
};
use crate::ori::{Ori, ANTICLOCKWISE_90, ANY, BASE, CLOCKWISE_90, HALF};
use crate::Backtracker;

pub const NUM_CELLS: usize = 256;
pub const NUM_TILES: usize = 256;
pub const NUM_ROWS: usize = 16;
pub const NUM_COLS: usize = 16;

// pub const NUM_CORNERS: u32 = 4;
// pub const NUM_EDGES: u32 = 56;
// pub const NUM_MIDS: usize = 196;

// pub const NUM_COLOURS: u32 = 22;
// pub const NUM_COLOURS_INC_GREY: u32 = 23;
// pub const NUM_EDGE_COLOURS: u32 = 5;
// pub const NUM_MID_COLOURS: u32 = 17;

const INVALID_CELL_IDX: u8 = 255;

#[derive(Debug)]
pub struct Cell {
    pub north_idx: u8,
    pub west_idx: u8,
    pub cell_type: CellType,
    pub ori: Ori,
}

pub const FILL_ORDER: [Cell; NUM_CELLS] = [
    Cell {
        north_idx: INVALID_CELL_IDX,
        west_idx: INVALID_CELL_IDX,
        cell_type: CORNER_TOP_LEFT,
        ori: CLOCKWISE_90,
    }, // idx 0
    Cell {
        north_idx: INVALID_CELL_IDX,
        west_idx: 0,
        cell_type: EDGE_TOP,
        ori: HALF,
    }, // idx 1
    Cell {
        north_idx: INVALID_CELL_IDX,
        west_idx: 1,
        cell_type: EDGE_TOP,
        ori: HALF,
    }, // idx 2
    Cell {
        north_idx: INVALID_CELL_IDX,
        west_idx: 2,
        cell_type: EDGE_TOP,
        ori: HALF,
    }, // idx 3
    Cell {
        north_idx: INVALID_CELL_IDX,
        west_idx: 3,
        cell_type: EDGE_TOP,
        ori: HALF,
    }, // idx 4
    Cell {
        north_idx: INVALID_CELL_IDX,
        west_idx: 4,
        cell_type: EDGE_TOP,
        ori: HALF,
    }, // idx 5
    Cell {
        north_idx: INVALID_CELL_IDX,
        west_idx: 5,
        cell_type: EDGE_TOP,
        ori: HALF,
    }, // idx 6
    Cell {
        north_idx: INVALID_CELL_IDX,
        west_idx: 6,
        cell_type: EDGE_TOP,
        ori: HALF,
    }, // idx 7
    Cell {
        north_idx: INVALID_CELL_IDX,
        west_idx: 7,
        cell_type: EDGE_TOP,
        ori: HALF,
    }, // idx 8
    Cell {
        north_idx: INVALID_CELL_IDX,
        west_idx: 8,
        cell_type: EDGE_TOP,
        ori: HALF,
    }, // idx 9
    Cell {
        north_idx: INVALID_CELL_IDX,
        west_idx: 9,
        cell_type: EDGE_TOP,
        ori: HALF,
    }, // idx 10
    Cell {
        north_idx: INVALID_CELL_IDX,
        west_idx: 10,
        cell_type: EDGE_TOP,
        ori: HALF,
    }, // idx 11
    Cell {
        north_idx: INVALID_CELL_IDX,
        west_idx: 11,
        cell_type: EDGE_TOP,
        ori: HALF,
    }, // idx 12
    Cell {
        north_idx: INVALID_CELL_IDX,
        west_idx: 12,
        cell_type: EDGE_TOP,
        ori: HALF,
    }, // idx 13
    Cell {
        north_idx: INVALID_CELL_IDX,
        west_idx: 13,
        cell_type: EDGE_TOP,
        ori: HALF,
    }, // idx 14
    Cell {
        north_idx: INVALID_CELL_IDX,
        west_idx: 14,
        cell_type: CORNER_TOP_RIGHT,
        ori: HALF,
    }, // idx 15
    Cell {
        north_idx: 0,
        west_idx: INVALID_CELL_IDX,
        cell_type: EDGE_LEFT,
        ori: CLOCKWISE_90,
    }, // idx 16
    Cell {
        north_idx: 1,
        west_idx: 16,
        cell_type: MID,
        ori: ANY,
    }, // idx 17
    Cell {
        north_idx: 2,
        west_idx: 17,
        cell_type: MID,
        ori: ANY,
    }, // idx 18
    Cell {
        north_idx: 3,
        west_idx: 18,
        cell_type: MID,
        ori: ANY,
    }, // idx 19
    Cell {
        north_idx: 4,
        west_idx: 19,
        cell_type: MID,
        ori: ANY,
    }, // idx 20
    Cell {
        north_idx: 5,
        west_idx: 20,
        cell_type: MID,
        ori: ANY,
    }, // idx 21
    Cell {
        north_idx: 6,
        west_idx: 21,
        cell_type: MID,
        ori: ANY,
    }, // idx 22
    Cell {
        north_idx: 7,
        west_idx: 22,
        cell_type: MID,
        ori: ANY,
    }, // idx 23
    Cell {
        north_idx: 8,
        west_idx: 23,
        cell_type: MID,
        ori: ANY,
    }, // idx 24
    Cell {
        north_idx: 9,
        west_idx: 24,
        cell_type: MID,
        ori: ANY,
    }, // idx 25
    Cell {
        north_idx: 10,
        west_idx: 25,
        cell_type: MID,
        ori: ANY,
    }, // idx 26
    Cell {
        north_idx: 11,
        west_idx: 26,
        cell_type: MID,
        ori: ANY,
    }, // idx 27
    Cell {
        north_idx: 12,
        west_idx: 27,
        cell_type: MID,
        ori: ANY,
    }, // idx 28
    Cell {
        north_idx: 13,
        west_idx: 28,
        cell_type: MID,
        ori: ANY,
    }, // idx 29
    Cell {
        north_idx: 14,
        west_idx: 29,
        cell_type: MID,
        ori: ANY,
    }, // idx 30
    Cell {
        north_idx: 15,
        west_idx: 30,
        cell_type: EDGE_RIGHT,
        ori: ANTICLOCKWISE_90,
    }, // idx 31
    Cell {
        north_idx: 16,
        west_idx: INVALID_CELL_IDX,
        cell_type: EDGE_LEFT,
        ori: CLOCKWISE_90,
    }, // idx 32
    Cell {
        north_idx: 17,
        west_idx: 32,
        cell_type: MID,
        ori: ANY,
    }, // idx 33
    Cell {
        north_idx: 18,
        west_idx: 33,
        cell_type: MID,
        ori: ANY,
    }, // idx 34
    Cell {
        north_idx: 19,
        west_idx: 34,
        cell_type: MID,
        ori: ANY,
    }, // idx 35
    Cell {
        north_idx: 20,
        west_idx: 35,
        cell_type: MID,
        ori: ANY,
    }, // idx 36
    Cell {
        north_idx: 21,
        west_idx: 36,
        cell_type: MID,
        ori: ANY,
    }, // idx 37
    Cell {
        north_idx: 22,
        west_idx: 37,
        cell_type: MID,
        ori: ANY,
    }, // idx 38
    Cell {
        north_idx: 23,
        west_idx: 38,
        cell_type: MID,
        ori: ANY,
    }, // idx 39
    Cell {
        north_idx: 24,
        west_idx: 39,
        cell_type: MID,
        ori: ANY,
    }, // idx 40
    Cell {
        north_idx: 25,
        west_idx: 40,
        cell_type: MID,
        ori: ANY,
    }, // idx 41
    Cell {
        north_idx: 26,
        west_idx: 41,
        cell_type: MID,
        ori: ANY,
    }, // idx 42
    Cell {
        north_idx: 27,
        west_idx: 42,
        cell_type: MID,
        ori: ANY,
    }, // idx 43
    Cell {
        north_idx: 28,
        west_idx: 43,
        cell_type: MID,
        ori: ANY,
    }, // idx 44
    Cell {
        north_idx: 29,
        west_idx: 44,
        cell_type: MID,
        ori: ANY,
    }, // idx 45
    Cell {
        north_idx: 30,
        west_idx: 45,
        cell_type: MID,
        ori: ANY,
    }, // idx 46
    Cell {
        north_idx: 31,
        west_idx: 46,
        cell_type: EDGE_RIGHT,
        ori: ANTICLOCKWISE_90,
    }, // idx 47
    Cell {
        north_idx: 32,
        west_idx: INVALID_CELL_IDX,
        cell_type: EDGE_LEFT,
        ori: CLOCKWISE_90,
    }, // idx 48
    Cell {
        north_idx: 33,
        west_idx: 48,
        cell_type: MID,
        ori: ANY,
    }, // idx 49
    Cell {
        north_idx: 34,
        west_idx: 49,
        cell_type: MID,
        ori: ANY,
    }, // idx 50
    Cell {
        north_idx: 35,
        west_idx: 50,
        cell_type: MID,
        ori: ANY,
    }, // idx 51
    Cell {
        north_idx: 36,
        west_idx: 51,
        cell_type: MID,
        ori: ANY,
    }, // idx 52
    Cell {
        north_idx: 37,
        west_idx: 52,
        cell_type: MID,
        ori: ANY,
    }, // idx 53
    Cell {
        north_idx: 38,
        west_idx: 53,
        cell_type: MID,
        ori: ANY,
    }, // idx 54
    Cell {
        north_idx: 39,
        west_idx: 54,
        cell_type: MID,
        ori: ANY,
    }, // idx 55
    Cell {
        north_idx: 40,
        west_idx: 55,
        cell_type: MID,
        ori: ANY,
    }, // idx 56
    Cell {
        north_idx: 41,
        west_idx: 56,
        cell_type: MID,
        ori: ANY,
    }, // idx 57
    Cell {
        north_idx: 42,
        west_idx: 57,
        cell_type: MID,
        ori: ANY,
    }, // idx 58
    Cell {
        north_idx: 43,
        west_idx: 58,
        cell_type: MID,
        ori: ANY,
    }, // idx 59
    Cell {
        north_idx: 44,
        west_idx: 59,
        cell_type: MID,
        ori: ANY,
    }, // idx 60
    Cell {
        north_idx: 45,
        west_idx: 60,
        cell_type: MID,
        ori: ANY,
    }, // idx 61
    Cell {
        north_idx: 46,
        west_idx: 61,
        cell_type: MID,
        ori: ANY,
    }, // idx 62
    Cell {
        north_idx: 47,
        west_idx: 62,
        cell_type: EDGE_RIGHT,
        ori: ANTICLOCKWISE_90,
    }, // idx 63
    Cell {
        north_idx: 48,
        west_idx: INVALID_CELL_IDX,
        cell_type: EDGE_LEFT,
        ori: CLOCKWISE_90,
    }, // idx 64
    Cell {
        north_idx: 49,
        west_idx: 64,
        cell_type: MID,
        ori: ANY,
    }, // idx 65
    Cell {
        north_idx: 50,
        west_idx: 65,
        cell_type: MID,
        ori: ANY,
    }, // idx 66
    Cell {
        north_idx: 51,
        west_idx: 66,
        cell_type: MID,
        ori: ANY,
    }, // idx 67
    Cell {
        north_idx: 52,
        west_idx: 67,
        cell_type: MID,
        ori: ANY,
    }, // idx 68
    Cell {
        north_idx: 53,
        west_idx: 68,
        cell_type: MID,
        ori: ANY,
    }, // idx 69
    Cell {
        north_idx: 54,
        west_idx: 69,
        cell_type: MID,
        ori: ANY,
    }, // idx 70
    Cell {
        north_idx: 55,
        west_idx: 70,
        cell_type: MID,
        ori: ANY,
    }, // idx 71
    Cell {
        north_idx: 56,
        west_idx: 71,
        cell_type: MID,
        ori: ANY,
    }, // idx 72
    Cell {
        north_idx: 57,
        west_idx: 72,
        cell_type: MID,
        ori: ANY,
    }, // idx 73
    Cell {
        north_idx: 58,
        west_idx: 73,
        cell_type: MID,
        ori: ANY,
    }, // idx 74
    Cell {
        north_idx: 59,
        west_idx: 74,
        cell_type: MID,
        ori: ANY,
    }, // idx 75
    Cell {
        north_idx: 60,
        west_idx: 75,
        cell_type: MID,
        ori: ANY,
    }, // idx 76
    Cell {
        north_idx: 61,
        west_idx: 76,
        cell_type: MID,
        ori: ANY,
    }, // idx 77
    Cell {
        north_idx: 62,
        west_idx: 77,
        cell_type: MID,
        ori: ANY,
    }, // idx 78
    Cell {
        north_idx: 63,
        west_idx: 78,
        cell_type: EDGE_RIGHT,
        ori: ANTICLOCKWISE_90,
    }, // idx 79
    Cell {
        north_idx: 64,
        west_idx: INVALID_CELL_IDX,
        cell_type: EDGE_LEFT,
        ori: CLOCKWISE_90,
    }, // idx 80
    Cell {
        north_idx: 65,
        west_idx: 80,
        cell_type: MID,
        ori: ANY,
    }, // idx 81
    Cell {
        north_idx: 66,
        west_idx: 81,
        cell_type: MID,
        ori: ANY,
    }, // idx 82
    Cell {
        north_idx: 67,
        west_idx: 82,
        cell_type: MID,
        ori: ANY,
    }, // idx 83
    Cell {
        north_idx: 68,
        west_idx: 83,
        cell_type: MID,
        ori: ANY,
    }, // idx 84
    Cell {
        north_idx: 69,
        west_idx: 84,
        cell_type: MID,
        ori: ANY,
    }, // idx 85
    Cell {
        north_idx: 70,
        west_idx: 85,
        cell_type: MID,
        ori: ANY,
    }, // idx 86
    Cell {
        north_idx: 71,
        west_idx: 86,
        cell_type: MID,
        ori: ANY,
    }, // idx 87
    Cell {
        north_idx: 72,
        west_idx: 87,
        cell_type: MID,
        ori: ANY,
    }, // idx 88
    Cell {
        north_idx: 73,
        west_idx: 88,
        cell_type: MID,
        ori: ANY,
    }, // idx 89
    Cell {
        north_idx: 74,
        west_idx: 89,
        cell_type: MID,
        ori: ANY,
    }, // idx 90
    Cell {
        north_idx: 75,
        west_idx: 90,
        cell_type: MID,
        ori: ANY,
    }, // idx 91
    Cell {
        north_idx: 76,
        west_idx: 91,
        cell_type: MID,
        ori: ANY,
    }, // idx 92
    Cell {
        north_idx: 77,
        west_idx: 92,
        cell_type: MID,
        ori: ANY,
    }, // idx 93
    Cell {
        north_idx: 78,
        west_idx: 93,
        cell_type: MID,
        ori: ANY,
    }, // idx 94
    Cell {
        north_idx: 79,
        west_idx: 94,
        cell_type: EDGE_RIGHT,
        ori: ANTICLOCKWISE_90,
    }, // idx 95
    Cell {
        north_idx: 80,
        west_idx: INVALID_CELL_IDX,
        cell_type: EDGE_LEFT,
        ori: CLOCKWISE_90,
    }, // idx 96
    Cell {
        north_idx: 81,
        west_idx: 96,
        cell_type: MID,
        ori: ANY,
    }, // idx 97
    Cell {
        north_idx: 82,
        west_idx: 97,
        cell_type: MID,
        ori: ANY,
    }, // idx 98
    Cell {
        north_idx: 83,
        west_idx: 98,
        cell_type: MID,
        ori: ANY,
    }, // idx 99
    Cell {
        north_idx: 84,
        west_idx: 99,
        cell_type: MID,
        ori: ANY,
    }, // idx 100
    Cell {
        north_idx: 85,
        west_idx: 100,
        cell_type: MID,
        ori: ANY,
    }, // idx 101
    Cell {
        north_idx: 86,
        west_idx: 101,
        cell_type: MID,
        ori: ANY,
    }, // idx 102
    Cell {
        north_idx: 87,
        west_idx: 102,
        cell_type: MID,
        ori: ANY,
    }, // idx 103
    Cell {
        north_idx: 88,
        west_idx: 103,
        cell_type: MID,
        ori: ANY,
    }, // idx 104
    Cell {
        north_idx: 89,
        west_idx: 104,
        cell_type: MID,
        ori: ANY,
    }, // idx 105
    Cell {
        north_idx: 90,
        west_idx: 105,
        cell_type: MID,
        ori: ANY,
    }, // idx 106
    Cell {
        north_idx: 91,
        west_idx: 106,
        cell_type: MID,
        ori: ANY,
    }, // idx 107
    Cell {
        north_idx: 92,
        west_idx: 107,
        cell_type: MID,
        ori: ANY,
    }, // idx 108
    Cell {
        north_idx: 93,
        west_idx: 108,
        cell_type: MID,
        ori: ANY,
    }, // idx 109
    Cell {
        north_idx: 94,
        west_idx: 109,
        cell_type: MID,
        ori: ANY,
    }, // idx 110
    Cell {
        north_idx: 95,
        west_idx: 110,
        cell_type: EDGE_RIGHT,
        ori: ANTICLOCKWISE_90,
    }, // idx 111
    Cell {
        north_idx: 96,
        west_idx: INVALID_CELL_IDX,
        cell_type: EDGE_LEFT,
        ori: CLOCKWISE_90,
    }, // idx 112
    Cell {
        north_idx: 97,
        west_idx: 112,
        cell_type: MID,
        ori: ANY,
    }, // idx 113
    Cell {
        north_idx: 98,
        west_idx: 113,
        cell_type: MID,
        ori: ANY,
    }, // idx 114
    Cell {
        north_idx: 99,
        west_idx: 114,
        cell_type: MID,
        ori: ANY,
    }, // idx 115
    Cell {
        north_idx: 100,
        west_idx: 115,
        cell_type: MID,
        ori: ANY,
    }, // idx 116
    Cell {
        north_idx: 101,
        west_idx: 116,
        cell_type: MID,
        ori: ANY,
    }, // idx 117
    Cell {
        north_idx: 102,
        west_idx: 117,
        cell_type: MID,
        ori: ANY,
    }, // idx 118
    Cell {
        north_idx: 103,
        west_idx: 118,
        cell_type: MID,
        ori: ANY,
    }, // idx 119
    Cell {
        north_idx: 104,
        west_idx: 119,
        cell_type: MID,
        ori: ANY,
    }, // idx 120
    Cell {
        north_idx: 105,
        west_idx: 120,
        cell_type: MID,
        ori: ANY,
    }, // idx 121
    Cell {
        north_idx: 106,
        west_idx: 121,
        cell_type: MID,
        ori: ANY,
    }, // idx 122
    Cell {
        north_idx: 107,
        west_idx: 122,
        cell_type: MID,
        ori: ANY,
    }, // idx 123
    Cell {
        north_idx: 108,
        west_idx: 123,
        cell_type: MID,
        ori: ANY,
    }, // idx 124
    Cell {
        north_idx: 109,
        west_idx: 124,
        cell_type: MID,
        ori: ANY,
    }, // idx 125
    Cell {
        north_idx: 110,
        west_idx: 125,
        cell_type: MID,
        ori: ANY,
    }, // idx 126
    Cell {
        north_idx: 111,
        west_idx: 126,
        cell_type: EDGE_RIGHT,
        ori: ANTICLOCKWISE_90,
    }, // idx 127
    Cell {
        north_idx: 112,
        west_idx: INVALID_CELL_IDX,
        cell_type: EDGE_LEFT,
        ori: CLOCKWISE_90,
    }, // idx 128
    Cell {
        north_idx: 113,
        west_idx: 128,
        cell_type: MID,
        ori: ANY,
    }, // idx 129
    Cell {
        north_idx: 114,
        west_idx: 129,
        cell_type: MID,
        ori: ANY,
    }, // idx 130
    Cell {
        north_idx: 115,
        west_idx: 130,
        cell_type: MID,
        ori: ANY,
    }, // idx 131
    Cell {
        north_idx: 116,
        west_idx: 131,
        cell_type: MID,
        ori: ANY,
    }, // idx 132
    Cell {
        north_idx: 117,
        west_idx: 132,
        cell_type: MID,
        ori: ANY,
    }, // idx 133
    Cell {
        north_idx: 118,
        west_idx: 133,
        cell_type: MID,
        ori: ANY,
    }, // idx 134
    Cell {
        north_idx: 119,
        west_idx: 134,
        cell_type: MID,
        ori: ANY,
    }, // idx 135
    Cell {
        north_idx: 120,
        west_idx: 135,
        cell_type: MID,
        ori: ANY,
    }, // idx 136
    Cell {
        north_idx: 121,
        west_idx: 136,
        cell_type: MID,
        ori: ANY,
    }, // idx 137
    Cell {
        north_idx: 122,
        west_idx: 137,
        cell_type: MID,
        ori: ANY,
    }, // idx 138
    Cell {
        north_idx: 123,
        west_idx: 138,
        cell_type: MID,
        ori: ANY,
    }, // idx 139
    Cell {
        north_idx: 124,
        west_idx: 139,
        cell_type: MID,
        ori: ANY,
    }, // idx 140
    Cell {
        north_idx: 125,
        west_idx: 140,
        cell_type: MID,
        ori: ANY,
    }, // idx 141
    Cell {
        north_idx: 126,
        west_idx: 141,
        cell_type: MID,
        ori: ANY,
    }, // idx 142
    Cell {
        north_idx: 127,
        west_idx: 142,
        cell_type: EDGE_RIGHT,
        ori: ANTICLOCKWISE_90,
    }, // idx 143
    Cell {
        north_idx: 128,
        west_idx: INVALID_CELL_IDX,
        cell_type: EDGE_LEFT,
        ori: CLOCKWISE_90,
    }, // idx 144
    Cell {
        north_idx: 129,
        west_idx: 144,
        cell_type: MID,
        ori: ANY,
    }, // idx 145
    Cell {
        north_idx: 130,
        west_idx: 145,
        cell_type: MID,
        ori: ANY,
    }, // idx 146
    Cell {
        north_idx: 131,
        west_idx: 146,
        cell_type: MID,
        ori: ANY,
    }, // idx 147
    Cell {
        north_idx: 132,
        west_idx: 147,
        cell_type: MID,
        ori: ANY,
    }, // idx 148
    Cell {
        north_idx: 133,
        west_idx: 148,
        cell_type: MID,
        ori: ANY,
    }, // idx 149
    Cell {
        north_idx: 134,
        west_idx: 149,
        cell_type: MID,
        ori: ANY,
    }, // idx 150
    Cell {
        north_idx: 135,
        west_idx: 150,
        cell_type: MID,
        ori: ANY,
    }, // idx 151
    Cell {
        north_idx: 136,
        west_idx: 151,
        cell_type: MID,
        ori: ANY,
    }, // idx 152
    Cell {
        north_idx: 137,
        west_idx: 152,
        cell_type: MID,
        ori: ANY,
    }, // idx 153
    Cell {
        north_idx: 138,
        west_idx: 153,
        cell_type: MID,
        ori: ANY,
    }, // idx 154
    Cell {
        north_idx: 139,
        west_idx: 154,
        cell_type: MID,
        ori: ANY,
    }, // idx 155
    Cell {
        north_idx: 140,
        west_idx: 155,
        cell_type: MID,
        ori: ANY,
    }, // idx 156
    Cell {
        north_idx: 141,
        west_idx: 156,
        cell_type: MID,
        ori: ANY,
    }, // idx 157
    Cell {
        north_idx: 142,
        west_idx: 157,
        cell_type: MID,
        ori: ANY,
    }, // idx 158
    Cell {
        north_idx: 143,
        west_idx: 158,
        cell_type: EDGE_RIGHT,
        ori: ANTICLOCKWISE_90,
    }, // idx 159
    Cell {
        north_idx: 144,
        west_idx: INVALID_CELL_IDX,
        cell_type: EDGE_LEFT,
        ori: CLOCKWISE_90,
    }, // idx 160
    Cell {
        north_idx: 145,
        west_idx: 160,
        cell_type: MID,
        ori: ANY,
    }, // idx 161
    Cell {
        north_idx: 146,
        west_idx: 161,
        cell_type: MID,
        ori: ANY,
    }, // idx 162
    Cell {
        north_idx: 147,
        west_idx: 162,
        cell_type: MID,
        ori: ANY,
    }, // idx 163
    Cell {
        north_idx: 148,
        west_idx: 163,
        cell_type: MID,
        ori: ANY,
    }, // idx 164
    Cell {
        north_idx: 149,
        west_idx: 164,
        cell_type: MID,
        ori: ANY,
    }, // idx 165
    Cell {
        north_idx: 150,
        west_idx: 165,
        cell_type: MID,
        ori: ANY,
    }, // idx 166
    Cell {
        north_idx: 151,
        west_idx: 166,
        cell_type: MID,
        ori: ANY,
    }, // idx 167
    Cell {
        north_idx: 152,
        west_idx: 167,
        cell_type: MID,
        ori: ANY,
    }, // idx 168
    Cell {
        north_idx: 153,
        west_idx: 168,
        cell_type: MID,
        ori: ANY,
    }, // idx 169
    Cell {
        north_idx: 154,
        west_idx: 169,
        cell_type: MID,
        ori: ANY,
    }, // idx 170
    Cell {
        north_idx: 155,
        west_idx: 170,
        cell_type: MID,
        ori: ANY,
    }, // idx 171
    Cell {
        north_idx: 156,
        west_idx: 171,
        cell_type: MID,
        ori: ANY,
    }, // idx 172
    Cell {
        north_idx: 157,
        west_idx: 172,
        cell_type: MID,
        ori: ANY,
    }, // idx 173
    Cell {
        north_idx: 158,
        west_idx: 173,
        cell_type: MID,
        ori: ANY,
    }, // idx 174
    Cell {
        north_idx: 159,
        west_idx: 174,
        cell_type: EDGE_RIGHT,
        ori: ANTICLOCKWISE_90,
    }, // idx 175
    Cell {
        north_idx: 160,
        west_idx: INVALID_CELL_IDX,
        cell_type: EDGE_LEFT,
        ori: CLOCKWISE_90,
    }, // idx 176
    Cell {
        north_idx: 161,
        west_idx: 176,
        cell_type: MID,
        ori: ANY,
    }, // idx 177
    Cell {
        north_idx: 162,
        west_idx: 177,
        cell_type: MID,
        ori: ANY,
    }, // idx 178
    Cell {
        north_idx: 163,
        west_idx: 178,
        cell_type: MID,
        ori: ANY,
    }, // idx 179
    Cell {
        north_idx: 164,
        west_idx: 179,
        cell_type: MID,
        ori: ANY,
    }, // idx 180
    Cell {
        north_idx: 165,
        west_idx: 180,
        cell_type: MID,
        ori: ANY,
    }, // idx 181
    Cell {
        north_idx: 166,
        west_idx: 181,
        cell_type: MID,
        ori: ANY,
    }, // idx 182
    Cell {
        north_idx: 167,
        west_idx: 182,
        cell_type: MID,
        ori: ANY,
    }, // idx 183
    Cell {
        north_idx: 168,
        west_idx: 183,
        cell_type: MID,
        ori: ANY,
    }, // idx 184
    Cell {
        north_idx: 169,
        west_idx: 184,
        cell_type: MID,
        ori: ANY,
    }, // idx 185
    Cell {
        north_idx: 170,
        west_idx: 185,
        cell_type: MID,
        ori: ANY,
    }, // idx 186
    Cell {
        north_idx: 171,
        west_idx: 186,
        cell_type: MID,
        ori: ANY,
    }, // idx 187
    Cell {
        north_idx: 172,
        west_idx: 187,
        cell_type: MID,
        ori: ANY,
    }, // idx 188
    Cell {
        north_idx: 173,
        west_idx: 188,
        cell_type: MID,
        ori: ANY,
    }, // idx 189
    Cell {
        north_idx: 174,
        west_idx: 189,
        cell_type: MID,
        ori: ANY,
    }, // idx 190
    Cell {
        north_idx: 175,
        west_idx: 190,
        cell_type: EDGE_RIGHT,
        ori: ANTICLOCKWISE_90,
    }, // idx 191
    Cell {
        north_idx: 176,
        west_idx: INVALID_CELL_IDX,
        cell_type: EDGE_LEFT,
        ori: CLOCKWISE_90,
    }, // idx 192
    Cell {
        north_idx: 177,
        west_idx: 192,
        cell_type: MID,
        ori: ANY,
    }, // idx 193
    Cell {
        north_idx: 178,
        west_idx: 193,
        cell_type: MID,
        ori: ANY,
    }, // idx 194
    Cell {
        north_idx: 179,
        west_idx: 194,
        cell_type: MID,
        ori: ANY,
    }, // idx 195
    Cell {
        north_idx: 180,
        west_idx: 195,
        cell_type: MID,
        ori: ANY,
    }, // idx 196
    Cell {
        north_idx: 181,
        west_idx: 196,
        cell_type: MID,
        ori: ANY,
    }, // idx 197
    Cell {
        north_idx: 182,
        west_idx: 197,
        cell_type: MID,
        ori: ANY,
    }, // idx 198
    Cell {
        north_idx: 183,
        west_idx: 198,
        cell_type: MID,
        ori: ANY,
    }, // idx 199
    Cell {
        north_idx: 184,
        west_idx: 199,
        cell_type: MID,
        ori: ANY,
    }, // idx 200
    Cell {
        north_idx: 185,
        west_idx: 200,
        cell_type: MID,
        ori: ANY,
    }, // idx 201
    Cell {
        north_idx: 186,
        west_idx: 201,
        cell_type: MID,
        ori: ANY,
    }, // idx 202
    Cell {
        north_idx: 187,
        west_idx: 202,
        cell_type: MID,
        ori: ANY,
    }, // idx 203
    Cell {
        north_idx: 188,
        west_idx: 203,
        cell_type: MID,
        ori: ANY,
    }, // idx 204
    Cell {
        north_idx: 189,
        west_idx: 204,
        cell_type: MID,
        ori: ANY,
    }, // idx 205
    Cell {
        north_idx: 190,
        west_idx: 205,
        cell_type: MID,
        ori: ANY,
    }, // idx 206
    Cell {
        north_idx: 191,
        west_idx: 206,
        cell_type: EDGE_RIGHT,
        ori: ANTICLOCKWISE_90,
    }, // idx 207
    Cell {
        north_idx: 192,
        west_idx: INVALID_CELL_IDX,
        cell_type: EDGE_LEFT,
        ori: CLOCKWISE_90,
    }, // idx 208
    Cell {
        north_idx: 193,
        west_idx: 208,
        cell_type: MID,
        ori: ANY,
    }, // idx 209
    Cell {
        north_idx: 194,
        west_idx: 209,
        cell_type: MID,
        ori: ANY,
    }, // idx 210
    Cell {
        north_idx: 195,
        west_idx: 210,
        cell_type: MID,
        ori: ANY,
    }, // idx 211
    Cell {
        north_idx: 196,
        west_idx: 211,
        cell_type: MID,
        ori: ANY,
    }, // idx 212
    Cell {
        north_idx: 197,
        west_idx: 212,
        cell_type: MID,
        ori: ANY,
    }, // idx 213
    Cell {
        north_idx: 198,
        west_idx: 213,
        cell_type: MID,
        ori: ANY,
    }, // idx 214
    Cell {
        north_idx: 199,
        west_idx: 214,
        cell_type: MID,
        ori: ANY,
    }, // idx 215
    Cell {
        north_idx: 200,
        west_idx: 215,
        cell_type: MID,
        ori: ANY,
    }, // idx 216
    Cell {
        north_idx: 201,
        west_idx: 216,
        cell_type: MID,
        ori: ANY,
    }, // idx 217
    Cell {
        north_idx: 202,
        west_idx: 217,
        cell_type: MID,
        ori: ANY,
    }, // idx 218
    Cell {
        north_idx: 203,
        west_idx: 218,
        cell_type: MID,
        ori: ANY,
    }, // idx 219
    Cell {
        north_idx: 204,
        west_idx: 219,
        cell_type: MID,
        ori: ANY,
    }, // idx 220
    Cell {
        north_idx: 205,
        west_idx: 220,
        cell_type: MID,
        ori: ANY,
    }, // idx 221
    Cell {
        north_idx: 206,
        west_idx: 221,
        cell_type: MID,
        ori: ANY,
    }, // idx 222
    Cell {
        north_idx: 207,
        west_idx: 222,
        cell_type: EDGE_RIGHT,
        ori: ANTICLOCKWISE_90,
    }, // idx 223
    Cell {
        north_idx: 208,
        west_idx: INVALID_CELL_IDX,
        cell_type: EDGE_LEFT,
        ori: CLOCKWISE_90,
    }, // idx 224
    Cell {
        north_idx: 209,
        west_idx: 224,
        cell_type: MID,
        ori: ANY,
    }, // idx 225
    Cell {
        north_idx: 210,
        west_idx: 225,
        cell_type: MID,
        ori: ANY,
    }, // idx 226
    Cell {
        north_idx: 211,
        west_idx: 226,
        cell_type: MID,
        ori: ANY,
    }, // idx 227
    Cell {
        north_idx: 212,
        west_idx: 227,
        cell_type: MID,
        ori: ANY,
    }, // idx 228
    Cell {
        north_idx: 213,
        west_idx: 228,
        cell_type: MID,
        ori: ANY,
    }, // idx 229
    Cell {
        north_idx: 214,
        west_idx: 229,
        cell_type: MID,
        ori: ANY,
    }, // idx 230
    Cell {
        north_idx: 215,
        west_idx: 230,
        cell_type: MID,
        ori: ANY,
    }, // idx 231
    Cell {
        north_idx: 216,
        west_idx: 231,
        cell_type: MID,
        ori: ANY,
    }, // idx 232
    Cell {
        north_idx: 217,
        west_idx: 232,
        cell_type: MID,
        ori: ANY,
    }, // idx 233
    Cell {
        north_idx: 218,
        west_idx: 233,
        cell_type: MID,
        ori: ANY,
    }, // idx 234
    Cell {
        north_idx: 219,
        west_idx: 234,
        cell_type: MID,
        ori: ANY,
    }, // idx 235
    Cell {
        north_idx: 220,
        west_idx: 235,
        cell_type: MID,
        ori: ANY,
    }, // idx 236
    Cell {
        north_idx: 221,
        west_idx: 236,
        cell_type: MID,
        ori: ANY,
    }, // idx 237
    Cell {
        north_idx: 222,
        west_idx: 237,
        cell_type: MID,
        ori: ANY,
    }, // idx 238
    Cell {
        north_idx: 223,
        west_idx: 238,
        cell_type: EDGE_RIGHT,
        ori: ANTICLOCKWISE_90,
    }, // idx 239
    Cell {
        north_idx: 224,
        west_idx: INVALID_CELL_IDX,
        cell_type: CORNER_BOTTOM_LEFT,
        ori: BASE,
    }, // idx 240
    Cell {
        north_idx: 225,
        west_idx: 240,
        cell_type: EDGE_BOTTOM,
        ori: BASE,
    }, // idx 241
    Cell {
        north_idx: 226,
        west_idx: 241,
        cell_type: EDGE_BOTTOM,
        ori: BASE,
    }, // idx 242
    Cell {
        north_idx: 227,
        west_idx: 242,
        cell_type: EDGE_BOTTOM,
        ori: BASE,
    }, // idx 243
    Cell {
        north_idx: 228,
        west_idx: 243,
        cell_type: EDGE_BOTTOM,
        ori: BASE,
    }, // idx 244
    Cell {
        north_idx: 229,
        west_idx: 244,
        cell_type: EDGE_BOTTOM,
        ori: BASE,
    }, // idx 245
    Cell {
        north_idx: 230,
        west_idx: 245,
        cell_type: EDGE_BOTTOM,
        ori: BASE,
    }, // idx 246
    Cell {
        north_idx: 231,
        west_idx: 246,
        cell_type: EDGE_BOTTOM,
        ori: BASE,
    }, // idx 247
    Cell {
        north_idx: 232,
        west_idx: 247,
        cell_type: EDGE_BOTTOM,
        ori: BASE,
    }, // idx 248
    Cell {
        north_idx: 233,
        west_idx: 248,
        cell_type: EDGE_BOTTOM,
        ori: BASE,
    }, // idx 249
    Cell {
        north_idx: 234,
        west_idx: 249,
        cell_type: EDGE_BOTTOM,
        ori: BASE,
    }, // idx 250
    Cell {
        north_idx: 235,
        west_idx: 250,
        cell_type: EDGE_BOTTOM,
        ori: BASE,
    }, // idx 251
    Cell {
        north_idx: 236,
        west_idx: 251,
        cell_type: EDGE_BOTTOM,
        ori: BASE,
    }, // idx 252
    Cell {
        north_idx: 237,
        west_idx: 252,
        cell_type: EDGE_BOTTOM,
        ori: BASE,
    }, // idx 253
    Cell {
        north_idx: 238,
        west_idx: 253,
        cell_type: EDGE_BOTTOM,
        ori: BASE,
    }, // idx 254
    Cell {
        north_idx: 239,
        west_idx: 254,
        cell_type: CORNER_BOTTOM_RIGHT,
        ori: ANTICLOCKWISE_90,
    }, // idx 255
];

pub const DISPLAY_TO_FILL_ORDER: [i16; NUM_TILES] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73,
    74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97,
    98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116,
    117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135,
    136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154,
    155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173,
    174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192,
    193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211,
    212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227, 228, 229, 230,
    231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249,
    250, 251, 252, 253, 254, 255,
];

pub const TILES: [u8; 1024] = [
    1, 5, 0, 0, // id: 0, tile_type: CORNER
    1, 2, 0, 0, // id: 1, tile_type: CORNER
    3, 5, 0, 0, // id: 2, tile_type: CORNER
    5, 3, 0, 0, // id: 3, tile_type: CORNER
    1, 1, 0, 1, // id: 4, tile_type: EDGE
    7, 3, 0, 1, // id: 5, tile_type: EDGE
    4, 1, 0, 1, // id: 6, tile_type: EDGE
    4, 4, 0, 1, // id: 7, tile_type: EDGE
    8, 5, 0, 1, // id: 8, tile_type: EDGE
    5, 2, 0, 1, // id: 9, tile_type: EDGE
    11, 3, 0, 1, // id: 10, tile_type: EDGE
    6, 2, 0, 1, // id: 11, tile_type: EDGE
    6, 4, 0, 1, // id: 12, tile_type: EDGE
    16, 2, 0, 1, // id: 13, tile_type: EDGE
    7, 1, 0, 3, // id: 14, tile_type: EDGE
    13, 5, 0, 3, // id: 15, tile_type: EDGE
    10, 4, 0, 3, // id: 16, tile_type: EDGE
    14, 4, 0, 3, // id: 17, tile_type: EDGE
    5, 3, 0, 3, // id: 18, tile_type: EDGE
    11, 3, 0, 3, // id: 19, tile_type: EDGE
    3, 2, 0, 3, // id: 20, tile_type: EDGE
    9, 1, 0, 3, // id: 21, tile_type: EDGE
    9, 4, 0, 3, // id: 22, tile_type: EDGE
    15, 1, 0, 3, // id: 23, tile_type: EDGE
    16, 1, 0, 3, // id: 24, tile_type: EDGE
    1, 3, 0, 5, // id: 25, tile_type: EDGE
    1, 5, 0, 5, // id: 26, tile_type: EDGE
    7, 5, 0, 5, // id: 27, tile_type: EDGE
    13, 5, 0, 5, // id: 28, tile_type: EDGE
    5, 4, 0, 5, // id: 29, tile_type: EDGE
    11, 3, 0, 5, // id: 30, tile_type: EDGE
    15, 5, 0, 5, // id: 31, tile_type: EDGE
    6, 3, 0, 5, // id: 32, tile_type: EDGE
    6, 2, 0, 5, // id: 33, tile_type: EDGE
    12, 4, 0, 5, // id: 34, tile_type: EDGE
    17, 2, 0, 5, // id: 35, tile_type: EDGE
    13, 1, 0, 2, // id: 36, tile_type: EDGE
    2, 4, 0, 2, // id: 37, tile_type: EDGE
    8, 4, 0, 2, // id: 38, tile_type: EDGE
    14, 3, 0, 2, // id: 39, tile_type: EDGE
    14, 5, 0, 2, // id: 40, tile_type: EDGE
    11, 1, 0, 2, // id: 41, tile_type: EDGE
    11, 3, 0, 2, // id: 42, tile_type: EDGE
    11, 5, 0, 2, // id: 43, tile_type: EDGE
    3, 1, 0, 2, // id: 44, tile_type: EDGE
    15, 2, 0, 2, // id: 45, tile_type: EDGE
    6, 2, 0, 2, // id: 46, tile_type: EDGE
    12, 2, 0, 2, // id: 47, tile_type: EDGE
    1, 4, 0, 4, // id: 48, tile_type: EDGE
    7, 1, 0, 4, // id: 49, tile_type: EDGE
    7, 3, 0, 4, // id: 50, tile_type: EDGE
    4, 1, 0, 4, // id: 51, tile_type: EDGE
    5, 2, 0, 4, // id: 52, tile_type: EDGE
    3, 2, 0, 4, // id: 53, tile_type: EDGE
    3, 4, 0, 4, // id: 54, tile_type: EDGE
    6, 5, 0, 4, // id: 55, tile_type: EDGE
    12, 1, 0, 4, // id: 56, tile_type: EDGE
    12, 4, 0, 4, // id: 57, tile_type: EDGE
    16, 3, 0, 4, // id: 58, tile_type: EDGE
    17, 5, 0, 4, // id: 59, tile_type: EDGE
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
    pub const ADD_TILE_FUNCTIONS: [fn(&mut Self, usize) -> (); 256] = [
        Backtracker::add_tile_0,
        Backtracker::add_tile_top_edge,
        Backtracker::add_tile_top_edge,
        Backtracker::add_tile_top_edge,
        Backtracker::add_tile_top_edge,
        Backtracker::add_tile_top_edge,
        Backtracker::add_tile_top_edge,
        Backtracker::add_tile_top_edge,
        Backtracker::add_tile_top_edge,
        Backtracker::add_tile_top_edge,
        Backtracker::add_tile_top_edge,
        Backtracker::add_tile_top_edge,
        Backtracker::add_tile_top_edge,
        Backtracker::add_tile_top_edge,
        Backtracker::add_tile_top_edge,
        Backtracker::add_tile_top_right_corner,
        Backtracker::add_tile_left_edge,
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
        Backtracker::add_tile_mid,
        Backtracker::add_tile_right_edge,
        Backtracker::add_tile_left_edge,
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
        Backtracker::add_tile_mid,
        Backtracker::add_tile_right_edge,
        Backtracker::add_tile_left_edge,
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
        Backtracker::add_tile_mid,
        Backtracker::add_tile_right_edge,
        Backtracker::add_tile_left_edge,
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
        Backtracker::add_tile_mid,
        Backtracker::add_tile_right_edge,
        Backtracker::add_tile_left_edge,
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
        Backtracker::add_tile_mid,
        Backtracker::add_tile_right_edge,
        Backtracker::add_tile_left_edge,
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
        Backtracker::add_tile_mid,
        Backtracker::add_tile_right_edge,
        Backtracker::add_tile_left_edge,
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
        Backtracker::add_tile_mid,
        Backtracker::add_tile_right_edge,
        Backtracker::add_tile_left_edge,
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
        Backtracker::add_tile_mid,
        Backtracker::add_tile_right_edge,
        Backtracker::add_tile_left_edge,
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
        Backtracker::add_tile_mid,
        Backtracker::add_tile_right_edge,
        Backtracker::add_tile_left_edge,
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
        Backtracker::add_tile_mid,
        Backtracker::add_tile_right_edge,
        Backtracker::add_tile_left_edge,
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
        Backtracker::add_tile_mid,
        Backtracker::add_tile_right_edge,
        Backtracker::add_tile_left_edge,
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
        Backtracker::add_tile_mid,
        Backtracker::add_tile_right_edge,
        Backtracker::add_tile_left_edge,
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
        Backtracker::add_tile_mid,
        Backtracker::add_tile_right_edge,
        Backtracker::add_tile_left_edge,
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
        Backtracker::add_tile_mid,
        Backtracker::add_tile_right_edge,
        Backtracker::add_tile_bottom_left_corner,
        Backtracker::add_tile_bottom_edge,
        Backtracker::add_tile_bottom_edge,
        Backtracker::add_tile_bottom_edge,
        Backtracker::add_tile_bottom_edge,
        Backtracker::add_tile_bottom_edge,
        Backtracker::add_tile_bottom_edge,
        Backtracker::add_tile_bottom_edge,
        Backtracker::add_tile_bottom_edge,
        Backtracker::add_tile_bottom_edge,
        Backtracker::add_tile_bottom_edge,
        Backtracker::add_tile_bottom_edge,
        Backtracker::add_tile_bottom_edge,
        Backtracker::add_tile_bottom_edge,
        Backtracker::add_tile_bottom_edge,
        Backtracker::add_tile_final,
    ];
}

pub static TOP_LEFT_CORNER_OFFSET: usize = 2;

pub static TOP_RIGHT_CORNER_COLOUR_ARRAY: [u16; 5] = [
    0, 12, // west 2
    0, 0, 18, // west 5
];

pub static BOTTOM_LEFT_CORNER_COLOUR_ARRAY: [u16; 5] = [
    24, // north 1
    0, 30, // north 3
    0, 0,
];

pub static BOTTOM_RIGHT_CORNER_BICOLOUR_ARRAY: [[u16; 5]; 5] = [
    [0, 0, 0, 0, 0],  // north 1
    [36, 0, 0, 0, 0], // north 2
    [0, 0, 0, 0, 0],  // north 3
    [0, 0, 0, 0, 0],  // north 4
    [0, 0, 42, 0, 0], // north 5
];

pub static TOP_EDGES_COLOUR_ARRAY: [u16; 5] = [48, 66, 108, 130, 172];

pub static RIGHT_EDGES_BICOLOUR_ARRAY: [[u16; 17]; 5] = [
    [0, 0, 0, 190, 0, 0, 0, 0, 0, 0, 196, 202, 208, 0, 0, 0, 0], // north 1
    [
        0, 0, 214, 0, 224, 234, 0, 0, 0, 0, 0, 248, 0, 0, 254, 0, 260,
    ], // north 2
    [0, 0, 0, 0, 0, 266, 272, 0, 0, 0, 278, 0, 0, 284, 0, 290, 0], // north 3
    [
        0, 296, 302, 308, 0, 314, 0, 320, 326, 332, 0, 338, 0, 348, 0, 0, 0,
    ], // north 4
    [0, 0, 0, 0, 0, 354, 0, 0, 0, 0, 360, 0, 366, 0, 0, 0, 372], // north 5
];

pub static BOTTOM_EDGES_BICOLOUR_ARRAY: [[u16; 5]; 17] = [
    [0, 0, 0, 0, 0],         // north 1
    [0, 378, 0, 0, 0],       // north 2
    [0, 0, 384, 390, 0],     // north 3
    [400, 0, 0, 406, 0],     // north 4
    [412, 0, 0, 418, 0],     // north 5
    [424, 434, 0, 440, 446], // north 6
    [0, 0, 0, 456, 0],       // north 7
    [0, 462, 0, 0, 0],       // north 8
    [0, 0, 468, 0, 0],       // north 9
    [0, 0, 474, 0, 0],       // north 10
    [0, 480, 0, 0, 0],       // north 11
    [0, 494, 0, 500, 510],   // north 12
    [0, 516, 0, 0, 522],     // north 13
    [0, 528, 534, 0, 0],     // north 14
    [0, 540, 0, 0, 0],       // north 15
    [0, 0, 0, 546, 0],       // north 16
    [0, 0, 0, 552, 558],     // north 17
];

pub static LEFT_EDGES_COLOUR_ARRAY: [u16; 5] = [564, 582, 624, 642, 684];

pub static MIDS_BICOLOUR_ARRAY: [[u16; 17]; 17] = [
    [
        0, 706, 712, 726, 736, 742, 752, 0, 0, 758, 768, 778, 784, 0, 790, 796, 814,
    ], // north 1
    [
        824, 830, 852, 0, 858, 868, 878, 888, 894, 900, 906, 924, 930, 936, 0, 950, 956,
    ], // north 2
    [
        970, 976, 982, 988, 998, 1012, 1026, 1032, 1050, 1064, 1078, 1088, 1094, 1100, 1122, 0,
        1128,
    ], // north 3
    [
        0, 1134, 0, 1140, 0, 1146, 1156, 1170, 0, 0, 0, 1180, 1198, 1216, 1226, 1244, 0,
    ], // north 4
    [
        1258, 1264, 1278, 1292, 0, 1306, 1316, 1322, 1336, 1342, 0, 1356, 1362, 0, 1376, 1398, 1408,
    ], // north 5
    [
        1430, 0, 1444, 0, 0, 1458, 1468, 1478, 1500, 1510, 1516, 1522, 1528, 1534, 1560, 1570, 1576,
    ], // north 6
    [
        0, 0, 1590, 0, 0, 1600, 0, 1606, 1616, 0, 1626, 1636, 0, 1654, 0, 1664, 1670,
    ], // north 7
    [
        1684, 1698, 1708, 1718, 1728, 1750, 1760, 1770, 1788, 1794, 1800, 1814, 1820, 0, 1826,
        1836, 1858,
    ], // north 8
    [
        0, 1872, 1878, 1892, 1902, 1908, 0, 1926, 1932, 1938, 1952, 1962, 0, 1972, 1978, 1984, 0,
    ], // north 9
    [
        0, 1990, 2008, 2014, 2020, 2042, 2064, 2070, 2084, 2090, 2100, 2106, 2112, 2118, 2132,
        2142, 2152,
    ], // north 10
    [
        0, 0, 2166, 2176, 2182, 2188, 2194, 2200, 2206, 2212, 0, 2226, 0, 2236, 2250, 2256, 2266,
    ], // north 11
    [
        0, 2276, 2282, 2296, 2314, 2340, 0, 2346, 0, 2356, 0, 0, 2370, 0, 2380, 2402, 2416,
    ], // north 12
    [
        2430, 0, 0, 0, 2452, 0, 0, 2462, 2476, 2486, 0, 2496, 2510, 2520, 2526, 2536, 2550,
    ], // north 13
    [
        2556, 2566, 2572, 2582, 2596, 2606, 2616, 2622, 0, 2636, 2650, 2656, 2666, 2676, 2694,
        2716, 2726,
    ], // north 14
    [
        2732, 2746, 0, 2760, 2766, 0, 2776, 2786, 2800, 2810, 2828, 2838, 2848, 2862, 2872, 0, 2882,
    ], // north 15
    [
        2896, 2902, 2912, 2922, 2936, 2954, 0, 0, 2964, 2978, 0, 2996, 0, 3014, 0, 3028, 3042,
    ], // north 16
    [
        3068, 3082, 3100, 3122, 3128, 3138, 3148, 3158, 3168, 3182, 3188, 3194, 3208, 3226, 3232,
        3238, 3256,
    ], // north 17
];

pub static BICOLOUR_TILES: [u8; 3622] = [
    // unused
    0, 0, // topLeftCornersWithTwoColours
    2, 0, // 0
    1, 3, 1, 0, 2, 3, 4, 2, // topRightCornersWithTwoColours
    1, 0, // 2
    1, 2, 0, 99, 1, 0, // 5
    2, 2, 2, 99, // bottomLeftCornersWithTwoColours
    1, 0, // 256
    1, 0, 99, 1, 1, 0, // 768
    2, 0, 99, 4, // bottomRightCornersWithTwoColours
    1, 0, // 513
    1, 1, 99, 99, 1, 0, // 1283
    2, 1, 99, 99, // topEdgesWithTwoColours
    4, 0, // 1
    36, 2, 12, 1, 41, 2, 10, 1, 51, 2, 3, 3, 56, 2, 11, 3, 10, 0, // 2
    9, 2, 4, 0, 11, 2, 5, 0, 20, 2, 2, 2, 33, 2, 5, 4, 35, 2, 16, 4, 45, 2, 14, 1, 46, 2, 5, 1, 47,
    2, 11, 1, 52, 2, 4, 3, 53, 2, 2, 3, 5, 0, // 3
    32, 2, 5, 4, 39, 2, 13, 1, 42, 2, 10, 1, 50, 2, 6, 3, 58, 2, 15, 3, 10, 0, // 4
    7, 2, 3, 0, 12, 2, 5, 0, 16, 2, 9, 2, 17, 2, 13, 2, 22, 2, 8, 2, 34, 2, 11, 4, 37, 2, 1, 1, 38,
    2, 7, 1, 54, 2, 2, 3, 57, 2, 11, 3, 4, 0, // 5
    28, 2, 12, 4, 43, 2, 10, 1, 55, 2, 5, 3, 59, 2, 16, 3, // rightEdgesWithTwoColours
    1, 0, // 260
    51, 1, 3, 99, 1, 0, // 267
    41, 1, 1, 99, 1, 0, // 268
    56, 1, 3, 99, 1, 0, // 269
    36, 1, 1, 99, 2, 0, // 515
    20, 1, 2, 99, 53, 1, 3, 99, 2, 0, // 517
    9, 1, 0, 99, 52, 1, 3, 99, 3, 0, // 518
    11, 1, 0, 99, 33, 1, 4, 99, 46, 1, 1, 99, 1, 0, // 524
    47, 1, 1, 99, 1, 0, // 527
    45, 1, 1, 99, 1, 0, // 529
    35, 1, 4, 99, 1, 0, // 774
    32, 1, 4, 99, 1, 0, // 775
    50, 1, 3, 99, 1, 0, // 779
    42, 1, 1, 99, 1, 0, // 782
    39, 1, 1, 99, 1, 0, // 784
    58, 1, 3, 99, 1, 0, // 1026
    37, 1, 1, 99, 1, 0, // 1027
    54, 1, 3, 99, 1, 0, // 1028
    7, 1, 0, 99, 1, 0, // 1030
    12, 1, 0, 99, 1, 0, // 1032
    38, 1, 1, 99, 1, 0, // 1033
    22, 1, 2, 99, 1, 0, // 1034
    16, 1, 2, 99, 2, 0, // 1036
    34, 1, 4, 99, 57, 1, 3, 99, 1, 0, // 1038
    17, 1, 2, 99, 1, 0, // 1286
    55, 1, 3, 99, 1, 0, // 1291
    43, 1, 1, 99, 1, 0, // 1293
    28, 1, 4, 99, 1, 0, // 1297
    59, 1, 3, 99, // bottomEdgesWithTwoColours
    1, 0, // 514
    37, 0, 99, 3, 1, 0, // 771
    20, 0, 99, 1, 2, 0, // 772
    53, 0, 99, 1, 54, 0, 99, 3, 1, 0, // 1025
    7, 0, 99, 3, 1, 0, // 1028
    51, 0, 99, 0, 1, 0, // 1281
    9, 0, 99, 1, 1, 0, // 1284
    52, 0, 99, 1, 2, 0, // 1537
    11, 0, 99, 1, 12, 0, 99, 3, 1, 0, // 1538
    46, 0, 99, 1, 1, 0, // 1540
    55, 0, 99, 4, 2, 0, // 1541
    32, 0, 99, 2, 33, 0, 99, 1, 1, 0, // 1796
    50, 0, 99, 2, 1, 0, // 2050
    38, 0, 99, 3, 1, 0, // 2307
    22, 0, 99, 3, 1, 0, // 2563
    16, 0, 99, 3, 3, 0, // 2818
    41, 0, 99, 0, 42, 0, 99, 2, 43, 0, 99, 4, 1, 0, // 3074
    47, 0, 99, 1, 2, 0, // 3076
    56, 0, 99, 0, 57, 0, 99, 3, 1, 0, // 3077
    34, 0, 99, 3, 1, 0, // 3330
    36, 0, 99, 0, 1, 0, // 3333
    28, 0, 99, 4, 1, 0, // 3586
    39, 0, 99, 2, 1, 0, // 3587
    17, 0, 99, 3, 1, 0, // 3842
    45, 0, 99, 1, 1, 0, // 4100
    58, 0, 99, 2, 1, 0, // 4356
    59, 0, 99, 4, 1, 0, // 4357
    35, 0, 99, 1, // leftEdgesWithTwoColours
    4, 0, // 256
    7, 3, 3, 3, 9, 3, 1, 4, 11, 3, 1, 5, 12, 3, 3, 5, 10, 0, // 512
    36, 3, 0, 12, 37, 3, 3, 1, 38, 3, 3, 7, 39, 3, 2, 13, 41, 3, 0, 10, 42, 3, 2, 10, 43, 3, 4, 10,
    45, 3, 1, 14, 46, 3, 1, 5, 47, 3, 1, 11, 4, 0, // 768
    16, 3, 3, 9, 17, 3, 3, 13, 20, 3, 1, 2, 22, 3, 3, 8, 10, 0, // 1024
    50, 3, 2, 6, 51, 3, 0, 3, 52, 3, 1, 4, 53, 3, 1, 2, 54, 3, 3, 2, 55, 3, 4, 5, 56, 3, 0, 11, 57,
    3, 3, 11, 58, 3, 2, 15, 59, 3, 4, 16, 5, 0, // 1280
    28, 3, 4, 12, 32, 3, 2, 5, 33, 3, 1, 5, 34, 3, 3, 11, 35, 3, 1, 16,
    // midsWithTwoColours
    1, 0, // 258
    74, 2, 5, 7, 3, 0, // 259
    72, 2, 10, 7, 82, 2, 7, 2, 92, 2, 13, 5, 2, 0, // 260
    67, 2, 14, 12, 98, 2, 15, 16, 1, 0, // 261
    69, 2, 1, 1, 2, 0, // 262
    68, 2, 16, 12, 97, 2, 14, 16, 1, 0, // 263
    66, 2, 7, 12, 2, 0, // 266
    65, 2, 9, 12, 94, 2, 8, 5, 2, 0, // 267
    75, 2, 9, 13, 76, 2, 13, 13, 1, 0, // 268
    89, 2, 2, 14, 1, 0, // 269
    71, 2, 9, 7, 1, 0, // 271
    87, 0, 0, 15, 4, 0, // 272
    79, 2, 11, 4, 87, 2, 0, 14, 93, 2, 2, 5, 99, 2, 16, 16, 2, 0, // 273
    64, 2, 12, 12, 88, 2, 3, 14, 1, 0, // 513
    69, 3, 4, 1, 5, 0, // 514
    69, 0, 0, 4, 147, 0, 12, 9, 166, 1, 7, 3, 190, 1, 10, 9, 206, 3, 4, 16, 1, 0, // 515
    218, 2, 7, 15, 2, 0, // 517
    188, 0, 9, 13, 206, 2, 16, 1, 2, 0, // 518
    74, 1, 7, 0, 212, 2, 5, 2, 2, 0, // 519
    111, 3, 13, 14, 112, 3, 11, 14, 1, 0, // 520
    166, 0, 3, 1, 1, 0, // 521
    217, 2, 2, 11, 1, 0, // 522
    183, 3, 4, 16, 4, 0, // 523
    190, 0, 9, 1, 208, 2, 5, 7, 210, 2, 11, 4, 216, 2, 13, 14, 1, 0, // 524
    204, 1, 15, 9, 1, 0, // 525
    147, 3, 9, 1, 3, 0, // 526
    117, 0, 6, 15, 199, 1, 5, 9, 209, 2, 7, 4, 1, 0, // 528
    220, 2, 15, 16, 3, 0, // 529
    207, 2, 13, 7, 214, 2, 2, 8, 221, 2, 15, 16, 1, 0, // 769
    82, 3, 2, 7, 1, 0, // 770
    212, 3, 5, 5, 1, 0, // 771
    250, 3, 5, 16, 2, 0, // 772
    107, 0, 6, 10, 174, 3, 15, 4, 3, 0, // 773
    169, 0, 3, 5, 234, 1, 2, 13, 244, 3, 14, 16, 3, 0, // 774
    93, 0, 0, 15, 241, 0, 13, 15, 250, 2, 16, 2, 1, 0, // 775
    120, 3, 16, 9, 4, 0, // 776
    82, 1, 2, 0, 133, 1, 16, 6, 218, 1, 15, 1, 227, 3, 5, 13, 3, 0, // 777
    214, 0, 1, 16, 237, 0, 13, 11, 247, 1, 2, 10, 3, 0, // 778
    113, 1, 7, 6, 165, 0, 3, 5, 192, 3, 4, 7, 2, 0, // 779
    72, 1, 7, 0, 247, 3, 2, 8, 1, 0, // 780
    217, 0, 1, 8, 1, 0, // 781
    156, 3, 4, 16, 5, 0, // 782
    92, 1, 5, 0, 150, 0, 12, 16, 234, 3, 2, 4, 235, 3, 14, 4, 236, 3, 10, 8, 1, 0, // 783
    89, 0, 0, 11, 1, 0, // 785
    163, 0, 12, 11, 1, 0, // 1026
    166, 2, 1, 7, 1, 0, // 1028
    139, 0, 12, 8, 2, 0, // 1030
    165, 2, 2, 9, 169, 2, 2, 4, 3, 0, // 1031
    106, 3, 5, 13, 107, 3, 10, 2, 108, 3, 7, 11, 2, 0, // 1032
    171, 2, 15, 4, 181, 3, 4, 15, 4, 0, // 1036
    160, 0, 12, 14, 170, 2, 14, 4, 179, 2, 4, 11, 180, 2, 7, 15, 4, 0, // 1037
    139, 3, 8, 3, 140, 3, 15, 13, 142, 3, 8, 11, 143, 3, 15, 15, 2, 0, // 1038
    149, 0, 12, 7, 175, 2, 4, 8, 4, 0, // 1039
    67, 1, 12, 0, 88, 0, 0, 16, 121, 0, 6, 11, 173, 2, 8, 10, 3, 0, // 1040
    98, 1, 16, 0, 168, 2, 13, 13, 174, 2, 4, 2, 1, 0, // 1281
    79, 3, 15, 11, 3, 0, // 1282
    69, 1, 1, 0, 209, 3, 13, 7, 210, 3, 10, 11, 3, 0, // 1283
    174, 0, 3, 15, 234, 0, 13, 2, 235, 0, 13, 14, 3, 0, // 1284
    169, 3, 5, 2, 170, 3, 11, 14, 171, 3, 7, 15, 2, 0, // 1286
    240, 0, 13, 15, 243, 2, 14, 10, 1, 0, // 1287
    118, 3, 8, 11, 3, 0, // 1288
    192, 1, 2, 9, 225, 3, 10, 13, 226, 3, 8, 8, 1, 0, // 1289
    175, 0, 3, 13, 3, 0, // 1290
    187, 3, 15, 9, 188, 3, 13, 1, 189, 3, 13, 11, 1, 0, // 1292
    179, 0, 3, 11, 3, 0, // 1293
    151, 3, 7, 7, 152, 3, 13, 7, 153, 3, 11, 16, 5, 0, // 1295
    136, 1, 12, 12, 157, 0, 12, 11, 223, 1, 7, 7, 228, 0, 7, 16, 244, 2, 16, 2, 2, 0, // 1296
    181, 1, 3, 7, 186, 1, 13, 9, 5, 0, // 1297
    156, 1, 2, 12, 183, 1, 1, 9, 201, 1, 11, 9, 206, 1, 1, 1, 245, 2, 11, 15, 3, 0, // 1537
    92, 3, 2, 13, 93, 3, 15, 2, 94, 3, 9, 8, 3, 0, // 1539
    165, 1, 9, 3, 169, 1, 4, 3, 212, 0, 1, 5, 2, 0, // 1542
    185, 1, 7, 9, 212, 1, 2, 1, 2, 0, // 1543
    124, 3, 16, 8, 125, 3, 10, 15, 5, 0, // 1544
    74, 0, 0, 1, 185, 0, 9, 5, 208, 0, 1, 10, 230, 3, 14, 8, 231, 3, 15, 8, 2, 0, // 1545
    203, 1, 15, 9, 252, 3, 14, 11, 1, 0, // 1546
    199, 3, 1, 13, 1, 0, // 1547
    119, 0, 6, 16, 1, 0, // 1548
    229, 1, 14, 7, 1, 0, // 1549
    159, 3, 15, 15, 6, 0, // 1550
    106, 1, 3, 6, 182, 2, 9, 9, 197, 1, 14, 9, 227, 1, 2, 7, 240, 3, 15, 4, 241, 3, 15, 2, 2,
    0, // 1551
    97, 1, 16, 0, 243, 1, 10, 4, 1, 0, // 1552
    200, 1, 11, 9, 3, 0, // 1553
    68, 1, 12, 0, 250, 1, 2, 2, 255, 2, 15, 16, 2, 0, // 1795
    113, 2, 9, 7, 133, 2, 7, 16, 1, 0, // 1798
    106, 2, 13, 3, 2, 0, // 1800
    66, 1, 12, 0, 108, 2, 11, 3, 2, 0, // 1801
    109, 2, 10, 9, 118, 2, 11, 4, 2, 0, // 1803
    107, 2, 2, 3, 125, 2, 15, 5, 4, 0, // 1804
    112, 2, 14, 1, 116, 2, 16, 7, 121, 2, 3, 14, 132, 2, 9, 16, 2, 0, // 1806
    111, 2, 14, 1, 122, 2, 9, 14, 1, 0, // 1808
    117, 2, 1, 13, 3, 0, // 1809
    119, 2, 5, 10, 120, 2, 9, 2, 124, 2, 8, 5, 3, 0, // 2049
    71, 3, 12, 9, 72, 3, 2, 10, 74, 3, 1, 5, 2, 0, // 2050
    207, 3, 16, 13, 208, 3, 10, 5, 2, 0, // 2051
    82, 0, 0, 2, 192, 0, 9, 4, 2, 0, // 2052
    149, 1, 13, 12, 166, 3, 1, 1, 5, 0, // 2053
    151, 0, 12, 7, 152, 0, 12, 13, 181, 2, 15, 3, 209, 0, 1, 13, 223, 2, 14, 7, 2, 0, // 2054
    227, 2, 13, 2, 229, 2, 11, 14, 2, 0, // 2055
    113, 3, 2, 9, 116, 3, 11, 16, 4, 0, // 2056
    151, 1, 4, 12, 222, 3, 16, 8, 223, 3, 4, 14, 224, 3, 10, 11, 1, 0, // 2057
    226, 2, 8, 4, 1, 0, // 2058
    185, 3, 5, 5, 3, 0, // 2059
    162, 1, 16, 12, 224, 2, 11, 7, 225, 2, 13, 4, 1, 0, // 2060
    108, 1, 3, 6, 1, 0, // 2061
    66, 0, 0, 6, 2, 0, // 2063
    196, 0, 9, 16, 230, 2, 8, 5, 5, 0, // 2064
    171, 1, 4, 3, 180, 0, 3, 11, 193, 1, 8, 9, 218, 0, 1, 2, 231, 2, 8, 5, 3, 0, // 2065
    133, 0, 6, 2, 222, 2, 8, 7, 228, 2, 4, 14, 1, 0, // 2306
    214, 3, 16, 2, 3, 0, // 2307
    217, 1, 11, 1, 236, 0, 13, 10, 247, 0, 10, 2, 2, 0, // 2308
    139, 1, 3, 12, 175, 3, 13, 4, 1, 0, // 2309
    226, 0, 7, 8, 4, 0, // 2310
    94, 0, 0, 9, 124, 0, 6, 16, 230, 0, 7, 14, 231, 0, 7, 15, 1, 0, // 2312
    222, 0, 7, 16, 1, 0, // 2313
    226, 1, 4, 7, 3, 0, // 2314
    193, 3, 7, 15, 194, 3, 16, 15, 195, 3, 10, 16, 2, 0, // 2315
    109, 1, 9, 6, 173, 0, 3, 14, 2, 0, // 2316
    118, 1, 4, 6, 142, 1, 3, 12, 1, 0, // 2318
    237, 3, 11, 2, 1, 0, // 2319
    252, 2, 11, 5, 1, 0, // 2320
    203, 0, 9, 5, 4, 0, // 2562
    147, 1, 1, 12, 190, 2, 1, 10, 199, 2, 13, 5, 204, 2, 11, 15, 1, 0, // 2563
    120, 0, 6, 16, 1, 0, // 2564
    165, 3, 5, 2, 5, 0, // 2565
    183, 2, 16, 1, 186, 2, 15, 13, 187, 0, 9, 15, 192, 2, 7, 2, 201, 2, 16, 11, 5, 0, // 2566
    182, 3, 13, 9, 185, 2, 5, 7, 197, 2, 13, 14, 200, 2, 15, 11, 203, 2, 8, 15, 1, 0, // 2567
    109, 3, 8, 10, 3, 0, // 2568
    71, 0, 0, 12, 113, 0, 6, 2, 193, 2, 15, 8, 1, 0, // 2569
    94, 1, 5, 0, 2, 0, // 2570
    65, 1, 12, 0, 182, 0, 5, 13, 1, 0, // 2571
    195, 2, 16, 8, 1, 0, // 2572
    161, 0, 12, 14, 1, 0, // 2573
    65, 0, 0, 9, 3, 0, // 2574
    75, 0, 0, 10, 188, 2, 1, 4, 189, 2, 11, 4, 2, 0, // 2575
    122, 0, 6, 13, 198, 2, 14, 14, 2, 0, // 2576
    187, 2, 9, 4, 205, 2, 16, 15, 3, 0, // 2577
    132, 0, 6, 11, 194, 2, 15, 8, 196, 2, 7, 14, 2, 0, // 2819
    107, 1, 3, 6, 247, 2, 8, 2, 1, 0, // 2820
    173, 3, 14, 8, 1, 0, // 2821
    243, 3, 5, 14, 1, 0, // 2822
    208, 1, 7, 1, 1, 0, // 2823
    119, 3, 16, 5, 1, 0, // 2824
    72, 0, 0, 2, 1, 0, // 2825
    236, 1, 2, 13, 3, 0, // 2826
    75, 1, 13, 0, 109, 0, 6, 8, 190, 3, 1, 1, 2, 0, // 2828
    210, 1, 4, 1, 224, 1, 7, 7, 3, 0, // 2830
    76, 1, 13, 0, 216, 1, 14, 1, 225, 1, 4, 7, 1, 0, // 2831
    238, 0, 13, 16, 2, 0, // 2832
    125, 1, 5, 6, 249, 2, 11, 14, 2, 0, // 2833
    162, 0, 12, 7, 195, 1, 8, 9, 1, 0, // 3074
    217, 3, 8, 2, 3, 0, // 3075
    89, 1, 14, 0, 163, 1, 16, 12, 237, 1, 8, 13, 4, 0, // 3076
    108, 0, 6, 7, 121, 1, 14, 6, 142, 0, 12, 8, 179, 3, 11, 4, 6, 0, // 3077
    79, 0, 0, 15, 118, 0, 6, 8, 157, 1, 14, 12, 179, 1, 11, 3, 189, 0, 9, 13, 210, 0, 1, 10, 1,
    0, // 3078
    252, 0, 8, 14, 2, 0, // 3080
    180, 1, 15, 3, 224, 0, 7, 10, 3, 0, // 3082
    132, 1, 16, 6, 200, 3, 5, 15, 201, 3, 4, 16, 2, 0, // 3085
    160, 3, 14, 3, 161, 3, 14, 9, 5, 0, // 3087
    112, 1, 1, 6, 170, 1, 4, 3, 229, 0, 7, 5, 249, 0, 10, 15, 253, 3, 11, 15, 3, 0, // 3088
    204, 0, 9, 1, 245, 0, 4, 16, 253, 1, 11, 14, 3, 0, // 3089
    116, 1, 7, 6, 153, 1, 4, 12, 254, 0, 14, 16, 5, 0, // 3329
    64, 3, 16, 12, 65, 3, 9, 9, 66, 3, 6, 7, 67, 3, 3, 14, 68, 3, 5, 16, 2, 0, // 3333
    136, 2, 14, 12, 156, 2, 16, 2, 3, 0, // 3336
    149, 2, 3, 13, 151, 2, 7, 4, 162, 2, 10, 16, 2, 0, // 3337
    139, 2, 3, 3, 142, 2, 11, 3, 2, 0, // 3338
    71, 1, 7, 0, 147, 2, 1, 1, 3, 0, // 3340
    153, 2, 16, 4, 157, 2, 4, 14, 163, 2, 2, 16, 2, 0, // 3341
    64, 0, 0, 16, 136, 3, 4, 14, 1, 0, // 3342
    152, 2, 7, 4, 2, 0, // 3343
    160, 2, 3, 11, 161, 2, 9, 11, 3, 0, // 3344
    140, 2, 13, 3, 143, 2, 15, 3, 159, 2, 15, 5, 1, 0, // 3345
    150, 2, 2, 13, 2, 0, // 3585
    75, 3, 10, 9, 76, 3, 10, 13, 1, 0, // 3586
    188, 1, 4, 9, 2, 0, // 3587
    227, 0, 7, 5, 234, 2, 4, 2, 3, 0, // 3588
    106, 0, 6, 5, 140, 0, 12, 15, 168, 3, 15, 13, 2, 0, // 3589
    175, 1, 8, 3, 225, 0, 7, 10, 2, 0, // 3590
    92, 0, 0, 2, 199, 0, 9, 1, 1, 0, // 3591
    117, 3, 15, 1, 3, 0, // 3592
    152, 1, 4, 12, 207, 0, 1, 16, 209, 1, 4, 1, 3, 0, // 3594
    122, 1, 14, 6, 182, 1, 9, 5, 186, 3, 4, 15, 1, 0, // 3595
    236, 2, 8, 2, 2, 0, // 3596
    189, 1, 4, 9, 237, 2, 2, 8, 2, 0, // 3597
    149, 3, 7, 3, 150, 3, 16, 2, 4, 0, // 3598
    76, 0, 0, 10, 168, 0, 3, 15, 232, 0, 13, 14, 232, 3, 14, 13, 5, 0, // 3599
    111, 1, 1, 6, 197, 0, 9, 5, 216, 0, 1, 10, 232, 2, 13, 13, 235, 2, 4, 2, 2, 0, // 3600
    240, 2, 4, 5, 241, 2, 2, 5, 1, 0, // 3601
    238, 2, 10, 14, 3, 0, // 3841
    87, 3, 15, 0, 88, 3, 16, 3, 89, 3, 11, 2, 3, 0, // 3842
    111, 0, 6, 13, 112, 0, 6, 11, 216, 3, 10, 13, 1, 0, // 3844
    160, 1, 11, 12, 2, 0, // 3845
    170, 0, 3, 11, 235, 1, 2, 13, 2, 0, // 3847
    121, 3, 11, 3, 122, 3, 13, 9, 3, 0, // 3848
    223, 0, 7, 4, 228, 3, 16, 4, 229, 3, 5, 11, 2, 0, // 3849
    173, 1, 10, 3, 230, 1, 5, 7, 4, 0, // 3850
    161, 1, 11, 12, 196, 3, 16, 7, 197, 3, 5, 13, 198, 3, 14, 14, 2, 0, // 3851
    243, 0, 4, 5, 249, 3, 15, 11, 2, 0, // 3852
    252, 1, 5, 8, 253, 2, 15, 11, 3, 0, // 3853
    67, 0, 0, 3, 136, 0, 12, 4, 157, 3, 11, 4, 2, 0, // 3854
    232, 1, 13, 13, 238, 3, 16, 10, 2, 0, // 3855
    198, 0, 9, 14, 198, 1, 14, 9, 3, 0, // 3857
    97, 0, 0, 5, 244, 1, 2, 4, 254, 2, 11, 16, 1, 0, // 4097
    87, 1, 14, 0, 2, 0, // 4098
    117, 1, 13, 6, 218, 3, 2, 7, 2, 0, // 4099
    93, 1, 5, 0, 241, 1, 5, 13, 3, 0, // 4100
    143, 0, 12, 15, 180, 3, 11, 7, 181, 0, 7, 4, 4, 0, // 4101
    171, 0, 3, 7, 174, 1, 2, 3, 240, 1, 5, 13, 245, 3, 16, 11, 2, 0, // 4102
    125, 0, 6, 10, 159, 0, 12, 15, 3, 0, // 4105
    193, 0, 9, 7, 194, 0, 9, 16, 231, 1, 5, 7, 4, 0, // 4106
    187, 1, 4, 9, 203, 3, 5, 8, 204, 3, 1, 11, 205, 3, 15, 16, 4, 0, // 4108
    79, 1, 4, 0, 200, 0, 9, 5, 249, 1, 14, 10, 253, 0, 14, 11, 3, 0, // 4110
    140, 1, 3, 12, 168, 1, 13, 3, 186, 0, 9, 4, 3, 0, // 4112
    143, 1, 3, 12, 159, 1, 5, 12, 220, 1, 16, 1, 6, 0, // 4113
    98, 0, 0, 3, 99, 1, 16, 0, 205, 1, 15, 9, 220, 0, 1, 15, 221, 0, 1, 16, 255, 0, 5, 16, 3,
    0, // 4353
    97, 3, 5, 14, 98, 3, 3, 15, 99, 3, 15, 16, 4, 0, // 4354
    183, 0, 9, 4, 206, 0, 1, 4, 220, 3, 15, 15, 221, 3, 16, 15, 5, 0, // 4355
    150, 1, 13, 12, 156, 0, 12, 4, 214, 1, 8, 1, 244, 0, 4, 14, 250, 0, 2, 5, 1, 0, // 4356
    88, 1, 14, 0, 2, 0, // 4357
    153, 0, 12, 11, 228, 1, 14, 7, 2, 0, // 4358
    119, 1, 10, 6, 255, 3, 16, 15, 2, 0, // 4359
    132, 3, 11, 9, 133, 3, 2, 7, 2, 0, // 4360
    116, 0, 6, 11, 196, 1, 14, 9, 3, 0, // 4361
    124, 1, 5, 6, 195, 0, 9, 10, 222, 1, 7, 7, 1, 0, // 4362
    120, 1, 2, 6, 1, 0, // 4363
    238, 1, 14, 13, 3, 0, // 4364
    201, 0, 9, 4, 245, 1, 15, 4, 254, 1, 16, 14, 4, 0, // 4365
    64, 1, 12, 0, 68, 0, 0, 5, 162, 3, 7, 10, 163, 3, 11, 2, 1, 0, // 4366
    207, 1, 7, 1, 1, 0, // 4367
    254, 3, 16, 11, 4, 0, // 4368
    194, 1, 8, 9, 205, 0, 9, 15, 221, 1, 16, 1, 255, 1, 16, 5, 1, 0, // 4369
    99, 0, 0, 15, // prefillTiles
    0, 3, 4, 0, 4, 2, 0, 0, 6, 2, 3, 0, 14, 2, 6, 2, 5, 2, 6, 0, 21, 2, 8, 2, 10, 2, 10, 0, 23, 2,
    14, 2, 18, 2, 4, 2, 19, 2, 10, 2, 25, 2, 0, 4, 8, 2, 7, 0, 24, 2, 15, 2, 30, 2, 10, 4, 15, 2,
    12, 2, 3, 2, 4, 99, 26, 3, 4, 0, 60, 3, 12, 3, 137, 0, 12, 1, 128, 2, 2, 11, 101, 2, 8, 6, 100,
    0, 6, 10, 172, 0, 3, 10, 191, 1, 10, 9, 61, 1, 0, 0, 80, 3, 12, 13, 83, 2, 12, 8, 115, 1, 7, 6,
    130, 3, 14, 3, 141, 0, 12, 3, 158, 2, 5, 14, 31, 1, 4, 99, 27, 3, 4, 6, 104, 3, 10, 2, 81, 0,
    0, 12, 154, 3, 4, 4, 70, 1, 1, 0, 62, 3, 1, 6, 105, 3, 5, 1, 148, 0, 12, 8, 85, 2, 5, 8, 145,
    2, 12, 1, 146, 0, 12, 16, 219, 0, 1, 11, 86, 1, 8, 0, 63, 3, 5, 0, 91, 3, 10, 13, 40, 1, 1, 99,
    29, 3, 3, 4, 242, 3, 8, 10, 73, 2, 14, 7, 114, 0, 6, 8, 213, 0, 1, 14, 78, 1, 4, 0, 90, 3, 8,
    7, 138, 2, 3, 3, 177, 3, 11, 8, 155, 2, 4, 2, 144, 2, 1, 9, 164, 0, 3, 2, 251, 3, 8, 5, 178, 0,
    3, 10, 246, 3, 16, 15, 13, 1, 0, 99, 48, 3, 3, 0, 84, 3, 9, 3, 176, 3, 2, 9, 131, 2, 8, 15, 95,
    1, 15, 0, 77, 3, 11, 1, 215, 3, 15, 16, 103, 1, 6, 6, 102, 0, 6, 13, 233, 3, 2, 11, 211, 2, 1,
    10, 167, 2, 7, 13, 184, 1, 7, 9, 126, 1, 11, 6, 134, 3, 1, 2, 44, 1, 1, 99, 49, 3, 0, 6, 110,
    3, 10, 8, 248, 0, 10, 15, 202, 0, 9, 13, 127, 1, 11, 6, 129, 3, 5, 14, 239, 0, 13, 10, 123, 2,
    14, 14, 135, 2, 11, 16, 96, 0, 0, 13,
];

pub const PREFILL_DEPTH: usize = 90;

pub const PREFILL_TILES_OFFSET: [u16; 90] = [
    3262, 3266, 3270, 3274, 3278, 3282, 3286, 3290, 3294, 3298, 3302, 3306, 3310, 3314, 3318, 3322,
    3326, 3330, 3334, 3338, 3342, 3346, 3350, 3354, 3358, 3362, 3366, 3370, 3374, 3378, 3382, 3386,
    3390, 3394, 3398, 3402, 3406, 3410, 3414, 3418, 3422, 3426, 3430, 3434, 3438, 3442, 3446, 3450,
    3454, 3458, 3462, 3466, 3470, 3474, 3478, 3482, 3486, 3490, 3494, 3498, 3502, 3506, 3510, 3514,
    3518, 3522, 3526, 3530, 3534, 3538, 3542, 3546, 3550, 3554, 3558, 3562, 3566, 3570, 3574, 3578,
    3582, 3586, 3590, 3594, 3598, 3602, 3606, 3610, 3614, 3618,
];
