#[derive(Clone, Copy)]
pub struct ThreadParams {
    pub run_id: u64,
    pub thread_id: usize,
    pub stats_every_s: u64,
    pub is_mt_mode: bool,
    // Fix the first tile placement for this thread. Ensures each thread does something different.
    pub tile_0_idx: usize,
}
