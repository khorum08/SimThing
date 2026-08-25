//! Planted defect: volume-delay as a piecewise ladder, not exp(k * ln x).
use simthing_embedder::{bind, derive, overlay, populate, run};

fn volume_delay_staircase(ratio: f32) -> f32 {
    if ratio < 0.5 {
        1.0
    } else if ratio < 1.0 {
        1.15
    } else if ratio < 1.5 {
        1.5
    } else {
        2.5
    }
}

fn main() {
    let _ = volume_delay_staircase(2.0);
    let _ = (bind::shadow, derive::reserved_unowned, overlay::authored, populate::owner, run::tick);
}
