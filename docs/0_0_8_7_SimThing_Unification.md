# SimThing Unification — the recursive RF-Triad stem cell

> **Status: REVIEW CANDIDATE v3 (DA-authored 2026-09-05, Owner-commissioned) — the ingress
> point, API reference, and ADR for the unified SimThing object.** Rewritten after the
> complete Phase-15 corrective arc (15.0–15.7 all DA-graduated; final certificate 142
> suites / 535 pass / zero red @ `c4d1b7a8`). Every claim below is implemented and
> witnessed unless explicitly marked OPEN; §9 is the per-rung proof ledger. Presented to
> the engineering cross-rung re-review of 15.5–15.7 together with the arc itself.
> **RE-REVIEW COMPLETE 2026-09-05: REMAND** — the scoped 15.5-15.7 results stand, but
> one STOP integration hole (ordinary-session temporal ingress), two semantic defects
> (cap-collision completion; arena-ambiguity), and one abort-safety falsifier gate
> adoption; sec 8 carries the remediation sequence. After remediation and a fresh
> narrow re-review, this document either dissolves
> into `simthing_core_design.md` or stands as the anchored unification ADR. Until
> adoption, canonical law remains `simthing_core_design.md` (whose §8.4 carries a
> KNOWN-REMAND marker scheduled for deletion in the post-re-review canon rewrite).

## 0. What SimThing is (start here if you are new)

SimThing is a GPU-primary simulation kernel built from ONE recursive object. There is a
single stem-cell type — the SimThing — and every specialization in any simulation
(the world root, an empire, a star system, a factory, a grid cell) is that same anatomy
under different parameters. There is no second implementation anywhere: no special root
class, no leaf class, no per-domain market or scheduler.

The one sentence the whole architecture serves:

> A StemThing evaluates one recursively composable constrained-resource filter. Its
> upward projection is the sufficient response of its subtree to scarcity. Its downward
> projection is the resource flow allocated over each child edge. The child consumes
> that same edge resource as its own supply. Exact settlement is the deterministic
> constrained projection of that flow into canonical identity-bearing possession; it is
> not a second market.

Everything an implementer needs flows from five separated authorities:

    pressure asks · precedence orders · allocation chooses · commitment reserves ·
    Gu-Yang realizes

If you remember nothing else: **the substrate creates; authored content deforms** —
your simulation's policy is expressed by deforming universal machinery through sealed
EML programs, never by adding parallel machinery.

## 1. Implementing a simulation idea — the cold path

1. **Author a scenario** (native `.clause` ClauseScript, or generated spec). Scenario
   is authoring input; `SimSession` is the runtime root. Your entities become SimThings
   in one tree; spatial things live on the Movement-Front gridcell lattice.
2. **Declare resources and markets** with sealed **Draw templates** — a Draw is a claim
   TEMPLATE (offerings, lifecycle triggers, finite quantity envelope sealed at
   admission), never a grant, and never a second demand quantity: runtime demand is the
   one `T_d` product it authorizes.
3. **Express policy as EML modifier chains.** ClauseScript's ordered modifiers lower
   into the one closed EML opcode interpreter. Three sanctioned policy ports:
   - **allocation weight** (how eligible pressure shapes continuous share);
   - **persistence deformation** (how unresolved demand decays/saturates/expires as it
     carries to the next generation — `PersistenceDeformationProgram`);
   - **consequences** (what unresolved demand COSTS — valuation → CostBand funding →
     OverlayThing effects, via the consequence ingress).
4. **Open the session** — `SimSession::open(...)` mints a fresh 128-bit OS-entropy
   execution realm, seals the execution lease, admits the resident filter, and runs
   `ResidentRequired`: the GPU is the primary economic authority; the CPU is an opt-in
   oracle, never a fallback.
5. **Generations breathe** (§3): demand rises, allocation resolves on-device, exact
   possession settles, children consume their grants as supply, unresolved demand
   persists — all before any host readback.
6. **Observe through born state**: STEAD bands, impairment `P−F`, grants, U — never by
   walking descendants. Consequences and UI are consumers of the filter's outputs.
7. **Persist / restore / fork** with the typed identity doors (§4, execution identity).
8. **Never add**: a clearer, a disburser, an urgency manager, a demand curve plane, a
   second history, a per-domain market. If your idea seems to need one, it needs an
   EML program or a consumer instead — see §6's forbidden shapes.

## 2. The object (anatomy, as landed)

- **Identity**: a compact tree-local id. Realm (`TreeRealmId`), execution incarnation,
  and host locator are AMBIENT — O(1) per executing tree, never per-SimThing weight.
  Fresh open ⇒ NEW entropy realm; restore ⇒ same persisted realm, NEW incarnation;
  fork ⇒ new realm from recorded fork identity; stale incarnations fail closed; raw
  local ids never cross trees as identity (canon §8.2; 15.7).
- **The market germ** — one recursively composable constrained-resource filter:

      R_{v,N} : (B_{v,N}, Φ_{v,N}, {e_{c,N}}) → (E_{v,N}, {b_{v→c,N}})

  `B` the continuous normalization budget; `e_c` the once-selected eligible scalar
  (raw pressure P for entitlement-first, born Gu-Yang serviceable F for immediate
  flow); `E` the direct-child aggregate response; `b` the continuous `AllocatedFlow`
  BASIS. Raw U appears nowhere in R's same-generation inputs. Then the exact stage:

      (T_s, U) = Π^exact_{C(S_free, r, h)} (b)        F_v = Π^exact_C ∘ R_v

  Π is the frozen exact constrained projection — Q149 lossless integer limbs,
  basis_i = min(AllocatedFlow_i, requested_i) with **integer-exact request identity**
  (no float ever decides exact ownership; the full u32 domain is preserved — E5 law),
  largest remainder, generation-rotated exact ties, typed refusals. The feasible set
  `C`: **free** conserved supply (`S_free = S_total − S_reserved`, reservations being
  explicit commitments in the `in_flight` holding lifecycle — never requests, never
  precedence integers), request caps, and hard-precedence bands where a zero-basis
  claim consumes NO capacity (work-conserving immediate flow — E6 Owner law).
  `b` is never the conserved supply; `S^exact_{v,N} = G_{p→v,N}` is a different
  symbol by law. One composition; no second market; no clearer; no disburser.
- **Two recursion axes — mechanically distinct (15.5), physically homed (15.6)**:
  - SPATIAL, at generation N: immutable `T_s.G` is consumed as the child's exact
    supply by a CHANGED child granter over the child's OWN market, semantic rows, and
    descendants. Continuous stratum: the parent's live `AllocatedFlow` IS the child
    allocator's input (no propagated copies).
  - TEMPORAL, alone advancing to N+1: `d_effective(N+1) = d_authored(N+1) + f(U(N))`
    — exactly once, inside the single once-mint, emitting the ordinary demand product
    (`ResidentTemporalDemand`); `f` is an optional sealed deformation program whose
    admission makes `f(0)≠0` unrepresentable. **Authored deforms; the substrate
    creates.** FORBIDDEN, mutant-guarded: `d(N+1) = G(N) + f(U(N))`; spatial intake
    that increments the generation or retains the parent granter/scope; temporal
    policy mutating a `T_s`-shaped payload. One immutable `T_s` record, two
    conversion-free consumers/views (G → child supply; U → temporal carry).
  - PREPARED ≠ EXECUTES: N+1 inputs may be prepared resident-side with zero host
    readback, but N+1 economics run only once sealed Current N+1 exists.
- **The sufficient statistic is born state**: typed eligibility selects P or F once
  into the existing `AllocatorWeight`; λ is implicit in the normalized share; the
  minimal exact tuple is `(requested, AllocatedFlow)`. No ShadowPrice, DemandCurve,
  or response plane exists (15.0 minimality theorem).
- **The Field Triad** (STEAD observation/bands; PALMA potential/impedance/opportunity;
  Gu-Yang capacity/flux) are upstream born authorities; each influences a causal
  quantity EXACTLY ONCE, inside born eligible state — never re-derived by the filter.
- **Hard precedence** is orthogonal (priority/order-weight/score bands); continuous
  curvature never becomes exact winner-take-all; RF stage order / precedence /
  continuous share / exact residue are four distinct authorities (plus commitment —
  five with reservation).
- **Overlays and consequences** are lawful CONSUMERS of the filter's outputs, never
  peer market authorities. **Structural consequences** (placement, fission/fusion,
  remap) remain sparse CPU boundary products via `BoundaryRequest`.

## 3. The one lifecycle (generation N, as physically executed)

    sealed Current N  +  TreeGenerationPermit(N)  [one permit per tree-generation]
      → born Triad state (admitted field sweeps)
      → typed eligible pressure (P or F, per commitment class) → AllocatorWeight
      → child-share EML on live parent columns → AllocatedFlow      [R downward]
      → exact constrained projection Π at the terminal integration band
          (S_free after explicit commitments; work-conserving bands;
           integer-exact basis; Q149; rotated exact ties; typed refusals)
      → canonical T_s appended to the resident live head (bounded, typed egress)
      → SPATIAL: child market consumes literal T_s.G as its supply (same N,
           changed granter/scope — dispatch_spatial)
      → TEMPORAL: U → optional sealed deformation → ResidentTemporalDemand for N+1
           (once-mint; second mint typed-refused; prepared ≠ executed)
           **[OPEN: the resident temporal doors are proven but the ORDINARY
           SimSession does not yet invoke them — remediation rung chartered;
           the CPU once-mint door is likewise oracle-posture only]**
      → consequence consumers (valuation/CostBand/Overlay; observation; CausalBand)
      → N+1 proceeds from resident grants/U BEFORE any host materialization

Root binds authored supply at the same intake port (`dispatch(Some(rows))`); a leaf
settles into actuation at the same settlement port; interior nodes self-bind. One
operator, degenerate parameters — never a second one.

## 4. Canonical API reference — the doors every agent anchors on

**Session and execution identity (driver / core):**

| Door | Law in one line |
|---|---|
| `SimSession::open` / `open_with_clearing_posture` | fresh 128-bit OS-entropy realm; `ResidentRequired` ordinary posture; CPU opt-in, never fallback |
| `SimSession::open_restored(PersistedTreeExecutionIdentity)` | same persisted realm, NEW incarnation; old permits/capsules/seam facts fail |
| `SimSession::open_semantic_fork` | new realm minted from recorded fork identity |
| `TreeExecutionLease` (session-owned, opaque) | authority lives as long as the executor; executor holds only a non-minting verifier |
| `TreeGenerationPermit(N)` | non-cloneable, one per TREE-GENERATION (all spatial edges within it), minted from the lease, consumed at the barrier; validated at every economic door |
| `PersistedTreeExecutionIdentity` | THE inert durable record `{realm_bytes, incarnation}`; restoration's source of truth |

**Market admission and qualification (driver):**

| Door | Law in one line |
|---|---|
| `ResidentClearingRuntime::admit` / `admit_sealed_market_with_persistence_deformations` | admits a market INTO the ordinary session RF arena (no private planes); persistence deformations sealed at admission |
| `ResidentMarketQualification` | sealed bound evidence over the full lowering tuple (market identity, resource/scope/Draw shape, arena/topology digest, registry generation, precedence, EML identity, exact-basis identity, ABI); any mutation ⇒ `StaleMarketQualification`; non-lowerable market ⇒ `MarketCannotLower` |
| `rebind_after_topology_change` | lawful growth rebind preserving realm/incarnation/generation/live-head/semantic identity/deformations/provenance; never a fresh executor |
| `install_default_resident_rf_property` / `build_default_resident_arena_registry` | the default arena wiring for ordinary sessions |

**Dispatch (driver — every door permit-checked):**

| Door | Law in one line |
|---|---|
| `dispatch(…, Some(rows))` | root-authored supply at the intake port |
| `dispatch_with_commitment_partition` | explicit commitments reserve via the conserved `in_flight` lifecycle, reducing `S_free` |
| `dispatch_spatial` | the child market consumes immutable `T_s.G` at the SAME generation under a CHANGED granter/scope |
| `prepare_temporal_demands` / `dispatch_temporal` | the once-mint temporal carry: `d_authored(N+1) + f(U(N))` as ordinary demand; prepared resident-side, executed only under the N+1 permit |
| `materialize` | asynchronous host materialization — AFTER economics, never before |

**Demand, persistence, and consequences (spec / core / clausething):**

| Door | Law in one line |
|---|---|
| `produce_runtime_rf_next_generation_demands` (+ `RuntimeRfDemandGenerationAuthority`) | the CPU once-mint Current→Next door; non-Clone authority, compare-exchange |
| `PersistenceDeformationProgram::admit / deform` | sealed EML on the carry; envelope proven by interval arithmetic; `f(0)=0` unrepresentable otherwise |
| `submit_authored_persistence_consequence` (+ ClauseThing lowerer + Studio door) | the consequence ingress: U observed → valuation → CostBand → Overlay; consequence programs CANNOT type as demand (E0308) |
| `IntegrationSchedule::record_grant_lifecycle` / resident live head | ONE history; parent row due exactly at N+1; struct-literal authoring unrepresentable |
| `GenerationStamped<T>` / `SeamFact<T>` | payload-parametric authority envelopes; the economic payload type never translates |
| `ResidentConstrainedProduct` (`ResidentSettlementOutput` = `ResidentRecursiveSupplyIntake`) | one exact product type; role names are aliases; adapters are STOP-grade |

**The CPU oracle (spec / driver / embedder):**

| Door | Law in one line |
|---|---|
| `evaluate_recursive_resource_filter_oracle` (= `run_arena_allocation_oracle`) + `clear_constrained_scope` | the `CpuVendorizedOracle` filter interface — same admitted semantics, one contract |
| Five quarantined CPU doors (embedder access only via `run::cpu_filter_oracle`) | vendorized oracle vocabulary; may never gain an ordinary resident caller, fallback edge, or sixth sibling |

## 5. Laws that survive into every future constitution

1. **Germ self-consumption** (canon §1.5.1): emission type IS intake type; the
   recursion is the consumer; role names are aliases; adapters are STOP-grade.
2. **Opt-in CPU** (§8): a CPU brake at the stem cell is stillborn; unqualified
   adapters fail admission typed; the resident schedule segment is the authoritative
   live head with typed bounded egress.
3. **`exact market = Π^exact_C ∘ R`** — an exact constrained projection over free
   supply, caps, and precedence; not a second market, and not a mere quantizer. The
   neutral field degenerates bit-exact to the frozen request-proportional oracle.
4. **Authored deforms; the substrate creates** — persistence, pressure, feedback, and
   consequences are substrate ports; authored EML only shapes them, bounded and sealed.
5. **The five authorities are distinct**: pressure asks, precedence orders, allocation
   chooses, commitment reserves, Gu-Yang realizes. A request never reserves; a
   precedence integer never reserves; only explicit exact commitment reserves, and
   only the committed quantity, through the conserved holding lifecycle.
6. **Two recursion axes, never identified**: spatial at N (changed granter/scope);
   temporal alone to N+1 (once-mint, through the demand authority); the forbidden
   equation `d(N+1) = G + f(U)` is permanently mutant-guarded.
7. **Exactness never bends to representation**: the admitted u32 domain is never
   narrowed for arithmetic convenience; no float decides exact ownership; every
   ordering the outcome depends on is written law (physical-order invariance).
8. **Triad exactly once; generation pacing** (feed-forward only, no same-generation
   reweight/λ-convergence); **the consumer owns admission** (callers cannot self-bind,
   wires cannot self-authorize, refusals are pattern-matchable types).
9. **Execution identity is durable and unforgeable**: entropy realms, persisted
   identity records, incarnation migration, per-generation permits; qualification
   covers the COMPLETE semantic kernel bundle (planner included) with build-time
   provenance — a shipped binary needs no toolchain.
10. **Async non-foreclosure** (§8.2 F-laws) and **kernel containment** (applications
    lower INTO admitted vocabulary; convergence one-way) stand unchanged.
11. **Process law**: stamps at merge are machine truth; zero-red certificates at
    structural graduations; falsifiers BEFORE remedies; cross-product witnesses at
    unification reviews (witness substitution is the known blind spot); define once,
    consume everywhere.

## 6. Forbidden shapes (each RED by plant, census, or type)

New clearer/disburser/market/urgency/response authorities; ShadowPrice/DemandCurve
planes; second histories or persistence lanes; demand re-injection from consequence
programs; `G + f(U)` recurrence; pseudo-`T_s` mutation; same-generation re-clear;
host descendant walks or economic query engines; private continuous planes or
host-authored production weights; request-based reservation; float request identity;
per-edge generation permits; runtime toolchain probing; adapters between germ ports.

## 7. Extension horizons (all gated; absence is a decision)

- **RESOURCE-RESPONSE-COMPRESSION-0** (richer bounded A(λ) response families): UNMINTED
  behind its four-fact gate (identity reproduction; capability unexpressible by
  scalar/tuple state; O(1) per node; a real consumer or measured workload).
- **Remote/async execution**: future, not foreclosed; three dated debts (remote
  allocator authority, seam identity coverage, SeamFactId retry idempotence) are the
  entire known blocker set; seam vocabulary and realm law already landed.
- **Performance track** (post-closeout, from the dated ledger): Gu-Yang tiled gather;
  dense-materialization elision; segmented clearing at scale; the E9 pair (persistence
  transform prepack; O(n²) validation shader); TreeVertical PALMA lowering; a
  Phase-15-derived composite T_R metric (the 14.1 comparator stays frozen).
- **Cross-vendor adapter qualification**: dated follow-on to the fingerprint law.

## 8. Open items (truth as of 2026-09-05 — re-review COMPLETE: REMAND)

1. **STOP — ordinary-session temporal ingress**: the resident temporal doors and the
   authored persistence port have NO SimSession production caller (session admission
   passes empty deformations; the cross-product referee drives the runtime directly).
   Remediation rung chartered: real-session cross-product through actual permit
   boundaries, authored deformation admission, and the adjudicated answer to WHEN
   d_authored(N+1) becomes authoritative (U stays resident; never force early
   authoring to fit a helper shape).
2. **Cap collision**: engineering recommends SATURATE-AND-REDISTRIBUTE (bounded
   active-set water-filling: g_i = min(r_i, λ·b_i); freeze capped rows; repeat;
   Hamilton on the final active set; all no-collision results bit-identical) —
   Owner ruling pending; a feasible vector must never fail-close.
3. **Arena ambiguity**: `preferred_arena = None → arena[0]` is order-dependent
   economic binding in production — resolve-or-typed-refuse; folded into the
   session-integration rung.
4. **Generation-abort safety (falsifier first)**: permit Drop reopens the generation
   after economic side effects may have committed; plant the failure-after-dispatch
   falsifier; expected law = touched/faulted seal state with fail-stop poisoning,
   no same-generation replay without explicit recovery.
5. **Canon rewrite** — after remediation + fresh narrow re-review: delete the §8.4
   KNOWN-REMAND marker, land the corrected Π/R mathematics, correct the Gu-Yang
   lineage (Gu-Yang = the conservative-relaxation / cancellation-front insight;
   SaturatingFlux is the generic transport primitive, capacity saturation is not the
   definition), and apply the historical-docs disposition map.
6. **Track closeout** — Owner-gated behind all of the above.

## 9. Proof ledger — what is proven, where (the unified review)

| Claim | Rung / ruling | Decisive witness |
|---|---|---|
| One operator; λ implicit; minimal statistic (requested, AllocatedFlow) | 15.0 `5520215583` | executable oracle-side R through the frozen parity corpus, bit-for-bit |
| Filter vocabulary is the architecture; peer authorities 5→2; zero new | 15.1 `5527814998` | symbol-keyed peer-authority census; conversion-free aliases |
| Authored persistence policy (decay/saturation/expiry) | 15.2 `5526322376` | `MayCreateWithoutUnresolved` admission; 8-modifier chain; interval-arithmetic envelope |
| Consequence authoring ingress; consequence-only in types | 15.3 `5529923036` | E0308 fences; gen-10 clear → gen-12 Overlay effect end-to-end |
| Quarantined oracle vocabulary cannot define architecture | 15.4 `5532331425` | E0432 seals on bare paths; `cpu_filter_oracle` module |
| Axes separated; forbidden equation dead; E6 work-conserving/commitment law | 15.5 `5535368453` + Owner `5533372979` | THE CROSS-PRODUCT WITNESS (G4/U6 → child market → authored 2 → demand 8; half-deform 5; CPU/resident parity); `[0,1,9]`; six mutants RED |
| Real-substrate homing; sealed qualification; u32 exactness; E7 rebind | 15.6 `5547526204` + E5 `5536135335` | flat-star symbols deleted; live-weight flip flips winner; 2^24 pair = oracle winner in full production; cross-product rerun on real arena |
| Lifetime authority; entropy identity; permits; semantic-bundle qualification | 15.7 `5548610281` | double-open distinct realms; permit refusal matrix; four bundle-mutation referees; zero runtime rustc |
| Foundational Phase-14 substrate (plan/pressure/apportionment/parity/cutover) | 14.2–14.6 rulings (`5489390011`, `5504763991`, `5513743095`, `5515763386`, `5518579252`) | typed replay trust chain; neutral identity binding; Q149; nine-item parity; causal N→N+1 cutover |

Eight consecutive first-run-clean structural certificates (15.2→15.7 inclusive) stand
behind the arc; every remand in Phases 14–15 was one law applied to successive ports:
**the substrate owns the act; callers and payloads merely request it.**

## 10. Glossary

**T_s** — the canonical exact constrained product (possession): grant G + unresolved U
+ identity/provenance. **T_d / demand** — the one lawful demand quantity.
**P / F** — raw lawful pressure / Gu-Yang serviceable pressure (eligibility bases).
**b / AllocatedFlow** — the continuous allocation basis (policy, not possession).
**Π^exact** — the frozen exact constrained projection (Q149 + largest remainder).
**U** — unresolved lawful quantity; carries once to N+1 demand, optionally deformed.
**Draw** — sealed claim template (authorization metadata, never a grant or quantity).
**EML** — the one closed opcode policy language every authored behavior lowers into.
**STEAD / PALMA / Gu-Yang** — the born field Triad: observation/bands; potential,
impedance, opportunity; capacity and realized flux. **Realm / incarnation / permit** —
durable execution identity; transient execution of it; one generation's license.
**Live head** — the resident, admission-bounded authoritative head of the one
integration schedule. **BoundaryRequest** — the only door to structural CPU work.
