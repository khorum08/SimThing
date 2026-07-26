// CI trap: legitimate non-path BinaryHeap (event/priority scheduling).
// Must NOT trip BESPOKE-PATHFINDER.
use std::collections::BinaryHeap;

pub fn schedule_timed_events() {
    let mut heap: BinaryHeap<(i64, u64)> = BinaryHeap::new();
    heap.push((-10, 1));
    heap.push((-5, 2));
    let _ = heap.pop();
}
