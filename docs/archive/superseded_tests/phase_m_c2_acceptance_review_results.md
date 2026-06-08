# C-2-ACCEPT-0 — Design-Authority Ruling: Accept Atlas Admission Relaxation (with inline remediation)

**Reviewer:** Opus 4.8 (design authority — v7.8 track) + project-owner product direction
("close out map batching"). **Date:** 2026-05-30.
**Decision:** **ACCEPT C-2 (Option A), with two inline compile-break remediations applied during
review.** Bounded algebraic-G=0 atlas specs are now admittable at the designer/spec layer.
**Map batching is closed at the designer-surface level**; production atlas runtime / sparse-residency
scheduler remains a separate later gate (not opened). Physical gutter, active mask/source identity,
production runtime/default wiring/default-on atlas, A-0/B-0, L3, FrontierV2-5, ACT/EVENT/OBS/PIPE
remain rejected or deferred.

## Reviewed — code, not only the report

- `crates/simthing-spec/src/designer_admission/atlas.rs` — `AtlasAdmissionSpec::evaluate`.
- `crates/simthing-spec/src/designer_admission/diagnostic.rs` — the 7 new C-2 codes + the 3 match arms.
- `crates/simthing-spec/tests/c2_atlas_admission_relaxation.rs` — 14 tests.
- `crates/simthing-sim/src/**` — map-awareness scan **empty**.

## Compile breaks found and remediated (the report's "all tests pass" was false against the tree)

The landed C-2 **did not compile**. Two mechanical breaks, both fixed inline (no design or posture
change — completing the implementer's own work):

1. **Non-exhaustive match (3× E0004).** The 7 new `DesignerAdmissionDiagnosticCode` variants were
   added to the enum but not covered in `as_str`, `guardrail_class`, or `rejection_kind`. Wired:
   `as_str` → `C-2-ATLAS-SPEC-*` strings; `guardrail_class` → `MappingExpansion` (atlas-spec codes)
   and `RuntimeWiring` (`AtlasProductionRuntimeRejected`); `rejection_kind` → `AtlasWithoutGate`
   (the coarse category; the specific code carries the detail).
2. **Test imported a private submodule path (3× E0603).** `tests/c2_atlas_admission_relaxation.rs`
   used `simthing_spec::designer_admission::atlas::…` (private `mod atlas`). Fixed to the public
   re-export `simthing_spec::designer_admission::{…}`.

## Review answers (after remediation)

| # | Question | Finding |
|---|---|---|
| 1 | `AtlasAdmissionSpec` implements the C-2 surface? | **Yes** — request flag, profile, tile dims, homogeneity, isolation, oracle-backed, active budget, multiplier flag + explicit negatives. |
| 2 | Admits only algebraic G=0? | **Yes** — enum has a single variant; `isolation != AlgebraicTileLocalMaskG0` → reject. |
| 3 | Homogeneous-square enforced? | **Yes** — `tile_width != tile_height \|\| !homogeneous_square_tiles` → reject. |
| 4 | Protocol-oracle-backed required? | **Yes.** |
| 5 | Active `V78AtlasVramBudget.max_bytes` enforced? | **Yes** — `algebraic_bytes ≤ active_vram_budget.max_bytes`; over-budget → reject. |
| 6 | Multiplier reporting required? | **Yes** — on both spec and budget. |
| 7 | TypicalHugeCommodity modeled? | **Yes** — ≈1.37M cells → ≈0.163 GiB, fits 1.5 GiB. |
| 8 | HorizonDedicatedServerStress modeled? | **Yes** — 7,230,000 cells → ≈0.862 GiB, fits; gutter ≈5.826 GiB does not. |
| 9 | Physical gutter rejected as a C-2 path? | **Yes** — `AtlasSpecPhysicalGutterRequiresRaisedGateRejected`. |
| 10 | Active mask / source identity rejected? | **Yes** — `ActiveMaskRequestedWithoutGate` / `SourceIdentityRequestedWithoutGate`. |
| 11 | Production runtime / default-on rejected? | **Yes** — `AtlasProductionRuntimeRejected`. |
| 12 | No production runtime/scheduler/sparse-residency/semantic-WGSL/sim-awareness? | **Yes** — pure spec-layer admission metadata; no GPU/WGSL/sim. |
| 13 | Diagnostic codes specific? | **Yes** — 7 distinct C-2 codes, now fully wired through `as_str`/`guardrail_class`/`rejection_kind`. |
| 14 | Accept as the designer-facing closure of map batching? | **Yes.** |
| 15 | Next state? | **Map batching closed at the designer surface.** Production atlas runtime / sparse-residency scheduler is a separate later gate (not opened). A-0/B-0 stay **queued**; L3 parked. |

## Non-blocking observation (for the future runtime gate, not a remediation)

`estimate_total_dense_cells` is **profile-modeled** (a fixed estimate per `AtlasAdmissionProfile`),
not derived from the spec's actual tile dimensions/scenario. The code comments acknowledge this
("Real scenarios will provide exact numbers via the claim"). Acceptable for C-2 (bounded designer
admission, metadata-only): the two known envelopes are bounded and over-budget profiles are rejected.
**Per-spec-exact budget accounting** (from real scenario detail) is a refinement for the future
atlas runtime/scenario gate, not a C-2 blocker.

## Verification

| Command | Result |
|---|---|
| `cargo test -p simthing-spec --test c2_atlas_admission_relaxation` | **14/14 PASS** (after remediation) |
| `cargo test -p simthing-driver --test phase_m_c0_m4_atlas_protocol_oracle` | **13/13 PASS** |
| `cargo test -p simthing-driver --test phase_m_c1_atlas_scale_model` | **10/10 PASS** |
| `cargo test -p simthing-spec --test clause_spec0_frontier_v2_admission` | **25/25 PASS** |
| `cargo test -p simthing-spec --test v7_8_met_consumer_scenarios` | **10/10 PASS** |
| `cargo check --workspace` | **PASS** (pre-existing `simthing-driver` unused-import + `EmlTreeMeta` deprecation warnings only) |

## Guardrail confirmations (no authorization)

Production mapping runtime, default `SimSession` wiring, default-on atlas, physical gutter as a C-2
path, active-mask halo (M-6A), source identity / `source_mask` (M-5), sparse-residency scheduler/
cadence runtime, A-0/E-11B/E-11B-5, B-0/D-2/D-2a, ClauseThing/ClauseScript/L3, FrontierV2-5,
ACT-5/EVENT-3/OBS-5/PIPE-1, semantic/map-specific WGSL, `simthing-sim` map awareness — **all remain
unauthorized.** No invariant change; C-2 operates within the existing atlas invariant (C-0 was the
gate-passing M-4 PR).

## Ruling

**ACCEPT C-2.** Designer/spec admission now admits bounded **algebraic-G=0**, homogeneous-square,
protocol-oracle-backed atlas specs that fit the active `V78AtlasVramBudget` and carry mandatory
multiplier reporting; everything else stays rejected. Map batching is **closed at the designer
surface** (C-0 proof + C-1 model + C-2 admission). The atlas **production runtime / sparse-residency
scheduler** is a separate later gate, **not** opened. A-0/B-0 remain queued; L3 parked. v7.8
constitution / production-track split intact.
