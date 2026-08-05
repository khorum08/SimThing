import io, sys

LADDER = "docs/design_0_0_8_7_rf_arena_modernization.md"
DOC = "docs/EML_exp_ln_unification_expansion.md"

s = open(LADDER, encoding="utf-8").read()

# ---- Edit 1: 7.1 fence (insert before the lane cell) ----
a1 = ("**DISCHARGES BINDS `OWNER-CHANNEL-INTRINSIC-0`** — the clauses above are the proof obligations "
      "those amendments create; this citation exists so the parity gate can see the proof was widened "
      "with the scope. | DA-reserve")
# anchor appears in BOTH 7.1 and 7.2 rows — scope the edit to the 7.1 row line only
lines0 = s.split("\n")
i71 = [i for i, l in enumerate(lines0) if l.startswith("| 7.1 |")]
assert len(i71) == 1, "7.1 row line"
assert lines0[i71[0]].count(a1) == 1, "7.1 anchor within row"
fence = ("**DISCHARGES BINDS `OWNER-CHANNEL-INTRINSIC-0`** — the clauses above are the proof obligations "
         "those amendments create; this citation exists so the parity gate can see the proof was widened "
         "with the scope. **ACTIONBAND FENCE (Owner-committed adoption plan 2026-08-05; "
         "`EML_exp_ln_unification_expansion.md` §Adoption governs the determination):** movement is the "
         "CANONICAL SPATIAL WITNESS of a more general field-derived action composition under determination "
         "at 7.1a. Destination authority remains field/gradient-derived over admitted spatial topology; an "
         "authored waypoint or operator directive deforms/weights that potential through the existing "
         "overlay mechanism and NEVER becomes a privileged Destination/path object. 7.1 must NOT mint an "
         "`ActionBand` type, destination planner, predecessor/path structure, or generic action registry — "
         "7.1 remains a movement rung; its obligation is to not foreclose the general law. | DA-reserve")
lines0[i71[0]] = lines0[i71[0]].replace(a1, fence, 1)
s = "\n".join(lines0)

# ---- Edit 2: 8.2 amendment ----
a2 = "attrition (adversarial — this is what was called combat). NOT ALL CONTENTION IS ADVERSARIAL and the engine must not assume it is."
assert s.count(a2) == 1, "8.2 rule-list anchor"
r2 = ("and later-admitted numerical clearing laws. NOT ALL CONTENTION IS ADVERSARIAL and the engine must not "
      "assume it is. **ACTIONBAND AMENDMENT (Owner-committed 2026-08-05; determination at 7.1a):** the "
      "executed resolution surface is GENERIC CONSTRAINED CLEARING and may not require preclassification as "
      "'contention' or 'adversarial' — contention is the OVERSUBSCRIBED OBSERVATION of that surface; trivial "
      "and contested clearing are one path. **ATTRITION MOVES TO TEMPORAL PERSISTENCE** (what was called "
      "combat): unresolved claim U → ordinary observation/field input → EML persistence valuation → "
      "CostBand-funded consequence → later-generation state/claims — UNLESS a real same-generation consumer "
      "proves a distinct clearing law necessary. **`U != R`:** U (requested constrained quantity not granted) "
      "and CostBand R (granted value below another executable quantum) are distinct, independently observable "
      "quantities. **GENERATION PACING (non-negotiable):** no persistence consequence may synchronously "
      "re-enter the same resolution site — clear→persist→re-clear convergence inside one generation is "
      "forbidden; action chains propagate at GENERATION SPEED, and implementations must not collapse "
      "multi-stage satisfaction into an intra-generation solver merely to reduce latency.")
s = s.replace(a2, r2, 1)

a3 = "attrition is ONE authored resolution rule over owner channels, never a mechanism of its own."
assert s.count(a3) == 1, "8.2 binds-6.0 attrition anchor"
s = s.replace(a3, "attrition resolves over owner channels through the temporal-persistence path above, never a mechanism of its own.", 1)

a4 = "Attrition rows are this rung's contribution to"
assert s.count(a4) == 1, "8.2 allowlist anchor"
s = s.replace(a4, "Persistence-attrition rows are this rung's contribution to", 1)

a5 = "attrition and every other authored resolution rule expresses its outcome as overlays"
if s.count(a5) == 1:
    s = s.replace(a5, "persistence-funded attrition and every authored resolution rule expresses its outcome as overlays", 1)

# ---- Edit 3: mint 7.1a after the 7.1 row line ----
lines = s.split("\n")
idx = [i for i, l in enumerate(lines) if l.startswith("| 7.1 |")]
assert len(idx) == 1, "7.1 row line"
row71a = ("| 7.1a | `ACTIONBAND-COMPOSITION-PROBE-0` | **Phase 7 determination rung — Owner-committed adoption "
"plan 2026-08-05; `EML_exp_ln_unification_expansion.md` §Adoption governs.** Born-mortal workshop-leaf probe, "
"sequential AFTER 7.1 (one rung in flight; no cadence exception). **HYPOTHESIS UNDER TEST (Claim A — a "
"hypothesis, NOT standing law):** a resource-consuming determinate transition can be expressed through "
"potential → EML valuation → claim → clear → CostBand → consequence, while non-consuming evolution (STEAD "
"propagation, decay, integration, pure projection, observation, ordinary overlay application) remains outside. "
"**THREE non-spatial witnesses:** deficit/resource satisfaction; LinkGraph relational action; derivation/fission. "
"Movement is the FOURTH witness via the LANDED 7.1, and the probe's structural-comparison task reports which "
"stages are LITERALLY SHARED vs merely analogous vs special-seam against the real 7.1 implementation. "
"**Claim B falsifier (gradient authority) is MANDATORY:** every witness carries multiple competing attractors; "
"changing ONLY field/potential state changes chosen/weighted progress; changing ONLY overlay weighting changes "
"it again; no destination or action identity is edited anywhere. **FENCE:** no production `ActionBand` struct, "
"enum, registry, opcode, planner, or match arm may be created — the term exists in workshop/test names and "
"prose only. Livelock/starvation falsifiers are EXCLUDED here; they belong to the future `VECTOR-COSTBAND-PROBE-0`. "
"| **DISPOSITION RULE (two lawful outcomes):** (A) the common composition generalizes — witnesses share the "
"literal path with no domain branch and Claim B holds; ActionBand becomes eligible for DA canonization "
"(§22 texts go to ruling). (B) witnesses require materially different action semantics — ActionBand is "
"REJECTED as core law, workshop artifacts are reaped, and 7.1/8.2 retain only their independently-valid "
"amendments (gradient-derived movement; temporal attrition factorization). A probe that cannot conclude (B) "
"has failed regardless of green checks. Planted defects: a hard-coded destination and a bypassed-clearing "
"path must each red the referee. | Std — Grok | TODO |")
lines.insert(idx[0] + 1, row71a)
s = "\n".join(lines)
open(LADDER, "w", encoding="utf-8", newline="\n").write(s)
print("ladder edits applied")

# ---- Edit 4: adoption plan into the expansion doc ----
d = open(DOC, encoding="utf-8").read()
marker = "---\n\n## 0. Executive verdict"
assert d.count(marker) == 1, "doc insert anchor"
plan = """---

## COMMITTED ADOPTION PLAN AND SCHEDULE (Owner ruling 2026-08-05 — BINDING)

> **This plan is committed and landed in the 0.0.8.7 ladder. The ActionBand LAW itself remains under
> determination — committing to the test is not committing to the result.** Everything below this section
> retains workshop/review status.

**Landed now (one PR, this ruling):**

1. **7.1 fence** — movement is the canonical spatial witness; destination authority stays
   field/gradient-derived (waypoints are field-deforming overlays, never a privileged `Destination`);
   no `ActionBand` production type, planner, or path structure; 7.1 remains a movement rung.
2. **8.2 amendment** — the executed resolution surface is generic constrained clearing without
   contention/adversarial preclassification; attrition moves to CostBand-funded temporal persistence
   (unless a real same-generation consumer proves otherwise); `U != R` explicit; generation pacing law
   (no intra-generation clear→persist→re-clear; action chains propagate at generation speed).
3. **`7.1a ACTIONBAND-COMPOSITION-PROBE-0` minted** — born-mortal workshop determination rung.

**Schedule (one rung in flight; no cadence exception; no parked track):**

```text
7.1  movement — the spatial witness
  ↓
7.1a ACTIONBAND-COMPOSITION-PROBE-0 — three non-spatial witnesses
     (deficit satisfaction, LinkGraph relational action, derivation/fission)
     + structural comparison against the LANDED 7.1 implementation
  ↓
DA DETERMINATION — two lawful outcomes:
  (A) composition generalizes → §22 canonization texts go to DA ruling
  (B) rejected → workshop reaped; 7.1/8.2 keep only their
      independently-valid amendments
  ↓
7.2 → 8.1 → 8.2 (implements against the amended framing)
```

**Explicitly excluded from this commitment:** Vector CostBand and its livelock/starvation falsifier
(future `VECTOR-COSTBAND-PROBE-0`); the residency envelope; ALL performance/tile candidates (the
locality-first staircase stands; a separate track only if residual debt earns it); the §22 canonization
texts (await the determination).

**Provenance:** decorrelated Fable/Sol reviews; Sol's sequencing correction (sequential 7.1 → 7.1a beats
parallelism — the probe inspects the real movement implementation, not an imagined one) and redundancy
cuts (8.1 already conservation-only with seam holding accounts; 6.5 stamp obligations already
discharged) adopted in full.

---

## 0. Executive verdict"""
d = d.replace(marker, plan, 1)
open(DOC, "w", encoding="utf-8", newline="\n").write(d)
print("doc adoption plan inserted")
