
use crate::celltype::{
    CellType, MID,
    MID_LEFT, MID_TOP, MID_TOP_LEFT,
};
use crate::ori::{Ori, ANY};

pub const NUM_TILES: usize = 256;
pub const NUM_ROWS: usize = 16;
pub const NUM_COLS: usize = 16;

// pub const NUM_CORNERS: u32 = 4;
// pub const NUM_EDGES: u32 = 56;
pub const NUM_MIDS: usize = 196;

// pub const NUM_COLOURS: u32 = 22;
// pub const NUM_COLOURS_INC_GREY: u32 = 23;
// pub const NUM_EDGE_COLOURS: u32 = 5;
// pub const NUM_MID_COLOURS: u32 = 17;

pub const PLACE_MIDS_ONLY: bool = true;
pub const ANY_COLOUR: u8 = 17;

const INVALID_CELL_IDX: u8 = 255;

#[derive(Debug)]
pub struct Cell {
    pub north_idx: u8,
    pub west_idx: u8,
    pub cell_type: CellType,
    pub ori: Ori,
}

pub const FILL_ORDER: [Cell; 196] = [
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

pub const DISPLAY_TO_FILL_ORDER: [i16; NUM_TILES] = [
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, -1, -1, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, -1, -1, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, -1, -1, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, -1, -1, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, -1, -1, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, -1, -1, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, -1, -1, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, -1, -1, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, -1, -1, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, -1, -1, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, -1, -1, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, -1, -1, 168, 169, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 180, 181, -1, -1, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192, 193, 194, 195, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 
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

pub static TOP_LEFT_CORNER_OFFSET: u16 = 0; // unused
pub static TOP_RIGHT_CORNER_COLOUR_ARRAY: [u16; 0] = []; // unused
pub static BOTTOM_LEFT_CORNER_COLOUR_ARRAY: [u16; 0] = []; // unused
pub static BOTTOM_RIGHT_CORNER_BICOLOUR_ARRAY: [[u16; 0]; 0] = []; // unused
pub static RIGHT_EDGES_BICOLOUR_ARRAY: [[u16; 0]; 0] = []; // unused
pub static BOTTOM_EDGES_BICOLOUR_ARRAY: [[u16; 0]; 0] = []; // unused
pub static TOP_EDGES_COLOUR_ARRAY: [u16; 0] = []; // unused
pub static LEFT_EDGES_COLOUR_ARRAY: [u16; 0] = []; // unused

pub static MIDS_BICOLOUR_ARRAY: [[u16; 18]; 18] = [
    [2, 12, 30, 44, 54, 64, 78, 0, 84, 98, 112, 130, 140, 162, 172, 182, 200, 210, ], // north 1
    [388, 398, 420, 434, 444, 458, 468, 482, 488, 498, 512, 534, 548, 566, 580, 590, 604, 618, ], // north 2
    [816, 826, 840, 846, 856, 870, 884, 890, 908, 930, 948, 958, 972, 990, 1012, 0, 1018, 1032, ], // north 3
    [1218, 1224, 1230, 1240, 0, 1254, 1268, 1286, 1296, 1302, 1308, 1326, 1348, 1378, 1388, 1406, 1424, 1430, ], // north 4
    [1612, 1626, 1644, 1666, 1680, 1686, 1696, 1702, 1720, 1730, 0, 1748, 1754, 1768, 1774, 1796, 1806, 1828, ], // north 5
    [2014, 2040, 2046, 2060, 0, 2070, 2084, 2094, 2116, 2134, 2140, 2146, 2152, 2158, 2184, 2202, 2208, 2222, ], // north 6
    [2400, 2406, 2416, 2426, 0, 2436, 2450, 2472, 2486, 2500, 2506, 2532, 0, 2554, 2568, 2578, 2588, 2602, ], // north 7
    [2776, 2794, 2804, 2814, 2828, 2850, 2864, 2882, 2900, 2910, 2920, 2934, 2940, 2946, 2952, 2962, 2984, 3002, ], // north 8
    [3196, 3214, 3228, 3246, 3256, 3274, 3304, 3314, 3328, 3334, 3352, 3374, 3384, 3390, 3400, 3406, 0, 3420, ], // north 9
    [3614, 3620, 3638, 3644, 3658, 3680, 3702, 3712, 3726, 3740, 3750, 3756, 3766, 3776, 3794, 3812, 3822, 3836, ], // north 10
    [4026, 4032, 4042, 4056, 4070, 4080, 4090, 4096, 4106, 4120, 4138, 4160, 0, 4170, 4188, 4202, 4220, 4234, ], // north 11
    [0, 4412, 4426, 4440, 4458, 4488, 4494, 4516, 4530, 4544, 0, 0, 4558, 0, 4568, 4590, 4604, 4622, ], // north 12
    [4808, 4834, 4848, 4858, 4872, 0, 4886, 4892, 4910, 4936, 0, 4946, 4960, 4974, 4984, 4994, 5008, 5018, ], // north 13
    [5200, 5210, 5216, 5234, 5252, 5262, 5276, 5282, 5300, 5306, 5320, 5334, 5352, 5366, 5384, 5406, 5416, 5422, ], // north 14
    [5612, 5626, 0, 5644, 5658, 0, 5672, 5686, 5704, 5714, 5732, 5746, 5768, 5786, 5800, 0, 5814, 5828, ], // north 15
    [6018, 6028, 6038, 6052, 6066, 6084, 6094, 0, 6104, 6118, 6140, 6146, 0, 6164, 6182, 6188, 6206, 6236, ], // north 16
    [6426, 6444, 6466, 6488, 6494, 6504, 6514, 6536, 6546, 6564, 6570, 6576, 6590, 6612, 6618, 6624, 6646, 6652, ], // north 17
    [6846, 7024, 7222, 7408, 7590, 7776, 7954, 8128, 8322, 8516, 8706, 8884, 9070, 9252, 9442, 9632, 9822, 10016, ], // north ANY
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
    60, 3, 12, 3, 61, 3, 4, 9,
    4, 0, // 258
    62, 2, 6, 6, 74, 2, 5, 7, 78, 2, 14, 4, 95, 2, 15, 15,
    3, 0, // 259
    72, 2, 10, 7, 82, 2, 7, 2, 92, 2, 13, 5,
    2, 0, // 260
    67, 2, 14, 12, 98, 2, 15, 16,
    2, 0, // 261
    61, 2, 9, 0, 69, 2, 1, 1,
    3, 0, // 262
    63, 2, 0, 12, 68, 2, 16, 12, 97, 2, 14, 16,
    1, 0, // 263
    66, 2, 7, 12,
    3, 0, // 265
    70, 2, 4, 1, 85, 2, 5, 8, 90, 2, 7, 5,
    3, 0, // 266
    65, 2, 9, 12, 84, 2, 3, 8, 94, 2, 8, 5,
    4, 0, // 267
    73, 2, 14, 7, 75, 2, 9, 13, 76, 2, 13, 13, 91, 2, 13, 5,
    2, 0, // 268
    77, 2, 1, 4, 89, 2, 2, 14,
    5, 0, // 269
    60, 2, 3, 0, 63, 0, 0, 5, 71, 2, 9, 7, 80, 2, 13, 10, 81, 2, 12, 2,
    2, 0, // 270
    83, 2, 12, 8, 96, 2, 2, 16,
    2, 0, // 271
    86, 2, 11, 8, 87, 0, 0, 15,
    4, 0, // 272
    79, 2, 11, 4, 87, 2, 0, 14, 93, 2, 2, 5, 99, 2, 16, 16,
    2, 0, // 273
    64, 2, 12, 12, 88, 2, 3, 14,
    44, 0, // 274
    60, 3, 12, 3, 60, 2, 3, 0, 61, 3, 4, 9, 61, 2, 9, 0, 62, 2, 6, 6, 63, 0, 0, 5, 63, 2, 0, 12, 64, 2, 12, 12, 65, 2, 9, 12, 66, 2, 7, 12, 67, 2, 14, 12, 68, 2, 16, 12, 69, 2, 1, 1, 70, 2, 4, 1, 71, 2, 9, 7, 72, 2, 10, 7, 73, 2, 14, 7, 74, 2, 5, 7, 75, 2, 9, 13, 76, 2, 13, 13, 77, 2, 1, 4, 78, 2, 14, 4, 79, 2, 11, 4, 80, 2, 13, 10, 81, 2, 12, 2, 82, 2, 7, 2, 83, 2, 12, 8, 84, 2, 3, 8, 85, 2, 5, 8, 86, 2, 11, 8, 87, 0, 0, 15, 87, 2, 0, 14, 88, 2, 3, 14, 89, 2, 2, 14, 90, 2, 7, 5, 91, 2, 13, 5, 92, 2, 13, 5, 93, 2, 2, 5, 94, 2, 8, 5, 95, 2, 15, 15, 96, 2, 2, 16, 97, 2, 14, 16, 98, 2, 15, 16, 99, 2, 16, 16,
    2, 0, // 513
    69, 3, 4, 1, 70, 3, 8, 4,
    5, 0, // 514
    69, 0, 0, 4, 147, 0, 12, 9, 166, 1, 7, 3, 190, 1, 10, 9, 206, 3, 4, 16,
    3, 0, // 515
    128, 1, 11, 6, 134, 1, 16, 6, 218, 2, 7, 15,
    2, 0, // 516
    105, 0, 6, 5, 137, 1, 3, 12,
    3, 0, // 517
    77, 0, 0, 11, 188, 0, 9, 13, 206, 2, 16, 1,
    2, 0, // 518
    74, 1, 7, 0, 212, 2, 5, 2,
    3, 0, // 519
    62, 1, 6, 0, 111, 3, 13, 14, 112, 3, 11, 14,
    1, 0, // 520
    166, 0, 3, 1,
    2, 0, // 521
    213, 0, 1, 14, 217, 2, 2, 11,
    3, 0, // 522
    144, 0, 12, 2, 164, 0, 3, 2, 183, 3, 4, 16,
    5, 0, // 523
    190, 0, 9, 1, 208, 2, 5, 7, 210, 2, 11, 4, 211, 0, 1, 11, 216, 2, 13, 14,
    3, 0, // 524
    204, 1, 15, 9, 211, 2, 1, 10, 219, 2, 7, 16,
    4, 0, // 525
    145, 3, 8, 12, 146, 3, 16, 12, 147, 3, 9, 1, 148, 3, 8, 10,
    3, 0, // 526
    117, 0, 6, 15, 199, 1, 5, 9, 209, 2, 7, 4,
    2, 0, // 527
    78, 1, 4, 0, 213, 2, 1, 8,
    3, 0, // 528
    95, 1, 15, 0, 215, 2, 16, 8, 220, 2, 15, 16,
    3, 0, // 529
    207, 2, 13, 7, 214, 2, 2, 8, 221, 2, 15, 16,
    49, 0, // 530
    62, 1, 6, 0, 69, 0, 0, 4, 69, 3, 4, 1, 70, 3, 8, 4, 74, 1, 7, 0, 77, 0, 0, 11, 78, 1, 4, 0, 95, 1, 15, 0, 105, 0, 6, 5, 111, 3, 13, 14, 112, 3, 11, 14, 117, 0, 6, 15, 128, 1, 11, 6, 134, 1, 16, 6, 137, 1, 3, 12, 144, 0, 12, 2, 145, 3, 8, 12, 146, 3, 16, 12, 147, 0, 12, 9, 147, 3, 9, 1, 148, 3, 8, 10, 164, 0, 3, 2, 166, 0, 3, 1, 166, 1, 7, 3, 183, 3, 4, 16, 188, 0, 9, 13, 190, 0, 9, 1, 190, 1, 10, 9, 199, 1, 5, 9, 204, 1, 15, 9, 206, 3, 4, 16, 206, 2, 16, 1, 207, 2, 13, 7, 208, 2, 5, 7, 209, 2, 7, 4, 210, 2, 11, 4, 211, 0, 1, 11, 211, 2, 1, 10, 212, 2, 5, 2, 213, 0, 1, 14, 213, 2, 1, 8, 214, 2, 2, 8, 215, 2, 16, 8, 216, 2, 13, 14, 217, 2, 2, 11, 218, 2, 7, 15, 219, 2, 7, 16, 220, 2, 15, 16, 221, 2, 15, 16,
    2, 0, // 769
    81, 3, 12, 12, 82, 3, 2, 7,
    3, 0, // 770
    144, 1, 9, 12, 164, 1, 9, 3, 212, 3, 5, 5,
    1, 0, // 771
    250, 3, 5, 16,
    2, 0, // 772
    107, 0, 6, 10, 174, 3, 15, 4,
    3, 0, // 773
    169, 0, 3, 5, 234, 1, 2, 13, 244, 3, 14, 16,
    3, 0, // 774
    93, 0, 0, 15, 241, 0, 13, 15, 250, 2, 16, 2,
    1, 0, // 775
    120, 3, 16, 9,
    4, 0, // 776
    82, 1, 2, 0, 133, 1, 16, 6, 218, 1, 15, 1, 227, 3, 5, 13,
    5, 0, // 777
    214, 0, 1, 16, 237, 0, 13, 11, 247, 1, 2, 10, 248, 0, 10, 15, 251, 2, 5, 8,
    4, 0, // 778
    113, 1, 7, 6, 165, 0, 3, 5, 176, 1, 14, 3, 192, 3, 4, 7,
    2, 0, // 779
    72, 1, 7, 0, 247, 3, 2, 8,
    3, 0, // 780
    128, 0, 6, 1, 217, 0, 1, 8, 233, 1, 4, 13,
    4, 0, // 781
    104, 0, 6, 10, 154, 3, 4, 4, 155, 3, 8, 4, 156, 3, 4, 16,
    5, 0, // 782
    92, 1, 5, 0, 150, 0, 12, 16, 234, 3, 2, 4, 235, 3, 14, 4, 236, 3, 10, 8,
    1, 0, // 783
    89, 0, 0, 11,
    3, 0, // 785
    96, 0, 0, 13, 134, 0, 6, 1, 163, 0, 12, 11,
    46, 0, // 786
    72, 1, 7, 0, 81, 3, 12, 12, 82, 3, 2, 7, 82, 1, 2, 0, 89, 0, 0, 11, 92, 1, 5, 0, 93, 0, 0, 15, 96, 0, 0, 13, 104, 0, 6, 10, 107, 0, 6, 10, 113, 1, 7, 6, 120, 3, 16, 9, 128, 0, 6, 1, 133, 1, 16, 6, 134, 0, 6, 1, 144, 1, 9, 12, 150, 0, 12, 16, 154, 3, 4, 4, 155, 3, 8, 4, 156, 3, 4, 16, 163, 0, 12, 11, 164, 1, 9, 3, 165, 0, 3, 5, 169, 0, 3, 5, 174, 3, 15, 4, 176, 1, 14, 3, 192, 3, 4, 7, 212, 3, 5, 5, 214, 0, 1, 16, 217, 0, 1, 8, 218, 1, 15, 1, 227, 3, 5, 13, 233, 1, 4, 13, 234, 3, 2, 4, 234, 1, 2, 13, 235, 3, 14, 4, 236, 3, 10, 8, 237, 0, 13, 11, 241, 0, 13, 15, 244, 3, 14, 16, 247, 3, 2, 8, 247, 1, 2, 10, 248, 0, 10, 15, 250, 3, 5, 16, 250, 2, 16, 2, 251, 2, 5, 8,
    1, 0, // 1025
    60, 0, 0, 12,
    1, 0, // 1026
    166, 2, 1, 7,
    2, 0, // 1027
    164, 2, 1, 9, 176, 2, 9, 14,
    3, 0, // 1028
    137, 0, 12, 1, 138, 0, 12, 7, 139, 0, 12, 8,
    3, 0, // 1030
    158, 1, 14, 12, 165, 2, 2, 9, 169, 2, 2, 4,
    4, 0, // 1031
    105, 3, 5, 1, 106, 3, 5, 13, 107, 3, 10, 2, 108, 3, 7, 11,
    2, 0, // 1032
    171, 2, 15, 4, 181, 3, 4, 15,
    1, 0, // 1033
    84, 0, 0, 9,
    1, 0, // 1034
    126, 1, 11, 6,
    4, 0, // 1035
    141, 1, 3, 12, 167, 2, 7, 13, 172, 2, 10, 10, 178, 2, 5, 5,
    5, 0, // 1036
    160, 0, 12, 14, 170, 2, 14, 4, 177, 2, 8, 5, 179, 2, 4, 11, 180, 2, 7, 15,
    7, 0, // 1037
    137, 3, 1, 3, 138, 3, 7, 3, 139, 3, 8, 3, 140, 3, 15, 13, 141, 3, 3, 10, 142, 3, 8, 11, 143, 3, 15, 15,
    2, 0, // 1038
    149, 0, 12, 7, 175, 2, 4, 8,
    4, 0, // 1039
    67, 1, 12, 0, 88, 0, 0, 16, 121, 0, 6, 11, 173, 2, 8, 10,
    4, 0, // 1040
    98, 1, 16, 0, 130, 0, 6, 14, 168, 2, 13, 13, 174, 2, 4, 2,
    1, 0, // 1041
    103, 1, 6, 6,
    45, 0, // 1042
    60, 0, 0, 12, 67, 1, 12, 0, 84, 0, 0, 9, 88, 0, 0, 16, 98, 1, 16, 0, 103, 1, 6, 6, 105, 3, 5, 1, 106, 3, 5, 13, 107, 3, 10, 2, 108, 3, 7, 11, 121, 0, 6, 11, 126, 1, 11, 6, 130, 0, 6, 14, 137, 0, 12, 1, 137, 3, 1, 3, 138, 0, 12, 7, 138, 3, 7, 3, 139, 0, 12, 8, 139, 3, 8, 3, 140, 3, 15, 13, 141, 3, 3, 10, 141, 1, 3, 12, 142, 3, 8, 11, 143, 3, 15, 15, 149, 0, 12, 7, 158, 1, 14, 12, 160, 0, 12, 14, 164, 2, 1, 9, 165, 2, 2, 9, 166, 2, 1, 7, 167, 2, 7, 13, 168, 2, 13, 13, 169, 2, 2, 4, 170, 2, 14, 4, 171, 2, 15, 4, 172, 2, 10, 10, 173, 2, 8, 10, 174, 2, 4, 2, 175, 2, 4, 8, 176, 2, 9, 14, 177, 2, 8, 5, 178, 2, 5, 5, 179, 2, 4, 11, 180, 2, 7, 15, 181, 3, 4, 15,
    3, 0, // 1281
    77, 3, 11, 1, 78, 3, 1, 14, 79, 3, 15, 11,
    4, 0, // 1282
    69, 1, 1, 0, 70, 0, 0, 8, 209, 3, 13, 7, 210, 3, 10, 11,
    5, 0, // 1283
    154, 0, 12, 4, 155, 0, 12, 8, 174, 0, 3, 15, 234, 0, 13, 2, 235, 0, 13, 14,
    3, 0, // 1284
    169, 3, 5, 2, 170, 3, 11, 14, 171, 3, 7, 15,
    1, 0, // 1285
    154, 1, 2, 12,
    2, 0, // 1286
    240, 0, 13, 15, 243, 2, 14, 10,
    1, 0, // 1287
    118, 3, 8, 11,
    4, 0, // 1288
    114, 0, 6, 8, 192, 1, 2, 9, 225, 3, 10, 13, 226, 3, 8, 8,
    2, 0, // 1289
    175, 0, 3, 13, 242, 2, 10, 10,
    4, 0, // 1290
    61, 1, 0, 0, 187, 3, 15, 9, 188, 3, 13, 1, 189, 3, 13, 11,
    1, 0, // 1292
    179, 0, 3, 11,
    3, 0, // 1293
    151, 3, 7, 7, 152, 3, 13, 7, 153, 3, 11, 16,
    1, 0, // 1294
    233, 3, 2, 11,
    5, 0, // 1295
    136, 1, 12, 12, 157, 0, 12, 11, 223, 1, 7, 7, 228, 0, 7, 16, 244, 2, 16, 2,
    2, 0, // 1296
    181, 1, 3, 7, 186, 1, 13, 9,
    5, 0, // 1297
    156, 1, 2, 12, 183, 1, 1, 9, 201, 1, 11, 9, 206, 1, 1, 1, 245, 2, 11, 15,
    46, 0, // 1298
    61, 1, 0, 0, 69, 1, 1, 0, 70, 0, 0, 8, 77, 3, 11, 1, 78, 3, 1, 14, 79, 3, 15, 11, 114, 0, 6, 8, 118, 3, 8, 11, 136, 1, 12, 12, 151, 3, 7, 7, 152, 3, 13, 7, 153, 3, 11, 16, 154, 0, 12, 4, 154, 1, 2, 12, 155, 0, 12, 8, 156, 1, 2, 12, 157, 0, 12, 11, 169, 3, 5, 2, 170, 3, 11, 14, 171, 3, 7, 15, 174, 0, 3, 15, 175, 0, 3, 13, 179, 0, 3, 11, 181, 1, 3, 7, 183, 1, 1, 9, 186, 1, 13, 9, 187, 3, 15, 9, 188, 3, 13, 1, 189, 3, 13, 11, 192, 1, 2, 9, 201, 1, 11, 9, 206, 1, 1, 1, 209, 3, 13, 7, 210, 3, 10, 11, 223, 1, 7, 7, 225, 3, 10, 13, 226, 3, 8, 8, 228, 0, 7, 16, 233, 3, 2, 11, 234, 0, 13, 2, 235, 0, 13, 14, 240, 0, 13, 15, 242, 2, 10, 10, 243, 2, 14, 10, 244, 2, 16, 2, 245, 2, 11, 15,
    6, 0, // 1537
    63, 1, 12, 0, 90, 3, 8, 7, 91, 3, 10, 13, 92, 3, 2, 13, 93, 3, 15, 2, 94, 3, 9, 8,
    1, 0, // 1538
    105, 1, 3, 6,
    3, 0, // 1539
    165, 1, 9, 3, 169, 1, 4, 3, 212, 0, 1, 5,
    2, 0, // 1540
    177, 3, 11, 8, 178, 3, 10, 5,
    3, 0, // 1542
    178, 0, 3, 10, 185, 1, 7, 9, 212, 1, 2, 1,
    2, 0, // 1543
    124, 3, 16, 8, 125, 3, 10, 15,
    5, 0, // 1544
    74, 0, 0, 1, 185, 0, 9, 5, 208, 0, 1, 10, 230, 3, 14, 8, 231, 3, 15, 8,
    4, 0, // 1545
    85, 0, 0, 8, 203, 1, 15, 9, 251, 0, 2, 8, 252, 3, 14, 11,
    1, 0, // 1546
    199, 3, 1, 13,
    1, 0, // 1547
    119, 0, 6, 16,
    1, 0, // 1548
    229, 1, 14, 7,
    1, 0, // 1549
    159, 3, 15, 15,
    6, 0, // 1550
    106, 1, 3, 6, 182, 2, 9, 9, 197, 1, 14, 9, 227, 1, 2, 7, 240, 3, 15, 4, 241, 3, 15, 2,
    4, 0, // 1551
    97, 1, 16, 0, 129, 1, 11, 6, 158, 0, 12, 3, 243, 1, 10, 4,
    1, 0, // 1552
    200, 1, 11, 9,
    3, 0, // 1553
    68, 1, 12, 0, 250, 1, 2, 2, 255, 2, 15, 16,
    44, 0, // 1554
    63, 1, 12, 0, 68, 1, 12, 0, 74, 0, 0, 1, 85, 0, 0, 8, 90, 3, 8, 7, 91, 3, 10, 13, 92, 3, 2, 13, 93, 3, 15, 2, 94, 3, 9, 8, 97, 1, 16, 0, 105, 1, 3, 6, 106, 1, 3, 6, 119, 0, 6, 16, 124, 3, 16, 8, 125, 3, 10, 15, 129, 1, 11, 6, 158, 0, 12, 3, 159, 3, 15, 15, 165, 1, 9, 3, 169, 1, 4, 3, 177, 3, 11, 8, 178, 0, 3, 10, 178, 3, 10, 5, 182, 2, 9, 9, 185, 0, 9, 5, 185, 1, 7, 9, 197, 1, 14, 9, 199, 3, 1, 13, 200, 1, 11, 9, 203, 1, 15, 9, 208, 0, 1, 10, 212, 0, 1, 5, 212, 1, 2, 1, 227, 1, 2, 7, 229, 1, 14, 7, 230, 3, 14, 8, 231, 3, 15, 8, 240, 3, 15, 4, 241, 3, 15, 2, 243, 1, 10, 4, 250, 1, 2, 2, 251, 0, 2, 8, 252, 3, 14, 11, 255, 2, 15, 16,
    1, 0, // 1793
    62, 3, 1, 6,
    2, 0, // 1794
    128, 2, 2, 11, 134, 2, 2, 16,
    2, 0, // 1795
    113, 2, 9, 7, 133, 2, 7, 16,
    2, 0, // 1796
    103, 2, 16, 6, 126, 2, 9, 11,
    3, 0, // 1798
    105, 2, 1, 3, 106, 2, 13, 3, 129, 2, 14, 11,
    5, 0, // 1799
    62, 0, 0, 1, 100, 3, 10, 8, 101, 3, 11, 8, 102, 3, 13, 11, 103, 3, 3, 16,
    3, 0, // 1800
    66, 1, 12, 0, 108, 2, 11, 3, 115, 2, 8, 7,
    3, 0, // 1801
    109, 2, 10, 9, 114, 2, 4, 7, 118, 2, 11, 4,
    1, 0, // 1802
    131, 2, 8, 15,
    6, 0, // 1803
    100, 2, 8, 6, 104, 2, 2, 12, 107, 2, 2, 3, 110, 2, 8, 9, 123, 2, 14, 14, 125, 2, 15, 5,
    5, 0, // 1804
    101, 2, 8, 6, 112, 2, 14, 1, 116, 2, 16, 7, 121, 2, 3, 14, 132, 2, 9, 16,
    3, 0, // 1806
    102, 2, 11, 6, 111, 2, 14, 1, 122, 2, 9, 14,
    2, 0, // 1807
    130, 2, 3, 15, 135, 2, 11, 16,
    2, 0, // 1808
    117, 2, 1, 13, 127, 2, 13, 11,
    3, 0, // 1809
    119, 2, 5, 10, 120, 2, 9, 2, 124, 2, 8, 5,
    43, 0, // 1810
    62, 0, 0, 1, 62, 3, 1, 6, 66, 1, 12, 0, 100, 3, 10, 8, 100, 2, 8, 6, 101, 3, 11, 8, 101, 2, 8, 6, 102, 3, 13, 11, 102, 2, 11, 6, 103, 3, 3, 16, 103, 2, 16, 6, 104, 2, 2, 12, 105, 2, 1, 3, 106, 2, 13, 3, 107, 2, 2, 3, 108, 2, 11, 3, 109, 2, 10, 9, 110, 2, 8, 9, 111, 2, 14, 1, 112, 2, 14, 1, 113, 2, 9, 7, 114, 2, 4, 7, 115, 2, 8, 7, 116, 2, 16, 7, 117, 2, 1, 13, 118, 2, 11, 4, 119, 2, 5, 10, 120, 2, 9, 2, 121, 2, 3, 14, 122, 2, 9, 14, 123, 2, 14, 14, 124, 2, 8, 5, 125, 2, 15, 5, 126, 2, 9, 11, 127, 2, 13, 11, 128, 2, 2, 11, 129, 2, 14, 11, 130, 2, 3, 15, 131, 2, 8, 15, 132, 2, 9, 16, 133, 2, 7, 16, 134, 2, 2, 16, 135, 2, 11, 16,
    4, 0, // 2049
    71, 3, 12, 9, 72, 3, 2, 10, 73, 3, 10, 14, 74, 3, 1, 5,
    2, 0, // 2050
    207, 3, 16, 13, 208, 3, 10, 5,
    2, 0, // 2051
    82, 0, 0, 2, 192, 0, 9, 4,
    3, 0, // 2052
    138, 1, 3, 12, 149, 1, 13, 12, 166, 3, 1, 1,
    5, 0, // 2053
    151, 0, 12, 7, 152, 0, 12, 13, 181, 2, 15, 3, 209, 0, 1, 13, 223, 2, 14, 7,
    3, 0, // 2054
    90, 0, 0, 8, 227, 2, 13, 2, 229, 2, 11, 14,
    4, 0, // 2055
    113, 3, 2, 9, 114, 3, 8, 4, 115, 3, 7, 8, 116, 3, 11, 16,
    4, 0, // 2056
    151, 1, 4, 12, 222, 3, 16, 8, 223, 3, 4, 14, 224, 3, 10, 11,
    2, 0, // 2057
    115, 1, 7, 6, 226, 2, 8, 4,
    2, 0, // 2058
    184, 3, 8, 13, 185, 3, 5, 5,
    3, 0, // 2059
    162, 1, 16, 12, 224, 2, 11, 7, 225, 2, 13, 4,
    1, 0, // 2060
    108, 1, 3, 6,
    1, 0, // 2061
    66, 0, 0, 6,
    1, 0, // 2062
    167, 0, 3, 10,
    2, 0, // 2063
    196, 0, 9, 16, 230, 2, 8, 5,
    5, 0, // 2064
    171, 1, 4, 3, 180, 0, 3, 11, 193, 1, 8, 9, 218, 0, 1, 2, 231, 2, 8, 5,
    4, 0, // 2065
    133, 0, 6, 2, 219, 0, 1, 11, 222, 2, 8, 7, 228, 2, 4, 14,
    48, 0, // 2066
    66, 0, 0, 6, 71, 3, 12, 9, 72, 3, 2, 10, 73, 3, 10, 14, 74, 3, 1, 5, 82, 0, 0, 2, 90, 0, 0, 8, 108, 1, 3, 6, 113, 3, 2, 9, 114, 3, 8, 4, 115, 3, 7, 8, 115, 1, 7, 6, 116, 3, 11, 16, 133, 0, 6, 2, 138, 1, 3, 12, 149, 1, 13, 12, 151, 0, 12, 7, 151, 1, 4, 12, 152, 0, 12, 13, 162, 1, 16, 12, 166, 3, 1, 1, 167, 0, 3, 10, 171, 1, 4, 3, 180, 0, 3, 11, 181, 2, 15, 3, 184, 3, 8, 13, 185, 3, 5, 5, 192, 0, 9, 4, 193, 1, 8, 9, 196, 0, 9, 16, 207, 3, 16, 13, 208, 3, 10, 5, 209, 0, 1, 13, 218, 0, 1, 2, 219, 0, 1, 11, 222, 3, 16, 8, 222, 2, 8, 7, 223, 3, 4, 14, 223, 2, 14, 7, 224, 3, 10, 11, 224, 2, 11, 7, 225, 2, 13, 4, 226, 2, 8, 4, 227, 2, 13, 2, 228, 2, 4, 14, 229, 2, 11, 14, 230, 2, 8, 5, 231, 2, 8, 5,
    4, 0, // 2305
    83, 3, 13, 12, 84, 3, 9, 3, 85, 3, 8, 5, 86, 3, 14, 11,
    3, 0, // 2306
    213, 3, 14, 1, 214, 3, 16, 2, 215, 3, 15, 16,
    4, 0, // 2307
    217, 1, 11, 1, 236, 0, 13, 10, 247, 0, 10, 2, 251, 3, 8, 5,
    2, 0, // 2308
    139, 1, 3, 12, 175, 3, 13, 4,
    4, 0, // 2309
    70, 1, 1, 0, 114, 1, 7, 6, 155, 1, 2, 12, 226, 0, 7, 8,
    7, 0, // 2310
    85, 1, 8, 0, 94, 0, 0, 9, 124, 0, 6, 16, 177, 0, 3, 11, 230, 0, 7, 14, 231, 0, 7, 15, 251, 1, 8, 2,
    2, 0, // 2311
    100, 0, 6, 10, 101, 0, 6, 11,
    3, 0, // 2312
    90, 1, 5, 0, 115, 0, 6, 7, 222, 0, 7, 16,
    1, 0, // 2313
    226, 1, 4, 7,
    4, 0, // 2314
    110, 0, 6, 10, 193, 3, 7, 15, 194, 3, 16, 15, 195, 3, 10, 16,
    5, 0, // 2315
    109, 1, 9, 6, 148, 1, 1, 12, 173, 0, 3, 14, 242, 1, 10, 4, 248, 3, 15, 2,
    2, 0, // 2316
    118, 1, 4, 6, 142, 1, 3, 12,
    1, 0, // 2317
    145, 1, 1, 12,
    2, 0, // 2318
    184, 1, 7, 9, 237, 3, 11, 2,
    1, 0, // 2319
    252, 2, 11, 5,
    3, 0, // 2320
    131, 0, 6, 9, 202, 0, 9, 13, 203, 0, 9, 5,
    48, 0, // 2322
    70, 1, 1, 0, 83, 3, 13, 12, 84, 3, 9, 3, 85, 3, 8, 5, 85, 1, 8, 0, 86, 3, 14, 11, 90, 1, 5, 0, 94, 0, 0, 9, 100, 0, 6, 10, 101, 0, 6, 11, 109, 1, 9, 6, 110, 0, 6, 10, 114, 1, 7, 6, 115, 0, 6, 7, 118, 1, 4, 6, 124, 0, 6, 16, 131, 0, 6, 9, 139, 1, 3, 12, 142, 1, 3, 12, 145, 1, 1, 12, 148, 1, 1, 12, 155, 1, 2, 12, 173, 0, 3, 14, 175, 3, 13, 4, 177, 0, 3, 11, 184, 1, 7, 9, 193, 3, 7, 15, 194, 3, 16, 15, 195, 3, 10, 16, 202, 0, 9, 13, 203, 0, 9, 5, 213, 3, 14, 1, 214, 3, 16, 2, 215, 3, 15, 16, 217, 1, 11, 1, 222, 0, 7, 16, 226, 0, 7, 8, 226, 1, 4, 7, 230, 0, 7, 14, 231, 0, 7, 15, 236, 0, 13, 10, 237, 3, 11, 2, 242, 1, 10, 4, 247, 0, 10, 2, 248, 3, 15, 2, 251, 3, 8, 5, 251, 1, 8, 2, 252, 2, 11, 5,
    1, 0, // 2561
    61, 0, 0, 4,
    4, 0, // 2562
    147, 1, 1, 12, 190, 2, 1, 10, 199, 2, 13, 5, 204, 2, 11, 15,
    1, 0, // 2563
    120, 0, 6, 16,
    3, 0, // 2564
    84, 1, 8, 0, 164, 3, 2, 1, 165, 3, 5, 2,
    5, 0, // 2565
    183, 2, 16, 1, 186, 2, 15, 13, 187, 0, 9, 15, 192, 2, 7, 2, 201, 2, 16, 11,
    5, 0, // 2566
    182, 3, 13, 9, 185, 2, 5, 7, 197, 2, 13, 14, 200, 2, 15, 11, 203, 2, 8, 15,
    2, 0, // 2567
    109, 3, 8, 10, 110, 3, 10, 8,
    3, 0, // 2568
    71, 0, 0, 12, 113, 0, 6, 2, 193, 2, 15, 8,
    3, 0, // 2569
    94, 1, 5, 0, 131, 1, 15, 6, 184, 2, 13, 7,
    2, 0, // 2570
    65, 1, 12, 0, 182, 0, 5, 13,
    1, 0, // 2571
    195, 2, 16, 8,
    2, 0, // 2572
    126, 0, 6, 3, 161, 0, 12, 14,
    2, 0, // 2573
    65, 0, 0, 9, 144, 3, 2, 1,
    4, 0, // 2574
    75, 0, 0, 10, 188, 2, 1, 4, 189, 2, 11, 4, 202, 2, 8, 15,
    4, 0, // 2575
    122, 0, 6, 13, 176, 0, 3, 2, 191, 2, 10, 10, 198, 2, 14, 14,
    2, 0, // 2576
    187, 2, 9, 4, 205, 2, 16, 15,
    3, 0, // 2577
    132, 0, 6, 11, 194, 2, 15, 8, 196, 2, 7, 14,
    47, 0, // 2578
    61, 0, 0, 4, 65, 0, 0, 9, 65, 1, 12, 0, 71, 0, 0, 12, 75, 0, 0, 10, 84, 1, 8, 0, 94, 1, 5, 0, 109, 3, 8, 10, 110, 3, 10, 8, 113, 0, 6, 2, 120, 0, 6, 16, 122, 0, 6, 13, 126, 0, 6, 3, 131, 1, 15, 6, 132, 0, 6, 11, 144, 3, 2, 1, 147, 1, 1, 12, 161, 0, 12, 14, 164, 3, 2, 1, 165, 3, 5, 2, 176, 0, 3, 2, 182, 0, 5, 13, 182, 3, 13, 9, 183, 2, 16, 1, 184, 2, 13, 7, 185, 2, 5, 7, 186, 2, 15, 13, 187, 0, 9, 15, 187, 2, 9, 4, 188, 2, 1, 4, 189, 2, 11, 4, 190, 2, 1, 10, 191, 2, 10, 10, 192, 2, 7, 2, 193, 2, 15, 8, 194, 2, 15, 8, 195, 2, 16, 8, 196, 2, 7, 14, 197, 2, 13, 14, 198, 2, 14, 14, 199, 2, 13, 5, 200, 2, 15, 11, 201, 2, 16, 11, 202, 2, 8, 15, 203, 2, 8, 15, 204, 2, 11, 15, 205, 2, 16, 15,
    1, 0, // 2817
    80, 3, 12, 13,
    2, 0, // 2818
    148, 0, 12, 8, 211, 3, 11, 1,
    3, 0, // 2819
    104, 1, 12, 6, 107, 1, 3, 6, 247, 2, 8, 2,
    3, 0, // 2820
    141, 0, 12, 3, 172, 3, 10, 10, 173, 3, 14, 8,
    2, 0, // 2821
    242, 3, 8, 10, 243, 3, 5, 14,
    2, 0, // 2822
    178, 1, 5, 3, 208, 1, 7, 1,
    1, 0, // 2823
    119, 3, 16, 5,
    2, 0, // 2824
    72, 0, 0, 2, 167, 1, 13, 3,
    3, 0, // 2825
    100, 1, 6, 6, 110, 1, 9, 6, 236, 1, 2, 13,
    4, 0, // 2826
    75, 1, 13, 0, 109, 0, 6, 8, 190, 3, 1, 1, 191, 3, 14, 10,
    5, 0, // 2827
    172, 0, 3, 10, 172, 1, 10, 3, 191, 0, 9, 14, 242, 0, 4, 8, 246, 3, 16, 15,
    2, 0, // 2828
    210, 1, 4, 1, 224, 1, 7, 7,
    4, 0, // 2830
    76, 1, 13, 0, 91, 1, 5, 0, 216, 1, 14, 1, 225, 1, 4, 7,
    3, 0, // 2831
    73, 1, 7, 0, 123, 1, 14, 6, 238, 0, 13, 16,
    4, 0, // 2832
    125, 1, 5, 6, 239, 1, 14, 13, 248, 2, 2, 8, 249, 2, 11, 14,
    3, 0, // 2833
    162, 0, 12, 7, 195, 1, 8, 9, 246, 2, 15, 10,
    44, 0, // 2834
    72, 0, 0, 2, 73, 1, 7, 0, 75, 1, 13, 0, 76, 1, 13, 0, 80, 3, 12, 13, 91, 1, 5, 0, 100, 1, 6, 6, 104, 1, 12, 6, 107, 1, 3, 6, 109, 0, 6, 8, 110, 1, 9, 6, 119, 3, 16, 5, 123, 1, 14, 6, 125, 1, 5, 6, 141, 0, 12, 3, 148, 0, 12, 8, 162, 0, 12, 7, 167, 1, 13, 3, 172, 0, 3, 10, 172, 3, 10, 10, 172, 1, 10, 3, 173, 3, 14, 8, 178, 1, 5, 3, 190, 3, 1, 1, 191, 0, 9, 14, 191, 3, 14, 10, 195, 1, 8, 9, 208, 1, 7, 1, 210, 1, 4, 1, 211, 3, 11, 1, 216, 1, 14, 1, 224, 1, 7, 7, 225, 1, 4, 7, 236, 1, 2, 13, 238, 0, 13, 16, 239, 1, 14, 13, 242, 0, 4, 8, 242, 3, 8, 10, 243, 3, 5, 14, 246, 3, 16, 15, 246, 2, 15, 10, 247, 2, 8, 2, 248, 2, 2, 8, 249, 2, 11, 14,
    3, 0, // 3074
    77, 1, 4, 0, 211, 1, 10, 1, 217, 3, 8, 2,
    3, 0, // 3075
    89, 1, 14, 0, 163, 1, 16, 12, 237, 1, 8, 13,
    4, 0, // 3076
    108, 0, 6, 7, 121, 1, 14, 6, 142, 0, 12, 8, 179, 3, 11, 4,
    7, 0, // 3077
    79, 0, 0, 15, 118, 0, 6, 8, 157, 1, 14, 12, 179, 1, 11, 3, 189, 0, 9, 13, 210, 0, 1, 10, 233, 0, 13, 2,
    1, 0, // 3078
    252, 0, 8, 14,
    5, 0, // 3079
    102, 0, 6, 13, 126, 3, 3, 9, 127, 3, 15, 13, 128, 3, 1, 2, 129, 3, 5, 14,
    3, 0, // 3080
    180, 1, 15, 3, 219, 1, 16, 1, 224, 0, 7, 10,
    3, 0, // 3081
    86, 0, 0, 14, 101, 1, 6, 6, 177, 1, 5, 3,
    3, 0, // 3082
    132, 1, 16, 6, 200, 3, 5, 15, 201, 3, 4, 16,
    2, 0, // 3085
    160, 3, 14, 3, 161, 3, 14, 9,
    5, 0, // 3087
    112, 1, 1, 6, 170, 1, 4, 3, 229, 0, 7, 5, 249, 0, 10, 15, 253, 3, 11, 15,
    3, 0, // 3088
    204, 0, 9, 1, 245, 0, 4, 16, 253, 1, 11, 14,
    4, 0, // 3089
    116, 1, 7, 6, 135, 0, 6, 14, 153, 1, 4, 12, 254, 0, 14, 16,
    46, 0, // 3090
    77, 1, 4, 0, 79, 0, 0, 15, 86, 0, 0, 14, 89, 1, 14, 0, 101, 1, 6, 6, 102, 0, 6, 13, 108, 0, 6, 7, 112, 1, 1, 6, 116, 1, 7, 6, 118, 0, 6, 8, 121, 1, 14, 6, 126, 3, 3, 9, 127, 3, 15, 13, 128, 3, 1, 2, 129, 3, 5, 14, 132, 1, 16, 6, 135, 0, 6, 14, 142, 0, 12, 8, 153, 1, 4, 12, 157, 1, 14, 12, 160, 3, 14, 3, 161, 3, 14, 9, 163, 1, 16, 12, 170, 1, 4, 3, 177, 1, 5, 3, 179, 3, 11, 4, 179, 1, 11, 3, 180, 1, 15, 3, 189, 0, 9, 13, 200, 3, 5, 15, 201, 3, 4, 16, 204, 0, 9, 1, 210, 0, 1, 10, 211, 1, 10, 1, 217, 3, 8, 2, 219, 1, 16, 1, 224, 0, 7, 10, 229, 0, 7, 5, 233, 0, 13, 2, 237, 1, 8, 13, 245, 0, 4, 16, 249, 0, 10, 15, 252, 0, 8, 14, 253, 3, 11, 15, 253, 1, 11, 14, 254, 0, 14, 16,
    6, 0, // 3329
    63, 3, 5, 0, 64, 3, 16, 12, 65, 3, 9, 9, 66, 3, 6, 7, 67, 3, 3, 14, 68, 3, 5, 16,
    3, 0, // 3330
    137, 2, 3, 3, 145, 0, 12, 8, 146, 0, 12, 16,
    2, 0, // 3331
    81, 0, 0, 12, 144, 2, 1, 9,
    3, 0, // 3332
    60, 1, 0, 0, 141, 2, 10, 3, 158, 2, 5, 14,
    3, 0, // 3333
    136, 2, 14, 12, 154, 2, 4, 2, 156, 2, 16, 2,
    1, 0, // 3335
    104, 3, 10, 2,
    4, 0, // 3336
    138, 2, 3, 3, 149, 2, 3, 13, 151, 2, 7, 4, 162, 2, 10, 16,
    6, 0, // 3337
    83, 0, 0, 13, 139, 2, 3, 3, 142, 2, 11, 3, 145, 2, 12, 1, 148, 2, 10, 1, 155, 2, 4, 2,
    2, 0, // 3338
    71, 1, 7, 0, 147, 2, 1, 1,
    3, 0, // 3340
    153, 2, 16, 4, 157, 2, 4, 14, 163, 2, 2, 16,
    3, 0, // 3341
    64, 0, 0, 16, 81, 1, 2, 0, 136, 3, 4, 14,
    2, 0, // 3342
    80, 1, 10, 0, 152, 2, 7, 4,
    2, 0, // 3343
    160, 2, 3, 11, 161, 2, 9, 11,
    3, 0, // 3344
    140, 2, 13, 3, 143, 2, 15, 3, 159, 2, 15, 5,
    2, 0, // 3345
    146, 2, 12, 1, 150, 2, 2, 13,
    45, 0, // 3346
    60, 1, 0, 0, 63, 3, 5, 0, 64, 0, 0, 16, 64, 3, 16, 12, 65, 3, 9, 9, 66, 3, 6, 7, 67, 3, 3, 14, 68, 3, 5, 16, 71, 1, 7, 0, 80, 1, 10, 0, 81, 0, 0, 12, 81, 1, 2, 0, 83, 0, 0, 13, 104, 3, 10, 2, 136, 3, 4, 14, 136, 2, 14, 12, 137, 2, 3, 3, 138, 2, 3, 3, 139, 2, 3, 3, 140, 2, 13, 3, 141, 2, 10, 3, 142, 2, 11, 3, 143, 2, 15, 3, 144, 2, 1, 9, 145, 0, 12, 8, 145, 2, 12, 1, 146, 0, 12, 16, 146, 2, 12, 1, 147, 2, 1, 1, 148, 2, 10, 1, 149, 2, 3, 13, 150, 2, 2, 13, 151, 2, 7, 4, 152, 2, 7, 4, 153, 2, 16, 4, 154, 2, 4, 2, 155, 2, 4, 2, 156, 2, 16, 2, 157, 2, 4, 14, 158, 2, 5, 14, 159, 2, 15, 5, 160, 2, 3, 11, 161, 2, 9, 11, 162, 2, 10, 16, 163, 2, 2, 16,
    2, 0, // 3585
    75, 3, 10, 9, 76, 3, 10, 13,
    1, 0, // 3586
    188, 1, 4, 9,
    4, 0, // 3587
    96, 1, 16, 0, 227, 0, 7, 5, 233, 2, 11, 4, 234, 2, 4, 2,
    4, 0, // 3588
    106, 0, 6, 5, 140, 0, 12, 15, 167, 3, 10, 7, 168, 3, 15, 13,
    2, 0, // 3589
    175, 1, 8, 3, 225, 0, 7, 10,
    3, 0, // 3590
    91, 0, 0, 10, 92, 0, 0, 2, 199, 0, 9, 1,
    1, 0, // 3591
    117, 3, 15, 1,
    4, 0, // 3592
    152, 1, 4, 12, 184, 0, 9, 8, 207, 0, 1, 16, 209, 1, 4, 1,
    1, 0, // 3593
    202, 1, 15, 9,
    3, 0, // 3594
    122, 1, 14, 6, 182, 1, 9, 5, 186, 3, 4, 15,
    3, 0, // 3595
    80, 0, 0, 12, 236, 2, 8, 2, 239, 2, 15, 14,
    4, 0, // 3596
    102, 1, 6, 6, 127, 0, 6, 15, 189, 1, 4, 9, 237, 2, 2, 8,
    3, 0, // 3597
    83, 1, 8, 0, 149, 3, 7, 3, 150, 3, 16, 2,
    4, 0, // 3598
    76, 0, 0, 10, 168, 0, 3, 15, 232, 0, 13, 14, 232, 3, 14, 13,
    5, 0, // 3599
    111, 1, 1, 6, 197, 0, 9, 5, 216, 0, 1, 10, 232, 2, 13, 13, 235, 2, 4, 2,
    2, 0, // 3600
    240, 2, 4, 5, 241, 2, 2, 5,
    1, 0, // 3601
    238, 2, 10, 14,
    47, 0, // 3602
    75, 3, 10, 9, 76, 0, 0, 10, 76, 3, 10, 13, 80, 0, 0, 12, 83, 1, 8, 0, 91, 0, 0, 10, 92, 0, 0, 2, 96, 1, 16, 0, 102, 1, 6, 6, 106, 0, 6, 5, 111, 1, 1, 6, 117, 3, 15, 1, 122, 1, 14, 6, 127, 0, 6, 15, 140, 0, 12, 15, 149, 3, 7, 3, 150, 3, 16, 2, 152, 1, 4, 12, 167, 3, 10, 7, 168, 0, 3, 15, 168, 3, 15, 13, 175, 1, 8, 3, 182, 1, 9, 5, 184, 0, 9, 8, 186, 3, 4, 15, 188, 1, 4, 9, 189, 1, 4, 9, 197, 0, 9, 5, 199, 0, 9, 1, 202, 1, 15, 9, 207, 0, 1, 16, 209, 1, 4, 1, 216, 0, 1, 10, 225, 0, 7, 10, 227, 0, 7, 5, 232, 0, 13, 14, 232, 3, 14, 13, 232, 2, 13, 13, 233, 2, 11, 4, 234, 2, 4, 2, 235, 2, 4, 2, 236, 2, 8, 2, 237, 2, 2, 8, 238, 2, 10, 14, 239, 2, 15, 14, 240, 2, 4, 5, 241, 2, 2, 5,
    3, 0, // 3841
    87, 3, 15, 0, 88, 3, 16, 3, 89, 3, 11, 2,
    4, 0, // 3842
    111, 0, 6, 13, 112, 0, 6, 11, 213, 1, 8, 1, 216, 3, 10, 13,
    3, 0, // 3844
    130, 1, 15, 6, 160, 1, 11, 12, 176, 3, 2, 9,
    3, 0, // 3845
    78, 0, 0, 1, 170, 0, 3, 11, 235, 1, 2, 13,
    3, 0, // 3847
    121, 3, 11, 3, 122, 3, 13, 9, 123, 3, 10, 14,
    4, 0, // 3848
    73, 0, 0, 10, 223, 0, 7, 4, 228, 3, 16, 4, 229, 3, 5, 11,
    2, 0, // 3849
    173, 1, 10, 3, 230, 1, 5, 7,
    4, 0, // 3850
    161, 1, 11, 12, 196, 3, 16, 7, 197, 3, 5, 13, 198, 3, 14, 14,
    3, 0, // 3851
    191, 1, 10, 9, 243, 0, 4, 5, 249, 3, 15, 11,
    5, 0, // 3852
    86, 1, 8, 0, 129, 0, 6, 5, 135, 1, 16, 6, 252, 1, 5, 8, 253, 2, 15, 11,
    4, 0, // 3853
    67, 0, 0, 3, 136, 0, 12, 4, 157, 3, 11, 4, 158, 3, 3, 5,
    3, 0, // 3854
    232, 1, 13, 13, 238, 3, 16, 10, 239, 3, 10, 15,
    3, 0, // 3855
    123, 0, 6, 10, 198, 0, 9, 14, 198, 1, 14, 9,
    3, 0, // 3857
    97, 0, 0, 5, 244, 1, 2, 4, 254, 2, 11, 16,
    47, 0, // 3858
    67, 0, 0, 3, 73, 0, 0, 10, 78, 0, 0, 1, 86, 1, 8, 0, 87, 3, 15, 0, 88, 3, 16, 3, 89, 3, 11, 2, 97, 0, 0, 5, 111, 0, 6, 13, 112, 0, 6, 11, 121, 3, 11, 3, 122, 3, 13, 9, 123, 0, 6, 10, 123, 3, 10, 14, 129, 0, 6, 5, 130, 1, 15, 6, 135, 1, 16, 6, 136, 0, 12, 4, 157, 3, 11, 4, 158, 3, 3, 5, 160, 1, 11, 12, 161, 1, 11, 12, 170, 0, 3, 11, 173, 1, 10, 3, 176, 3, 2, 9, 191, 1, 10, 9, 196, 3, 16, 7, 197, 3, 5, 13, 198, 0, 9, 14, 198, 3, 14, 14, 198, 1, 14, 9, 213, 1, 8, 1, 216, 3, 10, 13, 223, 0, 7, 4, 228, 3, 16, 4, 229, 3, 5, 11, 230, 1, 5, 7, 232, 1, 13, 13, 235, 1, 2, 13, 238, 3, 16, 10, 239, 3, 10, 15, 243, 0, 4, 5, 244, 1, 2, 4, 249, 3, 15, 11, 252, 1, 5, 8, 253, 2, 15, 11, 254, 2, 11, 16,
    2, 0, // 4097
    87, 1, 14, 0, 95, 3, 1, 15,
    2, 0, // 4098
    117, 1, 13, 6, 218, 3, 2, 7,
    3, 0, // 4099
    93, 1, 5, 0, 241, 1, 5, 13, 248, 1, 8, 10,
    3, 0, // 4100
    143, 0, 12, 15, 180, 3, 11, 7, 181, 0, 7, 4,
    4, 0, // 4101
    171, 0, 3, 7, 174, 1, 2, 3, 240, 1, 5, 13, 245, 3, 16, 11,
    2, 0, // 4102
    125, 0, 6, 10, 159, 0, 12, 15,
    2, 0, // 4103
    130, 3, 14, 3, 131, 3, 9, 8,
    3, 0, // 4105
    193, 0, 9, 7, 194, 0, 9, 16, 231, 1, 5, 7,
    5, 0, // 4106
    187, 1, 4, 9, 202, 3, 13, 8, 203, 3, 5, 8, 204, 3, 1, 11, 205, 3, 15, 16,
    1, 0, // 4107
    246, 0, 10, 16,
    4, 0, // 4108
    79, 1, 4, 0, 200, 0, 9, 5, 249, 1, 14, 10, 253, 0, 14, 11,
    4, 0, // 4110
    127, 1, 11, 6, 140, 1, 3, 12, 168, 1, 13, 3, 186, 0, 9, 4,
    1, 0, // 4111
    239, 0, 13, 10,
    4, 0, // 4112
    95, 0, 0, 1, 143, 1, 3, 12, 159, 1, 5, 12, 220, 1, 16, 1,
    7, 0, // 4113
    98, 0, 0, 3, 99, 1, 16, 0, 205, 1, 15, 9, 215, 1, 8, 1, 220, 0, 1, 15, 221, 0, 1, 16, 255, 0, 5, 16,
    47, 0, // 4114
    79, 1, 4, 0, 87, 1, 14, 0, 93, 1, 5, 0, 95, 0, 0, 1, 95, 3, 1, 15, 98, 0, 0, 3, 99, 1, 16, 0, 117, 1, 13, 6, 125, 0, 6, 10, 127, 1, 11, 6, 130, 3, 14, 3, 131, 3, 9, 8, 140, 1, 3, 12, 143, 0, 12, 15, 143, 1, 3, 12, 159, 0, 12, 15, 159, 1, 5, 12, 168, 1, 13, 3, 171, 0, 3, 7, 174, 1, 2, 3, 180, 3, 11, 7, 181, 0, 7, 4, 186, 0, 9, 4, 187, 1, 4, 9, 193, 0, 9, 7, 194, 0, 9, 16, 200, 0, 9, 5, 202, 3, 13, 8, 203, 3, 5, 8, 204, 3, 1, 11, 205, 3, 15, 16, 205, 1, 15, 9, 215, 1, 8, 1, 218, 3, 2, 7, 220, 0, 1, 15, 220, 1, 16, 1, 221, 0, 1, 16, 231, 1, 5, 7, 239, 0, 13, 10, 240, 1, 5, 13, 241, 1, 5, 13, 245, 3, 16, 11, 246, 0, 10, 16, 248, 1, 8, 10, 249, 1, 14, 10, 253, 0, 14, 11, 255, 0, 5, 16,
    4, 0, // 4353
    96, 3, 13, 2, 97, 3, 5, 14, 98, 3, 3, 15, 99, 3, 15, 16,
    5, 0, // 4354
    183, 0, 9, 4, 206, 0, 1, 4, 219, 3, 11, 7, 220, 3, 15, 15, 221, 3, 16, 15,
    5, 0, // 4355
    150, 1, 13, 12, 156, 0, 12, 4, 214, 1, 8, 1, 244, 0, 4, 14, 250, 0, 2, 5,
    1, 0, // 4356
    88, 1, 14, 0,
    2, 0, // 4357
    153, 0, 12, 11, 228, 1, 14, 7,
    2, 0, // 4358
    119, 1, 10, 6, 255, 3, 16, 15,
    5, 0, // 4359
    103, 0, 6, 3, 132, 3, 11, 9, 133, 3, 2, 7, 134, 3, 1, 2, 135, 3, 14, 11,
    2, 0, // 4360
    116, 0, 6, 11, 196, 1, 14, 9,
    4, 0, // 4361
    124, 1, 5, 6, 195, 0, 9, 10, 215, 0, 1, 15, 222, 1, 7, 7,
    1, 0, // 4362
    120, 1, 2, 6,
    1, 0, // 4363
    238, 1, 14, 13,
    3, 0, // 4364
    201, 0, 9, 4, 245, 1, 15, 4, 254, 1, 16, 14,
    5, 0, // 4365
    64, 1, 12, 0, 68, 0, 0, 5, 146, 1, 1, 12, 162, 3, 7, 10, 163, 3, 11, 2,
    1, 0, // 4366
    207, 1, 7, 1,
    1, 0, // 4367
    254, 3, 16, 11,
    5, 0, // 4368
    194, 1, 8, 9, 205, 0, 9, 15, 221, 1, 16, 1, 246, 1, 10, 10, 255, 1, 16, 5,
    1, 0, // 4369
    99, 0, 0, 15,
    48, 0, // 4370
    64, 1, 12, 0, 68, 0, 0, 5, 88, 1, 14, 0, 96, 3, 13, 2, 97, 3, 5, 14, 98, 3, 3, 15, 99, 0, 0, 15, 99, 3, 15, 16, 103, 0, 6, 3, 116, 0, 6, 11, 119, 1, 10, 6, 120, 1, 2, 6, 124, 1, 5, 6, 132, 3, 11, 9, 133, 3, 2, 7, 134, 3, 1, 2, 135, 3, 14, 11, 146, 1, 1, 12, 150, 1, 13, 12, 153, 0, 12, 11, 156, 0, 12, 4, 162, 3, 7, 10, 163, 3, 11, 2, 183, 0, 9, 4, 194, 1, 8, 9, 195, 0, 9, 10, 196, 1, 14, 9, 201, 0, 9, 4, 205, 0, 9, 15, 206, 0, 1, 4, 207, 1, 7, 1, 214, 1, 8, 1, 215, 0, 1, 15, 219, 3, 11, 7, 220, 3, 15, 15, 221, 3, 16, 15, 221, 1, 16, 1, 222, 1, 7, 7, 228, 1, 14, 7, 238, 1, 14, 13, 244, 0, 4, 14, 245, 1, 15, 4, 246, 1, 10, 10, 250, 0, 2, 5, 254, 3, 16, 11, 254, 1, 16, 14, 255, 3, 16, 15, 255, 1, 16, 5,
    44, 0, // 4609
    60, 0, 0, 12, 60, 3, 12, 3, 61, 0, 0, 4, 61, 3, 4, 9, 62, 3, 1, 6, 63, 3, 5, 0, 63, 1, 12, 0, 64, 3, 16, 12, 65, 3, 9, 9, 66, 3, 6, 7, 67, 3, 3, 14, 68, 3, 5, 16, 69, 3, 4, 1, 70, 3, 8, 4, 71, 3, 12, 9, 72, 3, 2, 10, 73, 3, 10, 14, 74, 3, 1, 5, 75, 3, 10, 9, 76, 3, 10, 13, 77, 3, 11, 1, 78, 3, 1, 14, 79, 3, 15, 11, 80, 3, 12, 13, 81, 3, 12, 12, 82, 3, 2, 7, 83, 3, 13, 12, 84, 3, 9, 3, 85, 3, 8, 5, 86, 3, 14, 11, 87, 3, 15, 0, 87, 1, 14, 0, 88, 3, 16, 3, 89, 3, 11, 2, 90, 3, 8, 7, 91, 3, 10, 13, 92, 3, 2, 13, 93, 3, 15, 2, 94, 3, 9, 8, 95, 3, 1, 15, 96, 3, 13, 2, 97, 3, 5, 14, 98, 3, 3, 15, 99, 3, 15, 16,
    49, 0, // 4610
    62, 2, 6, 6, 69, 0, 0, 4, 69, 1, 1, 0, 70, 0, 0, 8, 74, 2, 5, 7, 77, 1, 4, 0, 78, 2, 14, 4, 95, 2, 15, 15, 105, 1, 3, 6, 111, 0, 6, 13, 112, 0, 6, 11, 117, 1, 13, 6, 128, 2, 2, 11, 134, 2, 2, 16, 137, 2, 3, 3, 144, 1, 9, 12, 145, 0, 12, 8, 146, 0, 12, 16, 147, 0, 12, 9, 147, 1, 1, 12, 148, 0, 12, 8, 164, 1, 9, 3, 166, 2, 1, 7, 166, 1, 7, 3, 183, 0, 9, 4, 188, 1, 4, 9, 190, 2, 1, 10, 190, 1, 10, 9, 199, 2, 13, 5, 204, 2, 11, 15, 206, 0, 1, 4, 206, 3, 4, 16, 207, 3, 16, 13, 208, 3, 10, 5, 209, 3, 13, 7, 210, 3, 10, 11, 211, 3, 11, 1, 211, 1, 10, 1, 212, 3, 5, 5, 213, 3, 14, 1, 213, 1, 8, 1, 214, 3, 16, 2, 215, 3, 15, 16, 216, 3, 10, 13, 217, 3, 8, 2, 218, 3, 2, 7, 219, 3, 11, 7, 220, 3, 15, 15, 221, 3, 16, 15,
    46, 0, // 4611
    72, 2, 10, 7, 81, 0, 0, 12, 82, 0, 0, 2, 82, 2, 7, 2, 89, 1, 14, 0, 92, 2, 13, 5, 93, 1, 5, 0, 96, 1, 16, 0, 104, 1, 12, 6, 107, 1, 3, 6, 113, 2, 9, 7, 120, 0, 6, 16, 128, 1, 11, 6, 133, 2, 7, 16, 134, 1, 16, 6, 144, 2, 1, 9, 150, 1, 13, 12, 154, 0, 12, 4, 155, 0, 12, 8, 156, 0, 12, 4, 163, 1, 16, 12, 164, 2, 1, 9, 165, 1, 9, 3, 169, 1, 4, 3, 174, 0, 3, 15, 176, 2, 9, 14, 192, 0, 9, 4, 212, 0, 1, 5, 214, 1, 8, 1, 217, 1, 11, 1, 218, 2, 7, 15, 227, 0, 7, 5, 233, 2, 11, 4, 234, 0, 13, 2, 234, 2, 4, 2, 235, 0, 13, 14, 236, 0, 13, 10, 237, 1, 8, 13, 241, 1, 5, 13, 244, 0, 4, 14, 247, 0, 10, 2, 247, 2, 8, 2, 248, 1, 8, 10, 250, 0, 2, 5, 250, 3, 5, 16, 251, 3, 8, 5,
    45, 0, // 4612
    60, 1, 0, 0, 67, 2, 14, 12, 84, 1, 8, 0, 88, 1, 14, 0, 98, 2, 15, 16, 103, 2, 16, 6, 105, 0, 6, 5, 106, 0, 6, 5, 107, 0, 6, 10, 108, 0, 6, 7, 121, 1, 14, 6, 126, 2, 9, 11, 130, 1, 15, 6, 137, 0, 12, 1, 137, 1, 3, 12, 138, 0, 12, 7, 138, 1, 3, 12, 139, 0, 12, 8, 139, 1, 3, 12, 140, 0, 12, 15, 141, 0, 12, 3, 141, 2, 10, 3, 142, 0, 12, 8, 143, 0, 12, 15, 149, 1, 13, 12, 158, 2, 5, 14, 160, 1, 11, 12, 164, 3, 2, 1, 165, 3, 5, 2, 166, 3, 1, 1, 167, 3, 10, 7, 168, 3, 15, 13, 169, 3, 5, 2, 170, 3, 11, 14, 171, 3, 7, 15, 172, 3, 10, 10, 173, 3, 14, 8, 174, 3, 15, 4, 175, 3, 13, 4, 176, 3, 2, 9, 177, 3, 11, 8, 178, 3, 10, 5, 179, 3, 11, 4, 180, 3, 11, 7, 181, 0, 7, 4,
    46, 0, // 4613
    61, 2, 9, 0, 69, 2, 1, 1, 70, 1, 1, 0, 77, 0, 0, 11, 78, 0, 0, 1, 79, 0, 0, 15, 114, 1, 7, 6, 118, 0, 6, 8, 136, 2, 14, 12, 151, 0, 12, 7, 152, 0, 12, 13, 153, 0, 12, 11, 154, 2, 4, 2, 154, 1, 2, 12, 155, 1, 2, 12, 156, 2, 16, 2, 157, 1, 14, 12, 169, 0, 3, 5, 170, 0, 3, 11, 171, 0, 3, 7, 174, 1, 2, 3, 175, 1, 8, 3, 179, 1, 11, 3, 181, 2, 15, 3, 183, 2, 16, 1, 186, 2, 15, 13, 187, 0, 9, 15, 188, 0, 9, 13, 189, 0, 9, 13, 192, 2, 7, 2, 201, 2, 16, 11, 206, 2, 16, 1, 209, 0, 1, 13, 210, 0, 1, 10, 223, 2, 14, 7, 225, 0, 7, 10, 226, 0, 7, 8, 228, 1, 14, 7, 233, 0, 13, 2, 234, 1, 2, 13, 235, 1, 2, 13, 240, 1, 5, 13, 242, 3, 8, 10, 243, 3, 5, 14, 244, 3, 14, 16, 245, 3, 16, 11,
    44, 0, // 4614
    63, 2, 0, 12, 68, 2, 16, 12, 74, 1, 7, 0, 85, 1, 8, 0, 90, 0, 0, 8, 91, 0, 0, 10, 92, 0, 0, 2, 93, 0, 0, 15, 94, 0, 0, 9, 97, 2, 14, 16, 105, 2, 1, 3, 106, 2, 13, 3, 119, 1, 10, 6, 124, 0, 6, 16, 125, 0, 6, 10, 129, 2, 14, 11, 158, 1, 14, 12, 159, 0, 12, 15, 165, 2, 2, 9, 169, 2, 2, 4, 177, 0, 3, 11, 178, 0, 3, 10, 178, 1, 5, 3, 182, 3, 13, 9, 185, 2, 5, 7, 185, 1, 7, 9, 197, 2, 13, 14, 199, 0, 9, 1, 200, 2, 15, 11, 203, 2, 8, 15, 208, 1, 7, 1, 212, 2, 5, 2, 212, 1, 2, 1, 227, 2, 13, 2, 229, 2, 11, 14, 230, 0, 7, 14, 231, 0, 7, 15, 240, 0, 13, 15, 241, 0, 13, 15, 243, 2, 14, 10, 250, 2, 16, 2, 251, 1, 8, 2, 252, 0, 8, 14, 255, 3, 16, 15,
    43, 0, // 4615
    62, 0, 0, 1, 62, 1, 6, 0, 66, 2, 7, 12, 100, 0, 6, 10, 100, 3, 10, 8, 101, 0, 6, 11, 101, 3, 11, 8, 102, 0, 6, 13, 102, 3, 13, 11, 103, 0, 6, 3, 103, 3, 3, 16, 104, 3, 10, 2, 105, 3, 5, 1, 106, 3, 5, 13, 107, 3, 10, 2, 108, 3, 7, 11, 109, 3, 8, 10, 110, 3, 10, 8, 111, 3, 13, 14, 112, 3, 11, 14, 113, 3, 2, 9, 114, 3, 8, 4, 115, 3, 7, 8, 116, 3, 11, 16, 117, 3, 15, 1, 118, 3, 8, 11, 119, 3, 16, 5, 120, 3, 16, 9, 121, 3, 11, 3, 122, 3, 13, 9, 123, 3, 10, 14, 124, 3, 16, 8, 125, 3, 10, 15, 126, 3, 3, 9, 127, 3, 15, 13, 128, 3, 1, 2, 129, 3, 5, 14, 130, 3, 14, 3, 131, 3, 9, 8, 132, 3, 11, 9, 133, 3, 2, 7, 134, 3, 1, 2, 135, 3, 14, 11,
    48, 0, // 4616
    66, 1, 12, 0, 71, 0, 0, 12, 72, 0, 0, 2, 73, 0, 0, 10, 74, 0, 0, 1, 82, 1, 2, 0, 90, 1, 5, 0, 108, 2, 11, 3, 113, 0, 6, 2, 114, 0, 6, 8, 115, 0, 6, 7, 115, 2, 8, 7, 116, 0, 6, 11, 133, 1, 16, 6, 138, 2, 3, 3, 149, 2, 3, 13, 151, 2, 7, 4, 151, 1, 4, 12, 152, 1, 4, 12, 162, 2, 10, 16, 166, 0, 3, 1, 167, 1, 13, 3, 171, 2, 15, 4, 180, 1, 15, 3, 181, 3, 4, 15, 184, 0, 9, 8, 185, 0, 9, 5, 192, 1, 2, 9, 193, 2, 15, 8, 196, 1, 14, 9, 207, 0, 1, 16, 208, 0, 1, 10, 209, 1, 4, 1, 218, 1, 15, 1, 219, 1, 16, 1, 222, 0, 7, 16, 222, 3, 16, 8, 223, 0, 7, 4, 223, 3, 4, 14, 224, 0, 7, 10, 224, 3, 10, 11, 225, 3, 10, 13, 226, 3, 8, 8, 227, 3, 5, 13, 228, 3, 16, 4, 229, 3, 5, 11, 230, 3, 14, 8, 231, 3, 15, 8,
    48, 0, // 4617
    70, 2, 4, 1, 83, 0, 0, 13, 84, 0, 0, 9, 85, 0, 0, 8, 85, 2, 5, 8, 86, 0, 0, 14, 90, 2, 7, 5, 94, 1, 5, 0, 100, 1, 6, 6, 101, 1, 6, 6, 109, 2, 10, 9, 110, 1, 9, 6, 114, 2, 4, 7, 115, 1, 7, 6, 118, 2, 11, 4, 124, 1, 5, 6, 131, 1, 15, 6, 139, 2, 3, 3, 142, 2, 11, 3, 145, 2, 12, 1, 148, 2, 10, 1, 155, 2, 4, 2, 173, 1, 10, 3, 175, 0, 3, 13, 177, 1, 5, 3, 184, 2, 13, 7, 193, 0, 9, 7, 194, 0, 9, 16, 195, 0, 9, 10, 202, 1, 15, 9, 203, 1, 15, 9, 213, 0, 1, 14, 214, 0, 1, 16, 215, 0, 1, 15, 217, 2, 2, 11, 222, 1, 7, 7, 226, 2, 8, 4, 226, 1, 4, 7, 230, 1, 5, 7, 231, 1, 5, 7, 236, 1, 2, 13, 237, 0, 13, 11, 242, 2, 10, 10, 247, 1, 2, 10, 248, 0, 10, 15, 251, 0, 2, 8, 251, 2, 5, 8, 252, 3, 14, 11,
    47, 0, // 4618
    61, 1, 0, 0, 65, 2, 9, 12, 65, 1, 12, 0, 71, 1, 7, 0, 75, 1, 13, 0, 84, 2, 3, 8, 94, 2, 8, 5, 109, 0, 6, 8, 110, 0, 6, 10, 113, 1, 7, 6, 120, 1, 2, 6, 122, 1, 14, 6, 126, 1, 11, 6, 131, 2, 8, 15, 132, 1, 16, 6, 144, 0, 12, 2, 147, 2, 1, 1, 161, 1, 11, 12, 164, 0, 3, 2, 165, 0, 3, 5, 176, 1, 14, 3, 182, 0, 5, 13, 182, 1, 9, 5, 183, 3, 4, 16, 184, 3, 8, 13, 185, 3, 5, 5, 186, 3, 4, 15, 187, 3, 15, 9, 187, 1, 4, 9, 188, 3, 13, 1, 189, 3, 13, 11, 190, 3, 1, 1, 191, 3, 14, 10, 192, 3, 4, 7, 193, 3, 7, 15, 194, 3, 16, 15, 195, 3, 10, 16, 196, 3, 16, 7, 197, 3, 5, 13, 198, 3, 14, 14, 199, 3, 1, 13, 200, 3, 5, 15, 201, 3, 4, 16, 202, 3, 13, 8, 203, 3, 5, 8, 204, 3, 1, 11, 205, 3, 15, 16,
    44, 0, // 4619
    72, 1, 7, 0, 73, 2, 14, 7, 75, 2, 9, 13, 76, 2, 13, 13, 80, 0, 0, 12, 91, 2, 13, 5, 100, 2, 8, 6, 104, 2, 2, 12, 107, 2, 2, 3, 109, 1, 9, 6, 110, 2, 8, 9, 119, 0, 6, 16, 123, 2, 14, 14, 125, 2, 15, 5, 141, 1, 3, 12, 148, 1, 1, 12, 162, 1, 16, 12, 167, 2, 7, 13, 172, 0, 3, 10, 172, 2, 10, 10, 172, 1, 10, 3, 173, 0, 3, 14, 178, 2, 5, 5, 190, 0, 9, 1, 191, 0, 9, 14, 191, 1, 10, 9, 195, 2, 16, 8, 208, 2, 5, 7, 210, 2, 11, 4, 211, 0, 1, 11, 216, 2, 13, 14, 224, 2, 11, 7, 225, 2, 13, 4, 236, 2, 8, 2, 238, 1, 14, 13, 239, 2, 15, 14, 242, 0, 4, 8, 242, 1, 10, 4, 243, 0, 4, 5, 246, 0, 10, 16, 246, 3, 16, 15, 247, 3, 2, 8, 248, 3, 15, 2, 249, 3, 15, 11,
    46, 0, // 4620
    77, 2, 1, 4, 79, 1, 4, 0, 86, 1, 8, 0, 89, 2, 2, 14, 101, 2, 8, 6, 102, 1, 6, 6, 108, 1, 3, 6, 112, 2, 14, 1, 116, 2, 16, 7, 118, 1, 4, 6, 121, 2, 3, 14, 126, 0, 6, 3, 127, 0, 6, 15, 128, 0, 6, 1, 129, 0, 6, 5, 132, 2, 9, 16, 135, 1, 16, 6, 142, 1, 3, 12, 153, 2, 16, 4, 157, 2, 4, 14, 160, 0, 12, 14, 161, 0, 12, 14, 163, 2, 2, 16, 170, 2, 14, 4, 177, 2, 8, 5, 179, 0, 3, 11, 179, 2, 4, 11, 180, 2, 7, 15, 189, 1, 4, 9, 200, 0, 9, 5, 201, 0, 9, 4, 204, 1, 15, 9, 210, 1, 4, 1, 211, 2, 1, 10, 217, 0, 1, 8, 219, 2, 7, 16, 224, 1, 7, 7, 229, 1, 14, 7, 233, 1, 4, 13, 237, 2, 2, 8, 245, 1, 15, 4, 249, 1, 14, 10, 252, 1, 5, 8, 253, 0, 14, 11, 253, 2, 15, 11, 254, 1, 16, 14,
    45, 0, // 4621
    60, 2, 3, 0, 63, 0, 0, 5, 64, 0, 0, 16, 64, 1, 12, 0, 65, 0, 0, 9, 66, 0, 0, 6, 67, 0, 0, 3, 68, 0, 0, 5, 71, 2, 9, 7, 80, 2, 13, 10, 81, 2, 12, 2, 81, 1, 2, 0, 83, 1, 8, 0, 104, 0, 6, 10, 136, 0, 12, 4, 136, 3, 4, 14, 137, 3, 1, 3, 138, 3, 7, 3, 139, 3, 8, 3, 140, 3, 15, 13, 141, 3, 3, 10, 142, 3, 8, 11, 143, 3, 15, 15, 144, 3, 2, 1, 145, 3, 8, 12, 145, 1, 1, 12, 146, 3, 16, 12, 146, 1, 1, 12, 147, 3, 9, 1, 148, 3, 8, 10, 149, 3, 7, 3, 150, 3, 16, 2, 151, 3, 7, 7, 152, 3, 13, 7, 153, 3, 11, 16, 154, 3, 4, 4, 155, 3, 8, 4, 156, 3, 4, 16, 157, 3, 11, 4, 158, 3, 3, 5, 159, 3, 15, 15, 160, 3, 14, 3, 161, 3, 14, 9, 162, 3, 7, 10, 163, 3, 11, 2,
    47, 0, // 4622
    75, 0, 0, 10, 76, 0, 0, 10, 76, 1, 13, 0, 80, 1, 10, 0, 83, 2, 12, 8, 91, 1, 5, 0, 92, 1, 5, 0, 96, 2, 2, 16, 102, 2, 11, 6, 106, 1, 3, 6, 111, 2, 14, 1, 117, 0, 6, 15, 122, 2, 9, 14, 127, 1, 11, 6, 140, 1, 3, 12, 149, 0, 12, 7, 150, 0, 12, 16, 152, 2, 7, 4, 167, 0, 3, 10, 168, 0, 3, 15, 168, 1, 13, 3, 175, 2, 4, 8, 182, 2, 9, 9, 184, 1, 7, 9, 186, 0, 9, 4, 188, 2, 1, 4, 189, 2, 11, 4, 197, 1, 14, 9, 199, 1, 5, 9, 202, 2, 8, 15, 207, 1, 7, 1, 209, 2, 7, 4, 216, 1, 14, 1, 225, 1, 4, 7, 227, 1, 2, 7, 232, 0, 13, 14, 232, 3, 14, 13, 232, 1, 13, 13, 233, 3, 2, 11, 234, 3, 2, 4, 235, 3, 14, 4, 236, 3, 10, 8, 237, 3, 11, 2, 238, 3, 16, 10, 239, 3, 10, 15, 240, 3, 15, 4, 241, 3, 15, 2,
    47, 0, // 4623
    67, 1, 12, 0, 73, 1, 7, 0, 78, 1, 4, 0, 86, 2, 11, 8, 87, 0, 0, 15, 88, 0, 0, 16, 89, 0, 0, 11, 97, 1, 16, 0, 111, 1, 1, 6, 112, 1, 1, 6, 121, 0, 6, 11, 122, 0, 6, 13, 123, 0, 6, 10, 123, 1, 14, 6, 129, 1, 11, 6, 130, 2, 3, 15, 135, 2, 11, 16, 136, 1, 12, 12, 157, 0, 12, 11, 158, 0, 12, 3, 160, 2, 3, 11, 161, 2, 9, 11, 170, 1, 4, 3, 173, 2, 8, 10, 176, 0, 3, 2, 191, 2, 10, 10, 196, 0, 9, 16, 197, 0, 9, 5, 198, 0, 9, 14, 198, 2, 14, 14, 198, 1, 14, 9, 213, 2, 1, 8, 216, 0, 1, 10, 223, 1, 7, 7, 228, 0, 7, 16, 229, 0, 7, 5, 230, 2, 8, 5, 232, 2, 13, 13, 235, 2, 4, 2, 238, 0, 13, 16, 239, 0, 13, 10, 243, 1, 10, 4, 244, 2, 16, 2, 249, 0, 10, 15, 252, 2, 11, 5, 253, 3, 11, 15, 254, 3, 16, 11,
    47, 0, // 4624
    79, 2, 11, 4, 87, 2, 0, 14, 93, 2, 2, 5, 95, 0, 0, 1, 95, 1, 15, 0, 98, 1, 16, 0, 99, 2, 16, 16, 117, 2, 1, 13, 125, 1, 5, 6, 127, 2, 13, 11, 130, 0, 6, 14, 131, 0, 6, 9, 140, 2, 13, 3, 143, 2, 15, 3, 143, 1, 3, 12, 159, 2, 15, 5, 159, 1, 5, 12, 168, 2, 13, 13, 171, 1, 4, 3, 174, 2, 4, 2, 180, 0, 3, 11, 181, 1, 3, 7, 186, 1, 13, 9, 187, 2, 9, 4, 193, 1, 8, 9, 194, 1, 8, 9, 200, 1, 11, 9, 202, 0, 9, 13, 203, 0, 9, 5, 204, 0, 9, 1, 205, 0, 9, 15, 205, 2, 16, 15, 215, 2, 16, 8, 218, 0, 1, 2, 220, 2, 15, 16, 220, 1, 16, 1, 221, 1, 16, 1, 231, 2, 8, 5, 239, 1, 14, 13, 240, 2, 4, 5, 241, 2, 2, 5, 245, 0, 4, 16, 246, 1, 10, 10, 248, 2, 2, 8, 249, 2, 11, 14, 253, 1, 11, 14, 255, 1, 16, 5,
    48, 0, // 4625
    64, 2, 12, 12, 68, 1, 12, 0, 88, 2, 3, 14, 96, 0, 0, 13, 97, 0, 0, 5, 98, 0, 0, 3, 99, 0, 0, 15, 99, 1, 16, 0, 103, 1, 6, 6, 116, 1, 7, 6, 119, 2, 5, 10, 120, 2, 9, 2, 124, 2, 8, 5, 132, 0, 6, 11, 133, 0, 6, 2, 134, 0, 6, 1, 135, 0, 6, 14, 146, 2, 12, 1, 150, 2, 2, 13, 153, 1, 4, 12, 156, 1, 2, 12, 162, 0, 12, 7, 163, 0, 12, 11, 183, 1, 1, 9, 194, 2, 15, 8, 195, 1, 8, 9, 196, 2, 7, 14, 201, 1, 11, 9, 205, 1, 15, 9, 206, 1, 1, 1, 207, 2, 13, 7, 214, 2, 2, 8, 215, 1, 8, 1, 219, 0, 1, 11, 220, 0, 1, 15, 221, 0, 1, 16, 221, 2, 15, 16, 222, 2, 8, 7, 228, 2, 4, 14, 238, 2, 10, 14, 244, 1, 2, 4, 245, 2, 11, 15, 246, 2, 15, 10, 250, 1, 2, 2, 254, 0, 14, 16, 254, 2, 11, 16, 255, 0, 5, 16, 255, 2, 15, 16,
    0, 0, // 4626
    60, 0, 0, 12, 60, 3, 12, 3, 60, 2, 3, 0, 60, 1, 0, 0, 61, 0, 0, 4, 61, 3, 4, 9, 61, 2, 9, 0, 61, 1, 0, 0, 62, 0, 0, 1, 62, 3, 1, 6, 62, 2, 6, 6, 62, 1, 6, 0, 63, 0, 0, 5, 63, 3, 5, 0, 63, 2, 0, 12, 63, 1, 12, 0, 64, 0, 0, 16, 64, 3, 16, 12, 64, 2, 12, 12, 64, 1, 12, 0, 65, 0, 0, 9, 65, 3, 9, 9, 65, 2, 9, 12, 65, 1, 12, 0, 66, 0, 0, 6, 66, 3, 6, 7, 66, 2, 7, 12, 66, 1, 12, 0, 67, 0, 0, 3, 67, 3, 3, 14, 67, 2, 14, 12, 67, 1, 12, 0, 68, 0, 0, 5, 68, 3, 5, 16, 68, 2, 16, 12, 68, 1, 12, 0, 69, 0, 0, 4, 69, 3, 4, 1, 69, 2, 1, 1, 69, 1, 1, 0, 70, 0, 0, 8, 70, 3, 8, 4, 70, 2, 4, 1, 70, 1, 1, 0, 71, 0, 0, 12, 71, 3, 12, 9, 71, 2, 9, 7, 71, 1, 7, 0, 72, 0, 0, 2, 72, 3, 2, 10, 72, 2, 10, 7, 72, 1, 7, 0, 73, 0, 0, 10, 73, 3, 10, 14, 73, 2, 14, 7, 73, 1, 7, 0, 74, 0, 0, 1, 74, 3, 1, 5, 74, 2, 5, 7, 74, 1, 7, 0, 75, 0, 0, 10, 75, 3, 10, 9, 75, 2, 9, 13, 75, 1, 13, 0, 76, 0, 0, 10, 76, 3, 10, 13, 76, 2, 13, 13, 76, 1, 13, 0, 77, 0, 0, 11, 77, 3, 11, 1, 77, 2, 1, 4, 77, 1, 4, 0, 78, 0, 0, 1, 78, 3, 1, 14, 78, 2, 14, 4, 78, 1, 4, 0, 79, 0, 0, 15, 79, 3, 15, 11, 79, 2, 11, 4, 79, 1, 4, 0, 80, 0, 0, 12, 80, 3, 12, 13, 80, 2, 13, 10, 80, 1, 10, 0, 81, 0, 0, 12, 81, 3, 12, 12, 81, 2, 12, 2, 81, 1, 2, 0, 82, 0, 0, 2, 82, 3, 2, 7, 82, 2, 7, 2, 82, 1, 2, 0, 83, 0, 0, 13, 83, 3, 13, 12, 83, 2, 12, 8, 83, 1, 8, 0, 84, 0, 0, 9, 84, 3, 9, 3, 84, 2, 3, 8, 84, 1, 8, 0, 85, 0, 0, 8, 85, 3, 8, 5, 85, 2, 5, 8, 85, 1, 8, 0, 86, 0, 0, 14, 86, 3, 14, 11, 86, 2, 11, 8, 86, 1, 8, 0, 87, 0, 0, 15, 87, 3, 15, 0, 87, 2, 0, 14, 87, 1, 14, 0, 88, 0, 0, 16, 88, 3, 16, 3, 88, 2, 3, 14, 88, 1, 14, 0, 89, 0, 0, 11, 89, 3, 11, 2, 89, 2, 2, 14, 89, 1, 14, 0, 90, 0, 0, 8, 90, 3, 8, 7, 90, 2, 7, 5, 90, 1, 5, 0, 91, 0, 0, 10, 91, 3, 10, 13, 91, 2, 13, 5, 91, 1, 5, 0, 92, 0, 0, 2, 92, 3, 2, 13, 92, 2, 13, 5, 92, 1, 5, 0, 93, 0, 0, 15, 93, 3, 15, 2, 93, 2, 2, 5, 93, 1, 5, 0, 94, 0, 0, 9, 94, 3, 9, 8, 94, 2, 8, 5, 94, 1, 5, 0, 95, 0, 0, 1, 95, 3, 1, 15, 95, 2, 15, 15, 95, 1, 15, 0, 96, 0, 0, 13, 96, 3, 13, 2, 96, 2, 2, 16, 96, 1, 16, 0, 97, 0, 0, 5, 97, 3, 5, 14, 97, 2, 14, 16, 97, 1, 16, 0, 98, 0, 0, 3, 98, 3, 3, 15, 98, 2, 15, 16, 98, 1, 16, 0, 99, 0, 0, 15, 99, 3, 15, 16, 99, 2, 16, 16, 99, 1, 16, 0, 100, 0, 6, 10, 100, 3, 10, 8, 100, 2, 8, 6, 100, 1, 6, 6, 101, 0, 6, 11, 101, 3, 11, 8, 101, 2, 8, 6, 101, 1, 6, 6, 102, 0, 6, 13, 102, 3, 13, 11, 102, 2, 11, 6, 102, 1, 6, 6, 103, 0, 6, 3, 103, 3, 3, 16, 103, 2, 16, 6, 103, 1, 6, 6, 104, 0, 6, 10, 104, 3, 10, 2, 104, 2, 2, 12, 104, 1, 12, 6, 105, 0, 6, 5, 105, 3, 5, 1, 105, 2, 1, 3, 105, 1, 3, 6, 106, 0, 6, 5, 106, 3, 5, 13, 106, 2, 13, 3, 106, 1, 3, 6, 107, 0, 6, 10, 107, 3, 10, 2, 107, 2, 2, 3, 107, 1, 3, 6, 108, 0, 6, 7, 108, 3, 7, 11, 108, 2, 11, 3, 108, 1, 3, 6, 109, 0, 6, 8, 109, 3, 8, 10, 109, 2, 10, 9, 109, 1, 9, 6, 110, 0, 6, 10, 110, 3, 10, 8, 110, 2, 8, 9, 110, 1, 9, 6, 111, 0, 6, 13, 111, 3, 13, 14, 111, 2, 14, 1, 111, 1, 1, 6, 112, 0, 6, 11, 112, 3, 11, 14, 112, 2, 14, 1, 112, 1, 1, 6, 113, 0, 6, 2, 113, 3, 2, 9, 113, 2, 9, 7, 113, 1, 7, 6, 114, 0, 6, 8, 114, 3, 8, 4, 114, 2, 4, 7, 114, 1, 7, 6, 115, 0, 6, 7, 115, 3, 7, 8, 115, 2, 8, 7, 115, 1, 7, 6, 116, 0, 6, 11, 116, 3, 11, 16, 116, 2, 16, 7, 116, 1, 7, 6, 117, 0, 6, 15, 117, 3, 15, 1, 117, 2, 1, 13, 117, 1, 13, 6, 118, 0, 6, 8, 118, 3, 8, 11, 118, 2, 11, 4, 118, 1, 4, 6, 119, 0, 6, 16, 119, 3, 16, 5, 119, 2, 5, 10, 119, 1, 10, 6, 120, 0, 6, 16, 120, 3, 16, 9, 120, 2, 9, 2, 120, 1, 2, 6, 121, 0, 6, 11, 121, 3, 11, 3, 121, 2, 3, 14, 121, 1, 14, 6, 122, 0, 6, 13, 122, 3, 13, 9, 122, 2, 9, 14, 122, 1, 14, 6, 123, 0, 6, 10, 123, 3, 10, 14, 123, 2, 14, 14, 123, 1, 14, 6, 124, 0, 6, 16, 124, 3, 16, 8, 124, 2, 8, 5, 124, 1, 5, 6, 125, 0, 6, 10, 125, 3, 10, 15, 125, 2, 15, 5, 125, 1, 5, 6, 126, 0, 6, 3, 126, 3, 3, 9, 126, 2, 9, 11, 126, 1, 11, 6, 127, 0, 6, 15, 127, 3, 15, 13, 127, 2, 13, 11, 127, 1, 11, 6, 128, 0, 6, 1, 128, 3, 1, 2, 128, 2, 2, 11, 128, 1, 11, 6, 129, 0, 6, 5, 129, 3, 5, 14, 129, 2, 14, 11, 129, 1, 11, 6, 130, 0, 6, 14, 130, 3, 14, 3, 130, 2, 3, 15, 130, 1, 15, 6, 131, 0, 6, 9, 131, 3, 9, 8, 131, 2, 8, 15, 131, 1, 15, 6, 132, 0, 6, 11, 132, 3, 11, 9, 132, 2, 9, 16, 132, 1, 16, 6, 133, 0, 6, 2, 133, 3, 2, 7, 133, 2, 7, 16, 133, 1, 16, 6, 134, 0, 6, 1, 134, 3, 1, 2, 134, 2, 2, 16, 134, 1, 16, 6, 135, 0, 6, 14, 135, 3, 14, 11, 135, 2, 11, 16, 135, 1, 16, 6, 136, 0, 12, 4, 136, 3, 4, 14, 136, 2, 14, 12, 136, 1, 12, 12, 137, 0, 12, 1, 137, 3, 1, 3, 137, 2, 3, 3, 137, 1, 3, 12, 138, 0, 12, 7, 138, 3, 7, 3, 138, 2, 3, 3, 138, 1, 3, 12, 139, 0, 12, 8, 139, 3, 8, 3, 139, 2, 3, 3, 139, 1, 3, 12, 140, 0, 12, 15, 140, 3, 15, 13, 140, 2, 13, 3, 140, 1, 3, 12, 141, 0, 12, 3, 141, 3, 3, 10, 141, 2, 10, 3, 141, 1, 3, 12, 142, 0, 12, 8, 142, 3, 8, 11, 142, 2, 11, 3, 142, 1, 3, 12, 143, 0, 12, 15, 143, 3, 15, 15, 143, 2, 15, 3, 143, 1, 3, 12, 144, 0, 12, 2, 144, 3, 2, 1, 144, 2, 1, 9, 144, 1, 9, 12, 145, 0, 12, 8, 145, 3, 8, 12, 145, 2, 12, 1, 145, 1, 1, 12, 146, 0, 12, 16, 146, 3, 16, 12, 146, 2, 12, 1, 146, 1, 1, 12, 147, 0, 12, 9, 147, 3, 9, 1, 147, 2, 1, 1, 147, 1, 1, 12, 148, 0, 12, 8, 148, 3, 8, 10, 148, 2, 10, 1, 148, 1, 1, 12, 149, 0, 12, 7, 149, 3, 7, 3, 149, 2, 3, 13, 149, 1, 13, 12, 150, 0, 12, 16, 150, 3, 16, 2, 150, 2, 2, 13, 150, 1, 13, 12, 151, 0, 12, 7, 151, 3, 7, 7, 151, 2, 7, 4, 151, 1, 4, 12, 152, 0, 12, 13, 152, 3, 13, 7, 152, 2, 7, 4, 152, 1, 4, 12, 153, 0, 12, 11, 153, 3, 11, 16, 153, 2, 16, 4, 153, 1, 4, 12, 154, 0, 12, 4, 154, 3, 4, 4, 154, 2, 4, 2, 154, 1, 2, 12, 155, 0, 12, 8, 155, 3, 8, 4, 155, 2, 4, 2, 155, 1, 2, 12, 156, 0, 12, 4, 156, 3, 4, 16, 156, 2, 16, 2, 156, 1, 2, 12, 157, 0, 12, 11, 157, 3, 11, 4, 157, 2, 4, 14, 157, 1, 14, 12, 158, 0, 12, 3, 158, 3, 3, 5, 158, 2, 5, 14, 158, 1, 14, 12, 159, 0, 12, 15, 159, 3, 15, 15, 159, 2, 15, 5, 159, 1, 5, 12, 160, 0, 12, 14, 160, 3, 14, 3, 160, 2, 3, 11, 160, 1, 11, 12, 161, 0, 12, 14, 161, 3, 14, 9, 161, 2, 9, 11, 161, 1, 11, 12, 162, 0, 12, 7, 162, 3, 7, 10, 162, 2, 10, 16, 162, 1, 16, 12, 163, 0, 12, 11, 163, 3, 11, 2, 163, 2, 2, 16, 163, 1, 16, 12, 164, 0, 3, 2, 164, 3, 2, 1, 164, 2, 1, 9, 164, 1, 9, 3, 165, 0, 3, 5, 165, 3, 5, 2, 165, 2, 2, 9, 165, 1, 9, 3, 166, 0, 3, 1, 166, 3, 1, 1, 166, 2, 1, 7, 166, 1, 7, 3, 167, 0, 3, 10, 167, 3, 10, 7, 167, 2, 7, 13, 167, 1, 13, 3, 168, 0, 3, 15, 168, 3, 15, 13, 168, 2, 13, 13, 168, 1, 13, 3, 169, 0, 3, 5, 169, 3, 5, 2, 169, 2, 2, 4, 169, 1, 4, 3, 170, 0, 3, 11, 170, 3, 11, 14, 170, 2, 14, 4, 170, 1, 4, 3, 171, 0, 3, 7, 171, 3, 7, 15, 171, 2, 15, 4, 171, 1, 4, 3, 172, 0, 3, 10, 172, 3, 10, 10, 172, 2, 10, 10, 172, 1, 10, 3, 173, 0, 3, 14, 173, 3, 14, 8, 173, 2, 8, 10, 173, 1, 10, 3, 174, 0, 3, 15, 174, 3, 15, 4, 174, 2, 4, 2, 174, 1, 2, 3, 175, 0, 3, 13, 175, 3, 13, 4, 175, 2, 4, 8, 175, 1, 8, 3, 176, 0, 3, 2, 176, 3, 2, 9, 176, 2, 9, 14, 176, 1, 14, 3, 177, 0, 3, 11, 177, 3, 11, 8, 177, 2, 8, 5, 177, 1, 5, 3, 178, 0, 3, 10, 178, 3, 10, 5, 178, 2, 5, 5, 178, 1, 5, 3, 179, 0, 3, 11, 179, 3, 11, 4, 179, 2, 4, 11, 179, 1, 11, 3, 180, 0, 3, 11, 180, 3, 11, 7, 180, 2, 7, 15, 180, 1, 15, 3, 181, 0, 7, 4, 181, 3, 4, 15, 181, 2, 15, 3, 181, 1, 3, 7, 182, 0, 5, 13, 182, 3, 13, 9, 182, 2, 9, 9, 182, 1, 9, 5, 183, 0, 9, 4, 183, 3, 4, 16, 183, 2, 16, 1, 183, 1, 1, 9, 184, 0, 9, 8, 184, 3, 8, 13, 184, 2, 13, 7, 184, 1, 7, 9, 185, 0, 9, 5, 185, 3, 5, 5, 185, 2, 5, 7, 185, 1, 7, 9, 186, 0, 9, 4, 186, 3, 4, 15, 186, 2, 15, 13, 186, 1, 13, 9, 187, 0, 9, 15, 187, 3, 15, 9, 187, 2, 9, 4, 187, 1, 4, 9, 188, 0, 9, 13, 188, 3, 13, 1, 188, 2, 1, 4, 188, 1, 4, 9, 189, 0, 9, 13, 189, 3, 13, 11, 189, 2, 11, 4, 189, 1, 4, 9, 190, 0, 9, 1, 190, 3, 1, 1, 190, 2, 1, 10, 190, 1, 10, 9, 191, 0, 9, 14, 191, 3, 14, 10, 191, 2, 10, 10, 191, 1, 10, 9, 192, 0, 9, 4, 192, 3, 4, 7, 192, 2, 7, 2, 192, 1, 2, 9, 193, 0, 9, 7, 193, 3, 7, 15, 193, 2, 15, 8, 193, 1, 8, 9, 194, 0, 9, 16, 194, 3, 16, 15, 194, 2, 15, 8, 194, 1, 8, 9, 195, 0, 9, 10, 195, 3, 10, 16, 195, 2, 16, 8, 195, 1, 8, 9, 196, 0, 9, 16, 196, 3, 16, 7, 196, 2, 7, 14, 196, 1, 14, 9, 197, 0, 9, 5, 197, 3, 5, 13, 197, 2, 13, 14, 197, 1, 14, 9, 198, 0, 9, 14, 198, 3, 14, 14, 198, 2, 14, 14, 198, 1, 14, 9, 199, 0, 9, 1, 199, 3, 1, 13, 199, 2, 13, 5, 199, 1, 5, 9, 200, 0, 9, 5, 200, 3, 5, 15, 200, 2, 15, 11, 200, 1, 11, 9, 201, 0, 9, 4, 201, 3, 4, 16, 201, 2, 16, 11, 201, 1, 11, 9, 202, 0, 9, 13, 202, 3, 13, 8, 202, 2, 8, 15, 202, 1, 15, 9, 203, 0, 9, 5, 203, 3, 5, 8, 203, 2, 8, 15, 203, 1, 15, 9, 204, 0, 9, 1, 204, 3, 1, 11, 204, 2, 11, 15, 204, 1, 15, 9, 205, 0, 9, 15, 205, 3, 15, 16, 205, 2, 16, 15, 205, 1, 15, 9, 206, 0, 1, 4, 206, 3, 4, 16, 206, 2, 16, 1, 206, 1, 1, 1, 207, 0, 1, 16, 207, 3, 16, 13, 207, 2, 13, 7, 207, 1, 7, 1, 208, 0, 1, 10, 208, 3, 10, 5, 208, 2, 5, 7, 208, 1, 7, 1, 209, 0, 1, 13, 209, 3, 13, 7, 209, 2, 7, 4, 209, 1, 4, 1, 210, 0, 1, 10, 210, 3, 10, 11, 210, 2, 11, 4, 210, 1, 4, 1, 211, 0, 1, 11, 211, 3, 11, 1, 211, 2, 1, 10, 211, 1, 10, 1, 212, 0, 1, 5, 212, 3, 5, 5, 212, 2, 5, 2, 212, 1, 2, 1, 213, 0, 1, 14, 213, 3, 14, 1, 213, 2, 1, 8, 213, 1, 8, 1, 214, 0, 1, 16, 214, 3, 16, 2, 214, 2, 2, 8, 214, 1, 8, 1, 215, 0, 1, 15, 215, 3, 15, 16, 215, 2, 16, 8, 215, 1, 8, 1, 216, 0, 1, 10, 216, 3, 10, 13, 216, 2, 13, 14, 216, 1, 14, 1, 217, 0, 1, 8, 217, 3, 8, 2, 217, 2, 2, 11, 217, 1, 11, 1, 218, 0, 1, 2, 218, 3, 2, 7, 218, 2, 7, 15, 218, 1, 15, 1, 219, 0, 1, 11, 219, 3, 11, 7, 219, 2, 7, 16, 219, 1, 16, 1, 220, 0, 1, 15, 220, 3, 15, 15, 220, 2, 15, 16, 220, 1, 16, 1, 221, 0, 1, 16, 221, 3, 16, 15, 221, 2, 15, 16, 221, 1, 16, 1, 222, 0, 7, 16, 222, 3, 16, 8, 222, 2, 8, 7, 222, 1, 7, 7, 223, 0, 7, 4, 223, 3, 4, 14, 223, 2, 14, 7, 223, 1, 7, 7, 224, 0, 7, 10, 224, 3, 10, 11, 224, 2, 11, 7, 224, 1, 7, 7, 225, 0, 7, 10, 225, 3, 10, 13, 225, 2, 13, 4, 225, 1, 4, 7, 226, 0, 7, 8, 226, 3, 8, 8, 226, 2, 8, 4, 226, 1, 4, 7, 227, 0, 7, 5, 227, 3, 5, 13, 227, 2, 13, 2, 227, 1, 2, 7, 228, 0, 7, 16, 228, 3, 16, 4, 228, 2, 4, 14, 228, 1, 14, 7, 229, 0, 7, 5, 229, 3, 5, 11, 229, 2, 11, 14, 229, 1, 14, 7, 230, 0, 7, 14, 230, 3, 14, 8, 230, 2, 8, 5, 230, 1, 5, 7, 231, 0, 7, 15, 231, 3, 15, 8, 231, 2, 8, 5, 231, 1, 5, 7, 232, 0, 13, 14, 232, 3, 14, 13, 232, 2, 13, 13, 232, 1, 13, 13, 233, 0, 13, 2, 233, 3, 2, 11, 233, 2, 11, 4, 233, 1, 4, 13, 234, 0, 13, 2, 234, 3, 2, 4, 234, 2, 4, 2, 234, 1, 2, 13, 235, 0, 13, 14, 235, 3, 14, 4, 235, 2, 4, 2, 235, 1, 2, 13, 236, 0, 13, 10, 236, 3, 10, 8, 236, 2, 8, 2, 236, 1, 2, 13, 237, 0, 13, 11, 237, 3, 11, 2, 237, 2, 2, 8, 237, 1, 8, 13, 238, 0, 13, 16, 238, 3, 16, 10, 238, 2, 10, 14, 238, 1, 14, 13, 239, 0, 13, 10, 239, 3, 10, 15, 239, 2, 15, 14, 239, 1, 14, 13, 240, 0, 13, 15, 240, 3, 15, 4, 240, 2, 4, 5, 240, 1, 5, 13, 241, 0, 13, 15, 241, 3, 15, 2, 241, 2, 2, 5, 241, 1, 5, 13, 242, 0, 4, 8, 242, 3, 8, 10, 242, 2, 10, 10, 242, 1, 10, 4, 243, 0, 4, 5, 243, 3, 5, 14, 243, 2, 14, 10, 243, 1, 10, 4, 244, 0, 4, 14, 244, 3, 14, 16, 244, 2, 16, 2, 244, 1, 2, 4, 245, 0, 4, 16, 245, 3, 16, 11, 245, 2, 11, 15, 245, 1, 15, 4, 246, 0, 10, 16, 246, 3, 16, 15, 246, 2, 15, 10, 246, 1, 10, 10, 247, 0, 10, 2, 247, 3, 2, 8, 247, 2, 8, 2, 247, 1, 2, 10, 248, 0, 10, 15, 248, 3, 15, 2, 248, 2, 2, 8, 248, 1, 8, 10, 249, 0, 10, 15, 249, 3, 15, 11, 249, 2, 11, 14, 249, 1, 14, 10, 250, 0, 2, 5, 250, 3, 5, 16, 250, 2, 16, 2, 250, 1, 2, 2, 251, 0, 2, 8, 251, 3, 8, 5, 251, 2, 5, 8, 251, 1, 8, 2, 252, 0, 8, 14, 252, 3, 14, 11, 252, 2, 11, 5, 252, 1, 5, 8, 253, 0, 14, 11, 253, 3, 11, 15, 253, 2, 15, 11, 253, 1, 11, 14, 254, 0, 14, 16, 254, 3, 16, 11, 254, 2, 11, 16, 254, 1, 16, 14, 255, 0, 5, 16, 255, 3, 16, 15, 255, 2, 15, 16, 255, 1, 16, 5,
];
