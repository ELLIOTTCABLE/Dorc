# 24H — first-contact polish charter (recon findings + the human's ack-rulings)

AI-authored (Opus conductor), 2026-07-05, round 24. The charter for the pre-r25 polish pass:
a read-only recon agent swept the CLI surface as a naive admin (transcript-gauntlet), extracted
the owner's CLI taste (jdx/mise conventions sampled live; Rust/Elm diagnostic style; dotfiles
read-only, personal-infra scrubbed), and produced a {cheap-apply | churn-batch} plan; the human
then ruled on every churn item. THIS note = the rulings + the hard invariants binding the apply
pass. The recon's full detail (gauntlet table, golden-churn counts per item, welds-compliance
enumeration) lives in the recon agent's report; the load-bearing conclusions are restated here.

## Recon headlines (the three worst first-contact hazards, all confirmed live)
- A book with a SYNTAX ERROR exits 0 and still emits a partial probe artifact to stdout.
- The plan-mode stderr firehose: a 5-line `dq-site-unresolvable` stanza per unprobeable site,
  including for `set -eu` and plain assignments (a 5,000-line book ⇒ 50,002 stderr lines).
- Every diagnostic location is a raw byte-offset (`--> 58:80`) — no file:line:col anywhere.

## The ack-rulings (human, 2026-07-05 — binding)

- **ack-1 exit codes: ADOPTED + family-designed.** Parse-failure exits nonzero. Design a small
  exit-code TABLE: reserve a **10+ range** for the class "fast-fail, vacuous/obvious failure,
  semantic, specific-to-dorc"; assign ONE specific code from the range to parse-errors (builder
  picks + documents the table in --help territory). Costs a conscious harness-contract touch
  (the crash-guard on the 14 expected-diagnostics cases).
- **ack-2 `dorc why`: ADOPTED + two HARD additions.**
  (i) **The unargumented default**: bare `dorc why …` (no line-address) reports on *the things
  that went wrong* — the problematic subset of the CURRENT analysis (refusals, walls formed,
  guards inserted, can't-tells, incoherences), not all lines — "can't be typing lines manually
  when you're already annoyed." Interpreted as current-run problems ONLY (any cross-run
  "recency" memory is the parked kSTATE knob — not built; flagged to the human as the reading).
  (ii) **rul24-lineno-identity (a product invariant, not a nicety):** there is ONE line-number
  space — the SOURCE file's. Whatever any Dorc output prints as `2 | printf …` MUST be
  queryable as `:2`; every printed line-number and every accepted line-address refer to the
  same source lines; any transformation (strip, insertion, elision-comment) preserves the
  mapping 1:1 with dorc-why. Line numbers are source-truth everywhere or nowhere.
- **ack-3 flags: keep `--trust-footprints`** (and current names generally) for now.
- **ack-4 vocabulary: ADOPTED, reason refined.** The unicode line is about ARTISTIC/unnecessary
  unicode; semantically-correct characters are fine ("literally using characters that are
  correct is just using language"). The real defect is JARGON: concepts like "drove to ⊤" are
  too technical for user-facing surfaces. Replace with quality, clear, simple, UNAMBIGUOUS
  English (builder mints a small consistent vocabulary, e.g. ⊤-class → "couldn't be
  resolved"); `⊤` stays in code/comments/corpus.
- **ack-5 dependencies: LIBERALIZED.** Deps are fine OUTSIDE the DST-kernel — "boring UI crap
  is exactly where dependencies buy us the most for the least cost." Adopt a color crate
  (anstream-class, handles Windows VT); probably adopt clap for the arg surface. The kernel
  crates stay dependency-clean (inv-determinism unmoved). cargo-deny compliance verified by
  recon (MIT/Apache/Unicode all pass).
- **ack-6 oracle discovery: explicit for the spike** (`-o` repeatable + `--oracle-dir`), plus a
  HINT when sibling `*.oracle.sh` files exist unloaded (magic suggests, never loads). Human:
  "middling feels… revisit outside spike."
- **ack-7 artifact comments: human ack-in-progress** — representative file handed over
  (`guard23-ternary-flagship/expected.out` primary; `strawman24-derived-survive/expected.out`
  for survival/derived flavor). No rewording until/unless the human returns edits.
- **ack-8 full rustc caret-art: DEFERRED post-first-blood** (line:col + region-reorder +
  excerpt is the accepted 80/20).

## Also in the apply scope (the recon's cheap tier, pre-agreed, golden-safe)
help-is-success(stdout,0) + rich usage · `--version` · did-you-mean on flags/modes · humane
file-error phrasing · **file:line:col regions** (the byte-offset fix; feeds rul24-lineno-identity)
· region-before-notes ordering + source excerpt · **firehose fix** (suppress
structurally-unprobeable noise incl. assignments/`set`; aggregate the rest into one honest
line) · isatty color on stderr severity (NO_COLOR honored; plain when piped — load-bearing for
golden-safety) · env-mirrors-flags (`DORC_*`) · plan-summary doc-comment fix (`may-alias=`).
Flow picks: `--results FILE` (+stdin default) · `-o` repeatable + `--oracle-dir` + loaded-oracles
advisory line · multi-book concatenation-as-one-unit · `dorc why` with `book.sh:N` / content-match
/ zero-arg-all + the ack-2 additions; ASCII depth-indented cause-chains, color = tier/severity
channel; no pagers, no interactivity.

## Fences (unchanged, restated for the apply agent)
Byte-floored receipt-free artifacts (stdout untouched except under a future ack-7 ruling);
rul-attention-honesty; stdout=artifact stderr=diagnostics; kernel purity + kernel dep-freedom;
kSTATE/kCOMMS/kUNIT/kOOB as fenced in the recon; the e2e goldens are stdout-only — stderr work
churns only the 14 expected-diagnostics + 13 expected-why needle files, tracked consciously.
