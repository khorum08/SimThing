# 0088 current-orientation pointer referee repair

PROBATION / RED proof-present / OPEN / UNMERGED.
Separate bounded repair dispatch Board 5634333792; HD-RECEIPT 70df16e1babd;
ORIENT-RECEIPT 28f56884d309; stamp 73e54d6b56b7266b;
inherited 48-anchor ACK Board 5612017867.
Base: 707b18a1f039a7bd44338ded0747121c25f0eebf (#2040).
PR #2041 remains the unchanged blocked certification record.

The existing Board/orientation property-admission referee reads the completed
0.0.8.7 design row for its expected pointer while Board renders the current
orientation pointer. The current committed test therefore expects `none` against
`0088-INGRESS-FIDELITY-0`. This commit preserves that RED before any assertion repair.

Only the explicitly consumed referee is renewed from the pen, retaining its
identity, original birth/class/verdict and DSU 0 -> 1, as demonstrated in #2041.
The named downstream-utility now includes this separately authorized repair;
this is the same first renewal, not a second survival. No other row is renewed.

The subsequent bounded repair must compare Board against the current orientation
pointer, preserve every live property-admission inventory assertion, and introduce
no hard-coded current rung, bypass or ignore. Exact RED/GREEN heads and checks will
be bound in the completed packet and PR/Board return.
