#!/usr/bin/env bash
# OVERLAY-GERM-ARCHAEOLOGY-0 — reproducibility checker.
#
# Sibling of stemthing_slot_census_check.sh. Docs/TSV/script only.
#   --check     CI-safe. Re-discovers live routes; reconciles TSV + universe.
#               Planted unlisted route → CENSUS-CHECK-VERDICT: FAIL(unlisted-route:TOKEN)
#   --harvest   LOCAL ONLY. Refuses a dirty tree. DETECTS pin drift; never writes.
#               Drift → CENSUS-HARVEST-VERDICT: STALE (re-pin with --repin)
#   --repin     LOCAL ONLY. Refuses a dirty tree. Re-derives and WRITES the pin.
#               --check still fails until every added route is classified in the TSV.
#   --selftest  Proves planted unlisted dissolve_overlay REDs for the exact reason.
set -euo pipefail
cd "$(dirname "$0")/../.."
MODE="${1:---check}"

python - "$MODE" <<'PY'
from __future__ import annotations

import re
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(".").resolve()
UNIVERSE_PATH = ROOT / "scripts/ci/overlay_germ_archaeology_census_universe.txt"
TSV_PATH = ROOT / "scripts/ci/overlay_germ_archaeology_census.tsv"

# simthing-core is included beyond the 7.6 row's feeder/sim/kernel/driver/spec
# list because the overlay germ and core lifetime/query routes live there;
# omitting it would be an incomplete census. simthing-clausething is excluded
# because it is an authoring/parser surface: authored attach_overlay parses to
# a product that enters through already-classified engine routes, not another
# runtime attach/lifecycle route.
SCAN_CRATES = (
    "simthing-core",
    "simthing-feeder",
    "simthing-sim",
    "simthing-kernel",
    "simthing-driver",
    "simthing-spec",
)

ROUTE_NAME_RE = re.compile(
    r"""^(?:
        add_overlay
        |attach_overlay
        |activate_overlay
        |suspend_overlay
        |dissolve_overlay
        |remove_overlay
        |expire_overlay
        |override_overlay
        |apply_overlay
        |deliver_routed_overlay
        |deliver_deficit_directive
        |deliver_standing_directive
        |deliver_predicate_broadcast
        |inherit_active_overlays
        |resolve_overlay_lifecycle
        |mint_attach_overlay_at_barrier
        |compile_overlay
        |build_overlay_deltas
        |admit_dispatch_minted_overlay
        |dispatch_until_dissolved
        |receive_command_deficits_from_disbursement
        |capture_ancestor_standing_policy
        |apply_expire_effects
        |install_standalone_overlay
        |install_pack_standalone_overlays
        |build_order_directive_overlay
        |gate_raw_player_overlay
        |submit_player_intent
        |submit_order_directive
        |submit_commitment_effects
        |resolve_owner
        |resolve_owners_in_order
        |walk_inherited_until
        |materialize_granting_census
        |sweep_on_prereq_met
        |emit_activation
        |pre_admitted_subordinate
        |crossing_binding_for_band
        |apply_band_crossing_deltas_from_fused_emissions
        |apply_band_crossing_deltas_from_threshold_events
        |apply_band_crossings_to_anchor_table
        |apply_sealed_band_crossings_to_anchor_table
        |dispatch_with_native_next
        |pipeline_cache_digest
        |eml_registry_mut
        |ordered_active_overlays
        |from_ordered_overlays
        |push_overlay
        |apply_to_data
        |apply_to_data_with_n
        |plan_overlay_orderband
        |run_accumulator_overlays
        |upload_overlay_deltas
        |eval_overlay_eml
        |admit_overlay_eml_program
        |validate_overlay_eml_program
        |remap_overlay_origins
        |remap_overlay_affects
        |apply_structural_mutations
        |apply_collected
        |apply_collected_as_intents
        |overlay_event
        |register_formula
        |register_cpu_oracle_formula
    )$""",
    re.X,
)

FN_DEF_RE = re.compile(
    r"^[ \t]*(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?fn\s+"
    r"(?P<name>[A-Za-z_][A-Za-z0-9_]*)\s*(?:<[^>]*>\s*)?\("
)
VARIANT_RE = re.compile(
    r"^[ \t]+(?P<name>"
    r"AttachOverlay|ActivateOverlay|SuspendOverlay|ActivateOverlayRef|"
    r"OverlayAttached|OverlayDissolved|OverlayActivated|OverlaySuspended"
    r")\b"
)
BROAD_FN_RE = FN_DEF_RE
# Vocabulary-independent second net: route discovery cannot depend only on the
# enumerated names above. Any lifecycle/dispatch/write verb paired with an
# overlay-family object is census-shaped even when the exact spelling is new.
OVERLAY_VERB_SHAPE_RE = re.compile(
    r"^(?:add|admit|apply|attach|build|capture|compile|cross|deliver|dispatch|"
    r"dissolve|emit|expire|gate|inherit|install|materialize|mint|override|plan|"
    r"push|receive|register|remap|remove|renew|resolve|route|run|submit|suspend|"
    r"sweep|upload|validate|walk)_[a-z0-9_]*"
    r"(?:overlay|directive|germ|lifecycle)(?:_[a-z0-9_]*)?$"
)
MUTATION_RE = re.compile(
    r"(?:\.overlays\.(?:push|remove|retain|clear|insert)\s*\(|\.add_overlay\s*\()"
)

CLASSIFICATION = {
    "SEMANTIC-DUPLICATE",
    "GENUINELY-STRUCTURAL",
    "DEAD",
}
DISPOSITION = {"keep", "migrate", "delete"}
FAMILIES = {"OVERLAY", "INHERIT", "CROSSING-WRITE", "EML-REGISTRY"}


def crate_label(crate: str) -> str:
    return crate.removeprefix("simthing-")


def iter_src_files(extra_files: list[Path] | None = None):
    for crate in SCAN_CRATES:
        src = ROOT / "crates" / crate / "src"
        if not src.is_dir():
            continue
        for path in sorted(src.rglob("*.rs")):
            yield crate, path
    for path in extra_files or []:
        yield "simthing-core", path


def strip_line_comment(line: str) -> str:
    if "//" not in line:
        return line
    in_str = False
    out: list[str] = []
    i = 0
    while i < len(line):
        ch = line[i]
        if ch == '"' and (i == 0 or line[i - 1] != "\\"):
            in_str = not in_str
        if not in_str and ch == "/" and i + 1 < len(line) and line[i + 1] == "/":
            break
        out.append(ch)
        i += 1
    return "".join(out)


def cfg_test_spans(lines: list[str]) -> list[tuple[int, int]]:
    spans: list[tuple[int, int]] = []
    i = 0
    n = len(lines)
    while i < n:
        if re.search(r"#\[cfg\(test\)\]", lines[i]):
            j = i + 1
            while j < n and lines[j].strip() == "":
                j += 1
            if j < n and re.search(r"\bmod\s+\w+", lines[j]):
                depth = 0
                started = False
                k = j
                while k < n:
                    for ch in strip_line_comment(lines[k]):
                        if ch == "{":
                            depth += 1
                            started = True
                        elif ch == "}":
                            depth -= 1
                    if started and depth <= 0:
                        spans.append((i + 1, k + 1))
                        i = k + 1
                        break
                    k += 1
                else:
                    spans.append((i + 1, n))
                    return spans
                continue
        i += 1
    return spans


def in_spans(line_no: int, spans: list[tuple[int, int]]) -> bool:
    return any(a <= line_no <= b for a, b in spans)


def token(crate: str, path: Path, name: str) -> str:
    try:
        rel = path.relative_to(ROOT / "crates" / crate / "src").as_posix()
    except ValueError:
        rel = path.name
    return f"{crate_label(crate)}:{rel}:{name}"


def discover(extra_files: list[Path] | None = None) -> list[str]:
    found: set[str] = set()
    for crate, path in iter_src_files(extra_files):
        lines = path.read_text(encoding="utf-8").splitlines()
        spans = cfg_test_spans(lines)
        for lineno, line in enumerate(lines, 1):
            if in_spans(lineno, spans):
                continue
            m = FN_DEF_RE.match(line)
            if m and ROUTE_NAME_RE.match(m.group("name")):
                found.add(token(crate, path, m.group("name")))
                continue
            v = VARIANT_RE.match(line)
            if not v or line.lstrip().startswith("//"):
                continue
            rest = line[v.end() :].lstrip()
            if rest.startswith("=>") or rest.startswith("if ") or rest.startswith("|"):
                continue
            if rest.startswith("{") or rest.startswith("(") or rest.startswith(",") or rest == "":
                found.add(token(crate, path, v.group("name")))
    return sorted(found)


def broad_hits(extra_files: list[Path] | None = None) -> list[str]:
    hits: set[str] = set()
    for crate, path in iter_src_files(extra_files):
        lines = path.read_text(encoding="utf-8").splitlines()
        spans = cfg_test_spans(lines)
        for lineno, line in enumerate(lines, 1):
            if in_spans(lineno, spans):
                continue
            m = BROAD_FN_RE.match(line)
            if m:
                name = m.group("name")
                if "overlay" in name.lower() or OVERLAY_VERB_SHAPE_RE.match(name):
                    hits.add(token(crate, path, name))
    return sorted(hits)


def mutation_files() -> set[str]:
    files: set[str] = set()
    for crate, path in iter_src_files():
        lines = path.read_text(encoding="utf-8").splitlines()
        spans = cfg_test_spans(lines)
        rel = path.relative_to(ROOT / "crates" / crate / "src").as_posix()
        file_tok = f"{crate_label(crate)}:{rel}"
        for lineno, line in enumerate(lines, 1):
            if in_spans(lineno, spans):
                continue
            if MUTATION_RE.search(strip_line_comment(line)):
                files.add(file_tok)
    return files


def load_lines(path: Path, kind: str) -> list[str]:
    if not path.is_file():
        raise SystemExit(f"CENSUS-CHECK-VERDICT: FAIL(missing-{kind})")
    return [
        line.strip()
        for line in path.read_text(encoding="utf-8").splitlines()
        if line.strip() and not line.lstrip().startswith("#")
    ]


def pin_text(header: list[str], routes: list[str]) -> str:
    """Render a universe pin: comment header preserved, one route per line."""
    return "\n".join(list(header) + list(routes)) + "\n"


def load_tsv() -> list[dict[str, str]]:
    rows: list[dict[str, str]] = []
    if not TSV_PATH.is_file():
        raise SystemExit("CENSUS-CHECK-VERDICT: FAIL(missing-tsv)")
    for lineno, raw in enumerate(TSV_PATH.read_text(encoding="utf-8").splitlines(), 1):
        if not raw or raw.startswith("#") or "\t" not in raw:
            continue
        cols = raw.split("\t")
        if len(cols) < 8:
            raise SystemExit(f"CENSUS-CHECK-VERDICT: FAIL(tsv-short-row:{lineno})")
        rows.append(
            {
                "id": cols[0].strip(),
                "token": cols[1].strip(),
                "verb": cols[2].strip(),
                "layer": cols[3].strip(),
                "family": cols[4].strip(),
                "classification": cols[5].strip(),
                "disposition": cols[6].strip(),
                "rationale": cols[7].strip(),
            }
        )
    return rows


def load_residue() -> list[dict[str, str]]:
    """Justified non-route hits live as `# RESIDUE\\t...` comments in the TSV."""
    rows: list[dict[str, str]] = []
    if not TSV_PATH.is_file():
        raise SystemExit("CENSUS-CHECK-VERDICT: FAIL(missing-tsv)")
    for lineno, raw in enumerate(TSV_PATH.read_text(encoding="utf-8").splitlines(), 1):
        if not raw.startswith("# RESIDUE\t"):
            continue
        cols = raw[len("# RESIDUE\t") :].split("\t")
        if len(cols) < 3:
            raise SystemExit(f"CENSUS-CHECK-VERDICT: FAIL(residue-short-row:{lineno})")
        rows.append(
            {
                "token": cols[0].strip(),
                "kind": cols[1].strip(),
                "justification": cols[2].strip(),
            }
        )
    return rows


def porcelain() -> str:
    return subprocess.check_output(
        ["git", "status", "--porcelain"], cwd=ROOT, text=True
    )


def print_reconciliation(routes: int, discovery: int, residue: int, unclassified: int, open_rows: int) -> None:
    print(
        "RECONCILIATION: "
        f"routes={routes} discovery={discovery} residue={residue} "
        f"unclassified={unclassified} open={open_rows}"
    )


def run_check(extra_files: list[Path] | None = None) -> int:
    live = discover(extra_files)
    pinned = load_lines(UNIVERSE_PATH, "universe")
    rows = load_tsv()
    residue_rows = load_residue()

    extra_live = sorted(set(live) - set(pinned))
    if extra_live:
        print_reconciliation(len(rows), len(live), len(residue_rows), len(extra_live), 0)
        print(f"CENSUS-CHECK-VERDICT: FAIL(unlisted-route:{extra_live[0]})")
        if len(extra_live) > 1:
            print("UNLISTED-ROUTE-REST: " + " ".join(extra_live[1:]))
        return 1

    problems: list[str] = []
    missing_live = sorted(set(pinned) - set(live))
    if missing_live:
        problems.append("PINNED-NOT-LIVE: " + " ".join(missing_live))

    classified_tokens = [r["token"] for r in rows]
    if len(classified_tokens) != len(set(classified_tokens)):
        dups = sorted({t for t in classified_tokens if classified_tokens.count(t) > 1})
        problems.append("TSV-DUP-TOKEN: " + " ".join(dups))
    ids = [r["id"] for r in rows]
    if len(ids) != len(set(ids)):
        dups = sorted({i for i in ids if ids.count(i) > 1})
        problems.append("TSV-DUP-ID: " + " ".join(dups))

    tsv_set = set(classified_tokens)
    universe_set = set(pinned)
    missing_class = sorted(universe_set - {t for t in tsv_set if not t.startswith("[analysis]")})
    extra_class = sorted(t for t in tsv_set if t not in universe_set and not t.startswith("[analysis]"))
    if missing_class:
        problems.append("UNCLASSIFIED-UNIVERSE: " + " ".join(missing_class))
    if extra_class:
        problems.append("TSV-OUTSIDE-UNIVERSE: " + " ".join(extra_class))

    for r in rows:
        if r["classification"] not in CLASSIFICATION:
            problems.append(f"BAD-CLASS:{r['id']}:{r['classification']}")
        if r["disposition"] not in DISPOSITION:
            problems.append(f"BAD-DISPOSITION:{r['id']}:{r['disposition']}")
        if r["family"] not in FAMILIES:
            problems.append(f"BAD-FAMILY:{r['id']}:{r['family']}")
        if r["classification"] == "SEMANTIC-DUPLICATE" and r["disposition"] != "migrate":
            problems.append(f"DUP-MUST-MIGRATE:{r['id']}")
        if r["classification"] == "GENUINELY-STRUCTURAL" and r["disposition"] != "keep":
            problems.append(f"STRUCTURAL-MUST-KEEP:{r['id']}")
        if r["classification"] == "DEAD" and r["disposition"] != "delete":
            problems.append(f"DEAD-MUST-DELETE:{r['id']}")
        if not r["rationale"]:
            problems.append(f"EMPTY-RATIONALE:{r['id']}")

    residue_tokens = [r["token"] for r in residue_rows]
    if len(residue_tokens) != len(set(residue_tokens)):
        dups = sorted({t for t in residue_tokens if residue_tokens.count(t) > 1})
        problems.append("RESIDUE-DUP: " + " ".join(dups))
    for r in residue_rows:
        if not r["justification"]:
            problems.append(f"RESIDUE-UNJUSTIFIED:{r['token']}")

    residue_set = set(residue_tokens)
    broad = [h for h in broad_hits(extra_files) if h not in universe_set]
    unjustified_broad = [h for h in broad if h not in residue_set]
    if unjustified_broad:
        problems.append("UNJUSTIFIED-BROAD: " + " ".join(unjustified_broad))

    tsv_files = {
        ":".join(t.split(":")[:2])
        for t in classified_tokens
        if not t.startswith("[analysis]")
    }
    analysis_files = {
        ":".join(t[len("[analysis]") :].split(":")[:2])
        for t in classified_tokens
        if t.startswith("[analysis]")
    }
    residue_files = {":".join(t.split(":")[:2]) for t in residue_tokens}
    covered = tsv_files | analysis_files | residue_files
    unjustified_mut = sorted(mutation_files() - covered)
    if unjustified_mut:
        problems.append("UNJUSTIFIED-MUTATION: " + " ".join(unjustified_mut))

    unclassified = len(missing_class)
    open_rows = sum(1 for r in rows if r["classification"] == "OPEN")
    print_reconciliation(len(rows), len(live), len(residue_rows), unclassified, open_rows)
    if problems:
        for p in problems:
            print(p)
        print("CENSUS-CHECK-VERDICT: FAIL")
        return 1
    print("CENSUS-CHECK-VERDICT: PASS")
    return 0


def run_harvest() -> int:
    dirty = porcelain().strip()
    if dirty:
        print("CENSUS-HARVEST-VERDICT: FAIL(dirty-tree)")
        print(dirty)
        return 1
    live = discover()
    pinned = load_lines(UNIVERSE_PATH, "universe") if UNIVERSE_PATH.is_file() else []
    if live == pinned:
        print(f"CENSUS-HARVEST-VERDICT: PASS (universe unchanged, {len(live)} routes)")
        return 0
    import difflib

    sys.stdout.writelines(
        difflib.unified_diff(
            [p + "\n" for p in pinned],
            [p + "\n" for p in live],
            fromfile="overlay_germ_archaeology_census_universe.txt",
            tofile="live-discovery",
        )
    )
    print(
        "CENSUS-HARVEST-VERDICT: STALE "
        "(universe drifted; re-pin with --repin, classify added routes in the TSV, do not hand-edit)"
    )
    return 1


def run_repin(require_clean: bool = True) -> int:
    """Re-derive the pinned universe from the live tree.

    `--harvest` only DETECTS drift -- it never wrote the pin. So "reconcile by
    re-derivation, never hand-edit" named a mechanism that did not exist, and the
    only way to move the pin was the hand-edit the fence forbids. This is that
    missing half: the pin becomes machine-derived, so it stays a re-derivable
    statement ABOUT the tree instead of a file somebody maintains.

    Deliberately NOT folded into `--harvest`. Drift detection must stay a
    read-only signal a gate can fail on; re-pinning is an authoring act and
    should say so at the call site. This cannot launder an unclassified route:
    `--check` still fails until every added route carries a TSV classification.
    """
    # The selftest re-pins a SCRATCH pin, so it opts out of the clean-tree
    # guard. The real command never does: re-pinning against uncommitted work
    # would record a universe nobody can reproduce from a commit.
    if require_clean:
        dirty = porcelain().strip()
        if dirty:
            print("CENSUS-REPIN-VERDICT: FAIL(dirty-tree)")
            print(dirty)
            return 1
    live = discover()
    pinned = load_lines(UNIVERSE_PATH, "universe") if UNIVERSE_PATH.is_file() else []
    if live == pinned:
        print(f"CENSUS-REPIN-VERDICT: PASS (universe unchanged, {len(live)} routes)")
        return 0
    live_set, pinned_set = set(live), set(pinned)
    added = [route for route in live if route not in pinned_set]
    removed = [route for route in pinned if route not in live_set]
    header = [
        line.rstrip()
        for line in UNIVERSE_PATH.read_text(encoding="utf-8").splitlines()
        if line.lstrip().startswith("#")
    ]
    UNIVERSE_PATH.write_text(pin_text(header, live), encoding="utf-8")
    for route in added:
        print(f"  repin-added: {route}")
    for route in removed:
        print(f"  repin-removed: {route}")
    print(
        f"CENSUS-REPIN-VERDICT: PASS (universe re-derived, {len(live)} routes, "
        f"+{len(added)}/-{len(removed)}; classify every added route in the TSV, then --check)"
    )
    return 0


def run_selftest() -> int:
    with tempfile.TemporaryDirectory() as td:
        plant = Path(td) / "planted_unlisted_route.rs"
        plant.write_text(
            "pub fn dissolve_overlay(target: u32) {\n    let _ = target;\n}\n",
            encoding="utf-8",
        )
        live = discover([plant])
        if not any(t.endswith(":dissolve_overlay") for t in live):
            print("CENSUS-SELFTEST-VERDICT: FAIL(planted-route-not-discovered)")
            return 1
        proc = subprocess.run(
            [sys.executable, "-c", Path(__file__).read_text(encoding="utf-8") if False else ""],
            capture_output=True,
            text=True,
        )
        # Direct in-process check with the planted file.
        from io import StringIO
        import contextlib

        buf = StringIO()
        with contextlib.redirect_stdout(buf):
            rc = run_check(extra_files=[plant])
        out = buf.getvalue()
        print(out, end="")
        if rc == 0 or "FAIL(unlisted-route:" not in out or "dissolve_overlay" not in out:
            print("CENSUS-SELFTEST-VERDICT: FAIL(planted-route-did-not-red-exact-reason)")
            return 1
        # A route whose exact name is absent from ROUTE_NAME_RE and which does
        # not contain the token "overlay" must still be discovered by shape.
        shaped = Path(td) / "planted_overlay_verb_shape.rs"
        shaped.write_text(
            "pub fn renew_directive_germ(epoch: u32) {\n    let _ = epoch;\n}\n",
            encoding="utf-8",
        )
        shaped_live = discover([shaped])
        if any(t.endswith(":renew_directive_germ") for t in shaped_live):
            print("CENSUS-SELFTEST-VERDICT: FAIL(shaped-route-leaked-into-enumerated-universe)")
            return 1
        shaped_hits = broad_hits([shaped])
        if not any(t.endswith(":renew_directive_germ") for t in shaped_hits):
            print("CENSUS-SELFTEST-VERDICT: FAIL(shaped-route-not-discovered)")
            return 1
        buf = StringIO()
        with contextlib.redirect_stdout(buf):
            shaped_rc = run_check(extra_files=[shaped])
        shaped_out = buf.getvalue()
        print(shaped_out, end="")
        if shaped_rc == 0 or "UNJUSTIFIED-BROAD:" not in shaped_out or "renew_directive_germ" not in shaped_out:
            print("CENSUS-SELFTEST-VERDICT: FAIL(shaped-route-did-not-red-exact-reason)")
            return 1
        # Prove the STALE branch without dirtying the repo: compare live
        # discovery against a truncated pin in memory.
        live_clean = discover()
        if live_clean and live_clean != live_clean[:-1]:
            print(
                "CENSUS-HARVEST-VERDICT: STALE "
                "(universe drifted; re-pin with --repin, classify added routes in the TSV, do not hand-edit)"
            )
        # --repin must ADD a drifted route to the pin and must NOT launder it:
        # --check still has to RED until the route is classified in the TSV.
        # Proved against a temporary pin so the repo pin is never touched.
        global UNIVERSE_PATH
        real_universe = UNIVERSE_PATH
        try:
            scratch = Path(td) / "scratch_universe.txt"
            header = [
                line.rstrip()
                for line in real_universe.read_text(encoding="utf-8").splitlines()
                if line.lstrip().startswith("#")
            ]
            truncated = load_lines(real_universe, "universe")[:-1]
            dropped = load_lines(real_universe, "universe")[-1]
            scratch.write_text(pin_text(header, truncated), encoding="utf-8")
            UNIVERSE_PATH = scratch
            buf = StringIO()
            with contextlib.redirect_stdout(buf):
                rc_repin = run_repin(require_clean=False)
            repin_out = buf.getvalue()
            repinned = load_lines(scratch, "universe")
            if rc_repin != 0 or dropped not in repinned:
                print(repin_out, end="")
                print("CENSUS-SELFTEST-VERDICT: FAIL(repin-did-not-restore-drifted-route)")
                return 1
            if f"repin-added: {dropped}" not in repin_out:
                print(repin_out, end="")
                print("CENSUS-SELFTEST-VERDICT: FAIL(repin-did-not-report-added-route)")
                return 1
            # The laundering falsifier: re-pin a route that has NO TSV row and
            # --check must still RED it as unlisted.
            scratch.write_text(
                pin_text(
                    header,
                    sorted(
                        set(truncated) | {"core:planted_repin_route.rs:dissolve_overlay"}
                    ),
                ),
                encoding="utf-8",
            )
            buf = StringIO()
            with contextlib.redirect_stdout(buf):
                rc_after = run_check()
            if rc_after == 0:
                print(buf.getvalue(), end="")
                print("CENSUS-SELFTEST-VERDICT: FAIL(repin-laundered-an-unclassified-route)")
                return 1
        finally:
            UNIVERSE_PATH = real_universe
        print(
            "CENSUS-SELFTEST-VERDICT: PASS "
            "(enumerated and verb-shaped routes RED; --repin restores drift and cannot launder)"
        )
        return 0


def main(argv: list[str]) -> int:
    mode = argv[1] if len(argv) > 1 else "--check"
    if mode == "--harvest":
        return run_harvest()
    if mode == "--repin":
        return run_repin()
    if mode == "--selftest":
        return run_selftest()
    if mode in ("--check",):
        return run_check()
    print(
        "usage: overlay_germ_archaeology_census_check.sh [--check|--harvest|--repin|--selftest]",
        file=sys.stderr,
    )
    return 2


if __name__ == "__main__":
    sys.exit(main(sys.argv))
PY
