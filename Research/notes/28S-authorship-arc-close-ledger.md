# 28S — error-message-authorship arc: close-out + residue ledger

> Scope: the 2026-08-12→13 authorship arc (its own handoff mislabeled it "r29"; it is not a
> round — r29 proper is the quarantined lane, off-limits). Covers the catalog-prose authorship
> sitting (Sonnet-conducted, Fable-built), the loom-chafe repair lane, the prose-provenance
> ratchet, and the agent pre-commit flip. Chronology lives in git; this ledger carries the
> surviving state and the residue. Handoff scratch (`_tmp-r29-conductor-handoff.md`) extracted
> here and deleted.

## What landed (lane tips, pre-fold: authorship `fbd19350` · chafe `b118ef2c` · hk flip
## `1ec5779f` · ratchet fold `9e275f23`; all reachable from `ai/main`)

- **71 of 83 assigned catalog codes prose-authored** via the sanctioned loom
  compile/promote loop (message registers; help where a real remedy existed).
  Arrangement/chrome prose untouched — deliberate mid-arc scope-down (human-typed).
  Post-arc census (`mise run prose:census`): catalog 15 migrated / 93 slop / 0 human;
  arrangement 210 / 1 / 0.
- **rul-prose-provenance-tier** — one shared enum, `aid::prose::ProseTier<T>
  {Migrated, Slop, WrittenByHumanOnly}`, for BOTH prose registries (`arrangement`'s
  `Words`/`OwnedWords` deleted; absence is `Option`, one idiom project-wide).
  `HelpRegister` deliberately NOT folded — register-existence is a different axis
  (`289` §2u). Tier never renders; the retype changed zero output bytes (verified:
  zero loom/golden churn across the branch).
- **rul-provenance-mint-table** — a loom edit mints `Slop`, always, whoever drives.
  `dorc-loom promote --human` is the ONLY `WrittenByHumanOnly` mint, and it refuses
  under agent environment markers (one named const: `CLAUDECODE`,
  `CLAUDE_CODE_ENTRYPOINT`; `DORC_HUMAN_COMMIT=1` is the human-at-the-keyboard
  escape). Overwriting a human register without the flag: agent-marked env proceeds
  with an INFORMATIONAL note (no error vocabulary; states no action necessary);
  human-marked env refuses with the `--human`-or-`--slop` hint. `Migrated` is never
  re-minted. `compile` refuses the marking flags (it publishes nothing).
- **rul-human-census-hook-gated** — `.githooks/commit-msg` refuses an `AI`/`AIa`-labelled
  commit that GROWS the `WrittenByHumanOnly(` occurrence-count in either generated
  lock (staged-blob vs HEAD-blob count-compare — honest precisely because the locks
  are serializer-pinned, one-line-per-register generated files). Self-tested both
  directions in `internal-tooling`'s `hook_selftest` via throwaway git repos.
- **fnd-fixpoint-does-not-protect-tier** — the byte-identity lock gates do NOT catch a
  self-consistent hand-edit of a tier discriminant: the promote mirror seeds from the
  compiled-in lock and carries tier forward, so a forged variant is its own fixpoint.
  The hook count-gate and the census are the actual nets; never cite the fixpoint gate
  as tier protection. (Refutes the prior conductor's handoff §6 claim.)
- **Agent pre-commit flip** — `HK_SKIP_HOOK=pre-commit` retired (it was residue of the
  pre-2026-07-26 `HK=0` full kill-switch, narrowed but never removed). Agent sessions
  now run pre-commit check-only and stash-free: `HK_FIX=0` + `HK_STASH=none` in
  `.claude/settings.json`. Verified falsifiably 2026-08-13, including the load-bearing
  negative: check-only WITHOUT `stash=none` still runs the full git-stash cycle.
- **Chafe repair lane** (14 briefed items → 10 fixed): `test:looms` path filtering (closed a
  silent false-green: a filter matching zero trials exited 0) · shim materialization stages
  to temps, renames only complete sets · squat-lint `__lend_map`/`__enter` gap, plus a
  second unreported gap in the munge collision/charclass lints · `walk_lend_body` now
  verb-keyed and brace-expanding · `loom:compile` bare-invocation Windows stack overflow
  (fixture book ~300 deep; worker thread, 64 MB) · `envelope: invocation` seat declaration
  so I/O-error cases commit production framing · caret re-point for the netns-invariance
  refusal · 16 stale `when_fires` citations re-pointed · both prose-state steering bullets
  re-synced onto the typed story.

## Residue (open, each with an owner)

- **tc-dorc-sh-shell-resolution** (HUMAN ruling owed — it edits a law) — `dorc-sh`
  resolves `sh` by raw OS PATH; `one-shell-answer`'s seat lives in `internal-tooling`,
  a dev-dependency explicitly not product code. Exits: move the seat product-side ·
  rule `dorc-sh` exempt (conductor lean: it executes target-side, where PATH `sh` is
  correct by construction — spell it as a scope-carve on the law) · a second copy
  (forbidden).
- **tc-aia-label-blocks-human-prose** — the census hook refuses `(AIa …)` commits that
  grow human registers; a human massaging AI prose up to human tier must commit
  without the AI label. May bite the prose sprint; revisit at first bite rather than
  pre-engineering an escape.
- **tc-comment-budget-versus-doc-law** — a flat ≤20 net-new comment budget cannot hold
  for a task minting a public module under the doc-comment-every-public-item rule
  (both builder lanes overran with honest accounting). Future briefs: exempt required
  one-line doc-comments, or scale the budget with new public items.
- **tc-fixture-commit-bypasses-signing** — `hook_selftest`'s throwaway repos commit
  with `gpgsign=false` + a dead `hooksPath` (not project history; without it the
  selftest hangs on a passphrase or fires the developer's global hooks). Accepted at
  conduct tier; recorded for visibility next to the never-bypass-signing rule.
- **Routing/taxonomy debts** — `tolerates-unknown-dimension` surfaces on the lint rung
  only (the discard in `oracle::validate` is deliberate; plan-lane routing is a `27R`
  rung-ownership decision) · I/O invocation errors exit through the usage synopsis
  (`EXIT_USAGE`) — the `291` §5a invocation-error-taxonomy change is owed ·
  `lint-tool-failed-without-findings` / `lint-tool-output-unparsable` need a
  scripted external-tool loom vocabulary (deliberately unbuilt: a fake keyed off the
  case's own `code:` would derive the answer from itself) ·
  `marker-version-unrecognized`'s span `todo:` stands · `lint-tool-absent` renders a
  `(raw)` fidelity tag on a run-level note (UX oddity, untouched) ·
  `--accept-metadata` reaches through `mise run loom:promote -- --accept-metadata`
  but the usage spec declares only positional cases (undocumented, fragile).
- **Skipped-code ledger** (from the authorship sitting) — 6 foreign-passthrough-hole
  codes the loom surface cannot reach (a `ForeignBytes`-sealed hole is absent from
  `loom:vars`, so rewriting the sentence risks silently dropping the hole):
  `cli-file-unreadable` · `cli-shim-dir-unwritable` · `dorc-sh-script-unreadable` ·
  `dorc-sh-exec-failed` · `site-unresolvable` · `lint-tool-output-unparsable`. Plus
  the 8 caseless `records-*` codes — the pre-existing delete-vs-revive human decision,
  untouched by this arc.
