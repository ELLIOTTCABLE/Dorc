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
