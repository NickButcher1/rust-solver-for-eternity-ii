use crate::autogen::PLACE_MIDS_ONLY;
use crate::backtracker::Backtracker;
use crate::backtrackermidsonly::BacktrackerMidsOnly;

mod autogen;
mod backtracker;
mod backtrackermidsonly;
mod celltype;
mod colour;
mod ori;
mod store;

fn main() {
    if PLACE_MIDS_ONLY {
        BacktrackerMidsOnly::new().solve();
    } else {
        Backtracker::new().solve();
    }
}
