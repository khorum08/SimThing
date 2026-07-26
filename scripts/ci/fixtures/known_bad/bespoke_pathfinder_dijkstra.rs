// CI fixture: BESPOKE-PATHFINDER — ordinary Dijkstra (dist/prev; no BinaryHeap / A* names).
// Must fire HEURISTIC INSPECT under CONSTITUTION-TRIPWIRES-0.
pub fn dijkstra_shortest_path(n: usize) -> (Vec<u32>, Vec<Option<usize>>) {
    let mut dist = vec![u32::MAX; n];
    let mut prev: Vec<Option<usize>> = vec![None; n];
    if n == 0 {
        return (dist, prev);
    }
    dist[0] = 0;
    for _ in 0..n {
        for u in 0..n {
            let du = dist[u];
            if du == u32::MAX {
                continue;
            }
            // relax_edge-shaped update into neighbor slots
            for v in 0..n {
                let cand = du.saturating_add(1);
                if cand < dist[v] {
                    dist[v] = cand;
                    prev[v] = Some(u);
                }
            }
        }
    }
    (dist, prev)
}
