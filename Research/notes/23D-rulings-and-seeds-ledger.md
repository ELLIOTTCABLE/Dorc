# 23D — round-23 rulings + design-seeds ledger (harvested from 23Z at restructure)

AI-authored, 2026-07-02. When `23Z` was restructured into a lean resumption prompt, the detailed
session rulings and design seeds that existed ONLY there were harvested here — this is the
durable record; `23Z` points at it. Everything human-ruled is marked; everything conductor-
synthesized is confidence-marked and NOT welded unless a signed home is cited. Authoritative
homes outrank this file: `plans/233` (stamped) · `notes/239` (signed) · `spike/CLAUDE.md`
round-23 rulings block.

## §1 The oracle ground-truth (human, 2026-07-02 — binds all future writing)

1. Oracles are JUST SH, often in the same file as the book. The added syntax is STRIP-ONLY: the
   strip removes type-annotations and rewrites `name.check()` → `name_check()`, nothing else;
   its output is runnable sh. Period-names are a semaphore opting into extra Dorc lint/warnings
   — NOT a different language.
2. The argparse-deconstruction is an ANALYZER TRICK, never a language constraint. An oracle may
   contain arbitrary sh — `rm -rf /` ships and wipes root. Oracles are constrained in what we
   ASSUME from them, never in what they may contain.
3. Therefore the check IS the oracle; the stripped whole body is what ships in BOTH lanes (probe
   under structural self-vouch; apply as guard). Lifting an invocation-relevant subset (what old
   rounds dangerously called "verbs") is an optional edge-case gated on the author's sh matching
   the abstract-interpretation constraints, and any lifted form must be BYTE-IDENTICAL TO A
   SUBSTRING of the oracle body. Maybe never worth building ("maybe just ship the entire oracle
   during both probe and apply" — human, unsure). The spike's st-2 check/probe split is
   spike-INTERNAL implementation, not design truth — a build-vs-design divergence to reconcile
   during the build (alongside the standing inv-one-observable text-vs-code flag).

## §2 Plan-surface rulings (human, 2026-07-02 — task-3; supplements the welded rul-attention-honesty)

- **The plan is the code; the code is the plan.** Original vision: `dorc plan >theplan.sh`, an
  *editable file* of what actually needs running. The spirit binds all richer UX: the render is
  the WHOLE runbook, original order (order sacred in display too), line-numbered,
  syntax-highlighted; elided lines present-but-greyed (comment-lines in file form, consistent
  with the r22 byte-floor); guards appear inline as real code with postfixed reason-comments
  (`hork nginx  # unmodeled`); predicted effort/time summary at the end. NO tier-sorted views.
- Tier vocabulary honesty: "elided" reserved for proven; the guard tier reads as expectation
  ("verify" — his simplification); every `runs` carries its reason.
- **Approval semantics:** byte-identity (edits run as-is, no sneaked re-analysis) binds the
  plain-file fallback path, which stays supported as the vibe-anchor. The richer multi-host
  surface (per-host specialization; present → modify-in-UI → re-analyze) is an OPEN deferred
  question. Candidate generalized invariant (conductor, unconfirmed): each per-host executable
  is a deterministic, inspectable function of (the reviewed surface × that host's facts); any
  edit yields a fresh reviewable surface. Binding regardless: no hidden danger; attention only
  via provable elision; everything presented as plain sh.

## §3 Guards-can't-serve rulings (human, 2026-07-02 — task-4)

Seven classes walked (captured stdout · pipelines · admin-read rc · run-delta verbs · loops ·
multi-operand · awkward homes); every fallback is `run` — the list prices performance, never
soundness. Rulings:
- **rc-consumer split DEFERRED pending experimentation** (task #13): whether errexit-IMPLICIT
  consumption blocks guarding is open — the human suspects painful breakage under EITHER
  default. Interim binding posture: guards mint only where NO explicit status reader exists
  (`&&`/`||` operands, `if`/`elif`/`while` conditions, `$?` readers refuse); no pin encodes
  either default. Uncontested floor: a written `|| fallback` site never guards.
- **Refuse-loudly-initially RATIFIED** for awkward homes (background, substitution positions,
  heredoc lines), conditional on default=run + a quiet/conflation mechanism (the r22 why-lens /
  diag-API arc + round-11's root-cause-only doctrine are that machinery).
- **THE ATOMIC-COMMAND AXIOM** ("a perfectly cromulent axiomatic atomic unit of Dorc Doing
  Stuff"): command-disassembly is HARD-DEFERRED, possibly forever. Multi-operand lines guard
  whole-line all-or-nothing; the enrichment path to granularity is the AUTHOR rewriting to a
  loop (command-specific de-looping is mildly an antipattern) — so it all folds under "figure
  out loops eventually" (per-iteration guarding, deferred). The pre-crisis partial-member arc
  (231-1e, per-member self-reach) is PARKED by this axiom.

## §4 Attention-chronology doctrine (human, 2026-07-02 — named, binding)

User attention is cheap in the right-after-hitting-return epoch, expensive in the
thirty-five-minutes-later epoch; Dorc's value-prop includes spending analyzer-CPU to SHIFT
attention-work into the cheap epoch. Consequences: no mid-apply prompts or stops, ever (an
*expected* stop removes surprise, not the return-visit cost); all decisions front-load into the
single approval; late events are report-items only. This is what killed the pause-at-wall/
partial-replan variant of re-observation and is constraint #1 on task #11.

## §5 Elide-half design seeds (conductor syntheses, ~SUSPECT, adversarial pass OWED before any weld)

- **The m×n negative-enumeration is DEAD everywhere** (derived, not assumed): it served only the
  poison-default's silence-clearing; no surviving license consults non-effect enumerations. A
  published oracle never writes `: fs.Path:.is_directory~`-style disclaimers.
- **Three surviving elision tiers:** (1) converged-case total elision — self-consistency, zero
  cross-oracle knowledge (modulo retained-opaque walls; the wall-wave is the recovery); (2) past
  retained CONTRACT-BOUND tools — authored POSITIVE first-order footprints (O(own-effects),
  honest under the horizon), cleared against the book's demand set by DISJOINTNESS; (3) past
  retained PAYLOAD-BOUND tools (apt-class) — derived footprints / waves / guards only.
- **THE VOCABULARY LAW (keep):** *positive conclusions may ride open vocabulary; negative
  conclusions require owned vocabulary or explicit consent.* Tier-2 disjointness is the design's
  one negative-conclusion license; guards/walls need only positives — why the floor never
  touches any of this.
- **Coherence gates disjointness** (the human's catch): the SYNONYM dual of round-17's homonym
  problem — two honest names, one referent; falsehood emergent from the pair, attribution
  target-less; "no shared name" is 233's silence one layer up. Fails toward UNDER-execute.
- **The namespace-ownership convention** (mainline candidate): reverse-DNS kinds have owners; an
  owner honestly guarantees no-synonyms WITHIN their namespace; disjointness concludes only
  within one namespace, never across; Dorc owns nothing but `org.dorc.*` (bootstrap vocabulary,
  adopted by gravity — wish-C survives as no-central-authority; no registry, no arbiter).
- **Grounding bridges** (permissionless tier-2 join): a positive, local, author-side declaration
  — "my `com.me.HorkState:x` is backed by `org.dorc.fs.Path:/var/lib/hork/x`" — is COORDINATE
  TRANSLATION, not kind-equivalence-mapping: after grounding, disjointness is COMPUTED
  entity-vs-entity inside the shared kind, never asserted per kind-pair. Ungrounded kinds have
  no disjointness power in either direction (absence = wall, never = disjoint). Passes 233's
  acid test (a stranger minting a new kind invalidates nothing). Cost: O(own-substrate) lines,
  possibly probe-DERIVED; residue = an "only"-shaped completeness claim about YOUR OWN
  substrate (honest-signable class, converged-vouch trust-tier).
- **Entity-aliasing fence** (the human's second catch): within-kind entity identity is not
  string comparison — symlinks/hardlinks/mounts/normalization are the synonym problem at entity
  granularity. Each shared kind's owner pins its entity-identity semantics; OS-level aliasing =
  named horizon-residue or probe-time resolution (`realpath`).
- **Refiled from the 234 triage:** demand/consumer-anchored poisoning (poison only cells a
  consumer in THIS book reads — precision/burden lever; the crossing-set and `dorc bump` are the
  same backward orientation); provider-equivalence (two providers, one kind) + runtime-traced
  footprint measurement (converged with 236b's measured-footprints; the parked 077/078 arc).
- **Horizon-bounded claims + derivation gradient:** authoritative seed in `notes/238` (first-
  order tool-contract horizon; reactions = one named residue class handled by host-oracles or a
  single disclosed exclusion; claims filled at authoring-time / probe-time / never — one
  mechanism, effort-graded). The carried sentence: *no claim without a horizon; no closure
  authored where it can only be derived.*

## §6 Escape-hatches + fork record (authoritative: `notes/235`)

hatch-isolate (main-mode, per-call-site-every-time, book-exclusive, EXECUTED-but-poison-
suppressed — consent-to-trust-a-running-command; the only knowingly-introduced wrong-elision
ever; strongest containment machinery) · hatch-bump-exclude (bump-only dependency-exclusion;
temporary-but-committable) · hatch-dont-run (dismissed — commenting-out exists). Fork outcome:
oracle-side global claims dead in every form; consent-priced attention gently-no (the
trust-well argument); 236-crosscheck independently converged on the admin one-liner =
fence-sitting data FOR eventual un-parking (task #10).

## §7 Session-process rulings worth keeping (human)

- Convergence terminology: "converged" = state-you-want, known+unknown noise-mutation tolerated
  — never "re-run is a literal no-op." Three-way: mutation-known-but-uncared /
  mutation-unknown / no-mutation-legitimately.
- Tooling never rescues a contract ("if it's only correct when a future build-tool maximally
  guards it, you've described it being incorrect").
- The vacuous-universal test: any authored claim whose quantifier ranges over unattended
  observables is dead testimony (killed the noop-license, the oracle-side fork, kind-coherence
  self-claims — three appearances in one session).
- The social-globality asymmetry (233 line-315): local claims risk yourself; global claims
  license eliding others' commands. Book-scoped consent is socially local (the book is the
  blast-container).
- Firewall-breach discipline (memory'd): when a proposal dissolves a welded separation — lead
  with the breach, price containment first, offer the non-breaching cousin, expect tabling.
