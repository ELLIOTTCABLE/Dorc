> QUARANTINED RAW LANE MATERIAL - 24K cross-model language-design crosscheck (2026-07-05).
> NOT adjudicated findings. The adjudicated verdict is Research/notes/24Kc-language-crosscheck-adjudication.md.
> Anthropic Fable lane, NEUTRAL stance (24Ka): verbatim extract from commit 19df800 on branch ai/24Ka-langreview. Byte-authoritative copy = the branch commit; canonical corpus home pending cherry-pick.
> Archived verbatim from the session scratchpad post-compaction, at the human's direction.
> Do not cite as authority; do not read during future crosscheck skill-ups (contaminates pre-registration).

# 24Ka — Fixture/strawmen census (mechanical; via census subagent, integrated 2026-07-05)

Denominators: 187 *.oracle.sh, 154 book.sh under spike/e2e/cases/; strawmen tallied apart.
Full tables in the subagent deliverable (reproduced condensed here); grep patterns stated
there per family. Key integrated figures:

## Naming
- 279 function defs total: 185 `X__predict` (underscore scheme) + 94 dotted roles + 0 plain.
- Dotted roles: is_converged 69 · touches 16 · predict 4 · reaches 2 · resolve 2 ·
  is_diverged 1.
- THE SAME ROLE both ways: predict = 185 underscore + 4 dotted. 71/187 files mix schemes
  internally (`apt_get__predict` beside `apt-get.is_converged`); 114 pure-underscore;
  2 pure-dotted. Hyphenated left-of-dot: apt-get only (~77 defs).
- Books: 153/154 pure POSIX; the single exception defines `apt_get__predict` in STRIPPED
  form (plain `pkg="$1"`, no marks) — a re-ingested artifact, consistent with strip-law.
- Shebangs: 0/187 oracles; 3/154 books.

## Marks and binds
- Inline value-binds `name : kind = "$1"`: 180 across 178 files; spelling RIGIDLY uniform
  (3-space indent, single spaces, always "$1"). Valueless binds `name : kind`: 9 (singleton
  cells, always sharing a line with the probe).
- Trailing establish-marks: 307 / 157 files. Negated (`.prop!`): 140 (installed! 128,
  enabled! 9, allowed! 3). Observe-marks `:?`: 30. Bare-kind reach-marks (`: file`): 2.
- ACK / POISON / tilde bare-mark statements: ZERO in the corpus (law preserves ACK/POISON
  as surviving grammar; the tilde is dead; none are exercised). All `^:`-lines in fixtures
  are ordinary POSIX null-command redirects in books.
- Properties observed: installed, active, enabled, allowed, present, tuned, written, fresh,
  v0155 (a version literal as property).

## Idioms
- Flag-strip loop `while [ "${1#-}" != "$1" ]; do shift; done`: 405 / 135 files (4 per
  fully-guarded oracle).
- Arity gate `if [ "$2" = "" ]`: 133 files; `${2-}` / `$#` variants: ZERO in e2e.
  (15x strawmen used `[ "$#" -le 2 ]` — the older corpus had the set-u-safe form; the
  blessed idiom regressed.)
- Decline: TWO coexisting conventions for identical semantics — explicit `*) return 2 ;;`
  (53 files) vs implicit unhandled-path (the guard23 ternary family, whose fixture comment
  says declines are spelled as unhandled paths). One intent, two spellings.
- `command -v` polarity split: spelled establish (` : `) in one fixture family and observe
  (`:?`) in others — same command, two mark polarities (deliberate per-fixture, still a
  divergence).
- DORC_* in fixtures: DORC_FLAGS (harness marker) only. NO DORC_REPORT anywhere; `UNK`
  breadcrumbs never emitted (USER_STORY-only protocol). 17x adversarial strawman used
  `${DORC_SCRATCH:?}` (note: THE DEFENSIVE FORM) for its lanes.
- `--` end-of-options: 74 uses / 44 files. `| grep -q` inside probe bodies: 3 files (the
  otelcol verdict) — probe bodies otherwise pipe-free per an-probe-shape.

## Strawmen strata (evolution evidence)
- 15x (oldest): dotted `.predict/.diff/.version` roles; explicit `return 0/1/2` verdicts;
  `$#` arity; verbose stderr refusals. No binds/marks/underscore-predict.
- 17x: plain-POSIX one-line predicate oracles (`docker_present()`); adversarial files carry
  the proto-bind `local w : com.frobber.Wombat{defrocked,frocked} = "$1"` + the recorded
  `dash -n 'Bad function name'` note.
- e2e (current): underscore-predict + dotted-verdict + binds/marks/`:?` + flag-strip +
  `[ "$2" = "" ]`. Roles .diff/.version vanished (deferred by design).

## Integration deltas applied to -05 findings
- finding-migration-debt: exact strata numbers (185 vs 4 same-role spellings; 71 mixed
  files); plus the decline-convention fork.
- finding-flagstrip-ceremony: 405/135 figure.
- finding-nounset-idioms: 133-file universality + the 15x->e2e regression ($# -> "$2");
  DORC_REPORT teaching-only.
- finding-mark-polysemy: counts; ACK/POISON law-vs-corpus gap; command -v polarity split.
- cleared-erasability-of-books: 153/154 at volume (the 1 exception stripped-form).
- finding-compiled-dialect: "oracle files with zero dialect constructs: 0" — no sh-only
  authoring path exists in practice for oracles.
