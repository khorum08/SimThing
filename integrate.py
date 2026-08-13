import re

# ---- 1. Core design: §6 adjudication note ----
p = "docs/simthing_core_design.md"
s = open(p, encoding="utf-8").read()
a1 = ("filters along the path. Until it graduates, the tuple above is the built shape. See\n"
      "[`stead_simthing_automata.md`](stead_simthing_automata.md).\n\n---\n\n## 7. Mapping")
assert s.count(a1) == 1, "sec6 tail"
note = ("filters along the path. Until it graduates, the tuple above is the built shape. See\n"
        "[`stead_simthing_automata.md`](stead_simthing_automata.md).\n\n"
        "**Intrinsic overlay closure (Owner-adjudicated 2026-08-10; Tier-2 addition).** The overlay is the\n"
        "**intrinsic actuation language of the StemThing**: ordinary numerical action resolves by\n"
        "activating, parameterizing, suspending, or dissolving an admitted overlay, and the base SimThing\n"
        "is the sole owner, emitter, and possessor of overlay lifetime — every disbursing overlay is\n"
        "attached to a SimThing up the tree. The closure forbids **peer** executors, never acting\n"
        "SimThings: anything that acts — a user seat, a network controller — is itself a SimThing,\n"
        "auditable as ordinary columns, API as admitted lanes, STEAD-bound for telemetry.\n"
        "[`stemthing_intrinsic_overlay_capability.md`](stemthing_intrinsic_overlay_capability.md) governs\n"
        "the actuation semantics (laws §18, probes §23, rungs 7.6–7.9); **full §6 canonization is bound to\n"
        "12.2 `CORE-CANONIZATION-0`** — this section's boundary-lifecycle sentences remain true until\n"
        "`GPU-OVERLAY-LIFECYCLE-EXTRACTION-0` graduates and are rewritten only then.\n\n---\n\n## 7. Mapping")
s = s.replace(a1, note, 1)

# ---- 2. Core design: §9 drift detector 14 ----
a2 = ("13. Am I minting a **StructuralCommitment** (or other decision effect) outside the sealed ingress\n"
      "    `ThresholdCrossingToken → EmissionToken → BoundaryEmissionToken`, or from a CPU/approximate\n"
      "    diagnostic?\n\n")
assert s.count(a2) == 1, "detector 13"
s = s.replace(a2, a2.rstrip("\n") + "\n14. Am I minting an **overlay manager, overlay service, `ActionThing`, second actuation path, or\n"
      "    per-leaf stamps of a subtree-scoped overlay** — instead of the intrinsic germ, the one\n"
      "    ancestor-resident instance, and the one actuation door\n"
      "    ([`stemthing_intrinsic_overlay_capability.md`](stemthing_intrinsic_overlay_capability.md))?\n\n", 1)
open(p, "w", encoding="utf-8", newline="\n").write(s)
print("core design: sec6 note + detector 14")

# ---- 3. agents.md: ONE line (189 -> 190, at cap) ----
p = "docs/agents.md"
s = open(p, encoding="utf-8").read()
a3 = "6. **MANDATORY for any spatial task** — [`stead_spatial_contract.md`](stead_spatial_contract.md)"
assert s.count(a3) == 1
s = s.replace(a3, "5b. **MANDATORY for any overlay/actuation task** — [`stemthing_intrinsic_overlay_capability.md`](stemthing_intrinsic_overlay_capability.md) (Owner-adjudicated closure: lifecycle, actuation door, ancestor residency, composition classes, designer-language lowering).\n" + a3, 1)
open(p, "w", encoding="utf-8", newline="\n").write(s)
print("agents.md: +1 line")

# ---- 4. scans.tsv: OVERLAY-PEER-AUTHORITY heuristic tripwire ----
p = "scripts/ci/scans.tsv"
s = open(p, encoding="utf-8").read()
if not s.endswith("\n"):
    s += "\n"
row = ("OVERLAY-PEER-AUTHORITY | HEURISTIC | crates/simthing-{core,kernel,gpu,sim,driver,spec,clausething,feeder,mapgenerator,mapeditor,tools}/src/** | "
       r"struct\s+(?:ActionThing|OverlayManager|OverlayService|SaturationListener|OverlayHistory|MovementPlanner)\b|enum\s+(?:ActionKind|OverlayCommandKind)\b|fn\s+\w*overlay_manager\w*\("
       " | compile_fail;^\\s*//!;^\\s*///;^\\s*// | stemthing_intrinsic_overlay_capability.md sections 17-19 + core design section 9 detector 14; HEURISTIC tripwire for peer overlay/action authority minted between the Owner adjudication and the 7.7/7.8 type landings | "
       "retire when 7.7 GPU-OVERLAY-LIFECYCLE-EXTRACTION-0 and 7.8 ACTIONBAND-OVERLAY-ACTUATION-0 make peer actuation authority uncompilable at the type boundary\n")
s += row
open(p, "w", encoding="utf-8", newline="\n").write(s)
print("scans.tsv: OVERLAY-PEER-AUTHORITY appended")

# ---- 5. known-bad fixture ----
open("scripts/ci/fixtures/known_bad/overlay_peer_authority.rs", "w", encoding="utf-8", newline="\n").write(
"""// Known-bad fixture: peer overlay/action authority beside the intrinsic StemThing germ.
pub struct OverlayManager {
    pub active: Vec<u32>,
}

impl OverlayManager {
    pub fn tick_lifecycle(&mut self) {
        self.active.retain(|v| *v > 0);
    }
}
""")
print("fixture written")

# ---- 6. selftest case ----
p = "scripts/ci/doctrine_selftest.sh"
s = open(p, encoding="utf-8").read()
a6 = '  case_run expect_constitution_reach_log_append'
assert s.count(a6) == 1, "selftest anchor"
s = s.replace(a6, '  case_run expect_heuristic_inspect "overlay_peer_authority" "OVERLAY-PEER-AUTHORITY" \\\n    setup_heuristic_kernel overlay_peer_authority.rs\n' + a6, 1)
open(p, "w", encoding="utf-8", newline="\n").write(s)
print("selftest case inserted")
