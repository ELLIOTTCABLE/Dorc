# 24P — respell specimens: five fixtures respelled by the conductor, the ack-gate for the churn

AI-authored (Fable conductor, 2026-07-09). The human directed (in-chat, same date): before the
corpus-wide respell churn is dispatched, ~five high-signal goldens are rewritten BY THE CONDUCTOR
under all the new rules, so the design can be fully acked in-code — "me saying 'oh, y'all did that
wrong' after editing hundreds of files would be a huge waste of tokens." This note is the durable
index: the specimen edits themselves live in the WORKING TREE, deliberately UNCOMMITTED (human
in-file review; the committed tip stays green for the r26 sibling). **If the working-tree edits are
ever lost, they regenerate mechanically from §2–§4 of this note.** The respell builder treats the
acked specimens as spec-by-example.

Rulings being exhibited: `24M` §1 (bare names · version-comment · reverse-DNS kinds · typeless
floor · rungs default) + the 2026-07-09 in-chat rulings (`24C` §r24-in-chat: rul24-totalistic-munge
· rul24-marker-v0.1 · rul24-marker-in-churn · rul24-idn-punycode · rul24-selector-pre-stdlib) +
the landed wave-1 verdict-lift (fix-return-decline-inert).

**Scope clarification (HUMAN-TYPED 2026-07-09, transcribed):** *implementing* punycode etc. does
NOT need to land in the spike — spec-notes for eventual extraction into the full spec suffice; the
spike does whatever is minimal and straightforward. (Concretely: the spike munge handles the ASCII
path — `-`→`_`, `.`→`_`, leading-digit `_`-prefix — and non-ASCII input keeps hitting the landed
`munge-name-invalid` loud refusal until the real implementation; the punycode acceptance is a spec
obligation, not a spike obligation.)

## §1 — The five specimens (working-tree paths; all in `spike/e2e/cases/`)

1. **`guard23-ternary-flagship/package.oracle.sh`** — the guard-tier centerpiece. Exhibits:
   marker line-1 · bare rename of a dotted verdict fn (`apt-get.is_converged` →
   `apt_get__is_converged`) · kind re-key in binds AND marks (`package` → `sm.dorc.Package`) ·
   `${2-}` nounset idiom · the unhandled-path decline STYLE (no catch-all) · comment repair of
   the now-stale "the dialect has no `return`" claim (superseded by fix-return-decline-inert).
   The case's `book.sh` is deliberately UNTOUCHED — a plain-sh book carries no dialect, needs no
   marker, stays byte-verbatim: that untouchedness is itself the exhibit.
2. **`strawman24-survive-simple/package.oracle.sh`** — the survival tier. Adds: the touches()
   TYPED-EMISSION migration — `printf 'package:%s\n' "$1"` (stringly, DEAD) becomes
   `printf '%s\n' "$1" : sm.dorc.Package` (raw entity lines; kind rides the trailing mark on the
   emitting command — the USER_STORY stage-7 form) · the explicit `*) return 2` decline STYLE
   (now genuinely READ post-verdict-lift; comment says why the catch-all answers can't-say).
3. **`strawman24-alias-provides/package.oracle.sh`** — the kind-keyed owner. Adds: the resolver
   rename `package.resolve()` → **`sm_dorc_Package__resolve()`** — the kind's dots forward-munge
   into the NAME (`sm.dorc.Package` → `sm_dorc_Package`); lookup is forward-only (the engine
   munges the referenced kind and finds the owner; never decodes the NAME back).
4. **`strawman24-reach-static-service/service.oracle.sh`** — multi-provider file + reaches().
   Adds: three providers renamed in one file (`hork__*`, `enablesvc__*`) ·
   `sm_dorc_Package__reaches()` with its static arm re-keyed (`printf '%s\n' "$1" :
   sm.dorc.Service`) · a second stdlib kind (`sm.dorc.Service`).
5. **`strawman24-pipe-guard-oracle-converged/{grep,otelcol}.oracle.sh`** — the Query/pipe tier.
   Adds: `sm.dorc.GrepMatch` (a non-obvious stdlib re-key) · the `:?` OBSERVE mark carrying a
   dotted kind (`:? sm.dorc.GrepMatch:"$pat".matched`) · a THIRD-PARTY author's kind under the
   vendor's own domain — `otelcol` → **`io.opentelemetry.Collector`** (reverse-DNS is not an
   sm.dorc monopoly; ≥2 dots holds) · the improvised empty-entity singleton bind
   (`v : io.opentelemetry.Collector`) kept faithful with a comment that the typeless floor
   (24L §2) retires it.

**The typeless-floor exemplar** (no existing fixture — the floor is unbuilt; this is the sixth
specimen, in-note only). A MARKER-FREE file; pure POSIX end to end; nothing to strip; per
rul24M-typeless-floor + rul24M-rungs-default this alone licenses guard+elide at foobar's own
converged sites once the floor lands:

```sh
foobar__is_converged() {
   case "${1-}" in
      sync-certs) foobar status --certs-current -- "${2-}" ;;
      *) return 2 ;;
   esac
}
```

## §2 — The mechanical rule-set the specimens encode (the builder's checklist)

- marker: `# dorc-lang/v0.1` as LINE 1 of every file carrying any dialect construct (binds,
  marks, `:?`); grammar accepts it anywhere in the first 10 physical lines — the corpus
  standardizes on line 1. A file with NO dialect constructs gets NO marker (the floor exemplar;
  plain books).
- names: every dotted role funcdef `X.role()` → `<munge(X)>__role()`; the dotted form is DEAD
  (parse arm deleted, per rul24-totalistic-munge — the engine change rides the same churn).
- munge (spike-minimal): `-`→`_` · `.`→`_` · a leading-digit RESULT gains a `_` prefix ·
  non-ASCII stays a loud `munge-name-invalid` refusal (punycode = spec-note, §0).
- kinds: every bare corpus kind re-keys — `package`→`sm.dorc.Package` · `service`→
  `sm.dorc.Service` · `grepmatch`→`sm.dorc.GrepMatch` · (encountered per-file: `pkgindex`,
  `file`, `firewall`, etc. → `sm.dorc.PkgIndex`, `sm.dorc.File`, `sm.dorc.Firewall` — PascalCase
  TypeName, per 24M's `sm.dorc.TypeName`) · third-party/vendor kinds go under the vendor's
  domain (`io.opentelemetry.Collector`), never sm.dorc.
- touches()/reaches() emission: raw entity lines; kind as the trailing mark on the emitting
  command (`printf '%s\n' "$1" : sm.dorc.Package`); the stringly `printf 'kind:%s\n'` form dies.
- verdict declines: BOTH styles legal and exhibited — unhandled-path (specimen 1) and explicit
  `return 2` (specimen 2); a reached literal `return 0` VOUCHES (landed verdict-lift). Comments
  claiming "the dialect has no return" are stale and must be repaired wherever found.
- nounset: `[ "$2" = "" ]` → `[ "${2-}" = "" ]` (the `${2-}` sweep); deeper nounset-hardening is
  the builder's judgment per-fixture.
- ACK/POISON bare-marks: delete from the grammar (zero corpus occurrences — nothing to respell).
- books: byte-untouched unless they host oracle functions (share-a-file), in which case they
  gain ONLY the marker.

## §3 — Grammar/engine flags the specimens surface (builder must handle; conductor-found)

- **flag-dotted-kind-in-marks (the sharp one):** the mark grammar's fact-label
  `kind:entity.prop` now contains dots INSIDE the kind token (`sm.dorc.Package:"$pkg".installed`)
  while `.` also introduces the property suffix. The quoted-entity form disambiguates
  structurally (kind = everything before the `:` that precedes the quoted entity), but the mark
  parser's kind-token charset must be extended to dotted names, and an UNQUOTED dotted entity is
  now ambiguous — the builder should ⊤-reject-with-diagnostic that corner rather than guess
  (inv-top-reject posture). Same extension for bind position (`pkg : sm.dorc.Package = "$1"`)
  and the `:?` observe mark.
- **flag-forward-munge-keying:** provider lookup flips from the literal-dotted/un-munge pair to
  forward-munge (book word → munge → NAME match). Spec sentence (human-directed): *an oracle
  named `<seg>__role` serves every book command word munging to `<seg>`*; two DISTINCT co-loaded
  source names munging to one segment = the landed `munge-name-collision` refusal.
- **flag-marker-gating-scope:** the marker gates SYNTAX only; `__role` NAME-recognition works in
  unmarked files too (rul24M-version-comment interpretation — required by the floor). So: dialect
  constructs in an unmarked file = a loud error naming the missing marker; a bare verdict fn in
  an unmarked file = the floor, fully live.
- **flag-stale-comments:** fixture comments citing dotted names, stringly touches, or
  "dialect has no return" must be repaired in the same pass (the guard23 stale-XFAIL-comment
  cleanup already rides the churn; this widens it to naming/emission claims).

## §4 — Predicted golden-delta classes (what the re-bless will show; conductor-inspected at merge)

1. Fact-label re-keys everywhere they render: plan comments, `why`-lens strings, records —
   `package:nginx.installed` → `sm.dorc.Package:nginx.installed`. The bulk of the diff.
2. Guard-preamble function names: NO delta (goldens already carry the munged `apt_get__*` forms
   — the strip was already emitting them).
3. Marker lines: NO delta in artifacts (markers live in fixtures; the shipped/off-ramp artifact
   is receipt-free and books don't gain markers unless share-a-file).
4. The guard23 stale-comment strings echoing into 6 goldens (`resid-guard23-stale-comments`).
5. `dorc-records`/trailer lanes: unchanged from r23 (the `dorc-records/v0.1` form-rename is
   FLAG-to-r26, their surface).

## §5 — Micro-decisions embedded here, for the human's ack (nack any individually)

- d1: marker standardized at line 1 (grammar allows first-10).
- d2: stdlib kind casing `sm.dorc.PascalCaseTypeName`.
- d3: typed-emission spelling = kind as trailing mark on the emitting printf, raw entity lines.
- d4: unquoted dotted-entity in marks ⊤-rejects (quoted form is the supported spelling).
- d5: kind-owner NAME = full munge of the kind (`sm_dorc_Package__resolve`) — long, accepted
  (24M: long names priced; the §4b length errand cleared 300-char names everywhere).
- d6: both decline styles stay legal (no lint nudging toward either).
- d7: third-party kinds under vendor domains (`io.opentelemetry.Collector`), exhibited in-corpus
  so the stdlib isn't the only pattern the fixtures teach.
- d8: the improvised otelcol auto-cell bind stays faithful until the floor retires it.
- d9: spike munge = ASCII-only path; punycode spec-noted (§0), not implemented.

## §6 — Process record

Conductor-authored under the human's explicit override of the Fable-conducts/Opus-codes split
(this task only). Specimens are UNCOMMITTED working-tree edits at tip `87da39c`-era; e2e at tip
is EXPECTED to fail for these five cases while the edits sit un-acked (engine at HEAD parses the
old spelling; the dotted-kind marks and re-keyed labels don't round-trip yet) — that is the
review artifact, not breakage. Respell dispatch is gated on the human's ack of §5 + the in-file
review; on ack, the builder brief = §2 checklist + §3 flags + the wave-1 deferral list
(`24C` resp-munge-policy / resp-collision-ship-refusal) + LIVING_STATUS queue item 2's fold-list.

## §7 — Post-ruling addendum (2026-07-09, after the specimen files were authored)

Two rulings landed after §1–§6 were written; the builder's checklist (§2) extends:

- **rul24-ditch-is-diverged (`24C`):** `is_diverged` is REMOVED, hard, in this same churn —
  out of the reserved-suffix set + the lift + the reservation lint's suffix table; the dual
  sense-flip glue and the `VerdictSense` parameter delete; the one exercising case
  (`guard23-inverted-vouch-never-backwards`) converts to pinning explicit-return manual
  inversion (`case $? in 1) return 0 ;; 0) return 1 ;; *) return 2 ;; esac` style) never
  licensing backwards. The role family is now predict / is_converged / touches / resolve /
  reaches. None of the §1 specimens used the diverged sense; they stand unmodified.
- **The one-liner-candidate rider:** while sweeping fixtures, FLAG (never convert) any oracle
  whose verdict could idiomatically be a `tool --check "$@"`-style passthrough; report the
  list. (The founding-one-liner PIN itself rides the floor build, not this churn —
  fd-headline-oneliner-gap, `24C`: quoted-`"$@"` is unmodeled in the tracer until then.)

## §8 — Review outcome + two riders + process supersession (2026-07-10)

**Ack state (human-typed 2026-07-10):** d1–d7 + d9 ACKED; the six modified specimen files
reviewed in-file, LGTM. d8 explained (the improvised empty-entity singleton bind IS the
hand-spelled auto-cell; `24L` §2 cites this very fixture as the floor's strain evidence, and
`24L`'s test obligations already schedule its retirement with the floor build — keep-faithful =
one churn per surface, and the floor gets a real before/after exemplar) — probable-ack,
one-word confirm pending.

**rider-comment-budget (human-directed; STANDING for all future builder briefs, not
respell-only).** Fixture/test churn must RIP comments, not update them: a comment that had to
be updated to survive the change is presumptively deletable, unless the file's specific
subject-matter IS the changed thing. The respell brief carries:
- *comment-keep-test* — a comment line survives only by passing one of: **keep-purpose** (the
  fixture's single what-this-case-pins header, ≤2 lines); **keep-subject** (explains the exact
  behavior THIS fixture exercises — test: if the sentence fits equally well in another fixture,
  it is ambient design-lore and fails); **keep-hazard** (warns that an innocent-looking edit
  breaks the pin non-obviously, in the fixture that pins that semantics); **keep-machine**
  (shebang + the `# dorc-lang/v0.1` marker — budget-exempt, never deleted; verified 2026-07-10:
  run.sh parses NO fixture-source comments — its `# dorc: guard` parsing reads rendered-artifact
  lines, and `expected-diagnostics` is a separate file).
- *comment-delete-defaults* (no justification needed): ruling-slug/note citations + dates;
  authorship/history trails; restating-the-code; cross-fixture references; reviewer-addressed
  correctness justifications; anything updated-to-survive (unless keep-subject). Comment
  knowledge homed nowhere in `Research/` is FLAGGED in the builder report, never kept inline.
- *comment-hard-budget* (mechanical; run + paste numbers before ending turn; non-negotiable):
  `awk '/^[[:space:]]*#/ && !/^#!/ && !/^# dorc-lang\//{c+=length($0)+1} END{print c}' cases/*/*.sh`
  must report ≤20% of the merge-base measure. Baseline measured 2026-07-10: 2,213 comment
  lines / ~192,009 bytes across the 126 cases' `.sh` files ⇒ target ≈ ≤38.4k bytes. Anti-gaming:
  converting whole-line comments to EOL comments is prohibited. Anti-Opus clamp (verbatim-class):
  "you will feel each comment is deadly-necessary; delete borderline comments anyway — the
  conductor re-adds at review, the cheap direction."
- The §1 specimens' own comment verbosity is NON-NORMATIVE (conductor-authored citation-trail
  style, exactly the delete-defaults class); the normalization pass below rips them first.

**rider-shebang (human-directed 2026-07-10).** Dialect-carrying fixture files (the ones
genuinely invalid-as-sh) gain a LINE-1 shebang onto a Dorc-provided strip-and-exec tool —
defensiveness for something already true. Plain-sh books get none (nothing to rescue).
Constraints (human-typed): no part of the shebang is required, parsed, or constrained by the
engine — quietly idiomatic only; the language-version does NOT ride the shebang (the
`# dorc-lang/v0.1` comment stays the gate). Consequence, amending d1: where a shebang exists
the marker slides to line 2 (grammar already accepts first-10; corpus convention becomes
"marker immediately after the shebang where one exists, else line 1"). Spike-minimal tool
implementation + one smoke e2e ride the churn. The exact SPELLING (single-token
`/usr/bin/env <tool>` vs `env -S` forms; tool name) firms from the shebang prior-art research
adjudication — recorded in §9 when it lands, alongside the one open fork: whether `dorc strip`
off-ramp OUTPUT rewrites a dorc-pointing shebang to a plain shell (off-ramp-serving, but
arguably the shebang-parsing the human prefers to avoid).

**Process supersession (amends §6, human-typed 2026-07-10):** after the research adjudication,
the CONDUCTOR normalizes the specimens in-tree (comment-rip to model the budget + shebang
stamp + any adjudication-driven spelling fixes), then COMMITS them as deliberately-failing
example-tests for the implementor (supersedes §6's "deliberately UNCOMMITTED"; FLAG-to-r26:
the r23 tip carries declared-failing spec cases until the respell lands; raw-red vs
harness-XFAIL mechanics = conductor's call at execution). Durability pass (this note +
LIVING_STATUS) precedes a conductor REWIND; the implementor dispatches post-rewind from the
durable record.

## §9 — The shebang adjudication + the normalization record (2026-07-10, conductor)

Evidence base: **`24Q`** (the prior-art digest; graded sources, taxonomy, gotcha inventory).
Decisions rest only on multi-source-convergent claims (the one-arg rule; env -S's holes;
interpreter-must-be-binary off-Linux); the digest's ~SUSPECT cells are deliberately not
load-bearing. These are conductor judgments under the human's stated constraints (nothing
required/parsed/constrained; quietly idiomatic; spelling delegated) — nack freely.

- **decision-shebang-spelling:** the corpus stamp is the single-token form
  **`#!/usr/bin/env dorc-sh`**. One interpreter + one argument = inside every kernel's
  delivery model (one-arg Linux/BSD, first-arg-only SVR4 lineage, splitting macOS all agree
  when there is only one token); no `env -S` dependency; ~30 bytes against the 127-byte floor.
  The marker sits on line 2 directly beneath it (§8's d1 amendment).
- **decision-shebang-iff-marker:** a fixture gains the shebang iff it carries the
  `# dorc-lang/v0.1` marker — one rule, one dialect-presence signal, two lines. Plain-sh books
  are untouched (whatever shebang they already have stands; none is added).
- **decision-dorc-sh-semantics (spike-minimal; spec-note tier beyond it):** `dorc-sh` =
  strip-if-marked (identity on plain sh), then `exec sh -c "$stripped_text" "$script_path"
  "$@"` — `$0` and `"$@"` fidelity preserved without a temp file (POSIX `sh -c cmd_string
  cmd_name args…` assigns `$0` from cmd_name). ARG_MAX bounds the -c string for pathological
  script sizes — disclosed, ru-26-style, fine for fixtures. Spike ships it as a second thin
  bin target riding the same churn as `dorc strip`, plus ONE smoke e2e (exec a marked oracle
  file via dorc-sh under the mocks discipline).
- **decision-executor-bandwidth (spec-note; the spike ships only the zero-arg form):**
  `dorc-sh [executor-cmd…] -- script [args…]`; with no `--`, argv[1] is the script and the
  executor defaults to `sh`. The executor-bearing form inherently requires `env -S` (multi-token
  shebang) and inherits `24Q` §3's portability matrix — documented, never required. The `--`
  convention is the guix-shell prior-art shape and removes the where-does-the-script-start
  ambiguity without inspecting the filesystem.
- **decision-strip-leaves-shebang (the §8 fork, resolved):** *(SUPERSEDED IN HALF,
  2026-07-12: the task-6 close's human-acked strip wording — `274` §13 "strip =
  prefix-erasure + shebang rewrite only"; `271:rul-dorc-prefix-head-synthesis` —
  reinstates a shebang-RUNNER rewrite in strip OUTPUT, so stripped artifacts are fully
  dorc-free. The other half stands: the ENGINE never parses or recognizes shebang
  content at analysis time, and `dorc-sh` itself never reads it.)* `dorc strip` NEVER touches the
  shebang; NOTHING in dorc parses or recognizes shebang content, period (the engine doesn't
  read it; dorc-sh itself doesn't either — a `#!` line is an ordinary comment to sh). This
  keeps the human's no-parsed/no-constrained constraint absolute and the kOOB comment-parse
  set closed at ONE (the marker). Off-ramp arithmetic: a stripped artifact is plain sh and
  ALWAYS runnable via `sh file` regardless of its shebang; direct `./file` on a dorc-less box
  fails loudly at env-lookup — honest, and repaired by editing one line by hand. (The digest's
  gotcha-9 caveat — some shells misattribute that ENOENT to the script name — noted, accepted.)
- **decision-no-polyglot:** the tclsh/perl re-exec family (`24Q` §4 taxonomy-polyglot) is the
  only graceful-degradation shape, but it costs 3 lines of executable ceremony per file and —
  disqualifying — a top-level `exec` statement inside oracle files that the analyzer would have
  to specially tolerate: a new parsed/constrained surface, the exact thing ruled out. Not the
  corpus idiom; recorded as a pattern an AUTHOR may hand-roll (it composes fine; dorc never
  needs to know).
- **naming:** `dorc-sh` is STRAWMAN-tier (a rename is a sed; low lock-in).

**Normalization applied (same date, this working tree):** the six specimens gained the
line-1 shebang + the §8 comment-rip (each now carries ≤4 comment lines beyond shebang+marker:
the purpose header plus keep-subject/keep-hazard lines only — the citation-trail headers are
gone; comment knowledge all pre-exists in this note, `24C`, and `24M`). FOUR cases carry
one-sided **`XFAIL`** files (harness idiom; suite stays green-with-declared-xfails for the r26
sibling). IMPLEMENTOR NOTE, load-bearing: those cases' goldens are STALE-OLD, not
desired-future — landing the respell produces a golden-DIFF, not XPASS; the flow is
delete-XFAIL → BLESS on a verified binary → conductor inspects the diff against §4's predicted
delta classes (never bless-first).

**§9b — execution record (2026-07-10; what landing the normalization taught):**
- **fd-dotted-kinds-parse-at-HEAD (deflates §3's first flag; builder intel).** The pipe-guard
  specimen pair is LIVE at HEAD: dotted kind tokens in bind position (`pat : sm.dorc.GrepMatch
  = "$1"`), in `:?` observe marks, and in rendered fact-labels all lift TODAY — the case plans
  `sites=4 elide=1 omit=3` with zero mark diagnostics, and the artifact renders the re-keyed
  label. §3's "the mark parser's kind-token charset must be extended" over-predicted for these
  shapes; what the builder still owes §3 is the UNQUOTED-dotted-entity ambiguity corner
  (⊤-reject posture) and per-shape verification elsewhere.
- **Specimen states diverge accordingly:** `strawman24-pipe-guard-oracle-converged` carries NO
  XFAIL — its only delta was the §4-class-1 fact-label re-key in `expected.out`, blessed NOW,
  conductor-inspected (exactly one line: `grepmatch:` → `sm.dorc.GrepMatch:`). The other four
  (flagship, survive-simple, alias-provides, reach-static-service) genuinely fail at HEAD —
  stale-old goldens plus, for the survival/reach pair, behavior (typed-emission `touches()` /
  re-keyed owner lookup not yet lifted); per-case decomposition is the builder's.
- **Harness facts surfaced (bind the implementor's final BLESS):** (a) XFAIL is
  golden-text-BLIND by design (structural gates only) — a case whose only failure is stale
  golden TEXT registers XPASS, which counts as red; that is why pipe-guard could not stay
  XFAIL. (b) A fresh BLESS rewrites ~11 unrelated cases' `expected.ran` goldens with
  bless-artifacts: empty-file→single-newline normalization, plus benign lax-order
  interleaving re-captures on two `RAN_ORDER=lax`-family cases (pipe-guard-floor,
  pipe-guard-unvouched-mid). These were REVERTED this round to honor r26's byte-stable-goldens
  contract; the implementor's bless will regenerate them and must inspect-and-revert (or
  consciously accept + record) the same set.
- **Harness edit:** `dorc_flags_selftest`'s anchor retargeted `strawman24-survive-simple` →
  `strawman24-survive-multiwall` (the old anchor is now an unparseable-at-HEAD specimen, which
  zeroed both selftest counts and FATALed the whole suite; multiwall verified 0→1 under the
  flag). Note in-code at the selftest.
- **Suite state at commit: all 126 pass** (121 live + 4 declared xfail + pipe-guard live),
  fresh `cargo build --workspace` + full foreground e2e, conductor's own hand.

## §10 — Dispatch gating changed at r24 close (2026-07-10; supersedes "dispatch on rewind")

Round 24 closed by reshuffle (`notes/24U`); the respell is now the FIRST stage of
`270:block-rebuild` and **waits on `270:block-settle`** — specifically the entity-algebra
design note's authored-spelling ack (`270:adj-entity-algebra`), because the structured
entity/selector algebra may move the mark grammar this churn rewrites into, and the fixture
sweep must happen exactly once. If the algebra moves mark spelling, the CONDUCTOR amends
these six specimens in-tree first (spec-by-example stays the review vehicle; never re-churn
hundreds of files to communicate a spelling change) and §4's predicted delta classes update
with them. Ack states at close: d1–d9 ALL acked (d8 typed 2026-07-10). One brief addition
from the r26 extractables (`262` §7, scheduled via `270`): gate-1's record-compare goes
order-insensitive in this same golden churn. Everything else in §2/§3/§5/§7/§8/§9 stands.

## §11 — the task-12 amendment pass (2026-07-12, conductor-directed; `277` §7a)

The entity-algebra design note (`277`) moved the mark grammar this churn rewrites into, so — per
`270:adj-entity-algebra` and the §8 spec-by-example precedent (ack the design in the files) — the
six specimens were amended in-tree a SECOND time before the corpus respell fires. All edits are in
`spike/e2e/cases/`, oracle `.sh` sources only; goldens (`expected.*`) and books (`book.sh`) were
NOT touched (goldens stay stale-old by design — the implementor blesses).

- **selector-introducer respell** (`271:rul-selector-introducer-hash`): every coordinate's old
  property-suffix `.prop` becomes an ATTACHED `#prop` in trailing marks —
  `…Package:"$pkg".installed` → `…"$pkg"#installed` (and `.installed!`→`#installed!`),
  `…Service:"$1".enabled` → `…"$1"#enabled`, `…GrepMatch:"$pat".matched` → `…"$pat"#matched`,
  `…Package:"$pkg".tuned` → `…"$pkg"#tuned`, and the empty-entity
  `io.opentelemetry.Collector:.v0155` → `…:#v0155`. Nine marks across the five oracle files.
  Kind-name dots (`sm.dorc.…`), bare-kind emission marks (`: sm.dorc.Package`), and binds
  (`pkg : sm.dorc.Package = "$1"`) are untouched. Predicted-delta consequence: §4's class-1
  (fact-label re-keys, the bulk of the diff) now ALSO carries the corpus-wide selector re-key
  `sm.dorc.Package:nginx.installed` → `sm.dorc.Package:nginx#installed` wherever a label renders
  (`277` §4a / §7b) — one more re-key riding the same golden churn.
- **role renames** (two exemplars, exhibited): `apt_get__touches` → `apt_get__disturbs`
  (`271:rul-touches-becomes-disturbs`) in survive-simple and alias-provides;
  `sm_dorc_Package__reaches` → `sm_dorc_Package__disturbance_reaches_only`
  (`271:rul-at-most-family-names`) in reach-static-service. The other `__touches` occurrences in
  reach-static-service (`hork__touches`, `enablesvc__touches`) are deliberately NOT renamed here —
  the amendment migrates only the two exemplar names; the corpus-wide `touches`→`disturbs` sweep is
  the respell builder's (`strawman24-derived-survive`'s golden still carries `apt_get__touches`,
  outside this specimen scope).
- **new-member exhibit** (`277` §4e): `strawman24-alias-provides` gains
  `sm_dorc_Package__state_stored_only_in()` with one `: fs` substrate emission and the
  `: user-invariant` colon-line. Status: conductor-PROPOSED grammar, awaiting the human's
  `delta-invariance-line-spelling` ack (`277` §8); exhibited in-code so the ack rides the file.
- **pipe-guard re-XFAIL**: `strawman24-pipe-guard-oracle-converged` re-gains the one-sided `XFAIL`
  file. §9b's un-XFAIL rationale — its only delta was a golden-text fact-label re-key, so it stayed
  live — no longer holds: the `#matched` selector UN-parses at HEAD, a structural failure the XFAIL
  absorbs (a golden-text-only diff would XPASS; this does not). The suite is now
  green-with-FIVE-declared-xfails (the four standing + pipe-guard); verified via a fresh
  `cargo build --workspace` + full foreground e2e, all 126 pass, no XPASS, no live regression.

Implementor flow is unchanged from §9: land the respell, delete the XFAILs, BLESS on a verified
binary, inspect the diff against §4's predicted delta classes (now including the selector re-key) —
never bless-first.
