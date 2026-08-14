# 28Q — The context-kernel unification: frames, closures, availability (THE refactor plan)

> Tier: LLM-authored plan (Fable conductor, from a human design dialogue, session
> `r28-megamerge`); subordinate to root docs and `spike/CLAUDE.md`. Grades as in
> `28K`/`28M`: **[TYPED]** human typed it · **[ACKED]** substance confirmed in dialogue ·
> **[PROPOSED]** conductor-derived, awaiting ratification. The three-pillar direction is
> ACKED; §4 is this plan's ack-ledger; mechanics are PROPOSED unless marked otherwise.
>
> **This is the single home for the analysis-kernel refactor** — the human's standing
> order is NO MORE PIECEMEAL work on this territory. It SUPERSEDES: the
> `28P:amend-plural-value-loss-hold` standing order (bitem4/bitem5/withhold-softening
> were held for exactly this plan; they land here as stage work) and `26K` §0b's
> kernel-batch agenda (absorbed into stage-iii; `26K` remains live for its §0a fruit arc
> and §5 sittings queue). It does NOT supersede `plans/27C` (still THE
> wrapper/context-entry spec — §3 here generalizes its availability behavior, touching
> none of its consent machinery) nor `28M` (still the committee-corner authority; §2 here
> consumes its §10 direction).
>
> Vocabulary, fixed for this territory [ACKED]: an **incarnation** is the lifetime
> marker of a re-creatable context (nearest precedent: TCP's connection incarnations,
> RFC 9293 — same identity coordinates, new lifetime; see §3 for the deliberately
> softer Dorc definition). A **lifecycle event** is the class of commands that change a
> context's availability, with exactly two primitives — a context **begins** or
> **ends** — and re-creation defined as their composition. An **availability window**
> is the interval of plan positions over which a context is available. "Epoch",
> "pivot", and "transit" are retired from this territory; older documents using them
> read through this vocabulary — via this map, since the old words SPLIT: "epoch" →
> incarnation (the lifetime) or availability window (the interval), by context;
> "transit" → a lifecycle event where reboot-shaped, context entry/exit where
> wrapper-shaped (`26K`'s transit-relative law reads under the first); "pivot" → host
> arrival + scope-entry.

## §0 — The design in one screen

Three asks, one model. (P1) Every kernel-derived truth that flows through a
can-be-committee API entry point — function resolution, argparse/identity application,
cell indices, verdict sets, dialect enrollment, footprints, helper closures — becomes
**positionally indexed**: keyed to the environment frame it is asked from, so loading a
file inside a subshell appends/shadows engine knowledge for questions asked in that
region. (P2) Every "owner"-keyed concept re-keys from single-file to the **entry-closure**
— the transitive closure of literal `.`-sourcing from an entry file — so authors factor
helpers into second files with no registry, package name, or manifest: sourcing IS the
package boundary, spelled in sh. (P3) World contexts (user, fs-view, netns, host, and
the incarnation axis) gain a single **availability predicate** over plan position;
probing enters only currently-available contexts; commands that begin or end contexts
are ordinary oracle-described commands; the old epoch-specific transit-relative law
generalizes to every dimension.

The unifying claim, `syn-one-context-two-planes`: every kernel query carries a two-sided
context — *where in the text am I asking from* (the load plane) and *where/when in the
world am I asking about* (the world plane) — and both planes obey ONE discipline:
piecewise-constant truth between events, region-scoped shadowing, and per-name/per-kind
crossing claims for what survives an event.

| | load plane (P1) | world plane (P3) |
|---|---|---|
| events | funcdef · `unset -f` · `.`-source · subshell open/close | walls (unmodeled mutators) · lifecycle events (a context begins/ends) · context entry/exit |
| regions | environment frames | availability windows; the intervals where a probed fact stays trustworthy (the wall machinery already computes these) |
| scoped shadowing | subshell re-source (dies at the paren) | an entered context (`sudo`/`chroot` region; an ssh'd host region) |
| crossing claims | a name survives events that do not touch it (sh-given) | a fact survives a lifecycle boundary iff its kind-owner marked `undivided-by-transit-across <axis>`; survives a wall iff footprint-disjoint |
| silence | unresolvable load ⇒ ⊤, walls | unmodeled command / unmarked kind ⇒ wall |

`syn-frame-tree-is-fork-semantics` — the load plane's "resolution DAG" is literally sh's
fork semantics: a subshell is a forked child environment whose mutations are invisible to
the parent; `.` is in-process mutation. The DAG is the fork tree crossed with linear
program order. PLT-standard vocabulary: these are scoped environments over a program
order; no coinage needed. The world plane is the same shape one level up: the apply is a
program order over world state; hosts and incarnations are the scopes; lifecycle events
are the mutations.

`syn-zero-new-spellings` — all three pillars consume EXISTING sh acts: sourcing,
subshells, `unset -f`, creator/destroyer commands with ordinary oracles, ssh lines.
Scope, sharpened at the `28R` review (every lane converged on the overclaim): this
binds the ADMIN/book surface only. P3's load-bearing input — how an oracle says a
command begins or ends a context — has NO existing spelling; that oracle-member
vocabulary is §10's priced budget, and this slug is never citable against it. It must
clear the sent-language-is-becoming-crufty bar.

## §1 — P1: definition-factored indices (`syn-definition-factored-indices`)

The as-built gap: `KindIndex`, `VerdictIndex`, the effect map, the dialect, and
`HelperIndex` are whole-unit merges, which is why bitem0 settled for the agreement veto
(`28P:dec-the-gate-is-agreement-not-re-resolution`) — true positional resolution over
merged indices risks the chimera (identity through one author's argparse, cells from
another's; pope-sin, invisible to goldens). The fix is a FACTORING, not per-frame copies:

1. Every derived row — a check, a cell declaration, an argparse arm-model, an enrolled
   dialect token, a footprint claim — is keyed by the **DefinitionId** that produced it:
   (SourceFileId, span, custody). Computed once, whole-unit, exactly as today. No index
   multiplication. HELPER CLOSURES are the ruled exception
   (`28R:§snapshot-emission-adjudication`, TYPED/ACKED): sh binds a body's calls at
   INVOCATION, so a closure is a property of the consuming FRAME, never of the
   definition — "computed once, whole-unit" is DEAD for closures
   (`rul-resolution-matches-shell-loading`). Stage-i ships the WITHHOLD floor (a frame
   whose live definition closes over a helper name that is plural-with-differing-bytes
   across frames withholds; `helper-declaration-contested` stands; constants per
   `28P:dec-constants-ride-per-contributing-file` likewise); the snapshot-transplant +
   instantiation-hash-dedup emission (`rul-snapshot-transplant-emission` ·
   `rul-instantiation-hash-dedup`) is its own stage between i and ii (§8).
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
  frame. The exact-subtraction consequence (`28P:adj-never-live-exactness-accepted`) is
  NOT dissolved — it GENERALIZES: under true resolution every funcenv precision bug is
  winner-shifting (it selects whose judgment governs a site, with no agreement veto
  behind it), so the whole frame solver is license-review-tier forever; funcenv
  precision work is never ordinary value-add. `oracle/CLAUDE.md
  live-source-is-the-only-resolution-seat` becomes "the frame-lookup is the only
  resolution seat".
- bitem1's built-but-unreachable hash-munge becomes reachable exactly as its ledger
  predicted (two frames, two live bodies, two munged names); the pinned-definitions
  machinery needs no redesign — `plan/CLAUDE.md pinned-definitions-are-the-artifact's-
  binding` already handles per-guard-site binding, and the positional regime already
  guarantees definition-precedes-guard.
- The seventh seat (`build_wrapped_vouches`, `28P:tc-wrapped-vouch-seat-has-no-positional-
  gate`) unifies with the other six. The whyworld/survival seats are NOT free riders:
  `28P` priced their unification as a dispatch ("re-lifting that seat's whole world" —
  `res-the-why-world-cut-is-now-visible`), and stage-i carries that price tag. And
  `tc-wrapped-lane-drops-a-case-bodied-in-book-verdict` was measured, never diagnosed —
  the oracle-only-vector reading is a HYPOTHESIS, so the case-bodied in-book wrapped
  fixture rides stage-i as an EXPECTED-TO-FLIP cell (the asserted cause gets tested,
  not trusted).

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

Closure-identity, stated as a function (the `28R` review found the bare
one-identity-per-entry reading DISSOLVES the fence — a book sourcing two strangers'
files would make them one speaker): custody units root PER SOURCED SUBTREE, never per
top entry. `28M` §10's carve is the rule — the not-co-author machinery binds only
SIBLING/COUSIN edges in the include-tree; an ANCESTOR edge takes custody of what it
sources — so the two-strangers book holds two mutually-FENCED sibling units under its
own custody of the composition. Three cells stay OPEN and are §9's, owed before
stage-ii: overlap/diamond identity (one shared helper file under two entries;
byte-dedup does not pick a speaker); closure MEMBERSHIP (sourcing-only vs the
CLI-sibling-loaded package shape bitem6 proved — `28M` §7 keys helper custody to the
CALLING entrypoint, which is caller-keyed, not closure-keyed); and the oracle-side
spelling itself (the payoff-gate paragraph below).

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
- **Diamond loading**: byte-identical files dedup (built) — sound TODAY only because
  every closure is a singleton (no oracle-side `.` exists); once sourcing lands, dedup
  re-keys to RESOLUTION-identity (bytes × what the file's own sourcing resolved to —
  `28R:fnd-dedup-keys-to-resolution`, the key `rul-instantiation-hash-dedup` already
  consumes at the ship seam) or the divergence is shown unrepresentable; differing
  bytes refuse (built);
  the fence binds only sibling/cousin edges in the include-tree, never above/below
  (`28M` §10 direction). Unit-identity keys to the defining file within the closure
  (version-skewed vendored copies refuse rather than dedup — the bitem1 rider restated
  closure-relative).

**`rul-blessing-flows-from-best-caller` [TYPED substance]** — custody and GRADE are
different relations and flow in different directions. Custody: the speaker is the
closure; helpers are subordinate for fence/attribution purposes. Grade
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
over sh semantics only — referent-agnostic-clean (`res-dot-blessing-is-engine-side`). Two sibling gates the
review surfaced: the ORACLE-side spelling does not exist either — a marked file's
load-inertness refuses top-level `.` (verified in code:
`oracle::load_inert::item_is_load_inert` admits only funcdefs + static assigns), so
every closure today is a singleton and the load-inert `.`-amendment is a §9 owed
ruling; and the RUNTIME prerequisite `28K:res-book-ships-its-load-closure` (a sourced
book needs its closure present at apply; named there, unbuilt) joins stage-ii's scope —
without it the dot-blessing lands analysis-green and runtime-dead.

## §3 — P3: availability over all contexts (`syn-availability-is-universal`)

Every world-context — the folded per-dimension key of `27C`'s composition algebra, now
including a host coordinate and an incarnation marker — carries an **availability
status as a function of plan position**: available · arrives-at(p) · departed-at(p) ·
never. Probe-time entry is availability at position zero. The domain statement,
sharpened at review (`28R`): those statuses are consumed MUST/MAY-split — entry
consumes only MUST-available (a conditionally-reached begin never licenses entry; a
MAY-run end conservatively distrusts — the wall machinery's existing direction); CFG
joins and loops land on unknown ⇒ guard/run — as the v0 FLOOR, not the model
[human, 2026-08-13]: the fact-lattice is EXPECTED to grow richer divergence-tracking
(per-branch world-states, rather than collapse-to-unknown at every conditional
creator); nothing may build a dependant or expectation on the collapse being
permanent; the richer resolution is a reserved seat, deliberately un-designed here.
And a window is a position-SET (an
interval LIST with gaps — the re-creation gap is structural), never a (start, end)
pair (`28R:vd-window-is-point-set`). A converged (elided) lifecycle command mints NO
event: no boundary, no fresh incarnation. Everything else derives:

**The incarnation, defined softly and on purpose** [the definitional paragraph;
human-directed]: an incarnation distinguishes LIFETIMES of one context — same identity
coordinates, new instance — for *keying* and *default distrust*, never as a hard
partition. The prior art this word comes from gets to divorce lifetimes completely (an
old TCP incarnation's segments are simply dead); Dorc deliberately does not, because
much of this design's value lives in CORRELATING instances across a lifecycle boundary.
The default is the ordinary floor: silence walls — a fact keyed to incarnation N
licenses nothing in N+1. Carry is the ordinary claims surface: the kind-owner's
crossing marks, unchanged (hardware/arch kinds survive a reboot; package state does
not). And one door is held open, named and NOT designed here
(`res-incarnation-correlation-door`, §9): cross-incarnation correlation/equivalence —
assessing a recreated instance as "the same thing" (the input-addressed-identity
family: built from the same recipe, converged as the same state). Without some such
semantic, a book containing an UNCONDITIONAL destroy-recreate can never be converged
downstream of it, on any day. Both poles are real: there are books where hard divorce
is precisely the hole this model closes, and books where the recreated instance must
be assessable as continuous; the subtleties belong to a later sitting.

- **Entry is uniform across dimensions** [ACKED, `rul-host-entry-is-ordinary-entry`]: a
  wrapper chain, an ssh line, or ambience denotes a context; probing enters it if
  available, under `27C`'s unchanged consent machinery (dial × vouch × entry form).
  `ssh host …` IS a peeling wrapper whose entry form is the connection dance
  (`ack-connection-dance-oracles-core` unchanged: the ssh oracle's own arms mint
  reachability facts; the engine mints nothing). The wrapper-law's "execs the remainder
  locally" fence is mechanical, not categorical — ssh's genuine extra obligation is the
  remote-shell RE-PARSE round-trip, which is exactly its entry-siting vouch discharge
  (and why the engine's own transport ships artifacts on stdin, `260` §5). Host ALIASING
  (`~/.ssh/config`, short names, IPs) is package-name aliasing's cousin: host identity is
  resolve-machinery territory, never string comparison
  (`res-host-identity-wants-resolve`). Three review carves bind this bullet: (1) the
  AMBIENT-HOST carve — the CLI-named target is entered by consent-of-invocation
  (running `dorc plan book.sh web1` IS the consent); dial × vouch × entry govern only
  in-book host shifts (without the carve, uniform entry would demand vouches for
  baseline probing — reductio). (2) Host identity must NOT ride the kind-resolver
  CONTRACT: for hosts a wrong MERGE is not conservative — it lets one host's
  measurement license another host's elision (the inverted failure direction vs
  USER_STORY's resolver pricing; `260`'s HostId-verbatim law and
  `an-host-as-adversary` both bind). v0: no host merging, duplicate probing is the
  price; any future merge is controller-authenticated identity, never string- or
  resolver-tier. (3) The ssh entry form CANNOT be `"$@"`-verbatim across the remote
  re-parse — this bullet and `27C`'s only-entry-shape ruling genuinely conflict; an
  owed ruling (§9), and the re-parse is `24T`'s payload-decomposition problem in
  network clothing. Probing hosts named IN books also widens the probe's network
  footprint onto the plan critical path — RULED [TYPED substance, 2026-08-13]:
  book-mentioned hosts require additional explicit consent, a `--fan-out`-shaped flag
  (name STRAWMAN). The line is CLI-authorization, not host-count: every CLI-named
  target is fair game (multiple targets, each its own plan); any host one hop further
  — denoted only inside a book — is not entered without the flag (honest walls
  otherwise). Residual policy under the flag (lazy entry / timeout budgets) stays
  stage-iii brief material, and `27C:render-authority-disclosure` grows a host
  coordinate.
- **Lifecycle events are not a reach-mode** — they are world-mutations that move
  contexts' availability, and there are exactly two primitives: a context **begins**
  (`useradd`, `mount`, `doctl compute droplet create`) or **ends** (`userdel`, `umount`,
  destroy). A re-creation (`reboot`, reimage, kexec) is an end composed with a begin —
  and a begin following an end is what MINTS a fresh incarnation of the same
  coordinates. You never enter an ended lifetime; a wrapper cannot denote a lifecycle
  event because an event is not a place questions are asked about. `lend_map` never
  grows a time key — nothing lends time (the surviving kernel of the older
  cousin-not-sibling reading; the rest of that separation is dissolved by this plan).
- **Begin/end-describing commands are ordinary oracle-described commands** [ACKED,
  `rul-availability-is-universal`]: `useradd alice` at line 6 makes the user-alice
  context arrives-at(6); `sudo -u alice foo` at line 40 guards on build days and — the
  generalized law — **elides on day N, because a CONVERGED creator elides, so its
  context is available at probe time, so entry-probing proceeds downstream**. One
  theorem, previously three instances: the old epoch transit-relative law, `27C`'s
  pre-mount chroot story, and the pivot-book day-N fold. Silence is today's floor,
  byte-identical: an unmodeled creator means arrival is unknown; probe entry simply
  fails dynamically; can't-say ⇒ guard/run. Guards in arriving contexts are sound BY
  CONSTRUCTION (in-sequence; the context has arrived when the guard runs). Destroyers
  symmetric: sites denoted in departed contexts are honestly can't-say ⇒ run. The
  inverse wait is the trajectory available→departed→available; wait-loops with
  MODELED-PURE delay bodies mint no events ⇒ wall-transparency derives — positively
  modeled purity only; an unmodeled body walls as ever, absence-of-events is never the
  license (the `26K`
  sit-wall-transparent-delay-loops ratification becomes a corollary to ack, not a
  standalone rule).
- **The integrity/analysis split is preserved, and reconciles `an-toctou-window`**: a
  planned lifecycle event is an IDENTIFIED-CAUSE, in-book event (the creator line is the
  cause) — analysis-plane, fails toward run. UNPLANNED lifetime churn (boot-id/host-key
  mismatch) is the integrity plane's withhold
  (`rul-integrity-failure-withholds-mutation`), never drift accounting. The welded
  WONTFIX on unattributed drift stands untouched.
- **Facts carry their context including the incarnation marker**; the controller mints
  incarnation identity (the slot `rul-attribution-is-controller-minted` reserves — this
  plan IS its named re-entry trigger: a second scope becomes representable, so carrying
  scope becomes checking scope; `WidthOneAttemptScope` goes multi, deliberately, at
  stage-iii). Crossing stays the kind-owner's `undivided-by-transit-across <axis>` mark
  — the lifetime axis mints a NEW axis value (`272` §6's "time" row is the Linux
  time-NAMESPACE, pidns's sibling, not a temporal slot; it stays reserved for timens). `an-host-as-adversary`
  honored: availability facts are host-scoped intake, bounded; no host speaks for
  another's availability.
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

## §4 — Rulings banked (this plan's ack-ledger; grades exact)

- **`rul-blessing-flows-from-best-caller`** [TYPED substance] — §2. Open:
  `pin-blessing-keying`.
- **`rul-command-v-is-a-stdlib-oracle`** [ACKED] — no engine blessing beyond the one
  narrow, referent-aware, load-plane capability ("do not analyze `command -v
  <unit-defined-fn>` as a PATH question"; the decidable-set v0 contract, unchanged) plus
  dorc-lang top-level load-inert whitelisting. World-plane `command -v <tool>` questions
  are an ordinary stdlib oracle's cell — probe-measured, self-vouched-by-existence,
  standin-tested now, value-dead until stdlib like everything else. The plane
  disambiguator is the operand's unit-definedness; `floor28-command-v-reads-fn-
  definedness` already pins the load-plane contract's narrowness. This retires the
  engine-blessing half of `28P:adj-command-v-blessing-routed-to-human`; the `.`-blessing
  sibling stays a separate, small, engine-side human ruling (§2).
- **`rul-host-entry-is-ordinary-entry`** [ACKED] — §3, with the re-parse siting rider
  and `res-host-identity-wants-resolve`.
- **`rul-availability-is-universal`** [ACKED] — §3, the useradd strawman.
- **The vocabulary family** [ACKED] — incarnation · lifecycle begins/ends with
  re-creation as their composition · availability window; with the softened incarnation
  definition (§3) and its correlation door held open.
- **`rul-verdict-primacy-at-the-ship-seat`** [TYPED substance, 2026-08-01 sitting] — at a
  vouched, mutation-capable site the VERDICT body ships as the probe check and its own
  reached answer is the convergence measurement; prediction never licenses elision. The
  as-built predict-wins preference (`verdict-lane-is-site-keyed`'s fallback ordering) is
  an unratified expedient inherited from the round-23 single-function era — re-cut at
  stage-0. The W-B keying-coherence half (the record keys the cell the shipped body
  measures) survives the inversion. Retires `28P:tc-split-family-elides-on-two-authors`
  at the license tier: the elide is a monologue again (one author's body, rc, and vouch);
  the cross-author residue is the sparing tier's, where the fence already stands.
- **`rul-erasure-license-splits-by-effect-class`** [ACKED] — every erasure rests on an
  authored license plus a probe measurement. Write-shapes (derived Establishes/Kills)
  elide only on the verdict's own answer + vouch; consumed-status stand-ins stay
  vouch-derived act-as-succeeded at the built conservative boundary
  (only-where-no-consumer-can-tell). Read-shapes (derived Pure — the model
  self-delegates the shape; the structural self-vouch) replace only with probe-MEASURED,
  delegation-produced values (`271:rul-composed-bytes-defer-and-floor`). Opaque/⊤ walls.
  Composes existing law (`an-elide-weld` · `rul-every-erased-establish-is-vouched` ·
  `28M:rul-predict-feeds-plan-never-apply`); banked so the split is citable in one place.
- **`rul-declared-observable-substitution-is-dead`** [TYPED substance] — `19A` §5's
  declared would-produce substitution and `20V` door-2 are dead-on-principle, not merely
  unbuilt: predict authors are trusted for convergence-tier promises that provably fail
  toward run, never for changes-what-your-book-does-at-runtime authorship.
  `28M:rul-predict-feeds-plan-never-apply` is the standing law; supersession markers sit
  at the sources, and the four stale `plan/src/lib.rs` comments die at stage-0.
- The three-pillar direction as a whole [ACKED — the DIRECTION only, never the
  mechanics; the per-item grades above govern, and this line is not citable for any
  PROPOSED mechanism].

## §5 — What this plan subsumes (the anti-piecemeal ledger)

Inherited and landed here: bitem4 (the fence build → stage-ii) · bitem5 (split-family
coherence detection, aid-plane, sized per `28M:lean-demotion-is-not-deletion` →
stage-ii) · the withhold-softening (→ stage-i, as true resolution) · the meet-direction
registry (`28P:tc-meet-direction-registry-not-built` → stage-ii, where the lattice
refactor has company) · `26K` §0b entire (local-exec, scope/incarnation slot,
wait-loops, inverse wait → stage-iii) · the wrapped-vouch and whyworld/survival seat
asymmetries (→ stage-i) · the ship-seam verdict-primacy re-cut (→ stage-0; rulings in
§4) · `res-survival-lanes-still-ship-closure-less` (`cli/CLAUDE.md
one-helper-index-two-lanes` → a rider on the emission stage's closure machinery) ·
`res-host-conditional-loading` gains its eventual story
(per-host frames keyed by decidable host facts) but STAYS v0-refused — named, not
scheduled; NB that eventual story is an AMENDMENT of
`funcenv-reads-source-literal-plane-only`, never an extension — the law's edge stays
crisp until then. Made-visible but NOT ruled here: `28M` §11's keep/lift +
registration verdicts · the tabled word-pooling corner (closure-keyed dialects are its
eventual hook; nothing more).

## §6 — What does not change (the preserved-invariant wall)

All license-plane law: `silence-licenses-nothing` · `inv-top-reject` · the
monologue/custody discipline (`28P` bitem3's types are consumed, not altered — until the
fence sitting rules the two-author composite) · `kFAIL` phase-keying · the sparing
algebra, ternary compare, set-lifting-universal-meet · `never-derive-separation` ·
`rul-only-oracle-bytes-ship` / `rul-argv-flows-bytes-do-not` · the `27C` consent
machinery (dial, vouches, entry-siting, composition algebra) · `two-plane-aid-law` ·
hermeticity-precondition · `rul-strawman-formats-no-compat` (every identity type minted
here renames freely pre-publication). And `syn-zero-new-spellings` (§0). Plus the
two-planes fence, stated once so no later lane misreads §0: the unification is
DISCIPLINE, never implementation — the load plane never grows a probe-data input
(`funcenv-reads-source-literal-plane-only` is permanent) and the world plane never
inherits load-plane certainty (its truths are measured/vouched/claimed, not
sh-given). The world plane's own precedent is the project's founding one:
available-expressions/PRE — piecewise truth over program points with kill events
(README's lazy-code-motion frame) — worth handing stage-iii builders by name.

## §7 — Constraint reconciliations (against the needs-ledgers and as-built law)

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
- `notes/28T` (the correctness-tooling plan) rides every stage: solver answers pass the
  post-fixpoint certifier and survival verdicts the reference re-derivation, with the
  aid-plane's own gates voting separately (the two-plane firewall — either plane kills a
  kernel change). Frame-plural fact-sets (stage-i) and closure-keyed compares (stage-ii)
  are certified/re-derived exactly as single-frame ones; a certification `Refused` on the
  new shapes is a finding, never churn. New core state structures follow the owned-facade
  law (`28T` w1-latticemap-facade) so the strict core stays checkable and translatable;
  the verified mini-model of the sparing algebra tracks any stage that moves
  compare/dialect/backing semantics (the two-position rule lands there first).

## §8 — Staging (each stage independently green; gates named)

Every stage additionally inherits `28T`'s checker gates — certifier + sparing
re-derivation green over the full corpus, both planes' votes — alongside its own
byte-identity gate.

- **stage-0-ship-seam** (the verdict-primacy re-cut; a deliberate behavior change,
  deliberately OUTSIDE stage-i's byte-identity gate): invert the ship-seat preference —
  at a vouched site the verdict body ships and measures (via its own marks or the
  auto-cell); the predict's argparse/cells keep feeding the static concern topology
  unchanged; ship-predict-alone stays licensed only where elision is already statically
  unavailable. Rip the four `19A`-era `plan/src/lib.rs` comments; re-bless
  `pin28-split-family-lane-separation` (the verdict body now runs — the monologue
  restored). Gate: churn confined to probe-artifact bytes/records plus the flipped
  fixture; site OUTCOMES byte-stable across the corpus (an outcome move is a finding,
  never churn). RETROACTIVE fold audit (stage-0 is built; `28R` found the outcome gate
  blind to this class): a records/fact-set diff over the corpus — site outcomes can
  hold while a named predict cell silently becomes an unmeasured auto-cell, and
  backings, survival, and why-chains consume those records; a lost measurement is a
  finding.
- **stage-i-definition-factoring** (P1): FIRST, commission the plural-idiom fixtures as
  DIFFERENTIAL cells (sentinel bodies under the two-binary floor — the bitem8 pattern:
  blessed-override above/below its `unset -f` · subshell re-source in/out · the
  helper-collision cell · munge reachability · the deep-stack caller-cone per the
  `28R:§snapshot` residue) — the byte-identity gate is VACUOUS on today's
  single-definition corpus exactly where the new machinery decides, so the fixtures
  land BEFORE the conversion, never as unblocks (the lane's own measure-first
  precedent: bitem6, item0). These are GROUND-TRUTH manifests, not behavior pins: the
  committed expectations are the real shells' own answers, measured once, never
  churned; the engine-agreement half of each cell activates when stage-i lands.
  Ordinary golden corpus cells for the plural idioms arrive AFTER the behavior lands
  (the human's lean, 2026-08-13: pin the future, never pin the hole) — the one
  deliberate exception is the case-bodied wrapped EXPECTED-TO-FLIP cell, kept as the
  diagnostic on §1's asserted cause. Then: DefinitionId keying of derived rows; the frame
  indirection at every resolution seat; retire the agreement veto, `live_source`, and
  `never_live` as separate mechanisms; unify the wrapped-vouch and whyworld/survival
  seats (priced per `28P` — a re-lift dispatch, not a rename); helper closures take
  the WITHHOLD floor (§1); the case-bodied in-book wrapped fixture rides as
  EXPECTED-TO-FLIP; `task-verify-definition-vector-walls` (`28R:§snapshot` residue).
  Gate: `syn-single-frame-byte-identical` (full corpus, both legs) AND the
  differential cells agreeing with the frame answer.
- **stage-emission-snapshot-transplant** (between i and ii;
  `28R:dec-stage-sequencing-withhold-floor` — named here so it cannot read as
  piecemeal): the `28R:§snapshot-emission-adjudication` rulings, TYPED/ACKED —
  snapshot-transplant emission + instantiation-hash dedup (bare/munged/withheld tiers;
  oracle-custody names only, book bytes never rewritten); defensive mode keyed to real
  in-process definition vectors only; mixed-custody vouch suspension (BUILD, with its
  day-one decline class and end-to-end why-chain); contested-name decline. Rider:
  `res-survival-lanes-still-ship-closure-less` (the cheap `HelperIndex` extension).
  Gate: a single-frame, collision-free world ships byte-identical (the sitting's own
  migration gate).
- **stage-ii-closure-custody** (P2): the `DefinitionCustody` internal re-key; the
  frame-relative, closure-keyed fence (build-as-spiked, UNRATIFIED marker stands until
  the fence sitting); kind-owner occupancy per closure; blessing-reachability for
  vocab-minting (pending `pin-blessing-keying`); bitem5's coherence detection; the
  meet-direction registry. Gate: single-closure byte-identity + the fence's six stage-E
  cells re-verified closure-keyed, PLUS the runtime half:
  `28K:res-book-ships-its-load-closure` (closure materialization at apply) with an
  EXECUTING e2e cell — the original book under its sourced tree, the artifact from an
  isolated cwd, and a missing sourced file failing honestly. Split discipline: the
  custody INFRASTRUCTURE (types, provenance, closure computation) is buildable now and
  license-inert; the POLICY consumers (the fence's permanence, `pin-blessing-keying`,
  the §9 membership cells) land only with their rulings — green on infrastructure
  never asserts policy.
- **stage-iii-world-scopes** (P3): the context-slot host×incarnation coordinates;
  availability computation from begin/end descriptions; scope-entry (ssh as entry;
  local-exec as the controller context); the scope types go multi with checking;
  wait-loop wall-transparency and the inverse wait as derived behavior. This stage IS
  the former `26K` §0b batch, human-led at its design edges (the §10 authored surface
  must be settled first for lifecycle description). Structure: definite-availability
  mechanics build against §3's domain statement; the authored begin/end surface is
  §10-gated; the correlation door stays its own sitting. Named dependency: ssh-as-entry
  needs the ssh oracle, which rides the stdlib revival (itself gated on the
  dialect-reach decision — §9 pin 3). Gate: a book with no lifecycle events, no
  host-denoting lines, and no local-exec is byte-identical (the stage's own
  ssh/local-exec behavior is deliberately outside the byte gate); the pivot strawman
  book renders honestly in both world-states.

Builder on-ramp (read in order): this document → `28M` §§7–11 → `28K` (executed lane
record; §10's as-built bitem ledger `28P`) → `27C` → the `spike/CLAUDE.md` invariant
sections cited in §6/§7 → `notes/28T` §1 (the checker/facade riders) + the
`verified-core-discipline` skill (loads itself when a builder nears the strict core). Unchanged pending work lives where it lived: `26K` §0a fruit
arc + §5 sittings queue · the three parked rulings (guard-tier classed-decline ·
records-8 · D9) in LIVING_STATUS · `tc-inert-mocks-rail-is-dash-shaped` (separate lane) ·
the loom Windows stack-overflow (separate small fix).

## §9 — Open pins and owed rulings (the complete list; nothing else is open here)

1. `pin-blessing-keying` — family-rooted vs closure-global (§2; one line).
2. `res-dot-blessing-is-engine-side` — the `.`-of-proven-load-inert blessing (§2;
   human-owned; gates P2's book-side payoff).
3. The stdlib `command` oracle — authoring rides the stdlib revival (which remains
   ALSO gated on the dialect-reach decision, `28O:fnd-dialect-tests-admit-only-string-
   comparison`; unchanged).
4. `res-incarnation-correlation-door` — cross-incarnation correlation/equivalence
   semantics (§3): when may a recreated instance be assessed as continuous with its
   predecessor, so that a book with an unconditional destroy-recreate can still
   converge downstream? Human head-state: sometimes yes (equivalent-by-construction),
   sometimes the hard divorce is exactly right; wants its own sitting; nothing here
   builds toward either pole.
5. The fence's permanence and the sparing-tier composite questions — the
   committee-corner sitting, parallel, unscheduled. (The license-tier half,
   `tc-split-family-elides-on-two-authors`, is RESOLVED by §4's verdict-primacy ruling.)
6. `28M` §11's keep/lift + registration verdicts — parallel, human's.
7. The ANALYZER-NEEDS flat-domain reconciliation paragraph — DISCHARGED (conductor,
   2026-08-13 catch-up pass; `an-flat-domain`/`an-context-key` now carry the §1 text).
8. `an-atmost-completion-signal` (exit-0 truncation) — pre-existing, human-owned,
   unchanged by this plan; listed so nobody reads §3's atomicity inheritance as closing
   it.
9. The oracle-side load-inert `.` amendment (a marked file sourcing a proven-load-inert
   file) — without it P2's closures stay singletons; a dialect-surface widening,
   human's (§2).
10. Closure MEMBERSHIP + overlap/diamond identity — the §2 open cells; owed before
    stage-ii's policy half. The diamond half is MECHANICALLY ACKED [human, 2026-08-13,
    skepticism recorded]: no single global rule is built — which inferred traits of
    sourced-ness propagate along `.`-edges, and where composing such speech is proper,
    is decided PER-SPECIFIC-QUESTION-asked-of-the-dependency, each consumer with its
    own must/may conservatism (three instances already ruled this way: custody-by-path ·
    grade-by-best-caller · dialect-by-licensed-closure). The conductor's
    sourcing-is-a-claim reframe and its consequences (e.g. one-utterance-one-speaker
    under resolution-keyed dedup dissolving `28R:fnd-diamond-fires-the-fence`) are
    NOT settled — the human is explicitly suspicious they close everything; revisit at
    the membership sitting; nothing builds on the reframe beyond the no-global-rule
    floor.
11. The ssh entry-shape ruling — `"$@"`-verbatim-only vs a carved ssh shape (`27C`'s
    punted in-guest-preamble decision reopened, or ssh carved out; `24T` is the
    prior) — rides the §10 dig's agenda (§3).
12. The two-position sparing rule — which position's closure/dialect governs the
    claim@p × backing@q meet once liveness goes frame-relative. PROPOSED conservative
    line on file (`28R`): collide unless both positions agree on the backing family's
    closure and dialect. Lands first in `28T`'s sparing mini-model; ack owed.
13. `rul-blessing-flows-from-best-caller` × the `28M` §11 keep-verdict — the
    reachability-elevation tension (`28R:adj-blessing-vs-keep-verdict`: enrollment
    becomes refactor-sensitive call-graph topology; the reverse-flow cell sits inside
    one closure); one typed line owed (re-affirm knowingly · fold into the spike-end
    instrument · key elevation to marks-reached-from-predict). Bites at stage-ii
    vocab-minting, not before.

## §10 — The authored surface (RESERVED)

How users express these concepts in oracles — begin/end description members or marks,
whether lifetime-crossing rides the existing `undivided-by-transit-across` mark spelling
or that token itself renames with this vocabulary, the ssh entry-form's authored half
(read `plans/24T` first — the remote re-parse is payload decomposition wearing a
network hat, never a vouch rider), and the sent-language-is-becoming-crufty ceremony
budget for all of it — is the next design dig, human-led. Deliberately empty until
that sitting.
