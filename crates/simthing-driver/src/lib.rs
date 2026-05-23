pub mod bench_limits;
pub mod install;
pub mod scenario;
pub mod session;
pub mod spec_replay;
pub mod spec_session;

pub use bench_limits::{check as check_bench_ceiling, ms_per_sim_day, CEILINGS};
pub use install::{compile_and_install, InstallError};
pub use scenario::{Scenario, ScenarioError, ShadowSeed};
pub use session::{RunSummary, SessionError, SimSession};
pub use spec_replay::{
    apply_spec_delta, apply_spec_snapshot, collect_spec_snapshot, diff_and_emit,
    json_to_spec_deltas, open_replay_with_spec, read_spec_replay_file, spec_deltas_to_json,
    CapabilityStateSnapshot, LoadedReplay, QueuedSelectionSnapshot, ReplayOpenError,
    ScriptedCooldownSnapshot, SpecDelta, SpecSnapshot,
};
pub use spec_session::{
    CapabilityInstanceKey, PreBoundarySnapshot, SpecSessionError, SpecSessionState,
};
