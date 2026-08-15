# Round 29 Ingress Closeout Handoff

## Critical

Any successor MUST stop before acting and reread 298, the 297 immediate and phase-three packet, the relevant builder invariants, and this handoff. Do not resume from this summary alone.

## Lineage

Merged into `ai/main` after `dfe8fdb5`. The source branch was
`ai/r29-resume-ingress-cli-live`.

## Implemented, Awaiting Review

The immediate implementation is merged. Strict live and replay admission, controller
scope, bounded whylog v2, fixture normalization, derivation closure, report-only evidence,
and hostsim byte-fault admission are complete.

The exact merged lineage passed the WSL workspace tests, 97/97 e2e cases, fmt, clippy
with warnings denied, cargo-deny, typos, and the loom fixpoint. Native shell-dependent
tests remain unavailable because `dash`/`sh` is absent from the Windows PATH.

The immediate unit is NOT YET accepted. Opaque review `29-reviewA` was attempted twice,
but both relays failed to return a narrow verdict. Its unread durable report is committed
at `Research/quarantine-DO-NOT-READ/29-reviewA-opaque-report.md`; a successor or the human
must recover the gate without exposing that report to an ordinary conductor.

> CORRECTED 2026-07-24 by the security conductor, who is outside the firewall and read
> the report directly. The gate did not fail - only its relay did. `29-reviewA` returned
> **ACK** over `49b66421..b6fde355`, with zero qualifying concerns and no new or revealed
> hidden invariant. Its two `~SUSPECT` residues (the deliberately narrow width-one
> attribution wrapper; legacy/raw surfaces surviving beside the new admission route) are
> recorded as fenced, not blocking. The four immediate ledger rows are ACCEPTED. Read the
> paragraph above as the historical state of a broken relay, not as an outstanding gate.

<!-- /* superceded in part by quarantine/306 (2026-08-15): "recorded as FENCED" overstates the
legacy/raw residue. `29A` §3's own wording is the accurate one — "remains reachable in
principle" — and the tree agrees with `29A`, not with this paragraph: `records::deframe` and
`LegacyPolicy::Tolerate` are `pub` in `dorc-plan` with no lexical gate, where
`sinv-production-fences` demands one for exactly this class. The three gates that exist
(`fixture_intake_is_unreachable_from_production`, `fixture_payloads_are_unreachable_from_production`,
`foreign_edge_constructor_is_fenced`) do not name them; the parser is UNCALLED, not fenced.
Phase-five legacy-parser fencing (`29A` §8 item 3) is therefore still owed, and a successor
reading "fenced" here would wrongly conclude there is nothing to schedule. */ -->
>
> Standing correction to the instruction below: **running an opaque accrual review over
> this material was a misfire, and no opaque review is necessary or wanted for round-29
> work.** The opaque reviewer exists to read NON-security work for security-critical
> findings; pointing it at a security round's own output inverts its purpose, and the
> relay attempt spent budget on a model unsuited to the material. This waiver is scoped
> to the security lane - the security conductor and its builders - and it is recorded
> here, in quarantine, precisely so that an ordinary out-quarantine conductor never reads
> it as license to skip the gate on their own work. Their obligation is unchanged.

## Next Decision

The gate is recovered and the four immediate ledger rows are accepted; phases four and
five remain unstarted and out of scope. Current status, the outstanding catch-up work,
and the specified repair path for the disabled report lane: `29A`.

Preserve separate branch `ai/r29-ingress` dirty evidence and unrelated `ai/r28-phase3-unit3-lock`; do not touch either. Phases 4 and 5 remain out of scope.
