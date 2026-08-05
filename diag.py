s = open("docs/design_0_0_8_7_rf_arena_modernization.md", encoding="utf-8").read()

a1 = ("**DISCHARGES BINDS `OWNER-CHANNEL-INTRINSIC-0`** — the clauses above are the proof obligations "
      "those amendments create; this citation exists so the parity gate can see the proof was widened "
      "with the scope. | DA-reserve")
print("a1 count:", s.count(a1))

i = s.find("**DISCHARGES BINDS")
real = s[i:i + len(a1)]
for k, (x, y) in enumerate(zip(a1, real)):
    if x != y:
        print("diverge at", k)
        print("anchor:", repr(a1[max(0, k - 25):k + 5]))
        print("file:  ", repr(real[max(0, k - 25):k + 5]))
        print("codepoints:", hex(ord(x)), "vs", hex(ord(y)))
        break
else:
    print("no divergence in overlap; lengths", len(a1), len(real))
