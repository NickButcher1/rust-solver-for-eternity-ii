use crate::autogen::{DISPLAY_TO_FILL_ORDER, NUM_COLS, NUM_MIDS, NUM_ROWS, NUM_TILES, TILES};
use crate::colour::{BUCAS_LETTER, GREY};
use crate::threadparams::ThreadParams;
use crate::PLACE_MIDS_ONLY;
use std::fs;
use std::fs::File;
use std::io::Write;
use string_builder::Builder;

static BASE_URL: &str = "https://e2.bucas.name/#puzzle=NickB&board_w=16&board_h=16&motifs_order=jblackwood&board_edges=";
static BOARD_PIECES_PARAM: &str = "&board_pieces=";

pub fn save_board_mids(
    thread_params: &ThreadParams,
    ids: [u8; NUM_MIDS],
    oris: [u8; NUM_MIDS],
    depth: usize,
) {
    save_board_internal(thread_params, &ids, &oris, depth);
}

pub fn save_board(
    thread_params: &ThreadParams,
    ids: [u8; NUM_TILES],
    oris: [u8; NUM_TILES],
    depth: usize,
) {
    save_board_internal(thread_params, &ids, &oris, depth);
}

fn save_board_internal(thread_params: &ThreadParams, ids: &[u8], oris: &[u8], depth: usize) {
    let mut builder = Builder::default();
    for row in 0..NUM_ROWS {
        for col in 0..NUM_COLS {
            let display_idx = row * NUM_COLS + col;
            let idx_i16 = DISPLAY_TO_FILL_ORDER[display_idx];
            let idx = idx_i16 as usize;

            if idx_i16 != -1 && idx <= depth {
                let real_tile_id = to_real_tile_id(ids[idx]);
                builder.append(format!("{:>3}/{} ", real_tile_id, oris[idx]));
            } else {
                builder.append("---/- ");
            }
        }
        builder.append("\n");
    }

    if NUM_TILES == 256 {
        builder.append("\n");
        builder.append(BASE_URL);
        for row in 0..NUM_ROWS {
            for col in 0..NUM_COLS {
                let display_idx = row * NUM_COLS + col;
                let idx_i16 = DISPLAY_TO_FILL_ORDER[display_idx];
                let idx = idx_i16 as usize;

                if idx_i16 != -1 && idx <= depth {
                    builder.append(format!(
                        "{}",
                        BUCAS_LETTER
                            [TILES[((ids[idx] as usize * 4) + oris[idx] as usize)] as usize]
                    ));
                    builder.append(format!(
                        "{}",
                        BUCAS_LETTER[TILES
                            [((ids[idx] as usize * 4) + ((oris[idx] + 1) % 4) as usize)]
                            as usize]
                    ));
                    builder.append(format!(
                        "{}",
                        BUCAS_LETTER[TILES
                            [((ids[idx] as usize * 4) + ((oris[idx] + 2) % 4) as usize)]
                            as usize]
                    ));
                    builder.append(format!(
                        "{}",
                        BUCAS_LETTER[TILES
                            [((ids[idx] as usize * 4) + ((oris[idx] + 3) % 4) as usize)]
                            as usize]
                    ));
                } else {
                    add_empty_cell(&mut builder);
                }
            }
        }

        builder.append(BOARD_PIECES_PARAM);

        for idx_i16 in DISPLAY_TO_FILL_ORDER.iter().take(NUM_TILES) {
            let idx = *idx_i16 as usize;

            if *idx_i16 != -1 && idx <= depth {
                let real_tile_id = to_real_tile_id(ids[idx]);
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

// Convert zero based tile ID back to real tile ID.
fn to_real_tile_id(id: u8) -> u32 {
    if PLACE_MIDS_ONLY {
        (id as u32) + 61
    } else {
        (id as u32) + 1
    }
}
