use crate::autogen::{
    Cell, BICOLOUR_TILES, BOTTOM_EDGES_BICOLOUR_ARRAY, BOTTOM_LEFT_CORNER_COLOUR_ARRAY,
    BOTTOM_RIGHT_CORNER_BICOLOUR_ARRAY, FILL_ORDER, LEFT_EDGES_COLOUR_ARRAY, MIDS_BICOLOUR_ARRAY,
    NUM_TILES, RIGHT_EDGES_BICOLOUR_ARRAY, TOP_EDGES_COLOUR_ARRAY, TOP_LEFT_CORNER_OFFSET,
    TOP_RIGHT_CORNER_COLOUR_ARRAY,
};
use crate::celltype::{
    CORNER_BOTTOM_LEFT, CORNER_BOTTOM_RIGHT, CORNER_TOP_LEFT, CORNER_TOP_RIGHT, EDGE_BOTTOM,
    EDGE_LEFT, EDGE_RIGHT, EDGE_TOP, MID,
};
use crate::{store, ThreadParams};
use separator::Separatable;
use std::thread;
use std::time::Duration;
use std::time::SystemTime;

// Hard coded, not configurable, so the compiler can optimise the code away when turned off.
static RECORD_DEPTH_STATS: bool = true;
static RECORD_DEPTH_SOLUTIONS: bool = true;
const MAX_THREADS: usize = 8;

static mut MAX_DEPTH: [usize; MAX_THREADS] = [200; MAX_THREADS];
static mut NUM_SOLUTIONS: [u64; MAX_THREADS] = [0; MAX_THREADS];
static mut NUM_SOLUTIONS_AT_DEPTH: [[u64; NUM_TILES]; MAX_THREADS] = [[0; NUM_TILES]; MAX_THREADS];

fn print_num_solutions(thread_params: ThreadParams, elapsed_seconds: u64) {
    let mut total: u64 = 0;

    unsafe {
        for (idx, num_solutions_at_depth) in
            NUM_SOLUTIONS_AT_DEPTH[thread_params.thread_id].iter().enumerate().take(NUM_TILES)
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
 * A simple backtracker.
 *
 * For now, only north and west edges are checked for matches when placing a tile. There is no reason that south and
 * east matching can't be added.
 */
pub struct Backtracker<'a> {
    thread_params: ThreadParams,
    max_depth: &'a mut usize,
    num_solutions: &'a mut u64,
    num_solutions_at_depth: &'a mut [u64; NUM_TILES],
    // Whether each tile ID has been placed in placed_tiles or not. Used to prevent placing duplicates.
    used_tiles: [bool; NUM_TILES],
    // The tiles that have currently been placed. Only valid up to the current depth - values are not cleared for better performance.
    placed_ids: [u8; NUM_TILES],
    placed_oris: [u8; NUM_TILES],
    placed_south_colour: [u8; NUM_TILES],
    placed_east_colour: [u8; NUM_TILES],
}

impl Backtracker<'_> {
    pub fn new(thread_params: ThreadParams) -> Self {
        unsafe {
            Self {
                thread_params,
                max_depth: &mut MAX_DEPTH[thread_params.thread_id],
                num_solutions: &mut NUM_SOLUTIONS[thread_params.thread_id],
                num_solutions_at_depth: &mut NUM_SOLUTIONS_AT_DEPTH[thread_params.thread_id],
                used_tiles: [false; NUM_TILES],
                placed_ids: [0; NUM_TILES],
                placed_oris: [0; NUM_TILES],
                placed_south_colour: [0; NUM_TILES],
                placed_east_colour: [0; NUM_TILES],
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
        self.add_tile(0);
        let elapsed_time = start_time.elapsed().unwrap().as_secs();
        print_num_solutions(self.thread_params, elapsed_time);
    }

    fn add_tile(&mut self, depth: usize) {
        let cell: &Cell = &FILL_ORDER[depth];
        // println!("CELL: {:?}, DEPTH {}", cell, depth);

        let tileoris_offset: u16 = match cell.cell_type {
            MID => {
                MIDS_BICOLOUR_ARRAY[self.placed_south_colour[cell.north_idx as usize] as usize]
                    [self.placed_east_colour[cell.west_idx as usize] as usize]
            }

            EDGE_RIGHT => {
                RIGHT_EDGES_BICOLOUR_ARRAY
                    [self.placed_south_colour[cell.north_idx as usize] as usize]
                    [self.placed_east_colour[cell.west_idx as usize] as usize]
            }

            EDGE_LEFT => {
                LEFT_EDGES_COLOUR_ARRAY[self.placed_south_colour[cell.north_idx as usize] as usize]
            }

            EDGE_TOP => {
                TOP_EDGES_COLOUR_ARRAY[self.placed_east_colour[cell.west_idx as usize] as usize]
            }

            EDGE_BOTTOM => {
                BOTTOM_EDGES_BICOLOUR_ARRAY
                    [self.placed_south_colour[cell.north_idx as usize] as usize]
                    [self.placed_east_colour[cell.west_idx as usize] as usize]
            }

            CORNER_TOP_RIGHT => {
                TOP_RIGHT_CORNER_COLOUR_ARRAY
                    [self.placed_east_colour[cell.west_idx as usize] as usize]
            }

            CORNER_BOTTOM_LEFT => {
                BOTTOM_LEFT_CORNER_COLOUR_ARRAY
                    [self.placed_south_colour[cell.north_idx as usize] as usize]
            }

            CORNER_TOP_LEFT => TOP_LEFT_CORNER_OFFSET,

            CORNER_BOTTOM_RIGHT => {
                BOTTOM_RIGHT_CORNER_BICOLOUR_ARRAY
                    [self.placed_south_colour[cell.north_idx as usize] as usize]
                    [self.placed_east_colour[cell.west_idx as usize] as usize]
            }

            other => panic!("Invalid cell_type {}", other),
        };

        let num_tiles = BICOLOUR_TILES[tileoris_offset as usize];
        if num_tiles == 0 {
            return;
        }

        for tiles_idx in
            ((tileoris_offset + 2)..(tileoris_offset + 2 + (num_tiles * 4) as u16)).step_by(4)
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
                        store::save_board(
                            &self.thread_params,
                            self.placed_ids,
                            self.placed_oris,
                            depth,
                        );
                    }
                }

                if depth == (NUM_TILES - 1) {
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

        store::save_board(
            &self.thread_params,
            self.placed_ids,
            self.placed_oris,
            NUM_TILES - 1,
        );
    }
}
