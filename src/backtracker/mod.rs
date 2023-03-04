#[cfg(feature = "backtracker-full")]
use crate::autogenfull::{BICOLOUR_TILES, NUM_TILES};
#[cfg(feature = "backtracker-mids")]
use crate::autogenmids::{BICOLOUR_TILES, NUM_TILES};
use crate::ThreadParams;
use separator::Separatable;
use std::thread;
use std::time::Duration;
use std::time::SystemTime;
use string_builder::Builder;

#[cfg(feature = "backtracker-full")]
mod backtrackerfull;
#[cfg(feature = "backtracker-mids")]
mod backtrackermids;

#[macro_export]
macro_rules! return_if_no_tiles {
    ($num_tiles:expr) => {
        if $num_tiles == 0 {
            return;
        }
    };
}

#[macro_export]
macro_rules! get_num_tiles {
    ($offset:expr) => {
        BICOLOUR_TILES[$offset] as usize
    };
}

#[macro_export]
macro_rules! get_tile_id {
    ($offset:expr) => {
        BICOLOUR_TILES[$offset] as usize
    };
}

#[macro_export]
macro_rules! is_tile_unplaced {
    ($self:expr, $id:expr) => {
        !$self.used_tiles[$id]
    };
}

#[macro_export]
macro_rules! place_tile {
    ($self:expr, $depth:expr, $id:expr, $tiles_idx:expr) => {
        $self.used_tiles[$id] = true;
        $self.placed_idx[$depth] = $tiles_idx;
        $self.placed_south_colour[$depth] = BICOLOUR_TILES[$tiles_idx + 2];
        $self.placed_east_colour[$depth] = BICOLOUR_TILES[$tiles_idx + 3];
    };
}

#[macro_export]
macro_rules! unplace_tile {
    ($self:expr, $id_usize:expr) => {
        $self.used_tiles[$id_usize] = false;
    };
}

#[macro_export]
macro_rules! record_count_at_depth {
    ($self:expr, $depth:expr) => {
        if RECORD_DEPTH_STATS {
            $self.num_solutions_at_depth[$depth] += 1;
        }
    };
}

#[macro_export]
macro_rules! record_partial_solution_at_depth {
    ($self:expr, $depth:expr) => {
        if RECORD_DEPTH_SOLUTIONS && $depth > *$self.max_depth {
            *$self.max_depth = $depth;
            store::save_board(&$self.thread_params, $self.placed_idx, $depth);
        }
    };
}

#[macro_export]
macro_rules! iterate_possible_tiles {
    ($self:expr, $tileoris_offset:expr, $num_tiles:expr) => {
        (($tileoris_offset + 2)..($tileoris_offset + 2 + ($num_tiles * 4))).step_by(4)
    };
}

#[macro_export]
macro_rules! try_next_cell {
    ($self:expr, $depth:expr) => {
        $self.add_tile_functions[$depth + 1]($self, $depth + 1);
    };
}

#[macro_export]
macro_rules! south_colour {
    ($self:expr, $cell:expr) => {
        $self.placed_south_colour[$cell.north_idx as usize] as usize
    };
}

#[macro_export]
macro_rules! east_colour {
    ($self:expr, $cell:expr) => {
        $self.placed_east_colour[$cell.west_idx as usize] as usize
    };
}

#[macro_export]
macro_rules! record_solution {
    ($self:expr) => {
        *$self.num_solutions += 1;

        store::save_board(&$self.thread_params, $self.placed_idx, NUM_TILES - 1);
    };
}

// Hard coded, not configurable, so the compiler can optimise the code away when turned off.
static RECORD_DEPTH_STATS: bool = true;
static RECORD_DEPTH_SOLUTIONS: bool = true;

#[cfg(feature = "backtracker-full")]
const MAX_THREADS: usize = 4;
#[cfg(feature = "backtracker-mids")]
const MAX_THREADS: usize = 7;

#[cfg(feature = "backtracker-full")]
const FIRST_DEPTH_TO_RECORD: usize = 204;
#[cfg(feature = "backtracker-mids")]
const FIRST_DEPTH_TO_RECORD: usize = 174;

static mut MAX_DEPTH: [usize; MAX_THREADS] = [FIRST_DEPTH_TO_RECORD; MAX_THREADS];
static mut NUM_SOLUTIONS: [u64; MAX_THREADS] = [0; MAX_THREADS];
static mut NUM_SOLUTIONS_AT_DEPTH: [[u64; NUM_TILES]; MAX_THREADS] = [[0; NUM_TILES]; MAX_THREADS];

fn print_num_solutions(thread_params: ThreadParams, elapsed_seconds: u64) {
    if elapsed_seconds == 0 {
        return;
    }

    let mut builder = Builder::default();
    let mut total: u64 = 0;

    if RECORD_DEPTH_STATS {
        unsafe {
            for (idx, num_solutions_at_depth) in NUM_SOLUTIONS_AT_DEPTH[thread_params.thread_id]
                .iter()
                .enumerate()
                .take(NUM_TILES)
            {
                if !thread_params.is_mt_mode {
                    builder.append(format!(
                        "{:>3} -> {}\n",
                        idx + 1,
                        num_solutions_at_depth.separated_string()
                    ));
                }
                total += num_solutions_at_depth;

                // Don't waste space printing the rest of the zeroes.
                if *num_solutions_at_depth == 0 {
                    break;
                }
            }
        }
    }

    let total_per_second = total / elapsed_seconds;
    unsafe {
        builder.append(format!(
            "THREAD {}  SOLUTIONS: {}  BEST: {}  TOTAL: {}    {} per second    {} seconds",
            thread_params.thread_id,
            NUM_SOLUTIONS[thread_params.thread_id],
            MAX_DEPTH[thread_params.thread_id] + 1,
            total.separated_string(),
            total_per_second.separated_string(),
            elapsed_seconds.separated_string()
        ));
    }

    println!("{}", builder.string().unwrap());
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
    placed_idx: [usize; NUM_TILES],
    placed_south_colour: [u8; NUM_TILES],
    placed_east_colour: [u8; NUM_TILES],
    add_tile_functions: [fn(&mut Self, usize) -> (); NUM_TILES],
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
                placed_idx: [0; NUM_TILES],
                placed_south_colour: [0; NUM_TILES],
                placed_east_colour: [0; NUM_TILES],
                add_tile_functions: Backtracker::ADD_TILE_FUNCTIONS,
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
        self.add_tile_functions[0](self, 0);
        let elapsed_time = start_time.elapsed().unwrap().as_secs();
        print_num_solutions(self.thread_params, elapsed_time);
    }

    fn add_tile_0_inner(&mut self, tiles_idx: usize) {
        let id = get_tile_id!(tiles_idx);

        place_tile!(self, 0, id, tiles_idx);

        record_count_at_depth!(self, 0);

        try_next_cell!(self, 0);

        unplace_tile!(self, id);
    }
}
