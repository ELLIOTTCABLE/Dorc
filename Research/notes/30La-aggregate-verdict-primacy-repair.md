# 30La - Aggregate verdict primacy is a correctness repair

> Tier: focused handoff from the 2026-08-20 design sidequest. `30L` remains the
> elision-region plan; this note records the narrower defect that must be repaired before
> that plan consumes aggregate evidence. Confidence: `+SURE` means directly read or
> exercised in the current tree; `~SUSPECT` means implementation direction left for the
> repair builder.

## 0. Ruling and schedule

`rul-predict-measured-aggregates-are-a-bug` [HUMAN-TYPED 2026-08-20] - the member-loop
and inline-call lanes measuring through `__predict` while licensing through
`__is_converged` is model incorrectness, not deliberately forfeited value. The old
`FORFEITS:forfeit-member-lanes-predict-measured` row is removed. A reached vouch for one
body does not authorize a host answer produced by another body.

+SURE This repair is the next conductor's narrow correctness rider, before
`30I:step-5b-build-bundle-projection` and step 6. It is independent of bundle mechanics,
but the new executable projections and real locator consumer should be built against the
final measuring-body semantics. `30L` may consume the repaired primitive; it must not
build route proofs over the interim. `30I` steps 7/8 remain later still.

## 1. Findings

+SURE Standalone mutation sites obey `28Q:rul-verdict-primacy-at-the-ship-seat`:
`compile_probe` asks `ship_auto`, emits the reached `__is_converged` body with
`verdict: true`, and retains the predict-derived cell as static topology
(`307:rul-primacy-moves-the-body-never-the-cell`).

+SURE Member classification explicitly prevents the same answer.
`analysis/src/effect.rs::member_family` passes an empty `VerdictIndex` as
`no_verdict_lane_in_members` and discards the verdict-measurement out-parameter. The
resulting member family contains predict-derived facts and no record of which verdict
body should measure them.

+SURE `plan/src/lib.rs::push_member_predicts` and `push_inline_predicts` accept only
`ship_body`; both emit per-member `ProbePredict` records with `verdict: false`. They
therefore execute `__predict` even though these aggregate sites can later mint a mutation
replacement.

+SURE `build_vouches_from_sets` separately expands `EstablishMembers` and `InlineCall`
into per-establish candidates and mints reached `ByVouch<VerdictVouch>` values from
`__is_converged`. `AllEstablishesVouched` correctly checks the exact ordered
`(site, fact)` population, but it does not bind those vouches to the body that produced
the host observations. Its exactness does not close this defect.

+SURE The resulting composition can under-execute. If `__predict` answers converged and
the reached `__is_converged` body would answer diverged, the current aggregate receives
the former host answer and the latter body's authorial permission. With the remaining
aggregate gates satisfied, that answer can replace the whole loop body or call.

+SURE The repair must preserve the existing separation: predict-derived description
continues to own gen/kill, backing, invalidation, and the site's static cell; the verdict
body owns the live convergence measurement and its vouch. Runtime verdict status must
not be converted back into topology (`rul-rc-reaches-genkill-only-through-decisions`).

## 2. Product-facing acceptance

Two XFAIL round-trip cases land with this report's precursor commit:

- `aggregate30-member-verdict-primacy`
- `aggregate30-inline-verdict-primacy`

+SURE Both cases use distinct live family members. The predict body calls an inert
`dpkg-query` mock returning 0; the verdict body calls an inert `aptcheck` mock returning
1. The target records are therefore `effect=absent rc=1` for both aggregate members.

+SURE At the defect tip both cases fail gate 1 in the intended way:

```text
authored:
site 0.0 effect=absent rc=1
site 0.1 effect=absent rc=1
produced:
site 0.0 effect=holds rc=0
site 0.1 effect=holds rc=0
```

+SURE The XFAIL lens keeps the suite green while preserving that target. Removing the
markers today makes both cases red on the probe-record mismatch. A correct repair makes
the existing fixtures XPASS without changing `probe-results.txt`, `expected.ran`, or
`head-expected.ran`: the verdict bodies emit the authored absent records and the apply
runs both mutations.

+SURE Promotion means removing each `XFAIL` and `head-expected.ran`, minting the target
transcript in `expected.out`, and retaining the cases as ordinary round-trip regressions.
Do not re-author the records to `holds`; that would encode the bug as expected behavior.

## 3. Repair acceptance criteria

The implementation is accepted when all of the following hold:

1. `accept-verdict-body-measures-each-member` - every vouched mutation establish in a
   member-loop or inline-call aggregate ships its reached `__is_converged` body; a
   prediction body never supplies that establish's convergence observation.
2. `accept-record-vouch-identity-aligns` - aggregate establishes, probe sub-records, and
   vouches are exact ordered populations. Missing, extra, duplicate, reordered,
   declined, unshippable, wrong-site, or wrong-fact material rejects the whole aggregate.
3. `accept-no-partial-aggregate-probe` - one member unable to ship its verdict commits no
   partial establish-record set capable of shifting a later observation onto another fact.
4. `accept-predict-topology-does-not-move` - changing the measuring body does not change
   predict-derived cells, gen/kill, backings, invalidators, freshness subjects, or why
   coordinates except for the truthful measuring-body locus.
5. `accept-query-substitution-stays-separate` - query-only inline sites continue to use
   probe-sourced read substitution and never manufacture a mutation vouch.
6. `accept-product-xfails-promote` - both product-facing cases above XPASS for the intended
   reason and become ordinary passing cases.
7. `accept-standalone-primacy-stays-green` - the existing standalone split-family verdict
   primacy cases remain unchanged.

## 4. Builder latitude and boundaries

+SURE The repair builder owns the internal representation, function signatures, and whether
the per-member verdict lane is carried beside facts or reconstructed from one shared typed
aggregate account. No type name or helper decomposition is ruled here.

+SURE Two semantic questions must be answered in code, not bypassed: which existing static
cell each verdict-produced member record keys, and how that identity remains exactly aligned
with `AllEstablishesVouched` and aggregate freshness. The standalone "move the body, never
the cell" ruling is the governing constraint, not permission to skip the aggregate proof.

+SURE This work does not require `30L`'s `ElisionRegion`, `RouteInstance`, universal route
meet, or loop propagation. Existing `site N.M` evidence identity and ordered aggregate
establishes are the floor. It also requires no bundle behavior, durable schema change,
new user spelling, or new diagnostic prose.

~SUSPECT The smallest implementation will extend aggregate classification with per-establish
verdict-measurement identity, pass the verdict ship seam into both aggregate probe builders,
and retain their existing stage-then-commit all-or-nothing shape. That is guidance, not a
prescribed patch; preserve the acceptance properties above if a cleaner representation appears.

## 5. Conductor close

+SURE Keep this as its own small serial slice before resuming `30I` 5b/6. Review the probe
artifact and product XFAIL promotion, not merely unit assertions: the defect is which authored
body crosses the controller-host boundary. Once folded, rewrite `30L` stage 0 as a satisfied
prerequisite and remove steering references that still describe predict-measured aggregates
as an accepted residue.
