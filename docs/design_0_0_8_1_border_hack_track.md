# SimThing 0.0.8.1 — Border Hack Track (`BH-`): the C_u saturating-flux stencil operator

> **Status: OPEN (2026-06-12) — seated as a generic GPU utility, PALMA-style.** Product
> authorization: borders, frontlines, and choke topology as **free-ish side effects of the
> heatmap pass** on late-game crowded maps — no border service, no segmentation pass, no border
> objects. Adjudicated by executive design authority from the C_u proposition digest
> (Gu & Yang 2025, arXiv:2509.20797, *Relaxation to equilibrium of conservative dynamics II*).
> Consumer anchor: the closed CT-3b+4a movement-front vertical
> ([`clausething/ct_vertical_consumer_contract.md`](clausething/ct_vertical_consumer_contract.md))
> and the SEAD horizon. One rung in flight; rung = one PR + one report + one status row.

---

## 1. Adjudication (binding)

1. **Not already expressible.** The admitted stencil operators (`Normalized`,
   `SourceCappedNormalized`, `Gradient`) carry admission-time constant weights. EvalEML cannot
   gather N4 neighbors (`SLOT_VALUE` reads the evaluated slot's own row). State-dependent
   weights — weights that are functions of the field itself — require a **new generic stencil
   operator variant**. No prior EML hack covers this.
2. **Admissible under the standing Tier-2 gate** (invariants, EML Gadget Library: "extending the
   generic substrate vocabulary is a Tier-2 gate, not a prohibition"): the operator is
   semantic-free (WGSL sees floats and caps; no faction/border/map nouns), CPU-oracle bit-exact,
   meaning pinned at admission, reusable by any RegionField. It is the same admission shape as
   `source_capped_normalized`.
3. **Provenance caveat (binding on every BH rung).** The local product form below is an
   **engineering ansatz inspired by** Gu–Yang's hydrodynamic-limit results (algebraic `t^{-d/2}`
   variance decay; macroscopic linearization of non-gradient exclusion). It is **not** a literal
   theorem of the paper, and **no BH rung may claim paper fidelity in a PASS row.** What we
   claim — and test exactly — are the operator's own properties: conservation, bounded
   stability, saturation choking, and fixed-point convergence. Model-exactly-or-reject attaches
   to *those* claims.
4. **Relationship to decision doctrine.** Gradient descent over fields remains the primary
   decision-emergence arena. C_u does not decide anything; it **shapes the field** so that its
   gradients already encode crowding — descent then routes around choke walls for free. Border
   positions are *readouts* of where flux froze, never objects, never a service.
5. **The conservation bonus (stronger than the digest claims).** The symmetric pairwise flux is
   antisymmetric under `i↔j`, so the operator is **mass-conserving by construction** — unlike
   the attenuating `Normalized` operator. This makes it the natural evolution operator for
   conservative RF-fed pressure (redistribution instead of decay). Conservation is
   property-tested (sum preserved to float-order determinism), not claimed exact-authoritative.

## 2. The operator (pinned v1 math — implement exactly this)

Per cell `i` with N4 neighbors `j`, on the existing ping-pong buffers:

```
σ(u)   = clamp(u / u_sat, 0, 1)                      // neighbor saturation, authored u_sat > 0
C_i    = χ · Π_{j ∈ N4(i)} (1 − σ(u_j))              // transient, register-only, never stored
u'_i   = u_i + Δt · Σ_{j ∈ N4(i)} ((C_i + C_j)/2) · (u_j − u_i)
```

- **v1 χ is an authored constant** `0 < χ ≤ 1` (structural compressibility). A state-dependent
  `χ(u)` ramp is a later rung if a consumer names it.
- **Admission bounds (hard):** `u_sat` finite `> 0`; `0 < χ ≤ 1`; CFL-style stability bound
  `Δt · χ ≤ 1/4` (N4); boundary mode `Zero`; horizon caps inherited unchanged; operator name in
  the spec enum: `SaturatingFlux { u_sat, chi }`.
- **Stowaway contract:** `C` is computed in registers from neighbor values the stencil already
  fetched, applied, and discarded — zero bytes of new global state for the dynamics. Border
  *consumption* (BH-1) is an explicit opt-in output column, not a hidden write.
- Out-of-band values: non-finite inputs are admission/runtime-rejected exactly as the existing
  operators handle them. No sqrt anywhere (exact-sqrt rule untriggered).

**Emergent behavior to property-test, not assert by faith:** saturated neighborhoods
(`σ→1 ⇒ C→0`) freeze flux (walls back up instead of diffusing); clear space (`C≈χ`) relaxes
fast; disturbances propagate until they hit contested zones and grind; the SEAD fixed-point pass
converges with walls in place.

## 3. The ladder

| Rung | Gate | Scope | Exit criteria |
|---|---|---|---|
| **BH-0** | T2 (substrate gate) **[FRONTIER]** | `SaturatingFlux` operator: WGSL variant in `StructuredFieldStencilOp` + `cpu_stencil_step` arm + spec admission (`RegionFieldOperatorSpec` widening + bounds) | CPU-oracle **bit-exact** parity across ping-pong horizons; property tests: conservation (sum-preserving), stability at the CFL bound, choke-wall formation (high-σ block freezes flux), clear-field limit (σ≈0 reduces to plain symmetric diffusion); fixed-point convergence under repeated ticks |
| **BH-1** | T1 | Border readout: optional output column carrying `1 − mean(C̄_edge)` (choke intensity) per cell, opt-in; Layer-2 reduce + threshold consumption proof (a "frontline formed" crossing) | Borders consumed through the existing reduce → EML → threshold path on a synthetic crowded fixture; default-off; no border objects |
| **BH-2** | deferred (named-consumer gate) | `(1 − C_u)` choke column as PALMA min-plus impedance feedstock `W` — the gradient-valley pathfinder coupling | Opens only when a movement consumer names it; no pathfinding engine, no route objects — D stays a field |
| **BH-3** | deferred (consumer-pulled) | ClauseThing authoring surface (`region_field` dialect gains the operator + caps) | Opens with the first ClauseScript-authored consumer |

BH-0 is frontier-gated (stability/parity judgment); BH-1 is mechanical once BH-0 lands.

## 4. Guardrails (restated, binding)

No semantic WGSL (the kernel sees `u`, `u_sat`, `χ`, `Δt` — floats). No border
service/object/graph; borders are field readouts. `simthing-sim` stays map-free and
BH-blind. Opt-in, default-off; spec presence enables nothing. CPU readback is oracle/diagnostic
only. No paper-fidelity claims (see §1.3). PALMA untouched until BH-2 opens with a named
consumer. No workspace test runs; fixtures are original and synthetic.

## 5. Status ledger

| Rung | Status | Report |
|---|---|---|
| BH-0 saturating-flux operator | NOT STARTED | — |
| BH-1 border readout + consumption | NOT STARTED | — |
| BH-2 PALMA impedance coupling | DEFERRED (named-consumer gate) | — |
| BH-3 ClauseThing authoring | DEFERRED (consumer-pulled) | — |

*Opened 2026-06-12 by product direction on the executive design authority's adjudication of the
C_u proposition digest. The hack is seated with pinned math so no future pass re-derives it
loosely; implementation begins at BH-0 in a fresh window.*
