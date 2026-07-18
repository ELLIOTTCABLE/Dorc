> QUARANTINED RAW LANE MATERIAL - 24K cross-model language-design crosscheck (2026-07-05).
> NOT adjudicated findings. The adjudicated verdict is Research/notes/24Kc-language-crosscheck-adjudication.md.
> Anthropic Fable lane, NEUTRAL stance (24Ka): verbatim extract from commit 19df800 on branch ai/24Ka-langreview. Byte-authoritative copy = the branch commit; canonical corpus home pending cherry-pick.
> Archived verbatim from the session scratchpad post-compaction, at the human's direction.
> Do not cite as authority; do not read during future crosscheck skill-ups (contaminates pre-registration).

# 24Ka — Phase 2 corpus observations (running log)

Format: obs-N (lesson-M) file:line — observation. Findings/cleared distilled later in -05.

## From README/DESIGN/IMPLEMENTATION/TODO

obs-1 (L1) DESIGN.md:110,595-641 — substrate lean: "pure-POSIX-sh; not bash/zsh"; kWHICHSH
  lean "dash >=0.5.13" with "testing that dash and our evaluator perform identically" as the
  stated payoff. TODO.md-ADDTL:17 confirms r23 h3 ruling "dash-ish... local expected".
obs-2 (L1,L2) DESIGN.md:76 — the off-ramp is LITERALLY dash: `ssh some-host.tld 'dash -s'
  <myscript.dorc.sh`.
obs-3 (L7) DESIGN.md:541, IMPLEMENTATION.md:289 — dotted role-fnames in both human docs
  (`mycmd.is_converged()`, `hork.is_converged()`); IMPLEMENTATION carries FIXME
  "spelling/details unsettled".
obs-4 (L2) IMPLEMENTATION.md:364-371 — authorship principle: "If we avoid generating code, if
  we hard-avoid transpilation, collation, or restriction, then there's always a particular bit
  of actual-sh, written by a particular actual human". Tension flagged vs strip pass (below).
obs-5 — IMPLEMENTATION.md:380-407 — their own law: "Be sh, or be *very*-not-sh, don't
  half-ass it"; admission at 315-318: "we've evolved a *very* not-spelled-as-sh typesystem."
obs-6 (L11) DESIGN.md:590-592 — "Dorc evolves into a typed-sh dialect, under protest:
  (UNFINISHED)" — the language section of the central design doc is literally unfinished;
  TODO-ADDTL:17: "nothing defines what the superset adds/drops vs POSIX-sh"; "the one and only
  construct I am punting on is eval".

## From USER_STORY.md (primary target)

obs-10 (L2,L7 — THE BIG ONE) USER_STORY.md:250-254,379,193-195 — every erasability claim is
  post-STRIP: "*stripped*, it is a plain `foobar_is_converged()` any shell can run"; "Still
  just sh: stripped, it runs on any POSIX box". The AUTHORED oracle form is not executable sh:
   - dotted fname: parse-FATAL on dash/ash (my main-6 empirics);
   - `dest : fb.Certs = "$1"` (:236-241): raw sh would exec command `dest` (127);
   - trailing marks `cmd args : kind...` (:241,264): raw sh passes `:` + mark as EXTRA ARGV to
     the real command — silent behavior change, not even an error;
   - stage-7 `printf '%s\n' "$1" : service` (:605): raw printf format-reuse prints extra
     lines ":" and "service" — corrupts the emitted data stream.
  So: authored-Dorc is a compiled source language (TS-position: .ts isn't runnable JS);
  stripped-Dorc is the sh. The docs' self-description ("no transpilation", "just sh",
  obs-4/5) contradicts the actual architecture. Books remain genuinely-sh (true erasability).
obs-11 (L1,L2,L7 — corollary) USER_STORY.md:233-235 — "They append to the book's own file —
  oracles and runbooks can share a file." Appending the oracle BREAKS THE BOOK's own dash
  off-ramp: dash parse-dies at the dotted fname (before any book line if oracle is at top;
  after the body but with rc=2 if appended at bottom). bash survives (defines, never calls).
  Substrate=dash + share-a-file + dotted-names: any two hold, all three cannot.
obs-12 (L8 CREDIT) USER_STORY.md:196-199 — guard shape `( check ) || command`: subshell +
  ||-left; explicitly designed to survive the book's set -eu ("an ||-left is errexit-exempt").
  set -u failures inside the check kill only the subshell -> rc>=2 -> falls through to run =
  fail-toward-run by construction. Genuinely robust; consuming-context awareness present.
obs-13 (L7 CREDIT) USER_STORY.md:267-270 — rc partition "0 = named sense holds; 1 =
  complement; >=2 = can't say, always runs" mirrors the grep/diff/cmp convention family
  (0/1/2) — semantics ALIGN with existing sh-culture rc conventions; crash statuses (126/127/
  130) all land in the safe >=2 cell. Well-chosen.
obs-14 (L12) USER_STORY.md:267-270,356-358 — decline idiom `*) return 2` = "declining is
  ordinary control-flow, not an annotation"; refusals carry OOB breadcrumb via
  `>>"$DORC_REPORT"`. Escape-hatch visibility good; BUT raw/no-Dorc execution with
  DORC_REPORT unset: `>>""` fails (and set -u makes it fatal). Blessed idiom should be
  "${DORC_REPORT:-/dev/null}". Small concrete erasability bug in the flagship example.
obs-15 (L1) USER_STORY.md:353 — stage-4 arity gate `[ "$2" = "" ]` is a set -u landmine
  (unset $2 under set -u = fatal, incl. when a colleague calls the stripped fn from a set -u
  script). Blessed spelling should be `[ "${2-}" = "" ]` or `[ $# -le 1 ]`. Stdlib-teaching
  example teaches the landmine form.
obs-16 (L21) USER_STORY.md:260-263,563,604 — kind namespace: "Nobody approves kind names;
  there is no registry. It only has to agree with itself" — but stages 5-7 make kinds the
  CROSS-oracle composition anchor compared by string equality (hork's `package:nginx` must
  match apt's kind); de-facto two-tier namespace (bare stdlib kinds `package`/`file`/
  `service`/`pkgindex` vs author-prefixed `fb.Certs`) is convention, not contract; kind
  collision+evolution semantics acknowledged open (TODO-ADDTL:21). Future-collision setup.
obs-17 (L6,L21) — role-name grammar: is_converged (predicate), predict (imperative),
  touches/reaches (3rd-person verb), resolve (imperative). Inconsistent grammar; the single
  most safety-critical distinction in the language (is_converged LICENSES elision; predict
  NEVER licenses) is carried by two names whose forms don't encode the difference.
  Left-of-dot is overloaded: command-names (foobar.*) vs kind-names (package.*), and kinds
  themselves contain dots (fb.Certs) — role = last segment, name = rest. A command genuinely
  named with a dot (deploy.sh) yields deploy.sh.is_converged().
obs-18 (L10 CREDIT) USER_STORY.md:90-91 — `sm.dorc.*` bootstrap vocabulary with
  "intentionally-invalid TLD, so strawman names cannot leak into reality" — deliberate
  anti-freeze mechanism; ALSO :258-259 "(Spelling settled 2026-07-03 — authoring the
  verdict-function IS the vouching act; no separate vouch syllable exists.)" — they already
  respelled once (tilde vouch-mark killed) pre-users: the strawman label has SOME reality.
obs-19 (L10 COUNTER) TODO-ADDTL:21 — `dest : fb.Certs = "$1"` is "stamped-in-practice with
  the strip pass paying the off-ramp; the formal weld pending" — human-reserved (dq-kOOB) but
  already load-bearing in fixtures. The strawman label is wearing off in exactly the
  lesson-10 way.
obs-20 (L16 CREDIT) USER_STORY renders — plan artifact: byte-preserved lines, elisions as
  comments-with-reasons, guards as the check-then-execute idiom humans write, still a
  runnable script. Strong vs the autoconf floor. ALSO note emitted guard names
  (`systemctl_check`) are illustrative post-strip names.
obs-21 (L3) — paste-survival: book lines paste fine (pure sh). AUTHORED oracle probe lines
  paste into a bash terminal with the trailing mark passed as live argv (silent wrong-argv
  execution of a read-only probe). Dotted fnames paste OK into bash/zsh interactive, fatal
  into dash-scripts.
obs-22 (L14 CREDIT) — the stage 0-7 ladder IS an explicit two-audience ramp (admin: plain sh
  forever; engineer: opt-in roles; kind-owner: rarer still), each rung priced in
  minutes-of-effort and paid in specific currencies. No per-ARTIFACT strictness marker
  (Sorbet-sigil analogue) — tier is emergent from which roles are defined; plausibly fine
  because roles are additive opt-ins, but no ratchet-lint story visible yet.
obs-23 (L9) — no version/epoch marker anywhere in any artifact so far (books bare sh —
  fine; oracles are dialect files with no dialect-version pragma; recognized-idiom set
  unversioned). Watch ORACLE_PROVIDES/KNOBS for any epoch story.
obs-24 (L18 partial-CREDIT) USER_STORY.md:150-153,224-228 — hint machinery names the next
  idiom to write and quantifies the payoff ("an oracle vouching its convergence would elide
  it when converged, and un-wall 1 downstream site") — actionability exemplary (in renders).
obs-25 (L5) USER_STORY.md:604-609 — annotation micro-grammar embeds shell quoting inside
  marks (`: fb.Certs:"$dest".synced` — interpolated var inside kind:entity.property); the
  mark channel has its own grammar (colon-separated triple) with sh-expansion semantics —
  two grammars braided in one line, analyzer-parsed only.
obs-26 — USER_STORY.md:392-397,436-437 — stage-5 stringly `kind:entity` emission flagged BY
  THEM as due to migrate to annotation-typed emission; "the in-band prefix (with its
  `| sed 's|^|kind:|'` dressing) should not be imitated". Self-caught in-band-typing smell;
  migration/codemod story for already-written touches() not stated (L10 watch).
