"""THE one anchor-lifecycle grammar. Both anchor_check.sh and anchor_query.sh
import this module; neither may define a lifecycle regex locally (harness fix
session 2026-09-03: the grammar previously lived in two parsers and drifted —
until: landed in check but not query, stopping a coder flight)."""
import re

PENDING_RE = re.compile(r"^pending:([A-Z0-9][A-Z0-9-]*-[0-9]+)$")
UNTIL_RE = re.compile(r"^until:([A-Z0-9][A-Z0-9-]*-[0-9]+)$")


def lifecycle_is_valid(value: str) -> bool:
    return (
        value == "canonical"
        or bool(PENDING_RE.fullmatch(value))
        or bool(UNTIL_RE.fullmatch(value))
    )
