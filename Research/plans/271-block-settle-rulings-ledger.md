# 271 — Block-settle rulings ledger (living)

AI-scribed, human-adjudicated (design-rubber-duck sessions, begun 2026-07-10). This is
the **single evolving durable** for the `270:block-settle` design-pass: rulings,
direction, and the surviving strawman spellings, accreted as the human works through the
session task list below. Explicitly sanctioned as a living document by the human
(2026-07-10); plan-tier per `plans/23D` precedent — update in place, annotate
supersessions, never silently delete a ruling.

Authority: root docs and human-TYPED rulings outrank this; this ledger *records* typed
rulings, it does not create them — an entry here without "(typed)" is conductor-drafted
direction awaiting ack (silence ≠ ack). Naming discipline per `270` §1: hyphenated
full-word slugs; entries minted here are referenced elsewhere as `271:<slug>`.

Sibling deliverable: the **entity-algebra design note** (the `24C:rul-selector-pre-stdlib`
obligation, tasks 1–2 below) is its own document when authored; this ledger carries the
rulings that shape it, not its body.

## Session task map (mirrors the conductor task list)

1. adj-entity-algebra, spelling half — the authored entity/selector mark grammar.
   CLOSED 2026-07-10: all six sub-rulings typed or dictated (see "The entity-algebra
   spelling direction" below); mechanical close-out rides task 12.
2. adj-entity-algebra, seams half — the two `24S:A7` reservations. CLOSED 2026-07-10
   (three typed acks + the "family" minting; see the seam rulings below).
3. adj-trichotomy-spelling (né `24S:A3`) — kind-owner per-axis context topology;
   the deliberate kOOB-redline reading.
4. adj-axis-vocabulary (né `24S:A1`) — ratify v1 = {user, fs-view} + ρ,
   versioned-closed, engine-owned. CLOSED.
5. wrapper context-function spelling — strawman the `24S` §2b surface
   (peel/axes/ρ-transform/self-vouch) in actual sh.
6. carrier declaration spelling — `24T` pin1 (which-arg-is-code, stdin shapes,
   argv-binding, carrier context record).
7. adj-capture-claim (né `219` fork-capture-claim-type) — probe-captured stdout:
   OutClaim-observation vs new claim-type.
8. adj-survival-flag-outcome (né `24S:A2` + the `--trust-footprints` rename) — the
   central trust ruling: flag gates the OUTCOME; outcome-centric name.
9. adj-stopping-point (né `24T:P-A6`) — typed ratification of the derivable analyzer
   ceiling (const-resolvability × no-escalation × no-cross-host).
10. adj-small-homes — systemd rc=255 vs the ≥2 partition; the
    probe-bodies-proved-non-mutable effect-check's home. CLOSED.
11. adj-payload-pins pre-read (né `24T:P-A1`/`P-A2`) — optional tail; formally owed at
    block-context implementation-planning.
12. 1-series close-out — amend the `24P` specimens (hash introducer; brace-alternation
    if firmed) + author the entity-algebra design note. UNBLOCKED (task 2 closed);
    ready to fire; this ledger carries everything the note needs.

## Rulings

### rul-axis-vocabulary-v1  (task 4; 2026-07-10; typed)
`24S` §3a/§7-A1 ratified as posed: v1 coordinate axes {user, fs-view} + reserved-never
{host}; ρ components {env-vars per-variable, positionals, cwd}; vocabulary engine-owned,
dialect-versioned; users never mint axes; expressibility-clause decline for wrappers that
exceed it. Rider (the human's typed shape): **fs-view is soft-deferred** — explicitly
named-and-shamed as "a hard enough problem that it could take its own entire round to get
right"; the spike does NOT attempt to get it Correct. Re-entry condition: fs-view comes
back in scope only if *specifically* needed to exercise r27 work (a Dorc language
feature); failing that, the spike-minimum is a Very Simple Fs Stub — effectively
{user, fs-straw}. The `24S` §3b honest ladder stays reserved-not-built, unchanged.
Consumers: the entity-algebra design note (context-qualifier seam only), wrapper-peel/
wrapper-sudo briefs, stdlib kind declarations.

### rul-rc-partition-stands  (task 10a; 2026-07-10; TYPED — "your analysis of the rc
seems correct. I buy your arguments"; riders r1–r3 bundle-acked therein)
TODO.md:23 ("systemd's 255-as-exit-fail? should we replace our >=2 to match?") answered:
**keep `rul-rc-partition` unchanged** (0 = named sense holds / 1 = complement / ≥2 =
confused-runs; spike/CLAUDE.md).

**rul-zero-one-inversion-pair** (the human's sharpening, typed same date; his coinage
"ruling-0/1-inversion-pair", slug-normalized): the verdict-bearing statuses are exactly
{0, 1}, an inversion-pair — a positive-sense member licenses on 0, a sense-flipped
member licenses on 1 via the mechanical flip. These two are the ONLY statuses that can
ever carry a verdict — hence the only statuses that can ever produce a skip (elide /
guard-pass / any non-run) — in any current or future decision table. ≥2 is
meaningless/error/warn/anything-else, is NEVER inverted under any future design, and
can never license anything: a ≥2 answer always falls through to run, un-verdicted.
systemd's `ExecCondition` ternary (0 proceed / 1–254
skip-cleanly / 255 fail; `24R:repurp-finding89`) is cited as convergent-evolution
*validation* of the ternary check-contract shape, not adopted: its layout puts every
machinery-mintable status (126/127, 128+n, timeout 124/125) inside the verdict-bearing
complement region, which is exactly what our flat ≥2 sink exists to prevent — and any
sense-flipped family member (the mechanical-inversion glue in rul-rc-partition; future
name-extensions like is_noop) would turn a machinery-minted status into a licensed SKIP
under that layout. Riders: (r1) the ≥2 region stays semantically FLAT forever — no
status inside it may acquire distinct verdict meaning; decline-vs-crash distinctions
live in the report/diagnostic lanes (runner MAY narrate well-known tool-rc's in why-lane
diagnostics); (r2) exterior coherence is a delegation-body concern — ordinary
`case $? in` remap arms; stdlib quality-bar line: delegating to a tool with a documented
non-test-shaped exit vocabulary (LSB status, terraform -detailed-exitcode) requires an
explicit remap arm (lintable); (r3) Dorc-as-tool outward contracts (`dorc plan
--exit-code`, dorc-run) are separate per-surface decisions, unbound by this partition.

### rul-effect-check-home-typeless-floor  (task 10b; 2026-07-10; TYPED — "I ack your
10b arguments; let's roll it into the typeless-floor")
The owed static effect-check of verdict-function bodies (TODO.md:43; `24R` §0a
"machine-assist the contract") homes as a **typeless-floor brief rider**
(`270:block-rebuild`), not wrapper-peel: same seam (verdict-bearing providers threaded
into classify as data — the licensing chokepoint; the effect-check is one more predicate
at that gate), lands BEFORE block-context widens the shipped-probe surface
(probe-outside + whole-ρ replication + per-inner payload checks), and keeps the
referendum-carrying wrapper-peel brief lean. **Sizing rider (typed): low stakes — build
only if it falls out nearly-free at that seam; punt-empowered, zero guilt** ("not worth
the brainspace" beyond that; DX defaults out-of-scope for the spike). Trust framing
(three layers): the static check catches the shell-visible accident class ONLY (stray
writes/redirects/mutators in the body — the paranoid-author bug class); the
tool-invocation interior stays vouch-tier forever (frame problem — the oracle self-vouch
is not eliminable); the `plans/077` runtime observe backstop stays reserved as the
someday third layer, unaffected. Rider details: the sanctioned OOB write lanes
(`$DORC_REPORT`-class breadcrumbs) carve out as non-mutative-for-this-purpose or every
stage-4 refusal idiom fails the check; tmpfiles NOT blessed in v1; the rider brief
verifies against `24C`'s actual classify/effect shape rather than assuming it.

### rul-no-mutating-guards  (task 10b; 2026-07-10; TYPED, emphatic)
One bar, permanently: a verdict-function body that fails the non-mutation proof lifts
NOWHERE — not as a probe and **not as an apply-time guard**. The human, verbatim: "We
*do not* insert *mutation* that we just proved to be mutation, out-of-order, in
not-user-spelling, into apply bodies. Extremely nope." (The harm-0 argument: apply-phase
mutation is the user's own lines; a guard is Dorc-inserted code, and a mutating guard is
mutation Dorc authored into the apply that the bare book never had.) Kills the two-bar
option ("good enough to guard but not to probe") forever.

### rul-proven-mutation-fails-fast  (task 10b; 2026-07-10; TYPED, gentle-lean →
consistency)
Disposition table for the effect-check, superseding the earlier draft's uniform
no-lift: **proven-mutates ⇒ genuine fail-fast** (plan-time, before network, loud —
dictate-tier), *whatever the provability source* — structural (shell-level: a real-path
write-redirect; permanent, corpus-independent) or oracle-sourced (a loaded oracle
declares a body command mutative; corpus-RELATIVE). Conductor-framing within the
ruling: this mints no third hard-error category — a function whose __role name declares
read-only-answering and whose body provably mutates is an instance of the standing
"declarations that genuinely contradict each other" category. The corpus-flip edge
(a previously-green book hard-errors when a newly-loaded oracle proves a latent breach)
is ACCEPTED for consistency — justification on record: the flip discloses a standing
violation (the book was always mutating in the probe lane), not a regression. The two
provenance classes stay SEPARATED in diagnostics/docs (the human's gut, typed):
structural proofs are permanent and near-warrantable; oracle-sourced proofs name the
proving oracle in the dq and are never marketed as coverage. ~~Residual class: unprovable
stays NO-LIFT~~ — **SUPERSEDED 2026-07-10 (human soft-hold: "every command in an oracle
must also have an oracle" is chicken-and-egg AND gradual-enhancement hostile) by
`rul-unprovable-rides-the-vouch` below.**

### rul-unprovable-rides-the-vouch  (task 10b; 2026-07-10; TYPED — "Okay, acked.
Durable" — replaces the superseded no-lift residual)
The **unprovable** region of a verdict-function body (a foreign command with no loaded
effect-knowledge; a ⊤-valued redirect target) **ships on the authored vouch, exactly as
today** — probe AND guard, no lift-refusal, no error. Grounds: the oracle lane's
probe-license was always the VOUCH, not proof (DESIGN: "vouched-safe-to-run … either by
the author, or by provable Dorc-provenance" — two license sources; the under-approximate
"better to ship no probe at all" mandate governs the Dorc-provenance path, i.e. lifting
admins' in-book guards, which stays proof-gated and is NOT loosened by this ruling). The
effect-check is therefore **falsification-first, never a completeness gate** (`24R` §0a
says machine-ASSIST): it fail-fasts on proven contradiction
(`rul-proven-mutation-fails-fast`), it may positively report fully-verified bodies, and
third-party silence never revokes a first-party license. Riders: (r1) each unprovable
command earns a hint-tier note (author-facing lint surface primarily, kWARN-rich):
"outside machine verification; your vouch carries it" — with the standard
enhancement-pointer shape (a read-blessing for that tool would verify the region);
(r2) verified-vs-vouch-carried is a REPORTABLE distinction per body — the honest answer
to watch-machinery-raises-disclaim-burden is "Dorc verifies what it can and tells you
which parts rest on your word," never warranty language; (r3) the filter-tool cohort
(jq/awk/sed/cut in check pipelines) is the expected common unprovable case — the
stdlib's existing read-blessing species (the stage-1 "vouched probe-safe reads") covers
it opportunistically, but is never a precondition for an oracle to lift;
(r4) **vouch-scope-is-the-body-never-the-tool** (the human's sharpening, typed same
date, né "temporary-other-vouches(???)"): the body-vouch is a claim about a REGION OF
SH — "everything I typed inside this contracted non-mutative block" — never a claim
about the command families it happens to invoke. It therefore mints NO command-family
fact: an unmodeled command carried in one oracle's body stays ⊤ everywhere else — in
books it remains a full poison-wall (unmodeled-drives-to-⊤ law unchanged), and it gains
nothing in any other oracle's body. Non-transitivity is automatic rather than ruled,
because no fact about the tool was ever asserted to transit. Exactly one vouch reaches
OUT of its file: a tool's own oracle reaching that tool's book-sites — and it reaches
precisely because it IS a claim about the command family, argv-keyed. Two claim-species,
two scopes; they never convert into each other.

### The entity-algebra spelling direction  (task 1; 2026-07-10; consolidated — each
sub-entry marked typed or awaiting-ack)

The coordinate, everywhere it appears (facts, backings, footprints, disjointness), is
the **flat three-place `(kind, entity, selector)`**.

- **rul-coordinate-shape-flat-three-place (typed).** Flat ratified; `plans/17N` §4's
  recursive-struct lean declined as coordinate shape — its motivators (cross-kind
  handles, aliased names) live in the owner-declared function mechanisms *between* flat
  coordinates (`resolve()`, `reaches()`/`manifest()`), so the coordinate stays a name.
  Riders: later structural expansion must remain language-design-easy, and the bare
  selector-less form permanently means "true / occupied / whole-entity". Engine hedge:
  `SelectorId` stays opaque/interned; every selector comparison lives behind ONE
  choke-point function (`selector_covers`-shaped), so future structure lands in one
  function, never a re-key.
- **rul-selector-introducer-hash (typed, PERMANENT).** `#` introduces a selector:
  `sm.dorc.Service:"$svc"#enabled` (the HTML-anchor / commit-ID "narrow to this
  specific thing" vibe). Quoting stays supported where charsets collide, exactly as
  under the old `.`. Rides the corpus-respell churn (marks, `:?` observes, render/
  why-lens fact-labels, records); `notes/24P` specimens amended conductor-side at
  1-series close; USER_STORY's `.synced`/`.matched` staleness accrues to the punted
  root-doc pass.
- **rul-emission-selector-on-mark (typed).** `touches()`/`reaches()` claim a cell via
  the selector riding the trailing mark on the emitting statement —
  `printf '%s\n' "$1"  : sm.dorc.Service#active` — applying to every line that
  statement emits; emission lines stay raw entities (no output dressing); a
  selector-less mark = whole entity (the floor). Multi-cell claims get a list shape:
  brace alternation `: sm.dorc.Service#{enabled,active}` (direction; exact grammar in
  the design note).
- **rul-binds-entity-only-provisional (typed, SOFT — ~GUESS both sides).** Binds name
  entities, never cells; facts about cells attach via marks on probing/emitting
  commands. Door open on counterexample; nobody worried.
- **rul-kind-or-selector-is-a-behaviour-choice (typed).** Minting a KIND vs a SELECTOR
  is a user-facing behaviour selection, not ontology — "the point of having both types
  of thingie IS to have differing behaviour for them." The menu: **kind = isolation**
  (cross-kind disjoint by construction, `reaches()`-bridged only; claims spare everyone;
  no coupling; claim-only usable — logs, caches, scratch state); **selector = coupling**
  (same-entity collide-by-default; subscription to the entity's kill-traffic in both
  directions). The behaviour-menu teaching rides the stdlib quality-bar checklist.
- **rul-selector-disjointness-dialect-scoped (TYPED 2026-07-10 — dictated "as ruled
  for now, and see where the spike goes with it"; spike-provisional, revisit on build
  contact).** The rule for when token-inequality may be read as
  cell-disjointness (the survival license — the one lane where string-noise fails
  toward under-execution rather than toward run):
  - **Minting.** A selector token enters the system only as an annotation on a runnable
    measurement line — verdict-probe marks and `:?` observe marks both mint;
    claims/touches never mint. No dislocated declarations exist anywhere: a kind's
    cell-structure is the projection of what loaded oracles measure. (The stage-5
    principle, one level up: a fact's backing IS what its probe reads; a kind's
    vocabulary IS what its probes measure.)
  - **`dialect(source, kind)`** = the selector tokens that source's measurement-marks
    carry for that kind. No global per-kind vocabulary exists.
  - **Comparison.** For same-entity coordinates: a claim SPARES a backing iff
    claim-token ∈ dialect(the backing's minting source, kind) AND claim-token ≠
    backing-token. Everything else COLLIDES — selector-less claims, unminted tokens,
    and cross-dialect tokens are all ⊤-selector (collide with every cell of the
    entity). Entity-level disjointness across different entities/kinds is unchanged.
  - **Properties**, each load-bearing: empty world ⇒ byte-identical to entity-granular
    HEAD · noise fails safe on BOTH sides (claim typos/synonyms collide; a probe-side
    typo makes the honest claim-token out-of-dialect ⇒ collide) · monotone (a newly
    loaded source never alters comparisons against other sources' backings) · no
    self-licensing (claims cannot mint; probes govern only their own facts' sparing) ·
    **subscription semantics are native** (a cell hung on a shared kind is
    auto-subscribed to the entity's kill-traffic — a foreign claim can never spare your
    backing unless your own file mints its token; a source's dialect IS its
    kill-surface control, each minted token a kill opted out of; this dissolves `233`
    hard-1 — the enumerate-the-future burden relocates from the kind's incumbent
    authors, who cannot know, to the property's extender, who can — and dissolves
    `24F` §10 alternative-6 — its default is the collide-default, and the
    invalidation-basis declaration is exactly the complement of the minted dialect) ·
    attributable (every dialect member cites its minting measurement line; the
    unknown-token hint renders the backing source's measured cells with those lines) ·
    DST-clean (dialects are a pure function of loaded oracle text).
  - **Worked minimum:**

    ```sh
    # dorc-lang/v0.1
    # a kind's dialect is minted by its measurement lines:
    systemctl__is_converged() {
       case "${1-}" in
       enable) systemctl is-enabled --quiet -- "${2-}"   : sm.dorc.Service:"$2"#enabled ;;
       start)  systemctl is-active  --quiet -- "${2-}"   : sm.dorc.Service:"$2"#active  ;;
       *) return 2 ;;
       esac
    }
    # dialect(this source, sm.dorc.Service) = {enabled, active}; so its restart claim —
    systemctl__touches() {
       verb="${1-}"; shift
       case "$verb" in restart) printf '%s\n' "$1"   : sm.dorc.Service#active ;; esac
    }
    # — spares an #enabled-backed fact (in-dialect, unequal). A foreign dialect only
    # ever collides; hanging a cell on a shared kind is subscribing to its kill-traffic:
    scan_cve__predict() { cve-tool --check -- "${1-}"   :? sm.dorc.Package:"$1"#cve_clean ;}
    ```

  - **Fences / residues, named:** **ghost-ack-mark** — "acknowledge a cell without
    touching or measuring it" (the spare-opt-in for expensive-probe subscribers; the
    channel-publisher noise mirror, e.g. a cve-db-refresh whose `#cve_clean` claims
    over-kill foreign probers' unrelated facts) — stays UNBUILT; its absence fails safe
    (over-kill / check-tax, never wrongness; `kPROBING` banding governs expensive
    re-scans); reconsider only on field evidence · same-token-divergent-meaning is not
    addressed here by design (adjudicability-list tier, `270:adj-adjudicability-list`) ·
    physically-overlapping cells within one source's dialect are inherent to narrowing,
    differential-discharge-testable (mutate via the claiming verb, watch the sibling
    probe flip) · engine cost: per-(kind × source) dialect sets + backing provenance
    threaded into the comparison (-GUESS cheap given the ProvId arena); authored
    surface: zero new · this precedent does NOT auto-generalize to task 3's trichotomy
    (there may be no measurement line for "user-invariant" to ride; adjudicate task 3
    on its own merits).
  - **Consumers:** the entity-algebra design note; entity-algebra-rebuild
    (`selector_covers` implements this rule); stdlib quality-bar (behaviour-menu +
    remap/dialect teaching).
  - **Amendment (typed 2026-07-10, task 2):** the ruling's "source" is **the family**
    (`rul-family` below) — name-derived, NEVER file-derived. `dialect(family, kind)`.
    File/directory structure stays semantically opaque; the safety story's phrase
    "single-source responsibility" reads "single-TARGET description" (a family is a
    coherence unit, not an authorship unit; attribution stays line-level).

### The entity-algebra seams  (task 2; 2026-07-10; all typed)

- **rul-family (minted and retained; the human's definition, sharpened).** A *family*
  is the set of non-overlapping/non-contradicting, not-guaranteed-same-author,
  within-compilation-unit `__role` functions that describe-to-Dorc one
  **description-target** — currently one of two species: a COMMAND (the munged
  tool-segment; all `systemctl__*`) or a KIND (all `sm_dorc_Package__*`). Membership
  is by name-construction only — never by file, never by author (`24F` §10's
  "two families" wording, promoted to defined status). The per-species role vocabulary
  is engine-owned, closed-at-a-version, totalistic (pending task 3's additions), and
  extends BY NEW NAME ONLY, never by re-reading an existing member
  (`24C:rul-ditch-is-diverged`'s extension law). Each member optional,
  analyzed-if-present, silence = floor. Families are coherence units (the
  conflict/collision refusals police overlap), not authorship units.
- **rul-seam-context-slot-and-relational-chokepoint (né decision-context-qualifier-
  segment, reframed under the human's fs-view challenge).** The coordinate
  representation grows a context slot (default ambient) — at representation tier a
  space-tag and a qualifier field are the SAME datum, so the qualifier-segment vs
  spaces-with-bridges fork dissolves there. The genuine fork (function-shaped vs
  relation-shaped cross-context identification) defers INTO the comparison: ALL
  coordinate comparison sits behind one whole-coordinate choke-point that MAY answer
  relationally — **per-axis pointwise decomposition is never baked into the API**
  (pointwise is the user-axis special case; fs-view's enumerated-bridge and
  may-alias-floor rungs plug into the same seam later). On record: spaces-with-bridges
  does NOT fully handle fs-view either — mount-state-minted within-context aliasing is
  invisible to any per-boundary machinery and the map-reads are privilege-gated
  (`24S` §3b); DETECT-and-DEGRADE stands regardless of this seam's shape.
- **rul-seam-kind-owner-registry-room.** One engine-internal, extensible per-target
  registry: the at-most-one-per-(family × role) rule generalized to clause families
  not yet minted, loud on conflict. ZERO file semantics — no blessed files, no
  co-location; the homogeneous-compilation-unit posture (strong lean, not welded) is
  affirmed untouched; the only per-ingest-unit fact remains the dialect marker.

## Direction & open threads

- **task-3+task-5 sitting OPENED (2026-07-10; explainer delivered in-chat; every lean
  below is conductor-drafted, NOTHING acked):** the question slugs posed —
  **q-trichotomy-seat**: ~~minted free-standing clause vs sh-bodied per-KIND
  role-function vs measurement-line marks~~ — SUPERSEDED same date by the human's
  greatest-hits replay ("what do people write in sh that expresses I-change-per-user?"),
  which minted **candidate-address-derived-topology**: a NEW per-KIND family member
  (extend-by-new-name, `reaches()`-rhyming but never a re-read; name deliberately
  unchosen, must bias completeness per `24G` §6) whose body spells WHERE THE KIND'S
  STATE LIVES in ordinary sh — `sm_dorc_Cron__lives_at() { printf
  '/var/spool/cron/crontabs/%s\n' "$(id -un)" : file ;}` (STRAWMAN) — with the
  trichotomy DERIVED by dependence-analysis of the traced body under ρ: no
  user-axis-owned input ⇒ invariant (identity bridge + probe-outside license) ·
  user-derived address ⇒ sensitive WITH per-value keying free (the address family IS
  the user-axis map) · untraceable ⇒ ⊤ ⇒ silence-floor (degrades to NO-license, fixing
  candidate-B's fuzzy-license lane). The wild idiom it lifts: user-as-address-parameter
  (`$HOME`, `~`, `$USER`-in-path, XDG); man-page FILES sections are the pre-existing
  documentation twin. Dissolves the kOOB tension (no new syntax; q-clause-grammar DIES
  if D wins) · passes the graduation test natively (static printf arms + dynamic
  host-question arms, dynamic-frame precedent) · real off-ramp value · selector-granular
  mixed topology (git-config `#system` invariant / `#global` sensitive — candidate A's
  flat token cannot express this). Strains, named: non-file-addressable state
  (`sm.dorc.Service#active` = pid-1 memory; mostly recoverable — `#enabled` = wants/
  symlinks, sysctl = /proc/sys — ~SUSPECT near-total coverage, -GUESS residue small);
  invariance-license rests on an EXHAUSTIVENESS contract ("state lives here and ONLY
  here" — the touches() at-most knife, same tier, no new trust category); one
  engine-derived link enters the attribution chain (task 8 must re-read its
  adjudicability condition against derived-not-declared); engine cost = traced surface
  + dependence analysis + blessed who-am-I capture vocabulary (`$USER`/`$LOGNAME`/
  `$HOME`/`$(id -un)`/`$(whoami)` → the user-axis value), landing on block-context's
  ρ-tracing + the reserved backing-SETS seam. Conductor lean REVISED: D primary
  (~SUSPECT — twenty minutes old, un-contacted by the engine's classify/backing shape);
  A demoted to named fallback for unaddressable kinds only-if-field-forced (resist:
  two-mechanisms-for-one-act, the `24S:A5` shape). Set-asides from the idiom sweep,
  recorded: `id -u` root-guards = privilege-requirement not state-topology (imp-1
  cell); tool `-u` flags = company-it-keeps inference, rejected tier ·
  **collapse-analysis for candidate-address-derived-topology** (human challenge, same
  date: "extend-by-new-name isn't live yet — does this collapse with previous work?";
  conductor findings, awaiting ack): (i) NOT `reaches()` — the rhyme is a
  single-file-store coincidence (cron's store = its own reach; dpkg splits them:
  `dpkg -L` payload/causal-edges vs `/var/lib/dpkg` fact-store — disjoint emission
  sets, disjoint consumers); merge-into-one-member-with-per-arm-marks considered and
  declined (two contract texts in one body; name-as-contract loses its force exactly
  at adjudication time); (ii) YES downward — D's implementation is a pure consumer of
  scheduled block-rebuild machinery: un-collapsed address recipes = the
  value-recipe-reshape; re-evaluation/dependence-analysis under a context's ρ = the
  reserved relational chokepoint (`rul-seam-context-slot-and-relational-chokepoint`);
  storage = the reserved backing-SETS seam. New authored surface, ZERO new engine
  lanes; (iii) NOT upward into the measurement-line lane — per-probe read-set
  disclosure as the invariance carrier would require exhaustive open-world enumeration
  (the opaques7-finding20 objection-class: checks transitively read nsswitch/libc/…);
  observe-disclosures stay the safe optional widener per observe-backing-widening; the
  kind-level store question is FILES-section-small, the per-probe read-set question is
  not; (iv) BORDER FENCE REQUIRED — `cron:alice ↔ file:/var/spool/…/alice` is a
  cross-kind identification, and the corpus already holds three pointers at ONE parked
  mechanism (`24M:rul-kind-unify-owed` · `24C:strain-coreference-crosskind` ·
  `24S:A5` "design it once"). Proposed fence: v1 consumes D's address recipes
  EXCLUSIVELY for the per-axis dependence bit — never address-vs-coordinate
  identification, never address-granular disjointness (shared-store kinds would
  over-merge: every Package cell shares `/var/lib/dpkg`); when co-reference unparks,
  D's member is its authored INPUT (one surface, staged consumers). D registered as
  the FOURTH pointer so the eventual unifier finds it. Meta, conceded: the prior
  lean's extend-by-new-name invocation was compat-register; pre-ship the law is
  design-hygiene (one question per name), hardening into compat at stdlib-ship ·
  **microtypesystem-challenge findings** (human challenge, same date: "is this a
  half-assed higher-kinded typesystem — another poorly-derived microtypesystem stage?";
  conductor findings, awaiting ack): the formal object named — cells are a
  CONTEXT-INDEXED FAMILY; D's derivation is a DEPENDENCY/NON-INTERFERENCE analysis
  (DCC lineage; the blessed who-am-I set = the primitive labeling every such analysis
  requires; AGENTS' own "tainting"); and the complete object is the chokepoint as ONE
  THREE-VALUED RELATION `compare(cellA,cellB) ∈ {same(coord), disjoint, unknown}` with
  every authored surface a GENERATOR feeding it. Hits conceded: (h1, serious) the
  trichotomy's `sensitive` CONFLATES per-value keying (re-indexing, license-free) with
  a separation license (survival-feeding, knife-tier) — and D would derive separation
  from ADDRESS INEQUALITY, the naive-string sin `24S` §3b branded wrong for fs-view
  (paths alias on the user axis too); identification-by-non-interference is
  sound-shaped, separation-by-difference is NOT; (h2) granularity was fixed by
  accident (git-config broke candidate A; D's case-arms happened to be per-selector) —
  relation-first derivation would have forced per-cell grain on day one; (h3) the
  four-token enumeration is defensible ONLY via the relation view (= the possible
  uniform answers to the two consumer queries), unverified until now. Where the
  accusation fails: generator-incompleteness is VALUE-LOSS ONLY by the safe-bottom
  default (unknown ⇒ run/collide/wall) — an evolving generator set is survivable by
  design; the bite classes stay incorrectness (knife, attributed) and contradiction
  (refuse-both). Fix proposed: the entity-algebra design note (task 12) gains a
  ONE-PAGE formal section (kVERIFY-calibrate tier — spec, not proof): the relation +
  consumer map, the generator registry (each surface × verdicts-it-may-generate ×
  license tier), and the forced v1 decomposition of candidate D — **derive invariance
  and per-value keying; NEVER derive separation** (disjointness across context-values
  arrives only as a declared/owned act with dialect-grade care, later; cost = only the
  within-kind cross-context disjointness dividend, modest — cross-kind stays free,
  line-8-class sites stay guarded via imp-1 regardless). The formal section doubles as
  the `24S:A5` design-once interface (bridges = the same-via-map generator class) ·
  **strawmen-survival-round** (human-directed, 2026-07-10: five non-trivial
  real-world examples worked through both surviving shapes, in-tree at
  `notes/27x-strawmen-topology/` — systemd user-units · git-config scopes ·
  postgres peer-auth · homebrew prefix · docker rootless; conductor findings,
  awaiting ack): **f-narrowed-knife** — under the co-reference fence, INVARIANT
  addresses may be approximate (consumed only for dependence shape); the
  exhaustiveness knife narrows to "never forget an axis-DEPENDENT store"
  (systemd `#active`'s `/run` stand-in is fine) · **f-kinds-are-topology-units** —
  mixed-topology tools resolve by argv-driven kind-binding (Service/UserService
  split); both shapes need the split, D makes each half derivable/lintable, A
  asserts blind; extends the behaviour-menu · **f-address-entities-dissolve** (the
  biggest) — address-shaped entities + ρ absorb most apparent sensitivity
  (git-config needs ZERO kind topology; the `sudo git config --global` footgun
  keys correctly for free); topology machinery is needed ONLY for logical-entity
  kinds; the bootstrap file kind's grounding is ENGINE-SUPPLIED (authored identity
  `lives_at` = cargocult); new behaviour-menu line: logical entities buy cross-tool
  collaboration at the price of owned topology, address entities are
  collaboration-poor but topology-free; a lint writes itself ·
  **f-referent-vs-access** — postgres: invariance is TRUE at referent tier; access
  gating is per-PROBE, enforced by rc-reality (license ≠ ability); imp-1
  composition graceful; shapes tie; cross-kind wall-bounding needs no topology at
  all · **f-dynamic-arms-need-capture-claims** — homebrew: a captured address root
  (`$(brew --prefix)`) is statically unclassifiable, so D's dynamic arms are
  CONSUMERS of task-7's read-blessing vocabulary, which therefore needs an
  AXIS-INDEPENDENCE value-bound dimension (task 7 has a second customer — couple
  the sittings); meanwhile Shape A scores its one clean win (cheap assertion where
  D waits on unbuilt machinery) · **f-carve-validated** — docker: the PLAIN
  system-daemon host is a live counterexample where derived separation would
  silently under-execute (alice and root genuinely share one daemon; a probed
  fact "surviving" root's `docker system prune` would be wrong in the default
  install) — never-derive-separation upgraded precaution → demonstrated-necessary;
  A is inexpressible for docker while D's conservative projection ≥ A's best
  honest token; the per-host topology refinement is banked, chronology-priced,
  deferred alongside task-7's planes-meet-at-chronology cell ·
  **terminology proposal** — split `sensitive`'s conflation in the design note:
  `keyed` (derived, safe re-indexing) vs `partitioned` (declared-only separation,
  knife) · **net assessment**: the relation+generators formulation SURVIVED (no
  example needed a new verdict class; every wrinkle landed as generator-scope or
  consumer plumbing); D-primary strengthened overall but shown DEPENDENT on
  task-7 vocabulary for dynamic arms; A's fallback role confirmed narrow-but-real
  (capture-rooted/unaddressable kinds wanting a cheap assertion) ·
  **q-map-defers-with-fs-view**: sensitive-with-map's canonical home is the fs-view
  axis, so the fs-view map rides the soft-deferral — and under candidate D the
  user-axis "map" is subsumed (the traced address family); v1 authored surface =
  addresses on the user axis only · **q-wrapper-member-count**: one
  context member vs two members split along `24S` §1's own ρ-vs-axes seam (lean = two;
  partial-decline composability — su's ρ declines while its peel+user-axis still pays —
  plus the peel cross-check falling out free from dual argparse) ·
  **q-rho-closure-spelling**: the ρ-transform member spelled as an EXECUTABLE
  env-closure (`env -i TERM="$TERM" … "$@"`) — statically traced for the per-variable
  map, shippable as `24S` §6b's whole-ρ replication builder, real off-ramp value;
  engine reads closure bodies against a small blessed-idiom contract ·
  **q-identity-spelling**: the one real grammar hole — a POSITIVE per-axis
  "moves-nothing" spelling, since per-axis silence must stay ⊤ (opaques7-finding2) ·
  self-effects: no new spelling proposed (wrapper's own `touches()` + the standing
  oracle-vouches-for-itself rule).

- **watch-machinery-raises-disclaim-burden** (human observation, 2026-07-10, task 10b —
  "not a nack"): the effect-check "leans perilously close to promising non-mutation";
  every layer of machinery *supporting* the user here raises the documentation/education
  bar for disclaiming that the machinery isn't *enough*. Standing education posture:
  warranty only the structural class; frame everything else as lint-against-accidents;
  the authored vouch remains THE contract. Same shape as the `--trust-footprints`
  "marketing at best, theatre at worst" honesty note — watch it wherever new
  contract-supporting machinery lands.
- Conversational-strawman discipline (conductor note-to-self): function names in
  strawmen use the bare munged `tool__role` form per `24M` (`foobar__is_converged`),
  not single-underscore; the `# dorc-lang/v0.1` marker line is exact-match and stands
  ALONE — never append prose to it.
- **observe-backing-widening (drafted-awaiting-ack; from the `:?`/ACK exploration,
  2026-07-10):** an Observe (`:?`) mark inside a verdict-function body should WIDEN
  that fact's backing to include the observed coordinate — the stage-5 principle taken
  literally (a backing IS what its probe reads; a disclosed read is part of the
  read-set). Tip's `Backing` is single-coordinate by construction, so this needs
  backing-SETS — an entity-algebra-rebuild seam to reserve. Safe direction (only ever
  widens the kill-surface); patches the disclosed-read corner of the overlapping-cells
  residue; and makes acknowledge-by-observing coherent (a standalone observe-only
  site mints its token into the source's dialect without coupling any verdict-fact's
  backing).
- **ack-stays-dead (conductor opinion on record; from the same exploration):** the ACK
  `~` and POISON bare-marks stay dead through the respell as staged (`24P` §2) — they
  are zombie no-ops at tip (parsed, licenses-nothing, since `23D` §5 killed
  negative-enumeration; zero corpus occurrences). The ghost-ack job (orthogonality
  opt-out for expensive-probe subscribers) is partially covered by
  acknowledge-by-observing where a natural read exists; any future revival should be a
  rider on a measurement line, never a bare dislocated mark, and only on field
  evidence.
- **Task-2 (the two `24S:A7` seam reservations: context-qualifier segment; kind-owner
  registry room) — deferred to a fresh turn**; re-explain from scratch, slower. Note
  for that re-explainer: `rul-axis-vocabulary-v1`'s fs-view soft-deferral shrinks the
  qualifier to effectively {user, fs-straw}.
- **two-observation-planes (vocabulary for the task-7 sitting; human-checked
  2026-07-10):** distinguish WORLD-PLANE observation (a coordinate statement —
  `kind:entity#selector` — unknowable to a referent-agnostic engine, hence
  must-be-annotated: the Establish/Observe marks' job) from VALUE-PLANE observation
  (bytes through the program's own dataflow — captures, pipes, AND the rc channel —
  structurally visible in the AST, no annotation needed for the edge to exist;
  oracle-knowledge adds only permission = read-blessing and shape = value-bounds).
  The rc channel already has both planes (structural flow + mark-given world-meaning);
  stdout has flow only (`OutClaim` reserved, unproduced). Task-7 restated: does
  captured stdout get a world-plane identity, and from where? The planes must meet at
  CHRONOLOGY (a captured probe-time value's survival past a wall is a
  where-does-its-truth-live question — `$(hostname)` vs `$(cat /etc/flag)` are
  structurally identical captures with opposite answers; safe floor = interposed wall
  demotes capture-consumers). Candidate to evaluate skeptically at task 7, NOT
  pre-decided: the producing read-blessing carries Observe coordinates and the
  captured value INHERITS its backing from them — one mechanism, both channels.
- **Owed at 1-series close:** the `notes/24P` specimen amendment pass (hash introducer
  at minimum; brace-alternation if its grammar firms); then the entity-algebra design
  note itself.
