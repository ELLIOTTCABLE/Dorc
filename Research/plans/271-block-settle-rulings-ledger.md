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

## Session task map (mirrors the conductor task list, 2026-07-10)

1. adj-entity-algebra, spelling half — the authored entity/selector mark grammar
   (gates block-rebuild; needs typed ack).
2. adj-entity-algebra, seams half — the two `24S:A7` reservations (context-qualifier
   slot; kind-owner per-axis room).
3. adj-trichotomy-spelling (né `24S:A3`) — kind-owner per-axis context topology;
   the deliberate kOOB-redline reading.
4. adj-axis-vocabulary (né `24S:A1`) — ratify v1 = {user, fs-view} + ρ,
   versioned-closed, engine-owned.
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
    probe-bodies-proved-non-mutable effect-check's home.
11. adj-payload-pins pre-read (né `24T:P-A1`/`P-A2`) — optional tail; formally owed at
    block-context implementation-planning.

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

### rul-coordinate-shape-flat-three-place  (task 1; 2026-07-10; typed)
The coordinate promoted into the intersection plane (footprints × backings ×
disjointness) is the **flat three-place `(kind, entity, selector)`** — the shape
`FactKey` already has; `Backing`'s deliberate selector-drop retires in the
entity-algebra-rebuild. The `plans/17N` §4 recursive/JSON-adjacent-struct lean is
DECLINED as coordinate shape — its recursion motivators (cross-kind handles, aliased
names) were since absorbed by owner-declared *mechanisms between* flat coordinates
(`resolve()` canonicalization; `reaches()`/`manifest()` expansion), so the coordinate
stays a name. Human riders (typed): he is amenable-not-evangelized — unsold that NO
structural-utility corner remains; the position holds because later expansion is
"language-design easy" **as long as the bare form continues to backwards-compatibly
mean "true"/"occupied"** (the 17N present-key-is-true default, preserved as law:
selector-less = whole-entity/occupied, forever). Conductor hedge riding the ruling:
`SelectorId` stays opaque/interned and every selector comparison lives behind ONE
choke-point function (`selector_covers`-shaped), so any future structure lands in one
function, not a re-key. Consumers: entity-algebra design note, entity-algebra-rebuild,
corpus-respell.

### rul-selector-introducer-hash  (task 1; 2026-07-10; typed, PERMANENT)
The selector introducer is **`#`**, replacing tip's `.` everywhere a selector is
spelled: `sm.dorc.Service:"$svc"#enabled`; `:? sm.dorc.GrepMatch:"$pat"#matched`.
Human rationale (typed): matches the HTML-anchor / commit-ID vibe of "narrow down to
this specific thing." Rider (typed): `#` still compounds with several entity
constructs, so **quoting must stay supported** — the quoted-entity form remains the
supported spelling where charsets collide (exactly as was already true under period).
Churn: rides the corpus-respell with everything else (mark grammar + observe marks +
render/why-lens fact-label strings + records); the `notes/24P` specimens get a
conductor-side amendment pass at 1-series close (the anticipated block-settle move);
`24P` d4's unquoted-dotted-entity ⊤-reject corner gets re-derived under `#` in the
design note. New root-doc staleness accrues to the punted root-doc pass: USER_STORY's
`.synced`/`.matched` forms.

### rul-emission-selector-on-mark  (task 1; 2026-07-10; typed ack + direction)
`touches()`/`reaches()` claim a cell by the selector riding the **trailing mark**
alongside the kind — `printf '%s\n' "$1"  : sm.dorc.Service#active` — applying to
every line that statement emits; emission lines stay raw entities (no output
dressing); a selector-less mark = whole entity (today's exact meaning, the floor).
Direction (human strawman, same date, grammar NOT yet ruled): multi-cell claims need
a **list-y shape** to avoid seven-line touches bodies for complex structures — leading
strawman is shell-obvious alternation-braces, `: sm.dorc.Service#{enabled,active}`.
Grammar detail owed to the entity-algebra design note.

### rul-binds-entity-only-provisional  (task 1; 2026-07-10; typed, SOFT — ~GUESS-shaped
on both sides)
Binds name **entities, never cells**: `pkg : sm.dorc.Package = "$1"` binds identity;
facts about cells attach via marks on probing/emitting commands. Neither party is
confident beyond ~GUESS; the door stays open if a bind-to-a-cell counterexample ever
arrives; nobody is worried. (Rationale on record: letting binds carry selectors blurs
identity — `resolve()`'s domain — with state — the probe's domain; the 17N Seam
warning.)

*(entry format for future additions:)*

<!--
### rul-<full-word-slug>  (task N; YYYY-MM-DD; typed | drafted-awaiting-ack)
One-paragraph ruling. Surviving strawman sh inline where spelling-relevant.
Supersedes / amends: <docID:slug refs>. Consumers: <which block/brief this binds>.
-->

## Direction & open threads

*(non-ruling steers, rejected alternatives worth remembering, and threads cut short by
a rewind — so the next context can resume mid-thought.)*

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
  not single-underscore.
- **decision-selector-vocabulary-gating — OPEN, awaiting ruling (task 1's last open
  item); proposal REVISED 2026-07-10 after the human's annotation-not-declaration
  challenge.** History, compressed: draft-1 was a kind-owner *declared* selector
  vocabulary (a behaviour-dislocated declaration — would have been the project's FIRST;
  the human flagged the kOOB/Ansible-death direction and asked for a
  spelled-in-natural-sh lift instead; his gentle suspicion "annotation-not-declaration"
  confirmed). Draft-2, the live drafted-awaiting-ack mechanism,
  **measured-selector-universe** (slug candidate `rul-vocabulary-is-what-probes-measure`):
  the natural sh construct carrying cell-names is the **per-facet status read** the
  world's tools already ship (`systemctl is-enabled` vs `is-active`; `dpkg -s` vs
  `dpkg -V`; `git diff --quiet` vs `--cached --quiet`) — the selector token is an
  ANNOTATION on the measuring line (existing probe-mark grammar, `#` per
  `rul-selector-introducer-hash`, zero new surface); the kind's cell-universe is
  PROJECTED as the set of selector tokens annotating loaded runnable measurements of
  that kind (the stage-5 principle one level up: a fact's backing IS what its probe
  reads ⇒ a kind's vocabulary IS what its probes measure). Gate rule: selector
  DISJOINTNESS (the survival license) concluded only between measured tokens; an
  unmeasured token = ⊤-selector (collides with every cell of its entity), either side —
  the both-sides symmetry now falls out by construction (claims can only narrow against
  the measurement side's actual dialect). Empty universe ⇒ byte-identical to today.
  Each universe member cites its minting measurement line (more attributable than a
  declaration). Dissolves draft-1 costs: new-declaration-family, owner-bottleneck,
  bootstrap-authoring-tax. Honest residues: multi-dialect synonym hole (two probe
  authors, one kind, two tokens for one cell — warn-tier divergence lint now;
  co-reference-unification later); same-token-divergent-meaning (unchanged,
  adjudicability-list tier); physically-overlapping cells (inherent to narrowing;
  differential-discharge-testable — mutate via claiming verb, watch the sibling probe
  flip); universe is a pure function of loaded oracle text (DST-clean); loading an
  oracle can upgrade a ⊤ claim to a narrowing one (monotone-value direction, disclose).
  Open sub-question: WHO mints — any loaded probe-mark (conductor lean, ~SUSPECT:
  maximally gradual, no ownership machinery, divergence-lint covered) vs
  owner-co-located marks only (tighter; reintroduces ownership dependency most v1 kinds
  lack). Forward-implication flagged: this precedent (annotation-lift over minted
  declaration) directly pressures `270:adj-trichotomy-spelling`'s recorded
  minted-syntax lean — carry into task 3.
  **Draft-2b (2026-07-10, same session — the human ordered a kill-attempt on draft-2
  ("suspect you folded to my pushback too easily"); the attempt found a real crack and
  forced a repair):** the crack — under any-loaded minting, a SECOND probing dialect on
  one kind (hork's own probe marks `#running` for a cell stdlib measures as `#active`)
  re-mints the synonym-survival hole the gate exists to close; the mitigation trilemma
  is genuine: warn-tier lint doesn't close a survival hole / hard-degrade-on-divergence
  breaks the monotonic contract (loading a third-party oracle would strip stdlib's
  precision) / owner-scoping requires ownership machinery the project deliberately
  lacks (draft-1's declaration was secretly an OWNERSHIP ANCHOR, not just a token-set).
  The repair that dissolves the trilemma — **backing-dialect-scoped comparison**: no
  global per-kind universe exists; dialect(source, kind) = the selector tokens that
  source's probe-marks measure; a claim spares a backing iff claim-token ∈
  dialect(BACKING's minting source, kind) ∧ ≠ backing-token; everything else collides.
  Walked cells: stdlib-only value case spares ✓; claim-typo collides ✓; probe-typo
  collides ✓; cross-dialect synonym (backing `#running`(hork) vs claim `#active`)
  collides ✓ — hole closed; monotone ✓ (a new source never alters comparisons against
  other sources' backings); claims can never self-license (probes mint only for their
  own facts; claim-side noise only ever collides = safe); who-mints DISSOLVED
  (minting is inherently per-source). Complexity honesty: authored surface stays zero,
  but engine-side cost relocates upward (per-(kind × source) dialect sets; backing
  provenance threaded into comparison; -GUESS cheap given the ProvId arena).
  **Honest corners where draft-1 (declaration) remains strictly stronger:**
  (1) claim-only / write-only cells — a declared-but-unprobeable cell (`#logbuffer`)
  can partition claims under draft-1 but is ⊤ under any measurement-lift; narrowed by
  the escape that unprobeable disturbances usually spell as OTHER entities/kinds
  (`sm.dorc.File:/var/log/…`), and by write-the-cheap-probe; residue disclosed.
  (2) the ownership anchor itself (if the project ever wants one, a declaration is
  where it would have lived). Attribution under draft-2b ≈ draft-1 (single-source
  responsibility; author-level rather than always line-level for incoherent mark-pairs;
  claim-omission stays the footprint omission-class, unchanged from draft-1).
  Task-3 caveat sharpened: the annotation-lift precedent may NOT generalize to the
  per-axis trichotomy — there may be no natural measurement line for "user-invariant"
  to annotate; do not let this episode pre-decide task 3 in either direction.
  **The 233-cve_clean walk (2026-07-10, human-directed — the classic subscription
  example run through draft-2b):** findings, all recorded as draft-refinements:
  (i) OBSERVE marks (`:?`) mint dialect exactly like verdict-probe marks — both are
  measurement-annotations on runnable reads; claims/touches never mint (unchanged).
  (ii) The `233` §4 hard-1 objection ("apt's author couldn't know somebody would write
  a cve-checker") DISSOLVES under backing-dialect scoping: foreign claims are judged in
  the SUBSCRIBER's dialect, so the enumerate-the-future burden relocates from the
  kind's incumbent authors (impossible) to the property's extender (the one party who
  knows their cell's invalidation semantics). (iii) `24F` §10 alternative-6 DISSOLVES
  into draft-2b: its default (killed-by-any-entity-touch) = the collide-default,
  strictly strengthened to killed-by-any-foreign-claim; the extender's
  invalidation-basis declaration = the complement of their minted dialect — the
  declaration became annotation. (iv) The draft-1/global-vocabulary counterfactual is
  BROKEN on this example, twice: third-party property-extension chokes on the
  ownership bottleneck, and once admitted, in-vocabulary sparing silently BREAKS the
  subscription (apt's `#installed` claim spares the stale `#cve_clean` verdict past a
  package upgrade — under-execution-flavored) unless alt6 bases are declared on top.
  (v) **ghost-ack-mark, named**: "acknowledge a cell without touching or measuring
  it" — recurs as 233's deleted ACK `~`, alt6's basis-list, draft-1's vocabulary, and
  now the subscriber's spare-opt-in (opting OUT of a kill requires minting a token one
  has no natural read for) plus the channel-publisher's noise mirror (a cve-db-refresh
  claiming `#cve_clean` over-kills foreign probers' unrelated facts). Position: stays
  UNBUILT; absence fails safe (over-kill / check-tax, never wrongness; kPROBING
  banding governs expensive re-scans); earns its dislocation only on field evidence.

### rul-kind-or-selector-is-a-behaviour-choice  (task 1; 2026-07-10; typed)
The choice between minting a new KIND versus a SELECTOR on an existing kind is a
user-facing behaviour selection, not an ontological one — "the point of having both
types of thingie IS to have differing behaviour for them" (human, verbatim). Any
design/API surface Dorc provides is "functions to call — tools to use and abuse";
authors pick based on the behaviour they want, provided constraints and behaviours are
well-documented. The menu under draft-2b: kind = ISOLATION (cross-kind disjoint by
construction, reaches()-bridged only; claims spare everyone; no coupling, no
vocabulary, claim-only usable — the logbuffer/cache/scratch tool); selector =
COUPLING (same-entity collide-by-default; subscription to the entity's kill-traffic
in both directions — the cve_clean tool). Consequence: the draft-1-advantage
"claim-only cells" RETRACTS to the ghost-ack-mark sliver (see the open thread); the
docs/teaching obligation (behaviour-menu documentation) rides the stdlib
quality-bar checklist.
- **Task-2 (the two `24S:A7` seam reservations: context-qualifier segment;
  kind-owner registry room) — DEFERRED to fresh turns** (human, 2026-07-10: did not
  land on first pass; re-explain from scratch, slower, when picked back up). Note for
  that re-explainer: `271:rul-axis-vocabulary-v1`'s fs-view soft-deferral shrinks the
  qualifier to effectively {user, fs-straw}.
- **Owed at 1-series close:** the `notes/24P` specimen amendment pass (hash introducer
  at minimum; brace-alternation if its grammar firms); then the entity-algebra design
  note itself.
