# RESOLUTION-SITE-SPLIT-0 results

- Track: 0.0.8.7 RF arena modernization (rung 6.2b)
- Status: **IN PROGRESS — prerequisite telemetry landed BEFORE resolution-site code (this commit contains no implementation)**
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
