# V7.6 StructuredFieldStencilOp Promotion — Test Results

| Field | Value |
|---|---|
| Date/time | 2026-05-19 (local verification run) |
| Base HEAD | `57d915bf7285a156a9d799372ad504a84039b5af` |
| Final commit SHA | `76803c3` |
| rustc | 1.95.0 (59807616e 2026-04-14) |
| cargo | 1.95.0 (f2d3ce0bd 2026-03-21) |
| Platform | Windows 10 (win32 10.0.26200) |
| GPU | Available (WGSL stencil + parent EML GPU tests executed) |
| Full log | [`v7_6_structured_field_stencil_promotion_full.log`](v7_6_structured_field_stencil_promotion_full.log) |

---

## Commands run

| Command | Result |
|---|---|
| `cargo test -p simthing-gpu --test structured_field_stencil -- --nocapture` | **PASS** (5/5) |
| `cargo test -p simthing-driver --test structured_field_stencil_parent_eml -- --nocapture` | **PASS** (2/2) |
| `cargo test -p simthing-spec --test eml_field_formula_admission -- --nocapture` | **PASS** (2/2) |
| `cargo test -p simthing-spec --test resource_flow_nested_participant_roundtrip -- --nocapture` | **PASS** (2/2) |
| `cargo test -p simthing-driver --test e11b_nested_materialization_ron_session -- --nocapture` | **PASS** (3/3) |
| `cargo test -p simthing-driver --test e11b_nested_materialization -- --nocapture` | **PASS** (10/10) |
| `cargo test -p simthing-driver --test e11b_nested_hierarchy_gpu -- --nocapture` | **PASS** (12/12) |
| `cargo test -p simthing-driver --test e11b_nested_fission_gap -- --nocapture` | **PASS** (13/13) |
| `cargo check --workspace` | **PASS** |
| `cargo test --workspace` (`CARGO_BUILD_JOBS=1`) | **PASS** (no failures in full log) |

---

## Test coverage notes

### Test A — WGSL compile and 3×3 correctness
GPU normalized NSEW stencil on 3×3 center source matches CPU oracle (`max_error` within tolerance).

### Test B — Ping-pong correctness
3×3 and 10×10 grids at H ∈ {1, 2, 4, 8}; GPU matches CPU oracle; ping-pong path used for H>1.

### Test C — 10×10 H8 tactical horizon
Top-left source cluster; `[4][4]` receives nonzero bounded value; gradient direction consistent.

### Test D — Source cap / horizon cap
`source_capped_normalized` bounds extended horizon; default cap rejects H=16 without `allow_extended_horizon`; H=16 allowed with flag + cap.

### Test E — Column-aware parent EML
Local stencil field → column-aware SlotRange reduction → parent personality columns → `field_urgency` EvalEML on later band; `urgency_B > urgency_A > 0`.

### Test F — EML admission
`field_pressure`, `field_urgency`, `field_decay`, `bounded_field_update`, and `conversion_rate` accepted via legacy whitelist and C-8 `register_formula`.

### Test G — Production defaults unaffected
`PipelineFlags::default().use_accumulator_resource_flow == false`; Resource Flow opt-in posture unchanged; `passes.rs` does not reference `StructuredFieldStencilOp`.

### Test H — E-11B regressions
All five E-11B regression targets green after V7.6 WGSL allowlist update (`structured_field_stencil.wgsl` admitted as generic semantic-free shader).

---

## V7.6 Promotion Decision Summary

| Area | PASS / PARTIAL / FAIL | Evidence |
|---|---|---|
| V7.6 doc bump | PASS | `docs/design_v7_6.md`, active doc pointers updated |
| WGSL guardrail relaxation | PASS | Generic `structured_field_stencil.wgsl` live; E-11B guards allow V7.6-approved list |
| EML admission relocation | PASS | Four field classes added to `WHITELISTED_FORMULA_CLASSES`; admission tests green |
| StructuredFieldStencilOp live primitive | PASS | `simthing-gpu` module + shader; not wired into production pass graph |
| Ping-pong correctness | PASS | Test B + unit validation |
| Source/horizon stability constraints | PASS | Test D + config validation (`DEFAULT_HORIZON_CAP=8`) |
| Column-aware parent EML | PASS | Test E |
| Production defaults unaffected | PASS | Test G + guard test |
| E-11B regressions | PASS | Test H suite (40 tests across five targets) |

## Final verdict

**PASS**

## Notes

- First PR intentionally promoting a preserved WGSL workshop prototype into live **library** code as a semantic-free generic primitive.
- `StructuredFieldStencilOp` is opt-in via direct API use; default production pipeline behavior unchanged.
- Active mask mode documented but not production-authorized pending halo/frontier semantics.
- Directed stencil operator implemented optionally; normalized / source_capped_normalized are default production modes.
- Mapping/location runtime explicitly out of scope for this promotion.
