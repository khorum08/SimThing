#!/usr/bin/env python3
"""DETACHABILITY-GATE-0 (Owner mandate 2026-07-30).

SimThing must exist without ClauseThing. The engine may never depend on the
authoring/app layer; the authoring layer depends on the engine, never the
reverse.

Two tiers, deliberately different in severity:

  PRODUCTION coupling -> hard FAIL. This is already true today (verified: no
  engine crate lists an authoring crate in [dependencies]), so the gate LOCKS
  IN a property the repo already has rather than demanding new work. Any future
  attempt to reach for hydration from the engine now fails here instead of at
  review.

  PROOF coupling (dev-dependencies) -> metered against a ceiling that may only
  decrease. The engine's *tests* still reach for the authoring layer to hydrate
  a scenario; that is the coupling the invariant set retires. Failing it today
  would red master, so it ratchets instead: fix, then ratchet.

Usage:
  python3 scripts/ci/detachability_check.sh
  python3 scripts/ci/detachability_check.sh --selftest
"""
import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parents[2]

ENGINE = [
    "simthing-core",
    "simthing-spec",
    "simthing-kernel",
    "simthing-sim",
    "simthing-gpu",
    "simthing-feeder",
    "simthing-driver",
]
AUTHORING = ["simthing-clausething", "simthing-mapeditor", "simthing-workshop"]

# Proof-coupling ceiling. MAY ONLY DECREASE. Each reduction is a rung that
# moved an engine proof onto the invariant set instead of a hydrated scenario.
DEV_COUPLING_CEILING = 0


def sections(manifest: str) -> dict[str, str]:
    out: dict[str, str] = {}
    current = ""
    for line in manifest.splitlines():
        m = re.match(r"^\[([^\]]+)\]\s*$", line.strip())
        if m:
            current = m.group(1)
            out.setdefault(current, "")
            continue
        if current:
            out[current] += line + "\n"
    return out


def scan(root: pathlib.Path) -> tuple[list[str], list[str]]:
    prod: list[str] = []
    dev: list[str] = []
    for crate in ENGINE:
        manifest = root / "crates" / crate / "Cargo.toml"
        if not manifest.exists():
            continue
        secs = sections(manifest.read_text(encoding="utf-8", errors="replace"))
        for name, body in secs.items():
            if not name.endswith("dependencies"):
                continue
            is_dev = name.startswith("dev-") or name.endswith(".dev-dependencies")
            for auth in AUTHORING:
                if re.search(rf"^\s*{re.escape(auth)}\s*=", body, re.M):
                    (dev if is_dev else prod).append(f"{crate} -> {auth} [{name}]")
    return prod, dev


def run(root: pathlib.Path, ceiling: int) -> int:
    prod, dev = scan(root)
    for row in prod:
        print(f"  - PRODUCTION coupling: {row}")
    for row in dev:
        print(f"  - proof coupling (metered): {row}")
    failed = False
    if prod:
        print(
            "  remedy: the engine may not link the authoring layer. Move the call "
            "behind a spec/data boundary the engine reads back."
        )
        failed = True
    if len(dev) > ceiling:
        print(
            f"  remedy: proof coupling {len(dev)} exceeds ceiling {ceiling}; the "
            "ceiling may only decrease. Prove the invariant over inline-constructed "
            "input instead of a hydrated scenario."
        )
        failed = True
    verdict = "FAIL" if failed else "PASS"
    print(
        f"DETACHABILITY-VERDICT: {verdict} production_coupling={len(prod)} "
        f"proof_coupling={len(dev)} ceiling={ceiling}"
    )
    return 1 if failed else 0


def selftest() -> int:
    import tempfile

    failures: list[str] = []
    with tempfile.TemporaryDirectory() as td:
        tmp = pathlib.Path(td)
        (tmp / "crates" / "simthing-core").mkdir(parents=True)
        (tmp / "crates" / "simthing-driver").mkdir(parents=True)

        # clean tree passes
        (tmp / "crates" / "simthing-core" / "Cargo.toml").write_text(
            "[dependencies]\nserde = \"1\"\n", encoding="utf-8"
        )
        (tmp / "crates" / "simthing-driver" / "Cargo.toml").write_text(
            "[dependencies]\nserde = \"1\"\n", encoding="utf-8"
        )
        if run(tmp, 1) != 0:
            failures.append("clean tree should PASS")

        # production coupling must FAIL
        (tmp / "crates" / "simthing-core" / "Cargo.toml").write_text(
            "[dependencies]\nsimthing-clausething = { path = \"../x\" }\n",
            encoding="utf-8",
        )
        if run(tmp, 1) == 0:
            failures.append("production coupling should FAIL")

        # dev coupling over ceiling must FAIL
        (tmp / "crates" / "simthing-core" / "Cargo.toml").write_text(
            "[dev-dependencies]\nsimthing-clausething = { path = \"../x\" }\n",
            encoding="utf-8",
        )
        (tmp / "crates" / "simthing-driver" / "Cargo.toml").write_text(
            "[dev-dependencies]\nsimthing-mapeditor = { path = \"../y\" }\n",
            encoding="utf-8",
        )
        if run(tmp, 1) == 0:
            failures.append("dev coupling over ceiling should FAIL")
        if run(tmp, 2) != 0:
            failures.append("dev coupling at ceiling should PASS")

    if failures:
        for f in failures:
            print(f"  - {f}")
        print(f"DETACHABILITY-SELFTEST: FAIL ({len(failures)})")
        return 1
    print("DETACHABILITY-SELFTEST: PASS (4 fixtures)")
    return 0


if __name__ == "__main__":
    sys.exit(selftest() if "--selftest" in sys.argv[1:] else run(ROOT, DEV_COUPLING_CEILING))
