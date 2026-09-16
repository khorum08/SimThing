# Settlement correctness — leaf residual closure + ONE-integration-authority + ELEVENTH E8 roll

DA increment per Orchestration relay `5690342946` (Astra return `5690326096` accepted).
**Model 1 remains UNDECIDED; Model 2 remains CLOSED; no construction API or Model-2 surface
is admitted.** Draft #2062 stays OPEN/UNMERGED and returns to Astra rebased, with ONE
narrower finding returned below (per the relay's own STOP instruction) instead of silently
resolved.

## Defect A — balance-governed leaves omitted from residual closure (proven, repaired)

`append_residual_closure_ops` filtered `is_interior()`; leaves never received the
seed/add producers, so leaf `balance_rate`/`Balance` stayed zero while incoming
`AllocatedFlow` was correct (reproduced on the reference machine across A, B, A+B, both
arena orders, child reversal, component AND ordinary; parent-surplus positive control
rate=1/balance=1 proving the door executes).

**Repair:** every balance-governed participant receives the ONE admitted residual
semantics. A leaf's budget is its own `intrinsic_flow_col` (its `intrinsic_flow_sum_col`
is never produced) and it plans no child-subtraction op; interiors take the byte-identical
historical path (`depth==0||is_leaf` reduces to the old `depth==0` for interiors; same
ops, same bands). No stock ledger, host post-pass, readback correction, second allocator,
or Model-2 commitment layer.

## Defect B — ordinary over-integration ×3: EXACT cause proven before repair

Decomposed empirically (instrumented worktree at #2062's head; dispatch counts, per-op
census, and ablation):

1. Ordinary session dispatches RF bands ONCE and the base tick's C-7 velocity accumulator
   ONCE (`RF-BANDS-DISPATCH` ×1, `BASE-VELOCITY-DISPATCH` ×1).
2. Final RF upload census: `SYNC-PASS arenas=["a", "residency-row-capacity"]` →
   `integ_ops=[8,8,8,4,4,4]` — **each arena's plan embedded the FULL registry-wide
   governed integration at its own integration band** (`plan_governed_integration_at_band`
   called inside the per-arena loop). Every ordinary session holds the user arena PLUS the
   built-in `residency-row-capacity` arena, so every governed rate integrated ≥2× in the
   RF dispatch.
3. Ablating the RF-embedded integration: component 0.5→0.0 (RF was its only integrator,
   1×) and ordinary 1.5→0.5 (base velocity is clean at exactly 1×).
4. Total: base C-7 (1×) + per-arena embedding (2×) = the observed 3×. The factor is
   `1 + arena_count`, not a constant.

**Repair — ONE integration authority:** per-arena plans carry no governed integration
(sync passes the empty set); the sync appends ONE registry-wide integration tail at the
max arena integration band (all slots — the same coverage C-7 provides); and the kernel
tick pipeline's C-7 velocity dispatch stands down while an RF accumulator session is
active (`!accumulator_resource_flow_active`, a kernel-owned flag — no kernel-containment
breach; `passes.rs`/`gpu_sync.rs` are not seal components). With no RF session, C-7
remains the sole authority exactly as before. A governed rate integrates exactly once per
generation through the one active authority.

## Executed proof (reference machine, #2062's own matrix run against the repaired tree)

- **Leaf stock matrix GREEN in full:** A `[.75,.75,.75]/[.25,.25,.25]`, B mirrored, across
  single/joint resources, BOTH arena orders, child reversal, component AND ordinary
  resident session — flow, rate, and owned Balance settle in the same generation.
- **Parent-surplus positive control GREEN both paths** (rate=1/balance=1 — interior law
  preserved).
- Prior law GREEN: interior-policy overlay, post-RF observation, convergence seal, driver
  lib witnesses (including the two new ones below).

## Witnesses added (ledgered)

- `balance_governed_leaves_receive_residual_closure` — leaf: seed-from-own-intrinsic +
  add-allocated, NO subtraction; interior keeps the 3-op historical shape.
- `arena_plans_carry_no_governed_integration` — a production-shaped arena plan contains
  zero `IntegrateWithClamp` ops; the tail is the sync's alone (mutant: re-embedding
  per-arena integration fails this witness).

## RETURNED FINDING — seeded-rate diagnostic is now unexpressive on an arena leaf

Proof-floor case "pre-seeded `.5` rate integrates to exactly `.5`" conflicts with rule 1
on its own fixture: once a balance-governed ARENA LEAF lawfully owns residual settlement,
its rate is DERIVED each generation (seed+add), so a hand-seeded free rate on that leaf is
overwritten by the law itself — the fixture's premise, not an integration defect
(the integration-once law is proven by ablation and by the leaf matrix above).
**Recommendation to Orchestration/Astra:** re-express the seeded diagnostic on a
balance-governed property whose host is NOT an arena participant (no arena roles) — there
C-7 (or the RF tail, all-slots) integrates the seeded rate exactly once and the
multiplicity check is clean. No law is weakened here; the choice belongs to the leaf.

## ELEVENTH E8 roll (plan.rs + sync.rs are bundle components)

- **Old-pin refusal (RED, required first), repaired source, reference tuple:** ordinary
  ingress `UnqualifiedAdapter { required: 15419392269796833060, observed:
  17752583138607907014 }`; parity referee FAILED with
  `RESIDENT-CLEARING-QUALIFICATION-FINGERPRINT: f65dd84eae5040c6` — two independent
  derivations of the same observed value.
- **Roll:** both literals atomically `0xd5fc_af92_eda4_f724` → `0xf65d_d84e_ae50_40c6`;
  no third site; no comparator/component-list/`build.rs`/Cargo change.
- Pin chain: `0xb295…` → `0x42a7…` → `0x6f28…` → `0x6fe1…` → `0xd5fc…` →
  `0xf65d_d84e_ae50_40c6`.
- **Battery at floor:** parity 1/1 + mutant matrix, score-and-bands 3/3, runtime 4/4.

## Clean-checkout proof

- commit: `b228e4f0` (the exact eleventh-roll commit)
- command: fresh `git clone` at that exact commit (canonical LF checkout), sibling
  directory `simthing-e8-roll11-verify`, never %TEMP%;
  `cargo test -p simthing-workshop --features simthing-gpu/eml-resource-profiling --test resident_clearing_parity_0 -- --nocapture --test-threads=1`
- observed: `RESIDENT-CLEARING-QUALIFICATION-FINGERPRINT: f65dd84eae5040c6`; referee
  `1 passed; 0 failed` — pinned fingerprint reproduced from the fresh canonical-LF clone.
