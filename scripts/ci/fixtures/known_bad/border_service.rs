// CI fixture: BORDER-SERVICE — contour/frontline border-service reach.
// Must fire HEURISTIC INSPECT under CONSTITUTION-TRIPWIRES-0.
pub struct FrontlineTracer;

pub fn contour_extraction_border_service() -> FrontlineTracer {
    let _ = marching_squares_stub();
    FrontlineTracer
}

fn marching_squares_stub() -> u32 {
    0
}
