// CI fixture: RAW-DATA-INDEX — production-shaped raw lane index (HEURISTIC).
pub struct LaneProbe {
    pub data: [f32; 4],
}

pub fn read_lane(probe: &LaneProbe) -> f32 {
    probe.data[0]
}
