
use crate::Backtracker;
use crate::celltype::{
    CellType, MID,
    MID_LEFT, MID_TOP, MID_TOP_LEFT,
};
use crate::ori::{Ori, ANY};

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

pub const ANY_COLOUR: usize = 17;

const INVALID_CELL_IDX: u8 = 255;

#[derive(Debug)]
pub struct Cell {
    pub north_idx: u8,
    pub west_idx: u8,
    pub cell_type: CellType,
    pub ori: Ori,
}

pub const FILL_ORDER: [Cell; NUM_TILES] = [
    Cell { north_idx: INVALID_CELL_IDX, west_idx: INVALID_CELL_IDX, cell_type: MID_TOP_LEFT, ori: ANY }, // idx 0
    Cell { north_idx: INVALID_CELL_IDX, west_idx: 0, cell_type: MID_TOP, ori: ANY }, // idx 1
    Cell { north_idx: INVALID_CELL_IDX, west_idx: 1, cell_type: MID_TOP, ori: ANY }, // idx 2
    Cell { north_idx: INVALID_CELL_IDX, west_idx: 2, cell_type: MID_TOP, ori: ANY }, // idx 3
    Cell { north_idx: INVALID_CELL_IDX, west_idx: 3, cell_type: MID_TOP, ori: ANY }, // idx 4
    Cell { north_idx: INVALID_CELL_IDX, west_idx: 4, cell_type: MID_TOP, ori: ANY }, // idx 5
    Cell { north_idx: INVALID_CELL_IDX, west_idx: 5, cell_type: MID_TOP, ori: ANY }, // idx 6
    Cell { north_idx: INVALID_CELL_IDX, west_idx: 6, cell_type: MID_TOP, ori: ANY }, // idx 7
    Cell { north_idx: INVALID_CELL_IDX, west_idx: 7, cell_type: MID_TOP, ori: ANY }, // idx 8
    Cell { north_idx: INVALID_CELL_IDX, west_idx: 8, cell_type: MID_TOP, ori: ANY }, // idx 9
    Cell { north_idx: INVALID_CELL_IDX, west_idx: 9, cell_type: MID_TOP, ori: ANY }, // idx 10
    Cell { north_idx: INVALID_CELL_IDX, west_idx: 10, cell_type: MID_TOP, ori: ANY }, // idx 11
    Cell { north_idx: INVALID_CELL_IDX, west_idx: 11, cell_type: MID_TOP, ori: ANY }, // idx 12
    Cell { north_idx: INVALID_CELL_IDX, west_idx: 12, cell_type: MID_TOP, ori: ANY }, // idx 13
    Cell { north_idx: 0, west_idx: INVALID_CELL_IDX, cell_type: MID_LEFT, ori: ANY }, // idx 14
    Cell { north_idx: 1, west_idx: 14, cell_type: MID, ori: ANY }, // idx 15
    Cell { north_idx: 2, west_idx: 15, cell_type: MID, ori: ANY }, // idx 16
    Cell { north_idx: 3, west_idx: 16, cell_type: MID, ori: ANY }, // idx 17
    Cell { north_idx: 4, west_idx: 17, cell_type: MID, ori: ANY }, // idx 18
    Cell { north_idx: 5, west_idx: 18, cell_type: MID, ori: ANY }, // idx 19
    Cell { north_idx: 6, west_idx: 19, cell_type: MID, ori: ANY }, // idx 20
    Cell { north_idx: 7, west_idx: 20, cell_type: MID, ori: ANY }, // idx 21
    Cell { north_idx: 8, west_idx: 21, cell_type: MID, ori: ANY }, // idx 22
    Cell { north_idx: 9, west_idx: 22, cell_type: MID, ori: ANY }, // idx 23
    Cell { north_idx: 10, west_idx: 23, cell_type: MID, ori: ANY }, // idx 24
    Cell { north_idx: 11, west_idx: 24, cell_type: MID, ori: ANY }, // idx 25
    Cell { north_idx: 12, west_idx: 25, cell_type: MID, ori: ANY }, // idx 26
    Cell { north_idx: 13, west_idx: 26, cell_type: MID, ori: ANY }, // idx 27
    Cell { north_idx: 14, west_idx: INVALID_CELL_IDX, cell_type: MID_LEFT, ori: ANY }, // idx 28
    Cell { north_idx: 15, west_idx: 28, cell_type: MID, ori: ANY }, // idx 29
    Cell { north_idx: 16, west_idx: 29, cell_type: MID, ori: ANY }, // idx 30
    Cell { north_idx: 17, west_idx: 30, cell_type: MID, ori: ANY }, // idx 31
    Cell { north_idx: 18, west_idx: 31, cell_type: MID, ori: ANY }, // idx 32
    Cell { north_idx: 19, west_idx: 32, cell_type: MID, ori: ANY }, // idx 33
    Cell { north_idx: 20, west_idx: 33, cell_type: MID, ori: ANY }, // idx 34
    Cell { north_idx: 21, west_idx: 34, cell_type: MID, ori: ANY }, // idx 35
    Cell { north_idx: 22, west_idx: 35, cell_type: MID, ori: ANY }, // idx 36
    Cell { north_idx: 23, west_idx: 36, cell_type: MID, ori: ANY }, // idx 37
    Cell { north_idx: 24, west_idx: 37, cell_type: MID, ori: ANY }, // idx 38
    Cell { north_idx: 25, west_idx: 38, cell_type: MID, ori: ANY }, // idx 39
    Cell { north_idx: 26, west_idx: 39, cell_type: MID, ori: ANY }, // idx 40
    Cell { north_idx: 27, west_idx: 40, cell_type: MID, ori: ANY }, // idx 41
    Cell { north_idx: 28, west_idx: INVALID_CELL_IDX, cell_type: MID_LEFT, ori: ANY }, // idx 42
    Cell { north_idx: 29, west_idx: 42, cell_type: MID, ori: ANY }, // idx 43
    Cell { north_idx: 30, west_idx: 43, cell_type: MID, ori: ANY }, // idx 44
    Cell { north_idx: 31, west_idx: 44, cell_type: MID, ori: ANY }, // idx 45
    Cell { north_idx: 32, west_idx: 45, cell_type: MID, ori: ANY }, // idx 46
    Cell { north_idx: 33, west_idx: 46, cell_type: MID, ori: ANY }, // idx 47
    Cell { north_idx: 34, west_idx: 47, cell_type: MID, ori: ANY }, // idx 48
    Cell { north_idx: 35, west_idx: 48, cell_type: MID, ori: ANY }, // idx 49
    Cell { north_idx: 36, west_idx: 49, cell_type: MID, ori: ANY }, // idx 50
    Cell { north_idx: 37, west_idx: 50, cell_type: MID, ori: ANY }, // idx 51
    Cell { north_idx: 38, west_idx: 51, cell_type: MID, ori: ANY }, // idx 52
    Cell { north_idx: 39, west_idx: 52, cell_type: MID, ori: ANY }, // idx 53
    Cell { north_idx: 40, west_idx: 53, cell_type: MID, ori: ANY }, // idx 54
    Cell { north_idx: 41, west_idx: 54, cell_type: MID, ori: ANY }, // idx 55
    Cell { north_idx: 42, west_idx: INVALID_CELL_IDX, cell_type: MID_LEFT, ori: ANY }, // idx 56
    Cell { north_idx: 43, west_idx: 56, cell_type: MID, ori: ANY }, // idx 57
    Cell { north_idx: 44, west_idx: 57, cell_type: MID, ori: ANY }, // idx 58
    Cell { north_idx: 45, west_idx: 58, cell_type: MID, ori: ANY }, // idx 59
    Cell { north_idx: 46, west_idx: 59, cell_type: MID, ori: ANY }, // idx 60
    Cell { north_idx: 47, west_idx: 60, cell_type: MID, ori: ANY }, // idx 61
    Cell { north_idx: 48, west_idx: 61, cell_type: MID, ori: ANY }, // idx 62
    Cell { north_idx: 49, west_idx: 62, cell_type: MID, ori: ANY }, // idx 63
    Cell { north_idx: 50, west_idx: 63, cell_type: MID, ori: ANY }, // idx 64
    Cell { north_idx: 51, west_idx: 64, cell_type: MID, ori: ANY }, // idx 65
    Cell { north_idx: 52, west_idx: 65, cell_type: MID, ori: ANY }, // idx 66
    Cell { north_idx: 53, west_idx: 66, cell_type: MID, ori: ANY }, // idx 67
    Cell { north_idx: 54, west_idx: 67, cell_type: MID, ori: ANY }, // idx 68
    Cell { north_idx: 55, west_idx: 68, cell_type: MID, ori: ANY }, // idx 69
    Cell { north_idx: 56, west_idx: INVALID_CELL_IDX, cell_type: MID_LEFT, ori: ANY }, // idx 70
    Cell { north_idx: 57, west_idx: 70, cell_type: MID, ori: ANY }, // idx 71
    Cell { north_idx: 58, west_idx: 71, cell_type: MID, ori: ANY }, // idx 72
    Cell { north_idx: 59, west_idx: 72, cell_type: MID, ori: ANY }, // idx 73
    Cell { north_idx: 60, west_idx: 73, cell_type: MID, ori: ANY }, // idx 74
    Cell { north_idx: 61, west_idx: 74, cell_type: MID, ori: ANY }, // idx 75
    Cell { north_idx: 62, west_idx: 75, cell_type: MID, ori: ANY }, // idx 76
    Cell { north_idx: 63, west_idx: 76, cell_type: MID, ori: ANY }, // idx 77
    Cell { north_idx: 64, west_idx: 77, cell_type: MID, ori: ANY }, // idx 78
    Cell { north_idx: 65, west_idx: 78, cell_type: MID, ori: ANY }, // idx 79
    Cell { north_idx: 66, west_idx: 79, cell_type: MID, ori: ANY }, // idx 80
    Cell { north_idx: 67, west_idx: 80, cell_type: MID, ori: ANY }, // idx 81
    Cell { north_idx: 68, west_idx: 81, cell_type: MID, ori: ANY }, // idx 82
    Cell { north_idx: 69, west_idx: 82, cell_type: MID, ori: ANY }, // idx 83
    Cell { north_idx: 70, west_idx: INVALID_CELL_IDX, cell_type: MID_LEFT, ori: ANY }, // idx 84
    Cell { north_idx: 71, west_idx: 84, cell_type: MID, ori: ANY }, // idx 85
    Cell { north_idx: 72, west_idx: 85, cell_type: MID, ori: ANY }, // idx 86
    Cell { north_idx: 73, west_idx: 86, cell_type: MID, ori: ANY }, // idx 87
    Cell { north_idx: 74, west_idx: 87, cell_type: MID, ori: ANY }, // idx 88
    Cell { north_idx: 75, west_idx: 88, cell_type: MID, ori: ANY }, // idx 89
    Cell { north_idx: 76, west_idx: 89, cell_type: MID, ori: ANY }, // idx 90
    Cell { north_idx: 77, west_idx: 90, cell_type: MID, ori: ANY }, // idx 91
    Cell { north_idx: 78, west_idx: 91, cell_type: MID, ori: ANY }, // idx 92
    Cell { north_idx: 79, west_idx: 92, cell_type: MID, ori: ANY }, // idx 93
    Cell { north_idx: 80, west_idx: 93, cell_type: MID, ori: ANY }, // idx 94
    Cell { north_idx: 81, west_idx: 94, cell_type: MID, ori: ANY }, // idx 95
    Cell { north_idx: 82, west_idx: 95, cell_type: MID, ori: ANY }, // idx 96
    Cell { north_idx: 83, west_idx: 96, cell_type: MID, ori: ANY }, // idx 97
    Cell { north_idx: 84, west_idx: INVALID_CELL_IDX, cell_type: MID_LEFT, ori: ANY }, // idx 98
    Cell { north_idx: 85, west_idx: 98, cell_type: MID, ori: ANY }, // idx 99
    Cell { north_idx: 86, west_idx: 99, cell_type: MID, ori: ANY }, // idx 100
    Cell { north_idx: 87, west_idx: 100, cell_type: MID, ori: ANY }, // idx 101
    Cell { north_idx: 88, west_idx: 101, cell_type: MID, ori: ANY }, // idx 102
    Cell { north_idx: 89, west_idx: 102, cell_type: MID, ori: ANY }, // idx 103
    Cell { north_idx: 90, west_idx: 103, cell_type: MID, ori: ANY }, // idx 104
    Cell { north_idx: 91, west_idx: 104, cell_type: MID, ori: ANY }, // idx 105
    Cell { north_idx: 92, west_idx: 105, cell_type: MID, ori: ANY }, // idx 106
    Cell { north_idx: 93, west_idx: 106, cell_type: MID, ori: ANY }, // idx 107
    Cell { north_idx: 94, west_idx: 107, cell_type: MID, ori: ANY }, // idx 108
    Cell { north_idx: 95, west_idx: 108, cell_type: MID, ori: ANY }, // idx 109
    Cell { north_idx: 96, west_idx: 109, cell_type: MID, ori: ANY }, // idx 110
    Cell { north_idx: 97, west_idx: 110, cell_type: MID, ori: ANY }, // idx 111
    Cell { north_idx: 98, west_idx: INVALID_CELL_IDX, cell_type: MID_LEFT, ori: ANY }, // idx 112
    Cell { north_idx: 99, west_idx: 112, cell_type: MID, ori: ANY }, // idx 113
    Cell { north_idx: 100, west_idx: 113, cell_type: MID, ori: ANY }, // idx 114
    Cell { north_idx: 101, west_idx: 114, cell_type: MID, ori: ANY }, // idx 115
    Cell { north_idx: 102, west_idx: 115, cell_type: MID, ori: ANY }, // idx 116
    Cell { north_idx: 103, west_idx: 116, cell_type: MID, ori: ANY }, // idx 117
    Cell { north_idx: 104, west_idx: 117, cell_type: MID, ori: ANY }, // idx 118
    Cell { north_idx: 105, west_idx: 118, cell_type: MID, ori: ANY }, // idx 119
    Cell { north_idx: 106, west_idx: 119, cell_type: MID, ori: ANY }, // idx 120
    Cell { north_idx: 107, west_idx: 120, cell_type: MID, ori: ANY }, // idx 121
    Cell { north_idx: 108, west_idx: 121, cell_type: MID, ori: ANY }, // idx 122
    Cell { north_idx: 109, west_idx: 122, cell_type: MID, ori: ANY }, // idx 123
    Cell { north_idx: 110, west_idx: 123, cell_type: MID, ori: ANY }, // idx 124
    Cell { north_idx: 111, west_idx: 124, cell_type: MID, ori: ANY }, // idx 125
    Cell { north_idx: 112, west_idx: INVALID_CELL_IDX, cell_type: MID_LEFT, ori: ANY }, // idx 126
    Cell { north_idx: 113, west_idx: 126, cell_type: MID, ori: ANY }, // idx 127
    Cell { north_idx: 114, west_idx: 127, cell_type: MID, ori: ANY }, // idx 128
    Cell { north_idx: 115, west_idx: 128, cell_type: MID, ori: ANY }, // idx 129
    Cell { north_idx: 116, west_idx: 129, cell_type: MID, ori: ANY }, // idx 130
    Cell { north_idx: 117, west_idx: 130, cell_type: MID, ori: ANY }, // idx 131
    Cell { north_idx: 118, west_idx: 131, cell_type: MID, ori: ANY }, // idx 132
    Cell { north_idx: 119, west_idx: 132, cell_type: MID, ori: ANY }, // idx 133
    Cell { north_idx: 120, west_idx: 133, cell_type: MID, ori: ANY }, // idx 134
    Cell { north_idx: 121, west_idx: 134, cell_type: MID, ori: ANY }, // idx 135
    Cell { north_idx: 122, west_idx: 135, cell_type: MID, ori: ANY }, // idx 136
    Cell { north_idx: 123, west_idx: 136, cell_type: MID, ori: ANY }, // idx 137
    Cell { north_idx: 124, west_idx: 137, cell_type: MID, ori: ANY }, // idx 138
    Cell { north_idx: 125, west_idx: 138, cell_type: MID, ori: ANY }, // idx 139
    Cell { north_idx: 126, west_idx: INVALID_CELL_IDX, cell_type: MID_LEFT, ori: ANY }, // idx 140
    Cell { north_idx: 127, west_idx: 140, cell_type: MID, ori: ANY }, // idx 141
    Cell { north_idx: 128, west_idx: 141, cell_type: MID, ori: ANY }, // idx 142
    Cell { north_idx: 129, west_idx: 142, cell_type: MID, ori: ANY }, // idx 143
    Cell { north_idx: 130, west_idx: 143, cell_type: MID, ori: ANY }, // idx 144
    Cell { north_idx: 131, west_idx: 144, cell_type: MID, ori: ANY }, // idx 145
    Cell { north_idx: 132, west_idx: 145, cell_type: MID, ori: ANY }, // idx 146
    Cell { north_idx: 133, west_idx: 146, cell_type: MID, ori: ANY }, // idx 147
    Cell { north_idx: 134, west_idx: 147, cell_type: MID, ori: ANY }, // idx 148
    Cell { north_idx: 135, west_idx: 148, cell_type: MID, ori: ANY }, // idx 149
    Cell { north_idx: 136, west_idx: 149, cell_type: MID, ori: ANY }, // idx 150
    Cell { north_idx: 137, west_idx: 150, cell_type: MID, ori: ANY }, // idx 151
    Cell { north_idx: 138, west_idx: 151, cell_type: MID, ori: ANY }, // idx 152
    Cell { north_idx: 139, west_idx: 152, cell_type: MID, ori: ANY }, // idx 153
    Cell { north_idx: 140, west_idx: INVALID_CELL_IDX, cell_type: MID_LEFT, ori: ANY }, // idx 154
    Cell { north_idx: 141, west_idx: 154, cell_type: MID, ori: ANY }, // idx 155
    Cell { north_idx: 142, west_idx: 155, cell_type: MID, ori: ANY }, // idx 156
    Cell { north_idx: 143, west_idx: 156, cell_type: MID, ori: ANY }, // idx 157
    Cell { north_idx: 144, west_idx: 157, cell_type: MID, ori: ANY }, // idx 158
    Cell { north_idx: 145, west_idx: 158, cell_type: MID, ori: ANY }, // idx 159
    Cell { north_idx: 146, west_idx: 159, cell_type: MID, ori: ANY }, // idx 160
    Cell { north_idx: 147, west_idx: 160, cell_type: MID, ori: ANY }, // idx 161
    Cell { north_idx: 148, west_idx: 161, cell_type: MID, ori: ANY }, // idx 162
    Cell { north_idx: 149, west_idx: 162, cell_type: MID, ori: ANY }, // idx 163
    Cell { north_idx: 150, west_idx: 163, cell_type: MID, ori: ANY }, // idx 164
    Cell { north_idx: 151, west_idx: 164, cell_type: MID, ori: ANY }, // idx 165
    Cell { north_idx: 152, west_idx: 165, cell_type: MID, ori: ANY }, // idx 166
    Cell { north_idx: 153, west_idx: 166, cell_type: MID, ori: ANY }, // idx 167
    Cell { north_idx: 154, west_idx: INVALID_CELL_IDX, cell_type: MID_LEFT, ori: ANY }, // idx 168
    Cell { north_idx: 155, west_idx: 168, cell_type: MID, ori: ANY }, // idx 169
    Cell { north_idx: 156, west_idx: 169, cell_type: MID, ori: ANY }, // idx 170
    Cell { north_idx: 157, west_idx: 170, cell_type: MID, ori: ANY }, // idx 171
    Cell { north_idx: 158, west_idx: 171, cell_type: MID, ori: ANY }, // idx 172
    Cell { north_idx: 159, west_idx: 172, cell_type: MID, ori: ANY }, // idx 173
    Cell { north_idx: 160, west_idx: 173, cell_type: MID, ori: ANY }, // idx 174
    Cell { north_idx: 161, west_idx: 174, cell_type: MID, ori: ANY }, // idx 175
    Cell { north_idx: 162, west_idx: 175, cell_type: MID, ori: ANY }, // idx 176
    Cell { north_idx: 163, west_idx: 176, cell_type: MID, ori: ANY }, // idx 177
    Cell { north_idx: 164, west_idx: 177, cell_type: MID, ori: ANY }, // idx 178
    Cell { north_idx: 165, west_idx: 178, cell_type: MID, ori: ANY }, // idx 179
    Cell { north_idx: 166, west_idx: 179, cell_type: MID, ori: ANY }, // idx 180
    Cell { north_idx: 167, west_idx: 180, cell_type: MID, ori: ANY }, // idx 181
    Cell { north_idx: 168, west_idx: INVALID_CELL_IDX, cell_type: MID_LEFT, ori: ANY }, // idx 182
    Cell { north_idx: 169, west_idx: 182, cell_type: MID, ori: ANY }, // idx 183
    Cell { north_idx: 170, west_idx: 183, cell_type: MID, ori: ANY }, // idx 184
    Cell { north_idx: 171, west_idx: 184, cell_type: MID, ori: ANY }, // idx 185
    Cell { north_idx: 172, west_idx: 185, cell_type: MID, ori: ANY }, // idx 186
    Cell { north_idx: 173, west_idx: 186, cell_type: MID, ori: ANY }, // idx 187
    Cell { north_idx: 174, west_idx: 187, cell_type: MID, ori: ANY }, // idx 188
    Cell { north_idx: 175, west_idx: 188, cell_type: MID, ori: ANY }, // idx 189
    Cell { north_idx: 176, west_idx: 189, cell_type: MID, ori: ANY }, // idx 190
    Cell { north_idx: 177, west_idx: 190, cell_type: MID, ori: ANY }, // idx 191
    Cell { north_idx: 178, west_idx: 191, cell_type: MID, ori: ANY }, // idx 192
    Cell { north_idx: 179, west_idx: 192, cell_type: MID, ori: ANY }, // idx 193
    Cell { north_idx: 180, west_idx: 193, cell_type: MID, ori: ANY }, // idx 194
    Cell { north_idx: 181, west_idx: 194, cell_type: MID, ori: ANY }, // idx 195
];

pub const DISPLAY_TO_FILL_ORDER: [i16; NUM_CELLS] = [
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, -1, -1, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, -1, -1, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, -1, -1, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, -1, -1, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, -1, -1, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, -1, -1, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, -1, -1, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, -1, -1, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, -1, -1, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, -1, -1, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, -1, -1, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, -1, -1, 168, 169, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 180, 181, -1, -1, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192, 193, 194, 195, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 
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
    [0, 2, 16, 26, 0, 32, 0, 0, 38, 48, 54, 0, 0, 60, 66, 72, 0, 82, ], // north 1
    [0, 144, 154, 0, 164, 170, 180, 0, 190, 196, 206, 220, 230, 244, 254, 260, 274, 288, ], // north 2
    [406, 412, 422, 428, 438, 448, 0, 458, 472, 494, 512, 518, 532, 550, 0, 0, 572, 582, ], // north 3
    [0, 0, 732, 738, 0, 744, 754, 760, 0, 0, 766, 776, 790, 796, 802, 812, 826, 832, ], // north 4
    [910, 920, 930, 0, 952, 958, 968, 974, 992, 998, 0, 1008, 1014, 1020, 1026, 1044, 1054, 1072, ], // north 5
    [1206, 0, 1224, 1234, 0, 1244, 0, 1258, 1280, 1298, 0, 1304, 1310, 1316, 1338, 1356, 1362, 1372, ], // north 6
    [0, 1514, 1520, 1526, 0, 1532, 1538, 1548, 1554, 1564, 1570, 1584, 0, 1598, 1612, 1622, 0, 1628, ], // north 7
    [1718, 1724, 1734, 1744, 1754, 1772, 1786, 1804, 1818, 1828, 1834, 0, 0, 0, 1848, 1858, 1868, 1882, ], // north 8
    [2020, 2030, 2040, 0, 2058, 2072, 0, 2098, 2112, 0, 2118, 2136, 0, 2142, 2148, 2154, 0, 2168, ], // north 9
    [0, 2294, 0, 2308, 2314, 2332, 0, 2354, 2360, 2370, 0, 2376, 2382, 2388, 2398, 2416, 2426, 2432, ], // north 10
    [0, 2546, 2552, 2566, 2576, 2586, 0, 0, 2596, 2602, 2608, 2626, 0, 2636, 2650, 2660, 2674, 2680, ], // north 11
    [0, 2790, 2796, 2806, 2816, 2846, 2852, 2870, 2880, 2890, 0, 0, 2896, 0, 2906, 2924, 2938, 2952, ], // north 12
    [0, 3090, 3096, 3102, 3108, 0, 3118, 3124, 3138, 3148, 0, 3154, 0, 3164, 3170, 3180, 3186, 3196, ], // north 13
    [0, 0, 3278, 0, 3296, 3302, 0, 3316, 3330, 3336, 3350, 3360, 3378, 3388, 3398, 3420, 3430, 3436, ], // north 14
    [0, 3570, 0, 3584, 3598, 0, 3608, 3622, 3636, 3646, 3664, 3678, 3700, 3710, 3724, 0, 3738, 3752, ], // north 15
    [3910, 3916, 3922, 3932, 3938, 3952, 3958, 0, 3968, 3974, 0, 3996, 0, 4014, 4024, 4030, 4044, 4074, ], // north 16
    [4212, 4230, 4252, 0, 4274, 4280, 4286, 4296, 4306, 0, 4316, 4322, 4332, 4346, 4352, 4358, 4372, 4378, ], // north 17
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ], // north ANY
];

pub static BICOLOUR_TILES: [u8; 4796] = [
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
    3, 0, // 258
    14, 2, 5, 7, 18, 2, 14, 4, 35, 2, 15, 15,
    2, 0, // 259
    22, 2, 7, 2, 32, 2, 13, 5,
    1, 0, // 260
    38, 2, 15, 16,
    1, 0, // 262
    37, 2, 14, 16,
    2, 0, // 265
    25, 2, 5, 8, 30, 2, 7, 5,
    1, 0, // 266
    34, 2, 8, 5,
    1, 0, // 267
    31, 2, 13, 5,
    1, 0, // 270
    36, 2, 2, 16,
    1, 0, // 271
    26, 2, 11, 8,
    2, 0, // 272
    19, 2, 11, 4, 39, 2, 16, 16,
    15, 0, // 274
    14, 2, 5, 7, 18, 2, 14, 4, 19, 2, 11, 4, 22, 2, 7, 2, 25, 2, 5, 8, 26, 2, 11, 8, 30, 2, 7, 5, 31, 2, 13, 5, 32, 2, 13, 5, 34, 2, 8, 5, 35, 2, 15, 15, 36, 2, 2, 16, 37, 2, 14, 16, 38, 2, 15, 16, 39, 2, 16, 16,
    2, 0, // 514
    87, 0, 12, 9, 146, 3, 4, 16,
    2, 0, // 515
    68, 1, 11, 6, 158, 2, 7, 15,
    1, 0, // 517
    146, 2, 16, 1,
    2, 0, // 518
    14, 1, 7, 0, 152, 2, 5, 2,
    2, 0, // 519
    51, 3, 13, 14, 52, 3, 11, 14,
    1, 0, // 521
    157, 2, 2, 11,
    2, 0, // 522
    84, 0, 12, 2, 123, 3, 4, 16,
    3, 0, // 523
    148, 2, 5, 7, 150, 2, 11, 4, 156, 2, 13, 14,
    2, 0, // 524
    144, 1, 15, 9, 159, 2, 7, 16,
    3, 0, // 525
    86, 3, 16, 12, 87, 3, 9, 1, 88, 3, 8, 10,
    2, 0, // 526
    139, 1, 5, 9, 149, 2, 7, 4,
    1, 0, // 527
    18, 1, 4, 0,
    3, 0, // 528
    35, 1, 15, 0, 155, 2, 16, 8, 160, 2, 15, 16,
    3, 0, // 529
    147, 2, 13, 7, 154, 2, 2, 8, 161, 2, 15, 16,
    29, 0, // 530
    14, 1, 7, 0, 18, 1, 4, 0, 35, 1, 15, 0, 51, 3, 13, 14, 52, 3, 11, 14, 68, 1, 11, 6, 84, 0, 12, 2, 86, 3, 16, 12, 87, 0, 12, 9, 87, 3, 9, 1, 88, 3, 8, 10, 123, 3, 4, 16, 139, 1, 5, 9, 144, 1, 15, 9, 146, 3, 4, 16, 146, 2, 16, 1, 147, 2, 13, 7, 148, 2, 5, 7, 149, 2, 7, 4, 150, 2, 11, 4, 152, 2, 5, 2, 154, 2, 2, 8, 155, 2, 16, 8, 156, 2, 13, 14, 157, 2, 2, 11, 158, 2, 7, 15, 159, 2, 7, 16, 160, 2, 15, 16, 161, 2, 15, 16,
    1, 0, // 769
    22, 3, 2, 7,
    2, 0, // 770
    84, 1, 9, 12, 152, 3, 5, 5,
    1, 0, // 771
    190, 3, 5, 16,
    2, 0, // 772
    47, 0, 6, 10, 114, 3, 15, 4,
    2, 0, // 773
    174, 1, 2, 13, 184, 3, 14, 16,
    2, 0, // 774
    181, 0, 13, 15, 190, 2, 16, 2,
    3, 0, // 776
    22, 1, 2, 0, 158, 1, 15, 1, 167, 3, 5, 13,
    5, 0, // 777
    154, 0, 1, 16, 177, 0, 13, 11, 187, 1, 2, 10, 188, 0, 10, 15, 191, 2, 5, 8,
    4, 0, // 778
    53, 1, 7, 6, 105, 0, 3, 5, 116, 1, 14, 3, 132, 3, 4, 7,
    1, 0, // 779
    187, 3, 2, 8,
    3, 0, // 780
    68, 0, 6, 1, 157, 0, 1, 8, 173, 1, 4, 13,
    4, 0, // 781
    44, 0, 6, 10, 94, 3, 4, 4, 95, 3, 8, 4, 96, 3, 4, 16,
    5, 0, // 782
    32, 1, 5, 0, 90, 0, 12, 16, 174, 3, 2, 4, 175, 3, 14, 4, 176, 3, 10, 8,
    2, 0, // 785
    36, 0, 0, 13, 103, 0, 12, 11,
    37, 0, // 786
    22, 3, 2, 7, 22, 1, 2, 0, 32, 1, 5, 0, 36, 0, 0, 13, 44, 0, 6, 10, 47, 0, 6, 10, 53, 1, 7, 6, 68, 0, 6, 1, 84, 1, 9, 12, 90, 0, 12, 16, 94, 3, 4, 4, 95, 3, 8, 4, 96, 3, 4, 16, 103, 0, 12, 11, 105, 0, 3, 5, 114, 3, 15, 4, 116, 1, 14, 3, 132, 3, 4, 7, 152, 3, 5, 5, 154, 0, 1, 16, 157, 0, 1, 8, 158, 1, 15, 1, 167, 3, 5, 13, 173, 1, 4, 13, 174, 3, 2, 4, 174, 1, 2, 13, 175, 3, 14, 4, 176, 3, 10, 8, 177, 0, 13, 11, 181, 0, 13, 15, 184, 3, 14, 16, 187, 3, 2, 8, 187, 1, 2, 10, 188, 0, 10, 15, 190, 3, 5, 16, 190, 2, 16, 2, 191, 2, 5, 8,
    1, 0, // 1027
    116, 2, 9, 14,
    1, 0, // 1028
    78, 0, 12, 7,
    2, 0, // 1030
    98, 1, 14, 12, 105, 2, 2, 9,
    1, 0, // 1031
    47, 3, 10, 2,
    1, 0, // 1032
    121, 3, 4, 15,
    2, 0, // 1035
    112, 2, 10, 10, 118, 2, 5, 5,
    3, 0, // 1036
    100, 0, 12, 14, 117, 2, 8, 5, 119, 2, 4, 11,
    1, 0, // 1037
    78, 3, 7, 3,
    1, 0, // 1038
    89, 0, 12, 7,
    2, 0, // 1039
    61, 0, 6, 11, 113, 2, 8, 10,
    3, 0, // 1040
    38, 1, 16, 0, 70, 0, 6, 14, 114, 2, 4, 2,
    1, 0, // 1041
    43, 1, 6, 6,
    19, 0, // 1042
    38, 1, 16, 0, 43, 1, 6, 6, 47, 3, 10, 2, 61, 0, 6, 11, 70, 0, 6, 14, 78, 0, 12, 7, 78, 3, 7, 3, 89, 0, 12, 7, 98, 1, 14, 12, 100, 0, 12, 14, 105, 2, 2, 9, 112, 2, 10, 10, 113, 2, 8, 10, 114, 2, 4, 2, 116, 2, 9, 14, 117, 2, 8, 5, 118, 2, 5, 5, 119, 2, 4, 11, 121, 3, 4, 15,
    2, 0, // 1281
    18, 3, 1, 14, 19, 3, 15, 11,
    2, 0, // 1282
    149, 3, 13, 7, 150, 3, 10, 11,
    5, 0, // 1283
    94, 0, 12, 4, 95, 0, 12, 8, 114, 0, 3, 15, 174, 0, 13, 2, 175, 0, 13, 14,
    1, 0, // 1285
    94, 1, 2, 12,
    2, 0, // 1286
    180, 0, 13, 15, 183, 2, 14, 10,
    1, 0, // 1287
    58, 3, 8, 11,
    4, 0, // 1288
    54, 0, 6, 8, 132, 1, 2, 9, 165, 3, 10, 13, 166, 3, 8, 8,
    1, 0, // 1289
    182, 2, 10, 10,
    2, 0, // 1290
    127, 3, 15, 9, 129, 3, 13, 11,
    1, 0, // 1292
    119, 0, 3, 11,
    1, 0, // 1293
    92, 3, 13, 7,
    1, 0, // 1294
    173, 3, 2, 11,
    4, 0, // 1295
    97, 0, 12, 11, 163, 1, 7, 7, 168, 0, 7, 16, 184, 2, 16, 2,
    2, 0, // 1296
    121, 1, 3, 7, 126, 1, 13, 9,
    4, 0, // 1297
    96, 1, 2, 12, 123, 1, 1, 9, 146, 1, 1, 1, 185, 2, 11, 15,
    33, 0, // 1298
    18, 3, 1, 14, 19, 3, 15, 11, 54, 0, 6, 8, 58, 3, 8, 11, 92, 3, 13, 7, 94, 0, 12, 4, 94, 1, 2, 12, 95, 0, 12, 8, 96, 1, 2, 12, 97, 0, 12, 11, 114, 0, 3, 15, 119, 0, 3, 11, 121, 1, 3, 7, 123, 1, 1, 9, 126, 1, 13, 9, 127, 3, 15, 9, 129, 3, 13, 11, 132, 1, 2, 9, 146, 1, 1, 1, 149, 3, 13, 7, 150, 3, 10, 11, 163, 1, 7, 7, 165, 3, 10, 13, 166, 3, 8, 8, 168, 0, 7, 16, 173, 3, 2, 11, 174, 0, 13, 2, 175, 0, 13, 14, 180, 0, 13, 15, 182, 2, 10, 10, 183, 2, 14, 10, 184, 2, 16, 2, 185, 2, 11, 15,
    4, 0, // 1537
    30, 3, 8, 7, 31, 3, 10, 13, 32, 3, 2, 13, 34, 3, 9, 8,
    2, 0, // 1539
    105, 1, 9, 3, 152, 0, 1, 5,
    2, 0, // 1540
    117, 3, 11, 8, 118, 3, 10, 5,
    3, 0, // 1542
    118, 0, 3, 10, 125, 1, 7, 9, 152, 1, 2, 1,
    5, 0, // 1544
    14, 0, 0, 1, 125, 0, 9, 5, 148, 0, 1, 10, 170, 3, 14, 8, 171, 3, 15, 8,
    4, 0, // 1545
    25, 0, 0, 8, 143, 1, 15, 9, 191, 0, 2, 8, 192, 3, 14, 11,
    1, 0, // 1546
    139, 3, 1, 13,
    1, 0, // 1548
    169, 1, 14, 7,
    1, 0, // 1549
    99, 3, 15, 15,
    5, 0, // 1550
    122, 2, 9, 9, 137, 1, 14, 9, 167, 1, 2, 7, 180, 3, 15, 4, 181, 3, 15, 2,
    4, 0, // 1551
    37, 1, 16, 0, 69, 1, 11, 6, 98, 0, 12, 3, 183, 1, 10, 4,
    1, 0, // 1552
    140, 1, 11, 9,
    2, 0, // 1553
    190, 1, 2, 2, 195, 2, 15, 16,
    35, 0, // 1554
    14, 0, 0, 1, 25, 0, 0, 8, 30, 3, 8, 7, 31, 3, 10, 13, 32, 3, 2, 13, 34, 3, 9, 8, 37, 1, 16, 0, 69, 1, 11, 6, 98, 0, 12, 3, 99, 3, 15, 15, 105, 1, 9, 3, 117, 3, 11, 8, 118, 0, 3, 10, 118, 3, 10, 5, 122, 2, 9, 9, 125, 0, 9, 5, 125, 1, 7, 9, 137, 1, 14, 9, 139, 3, 1, 13, 140, 1, 11, 9, 143, 1, 15, 9, 148, 0, 1, 10, 152, 0, 1, 5, 152, 1, 2, 1, 167, 1, 2, 7, 169, 1, 14, 7, 170, 3, 14, 8, 171, 3, 15, 8, 180, 3, 15, 4, 181, 3, 15, 2, 183, 1, 10, 4, 190, 1, 2, 2, 191, 0, 2, 8, 192, 3, 14, 11, 195, 2, 15, 16,
    1, 0, // 1794
    68, 2, 2, 11,
    1, 0, // 1795
    53, 2, 9, 7,
    1, 0, // 1796
    43, 2, 16, 6,
    1, 0, // 1798
    69, 2, 14, 11,
    2, 0, // 1799
    42, 3, 13, 11, 43, 3, 3, 16,
    1, 0, // 1800
    55, 2, 8, 7,
    2, 0, // 1801
    54, 2, 4, 7, 58, 2, 11, 4,
    1, 0, // 1802
    71, 2, 8, 15,
    3, 0, // 1803
    44, 2, 2, 12, 47, 2, 2, 3, 63, 2, 14, 14,
    3, 0, // 1804
    52, 2, 14, 1, 56, 2, 16, 7, 61, 2, 3, 14,
    3, 0, // 1806
    42, 2, 11, 6, 51, 2, 14, 1, 62, 2, 9, 14,
    2, 0, // 1807
    70, 2, 3, 15, 75, 2, 11, 16,
    1, 0, // 1808
    67, 2, 13, 11,
    22, 0, // 1810
    42, 3, 13, 11, 42, 2, 11, 6, 43, 3, 3, 16, 43, 2, 16, 6, 44, 2, 2, 12, 47, 2, 2, 3, 51, 2, 14, 1, 52, 2, 14, 1, 53, 2, 9, 7, 54, 2, 4, 7, 55, 2, 8, 7, 56, 2, 16, 7, 58, 2, 11, 4, 61, 2, 3, 14, 62, 2, 9, 14, 63, 2, 14, 14, 67, 2, 13, 11, 68, 2, 2, 11, 69, 2, 14, 11, 70, 2, 3, 15, 71, 2, 8, 15, 75, 2, 11, 16,
    1, 0, // 2049
    14, 3, 1, 5,
    2, 0, // 2050
    147, 3, 16, 13, 148, 3, 10, 5,
    2, 0, // 2051
    22, 0, 0, 2, 132, 0, 9, 4,
    2, 0, // 2052
    78, 1, 3, 12, 89, 1, 13, 12,
    4, 0, // 2053
    92, 0, 12, 13, 121, 2, 15, 3, 149, 0, 1, 13, 163, 2, 14, 7,
    3, 0, // 2054
    30, 0, 0, 8, 167, 2, 13, 2, 169, 2, 11, 14,
    4, 0, // 2055
    53, 3, 2, 9, 54, 3, 8, 4, 55, 3, 7, 8, 56, 3, 11, 16,
    3, 0, // 2056
    162, 3, 16, 8, 163, 3, 4, 14, 164, 3, 10, 11,
    2, 0, // 2057
    55, 1, 7, 6, 166, 2, 8, 4,
    1, 0, // 2058
    125, 3, 5, 5,
    3, 0, // 2059
    102, 1, 16, 12, 164, 2, 11, 7, 165, 2, 13, 4,
    2, 0, // 2063
    136, 0, 9, 16, 170, 2, 8, 5,
    2, 0, // 2064
    158, 0, 1, 2, 171, 2, 8, 5,
    3, 0, // 2065
    159, 0, 1, 11, 162, 2, 8, 7, 168, 2, 4, 14,
    34, 0, // 2066
    14, 3, 1, 5, 22, 0, 0, 2, 30, 0, 0, 8, 53, 3, 2, 9, 54, 3, 8, 4, 55, 3, 7, 8, 55, 1, 7, 6, 56, 3, 11, 16, 78, 1, 3, 12, 89, 1, 13, 12, 92, 0, 12, 13, 102, 1, 16, 12, 121, 2, 15, 3, 125, 3, 5, 5, 132, 0, 9, 4, 136, 0, 9, 16, 147, 3, 16, 13, 148, 3, 10, 5, 149, 0, 1, 13, 158, 0, 1, 2, 159, 0, 1, 11, 162, 3, 16, 8, 162, 2, 8, 7, 163, 3, 4, 14, 163, 2, 14, 7, 164, 3, 10, 11, 164, 2, 11, 7, 165, 2, 13, 4, 166, 2, 8, 4, 167, 2, 13, 2, 168, 2, 4, 14, 169, 2, 11, 14, 170, 2, 8, 5, 171, 2, 8, 5,
    2, 0, // 2305
    25, 3, 8, 5, 26, 3, 14, 11,
    2, 0, // 2306
    154, 3, 16, 2, 155, 3, 15, 16,
    4, 0, // 2307
    157, 1, 11, 1, 176, 0, 13, 10, 187, 0, 10, 2, 191, 3, 8, 5,
    3, 0, // 2309
    54, 1, 7, 6, 95, 1, 2, 12, 166, 0, 7, 8,
    6, 0, // 2310
    25, 1, 8, 0, 34, 0, 0, 9, 117, 0, 3, 11, 170, 0, 7, 14, 171, 0, 7, 15, 191, 1, 8, 2,
    3, 0, // 2312
    30, 1, 5, 0, 55, 0, 6, 7, 162, 0, 7, 16,
    1, 0, // 2313
    166, 1, 4, 7,
    4, 0, // 2315
    88, 1, 1, 12, 113, 0, 3, 14, 182, 1, 10, 4, 188, 3, 15, 2,
    1, 0, // 2316
    58, 1, 4, 6,
    1, 0, // 2318
    177, 3, 11, 2,
    1, 0, // 2319
    192, 2, 11, 5,
    3, 0, // 2320
    71, 0, 6, 9, 142, 0, 9, 13, 143, 0, 9, 5,
    31, 0, // 2322
    25, 3, 8, 5, 25, 1, 8, 0, 26, 3, 14, 11, 30, 1, 5, 0, 34, 0, 0, 9, 54, 1, 7, 6, 55, 0, 6, 7, 58, 1, 4, 6, 71, 0, 6, 9, 88, 1, 1, 12, 95, 1, 2, 12, 113, 0, 3, 14, 117, 0, 3, 11, 142, 0, 9, 13, 143, 0, 9, 5, 154, 3, 16, 2, 155, 3, 15, 16, 157, 1, 11, 1, 162, 0, 7, 16, 166, 0, 7, 8, 166, 1, 4, 7, 170, 0, 7, 14, 171, 0, 7, 15, 176, 0, 13, 10, 177, 3, 11, 2, 182, 1, 10, 4, 187, 0, 10, 2, 188, 3, 15, 2, 191, 3, 8, 5, 191, 1, 8, 2, 192, 2, 11, 5,
    3, 0, // 2562
    87, 1, 1, 12, 139, 2, 13, 5, 144, 2, 11, 15,
    1, 0, // 2564
    105, 3, 5, 2,
    4, 0, // 2565
    123, 2, 16, 1, 126, 2, 15, 13, 127, 0, 9, 15, 132, 2, 7, 2,
    5, 0, // 2566
    122, 3, 13, 9, 125, 2, 5, 7, 137, 2, 13, 14, 140, 2, 15, 11, 143, 2, 8, 15,
    1, 0, // 2568
    53, 0, 6, 2,
    2, 0, // 2569
    34, 1, 5, 0, 71, 1, 15, 6,
    1, 0, // 2570
    122, 0, 5, 13,
    1, 0, // 2572
    101, 0, 12, 14,
    1, 0, // 2573
    84, 3, 2, 1,
    2, 0, // 2574
    129, 2, 11, 4, 142, 2, 8, 15,
    4, 0, // 2575
    62, 0, 6, 13, 116, 0, 3, 2, 131, 2, 10, 10, 138, 2, 14, 14,
    2, 0, // 2576
    127, 2, 9, 4, 145, 2, 16, 15,
    1, 0, // 2577
    136, 2, 7, 14,
    28, 0, // 2578
    34, 1, 5, 0, 53, 0, 6, 2, 62, 0, 6, 13, 71, 1, 15, 6, 84, 3, 2, 1, 87, 1, 1, 12, 101, 0, 12, 14, 105, 3, 5, 2, 116, 0, 3, 2, 122, 0, 5, 13, 122, 3, 13, 9, 123, 2, 16, 1, 125, 2, 5, 7, 126, 2, 15, 13, 127, 0, 9, 15, 127, 2, 9, 4, 129, 2, 11, 4, 131, 2, 10, 10, 132, 2, 7, 2, 136, 2, 7, 14, 137, 2, 13, 14, 138, 2, 14, 14, 139, 2, 13, 5, 140, 2, 15, 11, 142, 2, 8, 15, 143, 2, 8, 15, 144, 2, 11, 15, 145, 2, 16, 15,
    1, 0, // 2818
    88, 0, 12, 8,
    3, 0, // 2819
    44, 1, 12, 6, 47, 1, 3, 6, 187, 2, 8, 2,
    2, 0, // 2820
    112, 3, 10, 10, 113, 3, 14, 8,
    2, 0, // 2821
    182, 3, 8, 10, 183, 3, 5, 14,
    2, 0, // 2822
    118, 1, 5, 3, 148, 1, 7, 1,
    1, 0, // 2825
    176, 1, 2, 13,
    1, 0, // 2826
    131, 3, 14, 10,
    4, 0, // 2827
    112, 0, 3, 10, 112, 1, 10, 3, 131, 0, 9, 14, 182, 0, 4, 8,
    2, 0, // 2828
    150, 1, 4, 1, 164, 1, 7, 7,
    3, 0, // 2830
    31, 1, 5, 0, 156, 1, 14, 1, 165, 1, 4, 7,
    2, 0, // 2831
    63, 1, 14, 6, 178, 0, 13, 16,
    3, 0, // 2832
    179, 1, 14, 13, 188, 2, 2, 8, 189, 2, 11, 14,
    1, 0, // 2833
    102, 0, 12, 7,
    27, 0, // 2834
    31, 1, 5, 0, 44, 1, 12, 6, 47, 1, 3, 6, 63, 1, 14, 6, 88, 0, 12, 8, 102, 0, 12, 7, 112, 0, 3, 10, 112, 3, 10, 10, 112, 1, 10, 3, 113, 3, 14, 8, 118, 1, 5, 3, 131, 0, 9, 14, 131, 3, 14, 10, 148, 1, 7, 1, 150, 1, 4, 1, 156, 1, 14, 1, 164, 1, 7, 7, 165, 1, 4, 7, 176, 1, 2, 13, 178, 0, 13, 16, 179, 1, 14, 13, 182, 0, 4, 8, 182, 3, 8, 10, 183, 3, 5, 14, 187, 2, 8, 2, 188, 2, 2, 8, 189, 2, 11, 14,
    1, 0, // 3074
    157, 3, 8, 2,
    2, 0, // 3075
    103, 1, 16, 12, 177, 1, 8, 13,
    2, 0, // 3076
    61, 1, 14, 6, 119, 3, 11, 4,
    7, 0, // 3077
    19, 0, 0, 15, 58, 0, 6, 8, 97, 1, 14, 12, 119, 1, 11, 3, 129, 0, 9, 13, 150, 0, 1, 10, 173, 0, 13, 2,
    1, 0, // 3078
    192, 0, 8, 14,
    4, 0, // 3079
    42, 0, 6, 13, 67, 3, 15, 13, 68, 3, 1, 2, 69, 3, 5, 14,
    2, 0, // 3080
    159, 1, 16, 1, 164, 0, 7, 10,
    2, 0, // 3081
    26, 0, 0, 14, 117, 1, 5, 3,
    1, 0, // 3082
    140, 3, 5, 15,
    2, 0, // 3085
    100, 3, 14, 3, 101, 3, 14, 9,
    4, 0, // 3087
    52, 1, 1, 6, 169, 0, 7, 5, 189, 0, 10, 15, 193, 3, 11, 15,
    3, 0, // 3088
    144, 0, 9, 1, 185, 0, 4, 16, 193, 1, 11, 14,
    3, 0, // 3089
    56, 1, 7, 6, 75, 0, 6, 14, 194, 0, 14, 16,
    34, 0, // 3090
    19, 0, 0, 15, 26, 0, 0, 14, 42, 0, 6, 13, 52, 1, 1, 6, 56, 1, 7, 6, 58, 0, 6, 8, 61, 1, 14, 6, 67, 3, 15, 13, 68, 3, 1, 2, 69, 3, 5, 14, 75, 0, 6, 14, 97, 1, 14, 12, 100, 3, 14, 3, 101, 3, 14, 9, 103, 1, 16, 12, 117, 1, 5, 3, 119, 3, 11, 4, 119, 1, 11, 3, 129, 0, 9, 13, 140, 3, 5, 15, 144, 0, 9, 1, 150, 0, 1, 10, 157, 3, 8, 2, 159, 1, 16, 1, 164, 0, 7, 10, 169, 0, 7, 5, 173, 0, 13, 2, 177, 1, 8, 13, 185, 0, 4, 16, 189, 0, 10, 15, 192, 0, 8, 14, 193, 3, 11, 15, 193, 1, 11, 14, 194, 0, 14, 16,
    1, 0, // 3330
    86, 0, 12, 16,
    1, 0, // 3331
    84, 2, 1, 9,
    1, 0, // 3332
    98, 2, 5, 14,
    2, 0, // 3333
    94, 2, 4, 2, 96, 2, 16, 2,
    1, 0, // 3335
    44, 3, 10, 2,
    3, 0, // 3336
    78, 2, 3, 3, 89, 2, 3, 13, 102, 2, 10, 16,
    2, 0, // 3337
    88, 2, 10, 1, 95, 2, 4, 2,
    1, 0, // 3338
    87, 2, 1, 1,
    2, 0, // 3340
    97, 2, 4, 14, 103, 2, 2, 16,
    1, 0, // 3342
    92, 2, 7, 4,
    2, 0, // 3343
    100, 2, 3, 11, 101, 2, 9, 11,
    1, 0, // 3344
    99, 2, 15, 5,
    2, 0, // 3345
    86, 2, 12, 1, 90, 2, 2, 13,
    20, 0, // 3346
    44, 3, 10, 2, 78, 2, 3, 3, 84, 2, 1, 9, 86, 0, 12, 16, 86, 2, 12, 1, 87, 2, 1, 1, 88, 2, 10, 1, 89, 2, 3, 13, 90, 2, 2, 13, 92, 2, 7, 4, 94, 2, 4, 2, 95, 2, 4, 2, 96, 2, 16, 2, 97, 2, 4, 14, 98, 2, 5, 14, 99, 2, 15, 5, 100, 2, 3, 11, 101, 2, 9, 11, 102, 2, 10, 16, 103, 2, 2, 16,
    4, 0, // 3587
    36, 1, 16, 0, 167, 0, 7, 5, 173, 2, 11, 4, 174, 2, 4, 2,
    1, 0, // 3589
    165, 0, 7, 10,
    3, 0, // 3590
    31, 0, 0, 10, 32, 0, 0, 2, 139, 0, 9, 1,
    3, 0, // 3592
    92, 1, 4, 12, 147, 0, 1, 16, 149, 1, 4, 1,
    1, 0, // 3593
    142, 1, 15, 9,
    3, 0, // 3594
    62, 1, 14, 6, 122, 1, 9, 5, 126, 3, 4, 15,
    2, 0, // 3595
    176, 2, 8, 2, 179, 2, 15, 14,
    4, 0, // 3596
    42, 1, 6, 6, 67, 0, 6, 15, 129, 1, 4, 9, 177, 2, 2, 8,
    2, 0, // 3597
    89, 3, 7, 3, 90, 3, 16, 2,
    2, 0, // 3598
    172, 0, 13, 14, 172, 3, 14, 13,
    5, 0, // 3599
    51, 1, 1, 6, 137, 0, 9, 5, 156, 0, 1, 10, 172, 2, 13, 13, 175, 2, 4, 2,
    2, 0, // 3600
    180, 2, 4, 5, 181, 2, 2, 5,
    1, 0, // 3601
    178, 2, 10, 14,
    33, 0, // 3602
    31, 0, 0, 10, 32, 0, 0, 2, 36, 1, 16, 0, 42, 1, 6, 6, 51, 1, 1, 6, 62, 1, 14, 6, 67, 0, 6, 15, 89, 3, 7, 3, 90, 3, 16, 2, 92, 1, 4, 12, 122, 1, 9, 5, 126, 3, 4, 15, 129, 1, 4, 9, 137, 0, 9, 5, 139, 0, 9, 1, 142, 1, 15, 9, 147, 0, 1, 16, 149, 1, 4, 1, 156, 0, 1, 10, 165, 0, 7, 10, 167, 0, 7, 5, 172, 0, 13, 14, 172, 3, 14, 13, 172, 2, 13, 13, 173, 2, 11, 4, 174, 2, 4, 2, 175, 2, 4, 2, 176, 2, 8, 2, 177, 2, 2, 8, 178, 2, 10, 14, 179, 2, 15, 14, 180, 2, 4, 5, 181, 2, 2, 5,
    3, 0, // 3842
    51, 0, 6, 13, 52, 0, 6, 11, 156, 3, 10, 13,
    3, 0, // 3844
    70, 1, 15, 6, 100, 1, 11, 12, 116, 3, 2, 9,
    2, 0, // 3845
    18, 0, 0, 1, 175, 1, 2, 13,
    3, 0, // 3847
    61, 3, 11, 3, 62, 3, 13, 9, 63, 3, 10, 14,
    3, 0, // 3848
    163, 0, 7, 4, 168, 3, 16, 4, 169, 3, 5, 11,
    2, 0, // 3849
    113, 1, 10, 3, 170, 1, 5, 7,
    4, 0, // 3850
    101, 1, 11, 12, 136, 3, 16, 7, 137, 3, 5, 13, 138, 3, 14, 14,
    3, 0, // 3851
    131, 1, 10, 9, 183, 0, 4, 5, 189, 3, 15, 11,
    5, 0, // 3852
    26, 1, 8, 0, 69, 0, 6, 5, 75, 1, 16, 6, 192, 1, 5, 8, 193, 2, 15, 11,
    2, 0, // 3853
    97, 3, 11, 4, 98, 3, 3, 5,
    3, 0, // 3854
    172, 1, 13, 13, 178, 3, 16, 10, 179, 3, 10, 15,
    3, 0, // 3855
    63, 0, 6, 10, 138, 0, 9, 14, 138, 1, 14, 9,
    3, 0, // 3857
    37, 0, 0, 5, 184, 1, 2, 4, 194, 2, 11, 16,
    39, 0, // 3858
    18, 0, 0, 1, 26, 1, 8, 0, 37, 0, 0, 5, 51, 0, 6, 13, 52, 0, 6, 11, 61, 3, 11, 3, 62, 3, 13, 9, 63, 0, 6, 10, 63, 3, 10, 14, 69, 0, 6, 5, 70, 1, 15, 6, 75, 1, 16, 6, 97, 3, 11, 4, 98, 3, 3, 5, 100, 1, 11, 12, 101, 1, 11, 12, 113, 1, 10, 3, 116, 3, 2, 9, 131, 1, 10, 9, 136, 3, 16, 7, 137, 3, 5, 13, 138, 0, 9, 14, 138, 3, 14, 14, 138, 1, 14, 9, 156, 3, 10, 13, 163, 0, 7, 4, 168, 3, 16, 4, 169, 3, 5, 11, 170, 1, 5, 7, 172, 1, 13, 13, 175, 1, 2, 13, 178, 3, 16, 10, 179, 3, 10, 15, 183, 0, 4, 5, 184, 1, 2, 4, 189, 3, 15, 11, 192, 1, 5, 8, 193, 2, 15, 11, 194, 2, 11, 16,
    1, 0, // 4097
    35, 3, 1, 15,
    1, 0, // 4098
    158, 3, 2, 7,
    2, 0, // 4099
    181, 1, 5, 13, 188, 1, 8, 10,
    1, 0, // 4100
    121, 0, 7, 4,
    3, 0, // 4101
    114, 1, 2, 3, 180, 1, 5, 13, 185, 3, 16, 11,
    1, 0, // 4102
    99, 0, 12, 15,
    2, 0, // 4103
    70, 3, 14, 3, 71, 3, 9, 8,
    1, 0, // 4105
    171, 1, 5, 7,
    5, 0, // 4106
    127, 1, 4, 9, 142, 3, 13, 8, 143, 3, 5, 8, 144, 3, 1, 11, 145, 3, 15, 16,
    4, 0, // 4108
    19, 1, 4, 0, 140, 0, 9, 5, 189, 1, 14, 10, 193, 0, 14, 11,
    2, 0, // 4110
    67, 1, 11, 6, 126, 0, 9, 4,
    1, 0, // 4111
    179, 0, 13, 10,
    3, 0, // 4112
    35, 0, 0, 1, 99, 1, 5, 12, 160, 1, 16, 1,
    7, 0, // 4113
    38, 0, 0, 3, 39, 1, 16, 0, 145, 1, 15, 9, 155, 1, 8, 1, 160, 0, 1, 15, 161, 0, 1, 16, 195, 0, 5, 16,
    34, 0, // 4114
    19, 1, 4, 0, 35, 0, 0, 1, 35, 3, 1, 15, 38, 0, 0, 3, 39, 1, 16, 0, 67, 1, 11, 6, 70, 3, 14, 3, 71, 3, 9, 8, 99, 0, 12, 15, 99, 1, 5, 12, 114, 1, 2, 3, 121, 0, 7, 4, 126, 0, 9, 4, 127, 1, 4, 9, 140, 0, 9, 5, 142, 3, 13, 8, 143, 3, 5, 8, 144, 3, 1, 11, 145, 3, 15, 16, 145, 1, 15, 9, 155, 1, 8, 1, 158, 3, 2, 7, 160, 0, 1, 15, 160, 1, 16, 1, 161, 0, 1, 16, 171, 1, 5, 7, 179, 0, 13, 10, 180, 1, 5, 13, 181, 1, 5, 13, 185, 3, 16, 11, 188, 1, 8, 10, 189, 1, 14, 10, 193, 0, 14, 11, 195, 0, 5, 16,
    4, 0, // 4353
    36, 3, 13, 2, 37, 3, 5, 14, 38, 3, 3, 15, 39, 3, 15, 16,
    5, 0, // 4354
    123, 0, 9, 4, 146, 0, 1, 4, 159, 3, 11, 7, 160, 3, 15, 15, 161, 3, 16, 15,
    5, 0, // 4355
    90, 1, 13, 12, 96, 0, 12, 4, 154, 1, 8, 1, 184, 0, 4, 14, 190, 0, 2, 5,
    1, 0, // 4357
    168, 1, 14, 7,
    1, 0, // 4358
    195, 3, 16, 15,
    2, 0, // 4359
    43, 0, 6, 3, 75, 3, 14, 11,
    2, 0, // 4360
    56, 0, 6, 11, 136, 1, 14, 9,
    2, 0, // 4361
    155, 0, 1, 15, 162, 1, 7, 7,
    1, 0, // 4363
    178, 1, 14, 13,
    2, 0, // 4364
    185, 1, 15, 4, 194, 1, 16, 14,
    3, 0, // 4365
    86, 1, 1, 12, 102, 3, 7, 10, 103, 3, 11, 2,
    1, 0, // 4366
    147, 1, 7, 1,
    1, 0, // 4367
    194, 3, 16, 11,
    3, 0, // 4368
    145, 0, 9, 15, 161, 1, 16, 1, 195, 1, 16, 5,
    1, 0, // 4369
    39, 0, 0, 15,
    34, 0, // 4370
    36, 3, 13, 2, 37, 3, 5, 14, 38, 3, 3, 15, 39, 0, 0, 15, 39, 3, 15, 16, 43, 0, 6, 3, 56, 0, 6, 11, 75, 3, 14, 11, 86, 1, 1, 12, 90, 1, 13, 12, 96, 0, 12, 4, 102, 3, 7, 10, 103, 3, 11, 2, 123, 0, 9, 4, 136, 1, 14, 9, 145, 0, 9, 15, 146, 0, 1, 4, 147, 1, 7, 1, 154, 1, 8, 1, 155, 0, 1, 15, 159, 3, 11, 7, 160, 3, 15, 15, 161, 3, 16, 15, 161, 1, 16, 1, 162, 1, 7, 7, 168, 1, 14, 7, 178, 1, 14, 13, 184, 0, 4, 14, 185, 1, 15, 4, 190, 0, 2, 5, 194, 3, 16, 11, 194, 1, 16, 14, 195, 3, 16, 15, 195, 1, 16, 5,
    // prefillTiles
    0, 0, 0, 12, 3, 0, 0, 5, 8, 2, 16, 12, 4, 0, 0, 16, 28, 2, 3, 14, 7, 1, 12, 0, 1, 0, 0, 4, 9, 2, 1, 1, 2, 2, 6, 6, 6, 2, 7, 12, 5, 0, 0, 9, 11, 1, 7, 0, 10, 3, 8, 4, 17, 0, 0, 11, 15, 2, 9, 13, 23, 2, 12, 8, 135, 0, 9, 10, 13, 2, 14, 7, 111, 2, 15, 4, 76, 2, 14, 12, 20, 2, 13, 10, 130, 0, 9, 1, 74, 2, 2, 16, 73, 0, 6, 2, 12, 2, 10, 7, 91, 1, 4, 12, 85, 1, 1, 12, 21, 2, 12, 2, 50, 3, 10, 8, 79, 2, 3, 3, 24, 1, 8, 0, 29, 3, 11, 2, 33, 1, 5, 0, 27, 3, 15, 0, 16, 3, 10, 13, 128, 2, 1, 4, 109, 0, 3, 5, 45, 2, 1, 3, 81, 0, 12, 3, 110, 3, 11, 14, 153, 2, 1, 8, 82, 2, 11, 3, 40, 1, 6, 6, 46, 3, 5, 13, 124, 1, 7, 9, 72, 1, 16, 6, 64, 3, 16, 8, 133, 0, 9, 7, 107, 1, 13, 3, 77, 1, 3, 12, 80, 3, 15, 13, 57, 0, 6, 15, 83, 2, 15, 3, 48, 0, 6, 7, 106, 0, 3, 1, 151, 1, 10, 1, 41, 2, 8, 6, 65, 3, 10, 15, 120, 0, 3, 11, 141, 0, 9, 4, 93, 0, 12, 11, 66, 0, 6, 3, 108, 3, 15, 13, 115, 2, 4, 8, 134, 0, 9, 16, 59, 2, 5, 10, 186, 0, 10, 16, 60, 2, 9, 2, 104, 2, 1, 9, 49, 0, 6, 8,
];

pub const PREFILL_DEPTH: usize = 70;

pub const PREFILL_TILES_OFFSET: [u16; 70] = [
    4516, 4520, 4524, 4528, 4532, 4536, 4540, 4544, 4548, 4552, 4556, 4560, 4564, 4568, 4572, 4576, 4580, 4584, 4588, 4592, 4596, 4600, 4604, 4608, 4612, 4616, 4620, 4624, 4628, 4632, 4636, 4640, 4644, 4648, 4652, 4656, 4660, 4664, 4668, 4672, 4676, 4680, 4684, 4688, 4692, 4696, 4700, 4704, 4708, 4712, 4716, 4720, 4724, 4728, 4732, 4736, 4740, 4744, 4748, 4752, 4756, 4760, 4764, 4768, 4772, 4776, 4780, 4784, 4788, 4792, 
];
