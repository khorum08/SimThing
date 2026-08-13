# Truth-only supersession pass: correct ONLY sentences the tree has already falsified.
p = "docs/simthing_core_design.md"
s = open(p, encoding="utf-8").read()

# F1: "fixed vocabulary" -> closed, admission-grown
old = ("stack over the **fixed** `EvalEML` vocabulary and executes in the same unified kernel.")
assert s.count(old) == 1, "F1"
s = s.replace(old, ("stack over the **closed, admission-grown** `EvalEML` vocabulary and executes in the same unified\n"
    "  kernel *(corrected 2026-08-13: the vocabulary is closed but no longer frozen — exact primitives\n"
    "  `EXP` and `LN` were admitted at 5.11/5.12 through the `ExactPrimitiveAdmission` door, and the\n"
    "  field-sweep edge context added `TARGET_VALUE`/`NEIGHBOR_VALUE` under the EML growth law)*."), 1)

# F2: extension ladder gains the exact-primitive door
old = ("(2) if a\ngenuinely new *generic* primitive is unavoidable, register it only through `OpcodeRegistrationGate` →\n"
       "`AdmittedEvalEmlOpcode` / `AdmittedEvalEmlCombine` (closed vocab; bit-exact CPU-oracle parity) — never a\n"
       "raw semantic opcode; (3) a scenario-specific or semantic op is **never** admissible (type-rejected).")
assert s.count(old) == 1, "F2"
s = s.replace(old, ("(2) if a\n"
    "genuinely new *generic* primitive is unavoidable, register it through the gate that matches its class:\n"
    "ordinary combines via `OpcodeRegistrationGate` → `AdmittedEvalEmlOpcode` / `AdmittedEvalEmlCombine`\n"
    "(closed vocab; bit-exact CPU-oracle parity), and **exact transcendental-class primitives via\n"
    "`ExactPrimitiveAdmission`** — sealed `PrimitiveDomain`, algorithm-as-spec with append-only semantics,\n"
    "exhaustive 2^32 digest per certified toolchain, and the cost key against the primitive's own gadget\n"
    "baseline (the door 5.10 built; `EXP` and `LN` are its admitted precedents, with `POW`, stabilized\n"
    "`Logistic`/`SoftmaxWeight`, and the literal `eml(x,y)` landed as gadget-library entries) — never a\n"
    "raw semantic opcode; (3) a scenario-specific or semantic op is **never** admissible (type-rejected)."), 1)

# F3: JIT is no longer only a default-off escape hatch
old = ("- **The JIT shader compiler** (`ProductionKernelRegistryShell`, default-off) is the performance\n"
       "  escape hatch on the same principle:")
assert s.count(old) == 1, "F3"
s = s.replace(old, ("- **The JIT shader compiler** is production, not an escape hatch *(corrected 2026-08-13: the 5.7\n"
    "  postfix-IR-to-WGSL SSA JIT, cached by sealed resource class and complete program identity, is an\n"
    "  ordinary field-sweep execution form — its generated PALMA kernel measured FASTER than the bespoke\n"
    "  shader at worst case; `ProductionKernelRegistryShell` remains the exact-authority door)*. The\n"
    "  original principle stands:"), 1)

# F4/F5: P2 + Layer-1 kernel naming
old = "generic `StructuredFieldStencilOp` kernel with the same authored weights** — no per-cell bespoke"
assert s.count(old) == 1, "F4"
s = s.replace(old, "generic field-sweep interpreter/JIT with the same authored map/fold/post program and weights**\n  (the pre-5.5 `StructuredFieldStencilOp` is a retained test-only referee) — no per-cell bespoke", 1)

old = "  StructuredFieldStencilOp evolves cell field columns (threat, disruption,"
assert s.count(old) == 1, "F5"
s = s.replace(old, "  The generic field sweep (authored map/fold/post; interpreter or JIT) evolves cell field\n  columns (threat, disruption,", 1)

# F6: production-operators block
old = ("**Production operators — the realized rule (Gu-Yang flux) and the reach utility (PALMA).** Two seated,\n"
       "semantic-free GPU operators give the automaton its production form, each a generic\n"
       "`StructuredFieldStencilOp`-family utility, not a new primitive or a semantic engine:")
assert s.count(old) == 1, "F6a"
s = s.replace(old, ("**Production operators — the realized rule (Gu-Yang flux) and the reach utility (PALMA).**\n"
    "*(Corrected 2026-08-13 — execution form superseded by the FIELD-SWEEP remodel, 5.4–5.7: both operators\n"
    "are now **authored `FieldSweepRegistration` instances over the one generic EML map/fold/post IR** —\n"
    "interpreted or JIT-compiled, bit-exact either way — carrying sealed `FieldLawProof`,\n"
    "`CanonicalOrderProof`, and for conservative folds the `UndirectedSymmetryCertificate`; the pre-remodel\n"
    "bespoke shaders survive only as test-only parity referees pending their 10.1 retirement. Adjacency is a\n"
    "registration axis — weighted `GridOffsets` N4/N8/radius-r presets and `LinkGraph` — with per-node\n"
    "conductance/χ certificates on graphs; N4 below is the reference instance, not the law.)* Two seated,\n"
    "semantic-free field laws give the automaton its production form, neither a new primitive nor a semantic\n"
    "engine:"), 1)

old = "`D = W + min(N4 D)` is a *field*, not a route"
assert s.count(old) == 1, "F6b"
s = s.replace(old, "`D = W + min(N D)` over the admitted adjacency (N4 reference; N8/radius-r/`LinkGraph` per 5.6) is a *field*, not a route", 1)

open(p, "w", encoding="utf-8", newline="\n").write(s)
print("core design: 6 supersession corrections applied")

# --- 12.2 scope amendment ---
p2 = "docs/design_0_0_8_7_rf_arena_modernization.md"
s = open(p2, encoding="utf-8").read()
old = ("| 12.2 | `CORE-CANONIZATION-0` | Phase 12: object model (P0 Root Contract + cycle + EML-ISA + Triad Doors + overlay law) into "
       "`simthing_core_design.md`; HARNESS phases canonize nothing; track closeout follows.")
assert s.count(old) == 1, "12.2 head"
s = s.replace(old, ("| 12.2 | `CORE-CANONIZATION-0` | Phase 12: object model (P0 Root Contract + cycle + EML-ISA + Triad Doors + overlay law) into "
    "`simthing_core_design.md`; HARNESS phases canonize nothing; track closeout follows. "
    "**OWNER SCOPE (approved 2026-08-13) — the closed-kernel rewrite:** home CostBand, ActionBand, OverlayThing "
    "(the intrinsic overlay closure), StemThing-A+B, the full RF/STEAD/PALMA/Gu-Yang Field Triad, and complete EML "
    "(`EXP`/`LN`, `ExactPrimitiveAdmission`, the grown gadget library) as facilities of ONE stem-cell germ; "
    "**the germ becomes the §1-level anatomy** — the SimThing presented as a closed simulation kernel from which "
    "everything fractally recurses, with current §§2–8 reorganized as expressions of that anatomy, never a subsystem list. "
    "DELETE superseded §4.1/§6/§7.2.1/§8 prose rather than annotating it (the dated correction notes accumulated "
    "between 5.5 and this rung are consumed and removed here). Written ONLY from graduated fact — zero anticipatory text."), 1)
open(p2, "w", encoding="utf-8", newline="\n").write(s)
print("12.2 scope amendment applied")
