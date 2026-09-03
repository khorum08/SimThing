# 0.0.8.7 SimThing Unification — the final recursive RF Triad object

> **Status: REVIEW CANDIDATE (DA-authored 2026-09-03, Owner-commissioned).** This document
> canonizes the SimThing recursive object AS IT NOW EXISTS after the full 0.0.8.7 arc
> (core ladder + Phases 13/14/15, every row DA-graduated, final certificate 137 suites /
> 512 pass / zero red @ `18b99513`). It is presented for engineering review before
> adoption. On adoption it either DISSOLVES into `simthing_core_design.md` (superseding
> the sections named in §8) or stands as the anchored unification ADR. Nothing here is
> aspiration: every claim below is implemented, witnessed, and stamped; the four
> deviations found by the closing deep-tree review are listed in §7 with dispositions.
> Until adoption, canonical law remains `simthing_core_design.md`.

## 1. The object

A SimThing is ONE recursive stem cell. Every specialization — WorldstateThing root,
Owner, gridcell, leaf — is the same anatomy under different parameters, never a second
implementation. The anatomy, as landed:

- **Identity**: a compact tree-local id. Realm (`TreeRealmId`), execution incarnation,
  and host locator are AMBIENT — O(1) per executing tree, never per-SimThing weight.
  Fork mints a new realm; stale incarnations fail closed; raw local ids never cross
  trees as identity (canon §8.2).
- **The market germ**: one recursively composable constrained-resource filter

      R_v : (S_v, Φ_v, {P_c, F_c, U_c, …}) → (P_up, {x_{v→c}}, U_v)

  Upward projection = the subtree's sufficient response to scarcity. Downward
  projection = resource flow per child edge. **exact market = Q ∘ R** — exact
  settlement is the deterministic quantization Q of the filter's downward flow into
  canonical identity-bearing possession (`T_s`); it is not a second market. There is
  no clearer. There is no disburser. (Canon §8.4; proven executable, 15.0.)
- **Two recursion axes, both substrate law**:
  - SPATIAL: `x_{p→v} = S_v` at both strata — continuous (the parent's live
    `AllocatedFlow` IS the child allocator's input; no propagated copies) and exact
    (`Q(x_{p→v}) = T_s(p→v)`, consumed by move at the child's intake).
  - TEMPORAL: `d_effective(v,N+1) = d_authored(v,N+1) + deform(U(v,N))` — exactly
    once, inside the single once-mint, where `deform` is an optional sealed EML
    program (absent = identity, bit-exact) whose admission makes `f(0)≠0`
    unrepresentable (`MayCreateWithoutUnresolved`). **Authored deforms; the
    substrate creates.**
- **The sufficient statistic** is born state, not new state: typed eligibility selects
  raw lawful pressure P (entitlement-first) or sealed Gu-Yang serviceable F
  (immediate flow) ONCE into the existing `AllocatorWeight`; λ is implicit in the
  normalized share; the minimal exact tuple is `(requested, AllocatedFlow)`. No
  ShadowPrice/DemandCurve/response plane exists (15.0 minimality theorem).
- **The Field Triad** (STEAD observation/bands, PALMA potential/impedance/opportunity,
  Gu-Yang capacity/flux) are upstream born authorities. Each influences a causal
  quantity EXACTLY ONCE — inside the born eligible state, never re-derived by the
  filter, never a second solve.
- **Hard precedence** is an orthogonal authority (`priority`/`order_weight`/score
  bands). Continuous curvature never becomes exact winner-take-all ordering; RF stage
  order / precedence / continuous share / exact residue are four distinct authorities.
- **Overlays** and consequence machinery are lawful CONSUMERS of the filter's outputs
  (U, impairment P−F, grants), never peer market authorities.
- **Structural consequences** (placement, fission/fusion, remap) remain sparse CPU
  boundary products via `BoundaryRequest` — the CPU's only ordinary job.

## 2. The one lifecycle (generation N, as physically executed)

    sealed Current N
      → born Triad state (STEAD/PALMA/Gu-Yang; admitted field sweeps)
      → typed eligible pressure (P or F, per commitment class) → AllocatorWeight
      → child-share EML on live parent columns → AllocatedFlow      [R downward]
      → exact quantization Q at ArenaBandLayout::integration_band
          basis = min(AllocatedFlow_i, requested_i), lossless Q149 limbs,
          largest remainder, generation-rotated exact ties, typed refusals
      → canonical T_s appended to the resident live head (bounded, typed egress)
      → child consumes the LITERAL T_s as supply (dispatch None; on-device)
      → unresolved U → optional sealed deformation → next-generation demand
          (once-mint; second mint typed-refused)                    [R temporal]
      → consequence consumers (valuation/CostBand/Overlay, observation, CausalBand)
      → N+1 proceeds from resident grants/U BEFORE any host materialization

Root binds authored supply at the same intake port (`dispatch(Some(rows))`); leaf
settles into actuation at the same settlement port. Root/interior/leaf are ONE
operator under degenerate parameters — a binding distinction, never a second one.

## 3. Canonical API surface — what every future agent anchors on

| Door | Law in one line | Home |
|---|---|---|
| `SimSession::open` | ordinary posture is `ResidentRequired`; CPU is opt-in, never fallback | driver |
| `RecursiveResourceFilterRuntime` (= `ResidentClearingRuntime`) | THE per-tree filter executor: continuous evaluator + exact session + buffers + live head | driver/gpu |
| `dispatch(…, Some(rows) / None)` | root-authored vs recursive intake; interior generations consume N's `T_s` buffer | driver |
| `prepare_root_continuous_allocation` → `plan_arena_allocation_with_pressure` | the PLAN owns pressure insertion; allocation is produced, never manufactured | driver |
| `produce_runtime_rf_next_generation_demands` (+ `RuntimeRfDemandGenerationAuthority`) | the once-mint Current→Next door; owns the U carry; non-Clone authority, compare-exchange | spec |
| `PersistenceDeformationProgram::admit / deform` | sealed EML on the carry; envelope proven by interval arithmetic at admission; `f(0)=0` law | core |
| `ResidentClearingPlan::replay_with_budget_envelope` (+ trusted envelope) | the consumer owns admission; wire budgets never self-authorize; typed replay errors | kernel |
| `TreeExecutionAuthority::seal` (+ `TreeGenerationAuthority` once-token) | one executing incarnation per generation authority | core |
| `IntegrationSchedule::record_grant_lifecycle` / resident live head | ONE history; parent row due exactly at N+1; struct-literal authoring unrepresentable | core |
| `GenerationStamped<T>` / `SeamFact<T>` | payload-parametric authority envelopes; the economic payload type NEVER translates | core |
| `ResidentConstrainedProduct` (roles: `ResidentSettlementOutput` = `ResidentRecursiveSupplyIntake`) | one exact product type; role names are aliases; any adapter is STOP-grade | kernel |
| `evaluate_recursive_resource_filter_oracle` (= `run_arena_allocation_oracle`) + `clear_constrained_scope` | the CpuVendorizedOracle's filter interface; same admitted semantics, one contract | driver/spec |
| Five quarantined CPU doors (`clear_constrained_claims_at_generation`, `clear_reduced_owner_channels[_at_generation]`, `clear_stamped_owner_channels`, `produce_runtime_rf_next_generation_demands` census row) | vendorized oracle vocabulary; may never gain an ordinary resident caller, fallback edge, or sixth sibling | spec |
| `GrowthEntitlementMarketBinding::resolve_batch_resident` | the ordinary production market consumer; quantity enters at the WEIGHT port (neutral identity) | driver |
| `BoundaryRequest` | the only door from resident economics to structural CPU work | sim |

## 4. Invariants that must survive into every future constitution

1. **Germ self-consumption** (canon §1.5.1): a germ's emission type IS its intake type;
   the recursion is the consumer; adapters are STOP-grade; role names are aliases.
2. **Opt-in CPU** (§8): waiting on the CPU is opt-in; a CPU brake at the stem cell is
   stillborn. `ClearingExecutionPosture{ResidentRequired, CpuVendorizedOracle}` is
   distinct from scheduling posture; unqualified adapters fail admission typed.
3. **exact market = Q ∘ R** (§8.4): quantization, not a second market. Q's mechanics
   (largest remainder, rotated ties, wide-integer refusals) are frozen law; only its
   numerator basis generalized (neutral degeneracy: a neutral field reproduces the
   requested-proportional oracle BIT-EXACT).
4. **Authored deforms; the substrate creates** (15.2): first-order persistence,
   pressure, and feedback are substrate; authored EML may only deform, bounded and
   sealed, inside the one mint.
5. **Triad exactly once**; **hard precedence orthogonal**; **generation pacing** (no
   same-generation reweight/re-clear/λ convergence — feed-forward only).
6. **Once-only laws**: pressure reduces once per edge (no descendant scans); U carries
   once (once-mint); every ordering the outcome depends on is WRITTEN law
   (physical-order invariance: rows, upload order, workgroups, partitions, epochs).
7. **The consumer owns admission** (14.2 R1→D lineage): callers cannot self-bind,
   wires cannot self-authorize, refusals are pattern-matchable types — never prose.
8. **Async non-foreclosure** (§8.2 F-laws): realm ambient; seam facts realm-qualified;
   emission+integration never atomic dual commit; delivery classes conserved/standing/
   observation; one clearing home per conserved scope per generation; no synchronous
   ancestor RPC in the hot loop; mutable global authority forbidden.
9. **Kernel containment**: ClauseThing and every application lower INTO admitted
   engine vocabulary; convergence is one-way; a kernel change motivated by an app is
   presumptively wrong.
10. **Define, don't validate**: doctrine lands as types > admission errors > guard
    scans > prose, in that order; witnesses key on symbols/content, never lines;
    grammars and gate lists are defined once and consumed everywhere.
11. **Process law**: graduation stamps at merge are machine truth; zero-red
    certificates at structural graduations; anchors at graduation with `until:`
    lifecycles for design authorities; measured numbers are dated evidence with
    reproducibility envelopes, never portable gates.

## 5. Execution substrate facts (landed, certified)

- GPU is the PRIMARY clearing authority; N and N+1 submit before any host
  materialization; the resident schedule segment is the authoritative live head with
  admission-bounded capacity and typed `ReplayEgressExhausted`.
- Two trees advance independently with overlapping local ids and divergent
  generations; no global clearer, lock, or barrier exists (censused at zero).
- Adapter qualification is mechanically exact (backend/vendor/device/driver/features/
  compiler/shader-hash/WG/ABI fingerprint; any field mutation invalidates); the
  current qualified tuple is `0x1c3ca3cf8e625e48`; cross-vendor is a dated follow-on.
- The 14.1 instrument is the frozen dated comparator (envelope-stamped; end-to-end
  −11.2% at cutover, baseline never re-blessed). It is byte-and-procedure frozen.
- The peer-authority census closed at 5→2 with survivors constitutionally pinned;
  net Phase-15 runtime effect: two aliases in, three authorities out.

## 6. Horizon and expansion strategies (all gated, none implied)

- **RESOURCE-RESPONSE-COMPRESSION-0** (richer bounded A(λ) response families, SIMD
  child-response evaluation): UNMINTED behind its four-fact gate — identity
  reproduction; capability unexpressible by scalar/tuple state without a second
  market; O(1) state per node; a real consumer or measured workload. Its absence is
  a decision.
- **Remote/async execution**: future and merely NOT foreclosed. The three dated
  debts (remote allocator authority; realm-qualified seam identity for every
  seam-visible id; SeamFactId retry idempotence) plus the two remaining deferred
  census rows are the entire known blocker set; seam vocabulary landed at 14.2.
- **Performance track** (post-closeout, from the dated ledger): Gu-Yang tiled
  gather; dense-materialization elision debt; segmented clearing at scale; a
  Phase-15-derived composite T_R metric if wanted (never by mutating 14.1);
  TreeVertical PALMA lowering (prefix/path composition on unique-path trees —
  physical lowering, never a second market law).
- **CPU oracle generalization**: the oracle now speaks the filter interface; future
  domains inherit R-vocabulary, and oracle-door retirement follows the existing
  census dispositions at their dated conditions.
- **Application horizon**: ClauseScript/Stellaris-class authoring exercises the
  germ through Draw templates, modifier-chain lowering (incl. persistence
  deformation), and consequence consumers — always above the engine boundary.

## 7. Closing deep-tree review — falsehood sweep findings (2026-09-03)

Hunting the 15.2 class: law promising ports that don't exist; prose describing
superseded anatomy; capabilities stranded without ingress. Four findings:

1. **Canon §1.5 grammar is a photograph of the OLD anatomy** (four steps now false:
   "EML valuation and effective clearing weight → authored constrained clearing →
   CostBand quantization → grant or flow disbursement"). DISPOSITION: rewrite §1.5's
   chain to the §2 lifecycle above at adoption of this document; the Draw/U/R
   paragraphs beneath it survive unchanged.
2. **The consequence chain has zero production or authoring ingress.** 15.0 proved
   it CONSEQUENCE-ONLY and 15.2 preserved it beside the port — but nothing invokes
   `fund_unresolved_persistence`/`AuthoredPersistenceValuation`/`PersistenceOverlayBinding`
   outside its own module, re-exports, and the 14.1 instrument; no ClauseScript
   lowering targets it. The mirror image of the 15.2 defect: capability lawful,
   wire absent. DISPOSITION: dated item with a named consumer — either a
   ClauseScript consequence-authoring ingress (the natural companion of the 15.2
   port, one small rung) or an explicit down-classification to
   instrument/oracle-only vocabulary. Engineering should choose which.
3. **The embedder still calls a quarantined door** (`clear_stamped_owner_channels`
   in `simthing-embedder/src/run.rs`). Likely lawful (the Vendor Door is host-side
   by design) but the door census's caller rows should CLASSIFY the embedder's
   posture explicitly so the quarantine law ("no ordinary resident caller") is
   checkable against it rather than assumed. DISPOSITION: one census row amendment.
4. **One noun residue**: canon line ~615 still says "the existing CPU clearer is the
   vendorized oracle." Post-15.1 vocabulary: the CPU FILTER ORACLE. DISPOSITION:
   one-word amendment riding the §1.5 rewrite.

No finding contradicts a graduated witness; all four are prose/wiring/classification
residue of the kind this review exists to catch before canonization freezes them.

## 8. Document disposition map (proposed)

| Document | Disposition on adoption |
|---|---|
| `simthing_core_design.md` | remains THE constitution; gains the §1.5 rewrite + §7 fixes; §8.2/8.3/8.4 already carry the async/cutover/filter law |
| this document | dissolves into the above OR stands as the anchored unification ADR — engineering's call; if standing, it takes `canonical` anchors on §§1–4 |
| `docs/workshop/SimThing_RF_Market_Core.md` | demote status header to HISTORICAL DESIGN RECORD (its normative content is canonized; it must stop reading as live design authority) |
| `docs/phase15_recursive_resource_filter_charter.md` | demote to HISTORICAL (anchors already repointed away; scope executed) |
| `docs/workshop/SimThing_Unification_Model.md` | superseded as description by this document; retain as the engineering-review record |
| stead/overlay/EML constitutional docs | untouched; this document cites, never duplicates |

## 9. Adoption mechanics

Engineering review → Owner ruling → one DA flight: apply §7 dispositions 1/3/4,
land the §8 status demotions, and either dissolve §§1–6 into core design or anchor
this document canonical. The closeout ceremony then reaps under the standing
Owner-gated hold with zero pending anchors, as the lifecycle law requires.
