# 30Da - The current kernel's narrow prediction-contract rider

> Scope: handoff from `30D` to the conductor implementing `30K`. This is not a
> second prediction-contract plan. `30D` owns that later arc; `30K` owns the
> effective-world settlement details.

## The narrow rider

The new prediction contract is deliberately post-kernel work. DREP syntax,
channel overrides, `return 2` migration, runtime confirmation, wrapper emission,
and `30J` family qualification do not join the current conversion. Half-building
them would remove today's decline mechanism before its replacement exists and
expand an already-large red window.

The current work preserves three boundaries only:

1. A predict Status is an opaque exact value. No effective-reach or plan-fold
   code may apply the verdict-function `0 / 1 / >=2` partition to it; a claimed
   status 2 must remain representable end to end.
2. Per-channel admission stays behind one firewall. `30K`'s effective Query
   validity is one conjunct deciding whether Status may enter the fold, not the
   definition of channel coverage. Do not spread `valid Query => rc claimed`
   across the settlement; later `30D` work must be able to add DREP coverage at
   that one edge.
3. Prediction execution/transport integrity stays upstream of settlement. It
   does not become a `ReachingWalls` fact, semantic site-act, no-execution proof,
   freshness state, survival input, or Spine field. `30K` consumes an already
   admitted channel view and does not explain why a channel was withheld.

The as-built `SiteRecord.rc` plus `Predicted<Rc>` firewall already have the right
shape. Preserve them rather than adding speculative coverage types. If the `30K`
conversion touches that fold seam, a focused valid-Query `rc=2` preservation pin
is warranted; otherwise no prediction-contract implementation is owed now. See
`30K` sections 4-6 for the frozen-input, Query-validity, and settlement rules.
