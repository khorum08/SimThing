import re

LADDER = "docs/design_0_0_8_7_rf_arena_modernization.md"
s = open(LADDER, encoding="utf-8").read()
lines = s.split("\n")

# ---- 1. Stamp 7.5c ----
i75c = [i for i, l in enumerate(lines) if l.startswith("| 7.5c |")]
assert len(i75c) == 1, "7.5c row"
row = lines[i75c[0]]
assert row.rstrip().endswith("| TODO |"), "7.5c status cell"
lines[i75c[0]] = row[: row.rstrip().rfind("| TODO |")] + "| **DA-GRADUATED / merged #1738 @ 73b72e6c** (Owner-relayed DA ruling; stamp records machine truth per the merge ritual) |"

# ---- 2. Mint 7.6-7.9 after the 7.5c row ----
rows = [
"| 7.6 | `OVERLAY-GERM-ARCHAEOLOGY-0` | **Overlay closure, rung 1 of 4 — anchored by `docs/stemthing_intrinsic_overlay_capability.md` (Owner-adjudicated 2026-08-10; probes §23; laws §18). Probe A.** Enumerate EVERY overlay attach/activate/suspend/dissolve/apply/override/expire route across feeder, sim, kernel, driver, and spec (`work.rs`, `patcher.rs`, `overlay_lifecycle.rs`, `tree_mutation.rs`, `overlay_prep.rs`, `automaton_reception.rs`, `compile/overlay.rs` are the known starting set — the census derives the full set from the tree, never from this list). Classify each route: SEMANTIC-DUPLICATE (migrates to the intrinsic germ), GENUINELY-STRUCTURAL (stays boundary), or DEAD. Deliver the census as a reproducible TSV + checker script (sibling of `stemthing_slot_census_check.sh` — universe pinned, re-derivable, staleness reads STALE). Carries the anchor's keep/migrate/delete disposition obligation (promotion criterion 15). Docs/TSV/script only — zero engine diff. | Census TSV lands with reconciliation line; every route classified with a disposition; checker `--check` PASS in CI and `--harvest` re-derives; zero unclassified residue; a planted unlisted route REDs the checker. | Std — Grok | TODO |",
"| 7.7 | `GPU-OVERLAY-LIFECYCLE-EXTRACTION-0` | **Overlay closure, rung 2 of 4 — the dedicated extraction rung the anchor's §23.1 mandates; never hidden in a composition rung. Probe B.** The largest remaining peer numerical authority is deleted: `resolve_overlay_lifecycle()` is retained as the CPU ORACLE while GPU takes ownership of overlay active/current/next state, dissolve-condition evaluation via EXISTING Phase-5 threshold registrations (field-output-driven lifecycle — blockade dissolves on a Gu-Yang crossing, rationing on an RF balance crossing), and `AfterTicks` WITHOUT a CPU decrement loop. `DissolveCondition` vocabulary maps to EML/threshold registrations at admission. Indistinguishability is the referee: same admitted state + recorded schedule → bit-identical lifecycle decisions, oracle vs GPU. Only after equivalence is proven does the CPU evaluator retire from the production path (oracle survives as referee residue). One-history: `OverlayDissolved`-class replay rides the existing stamped crossing/schedule surfaces — no `OverlayHistory` mechanism. | Oracle/GPU bit-identical across the witness battery incl. a planted divergence RED; `AfterTicks` proven with zero CPU decrement; a planted second crossing detector REDs the single-surface scan; CPU evaluator absent from the production path by grep/type proof, retained only as referee; replay of a pre-extraction recording reproduces bit-exactly post-extraction. | DA-reserve · Frontier — Codex/Fable | TODO |",
"| 7.8 | `ACTIONBAND-OVERLAY-ACTUATION-0` | **Overlay closure, rung 3 of 4 — probes C+D fused: the actuation door.** An ActionBand crossing resolves to a fixed admitted `OverlayStateNext` activation/parameterization (no direct world mutation, no domain executor); consequences realize NEXT generation through native substrate (Generation-Paced Actuation Law). One overlay parameterizes STEAD source/falloff, PALMA `W`/terminal value, and Gu-Yang conductance/capacity inputs strictly INSIDE their certified registration envelopes (Field Certificate Envelope Law — runtime variation never mutates adjacency, canonical order, conservation/symmetry, or χ certificates); field-seeded recurrent emissions pass bounded-feedback admission (gain, not just rate). **The landed 7.5c movement vendor RE-ANCHORS as the door's first production consumer**: its actuation path migrates to the intrinsic door with bit-identical GPU results — the oracle-first promotion pattern, movement proving the door rather than the door destabilizing movement. | Crossing → `OverlayStateNext` → next-generation consequence witness green; a certificate-envelope mutant (out-of-envelope parameter or certificate mutation) hard-errors at admission with a planted RED; private-solver/throughput grep proof; 7.5c parity retained bit-identical after migration to the door; an unbounded positive-feedback emission REDs bounded-feedback admission. | DA-reserve · Frontier — Codex/Fable | TODO |",
"| 7.9 | `OVERLAY-FRACTAL-CLOSURE-WITNESS-0` | **Overlay closure, rung 4 of 4 — probes E+F plus the anchor's §23.2 ADVERSARIAL designer scenario (small and deliberately hard, never vocabulary breadth).** Ancestor residency at scale: one authored policy overlay at a high ancestor over a large synthetic subtree — zero leaf stamping (Ancestor Overlay Residency Law), zero per-generation `O(depth × descendants)` rewalk (Inheritance Hot-Path Law, instrumented not asserted); composition classes admission-explicit with a conjunctive-restriction-weakening mutant RED; cross-tree actuation crossing ONLY the stamped receive/product seam; one-history replay proof. The §23.2 scenario in full: ancestor-scoped standing modifier + descendant-local modifier + generated long key compiled to generic binding + gated/triggered + timed + has-modifier predicate + bounded subtree projection + RF/Field-Triad feedback + ONE lawful temporal cycle (Current→Next) + ONE illegal same-generation algebraic cycle that must REJECT at admission with a span (Temporal Feedback Law). | Ten-element scenario green end-to-end; the illegal cycle rejects at admission (planted acceptance REDs); rewalk instrumentation shows O(changed-span) not O(depth × descendants); conjunctive mutant RED; leaf-stamping mutant RED (equivalent per-leaf instances rejected); cross-tree bypass attempt RED; replay bit-exact from existing history surfaces alone. | Frontier — coder | TODO |",
]
for off, r in enumerate(rows, start=1):
    lines.insert(i75c[0] + off, r)

s = "\n".join(lines)

# ---- 3. Phase 7 pillar row note ----
old_p7 = "| 7 | **Movement-Front execution** (Owner-added 2026-07-20)"
assert s.count(old_p7) == 1, "phase 7 pillar"
s = s.replace(old_p7, "| 7 | **Movement-Front execution** (Owner-added 2026-07-20; 7.6-7.9 carry the Owner-adjudicated intrinsic-overlay semantic closure — anchor `stemthing_intrinsic_overlay_capability.md`; the §22.B field-solver performance program is explicitly NOT minted and remains workshop research)", 1)

open(LADDER, "w", encoding="utf-8", newline="\n").write(s)
print("ladder: stamp + 4 rungs + pillar note")

# ---- 4. Anchor rows ----
A = "scripts/ci/doctrine_anchors.tsv"
a = open(A, encoding="utf-8").read()
if not a.endswith("\n"):
    a += "\n"
doc = "docs/stemthing_intrinsic_overlay_capability.md"
for aid, sec, doms in [
    ("overlay-closure-thesis", "heading:## 0. Executive thesis", "core-0087,overlay-closure,actionband"),
    ("overlay-germ", "heading:## 6. Candidate intrinsic overlay germ", "sim,driver,feeder,overlay-closure"),
    ("overlay-scale-laws", "heading:## 15. Ancestor residency, inheritance, and million-child scale", "kernel,sim,overlay-closure"),
    ("overlay-designer-closure", "heading:## 16. Designer-language closure: complex modifiers, scopes, and feedback", "spec,clausething,overlay-closure"),
    ("overlay-promoted-laws", "heading:## 18. Candidate promoted semantic laws", "core-0087,overlay-closure"),
]:
    a += f"{aid}\t{doc}\t{sec}\t{doms}\t0\n"
open(A, "w", encoding="utf-8", newline="\n").write(a)
print("anchors: 5 rows (hashes via resync)")

# ---- 5. Trigger wiring: overlay-closure domain on overlay-bearing globs ----
T = "scripts/ci/anchor_triggers.tsv"
t = open(T, encoding="utf-8").read()
tl = t.split("\n")
wired = 0
for i, l in enumerate(tl):
    g = l.split("\t")[0] if "\t" in l else ""
    if g in ("crates/simthing-sim/**", "crates/simthing-feeder/**", "crates/simthing-driver/**",
             "crates/simthing-kernel/**", "crates/simthing-spec/**", "crates/simthing-core/**"):
        if "overlay-closure" not in l:
            tl[i] = l + ",overlay-closure"
            wired += 1
t = "\n".join(tl)
open(T, "w", encoding="utf-8", newline="\n").write(t)
print(f"triggers: overlay-closure wired onto {wired} globs")
