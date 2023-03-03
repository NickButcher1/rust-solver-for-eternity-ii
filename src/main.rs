use crate::backtracker::Backtracker;
use crate::threadparams::ThreadParams;
use rand::Rng;
use std::thread;

#[cfg(feature = "backtracker-full")]
mod autogenfull;
#[cfg(feature = "backtracker-mids")]
mod autogenmids;
mod backtracker;
mod celltype;
mod colour;
mod ori;
mod store;
mod threadparams;

fn main() {
    let (num_threads, stats_every_s) = if cfg!(feature = "mt") {
        if cfg!(feature = "backtracker-mids") {
            (num_cpus::get() - 1, 300_000)
        } else {
            // Limit to four because there are only four first tiles (the four corner tiles).
            (4, 300_000)
        }
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

            Backtracker::new(thread_params).solve();
        }));
    }

    for thread in threads {
        // Wait for the thread to finish. Returns a result.
        let _ = thread.join();
    }
}
