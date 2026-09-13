# 0088 UI prototype — Leaf B0a non-latency capture

Status: PROBATION / proof-present / OPEN / UNMERGED to ORCHESTRATION ONLY.
Dispatch: Board 5651318597; base `1df365001131f137d18abb55da3c7fe593c6be71`.
This is instrument support. Full M16 comparison, latency instrument admission and migration
decision remain held. No release performance result or service-line compliance is claimed.

## Scopes and validity

`rehearsal_studio_m16_capture.rs` mounts a default-off Last-schedule collector over existing
Mapeditor state. No dependency, session, observer, command owner, simulation clock or render
resource is added. Existing smoothed operator telemetry is unchanged.

| Raw record | Exact scope | Qualification |
| --- | --- | --- |
| Frame | `Time<Real>::last_update()` minus its unsmoothed `delta()` to `last_update()` | Bevy main/render-loop cadence; not scanout or compositor frame delivery. Same source as Bevy's raw frame diagnostic, without its history/smoothing. Manual time strategies are explicitly marked synthetic. |
| CPU shared projection | `StudioSimClockTransport::readout` plus `build_studio_live_observation_readout`, enclosing monotonic start/end | Identical computation and boundaries in both actual clients, including the same snapshot cloning. Host wall elapsed time of CPU work; includes preemption, not OS thread CPU accounting. Excludes widget construction, layout, tessellation and render. |
| CPU egui adapter | `draw_live_observation`, including the shared projection | Egui label construction. The enclosing clock-transport interval also includes controls/input and this adapter. |
| CPU native adapter | Visible `sync_native_pane` body, from after visibility check through controls, observation Text and button colors | Includes shared projection, list/control labels and property text. Separate `native_input_router` interval includes existing event ownership/dispatch. Excludes later Bevy UI layout/render systems. |
| Publication | First assignment to app `live_bridge_readout` of a new scene/resident/generation tuple | App publication, after `bridge.readout()` and its existing readbacks; not GPU production time. Reassigning a paused generation retains its original publication timestamp. |
| Consumption | End of actual egui observation adapter / successful native Text acceptance | First acceptance **per client during the capture**, not visible-pixel acknowledgement. Includes an inline copy of its matching publication stamp, even when publication preceded capture. Subsequent paused redraws are deduplicated. |

All timestamps are nanoseconds relative to the collector's process-local monotonic origin.
Subtract a consumption's `stamp.published_ns` from `consumed_ns` for that sample's freshness.
No input-event latency field or surrogate endpoint exists. Bevy present scheduling, egui UI
construction, native Text assignment and source-to-ready are not substituted for M16 latency.

Only the shared projection CPU scope is directly like-for-like. The adapter scopes above
have different payloads and lifecycle boundaries; they are diagnostic components, not a
whole-client comparison. Nested projection/adapter/transport intervals overlap: **do not sum
them**. Whole-client CPU, layout/render/GPU cost, memory and the full frozen comparison are
not qualified by this support leaf. No workload or presentation payload was changed to make
the two adapters equivalent.

Every successful existing resident admission and bridge reset advances a measurement epoch;
failed admission never reaches that hook. Consumers must match scene, epoch and generation
and must have an attached bridge with no pending reset. Missing/stale matches produce raw
Rejected records, not a zero duration. Same-source/same-G0 replacement cannot reuse a stamp.
Instrumentation never opens, advances or subscribes a resident.

## Capture procedure and metadata

1. Build the admitted code; record exact code/instrument SHA, compiler and build command.
   Launch the existing Studio with `SIMTHING_M16_CONFIG` pointing to an absolute JSON path.
2. Use the unchanged pinned Meridian workload, existing source/profile/dependency identities,
   subscriptions, operation sequence and presentation settings. Warm up separately.
3. Press F9 after warmup. The collector reads the configuration, validates required metadata
   and starts a new bounded raw capture. A retained capture prevents another F9 start.
4. Press F10 to stop **before** serialization/filesystem I/O and export. Reaching capacity
   stops automatically, retaining the entire prefix and its explicit stop reason; F10 still
   exports it. There is no circular overwrite, slow-sample filter, averaging or tail clipping.
5. Use a distinct output and repetition identifier for each subsequent capture. After a
   successful export, F9 begins with empty samples and fresh frame/consumption deduplication.

Configuration shape (operator qualification is self-reported, not independently verified
by accepting this file; replace every example value with the actual condition):

```json
{
  "output": "C:/measurement-output/m16-egui-paused-r1.json",
  "max_samples": 100000,
  "metadata": {
    "condition": "egui paused; exact pane and interaction condition",
    "repetition": "1",
    "code_revision": "exact tested build SHA",
    "instrument_revision": "exact instrument code SHA; m16-b0a-v1",
    "seed": "actual authored seed",
    "hardware": "named reference machine; CPU/RAM/GPU",
    "backend": "actual renderer and resident backends",
    "driver": "actual driver version",
    "compiler": "rustc -Vv output",
    "build_flags": "exact release build flags and environment",
    "cache_state": "actual cold/warm cache qualification",
    "warmup": "exact excluded warmup procedure and duration",
    "exact_command": "exact build and launch commands",
    "operation_sequence": "exact frozen semantic operations",
    "workload_cardinalities": "all applicable frozen 5.7 cardinalities, or explicit unavailable",
    "presentation_settings": "exact pane state, AA, overlays, resolution, DPI and render settings",
    "subscriptions": "actual selection/subscription identities and counts"
  }
}
```

The output preserves the entire config verbatim, instrument schema ID, start/stop, stop reason,
initial observed facts and runtime fact changes. Observed facts include current authored
source/profile/dependency identities (null when absent), scene/resident epoch, native enabled,
clock pause/rate/TPS, selection, system count, bridge path, reset state, diagnostic panel
hiding, window physical size/scale/focus. Runtime records are sampled at Last; they describe
that endpoint, not an input-event timestamp or the exact within-frame instant of a setting
change. Use immutable conditions for comparison; do not infer finer transition timing from
these records. Operator metadata separately carries the full qualification not available
from these existing presentation snapshots. Capture instrumentation allocation/serialization
of in-memory sample records is itself overhead and is not subtracted from frame samples.

Frame intervals beginning before F9 (including the warmup-straddling interval) are excluded
explicitly. The first zero delta is omitted. All subsequent nonzero raw intervals are retained
until stop/capacity, once each; an export interval cannot be measured. File creation refuses
overwrite. An I/O error keeps raw data in memory and reports the error in Studio's log.
Preserve/move any colliding or partial file before retrying F10; exit without export loses
the in-memory capture. No background or shutdown export is claimed.

## Proof and routing

Five owning AUDIT referees, boundary `0088-UI-PROTOTYPE-0`, birth 0.0.8.8, DSU 0:
raw tail/duplicate/capacity/export/metadata; scene/resident/reset refusal; actual native
adapter and equal shared CPU computation; real pinned Meridian G0 admission/replacement;
actual egui adapter consumption/deduplication/refusal. Existing three native behavior
referees and source-to-ready admission referee remain required. Synthetic timing/layout
fixtures validate the instrument; they are not physical-machine performance observations.
Exact test head, command results, hosted Doctrine/INSPECT and fresh Clearance are bound in
the PR and Board return after execution. No full M16 samples are collected in this leaf.

The Board expects `rehearsal-measurement`, but its live class table explicitly forbids
Mapeditor. These files are admitted by the canonical HD and covered by
`rehearsal-studio-presentation`. No class/router/gate table is changed; the actual fresh
verdict and expected-class mismatch return to Orchestration. No merge or graduation.

Deletion endpoint: remove this module, its plugin/state/lib mounts and timestamp hooks in
the existing app publication, egui and native adapter paths. No persisted application-state
migration is needed. Exported measurement files are operator evidence, not scenario inputs.
