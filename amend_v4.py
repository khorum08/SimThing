p = "docs/design_0_0_8_7_rf_arena_modernization.md"
s = open(p, encoding="utf-8").read()
lines = s.split("\n")

# ---- 7.6: three archaeology enumerations (undispatched -> amend directly) ----
a = "line; every route classified with a disposition; checker `--check` PASS in CI and `--harvest` re-derives; zero unclassified residue; a planted unlisted route REDs the checker. | Std — Grok | TODO |"
assert s.count(a) == 1, "7.6"
s = s.replace(a, a.replace(
    "a planted unlisted route REDs the checker.",
    "a planted unlisted route REDs the checker. **UNIFICATION-SWEEP ADDITION (Sol/Fable v4, 2026-08-13; 7.6 undispatched at amendment):** the census ALSO enumerates and classifies (a) every ancestor-projection/inheritance implementation (`resolve_owner`-class walks, `inherit_active_overlays`, tier/capability grant propagation), (b) every crossing→state-write/consequence path (7.3 subordinate activation, lifecycle transitions, actuation writes), and (c) every EML program registry/cache/dispatch-grouping surface — giving 7.8a a measured universe before code lands."), 1)

# ---- 7.7: bindings d..h ----
a = "t from the production path by grep/type proof, retained only as referee; replay of a pre-extraction recording reproduces bit-exactly post-extraction. | DA-reserve"
assert s.count(a) == 1, "7.7"
add77 = (" **UNIFICATION BINDINGS (Sol/Fable v4, 2026-08-13):** (1) one canonical EML program identity/registry/library — no OverlayThing-local EML namespace, table, or evaluator; compiled-artifact cache keyed `(ProgramId, ArithSemanticsVersion, LoweringArm, ExecutionShape, BindingABI)` — semantic dedup universal, artifact reuse only where physically compatible, NO cross-instance CSE claim (planted overlay-local-table RED). "
"(2) crossing→consequence via the shared `CrossingConsequenceBinding` ABI (see 7.8). "
"(3) BINDS 7.2/7.3 Current/Next/swap discipline — no overlay-local epoch selector, per-row freshness bit, or second swap authority; measure carry at admitted cardinalities before ANY compaction (7.3's evidence forbids speculative active-list machinery). "
"(4) **Deadline Authority Law:** fixed-duration lifecycle is an admitted `deadline_generation = g_activation + duration` **denominated in the owning tree's generation authority** (the 6.1 stamp source), compared not decremented — no per-generation countdown write, no OverlayThing-local clock, no wall-clock cadence, no foreign-tree counter; overflow at construction FAILS CLOSED, never wraps; expiry fires through the ordinary Phase-5 crossing; forced-generation-lag witness (6.3 shape) with a global-clock mutant RED. "
"(5) OverlayThing template/instance residency reuses the session-frozen sparse facility residency/capacity accounting principles (6.5/ActionBand precedent) — distinct typed layout permitted, but no dynamic-capacity manager, no mid-session semantic-template mint (planted RED), no generic everything-table. "
"(6) CPU-visible semantic/persistence/structural egress rides the existing stamped ring/envelope with typed `OverlaySemanticDelta` payloads ONLY when CPU visibility is actually required — routine numerical transitions produce no CPU delta. "
"(7) durable binding targets = logical identity + PropertyId + role/binding identity; physical rows exist only in upload/epoch binding state (planted durable-row-capture RED). "
"(8) **Routed Lifecycle Epoch Law** (R1×R2 corollary): a duration-based lifecycle crossing a receive boundary transports its authored duration + provenance stamp, NEVER a foreign absolute deadline; the destination establishes `deadline_generation` against its OWN generation authority at residency — planted foreign-absolute-deadline mutant RED under forced generation skew.")
s = s.replace(a, a.split(" | DA-reserve")[0] + add77 + " | DA-reserve", 1)

# ---- 7.8: ABI + bindings ----
a = " grep proof; 7.5c parity retained bit-identical after migration to the door; an unbounded positive-feedback emission REDs bounded-feedback admission. | DA-reserve"
assert s.count(a) == 1, "7.8"
add78 = (" **UNIFICATION BINDINGS (Sol/Fable v4, 2026-08-13):** (1) `CrossingConsequenceBinding` is the ONE crossing→consequence substrate, three admitted arms: **ResidentNextWrite** (destination MUST belong to the facility-local resident plane already bound to this consequence — ActionBandStateNext, OverlayStateNext, admitted parameter/state lane Next; a direct foreign resident-plane write is a planted RED), **RoutedOverlayDelivery** (origin + target + admitted routing/filtering → destination receive ingress; cross-tree via the stamped seam; positive witness proves origin attribution, filter traversal, generation pacing, and ordinary receive-leg arrival — no hidden global route), and **StructuralAuthorization** (the existing sealed `BoundaryRequest` egress — an authorization, never a GPU state-plane write). Neither OverlayThing nor ActionBand may mint another post-crossing dispatcher (planted second-dispatcher RED). 7.3's landed subordinate-activation path is the REFERENCE INSTANCE of this shape — left untouched unless consolidation genuinely requires it, and then only oracle-first with the superseded dispatcher deleted. "
"(2) shares 7.7's canonical EML program identity/registry and logical-identity binding fence. "
"(3) shares the Routed Lifecycle Epoch Law (7.7 binding 8). "
"(4) **CostBand quotient audit (engineering):** if any production path feeding GPU actuation still resolves `N = floor(V/C)` numerically on CPU, absorb ONLY that true authority leak into the GPU CostBand resolver here — CPU oracle/tooling implementations are retained, and 7.8 is not widened for paths off the actuation route.")
s = s.replace(a, a.split(" | DA-reserve")[0] + add78 + " | DA-reserve", 1)

# ---- 7.9: telemetry replacement ----
a = "tive mutant RED; leaf-stamping mutant RED (equivalent per-leaf instances rejected); cross-tree bypass attempt RED; replay bit-exact from existing history surfaces alone. | Frontier"
assert s.count(a) == 1, "7.9"
add79 = (" **TELEMETRY BINDING (Sol/Fable v4, 2026-08-13 — replaces the withdrawn ordinary-census-column proposal):** overlay transition observability is WRITE-ONLY relative to the simulation — sparse opt-in transition counters piggybacking the existing transition execution, aggregated through the existing observation/snapshot lifecycle to LeWM/Studio; no extra dispatch, no per-template hot state at fine cardinality by default; a simulation-readable active-overlay count exists ONLY as an explicitly authored observation Property through the normal admission door — fine telemetry silently becoming sim-readable is a planted RED.")
s = s.replace(a, a.split(" | Frontier")[0] + add79 + " | Frontier", 1)

# ---- 7.8a: new rung between 7.8 and 7.9 ----
lines = s.split("\n")
i78 = [i for i, l in enumerate(lines) if l.startswith("| 7.8 |")]
assert len(i78) == 1, "7.8 row line"
row = ("| 7.8a | `DERIVED-SPAN-PROJECTION-INVALIDATION-0` | **Unification sweep, the ONE new architectural rung (Sol/Fable v4, 2026-08-13) — deliberately BEFORE 7.9 so the fractal witness consumes this substrate instead of minting an overlay-local lowering.** One generic PHYSICAL subtree-span substrate over 6.4 logical identity + 6.5 range vocabulary: **(a) downward effective projection via effective-profile interning** — a homogeneous subtree resolves to span → `EffectiveProfile` id (already-composed parameters/program identities); local divergence narrows child spans; dense per-row materialization is a DERIVED CACHE, never semantic ownership, built only where a kernel measurably profits; **(b) downward dirty-span propagation + epoch invalidation**; **(c) source-blind `DerivedDependencyIndex`** — frozen per-session admission metadata mapping AUTHORITATIVE CHANGED LOCI (logical identity + PropertyId + role + optional binding/profile id) → dependent effective spans / STEAD / PALMA / Gu-Yang registrations / derived work, with invalidation keyed by locus NEVER by writer subsystem (a `match change_source` branch is a planted RED); never a runtime-mutable registry or service (planted RED). **Consumers keep their distinct semantic combine laws — no universal value resolver:** overlay inheritance (7.9) is the first GPU consumer; owner effective-view batch resolution adopts the substrate only where useful, `resolve_owner()` RETAINED as the query/oracle surface; capability/tier admission projection binds where the shape genuinely applies; **no owner GPU identity plane before 9.2's interning** (`OwnerRef(String)` stands). **StemThing-B is forward-bound to the span/invalidation substrate, not to any consumer's representation.** | Million-row homogeneous subtree resolves to **O(distinct-profiles) effective descriptors, never O(descendants)** semantic instances or ancestor walks; ancestor change → small profile/span metadata update + PRECISE invalidation (exactly the dependent spans dirty, no global change scan); where a dense materializer is used, a witness proves it is derived cache (deleting it changes performance, never semantics); planted `O(depth × descendants)` per-generation rewalk mutant RED; planted writer-subsystem invalidation branch RED; planted runtime-mutable registry RED. | DA-reserve · Frontier — Codex/Fable | TODO |")
lines.insert(i78[0] + 1, row)
s = "\n".join(lines)

open(p, "w", encoding="utf-8", newline="\n").write(s)
print("v4 amendment applied: 7.6+, 7.7+8 bindings, 7.8+4 bindings, 7.8a minted, 7.9 telemetry")
