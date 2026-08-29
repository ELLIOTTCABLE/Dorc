# 30Rg — whole-product diagnostic proof

> Tier: quarantined builder ledger for `30Re:sched-enable-whole-product-code-proof`. Current
> state, not chronology. Rewrite stale claims in place.

## lane

| | |
|---|---|
| Worktree | `.claude/worktrees/r30-receipt-loom-code` |
| Branch | `ai/r30-receipt-loom-code` |
| Base | `3734290f` (carries `30Rf`'s landed work) |
| Inherited red | `dorc-aid::diag_tidy::every_variant_has_exactly_one_catalog_entry` — `DurableReceiptAmbiguous` has no catalog row |

## the three refusals, measured

A whole-product case at `crates/aid/tests/durable-receipt-ambiguous.loom` declaring
`code: durable-receipt-ambiguous` + `run: round-trip` + `fixpoint: executed` is refused in exactly
three places. Measured, not predicted:

1. **e2e runner** — `unread frontmatter key 'code'`. `code` is `run_lane: false` in
   `dorc_loom::FRONTMATTER_KEYS`, and the e2e runner refuses against that projection.
   `when-fires` and `why` are `run_lane: false` on the same footing, and a `code:` case carries
   them.
2. **looms runner** — `hygiene: case: replay block 0 does not surface "durable-receipt-ambiguous"`.
   The runner already DEFERS the transcript for a `fixpoint: executed` case ("[deferred to e2e]"),
   but the same-slug hygiene gate still demands the in-process drive render the code. For a
   whole-product case the code fires at the REAL BINARY, which is the whole point.
3. **catalog** — `every_variant_has_exactly_one_catalog_entry`, the inherited red: no row, because
   `dorc-loom publish` mints rows from `code:` cases and this one cannot load.

## what already exists and is being reused, not rebuilt

- `discover_looms` (`cli/tests/support.rs`) walks EVERY `crates/*/tests/`, so a `run:` loom sited
  in `crates/aid/tests/` is executed by the e2e runner. That siting is required, not incidental:
  `catalog_defining_cases::is_case_owned` resolves `CARGO_MANIFEST_DIR/tests/<slug>.loom`, so an
  aid slug's canonical case must live in aid's own collection (`aid/CLAUDE.md cases-live-here`).
- `scan_diagnostics` (gate-3) already validates declared slugs against the generated catalog and
  asserts each FIRED on stderr at any severity. It is the proof mechanism; nothing new is needed.
- `drive_extra_replays` already drives blocks 1..N sequentially in the one materialized dir, which
  is the multi-invocation shape a store-state case needs.

## the mechanical link

`code:` on a whole-product case is written into the materialized case's `expected-diagnostics`, so
the SAME key that mints the catalog row is the one gate-3 asserts fired. Not a slug coincidence and
not a comment: the row and the assertion have one source, and a case that declares a code its run
does not emit is red.

## measured obstacles beyond the vocabulary flag

- `run_replay_block` sets `.stderr(Stdio::null())`, so a diagnostic emitted by blocks 1..N is
  invisible to every gate. The ambiguity fires on a `dorc why --last`, which cannot be block 0
  (block 0's command is mandated to be `round_trip_command`).
- `Harness::profile` is ONE `ProfileSandbox` shared by every trial in the run, and every
  plan/round-trip drive publishes a receipt into it. So `dorc why --last` in an e2e case sees every
  other case's documents: the cohort is ambiguous for reasons outside the case, the listing is not
  reproducible, and the store accumulates toward `LocalLimits::V1.store_entries` (4096) across a
  full suite. A case minted to demonstrate a capability must OBSERVE that capability
  (`30Nf:fnd-multipart-never-placed-anything-in-production`), so the case needs its own profile.

## test state

Red, deliberately: the three refusals above are the lane's opening evidence.

## commits

- opening red evidence; see git log.

## deviations, unresolved `tc-*`, candidate invariants

- none recorded yet.
