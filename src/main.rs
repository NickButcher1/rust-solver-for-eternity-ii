use crate::autogen::PLACE_MIDS_ONLY;
use crate::backtracker::Backtracker;
use crate::backtrackermidsonly::BacktrackerMidsOnly;
use crate::threadparams::ThreadParams;
use rand::Rng;
use std::{env, thread};

mod autogen;
mod backtracker;
mod backtrackermidsonly;
mod celltype;
mod colour;
mod ori;
mod store;
mod threadparams;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (num_threads, stats_every_s) = if args.contains(&"mt".to_string()) {
        (num_cpus::get() - 1, 60_000)
    } else {
        (1, 10_000)
    };

    let run_id = rand::thread_rng().gen::<u64>();

    println!("Threads: {}", num_threads);

    let mut threads = vec![];

    for thread_id in 0..num_threads {
        // Spin up another thread
        threads.push(thread::spawn(move || {
            let thread_params = ThreadParams {
                run_id,
                thread_id,
                stats_every_s,
                is_mt_mode: num_threads > 1,
                tile_0_idx: thread_id,
            };

            if PLACE_MIDS_ONLY {
                BacktrackerMidsOnly::new(thread_params).solve();
            } else {
                Backtracker::new(thread_params).solve();
            }
        }));
    }

    for thread in threads {
        // Wait for the thread to finish. Returns a result.
        let _ = thread.join();
    }
}
