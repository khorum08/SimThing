// CI trap: known-legitimate mapeditor presentation projection/cache of polylines.
// Must NOT trip BORDER-SERVICE (presentation projection only; no semantic service API).
pub fn project_lane_polyline_screen_cache(points: &[[f32; 2]]) -> Vec<[f32; 2]> {
    // Screen-space projection cache only.
    points.iter().map(|p| [p[0] * 2.0, p[1] * 2.0]).collect()
}

pub fn hyperlane_display_polyline_buffer(src: &[f32], dst: &[f32]) -> Vec<[f32; 2]> {
    vec![[src[0], src[1]], [dst[0], dst[1]]]
}
