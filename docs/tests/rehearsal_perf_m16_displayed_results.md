# M16 B0b exact QPC / DISPLAYED offline join

Status: PROBATION / owner raw validity evidence pending / OPEN / UNMERGED.
Authority: Board 5653633248, DA 5651557467, OVL addendum 5653843173.
HD-RECEIPT: 47010c34edf2. Base: a8c117313dbe3796aa160ba12cad1cd677bdc5e6.

The Workshop leaf reads the independent Mapeditor probe's JSON wire format and
external PresentMon 2.5.1 v1 CSV. It has no capture/runtime/rendering or simulator
authority and adds no dependencies. The Mapeditor instrument is a separate
presentation leaf. This implementation and its synthetic falsification tests do
not prove that any actual Windows input has been displayed.

## Correlation contract

PresentMon release source: https://github.com/GameTechDev/PresentMon/releases/tag/v2.5.1.
The pinned x64 executable SHA-256 is
`9bec3083069f58f911e6a512f4806db51a27bd096103087bc1d05ef54c80a191`.
`--qpc_time --v1_metrics` exposes integer PresentStartTime as `QPCTime`, the
DISPLAYED offset as `msUntilDisplayed`, and final Presented outcome as `Dropped=0`.
The parser never replaces these with v2 CPUStartQPC or DisplayedTime. Absolute
clocks use integer arithmetic, including decimal-to-QPC conversion rounded to the
nearest QPC tick. Floats are used only for reporting elapsed intervals.

Each complete app present-span interval must contain exactly one process-filtered
CSV present. The mapping must be a bijection across the captured span interval
with one swap chain. No nearest-neighbor inference, reused CSV row, unmatched or
ambiguous span is credited. Raw dropped rows and stopped-tail spans remain in the
report. A known dropped present is excluded from the endpoint; the first later
matching present actually reported DISPLAYED is eligible. Unknown display status
cannot be forgiven by selecting a later convenient sample.

Only the same response identity, extracted main frame, capture/scene/resident
epoch and qualified runtime conditions may match. Reversed causal timestamps,
foreign identities/PIDs, missing six-case sequence, app rejection, stopped/reset
or changed source/client/window/adapter conditions invalidate the report. The
known in-flight frame extracted before F6 is retained and explicitly excluded.
The first eligible DISPLAYED endpoint reports application ingestion-to-submission,
submission-to-DISPLAYED, and their total separately.

## Falsifier fixed before Owner data collection

Each client supplies three alternating K=0/K=4 pairs. Expected delay is the sum
of the actual K submission-frame intervals following the delayed input frame.
Tolerance is fixed as **two times the largest observed frame interval in that
pair's input-to-response windows, plus two QPC ticks**. The two intervals allow
one input-phase and one display-queue-phase displacement; the ticks allow decimal
rounding at both endpoints. This is an instrument-validation tolerance, not a
new performance service line. Expected delay must exceed tolerance, measured
shift must be positive, and absolute signed residual must fit tolerance. Thus a
zero-effect delay cannot pass. Every raw interval, component, shift and signed
residual remains available; no slow sample is removed to obtain a pass.

Four synthetic integration tests pass before the owner run. They use an absolute
QPC clock above f64's exact integer range and exercise exact decimal components,
first DISPLAYED after a dropped frame, no-effect falsifier rejection, retained
negative residual, ambiguous/missing/unknown display rows, foreign PID, response
before input, stale scene/resident/capture epochs, reset and stopped display.
These are fabricated fixtures for analyzer behavior, clearly distinct from
owner-run raw proof. Final leaf checks, hosted Doctrine, full INSPECT and fresh
Clearance remain required on the final PR head.

## Lifecycle and pending evidence

All four test rows are AUDIT / ledger-only, birth track
0.0.8.8-integrated-rehearsal, DSU 0, boundary 0088-UI-PROTOTYPE-0.
The analyzer/tests/results are rung-local measurement scaffolding to retire with
the 1.3 experiment. Actual JSON, process-filtered CSV, frozen executable/tool
hashes, procedure, machine qualification, Owner screenshots and analysis reports
remain to be harvested. No B1 comparison or winning-client conclusion is made.
