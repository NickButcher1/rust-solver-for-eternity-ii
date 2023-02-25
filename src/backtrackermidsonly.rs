use crate::autogen::{Cell, ANY_COLOUR, BICOLOUR_TILES, FILL_ORDER, MIDS_BICOLOUR_ARRAY, NUM_MIDS};
use crate::celltype::{MID, MID_LEFT, MID_TOP, MID_TOP_LEFT};
use crate::store;
use separator::Separatable;
use std::thread;
use std::time::Duration;
use std::time::SystemTime;

// Hard coded, not configurable, so the compiler can optimise the code away when turned off.
static RECORD_DEPTH_STATS: bool = true;
static RECORD_DEPTH_SOLUTIONS: bool = true;
static mut MAX_DEPTH: usize = 170;

static mut NUM_SOLUTIONS: u64 = 0;
static mut NUM_SOLUTIONS_AT_DEPTH: [u64; NUM_MIDS] = [0; NUM_MIDS];

fn print_num_solutions(elapsed_seconds: u64) {
    unsafe {
        println!("NUM_SOLUTIONS {}", NUM_SOLUTIONS);
    }
    println!("NUM_SOLUTIONS_AT_DEPTH");
    let mut total: u64 = 0;

    unsafe {
        for (idx, num_solutions_at_depth) in
            NUM_SOLUTIONS_AT_DEPTH.iter().enumerate().take(NUM_MIDS)
        {
            println!(
                "{:>3} -> {}",
                idx + 1,
                num_solutions_at_depth.separated_string()
            );
            total += num_solutions_at_depth;

            // Don't waste space printing the rest of the zeroes.
            if *num_solutions_at_depth == 0 {
                break;
            }
        }
    }

    if elapsed_seconds != 0 {
        let total_per_second = total / elapsed_seconds;
        println!(
            "TOTAL: {}    {} per second    {} seconds",
            total.separated_string(),
            total_per_second.separated_string(),
            elapsed_seconds.separated_string()
        );
    }
}

/**
 * A simple backtracker for the mid tiles only.
 *
 * For now, only north and west edges are checked for matches when placing a tile. There is no reason that south and
 * east matching can't be added.
 */
pub struct BacktrackerMidsOnly {
    // Whether each tile ID has been placed in placed_tiles or not. Used to prevent placing duplicates.
    used_tiles: [bool; NUM_MIDS],
    // The tiles that have currently been placed. Only valid up to the current depth - values are not cleared for better performance.
    placed_ids: [u8; NUM_MIDS],
    placed_oris: [u8; NUM_MIDS],
    placed_south_colour: [u8; NUM_MIDS],
    placed_east_colour: [u8; NUM_MIDS],
}

impl BacktrackerMidsOnly {
    pub fn new() -> Self {
        Self {
            used_tiles: [false; NUM_MIDS],
            placed_ids: [0; NUM_MIDS],
            placed_oris: [0; NUM_MIDS],
            placed_south_colour: [0; NUM_MIDS],
            placed_east_colour: [0; NUM_MIDS],
        }
    }

    pub fn solve(&mut self) {
        thread::spawn(|| {
            let start_time = SystemTime::now();

            loop {
                let elapsed_time = start_time.elapsed();
                let elapsed_seconds = elapsed_time.unwrap().as_secs();
                print_num_solutions(elapsed_seconds);
                thread::sleep(Duration::from_millis(10_000));
            }
        });

        let start_time = SystemTime::now();
        self.add_tile_0();
        let elapsed_time = start_time.elapsed().unwrap().as_secs();
        print_num_solutions(elapsed_time);
    }

    fn add_tile_0(&mut self,) {
        let cell: &Cell = &FILL_ORDER[0];
        assert_eq!(cell.cell_type, MID_TOP_LEFT);
        let tileoris_offset: u16 = MIDS_BICOLOUR_ARRAY[ANY_COLOUR as usize][ANY_COLOUR as usize];
        let num_tiles: usize = NUM_MIDS * 4;

        for tiles_idx in ((tileoris_offset as u32 + 2_u32)
            ..(tileoris_offset as u32 + 2_u32 + (num_tiles * 4) as u32))
            .step_by(4)
        {
            let tiles_idx_usize = tiles_idx as usize;
            let id = BICOLOUR_TILES[tiles_idx_usize];
            let id_usize = id as usize;

            assert_eq!(self.used_tiles[id_usize], false);
            self.used_tiles[id_usize] = true;
            self.placed_ids[0] = id;
            self.placed_oris[0] = BICOLOUR_TILES[tiles_idx_usize + 1];
            self.placed_south_colour[0] = BICOLOUR_TILES[tiles_idx_usize + 2];
            self.placed_east_colour[0] = BICOLOUR_TILES[tiles_idx_usize + 3];

            if RECORD_DEPTH_STATS {
                unsafe {
                    NUM_SOLUTIONS_AT_DEPTH[0] += 1;
                }
            }

            self.add_tile(1);

            // No need to remove from placed_ids or placed_oris here. Ditto placed_south_colour and placed_east_colour.
            self.used_tiles[id_usize] = false;
        }
    }

    fn add_tile(&mut self, depth: usize) {
        let cell: &Cell = &FILL_ORDER[depth];
        // println!("CELL: {:?}, DEPTH {}", cell, depth);

        let tileoris_offset: u16 = match cell.cell_type {
            MID => {
                MIDS_BICOLOUR_ARRAY[self.placed_south_colour[cell.north_idx as usize] as usize]
                    [self.placed_east_colour[cell.west_idx as usize] as usize]
            }

            MID_TOP => {
                MIDS_BICOLOUR_ARRAY[ANY_COLOUR as usize]
                    [self.placed_east_colour[cell.west_idx as usize] as usize]
            }

            MID_LEFT => {
                MIDS_BICOLOUR_ARRAY[self.placed_south_colour[cell.north_idx as usize] as usize]
                    [ANY_COLOUR as usize]
            }

            other => panic!("Invalid cell_type {}", other),
        };

        let num_tiles: usize = BICOLOUR_TILES[tileoris_offset as usize] as usize;

        if num_tiles == 0 {
            return;
        }

        for tiles_idx in ((tileoris_offset as u32 + 2_u32)
            ..(tileoris_offset as u32 + 2_u32 + (num_tiles * 4) as u32))
            .step_by(4)
        {
            let tiles_idx_usize = tiles_idx as usize;
            let id = BICOLOUR_TILES[tiles_idx_usize];
            let id_usize = id as usize;

            if !self.used_tiles[id_usize] {
                self.used_tiles[id_usize] = true;
                self.placed_ids[depth] = id;
                self.placed_oris[depth] = BICOLOUR_TILES[tiles_idx_usize + 1];
                self.placed_south_colour[depth] = BICOLOUR_TILES[tiles_idx_usize + 2];
                self.placed_east_colour[depth] = BICOLOUR_TILES[tiles_idx_usize + 3];

                if RECORD_DEPTH_STATS {
                    unsafe {
                        NUM_SOLUTIONS_AT_DEPTH[depth] += 1;
                    }
                }

                if RECORD_DEPTH_SOLUTIONS {
                    unsafe {
                        if depth > MAX_DEPTH {
                            MAX_DEPTH = depth;
                            store::save_board_mids(self.placed_ids, self.placed_oris, depth);
                        }
                    }
                }

                if depth == (NUM_MIDS - 1) {
                    self.record_solution();
                } else {
                    self.add_tile(depth + 1);
                }

                // No need to remove from placed_ids or placed_oris here. Ditto placed_south_colour and placed_east_colour.
                self.used_tiles[id_usize] = false;
            }
        }
    }

    fn record_solution(&self) {
        unsafe {
            NUM_SOLUTIONS += 1;
        }
        store::save_board_mids(self.placed_ids, self.placed_oris, NUM_MIDS - 1);
    }
}
