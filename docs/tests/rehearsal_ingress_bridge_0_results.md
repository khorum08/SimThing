# 0088-INGRESS-FIDELITY-0: bridge convergence and post-RF publication reserve

Status: **PROBATION / proof-present / clearance-pending / OPEN / UNMERGED** bridge/UI leaf.
This packet does not close rung 1.1. The exact tested checkpoint and final check results are
recorded in the PR and Board return accompanying this committed packet.

Resume dispatch **5619313604**, ACK **5619921257**, HD-RECEIPT **`70df16e1babd`**.
Live base **`b4452211a20162d31d2364df13dee62d1fe5edc8`**. DA repair #2031 discharged
the publication and UI-caller reserves recorded below. After rebase, the retained publication
test passed unchanged: all canonical observations equal completed RF producer output, including
the formerly unselected child perturbation. Original reserve details remain historical evidence.

The UI now uses `StudioAppState::try_adopt_loaded_scenario_session` to admit a live candidate
before committing the document/settings. Its async loader keeps new geometry hidden and cancels
it if admission fails, preserving the previous scene. Manual JSON load, picker load, candidate
reopen and blank creation use the same transaction. Successful adoption clears the pending reset
so the next bridge system cannot detach the replacement it just admitted. The owning UI test
calls this actual transaction against a running session: rejection preserves the serialized
document/settings, nonempty canonical observation, execution identity and tick progression;
success changes the document and executes without a detach/reset. No `app/mod.rs` edit was needed.

Dispatch: Board 5612687963. Resume ACK: 5612810803. HD-RECEIPT: `5926d344a075`.
Coding ORIENT-RECEIPT: `28f56884d309`; all 48 anchor ACKs carried from Board 5612017867.
Base: `1fc9bb1f95c5fd485aa8dad2b012b0f2b948d6b5`. Branch:
`codex/0088-ingress-fidelity-0`.

## F1: preserved clean semantic RED

Commit **`e59dbef317689bf6c23a8a9cf208de35b0fa1946`**, also retained as
`codex/0088-ingress-f1-red`, contains the owning falsifier and inventory row with no
production changes. `git status --porcelain` was empty before and after this command:

```text
cargo test -p simthing-mapeditor --test rehearsal_ingress_fidelity -- --nocapture
```

Result: 0 passed, 1 failed; exit 101. The canonical programmatic control opened successfully
and installed 17 RF participants, including eight positive children, two zero-valued children,
two owners and unequal RF depth. It installed one distinct policy on each owner. The old
Studio bridge also opened successfully but installed only the session root, alpha and its
first three positive children. Twelve authored participants were missing, including all beta
participants and both zero-valued children. Both installed policy counts were zero.

This is a semantic installation failure, not a parser, compiler or adapter failure. The source
is authored at the existing public `HydratedScenarioPack` boundary using existing spec types
and fixed by a JSON roundtrip before installation. A small ClauseThing source supplies owner
identities. It is **not** the later native `.clause` / supported-interchange witness. Its
admission-time value checks use the installed runtime tree and RF admission report; they do
not claim GPU-born observation.

## Implementation checkpoint

- Deleted the three-candidate composition, synthetic RF property/budget producer and legacy
  hydrate-root live fallback. Profile creation now returns an error without canonical authority.
- Retained complete property definitions and values, RF parent edges and authored programs.
  Compiled properties live in the existing scenario registry; domain packs, events, capability
  programs, region fields and overlays proceed through ordinary spec installation. Envelope
  overlays use the existing domain-pack install door without kind/name filtering.
- Used `admit_intrinsic_owner_channels` for owner-reference convergence. Authored Location
  subtrees attach once at their top-level roots instead of repeating flattened child targets.
- Kept RF telemetry as a read-only locus chosen from the admitted graph after installation.
  It does not choose execution participants or construct economic values.
- Prepared replacement bridge state before committing it. Invalid programs preserve the
  prior execution identity, nonempty GPU observation table and tick count; the prior session
  continues afterwards. Explicit structural-shell preference refuses to discard an authored
  economy.

The focused F1 installation successor and bridge replacement test passed locally. F1 now
installs all 17 participants and both policies, preserves the two RF paths, and resolves each
child to its actual owner. The running safety variant authors its Balance with the existing
primary `Amount` role and requires nonempty observation rows. The original F1 input has only
named roles and is retained unchanged in the preserved RED commit.

## Historical reserve, discharged by #2031: canonical post-RF observation was stale

Owning first consumer:
`rehearsal_ingress_post_rf_observation_matches_born_allocations` in
`crates/simthing-mapeditor/tests/rehearsal_ingress_fidelity.rs`.

The running variant authors `AllocatedFlow` as the canonical observable `Amount` role and
adds two real deficit claimants to the original graph. An ordinary bridge opens the resident
session and executes one `step_once` through `consume_scheduled_ticks`. The test compares
`AnchorTableSnapshot` / `observe_hosted_property_cell` with a diagnostic read of the completed
GPU producer buffer, using registry-derived columns and admitted participant slots. The raw
read never substitutes for the canonical observation in application code or the acceptance.

Observed diagnostic values:

| Locus | Producer, child rate 8 | Producer, child rate 20 | Canonical read, both |
|---|---:|---:|---:|
| alpha | 25.636364 | 25.636364 | 0 |
| beta | 21.363638 | 21.363638 | 0 |
| beta_3, formerly unselected | -4.1272726 | 7.8727274 | 0 |

RF is active. The child perturbation reaches the actual producer output; the canonical
observation remained stale before #2031. The retained test requires the canonical read to equal
that completed output and then expose the child change. It is now **GREEN** solely from the
substrate repair, without changing, ignoring, inverting or replacing its assertions.

Code trace at the base engine implementation:

- `crates/simthing-driver/src/simulation_fabric.rs`,
  `run_simulation_fabric_hot_step`: ordinary tick, then RF bands, then the need-threshold rescan.
- `crates/simthing-kernel/src/world_state.rs`, `run_resource_flow_bands_with_fast_path`:
  dispatches RF writes to resolved values without an anchor-table maintenance dispatch.
- `rescan_accumulator_thresholds_after_resource_flow` returns immediately when
  `post_rf_need_threshold_regs` is empty. Its later anchor-maintaining dispatch is conditional
  on those unrelated registrations.

Required generic capability: publish completed RF writes through the canonical observation
authority in the ordinary session path, including sessions without need bindings. This is
driver/kernel ownership; a Studio-side mirror, raw-read fallback or dummy need registration
would evade the contract. The HD explicitly stops on any required engine-src edit and calls
for an admitted D increment landed with its first consumer. No engine file was edited.

An exploratory second-owner multiplier change did not change producer allocations in this
particular specimen. Only policy installation/preservation is proven here. The final
policy-sensitive application stimulus remains open; no additional engine-policy gap is claimed.

## Remaining ingress work and historical UI scope gap

Native `.clause` / supported canonical-JSON convergence, G4 portable dependency identities,
source file/span refusal provenance, full second-owner policy sensitivity, historical-stage
successor coverage and census transitions remain open. No census-ready signal is made.

The bridge replacement proof is not a proof of the complete UI transaction. Source tracing
found `app/ui.rs` adopting a loaded document and requesting a reset first; `app/mod.rs` then
calls `apply_live_bridge_reset_before_tick`, which detaches the old session before the new
admission attempt. The async loader and manual/native open call sites are in
`crates/simthing-mapeditor/src/app/ui.rs`, outside the HD. At least that caller surface needs
admission to stage the live candidate before document/scene adoption and to avoid detaching
an already admitted replacement. No edit to that surface was made. Orchestration should
resolve this narrow scope gap with the engine reserve; it must not infer complete UI safety
from the passing bridge test. This caller gap is now discharged by the admitted UI transaction
and its owning test described at the top of this packet.

No pen renewals were performed: the four new owning tests have behavior-regression /
AUDIT / ledger-only rows under `0.0.8.8-integrated-rehearsal`, DSU 0. They reproduce concrete
ingress and publication faults; no permanent-proof classification is claimed. No historical
test was weakened or deleted.
No self-triage, merge, census/protected-file change or final clearance was performed.
