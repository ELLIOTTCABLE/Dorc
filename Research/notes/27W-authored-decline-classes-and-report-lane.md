# 27W — authored decline-classes & the versioned report lane (design close)

AI-authored (Fable, design sitting with the human, 2026-07-18; follow-on to the
`AID-NEEDS`/`27V` sitting). The question that forced it: an oracle builder hit the gap
that a verdict-body's `return 2`s are semantically identical to the engine while
carrying materially different author-knowledge (permanently-unprobeable vs
not-yet-modeled vs interactive vs tool-editorial hazard) — all stranded in code
comments. This note closes the design for v1. Authority: root docs, `AID-NEEDS.md` Law,
`spike/CLAUDE.md` outrank; `27V` is the sibling build-phase this rides into. All
concrete names/spellings herein are STRAWMAN-tier unless the ack-ledger says otherwise;
render particulars ride `27V:rul-output-form-unwelded`.

## §0 — Ack-ledger (what is ruled vs leaned)

HUMAN-ACKED (2026-07-18, typed):
- **`27W:rul-versioned-entry`** (human-improved beyond the ask): the report sink's
  ENV NAME carries the format version — strawman `DREP_V1`; a future format mints
  `DREP_V2` accepting a different grammar; recognized names are permanent once
  published (the `__role`-name posture). No per-line version bytes.
- **`27W:rul-static-first-three-tier`** (try-out tier; may need refinement): classes
  discharge by static analysis by DEFAULT — tier-1 per-arm inventory (oracle alone, no
  book); tier-2 per-site classification (argv value-threads to a reached arm; shown at
  lint/plan time); tier-3 runtime emission as the ONLY-OPPORTUNITY fallback (dynamic
  argv; the shipped argparse reaches the arm at probe execution), deduped against
  tier-2 on (site, arm, class).
- **`27W:rul-emission-grammar-v1`** (qualified by versioning): line-oriented
  `<verb> <class> <free tail…>` emitted to the versioned sink; verb + class
  vocabularies engine-owned, append-only; unknown verb/class degrades to a generic
  author-noted line, never an error.
- **`27W:rul-flagless-selection`** (spike-tier soft): no new flag; verbosity is
  surface-selection (single-site `dorc why N` inlines the arm; zero-arg problems
  report shows class+count+pointer; plan render shows only the reason tail).
  Truncation/inclusion heuristics are render-particulars under `KNOBS:kFLOW`.
- **`27W:rul-strawman-tool-set`**: the canonical example set is `sysctl` (primary;
  write-only trigger keys = the unsound archetype; config-cascade = unmodeled;
  runtime-vs-persisted = the modeling-crutch caution) + `update-alternatives`
  (cameo; `--config`/`--all` = interactive). Chosen deliberately away from
  in-flight oracle work.
- **`27W:rul-report-surface-massaging`** (human-typed, near-verbatim): we are an
  sh-analyzer and emitter — report/why surfaces may read, massage, modify, and
  re-emit shown code (attach a whole adjacent comment regardless of length; omit
  lines that didn't value/CFG-contribute to the affected lines). Attribution
  requires implying authorship and directing repair-effort, NOT byte-for-byte
  inline printing. The byte-floor laws (two-surfaces; attention-honesty) bind the
  plan/artifact planes only and are untouched.

SOFT-ACKED (human, 2026-07-18, "soft ack on both" — build on them; cheap to revisit):
- **`27W:rul-class-starter-set`** (né lean) — closed v1 classes {unsound, unmodeled,
  interactive, hazard}; engine-owned, append-only.
- **`27W:rul-advise-verb-deferred`** (né lean) — v1 ships the `decline` verb ONLY; an
  `advise`-on-covered-arms verb stays a named seam gated on the modeling-crutch
  question (§4): advisory prose can relieve modeling pressure wrongly (the
  sysctl runtime-vs-persisted annoyance is properly TWO CELLS — `#value` /
  `#persisted` — not an advisory note).

LATE HUMAN-TYPED ADDITION (same day, post-close):
- **`27W:rul-report-noise-tolerant`** — gradual-enhancement applies to BOTH parties;
  be a conscientious middle-man while protecting them from each other. The report
  lane tolerates noise best-effort: nothing an author dumps into it is silently
  dropped — unrecognized verbs/classes and free-form lines are retained (sanitized,
  size-capped, attributed) and, under the MOST verbose mode / deepest pull tier, all
  of it prints, so even a cargo-culted barely-thought warning can be caught by a
  curious admin. Default surfaces stay ruthlessly selected; verbosity is the
  admission gate, never existence.

## §1 — The classes and what they route

The primary consumer is the engine's OWN aid routing, not admin eyes; prose is
secondary. Today the engine cannot distinguish no-oracle / deliberate-decline /
covered — the middle state is new information.

- **unsound** — permanently unprobeable (write-only trigger keys; nondeterministic
  reads). Routes: SUPPRESSES write-an-oracle/enhancement nags for the shape (the
  anti-nag: "I looked; stop asking"); `dorc why` renders "the author has ruled the
  question unanswerable."
- **unmodeled** — NYI; a better oracle could cover it. Routes: converts "unmodeled
  command" nags into targeted contribution invitations; honest coverage arithmetic
  in the lint inventory ("partially modeled: declines `-p` as not-yet-built").
- **interactive** — prompts by construction. Routes: headless-mode explanations;
  never nagged toward probing.
- **hazard** — the author's tool-editorial claim (deprecated/discouraged usage);
  the one class aimed at the ADMIN'S BOOK, and the only push-capable one: plan-tier
  warning, attributed frame, capped severity, deduped per book, detail behind pull.
  Class-only in v1 (the author's words reach the admin via show-the-code, §4).

Routing is aid-plane behaviour only: the rc-partition weld is untouched (≥2 stays a
flat sink in every decision table); the license plane never reads a class. A wrong
class steers ATTENTION (a wrong `unsound` silences deserved enhancement pressure) —
attributed to the arm's file:line, aid-plane harm only, accepted.

## §2 — The emission (one spelling, three consumers)

```sh
vm.drop_caches|vm.compact_memory)
   # write-only trigger: writing is an action, not state. the kernel
   # doesn't retain it; "is it already done" has no answer, ever.
   printf 'decline unsound %s is a write-only trigger key\n' "$key" \
      >>"${DREP_V1:-/dev/null}"
   return 2 ;;
```

- Plain sh, config-free, kOOB-clean; NOT dorc-lang surface (human lean, typed: mint
  language-surface only for concepts valueless outside the engine; this one is small
  and broadly beneficial — the emission is a working self-description facility any
  post-offramp harness can consume by setting the variable, the shellcheck-consumption
  posture pointed inward).
- The `${DREP_V1:-/dev/null}` default makes the idiom TOTAL: silently inert without
  Dorc, `set -u`-safe; the probe lane always sets the sink. Strip does not touch it
  (it is working sh, not annotation).
- The emission sits ON the declining path, before the `return 2` — which is what makes
  one spelling serve three consumers: the static analyzer value-threads the literal
  format (tiers 1–2); the probe emits it when dynamically reached (tier 3); an
  off-ramp consumer captures it by setting the sink.
- Report-lane plumbing constraints inherited from the transport design: one record per
  line, short records, PIPE_BUF-atomic (an-marker-atomicity); the free tail is author
  text — sanitized and size-capped at ingestion (an-output-sanitization is the owed
  fence); ingestion never errors on malformed lines and never silently DROPS them —
  unrecognized content is retained and printable at max verbosity
  (`27W:rul-report-noise-tolerant`).
- Tier-3 emissions flow into the whylog durable with the rest of the record stream
  (`27V` §2 contents, amended).

Considered and refused: distinct rcs (erodes the flat-sink weld; collides with tool
vocabularies) · a parallel `cmd__declines()` member (duplicates the argparse; drifts) ·
dorc-lang marks/tokens (valueless post-offramp; the emission is strictly better there —
one honest loss: a dynamically-constructed format string defeats static reading, and
degrades to tier-3; a lint nudge covers it).

## §3 — Static-first mechanics

Tier-1 (always): enumerate arms + classes from the oracle alone — feeds `dorc lint`'s
oracle-solo/coverage surfaces, zero sites needed. Tier-2 (dominant): the existing
argparse-walk resolves a site's reached arm when argv threads statically; the class
lands at lint/plan time attached to the site. Tier-3 (fallback): dynamic argv — the
class is unknowable pre-execution; the runtime emission into the report lane is the
only opportunity, surfaced then, deduped against tier-2 by (site, arm, class). The
decline collapse-evidence (`27V` Lane A) gains an `authored_reason` field carrying the
class + the arm's source span; minting-line threading supplies the file:line.

## §4 — Show-the-code (the prose answer), and the sh-shaped-explanation direction

The author's words reach the admin as CODE, never parsed prose: `dorc why N` on a
declined site inlines the oracle's arm — comments riding along as displayed text, the
emission line visible, file:line attached. Impersonation-proof (a code block is
inherently claimed-by), spam-resistant (authors inflate their why-presence only by
writing more load-bearing sh), kOOB-intact. Under `27W:rul-report-surface-massaging`
the excerpt is a RENDERING: slice to contributing lines, attach the whole adjacent
comment, mark elisions — authorship implied, repair directed, bytes not promised.

Banked DIRECTION (human-typed, exploratory; no build commitment): lean into
show-the-code for NON-oracle-sourced messages too — "we speak sh." In particular,
chronological chains of happenings might best render AS sh — a CFG-shaped explanation
rendered as a CFG (`cmd -a -b || fallback_thing   # ran, rc 1 → therefore ran`),
instead of "line 9, therefore line 16." Open dangers, named not solved: display-sh
must never masquerade as a runnable artifact (copy-paste hazard; the
never-engine-synthesized-sh law binds the EXECUTABLE plane and stays untouched, but a
display plane that synthesizes sh needs its own visual honesty); rides
`27V:rul-output-form-unwelded` + `KNOBS:kFLOW` entirely.

The modeling-crutch caution (standing): before any `advise`-verb ships, every
candidate advisory must answer "should the model just be richer instead?" — the
sysctl runtime-vs-persisted note is properly a second cell, not prose. Someday a lint
nudge ("this advisory looks like it wants a second cell").

## §5 — Build section (rider package for the dispatch)

- Riders into `27V`: Lane A decline-evidence `authored_reason` field · Lane B whylog
  contents include the report-lane stream · the report-lane ingestion (deframe,
  degrade-generic, dedup, sanitize+cap) lands with the records touch-point.
- New work this note owns: sink recognition (`DREP_V1`, name STRAWMAN — the redirect
  target `"${DREP_V1:-/dev/null}"` recognized statically; value-threading the literal
  format string; per-arm inventory + per-site classification) · the nag-routing hooks
  (first-wall/coverage nudges consume classes) · `dorc why` arm-inlining with the
  massaging heuristics (comment-attachment; contributing-lines slice; elision marks) ·
  e2e: one case per tier (static site-classed; dynamic runtime-emitted; oracle-solo
  inventory) + an `expected-why` pin on the inlined-arm render (content/structure
  needles, not byte-golden).
- Oracle-builder guidance (for the active stdlib-adjacent work): classify as you
  author — emit on every deliberate decline you can class; leave silent declines
  silent; put the rationale in a comment ON the arm (it will be shown, not parsed);
  literal format strings (a dynamic format demotes you to runtime-only).
- The canonical teaching example is the §2/`27W:rul-strawman-tool-set` pair; do NOT
  re-strawman from in-flight oracle work.

## §6 — Open / parked

- The class starter-set and the advise-verb deferral want typed acks (§0 flags).
- Author-prose-on-our-surfaces beyond show-the-code: deferred until the pointer +
  excerpt story demonstrably falls short (`26C:feeder-oracle-why-metadata` stays the
  umbrella).
- Hazard-class render details (framing, dedup keying, severity cap) are
  selection-policy particulars — build-latitude under the AID-NEEDS surface policies.
- The sh-shaped-explanation direction (§4) is exploratory: revisit when the why-report
  walker exists and real output can be compared.
