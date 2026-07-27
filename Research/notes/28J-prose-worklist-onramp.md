# 28J — the prose-authoring worklist + on-ramp (durable home)

AI-authored (Fable conductor, 2026-07-26). The lane-prose-onramp inventory (report-only
lane, banked compressed in `28H` §2), made durable and REFRESHED against the W4 folds
(carrier/parts/span landed; drifted-driver in flight). This is the human's W5 working
list. Live state at any moment: `sh ./_prose-worklist.sh` from `spike/`. Prose states
and gates: `spike/crates/aid/CLAUDE.md` (prose-three-state, arrangement-prose-marker).

## The worklist (209 rows at the onramp census)

| class | rows | edit-home |
|---|---|---|
| catalog `sm `-migrated | 75 registers / 74 codes | 40 have `crates/aid/tests/<slug>.loom` faces; 35 lock-only until cases exist |
| catalog `[unwritten:]` | 7 | ALL SEVEN have looms — the best first sitting |
| arrangement `Words::Migrated` | 106 (55 single-run / 51 multi-run) | lock-only until a driven why transcript stamps them (changing NOW — see below) |
| arrangement `Words::Unwritten` | 21 | lock-only; a wordless row's `[unwritten:]` span IS editable once a transcript carries it (`28H` span ruling 4); values-bearing ones need a hand-seed first |
| jargon-glyph allowlist (`⊤`/`⊄`) | 9 | 2 loom-faced (`unmodeled-wall-inventory`, `cli-help-page`), 7 lock; DELETE the row's `ASCII_SWEEP_ALLOWLIST` entry in the same commit (`no_allowlist_entry_is_stale` enforces) |

The 7 unwritten catalog codes, all loom-editable today: `marker-version-unrecognized` ·
`mark-unknown-verb` · `mark-rc-arity-exceeded` · `mark-standalone-rc-consumer` ·
`mark-hashcolon-malformed` · `host-evidence-admission-refused` · `whylog-unwritten`.

## The flows (onramp-lane-verified green end-to-end)

Loom path (the 47+ faced rows) — clean tree required:
```
$EDITOR spike/crates/aid/tests/<slug>.loom     # type over the `sm ` text / [unwritten:]
mise run loom:compile
mise run loom:promote                          # then `git --no-pager diff` (timed-output eats the preview)
mise run test
git commit spike/crates/aid/tests/<slug>.loom spike/crates/aid/src/catalog_lock.rs -m "(re aid) ..."
```

Lock path (rows without faces yet) — hand-seed, then bless owns the golden drift:
```
$EDITOR spike/crates/aid/src/arrangement_lock.rs   # Words::Unwritten -> Words::Migrated(&["..."])
mise run test                                      # affected transcripts drift
mise run bless -- <case-substring>                 # SCOPED bless now works (parts lane); scoped = bless-then-gate
git commit ... -m "(re aid) ..."
```
Order law when locks and goldens both move: promote FIRST, then rebuild, then bless
(`two-bless-paths-split-by-directory`). Human commits from a session: `DORC_HUMAN_COMMIT=1`.

Sharp edges: N values need N+1 word runs (`when_used` states each row's value count;
wrong arity is a loud debug panic) · hand-seeded rows must follow SERIALIZER field
order (`slug · occurrence · when_used · why · words`) · a transcript edit currently
appends a trailing `\n` to the stored register (known friction; don't "fix" the shape
mismatch by hand) · `[unwritten:]` and `[unnarrated:]` placeholder TEXT is computed —
type over it in a transcript, never re-create it as words.

## Top-10, highest value first (homes refreshed at the span fold)

1. `why-next-step-describe-walls` occ 0 — lock. Blanks the next-step of the common
   one-wall guarded case; the sharpest banked item.
2. The three drift rows (`why-receipt-book-drifted` · `why-drift-analysis-suppressed`
   · `why-drift-address-unanswerable`) — GAINING FACES NOW (the drifted-driver lane's
   new case); hold a day if you prefer transcript-editing them.
3. `why-receipt-when-replayed` — same: about to gain a face.
4. `why-participating-lines-closure` — lock (renders 5× across four whygallery cases;
   highest-frequency blank); face waits on the full driver (the open scope ask).
5. `why-receipt-dispositions-predicted` — same as 2/3: receipt-header row, face incoming.
6. The 7 unwritten catalog codes — loom, TODAY, fastest loop; the best first sitting.
7. `why-declines-*` occurrences 1–3 (explanation/join/next-steps-opener ×
   unmodeled/interactive/hazard) — lock; nine rows, three parallel families.
8. `why-reason-run-declined` + `why-improvement-declined-unmodeled` — lock.
9. The 40 loom-faced `sm ` catalog codes — loom, today; bulk burn-down, each
   self-contained.
10. The 9 jargon-glyph rows — 2 loom / 7 lock; allowlist entry deleted per commit.

## What the W4 folds changed since the census

- The transport now re-splits value-interleaved lines (one section, many fragments):
  multi-run rows are transcript-editable WHEREVER a driven transcript stamps them —
  the census's "51 multi-run = lock-only regardless" is DEAD as a permanent statement;
  it holds only until transcripts exist.
- The first driven why transcript (drifted-receipt case, `crates/aid/tests/`) is the
  in-flight drifted-driver lane; drift + receipt rows get faces there.
- Chain-row faces (the flagship strawman-a prose) wait on the full driver extraction —
  `28H:ask-full-driver-this-arc-or-r30`, with the human. Chain prose is lock-editable
  meanwhile.
- The trust-spent fix (parts lane) re-cut the webhost aggregate: the human's
  `f4f48316` red-line rebases over the fixed transcript.
- Under `28H` §0 render-surface-instability: the WORDS are the durable investment (they
  live in the registry); loom LAYOUT will churn — author words, skip layout-fiddling.
