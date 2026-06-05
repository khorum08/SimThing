# Architecture Decision Records

| ADR | Status | Topic |
|-----|--------|-------|
| [pr11_track_a_session_assembly.md](pr11_track_a_session_assembly.md) | Accepted | Driver-owned spec session state, generic boundary hook |
| [game_mode_session_installation.md](game_mode_session_installation.md) | Accepted (O1 landed PR #53) | RON-driven `open_from_spec` installation |
| [capability_effect_target_scope.md](capability_effect_target_scope.md) | Accepted (landed `7febdd1`) | EffectTarget — Owner default, overlay placement + `overlay_hosts` |
| [scripted_event_scope_model.md](scripted_event_scope_model.md) | Accepted (O4 landed `8904522`) | Per-owner scripted event instances |
| [spec_session_state_replay.md](spec_session_state_replay.md) | Accepted (O2 landed `2f2a7b5`) | Replay v3 — `SpecSnapshot`/`SpecDelta`, logical keys |
| [install_clone_then_commit.md](install_clone_then_commit.md) | Accepted (I1 landed `6b8de81`) | `preview_install` / `install_atomic` / Studio preview |
| [mapping_sparse_regioncell.md](mapping_sparse_regioncell.md) | Approved (architecture 2026-05-28; Phase M unlocked, no runtime) | Sparse RegionCell mapping — three-layer model, optimization doctrine, FIELD_POLICY surfacing |

AccumulatorOp v2: [`../adr_accumulator_op_v2.md`](../adr_accumulator_op_v2.md) (Proposed) · spec: [`../design_v7.md`](../design_v7.md)

**Parking and gates:** [`docs/design_v6.5.md`](../design_v6.5.md) · **GPU pipeline:** [`docs/design_v7.md`](../design_v7.md)
