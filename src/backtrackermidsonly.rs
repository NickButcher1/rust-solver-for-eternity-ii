use crate::autogen::{Cell, ANY_COLOUR, BICOLOUR_TILES, FILL_ORDER, MIDS_BICOLOUR_ARRAY, NUM_MIDS};
use crate::celltype::{MID, MID_LEFT, MID_TOP, MID_TOP_LEFT};
use crate::store;
use crate::threadparams::ThreadParams;
use separator::Separatable;
use std::thread;
use std::time::Duration;
use std::time::SystemTime;

// Hard coded, not configurable, so the compiler can optimise the code away when turned off.
static RECORD_DEPTH_STATS: bool = true;
static RECORD_DEPTH_SOLUTIONS: bool = true;
const MAX_THREADS: usize = 8;

static mut MAX_DEPTH: [usize; MAX_THREADS] = [170; MAX_THREADS];
static mut NUM_SOLUTIONS: [u64; MAX_THREADS] = [0; MAX_THREADS];
static mut NUM_SOLUTIONS_AT_DEPTH: [[u64; NUM_MIDS]; MAX_THREADS] = [[0; NUM_MIDS]; MAX_THREADS];

fn print_num_solutions(thread_params: ThreadParams, elapsed_seconds: u64) {
    let mut total: u64 = 0;

    if RECORD_DEPTH_STATS {
        unsafe {
            for (idx, num_solutions_at_depth) in
                NUM_SOLUTIONS_AT_DEPTH[thread_params.thread_id].iter().enumerate().take(NUM_MIDS)
            {
                if !thread_params.is_mt_mode {
                    println!(
                        "{:>3} -> {}",
                        idx + 1,
                        num_solutions_at_depth.separated_string()
                    );
                }
                total += num_solutions_at_depth;

                // Don't waste space printing the rest of the zeroes.
                if *num_solutions_at_depth == 0 {
                    break;
                }
            }
        }
    }

    if elapsed_seconds != 0 {
        let total_per_second = total / elapsed_seconds;
        unsafe {
            println!(
                "THREAD {}  SOLUTIONS: {}  BEST: {}  TOTAL: {}    {} per second    {} seconds",
                thread_params.thread_id,
                NUM_SOLUTIONS[thread_params.thread_id],
                MAX_DEPTH[thread_params.thread_id] + 1,
                total.separated_string(),
                total_per_second.separated_string(),
                elapsed_seconds.separated_string()
            );
        }
    }
}

/**
 * A simple backtracker for the mid tiles only.
 *
 * For now, only north and west edges are checked for matches when placing a tile. There is no reason that south and
 * east matching can't be added.
 */
pub struct BacktrackerMidsOnly<'a> {
    thread_params: ThreadParams,
    max_depth: &'a mut usize,
    num_solutions: &'a mut u64,
    num_solutions_at_depth: &'a mut [u64; NUM_MIDS],
    // Whether each tile ID has been placed in placed_tiles or not. Used to prevent placing duplicates.
    used_tiles: [bool; NUM_MIDS],
    // The tiles that have currently been placed. Only valid up to the current depth - values are not cleared for better performance.
    placed_ids: [u8; NUM_MIDS],
    placed_oris: [u8; NUM_MIDS],
    placed_south_colour: [u8; NUM_MIDS],
    placed_east_colour: [u8; NUM_MIDS],
}

impl BacktrackerMidsOnly<'_> {
    pub fn new(thread_params: ThreadParams) -> Self {
        unsafe {
            Self {
                thread_params,
                max_depth: &mut MAX_DEPTH[thread_params.thread_id],
                num_solutions: &mut NUM_SOLUTIONS[thread_params.thread_id],
                num_solutions_at_depth: &mut NUM_SOLUTIONS_AT_DEPTH[thread_params.thread_id],
                used_tiles: [false; NUM_MIDS],
                placed_ids: [0; NUM_MIDS],
                placed_oris: [0; NUM_MIDS],
                placed_south_colour: [0; NUM_MIDS],
                placed_east_colour: [0; NUM_MIDS],
            }
        }
    }

    pub fn solve(&mut self) {
        let thread_params = self.thread_params;
        thread::spawn(move || {
            let start_time = SystemTime::now();

            loop {
                let elapsed_time = start_time.elapsed();
                let elapsed_seconds = elapsed_time.unwrap().as_secs();
                print_num_solutions(thread_params, elapsed_seconds);
                thread::sleep(Duration::from_millis(thread_params.stats_every_s));
            }
        });

        let start_time = SystemTime::now();
        self.add_tile_0();
        let elapsed_time = start_time.elapsed().unwrap().as_secs();
        print_num_solutions(self.thread_params, elapsed_time);
    }

    fn add_tile_0(&mut self) {
        let cell: &Cell = &FILL_ORDER[0];
        assert_eq!(cell.cell_type, MID_TOP_LEFT);
        let tileoris_offset: u16 = MIDS_BICOLOUR_ARRAY[ANY_COLOUR as usize][ANY_COLOUR as usize];
        let num_tiles: usize = NUM_MIDS * 4;

        if self.thread_params.is_mt_mode {
            let tiles_idx: usize = tileoris_offset as usize + 2 + 4 * self.thread_params.tile_0_idx;
            self.add_tile_0_inner(tiles_idx);
        } else {
            for tiles_idx in ((tileoris_offset as u32 + 2_u32)
                ..(tileoris_offset as u32 + 2_u32 + (num_tiles * 4) as u32))
                .step_by(4)
            {
                self.add_tile_0_inner(tiles_idx as usize);
            }
        }
    }

    fn add_tile_0_inner(&mut self, tiles_idx: usize) {
        let id = BICOLOUR_TILES[tiles_idx];
        let id_usize = id as usize;

        assert!(!self.used_tiles[id_usize]);
        self.used_tiles[id_usize] = true;
        self.placed_ids[0] = id;
        self.placed_oris[0] = BICOLOUR_TILES[tiles_idx + 1];
        self.placed_south_colour[0] = BICOLOUR_TILES[tiles_idx + 2];
        self.placed_east_colour[0] = BICOLOUR_TILES[tiles_idx + 3];

        if RECORD_DEPTH_STATS {
            self.num_solutions_at_depth[0] += 1;
        }

        self.add_tile(1);

        // No need to remove from placed_ids or placed_oris here. Ditto placed_south_colour and placed_east_colour.
        self.used_tiles[id_usize] = false;
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
                    self.num_solutions_at_depth[depth] += 1;
                }

                if RECORD_DEPTH_SOLUTIONS {
                    if depth > *self.max_depth {
                        *self.max_depth = depth;
                        store::save_board_mids(
                            &self.thread_params,
                            self.placed_ids,
                            self.placed_oris,
                            depth,
                        );
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

    fn record_solution(&mut self) {
        *self.num_solutions += 1;

        store::save_board_mids(
            &self.thread_params,
            self.placed_ids,
            self.placed_oris,
            NUM_MIDS - 1,
        );
    }
}
