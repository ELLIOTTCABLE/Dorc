# 24M — language-design rulings: the 24K/24L remaining-ledger, closed (minus two lines)

AI-authored (Fable, design-synthesis/advisor session), 2026-07-07. The durable stamp of the
human's typed rulings (remote, this date) closing the language-design remaining-ledger that the
24K crosscheck (`24Kc`) and the 24L proposal left open — delivered against this session's
walkthrough of "what can't change once a stdlib of velocity is encoded." The walkthrough itself
was in-chat (ephemeral); this note is its durable home, minted for conductor handoff.

Authority layers, kept distinct throughout: **HUMAN-TYPED rulings** (binding) ·
**conductor-derived consequences/interpretations** (marked; nack freely) · **OPEN items**
(explicitly awaiting the human — §3; nothing in §3 is settled by silence).

---

## §1. The rulings (HUMAN-TYPED, 2026-07-07 — binding)

### rul24M-version-comment
A **version-comment, not a file-extension, gates non-POSIX syntax extensions.** Spelling
specifics unreserved ("meh") — probably a simple `# dorc-lang/v1` within the first ~10 lines;
the conductor may firm the exact grammar, the SHAPE is ruled.
- *Conductor-derived annotation (kOOB):* this is, as written, the redline's "comment-parsing"
  — sanctioned as a **closed set of ONE**. It is not a precedent: eol-comment annotations
  remain rejected (human, 24L dialogue); any second comment-parse is a fresh human ruling.
  KNOBS `kOOB` owes this dated marker.
- *Interpretation CONFIRMED (HUMAN-TYPED, 2026-07-07):* the marker gates **syntax** (binds,
  marks, any non-POSIX construct), not **name-semantics** — `__dorcism`-named functions are
  POSIX-legal and are recognized in unmarked files too (required by rul24M-typeless-floor).
  The human's sharpening, eyes-open: **the names are not lang-versioned and structurally
  cannot be** — name-recognition is a permanent, unversionable compat surface; if retroactive
  changes to it are ever needed, they will have to be contended with directly (no version
  escape exists). Ruled the right path regardless.

### rul24M-typeless-floor (stated as a design-RULE, not an implementation)
The typeless-oracle floor is **IN** (the anonymous/auto-minted cell), restated as law:

> **A user must always be able to cause their own command to elide — when it is the
> topologically-first changed command — without learning the typesystem or using anything
> other than POSIX-sh.**

- 24L §2–§7 remains the mechanism spec (auto-cell, four privacy fences, gating verification
  errand); this ruling stamps the *goal* the mechanism must satisfy, so the rule outlives any
  respell of the mechanism.
- The "topologically-first changed" qualifier is the honest wall-topology bound: below someone
  else's wall the floor promises guards, not elision — unchanged law.
- 24L's *location-gating* half (typed constructs illegal in books) is **superseded**: the
  human keeps full-dynamic-range oracles writable in-book (no Java-esque
  file-ceremony); the version-comment (rul24M-version-comment) is the gate, per-file, not
  per-location. Share-a-file lives, marker-gated; the erasure/off-ramp doc language scopes
  accordingly (unmarked book = plain sh; marked file = strips to plain sh via `dorc strip`).

### rul24M-bare-dorcism-names
The dots die, and there is **no prefix either** (eyes-open: "don't much care if it's
antisocial, I'm too much of an aesthete"). **`munged_cmd__dorcism()`-named functions are
assumed adherent to the spec.** Worst-case mitigation, reserved: choose slightly sh-unusual
role names (the ruby-esque predicates precedent) calibrated against real-world collision data
(GitHub hits) when finalizing role vocabulary.
- *Priced residue, recorded:* coincidental capture of an innocent user function matching
  `<munged-cmd>__<role>` is accepted, not prevented. Standing mitigations: the `__`
  separator's rarity in wild sh; role-name diligence above; the loud-friend law (a captured
  function misbehaves loudly, never silently); the reingest-collision floor (refuse-and-run)
  stays.
- *Conductor-derived consequence:* kind-keyed owner functions munge their kind's dots the same
  way (`sm.dorc.Package` → `sm_dorc_Package__resolve()`); per-TOOL and per-KIND families share
  the flat function namespace, disambiguated by role (touches/predict/is_* are tool-keyed;
  resolve/reaches kind-keyed). Long names accepted (<10%-of-authors surface).

### rul24M-reverse-dns-kinds
**Reverse-DNS kind names are MANDATORY. Minimum two dots; fewer draws a warning** (hardcoded
exceptions possible if ever wished; doubted). **Stdlib = `sm.dorc.TypeName` for now** —
deliberately invalid TLD, globally greppable for the migration whenever a real domain is
bought. A valid-TLD lint: maybe, doubted.
- *Conductor-derived consequences:* (a) every bare fixture kind (`package`, `service`, `file`,
  `pkgindex`, `firewall`, `grepmatch`, …) re-keys to `sm.dorc.*` **in the respell pass** — the
  same single corpus-churn/bless session, not a second one; (b) USER_STORY's `fb.Certs`
  exemplar violates two-dots and rewrites in the human-owned doc queue; (c) 24L auto-kinds
  stay engine-internal/unnameable (fence-unnameable) — the two-dot rule doesn't apply to them,
  but their reserved spelling must be un-collidable with any legal reverse-DNS name (spec
  obligation on the floor build).

### rul24M-kind-unify-owed
**A unify-or-similar mechanism is OWED** — the closest a collaborative, registry-less system
can ever get to kind 'removal' or 'rename'. **No mechanism now**; its necessity for edge-cases
and ecosystem growth is acknowledged and banked.
- *Conductor-derived seed linkage:* the natural future home is the parked cross-kind
  co-reference residual (`24C` strain-coreference-crosskind; ORACLE_PROVIDES
  grounding-bridges) — kind-unification is co-reference at kind granularity. Post-trial
  design work; explicitly not this spike.

### rul24M-rungs-default (HUMAN-TYPED, 2026-07-07 — closes open24M-rungs-default)

> **An unmarked verdict-function reads as full-license — guard and elide at its own sites —
> permanently. Any future rung machinery arrives as opt-down spellings on the withholding
> side; it never re-reads unmarked functions.**

Rationale (human's, near-verbatim): in gradual-enhancement, "making the command not run when
it's fine" is by far the default case; the gradient runs YOLO → careful-but-costs-me, so **no
annotation = maximum value, minimum-defensive is the correct pin**. Distinguish this sharply
from *things Dorc does for you*: Dorc's own inferences about the WORLD stay maximally
defensive (silence=wall, ⊤⇒run — all unchanged). This is an inference about what the *author
is saying*, and it enables Dorc to do *as much as you may want*, in a non-broken way — not
broken behavior (the under-execute class).

**rul24M-do-the-most-short-of-broken (general rule-of-thumb, HUMAN-minted in the same
ruling):** *"We do 'the most' for you that we CAN do, short of cases where doing more may
introduce genuinely BROKEN behaviour that nobody in their sane mind would want. 'Most people
want, but not everybody' ⇒ not broken; 'nobody would want' ⇒ broken."*

*Conductor check (invited; verdict: the choice is sound, one scoping fence recorded so the
rule-of-thumb is never over-cited):* the rule governs reading an author's OWN speech-acts
about their OWN tool's sites — authorship is the opt-in, and the wrong-vouch exposure stays
own-line, priced, attributed. It does NOT extend to cross-author trust-transport: the
survival tier stays double-gated (rul24-mode-gate untouched) precisely because there the
claim-subject ≠ blast-subject and the bite is "nobody would want"-shaped. Checked against the
welds: no collision — an unmarked *function* is an authored act, not the silence that
licenses nothing (absence of the function remains the wall); the elide still demands
probe-measured convergence + ambience + reproducible observables regardless. Coherence: this
ruling and rul24M-typeless-floor point the same direction (shave the ceremony between a new
user and value); welding the opposite would have contradicted the floor — the human's own
counterthesis, agreed.

Consequences: **P5's license story is unblocked** — stdlib verdict-functions are authored
under a permanent, stable reading that no future ladder can re-read; kCONTRACT-RUNGS
(single-vs-ladder) stays open for trial evidence *with its default now pinned*; KNOBS
kCONTRACT-RUNGS owes a dated marker (conductor-owed, §4).

## §2. The governance stance (human-asked "ack?" — ACKED, and recorded)

**There is no minting.** First use of a kind name is its creation. "Ownership" is not a thing
Dorc can see — it is somebody using a DNS name they pay rent for, nothing more. Enforcement is
social: ask nicely that authors use namespaces they actually control; **reserve the right to
ship hardcoded namespace fixes later if issues arise**; accept that without a registry we are
toothless — which is consistent with, and the price of, the welded no-registry stance.

Precision restatement of the resolver point (the prior phrasing was the advisor's error):
nothing proposed — or could propose — verifying resolver *authorship*; agreed impossible. What
exists (built, Stage 5A) and stays is narrower: **within one loaded analysis unit, two
resolvers claiming the same kind = refuse-both**. That is coherence-enforcement among the
files actually loaded, never ownership-enforcement; a foreign resolver for "your" kind, loaded
alone, is undetectable, permanently. The earlier "authors may not mint a resolver for a kind
they don't own" is struck as unenforceable; the social stance above replaces it.

Likewise the advisor's duplicate-mint warning is **DROPPED** (no mint-event exists;
same-kind-across-files is the contribution model working). Surviving mechanical checks, total:
the <2-dots warning (rul24M-reverse-dns-kinds) + the within-unit duplicate-resolver refusal.

## §3. OPEN — awaiting the human (ONE line remains; not settled by silence)

- **open24M-ack-poison-marks** (gates the respell, small): ACK and POISON bare-marks are law
  (strip-fidelity) with zero corpus occurrences — prune from the grammar, or deliberately
  exercise in the stdlib; decide at the respell so untested grammar doesn't ship into the
  stdlib era.
- ~~open24M-rungs-default~~ — **CLOSED 2026-07-07 → rul24M-rungs-default (§1).**

## §4. Conductor-owed riders (no human ruling needed; one line each)

- The respell commit carries the eyes-open sentence: the `/1` mark grammar ships knowing the
  24K polysemy critique; a mark respell is a `/2`-dialect concern (the version marker makes
  this cheap for the first time).
- `# dorc:` artifact trailer comments: declared human-facing/UNSTABLE at the golden-churn
  moment; machine consumers get the `dorc-records/1` lane (262 §7 handoff note — the
  additive-keys discipline is the pattern).
- P5-brief additions (extends `252 §9` memo-2): one blessed `command -v` mark polarity
  (currently establish in one fixture family, observe in another); the UNK/`DORC_REPORT`
  line-format either adopts additive-keys or is declared unstable; kinds per
  rul24M-reverse-dns-kinds; names per rul24M-bare-dorcism-names.
- KNOBS: the kOOB closed-set-of-one marker (§1); kTYANNOT's containment sentence rewrites
  (marker-gated, not location-gated).

## §4b. Verification addenda

- **name-length errand DONE (2026-07-07, human-directed; Opus, Kagi + WSL empirics).
  Verdict: +SURE zero realistic risk** — reverse-DNS-derived function names to ~300 chars are
  safe by enormous margin everywhere.
  - *Spec:* POSIX.1-2024 imposes **no length limit** on shell NAMEs or function names — XBD
    §3.216 "Name" is character-class + no-leading-digit only; XCU §2.9.5 says fname is a NAME,
    nothing more; wording byte-identical to Issue 7. The plausible-sounding limits don't apply:
    {ARG_MAX} is exec()/environment-scoped (def/call never execs); {LINE_MAX} scopes to
    text-processing utilities, not `sh` script parsing (and its 2048 floor is ~7× our worst
    case anyway); {NAME_MAX} is filenames. No FUNC_NAME_MAX-style constant exists.
    (pubs.opengroup.org/onlinepubs/9799919799/ — basedefs V1_chap03, utilities V3_chap02,
    limits.h.)
  - *Empirics (WSL2 Ubuntu 24.04 + git-bash msys2):* dash 0.5.12 · bash 5.2.21 (+ --posix) ·
    zsh 5.9 · busybox ash 1.36.1 · msys2 bash 5.3.9 — ALL pass define+invoke at 64/256/1024/
    4096/65536 chars, a **1,000,000-char name**, and a real script file whose first physical
    line is ~1MB; silent truncation explicitly falsified at every length (N−1-prefix probe
    fails to resolve); the 214-char `__is_converged` munged-kind shape executes everywhere.
    No ceiling found in any shell. Not measured (absent from image, no-install constraint):
    ksh93/mksh/yash — ~SUSPECT identical; cheap spot-check if ksh support ever matters.
  - **ca-munge-charclass (the actionable catch — respell-brief line, P5-brief line):** the
    real failure dimension is **character validity, not length**. The reverse-DNS→NAME munger
    MUST handle: (a) **leading-digit first labels** — `3com.example.Foo` munges to an INVALID
    name (leading digit); (b) **DNS hyphens** in labels (`my-corp.example.com`) — not NAME
    chars, need transliteration; (c) IDN/UTF-8 labels — ASCII-fold or refuse. These bite at
    any length and are where a naive munger actually breaks.
  - *ca-strict-set:* bash/zsh accept extra chars in fnames as extensions; dash/busybox hold
    the strict letters/digits/underscore set — stay strict for cross-shell safety.
  - *ca-export-f (distinct, not our use-case):* `export -f` serializes functions into the
    environment where ARG_MAX/MAX_ARG_STRLEN do apply at the next exec — an exec/environment
    limit, never a naming limit; plain define-and-call (the oracle contract) never crosses it.

## §5. Queue effects (for the resequenced r24 queue, LIVING_STATUS)

- The **kind-rule is now RULED** (was absent from the P5-blocker list — restored here): P5
  blockers read: respell + version-marker + ~~rungs ruling~~ **(CLOSED —
  rul24M-rungs-default)** + the dq-kOOB stamp (24L's pending-stamp is substantially
  discharged by rul24M-typeless-floor + rul24M-version-comment + the location-gating
  supersession; the formal stamp line is the human's).
- The **respell pass** now folds, in one churn: dot-death → bare `__dorcism` names
  (rul24M-bare-dorcism-names) · the version-comment (rul24M-version-comment) · kind re-key to
  `sm.dorc.*` (rul24M-reverse-dns-kinds) · touches() typed-emission migration (already folded)
  · loud-friend law · `dorc strip` first-class · open24M-ack-poison-marks once answered. ONE
  bless-and-inspect session at the end, as planned.
- USER_STORY/doc queue (human-owned): stage-3 exemplar becomes the typeless-floor version;
  `fb.Certs` → a two-dot spelling; share-a-file retold marker-gated; erasure-semantics
  identity language.
