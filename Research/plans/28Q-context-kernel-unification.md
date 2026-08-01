# 28Q — The context-kernel unification: frames, closures, availability (THE refactor plan)

> Tier: LLM-authored plan (Fable conductor, human design dialogue 2026-08-01, session
> `r28-megamerge`); subordinate to root docs and `spike/CLAUDE.md`. Grades as in `28K`/`28M`:
> **[TYPED]** human typed it · **[ACKED]** substance confirmed in dialogue · **[PROPOSED]**
> conductor-derived, awaiting ratification. The three-pillar synthesis as a whole is
> ACKED-in-direction ("this all sounds coherent to me", 2026-08-01, after four typed
> refinements banked in §5); mechanics below are PROPOSED unless marked otherwise.
>
> **This is the single home for the analysis-kernel refactor** — the human's standing order
> is NO MORE PIECEMEAL work on this territory. It SUPERSEDES: the `28P:amend-plural-value-
> loss-hold` standing order (bitem4/bitem5/withhold-softening were held for exactly this
> plan; they land here as stage work), `26K` §0b's kernel-batch agenda (absorbed into
> stage-iii; `26K` remains live for its §0a fruit arc and §5 sittings queue), and the
> chat-tier reach-mode taxonomy from the 2026-08-01 sitting's first synthesis message
> (corrected in §4). It does NOT supersede `plans/27C` (still THE wrapper/context-entry
> spec — §4 generalizes its availability behavior, touching none of its consent machinery)
> nor `28M` (still the committee-corner authority; §3 here consumes its §10 direction).
>
> Two term slots are deliberately unsettled pending a terminology survey (dispatched
> 2026-08-01): **[TERM-A]** = the context re-creation/lifetime marker (currently "epoch",
> overloaded) and **[TERM-B]** = the availability-changing event class (currently
> "transit"/"pivot", both disliked). This document uses the bracketed placeholders where
> precision matters and the old words only when citing older documents.

## §0 — The design in one screen

Three asks, one model. (P1) Every kernel-derived truth that flows through a
can-be-committee API entry point — function resolution, argparse/identity application,
cell indices, verdict sets, dialect enrollment, footprints, helper closures — becomes
**positionally indexed**: keyed to the environment frame it is asked from, so loading a
file inside a subshell appends/shadows engine knowledge for questions asked in that
region. (P2) Every "owner"-keyed concept re-keys from single-file to the **entry-closure**
— the transitive closure of literal `.`-sourcing from an entry file — so authors factor
helpers into second files with no registry, package name, or manifest: sourcing IS the
package boundary, spelled in sh. (P3) World contexts (user, fs-view, netns, host, and the
[TERM-A] axis) gain a single **availability predicate** over plan position; probing enters
only currently-available contexts; commands that create/destroy/re-create contexts are
ordinary oracle-described commands; the transit-relative epoch law generalizes to every
dimension.

The unifying claim, `syn-one-context-two-planes`: every kernel query carries a two-sided
context — *where in the text am I asking from* (the load plane) and *where/when in the
world am I asking about* (the world plane) — and both planes obey ONE discipline:
piecewise-constant truth between events, region-scoped shadowing, and per-name/per-kind
crossing claims for what survives an event.

| | load plane (P1) | world plane (P3) |
|---|---|---|
| events | funcdef · `unset -f` · `.`-source · subshell open/close | walls (unmodeled mutators) · [TERM-B] events (create/destroy/re-create) · context entry/exit |
| regions | environment frames | intervals where a probed fact stays trustworthy (the wall machinery already computes these) |
| scoped shadowing | subshell re-source (dies at the paren) | an entered context (`sudo`/`chroot` region; an ssh'd host region) |
| crossing claims | a name survives events that do not touch it (sh-given) | a fact survives a [TERM-B] iff its kind-owner marked `undivided-by-transit-across <axis>`; survives a wall iff footprint-disjoint |
| silence | unresolvable load ⇒ ⊤, walls | unmodeled command / unmarked kind ⇒ wall |

`syn-frame-tree-is-fork-semantics` — the load plane's "resolution DAG" is literally sh's
fork semantics: a subshell is a forked child environment whose mutations are invisible to
the parent; `.` is in-process mutation. The DAG is the fork tree crossed with linear
program order. PLT-standard vocabulary: these are scoped environments over a program
order; no coinage needed. The world plane is the same shape one level up: the apply is a
program order over world state; hosts and [TERM-A]-lifetimes are the scopes; [TERM-B]
events are the mutations. Retire "pivot" everywhere.

`syn-zero-new-spellings` — all three pillars consume EXISTING sh acts: sourcing,
subshells, `unset -f`, creator/destroyer commands with ordinary oracles, ssh lines.
No new authored surface anywhere (the §11 language dig may add *oracle-member vocabulary*
for describing creators; it must clear the sent-language-is-becoming-crufty bar).

## §1 — P1: definition-factored indices (`syn-definition-factored-indices`)

The as-built gap: `KindIndex`, `VerdictIndex`, the effect map, the dialect, and
`HelperIndex` are whole-unit merges, which is why bitem0 settled for the agreement veto
(`28P:dec-the-gate-is-agreement-not-re-resolution`) — true positional resolution over
merged indices risks the chimera (identity through one author's argparse, cells from
another's; pope-sin, invisible to goldens). The fix is a FACTORING, not per-frame copies:

1. Every derived row — a check, a cell declaration, an argparse arm-model, an enrolled
   dialect token, a footprint claim, a helper closure — is keyed by the **DefinitionId**
   that produced it: (SourceFileId, span, custody). Computed once, whole-unit, exactly as
   today. No index multiplication.
2. The ONLY per-frame structure is the frame → live-definition map, which `funcenv`
   already computes (positional, scope-stacked, frozen).
3. A query at site S = `live_definition(frame(S), name)` → read THAT definition's rows.
   One indirection, at the seats that already route through
   `live_source`/`answers_at` — the indirection point exists because bitem0/2/3 built it.

Consequences:

- **The veto retires in favor of true resolution**, structurally chimera-free: identity
  and cells hang off one DefinitionId, so the mismatched read becomes unrepresentable
  rather than gated. `28P:res-plural-families-withhold-off-peak` CLOSES — the
  blessed-override (above `unset -f`) and subshell re-source idioms ANSWER from their
  positionally-live definitions instead of going value-dead; `28K` §1
  `rul-scope-by-subshell-resource` and `28M` §3 `wall-verdict-tier-sovereignty` are
  finally delivered as written.
- `live_source`'s whole-unit text-scan winner and the fold's `never_live` subtraction are
  SUBSUMED: under frame-keyed liveness, a never-live definition is simply live at no
  frame; the exact-subtraction hazard (`28P:adj-never-live-exactness-accepted`) dissolves
  into the general mechanism. `oracle/CLAUDE.md live-source-is-the-only-resolution-seat`
  becomes "the frame-lookup is the only resolution seat".
- bitem1's built-but-unreachable hash-munge becomes reachable exactly as its ledger
  predicted (two frames, two live bodies, two munged names); the pinned-definitions
  machinery needs no redesign — `plan/CLAUDE.md pinned-definitions-are-the-artifact's-
  binding` already handles per-guard-site binding, and the positional regime already
  guarantees definition-precedes-guard.
- The seventh seat (`build_wrapped_vouches`, `28P:tc-wrapped-vouch-seat-has-no-positional-
  gate`) unifies with the other six for free; so do the whyworld/survival seats that today
  neither withdraw nor lift book definitions (`28P:res-whyworld-and-survival-do-not-
  withdraw`, `tc-wrapped-lane-drops-a-case-bodied-in-book-verdict` — both are
  oracle-only-vector coincidences that the one-lookup design deletes).

Constraints honored, verbatim commitments:

- **`an-flat-domain` / k-CFA** [reconciliation owed as one paragraph in ANALYZER-NEEDS]:
  this is NOT context-sensitivity in the k-CFA sense. Frames are intervals in one linear
  program order plus a statically-known fork tree — O(env-mutating statements), no
  call-strings, flat domain values, no closure recombination. The redline stands.
- **`funcenv-reads-source-literal-plane-only`** preserved: frame identity is
  ProgramText-graded; probe-provenance values never site a load decision;
  host-conditional loading stays ⊤ (`rul-unloadable-is-unlicensed` untouched).
- **`vocabulary-acts-stay-ambient`** carve preserved: the kind-owner trio answers
  world-noun questions, not book-region questions; their keying is §2's (closure), never
  frame. In-book vocabulary roles keep refuse-with-notice.
- **Gate: `syn-single-frame-byte-identical`** — a single-frame, single-closure world
  (today's entire corpus) produces byte-identical output. This is the migration gate for
  every stage-i commit, the generalization of empty-world-byte-identical.

## §2 — P2: the entry-closure is the speaker (`syn-closure-is-the-speaker`)

One identity, consumed everywhere "owner" appears: the **entry-closure** — the transitive
closure of literal `.`-sourcing reachable from an entry file (a CLI-named positional, or
the book). Sourcing is the promise "I treated this as if I wrote it"
(`28M:dir-ownership-is-transitive-inclusion` [TYPED]); the closure is the package boundary
spelled in sh — no registry, no owner-name, no manifest file, ever.

Re-keyed seats:

- **`core::DefinitionCustody` internals** — bitem3 designed this seam explicitly
  ("the re-key is a change to this type's internals and nothing else"): custody becomes
  closure-membership; consumers still only compare. One type, zero consumer churn.
- **The committee fence** (`28M` §4, ruled): live ROLE members *at a frame* spanning more
  than one closure ⇒ sparing-inert. Note the fence becomes frame-relative under P1 — a
  family can be single-closure inside a subshell region and plural at ambient — which is
  the correct reading of "live members". Helpers never create plurality (custody flows to
  the closure; they are not role members).
- **Kind-owner single-occupancy**: one CLOSURE mints a kind's vocabulary members;
  `28M` §11's registration lean ("registration-by-owner-use over the owner's closure")
  gets its substrate for free if ruled. `price-multi-file-package` softens to nothing —
  one entry sourcing its parts is one speaker; the bitem6-proven two-file helper-package
  shape becomes the canonical packaging story. `price-patch-author` is UNCHANGED
  (member-granular patching across closures is still committee; honesty preserved).
- **Diamond loading**: byte-identical files dedup (built); differing bytes refuse (built);
  the fence binds only sibling/cousin edges in the include-tree, never above/below
  (`28M` §10 direction). Unit-identity keys to the defining file within the closure
  (version-skewed vendored copies refuse rather than dedup — the bitem1 rider restated
  closure-relative).

**`rul-blessing-flows-from-best-caller` [TYPED substance 2026-08-01]** — custody and
GRADE are different relations and flow in different directions. Custody: the speaker is
the closure; helpers are subordinate for fence/attribution purposes. Grade
(trustworthiness-for-purpose): INFECTIOUS, outward from the blessed use — `predict()` and
`is_converged()` carry mildly-opposite care-leans (description-tier lexical discipline vs
judgment-tier proxy-checking; the ground for `28M` §11's verdict-word enrollment
exclusion), and a helper that a family's `predict()` ever reaches transitively was
demonstrably built under description-tier care — so its gen-marks carry predict-tier
vocab-minting rights EVEN when invoked from `is_converged()`. The exclusion is rescinded
for blessed TEXT, never for a blessed call-site; elevation flows from the best caller.
Consequence: the keep-the-exclusion lean gets cheaper (multi-arm verdict-only authors
factor describing lines into helpers reached by one thin predict; the stage-4½ hill
flattens). **Open pin `pin-blessing-keying`:** family-rooted-within-the-closure
(reachable from THIS family's predict members; conductor lean) vs closure-global
(`ack-shared-bytes-transitive-permission` reads closure-rooted). One line of ruling; only
bites when one closure hosts families of divergent care.

Boundaries: the closure is identity/attribution keying ONLY — never separation, never
trust-widening (keyed-not-partitioned stands); `AID:law-lineno-identity` survives
untouched (closures overlay files; line-spaces never merge; the multi-book concatenation
violation is a separate, orthogonal CLI-round fix). **Gate:** a single-closure world is
byte-identical. **Payoff gate, human-owned:** the `.`-of-a-proven-load-inert-file
blessing — until a book's top-level source stops walling, book-side closures are
analysis-real but value-dead (the gate bitem9 hit). This is an ENGINE ruling, not an
oracle: the property doing the work (load-inertness) is engine-proven, and the
classification "sourcing a proven-inert file disturbs nothing world-side" is a judgment
over sh semantics only — referent-agnostic-clean (`res-dot-blessing-is-engine-side`).

## §3 — P3: availability over all contexts (`syn-availability-is-universal`)

Every world-context — the folded per-dimension key of `27C`'s composition algebra, now
including a host coordinate and a [TERM-A] lifetime marker — carries an **availability
status as a function of plan position**: available · arrives-at(p) · departed-at(p) ·
never. Probe-time entry is availability at position zero. Everything else derives:

- **Entry is uniform across dimensions** [ACKED 2026-08-01,
  `rul-host-entry-is-ordinary-entry`]: a wrapper chain, an ssh line, or ambience denotes
  a context; probing enters it if available, under `27C`'s unchanged consent machinery
  (dial × vouch × entry form). `ssh host …` IS a peeling wrapper whose entry form is the
  connection dance (`ack-connection-dance-oracles-core` unchanged: the ssh oracle's own
  arms mint reachability facts; the engine mints nothing). The wrapper-law's "execs the
  remainder locally" fence is mechanical, not categorical — ssh's genuine extra
  obligation is the remote-shell RE-PARSE round-trip, which is exactly its entry-siting
  vouch discharge (and why the engine's own transport ships artifacts on stdin, `260` §5).
  Host ALIASING (`~/.ssh/config`, short names, IPs) is package-name aliasing's cousin:
  host identity is resolve-machinery territory, never string comparison
  (`res-host-identity-wants-resolve`).
- **[TERM-B] events are not a reach-mode** — they are world-mutations that move contexts'
  availability: creators (`useradd`, `mount`, `doctl compute droplet create`), destroyers
  (`userdel`, `umount`, destroy), re-creators (`reboot`, kexec, reinstall — identity
  churn: a new [TERM-A] lifetime of the same coordinates). You never enter a departed
  lifetime; a wrapper cannot denote a [TERM-B] because an event is not a place questions
  are asked about. `lend_map` correctly never grows a time key (nothing lends time).
- **Creators are ordinary oracle-described commands** [ACKED 2026-08-01,
  `rul-availability-is-universal`, via the useradd strawman]: `useradd alice` at line 6
  makes the user-alice context arrives-at(6); `sudo -u alice foo` at line 40 guards on
  build days and — the generalized law — **elides on day N, because a CONVERGED creator
  elides, so its context is available at probe time, so entry-probing proceeds
  downstream**. One theorem, previously three instances: the epoch transit-relative law,
  `27C`'s pre-mount chroot story, and the pivot's day-N fold. Silence is today's floor,
  byte-identical: an unmodeled creator means arrival is unknown; probe entry simply fails
  dynamically; can't-say ⇒ guard/run. Guards in arriving contexts are sound BY
  CONSTRUCTION (in-sequence; the context has arrived when the guard runs) — the epoch
  bank's "guards are epoch-proof" becomes a theorem. Destroyers symmetric: sites denoted
  in departed contexts are honestly can't-say ⇒ run. The inverse wait is the trajectory
  available→departed→available; wait-loops with pure-delay bodies mint no events ⇒
  wall-transparency derives (the `26K` sit-wall-transparent-delay-loops ratification
  becomes a corollary to ack, not a standalone rule).
- **The integrity/analysis split is preserved, and reconciles `an-toctou-window`**: a
  planned [TERM-B] is an IDENTIFIED-CAUSE, in-book event (the creator line is the cause)
  — analysis-plane, fails toward run. UNPLANNED lifetime churn (boot-id/host-key
  mismatch) is the integrity plane's withhold
  (`rul-integrity-failure-withholds-mutation`), never drift accounting. The welded
  WONTFIX on unattributed drift stands untouched.
- **Facts carry their context including the [TERM-A] marker**; the controller mints
  lifetime identity (the reserved generation-identity slot in
  `rul-attribution-is-controller-minted` — this plan IS its named re-entry trigger: a
  second scope becomes representable, so carrying scope becomes checking scope;
  `WidthOneAttemptScope` goes multi, deliberately, at stage-iii). Crossing stays the
  kind-owner's `undivided-by-transit-across <axis>` mark — time is one more axis,
  consuming `272` §6's reserved slot. `an-host-as-adversary` honored: availability facts
  are host-scoped intake, bounded; no host speaks for another's availability.
- **Residual host-specialness**, named and fenced: transport mechanics (channel
  capability, `kBOOT`) and the quarantined security concerns (cross-host fact custody,
  hostile-host intake; the kSTATE unparking coupling). Orthogonal to the model.
- **Scope-typing** (the four seam arrivals): a line's execution scope = the world-context
  its bytes run in. Split-books stands until stage-iii lands scope-entry; oracle-declared
  execution scope stays REJECTED (claim authors never relocate someone else's mutations).
  Local-exec: the controller is a context available-at-probe by definition — a supported
  mode falls out of the model rather than being bolted on.
- **kSTATE untouched**: availability is computed per-run from the plan's own structure
  plus probe-time reachability. Nothing persists; nothing is a cache.

## §4 — Corrections to the first synthesis (superseded chat-tier claims)

- The three-way reach-mode split (entered/connected/traversed) is DEAD: two concepts
  only — entry (all dimensions, host included) and [TERM-B] availability events.
- "Helpers ride under the calling entrypoint's closure" was HALF the story (custody);
  the grade direction is the opposite (`rul-blessing-flows-from-best-caller`, §2).
- Host/epoch as "the dimensions with availability" is DEAD: availability is universal
  (§3); host and [TERM-A] are ordinary coordinates whose availability trajectories are
  merely the most *interesting*.
- The prior conductor's "epoch is a temporal cousin, not a fifth sibling" resolves
  with precision: right about `lend_map` membership only; wrong as architectural
  separation. (The `26K` §0b terminology rider is discharged by this document + the
  pending term survey.)

## §5 — Rulings banked this sitting (ack-ledger; grades exact)

- **`rul-blessing-flows-from-best-caller`** [TYPED substance 2026-08-01] — §2. Open:
  `pin-blessing-keying`.
- **`rul-command-v-is-a-stdlib-oracle`** [ACKED 2026-08-01] — no engine blessing beyond
  the one narrow, referent-aware, load-plane capability ("do not analyze `command -v
  <unit-defined-fn>` as a PATH question"; the decidable-set v0 contract, unchanged) plus
  dorc-lang top-level load-inert whitelisting. World-plane `command -v <tool>` questions
  are an ordinary stdlib oracle's cell — probe-measured, self-vouched-by-existence,
  standin-tested now, value-dead until stdlib like everything else. The plane
  disambiguator is the operand's unit-definedness; `floor28-command-v-reads-fn-
  definedness` already pins the load-plane contract's narrowness. This retires the
  engine-blessing half of `28P:adj-command-v-blessing-routed-to-human`; the `.`-blessing
  sibling stays a separate, small, engine-side human ruling (§2).
- **`rul-host-entry-is-ordinary-entry`** [ACKED 2026-08-01] — §3, with the re-parse
  siting rider and `res-host-identity-wants-resolve`.
- **`rul-availability-is-universal`** [ACKED 2026-08-01] — §3, the useradd strawman.
- The three-pillar synthesis direction as a whole [ACKED 2026-08-01: "this all sounds
  coherent to me"].

## §6 — What this plan subsumes (the anti-piecemeal ledger)

Inherited and landed here: bitem4 (the fence build → stage-ii) · bitem5 (split-family
coherence detection, aid-plane, sized per `28M:lean-demotion-is-not-deletion` →
stage-ii) · the withhold-softening (→ stage-i, as true resolution) · the meet-direction
registry (`28P:tc-meet-direction-registry-not-built` → stage-ii, where the lattice
refactor has company) · `26K` §0b entire (local-exec, scope/[TERM-A] slot, wait-loops,
inverse wait → stage-iii) · the wrapped-vouch and whyworld/survival seat asymmetries
(→ stage-i) · `res-host-conditional-loading` gains its eventual story (per-host frames
keyed by decidable host facts) but STAYS v0-refused — named, not scheduled. Made-visible
but NOT ruled here: `tc-split-family-elides-on-two-authors` (composite-license
admissibility; the fence sitting's) · `28M` §11's keep/lift + registration verdicts ·
the tabled word-pooling corner (closure-keyed dialects are its eventual hook; nothing
more).

## §7 — What does not change (the preserved-invariant wall)

All license-plane law: `silence-licenses-nothing` · `inv-top-reject` · the
monologue/custody discipline (`28P` bitem3's types are consumed, not altered — until the
fence sitting rules the two-author composite) · `kFAIL` phase-keying · the sparing
algebra, ternary compare, set-lifting-universal-meet · `never-derive-separation` ·
`rul-only-oracle-bytes-ship` / `rul-argv-flows-bytes-do-not` · the `27C` consent
machinery (dial, vouches, entry-siting, composition algebra) · `two-plane-aid-law` ·
hermeticity-precondition · `rul-strawman-formats-no-compat` (every identity type minted
here renames freely pre-publication). And `syn-zero-new-spellings` (§0).

## §8 — Constraint reconciliations (from the 2026-08-01 needs-ledger extraction)

- `an-flat-domain`/`an-context-key`: reconciled §1; ANALYZER-NEEDS owes the paragraph.
- `funcenv-reads-source-literal-plane-only`: preserved §1.
- `28M` §10 is P2's canonical prior seat — this plan implements it, never re-derives.
- `pinned-definitions-are-the-artifact's-binding`: preserved; plural bodies at distinct
  frames use the already-built hash-munge; never two same-named funcdefs by any route.
- `an-toctou-window` (welded WONTFIX): reconciled §3 (identified-cause only).
- `rul-attribution-is-controller-minted`: its re-entry trigger fires at stage-iii by
  design; scope-carrying becomes scope-checking there and not before.
- The expense center is stage-i's TOUCH-COUNT (the resolution seats + `build_vouches`'
  map shape), not asymptotics — O(frames) is bounded by env-mutating statements and the
  corpus's common case is one frame. Size this first in the stage-i brief.
- `AID:law-lineno-identity` · `law-whylog-is-sensitive`: untouched by all three pillars;
  any stage that persists availability or frame state violates rec-5 and is out.

## §9 — Staging (each stage independently green; gates named)

- **stage-i-definition-factoring** (P1): DefinitionId keying of derived rows; the frame
  indirection at every resolution seat; retire the agreement veto, `live_source`, and
  `never_live` as separate mechanisms; unify the wrapped-vouch and whyworld/survival
  seats. Gate: `syn-single-frame-byte-identical` (full corpus, both legs). Unblocks:
  plural-idiom fixtures (expect new corpus cells, not churn).
- **stage-ii-closure-custody** (P2): the `DefinitionCustody` internal re-key; the
  frame-relative, closure-keyed fence (build-as-spiked, UNRATIFIED marker stands until
  the fence sitting); kind-owner occupancy per closure; blessing-reachability for
  vocab-minting (pending `pin-blessing-keying`); bitem5's coherence detection; the
  meet-direction registry. Gate: single-closure byte-identity + the fence's six stage-E
  cells re-verified closure-keyed.
- **stage-iii-world-scopes** (P3): the context-slot host×[TERM-A] coordinates;
  availability computation from creator/destroyer/re-creator descriptions; scope-entry
  (ssh as entry; local-exec as the controller context); the scope types go multi with
  checking; wait-loop wall-transparency and the inverse wait as derived behavior. This
  stage IS the former `26K` §0b batch, human-led at its design edges (the §11 authored
  surface must be settled first for creator description). Gate: a no-[TERM-B] book is
  byte-identical; the pivot strawman book renders honestly in both world-states.

Builder on-ramp (read in order): this document → `28M` §§7–11 → `28K` (executed lane
record; §10's as-built bitem ledger `28P`) → `27C` → the `spike/CLAUDE.md` invariant
sections cited in §7/§8. Unchanged pending work lives where it lived: `26K` §0a fruit
arc + §5 sittings queue · the three parked rulings (guard-tier classed-decline ·
records-8 · D9) in LIVING_STATUS · `tc-inert-mocks-rail-is-dash-shaped` (separate lane) ·
the loom Windows stack-overflow (separate small fix).

## §10 — Open pins and owed rulings (the complete list; nothing else is open here)

1. `pin-blessing-keying` — family-rooted vs closure-global (§2; one line).
2. `res-dot-blessing-is-engine-side` — the `.`-of-proven-load-inert blessing (§2;
   human-owned; gates P2's book-side payoff).
3. The stdlib `command` oracle — authoring rides the stdlib revival (which remains
   ALSO gated on the dialect-reach decision, `28O:fnd-dialect-tests-admit-only-string-
   comparison`; unchanged).
4. [TERM-A]/[TERM-B] — the terminology survey's recommendation, human-picked; this
   document then renames in place (`rul-strawman-formats-no-compat`).
5. The composite-license admissibility ruling (`tc-split-family-elides-on-two-authors`)
   and the fence's permanence — the committee-corner sitting, parallel, unscheduled.
6. `28M` §11's keep/lift + registration verdicts — parallel, human's.
7. The ANALYZER-NEEDS flat-domain reconciliation paragraph — conductor, at stage-i fold.
8. `an-atmost-completion-signal` (exit-0 truncation) — pre-existing, human-owned,
   unchanged by this plan; listed so nobody reads §3's atomicity inheritance as closing it.

## §11 — The authored surface (RESERVED)

How users express these concepts in oracles — creator/destroyer description members or
marks, the [TERM-B] classing spelling, whether `undivided-by-transit-across` gains the
time axis by mark or by kind-owner member, the ssh entry-form's authored half, and the
sent-language-is-becoming-crufty ceremony budget for all of it — is the next design dig,
human-led. Deliberately empty until that sitting.
