# RESOLUTION-SITE-SPLIT-0 results

- Track: 0.0.8.7 RF arena modernization (rung 6.2b)
- Status: **COMPLETE / DA-GRADUATED — merged #1621 @ 1a520eee** (DA independently planted synthesized-origin and dual-placement-divergence defects; each RED)
- Implementation base: `981ec1ebb5c04976798400c40f76e59bc6d3f0a6`
- ORIENT-RECEIPT: `6af1884543b0`
- orientation_rule_stamp: `5554b2613f8907ff`
- HD-RECEIPT: `d1e369c888e7`
- Dispatch: `5179129484`
- expected_route: `DA-RESERVE(gate-wiring)`
- Scope: **6.2b only**; no 6.3/downstream work

## Prerequisite telemetry — measure before build

The rung's prerequisite: run the EXISTING per-stage telemetry over a representative soak and
record `readback / tick_total` as the removable upper bound plus the allocation stages as the
barrier-only floor, BEFORE any resolution-site code. No performance threshold is invented; the
numbers below are evidence, not a gate.

Harness: the existing driver bench (`simthing bench`), which reports every `RunSummary`
per-stage field (`tick_event_readback_ms`, `boundary_value_readback_ms`,
`boundary_alert_collect_ms`, `boundary_pregrow_fission_ms`, `boundary_structural_ms`,
`tick_total_ms` family). Release build at the implementation base, run owner-local
(Windows 11, wgpu 22 / DX12).

### Soak configurations

Authored stress scales could not run (see finding below); the soaks use the stress builtins at
the largest scale that completes, plus the authored demo scenario unchanged:

| soak | builtin | n_slots | days | boundaries run |
|---|---|---|---|---|
| A | `threshold_stress` | 500 | 30 | 1 (29 skipped empty) |
| B | `fission_stress` | 500 | 30 | 1 (29 skipped empty) |
| C | `rebellion_demo` (authored, unchanged) | 16 | 8 | 1 (7 skipped empty) |

### Measured per-stage evidence

All values are accumulated ms over the full soak (the bench's native reporting).

| field | A: threshold_stress | B: fission_stress | C: rebellion_demo |
|---|---|---|---|
| `tick_measured_ms` | 937.970 | 819.720 | 6.715 |
| `tick_gpu_pipeline_submit_ms` | 931.288 | 814.930 | — (small) |
| `tick_event_readback_ms` | 6.639 | 4.748 | 1.457 |
| `boundary_ms` | 32.912 | 29.089 | 0.736 |
| `boundary_value_readback_ms` | 0.508 | 0.187 | 0.282 |
| `boundary_alert_collect_ms` | 0.012 | 0.005 | 0.003 |
| `boundary_pregrow_fission_ms` | 3.438 | 1.681 | 0.000 |
| `boundary_fission_ms` | 0.635 | 0.357 | 0.010 |
| `boundary_lineage_ms` | 0.026 | 0.025 | 0.001 |
| `boundary_request_drain_ms` | 0.002 | 0.002 | — |
| `boundary_pregrow_add_child_ms` | 0.000 | 0.000 | 0.000 |
| `boundary_structural_ms` | 0.016 | 0.010 | 0.004 |
| `boundary_dimension_rebuild_ms` | 0.000 | 0.000 | 0.000 |
| `boundary_final_capacity_ms` | 0.000 | 0.000 | 0.000 |
| `boundary_gpu_sync_ms` | 11.651 | 11.724 | 0.019 |

### The two recorded bounds

**Removable upper bound** (readback + identity-re-attachment stages over total measured work,
`(tick_event_readback + boundary_value_readback + boundary_alert_collect) / (tick_measured + boundary)`):

- A: 7.159 / 970.882 = **0.74%**
- B: 4.940 / 848.809 = **0.58%**
- C: 1.742 / 7.451 = **23.4%**

**Allocation-stage floor** (pre-grow, fission/fusion, lineage, request drain, AddChild pre-grow,
structural, dimension rebuild, final capacity — the stages that stay at the barrier in BOTH
modes):

- A: 4.117 / 970.882 = **0.42%**
- B: 2.075 / 848.809 = **0.24%**
- C: 0.015 / 7.451 = **0.20%**

### What the measurement says

At stress scale, neither readback nor allocation dominates: the fused GPU pipeline submit is
~96% of the tick. The removable upper bound from closing the loop is under 1% at that scale;
at small authored scale (16 slots) fixed readback latency is ~23% of a much smaller total. The
allocation floor is 0.2–0.4% everywhere — small, and permanently barrier-resident by design.

**The architectural case therefore carries this rung alone** — the exact outcome the ladder row
named as legitimate provided it is KNOWN rather than assumed. It is now known. No performance
threshold is invented and the implementation does not depend on a speedup claim.

### Finding (pre-existing on the implementation base, reported, not fixed here)

The authored stress scales are currently unrunnable on the owner machine: `threshold_stress`
(100,000 slots) and `fission_stress` (20,000 slots) both die with wgpu `Parent device is lost`
at `Queue::submit` inside `read_threshold_emissions`. Bisection: 500 slots completes
(~457 ms/tick at 2-day config), 1,000 slots is device-lost — consistent with the fused
threshold/reduction pass exceeding the Windows TDR (~2 s) as per-tick cost grows super-linearly
in slot count. `intent_stress` at 100,000 slots (zero threshold registrations) runs at
~18 ms/tick, and `cpu_gpu_parity_matrix_0` passes, so the GPU and the fused pipeline are
healthy at small registration counts; the blowup tracks threshold/reduction registration scale.
This pre-exists rung 6.2b (reproduced at the untouched implementation base), is invisible to CI
(no cargo test runs by standing Owner ruling; benches are manual), and belongs to the 6.3 soak
lane, not this rung. Recorded here so it is triaged rather than rediscovered.

## Landed surface (one model, two resolution sites)

- `simthing_sim::resolution_site` holds BOTH placements' identity doors side by side.
  `ResolutionSite::ClosedLoop` (the `Default`) re-attaches identity for converted crossings at the
  barrier through the admitted slot map (`SlotAllocator::owner_of` + registered column owners —
  the live authority), TOTAL over converted crossings and FAIL-CLOSED
  (`SlotIdentityReattachError`; never a default identity). `ResolutionSite::CpuAuthoritative` is
  the vendorized pre-split arm, relocated verbatim from `boundary.rs`
  (`collect_*_alerts_vendorized`): identity from the semantic table's registration-time entries —
  the mirror, demoted from authority to mirror by the parity referees.
- Converted semantics (incremental, no flag day): `VelocityAlert` and `AggregateAlert` — the two
  pure identity-re-attachment observation arms the ladder row names as the arms that EVAPORATE.
  Crossing selection is one vocabulary (the one `ThresholdRegistry` semantic table) at both
  placements; only the identity source differs. Unconverted arms (`FissionTrigger`,
  `FusionTrigger`, `PropertyExpiry`, `CapabilityUnlock`, `ScriptedEventTrigger`) run identical
  code in both placements; no fission/expiry/structural API gained a placement parameter, so
  allocation is placement-blind at the type level.
- `BoundaryProtocol` carries `resolution_site` (default `ClosedLoop` — the closed loop IS the
  default placement now) with `set_resolution_site` keeping the vendorized build selectable.
  The stage-1 alert collect dispatches on placement inside the existing stage; stage order,
  count, timing fields, and all seven allocation stages are untouched.
- Closed-loop overlay origination: `SlotSpaceOverlayDraft` carries `origin_slot`/`target_slot`
  in SLOT SPACE (the 6.0b required-`origin` type boundary holds at the CPU representation — a
  draft becomes an `Overlay` only through `mint_attach_overlay_at_barrier`). The barrier door
  re-attaches both ids through the admitted slot map, fails closed on an unadmitted slot, and
  mints `BoundaryRequest::AttachOverlay` with `affects` empty — routed delivery
  (`deliver_routed_overlay`, 6.0b) sets it, so direct-`affects` bypass is structurally
  impossible from this door.
- Planted-mutant referees are TEST-LOCAL ONLY (Remand 1, orchestrator `5179675096`): the
  mirror-drift registries are built inside the test through the ordinary `push` door with
  deliberately wrong registration-time identity; the transform-divergence draft is mutated in
  the test and run through the REAL mint door; the synthesized-origin comparator is constructed
  in the test as the request a defaulting door WOULD have minted, while the real door returns
  `Err` on the same input. No mutant constructor ships in the production module or crate exports.

## Biting proofs

| Proof | Result |
|---|---|
| Velocity-alert parity | identical oracle-minted crossings produce equal + `{:?}`-bit-identical + per-field `to_bits`-identical product streams at both placements; planted mirror-drift mutant REDs parity |
| Aggregate-alert parity | same referee shape over `THRESH_BUF_OUTPUT` crossings; planted mirror-drift mutant REDs parity |
| Slot-space origination | closed-loop draft->barrier mint and vendorized direct construction yield BIT-IDENTICAL `AttachOverlay` `BoundaryRequest` streams (incl. a planted `-0.0` payload); planted transform-divergence mutant REDs stream parity |
| Fail-closed slot->id | unadmitted origin slot, unadmitted target slot, and unadmitted crossing slot each return the named `SlotIdentityReattachError`; the test-local default-origin comparator constructs exactly the forbidden attributable overlay the real door rejects |
| Reception at both sites | both placements' requests arrive through the SAME `deliver_routed_overlay` with equal `DirectiveDeliveryReceipt`s and `{:?}`-identical trees |
| Incremental / no flag day | unconverted semantics produce identically empty converted-door output at both placements; `ResolutionSite::default()` and `BoundaryProtocol` default are `ClosedLoop`; vendorized stays selectable |
| Slot-space wire vocabulary | `size_of::<ThresholdEventGpu>() == 16` — the GPU wire event is exactly `{slot, col, value, event_kind}`; no identity lane exists |

## No in-shader SimThingId path (grep + type evidence)

The RESOLUTION vocabulary is identity-free end to end: `accumulator_op.wgsl` (the one fused
Pass B kernel — crossings, EML, transfers, sweeps, threshold emission) contains no
`sim_thing_id`/identity symbol (its only `IDENTITY` hits are the algebraic fold-identity
constants `COMBINE_IDENTITY`/`SCALE_IDENTITY`); the wire types `ThresholdEventGpu` /
`ThresholdEmissionGpu` carry `{slot, col, value/reg_idx, event_kind}` only. Recorded honestly:
the STEAD anchor-table maintenance shaders (`anchor_table_maintain/remap/magnitude_values.wgsl`)
carry an admission-minted opaque `sim_thing_id: u32` ROW LABEL used solely for remap row
matching — that is P0(e) "anchor identity minted at admission, stable across slot moves",
pre-existing, structural (not resolution), and untouched by this rung. No resolution or
decision path in shader space requires a `SimThingId`; identity attaches only at the CPU
barrier doors.

## Pipeline unmodified / allocation barrier-only

The diff over `boundary.rs` touches exactly: the `resolution_site` field + accessors, the
stage-1 placement dispatch, and the relocation of the two collector fns to
`resolution_site.rs`. The 13-stage sequence (module header list), the seven allocation stages,
`projected_fission_slots`, `resolve_fission_fusion`, `apply_structural_mutations`, and every
timing field are byte-untouched — verifiable by `git diff 981ec1eb..HEAD -- crates/simthing-sim/src/boundary.rs`.
`FissionTrigger` pre-grow sizing and all structural work remain barrier-only in BOTH placements
(no placement parameter exists on any allocation surface).

## Live evidence (closed-loop default running end to end)

At HEAD (ClosedLoop default), the 30-day threshold_stress soak (500 slots, incl. the mass-fission
boundary day) and the authored rebellion_demo complete with structurally IDENTICAL bench output
to the pre-change base runs (fission_events 499, n_slots 500->1000, boundary cadence, all byte
and upload counters — diff-verified field-for-field).

```text
cargo test -p simthing-sim --test resolution_site_split_0: 7/0
cargo test -p simthing-sim (full crate battery): 9 suites, 0 failures
cargo test -p simthing-driver --test determinism_matrix_0 --test cpu_gpu_parity_matrix_0
    --test simthing_automaton_rf_reception_0 --test write_door_band_delta_0: 16/0 (GPU legs live)
bash scripts/ci/agent_scan.sh: AGENT-SCAN-VERDICT: PASS delta_inspect=0
```

## Fence / STOP status

- No STOP condition was hit: slot-space identity sufficed for every converted semantic; slot->id
  re-attachment is total/fail-closed with no synthesized identity and no second identity
  authority; bit-identical parity required relocating identity re-attachment only, never a
  semantic change.
- Fences held: one semantic vocabulary (the existing `ThresholdSemantic` table at both sites);
  no in-shader `SimThingId` (see above); no synthesized/default `Overlay.origin`; no slot->id
  fallback; no shader-side allocation and zero WGSL diff; 13-stage pipeline unmodified; no
  direct-`affects` bypass; no second transport/queue/recorder; no scenario/domain witness
  (synthetic inline input only); no gate/invariant weakening; no 6.3/movement/contention work.

## Posture

**COMPLETE / DA-GRADUATED — merged #1621 @ 1a520eee.** No clearance, merge, pointer movement, or
successor-rung work is claimed. The pre-existing device-lost finding at authored stress scales
(see telemetry section) is reported for triage, not fixed here.
