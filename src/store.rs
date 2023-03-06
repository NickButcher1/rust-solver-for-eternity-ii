#[cfg(feature = "backtracker-full")]
use crate::autogenfull::{
    BICOLOUR_TILES, DISPLAY_TO_FILL_ORDER, NUM_CELLS, NUM_COLS, NUM_ROWS, NUM_TILES, TILES,
};
#[cfg(feature = "backtracker-mids")]
use crate::autogenmids::{
    BICOLOUR_TILES, DISPLAY_TO_FILL_ORDER, NUM_CELLS, NUM_COLS, NUM_ROWS, NUM_TILES, TILES,
};
use crate::colour::{BUCAS_LETTER, GREY};
use crate::threadparams::ThreadParams;
use std::fs;
use std::fs::File;
use std::io::Write;
use string_builder::Builder;

static BASE_URL: &str = "https://e2.bucas.name/#puzzle=NickB&board_w=16&board_h=16&motifs_order=jblackwood&board_edges=";
static BOARD_PIECES_PARAM: &str = "&board_pieces=";

fn north_colour(tiles_idx: [usize; NUM_TILES], depth: usize) -> usize {
    colour_for_side(tiles_idx, depth, 0)
}

fn east_colour(tiles_idx: [usize; NUM_TILES], depth: usize) -> usize {
    colour_for_side(tiles_idx, depth, 1)
}

fn south_colour(tiles_idx: [usize; NUM_TILES], depth: usize) -> usize {
    colour_for_side(tiles_idx, depth, 2)
}

fn west_colour(tiles_idx: [usize; NUM_TILES], depth: usize) -> usize {
    colour_for_side(tiles_idx, depth, 3)
}

fn colour_for_side(tiles_idx: [usize; NUM_TILES], depth: usize, ori: usize) -> usize {
    TILES[((to_id(tiles_idx, depth) * 4) + ((to_ori(tiles_idx, depth) + ori) % 4) as usize)]
        as usize
}
pub fn save_board(thread_params: &ThreadParams, tiles_idx: [usize; NUM_TILES], depth: usize) {
    let mut builder = Builder::default();
    for row in 0..NUM_ROWS {
        for col in 0..NUM_COLS {
            let display_idx = row * NUM_COLS + col;
            let idx_i16 = DISPLAY_TO_FILL_ORDER[display_idx];
            let idx = idx_i16 as usize;

            if idx_i16 != -1 && idx <= depth {
                let real_tile_id = to_real_tile_id(to_id(tiles_idx, idx));
                builder.append(format!("{:>3}/{} ", real_tile_id, to_ori(tiles_idx, idx)));
            } else {
                builder.append("---/- ");
            }
        }
        builder.append("\n");
    }

    if NUM_TILES >= 196 {
        builder.append("\n");
        builder.append(BASE_URL);
        // let mut idx: usize = 0;
        for row in 0..NUM_ROWS {
            for col in 0..NUM_COLS {
                let display_idx = row * NUM_COLS + col;
                let idx_i16 = DISPLAY_TO_FILL_ORDER[display_idx];
                if idx_i16 != -1 && (idx_i16 as usize) <= depth {
                    let idx = idx_i16 as usize;
                    builder.append(format!("{}", BUCAS_LETTER[north_colour(tiles_idx, idx)]));
                    builder.append(format!("{}", BUCAS_LETTER[east_colour(tiles_idx, idx)]));
                    builder.append(format!("{}", BUCAS_LETTER[south_colour(tiles_idx, idx)]));
                    builder.append(format!("{}", BUCAS_LETTER[west_colour(tiles_idx, idx)]));
                } else {
                    add_empty_cell(&mut builder);
                }
            }
        }

        builder.append(BOARD_PIECES_PARAM);

        for idx_i16 in DISPLAY_TO_FILL_ORDER.iter().take(NUM_CELLS) {
            let idx = *idx_i16 as usize;

            if *idx_i16 != -1 && idx <= depth {
                let real_tile_id = to_real_tile_id(to_id(tiles_idx, idx));
                builder.append(format!("{:0>3}", real_tile_id));
            } else {
                builder.append("000");
            }
        }
        builder.append("\n");
    }

    let board_string = builder.string().unwrap();

    let digest = md5::compute(&board_string);
    let uuid = digest;

    let filename = format!(
        "{}_{:?}_{}_{}.txt",
        depth + 1,
        uuid,
        thread_params.run_id,
        thread_params.thread_id
    );

    #[allow(deprecated)]
    let mut solutions_dir = std::env::home_dir().unwrap();
    solutions_dir.push("EternityIISolutions");
    fs::create_dir_all(&solutions_dir).unwrap();
    solutions_dir.push(filename);

    let mut file = match File::create(&solutions_dir) {
        Err(why) => panic!("Couldn't create {:?}: {}", solutions_dir, why),
        Ok(file) => file,
    };

    file.write_all(board_string.as_bytes())
        .unwrap_or_else(|_| panic!("Couldn't write to {:?}", solutions_dir));
}

fn add_empty_cell(builder: &mut Builder) {
    builder.append(format!("{}", BUCAS_LETTER[GREY as usize]));
    builder.append(format!("{}", BUCAS_LETTER[GREY as usize]));
    builder.append(format!("{}", BUCAS_LETTER[GREY as usize]));
    builder.append(format!("{}", BUCAS_LETTER[GREY as usize]));
}

fn to_id(tiles_idx: [usize; NUM_TILES], idx: usize) -> usize {
    BICOLOUR_TILES[tiles_idx[idx]] as usize
}

fn to_ori(tiles_idx: [usize; NUM_TILES], idx: usize) -> usize {
    BICOLOUR_TILES[tiles_idx[idx] + 1] as usize
}

// Convert zero based tile ID back to real tile ID.
fn to_real_tile_id(id: usize) -> usize {
    if cfg!(feature = "backtracker-mids") {
        id + 61
    } else {
        id + 1
    }
}
