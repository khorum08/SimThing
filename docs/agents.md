# SimThing — Agent Briefing

> **This file is a router, not a knowledge base.** It tells you what to read and how to run
> the project — nothing else. It deliberately contains **no implementation state** (the
> production-track status ledger owns that), **no design rationale** (`simthing_core_design.md`
> owns that), and **no history** (`docs/archive/` owns that). The previous 990-line v4-era
> briefing is archived verbatim at `archive/superseded_design/agents_v4_briefing.md`; do not
> treat it as current — its crate states, open decisions, and shader sketches are obsolete.

---

## Read order (mandatory, in this order)

1. [`simthing_core_design.md`](simthing_core_design.md) — **the paradigm. Hold it in context
   for your entire task.** The recursive GPU-resident SimThing tree, SimProperty→Value,
   resource flow arenas, overlays, the Movement-Front automaton, the EML opcode discipline,
   and the drift-detector litmus tests.
2. [`invariants.md`](invariants.md) — the binding rule tables. A violation is a compile
   error, a test failure, or a voided closure.
3. [`design_0_0_8_3.md`](design_0_0_8_3.md) — **the ACTIVE constitution** (carry-forward
   transient constitution §0 + the ClauseThing-vertical closeout addendum §A). It supersedes
   `design_0_0_8_1.md`; that predecessor stays in place and §B of 0.0.8.3 incorporates its
   operating mechanics (§2), parked inventory (§3), and closed questions (§4) by reference.
4. The live status ledger
   ([`design_0_0_8_0_consumer_pulled_production_track.md`](design_0_0_8_0_consumer_pulled_production_track.md))
   + the one test report for your slice. **Active track:**
   [`design_0_0_8_6_studio_live_ops.md`](design_0_0_8_6_studio_live_ops.md)
   (`STUDIO-SIM-CLOCK-UI-0`). No live `docs/todo.md` (archived workshop todos are not authority).
5. **If your task is in the ClauseScript / MapThing / MapGenerator vertical** (now CLOSED):
   start at [`clausething/ClauseThingDoc.md`](clausething/ClauseThingDoc.md) (clearinghouse:
   concepts/practices/APIs) and [`adr/ClauseThingADR.md`](adr/ClauseThingADR.md) (decisions).
   The production-track ladders are archived under `archive/closed_production/`.
6. **MANDATORY for any spatial task** — [`stead_spatial_contract.md`](stead_spatial_contract.md)
   is **required reading** (in addition to core §0/§7) before you touch any of: MapGen lattice,
   MapGeneratorCLI, Location grids / `grid_metadata` placements, the Movement-Front engine, STEAD,
   heatmaps / falloff / fronts, PALMA, Gu-Yang / SaturatingFlux, Resource Flow or Accumulator
   arenas whose participants are gridcell `Location`s, structural vs render coordinates, layout
   vs execution-profile admission, or field visualization. It is short, normative, and enforced by
   `crates/simthing-clausething/tests/stead_spatial_contract_guards.rs`. STEAD/Mapping has drifted
   catastrophically three times; this contract exists to make that impossible.

Nothing else by default. If your handoff cites additional rung-local links (≤3), read those
too. If something seems missing, search the repo before re-deriving from priors —
**source code is ground truth over any document.**

## Repository layout

```
crates/
  simthing-core/      Core types + role pathway → ColumnIndex (col_for_role; not ColumnIndex::new).
  simthing-kernel/    Sole runtime authority (ExactMagnitudeProof, decision ingress,
                      OpcodeRegistrationGate). Read sanctioned_surface.md + eml_gadget_library.md first.
  simthing-gpu/       wgpu, AccumulatorOp WGSL, EvalEML interpreter. Semantic-free.
  simthing-sim/       Tick/boundary orchestration. Semantic-/map-/arena-/gadget-free.
  simthing-driver/    Session assembly; production-path tests.
  simthing-spec/      Designer/RON admission; gadget registry. Semantics live HERE.
  simthing-feeder/    Bridge gameplay state ↔ GPU pipeline.
  simthing-workshop/  Sandbox probes. Never a production dependency.
docs/                 Active docs. docs/archive/ = historical only.
scenarios/            Authored RON scenarios.
```

## How to run tests

```
cd C:\Users\mvorm\SimThing
cargo fmt --all -- --check
```

**Default rule:** do **not** run `cargo test --workspace` unless the rung absolutely requires
full-workspace proof. For isolated parser/doc/license/report work, run **touched-crate tests**
plus inspection only.

**Targeted examples:**

```
cargo test -p simthing-clausething
cargo test -p simthing-driver mobility_gpu_kernel7
cargo test -p simthing-driver mobility_gpu_kernel8
cargo test -p simthing-driver mobility_gpu_kernel9
cargo test -p simthing-driver mobility_gpu_kernel10
cargo test -p simthing-driver mobility_gpu_kernel11
```

**Expensive mobility GPU replay gates (kernel7–kernel11):** conformance/replay/accounting/budget
batteries are `#[ignore]` and run only for mobility GPU replay/accounting/budget/dispatch/shader
changes:

```
cargo test -p simthing-driver mobility_gpu_kernel10 -- --ignored
```

Full workspace testing is not routine proof until a future TEST-PARING-0 classifies the corpus.
When a rung does require workspace scope, GPU tests skip cleanly when no adapter is available
(`try_gpu()` returns `None`). Other ignored diagnostics (e.g. timing capture, exhaustive sweeps)
run with `cargo test -- --ignored` only when explicitly needed.

Run a recorded session (record needs a GPU adapter; replay is CPU-only):

```
cargo run -p simthing-driver -- record --scenario scenarios/rebellion_demo.ron --out demo.replay.ldjson
cargo run -p simthing-driver -- replay --in demo.replay.ldjson
```

Named guard tests you must never weaken:

- `custom_layout_ethics_axis` — proof the property-layout generalization works beyond the
  standard amount/velocity/intensity layout. New layout capability ⇒ add a test in this pattern.
- `pass3_overlay_matches_evaluator` — proof GPU transform application stays bit-exact with
  the CPU `Evaluator` across all `TransformOp` variants. New transform variant ⇒ extend it
  with a parity assertion.

## CI doctrine-scan (the automated doctrinal screen)

Every PR and push runs the free GitHub **Doctrine Scan** (~1 min: self-test → scan → verdict). It is the
**mechanized rung-3 of the admission ladder — the automated DA scan layer, not governance theater.**

- A clean **RELIABLE** result is **DA-equivalent** — trust it, don't re-verify. **FAIL** is a HOLD: fix the
  violation, or — only if it is a legitimately new sanctioned door — add a *conforming* `allow/*.txt` record
  under the rigor below. **Never edit the scanner to dodge a valid finding.** **INSPECT** routes to §1A
  triage, never a silent pass.
- To change *what is screened*, read [`ci_screening_surface.md`](ci_screening_surface.md) — the authoritative
  map of the scan/allow/block lists and the **strict rigor** for a `scans.tsv` / allowlist entry (7 fields, a
  mandatory `promotion-blocker`, promote-to-a-type-not-a-scan, no-invariant-in-the-engine, prove-or-it-doesn't-land).
  A change to the screening surface updates that reference **in the same PR**.
- **The floor the CI cannot enforce, so you must:** run the check and **paste real output — never assert or
  fabricate**; **verify the tree, not the relayed report**; **no merge before DA clearance** on any authority /
  gate / PROBATION rung. Anti-kabuki guidance cuts ceremony, never this floor — see the handoff template §H
  (the binding authority; do not restate or dilute it elsewhere). A clean doctrine-scan **RELIABLE** is
  DA-equivalent for *what the scanner covers*; it does **not** replace weighted tree verification when the DA
  graduates code-facing / long-lifecycle / horizontally impactful work.
- **DA treeverify:** before load-bearing graduate/admit, `bash scripts/ci/da_treeverify.sh --pr <n>`
  → advisory `DA-TREEVERIFY-PROFILE` (not a clearance verdict). Require tree confirm for
  production/kernel/gate-wiring; relax for policy/stamps. Full ritual: [`agent_onboarding.md`](agent_onboarding.md).
- **Inner loop:** `cargo check -p <touched-crate>` → `bash scripts/ci/agent_scan.sh`. Whole-tree
  `doctrine_scan.sh` = CI/maintainer only.
- **Cold start:** `bash scripts/ci/orient.sh --role=coding` once; carry `ORIENT-RECEIPT`.
- **Handoffs (HD):** work arrives as a repo object rendered per role —
  `bash scripts/ci/handoff_dispatch.sh --render <coding|orchestrator|da> handoffs/<RUNG>.hd.md`; obey
  BUILD/FENCES/EXIT-PROOF, quote `HD-RECEIPT`. Schema: `handoff_template.md`; protocol/board: `agent_onboarding.md`.
- **Doctrine lookup:** `bash scripts/ci/anchor_query.sh` (not raw greps); anchored edits →
  `anchor_check.sh --resync`.
- **Clearance intake:** auto-posted **Clearance Report** sticky; `/clearance` exceptional.
- **Track closeout:** `bash scripts/ci/track_closeout.sh` sole birth_track closure authority.
- **Onboarding standard:** [`ci_screening_surface.md`](ci_screening_surface.md) §7 / §6 / §8.

## Agent completion discipline (mandatory for implementation rungs)

Before any SimThing **implementation or remediation** handoff, read and follow
[`tests/agent_completion_discipline_0.md`](tests/agent_completion_discipline_0.md).

Operational requirements:

- Create the `docs/tests/<rung>_results.md` skeleton **before** long validation.
- Draft the PR summary with boundary commitments and `PENDING` validation rows **before** merge.
- Run **focused** validation first; do not end the turn in raw cargo output.
- If cargo hangs or times out, terminate honestly and record PARTIAL/FAIL — never treat timeout as PASS.
- Update PR body, evidence index, and production synthesis before merge when doctrine/status changes.
- Post-merge evidence placeholders must be replaced by PR number and merge SHA before the next implementation rung.
- End every turn with the structured final summary (`Status`, `PR`, `Merge`, `Implemented`, `Validation`, `Evidence/docs`, `Known gaps`, `Next recommended action`).

## Working rules (the short version — binding text is in the read order above)

- Classify your change first: Tier-1 fast lane ships as one PR + one test report + one
  status row; Tier-2 keeps the full gated cadence
  (`workshop/phase_m_gating_and_doc_policy.md`).
- Everything is opt-in / default-off; no default `SimSession` wiring without a named gate.
- Exact claims carry bit-exact CPU-oracle parity (`f32::to_bits()`).
- A scenario is proven only through a real reduction — never admission-rejection tests plus
  a CPU math loop.
- Stop-and-escalate on any stop-condition in your handoff or any conflict with
  `simthing_core_design.md` §9. Do not rationalize; do not re-derive architecture.

## Code style notes

- No comments explaining what the code does. Names should do that.
- Comments only for non-obvious WHY: a hidden constraint, a specific invariant reference,
  a workaround for a wgpu behavior, a simulation design decision.
- Reference invariants when a comment explains a rule: `// invariant: velocity pin`.
- Tests live in the module they test (`#[cfg(test)] mod tests` at the bottom of each file).
- New types go in the module that owns them. Don't create new files for small additions.
- No `unwrap()` in non-test code without a comment explaining why the None case is impossible.

## Cursor Cloud VM caveats (non-obvious only)
- Install `python-is-python3` or otherwise ensure `python` resolves; `agent_scan.sh` invokes `python`.
- Toolchain ≥1.85 required (`simthing-clausething` is edition 2024); VM rustup stable is fine, cargo 1.83 cannot parse the workspace.
- No physical GPU (`/dev/dri` absent): before `simthing record`/`bench` or GPU-backed tests, `export VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json` and `unset DISPLAY` (stale `DISPLAY=:1` breaks wgpu adapter enumeration). Replay and pure-CPU tests need neither.
- Linux-runnable binaries: `mapgen` and `simthing` only; `simthing-studio` is Windows-only (exits on non-Windows; its lib still builds/tests here).
- `cargo fmt --all -- --check` may show pre-existing rustfmt drift; do not reformat untouched files. `scripts/ci/` is governance harness, not product build.
