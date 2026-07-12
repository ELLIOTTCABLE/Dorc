# 271 — Block-settle rulings ledger (living)

AI-scribed, human-adjudicated (design-rubber-duck sessions, begun 2026-07-10). This is
the **single evolving durable** for the `270:block-settle` design-pass: rulings,
direction, and arc-pointers, accreted as the human works through the session task list
below. Explicitly sanctioned as a living document by the human (2026-07-10); plan-tier
per `plans/23D` precedent — update in place, annotate supersessions, never silently
delete a ruling. Compressed 2026-07-12 at the human's direction (turn-by-turn noise
removed; full dialogue chronology lives in this file's git history; design bodies live
in the arc durables).

Authority: root docs and human-TYPED rulings outrank this; this ledger *records* typed
rulings, it does not create them — an entry here without "(typed)" is conductor-drafted
direction awaiting ack (silence ≠ ack). Naming discipline per `270` §1: hyphenated
full-word slugs; entries minted here are referenced elsewhere as `271:<slug>`.

Arc durables (the `272` precedent — extensive redesigns get their own document; this
ledger keeps rulings + pointers): **`notes/272`** (address-derived topology, task 3) ·
**`notes/273`** (the wrapper surface, task 5) · **`notes/274`** (the eval'er surface +
reentry token, task 6) · **`notes/275`** (value-predictions / the capture lane,
task 7). The **entity-algebra design note** (tasks 1–2's obligation, task 12) is its
own document when authored; its needed material is carried in full below.

## Session task map (mirrors the conductor task list)

1. adj-entity-algebra, spelling half — CLOSED 2026-07-10: all six sub-rulings typed
   or dictated ("The entity-algebra spelling direction" below); mechanical close-out
   rides task 12.
2. adj-entity-algebra, seams half — CLOSED 2026-07-10 (the seam rulings below).
3. adj-trichotomy-spelling (né `24S:A3`) — CLOSED 2026-07-11: dissolved into
   address-derived topology; durable = **`notes/272`** (statuses in its §12).
4. adj-axis-vocabulary (né `24S:A1`) — CLOSED (rul-axis-vocabulary-v1; amended by
   rul-networking-unpunt: netns ahead of full fs-view).
5. wrapper context-function spelling (né `24S` §2b) — CLOSED 2026-07-11: surface
   redesigned wholesale; durable = **`notes/273`** (statuses in its §12).
6. eval'er declaration spelling (né carrier) — CLOSED 2026-07-12; durable =
   **`notes/274`** (statuses in its §13). Ack chain: the blanket ack (typed
   2026-07-12: merge + synthesis + §2 ρ-split + strip wording) + the two close acks
   (the env riders of `274` §12 finding-env-ownership-confirmed bind as build
   obligations; both final-pass dispositions — scope-clarification and
   row-three-in-verdict-bodies/guard-building-declines — accepted). The `dorc:sh`
   synthesis's formal stamp discharged at close.
7. adj-capture-claim (né `219` fork-capture-claim-type) — CLOSED 2026-07-12;
   durable = **`notes/275`**. Rulings: rul-sin-ordering ·
   rul-measurement-is-authorship · rul-orthogonality-counterexample-test ·
   rul-composed-bytes-defer-and-floor · rul-value-prediction-species (all below).
8. adj-survival-flag-outcome (né `24S:A2` + the `--trust-footprints` rename) — the
   central trust ruling: flag gates the OUTCOME; outcome-centric name. Couplings
   accrued: re-read adjudicability against DERIVED-not-declared topology (`272`
   §11); the `273` §4 safety-inversion is its sharpening; the invited-rooms-values
   rule (`275` §4) — hint-lane values never feed survival.
9. adj-stopping-point (né `24T:P-A6`) — typed ratification of the derivable analyzer
   ceiling (const-resolvability × no-escalation × no-cross-host). `274` §12 noted
   the reentry strengthens the candidate.
10. adj-small-homes — CLOSED (rc-partition stands + inversion-pair; effect-check
    homes as a typeless-floor rider).
11. adj-payload-pins pre-read (né `24T:P-A1`/`P-A2`) — optional tail; formally owed
    at block-context implementation-planning.
12. 1-series close-out — amend the `24P` specimens (hash introducer;
    brace-alternation if firmed) + author the entity-algebra design note. UNBLOCKED.
    Imports: the spelling direction + seams below · `272` §1 formal spine, §4 carve,
    §5 fence, substrate-mark slot, keyed/partitioned vocabulary · the
    observe-backing-widening backing-SETS seam · the `dorc:` prefix-mark grammar
    position (`274` §10) · the uniqueness-bit rider (below) · `275`'s
    backing-inheritance candidate for formal ratification.
13. containers-lens re-skim (optional; rides the entity-algebra crosscheck).
14. triple-check the structural-vouch hard law in a fresh session; then adjudicate
    the deferred rul-only-oracle-bytes-ship (the `24J` raw-ship repair). Gates
    `273` §6 probe-form composition and `274` §5's transform lane.
15. semantics-proliferation stance (minted mid-task-6): whose interpretation of a
    tool's language/flags does an author answer for — eval'er heads the intense
    case (settled by ownership, `274`), GNU-vs-BSD grep the everyday one.

## Rulings

### rul-axis-vocabulary-v1  (task 4; 2026-07-10; typed)
`24S` §3a/§7-A1 ratified as posed: v1 coordinate axes {user, fs-view} + reserved-never
{host}; ρ components {env-vars per-variable, positionals, cwd}; vocabulary engine-owned,
dialect-versioned; users never mint axes; expressibility-clause decline for wrappers that
exceed it. Rider (typed): **fs-view is soft-deferred** — "a hard enough problem that it
could take its own entire round"; the spike does NOT attempt it; re-entry only if
specifically needed to exercise r27 work; spike-minimum = a Very Simple Fs Stub
({user, fs-straw}). The `24S` §3b honest ladder stays reserved-not-built. Consumers:
the entity-algebra design note (context-qualifier seam), wrapper briefs, stdlib kind
declarations.

### rul-rc-partition-stands  (task 10a; 2026-07-10; TYPED — "your analysis of the rc
seems correct. I buy your arguments"; riders r1–r3 bundle-acked)
TODO.md:23 answered: **keep `rul-rc-partition` unchanged** (0 = named sense holds /
1 = complement / ≥2 = confused-runs).

**rul-zero-one-inversion-pair** (the human's sharpening, typed same date): the
verdict-bearing statuses are exactly {0, 1}, an inversion-pair — the ONLY statuses
that can ever carry a verdict, hence ever produce a skip, in any current or future
decision table. ≥2 is meaningless/error/warn, is NEVER inverted, can never license.
systemd's `ExecCondition` ternary (`24R:repurp-finding89`) is convergent-evolution
VALIDATION, not adopted — its layout puts machinery-mintable statuses (126/127,
128+n, 124/125) inside the verdict-bearing region, exactly what our flat ≥2 sink
prevents. Riders: (r1) the ≥2 region stays semantically FLAT forever;
decline-vs-crash distinctions live in report/diagnostic lanes; (r2) exterior
coherence is a delegation-body concern — ordinary `case $? in` remap arms; stdlib
quality-bar: delegating to a tool with a documented non-test-shaped exit vocabulary
requires an explicit remap arm; (r3) Dorc-as-tool outward contracts (`dorc plan
--exit-code`, dorc-run) are separate per-surface decisions, unbound by this
partition.

### rul-effect-check-home-typeless-floor  (task 10b; 2026-07-10; TYPED)
The owed static effect-check of verdict-function bodies homes as a **typeless-floor
brief rider** (`270:block-rebuild`), not wrapper-peel: same seam (verdict-bearing
providers threaded into classify as data), lands before block-context widens the
shipped-probe surface, keeps the referendum-carrying wrapper-peel brief lean.
**Sizing rider (typed): low stakes — build only if nearly-free at that seam;
punt-empowered, zero guilt.** Trust framing: the static check catches the
shell-visible accident class ONLY; the tool-invocation interior stays vouch-tier
forever; the `plans/077` runtime backstop stays reserved. Riders: `$DORC_REPORT`-class
OOB write lanes carve out as non-mutative-for-this-purpose; tmpfiles NOT blessed in
v1; the rider brief verifies against `24C`'s actual classify/effect shape.

### rul-no-mutating-guards  (task 10b; 2026-07-10; TYPED, emphatic)
One bar, permanently: a verdict-function body that fails the non-mutation proof lifts
NOWHERE — not as a probe and **not as an apply-time guard**. Verbatim: "We *do not*
insert *mutation* that we just proved to be mutation, out-of-order, in
not-user-spelling, into apply bodies. Extremely nope." Kills the two-bar option
forever.

### rul-proven-mutation-fails-fast  (task 10b; 2026-07-10; TYPED)
**Proven-mutates ⇒ genuine fail-fast** (plan-time, pre-network, loud; dictate-tier),
whatever the provability source — structural (permanent, corpus-independent) or
oracle-sourced (corpus-relative). No third hard-error category: a read-only-named
function whose body provably mutates is an instance of the standing
"declarations that genuinely contradict" category. The corpus-flip edge (a
previously-green book hard-errors when a newly-loaded oracle proves a latent breach)
is ACCEPTED — the flip discloses a standing violation, not a regression. The two
provenance classes stay SEPARATED in diagnostics (structural = permanent,
near-warrantable; oracle-sourced names the proving oracle, never marketed as
coverage). The unprovable residual: superseded by rul-unprovable-rides-the-vouch.

### rul-unprovable-rides-the-vouch  (task 10b; 2026-07-10; TYPED — "Okay, acked.
Durable")
The **unprovable** region of a verdict-function body ships on the authored vouch,
exactly as today — probe AND guard, no lift-refusal. The oracle lane's probe-license
was always the VOUCH, not proof (DESIGN's two license sources; the under-approximate
mandate governs the Dorc-provenance path — lifting admins' in-book guards — which
stays proof-gated). The effect-check is **falsification-first, never a completeness
gate**. Riders: (r1) each unprovable command earns a hint-tier note (author-facing,
kWARN-rich) with the standard enhancement-pointer shape; (r2) verified-vs-
vouch-carried is a REPORTABLE per-body distinction — never warranty language;
(r3) the filter-tool cohort (jq/awk/sed/cut) is the expected common case — the
stdlib's read-blessing species covers it opportunistically, never as a precondition;
(r4) **vouch-scope-is-the-body-never-the-tool** (typed): the body-vouch is a claim
about a REGION OF SH, never about the command families it invokes — it mints NO
command-family fact; an unmodeled command carried in one oracle's body stays ⊤
everywhere else. Exactly one vouch reaches out of its file: a tool's own oracle
reaching that tool's book-sites, argv-keyed. Two claim-species, two scopes; never
convertible.

### The entity-algebra spelling direction  (task 1; 2026-07-10; consolidated — each
sub-entry marked typed or awaiting-ack)

The coordinate, everywhere it appears (facts, backings, footprints, disjointness), is
the **flat three-place `(kind, entity, selector)`**.

- **rul-coordinate-shape-flat-three-place (typed).** Flat ratified; `plans/17N` §4's
  recursive-struct lean declined as coordinate shape — its motivators live in the
  owner-declared function mechanisms *between* flat coordinates (`resolve()`,
  `reaches()`/`manifest()`), so the coordinate stays a name. Riders: later
  structural expansion must remain language-design-easy; the bare selector-less form
  permanently means "true / occupied / whole-entity". Engine hedge: `SelectorId`
  stays opaque/interned; every selector comparison lives behind ONE choke-point
  function (`selector_covers`-shaped).
- **rul-selector-introducer-hash (typed, PERMANENT).** `#` introduces a selector:
  `sm.dorc.Service:"$svc"#enabled`. Quoting supported where charsets collide. Rides
  the corpus-respell churn; `notes/24P` specimens amended conductor-side at 1-series
  close; USER_STORY's `.synced`/`.matched` staleness accrues to the punted root-doc
  pass.
- **rul-emission-selector-on-mark (typed).** `touches()`/`reaches()`-family members
  claim a cell via the selector riding the trailing mark on the emitting statement —
  `printf '%s\n' "$1"  : sm.dorc.Service#active` — applying to every line that
  statement emits; emission lines stay raw entities; a selector-less mark = whole
  entity. Multi-cell claims: brace alternation `: sm.dorc.Service#{enabled,active}`
  (direction; exact grammar in the design note).
- **rul-binds-entity-only-provisional (typed, SOFT — ~GUESS both sides).** Binds
  name entities, never cells; facts about cells attach via marks on
  probing/emitting commands. Door open on counterexample.
- **rul-kind-or-selector-is-a-behaviour-choice (typed).** Kind vs selector minting
  is a user-facing behaviour selection, not ontology. The menu: **kind = isolation**
  (cross-kind disjoint by construction, `reaches()`-bridged only; claim-only usable)
  vs **selector = coupling** (same-entity collide-by-default; subscription to the
  entity's kill-traffic both directions). Rides the stdlib quality-bar.
- **rul-selector-disjointness-dialect-scoped (TYPED 2026-07-10 — dictated "as ruled
  for now, and see where the spike goes with it"; spike-provisional).** When
  token-inequality may be read as cell-disjointness (the survival license):
  - **Minting.** A selector token enters only as an annotation on a runnable
    measurement line — verdict-probe marks and `:?` observe marks both mint;
    claims/touches never mint. No dislocated declarations: a kind's cell-structure
    is the projection of what loaded oracles measure.
  - **`dialect(source, kind)`** = the selector tokens that source's
    measurement-marks carry for that kind. No global per-kind vocabulary.
  - **Comparison.** Same-entity: a claim SPARES a backing iff claim-token ∈
    dialect(the backing's minting source, kind) AND claim-token ≠ backing-token.
    Everything else COLLIDES — selector-less claims, unminted tokens, cross-dialect
    tokens are all ⊤-selector. Cross-entity/kind disjointness unchanged.
  - **Properties** (each load-bearing): empty world ⇒ byte-identical to
    entity-granular HEAD · noise fails safe on BOTH sides · monotone (a newly
    loaded source never alters comparisons against other sources' backings) · no
    self-licensing · **subscription semantics native** (a cell hung on a shared
    kind is auto-subscribed to the entity's kill-traffic; a source's dialect IS its
    kill-surface control — dissolves `233` hard-1 and `24F` §10 alternative-6) ·
    attributable (every dialect member cites its minting line) · DST-clean.
  - **Worked minimum:**

    ```sh
    # dorc-lang/v0.1
    systemctl__is_converged() {
       case "${1-}" in
       enable) systemctl is-enabled --quiet -- "${2-}"   : sm.dorc.Service:"$2"#enabled ;;
       start)  systemctl is-active  --quiet -- "${2-}"   : sm.dorc.Service:"$2"#active  ;;
       *) return 2 ;;
       esac
    }
    systemctl__touches() {
       verb="${1-}"; shift
       case "$verb" in restart) printf '%s\n' "$1"   : sm.dorc.Service#active ;; esac
    }
    scan_cve__predict() { cve-tool --check -- "${1-}"   :? sm.dorc.Package:"$1"#cve_clean ;}
    ```

  - **Fences / residues:** ghost-ack-mark stays UNBUILT (absence fails safe;
    reconsider on field evidence) · same-token-divergent-meaning is
    adjudicability-list tier · physically-overlapping cells within one dialect are
    inherent to narrowing, differential-testable · engine cost: per-(kind × source)
    dialect sets + backing provenance in the comparison (-GUESS cheap) · this
    precedent does NOT auto-generalize to context topology (adjudicated separately;
    it went derived instead — `272`).
  - **Amendment (typed 2026-07-10, task 2):** the ruling's "source" is **the
    family** (rul-family) — name-derived, NEVER file-derived.
    `dialect(family, kind)`. "Single-source responsibility" reads "single-TARGET
    description"; attribution stays line-level.

### The entity-algebra seams  (task 2; 2026-07-10; all typed)

- **rul-family (minted; the human's definition, sharpened).** A *family* is the set
  of non-overlapping/non-contradicting, not-guaranteed-same-author,
  within-compilation-unit `__role` functions describing one **description-target** —
  two species: a COMMAND (all `systemctl__*`) or a KIND (all `sm_dorc_Package__*`).
  Membership is by name-construction only — never file, never author. The
  per-species role vocabulary is engine-owned, closed-at-a-version, and extends BY
  NEW NAME ONLY (`24C:rul-ditch-is-diverged`'s extension law). Each member optional;
  silence = floor. Families are coherence units, not authorship units.
- **rul-seam-context-slot-and-relational-chokepoint.** The coordinate representation
  grows a context slot (default ambient) — at representation tier a space-tag and a
  qualifier field are the SAME datum, so that fork dissolves. The genuine fork
  (function-shaped vs relation-shaped cross-context identification) defers INTO the
  comparison: ALL coordinate comparison sits behind one whole-coordinate choke-point
  that MAY answer relationally — per-axis pointwise decomposition is never baked
  into the API. On record: spaces-with-bridges does not fully handle fs-view either;
  DETECT-and-DEGRADE stands regardless.
- **rul-seam-kind-owner-registry-room.** One engine-internal extensible per-target
  registry: at-most-one-per-(family × role) generalized to clause families not yet
  minted, loud on conflict. ZERO file semantics; the homogeneous-compilation-unit
  posture affirmed untouched; the only per-ingest-unit fact remains the dialect
  marker.

## Rulings (task-3/task-5 sittings)

### rul-touches-becomes-disturbs  (2026-07-11; TYPED — "'touches -> disturbs' is a
winner")
`touches()` → **`disturbs()`** (liability register: recruits paranoid completeness).
Rides the corpus-respell churn window. The reaches/store deferrals resolved by
rul-at-most-family-names below.

### rul-class-prefixed-role-names  (2026-07-11; TYPED)
In all generic discussion and documentation, role-functions are written WITH their
keying class: `cmd__disturbs()`, `kind__reaches()` — never bare. Binds this ledger,
the design note, briefs, and doc prose forward.

### rul-networking-unpunt  (2026-07-11; TYPED — amends rul-axis-vocabulary-v1)
Containers make network-naming core 2026 single-host ops: **netns re-enters the axis
roadmap explicitly, sequenced AHEAD of full fs-view.** Single-host-single-universe
still governs spike3. Riders (conductor, un-nacked): the netns axis is
naming/scoping ONLY (no network access; `24S:imp-5` untouched); mechanically cheap
(`ip netns exec NAME` yields argv-named values like `sudo -u USER`; no aliasing
ladder); serves ip/nft/sysctl; docker-MANAGED networking stays endpoint-opaque.

### rul-lint-never-drives-design  (2026-07-11; TYPED, standing — human-voiced)
Lints are never a rescue from a bad design and are never discussed during design:
either re-design, or absorb the limitation as a core, frontloaded, README-class
constraint. Lints are built at the END of design, from that frontloaded list.
(The conductor's prior lint-flavored design arguments that sitting were STRUCK;
candidate D re-audited and stood without them.)

### rul-net-quality-u-curve  (2026-07-11; TYPED, standing — human-voiced)
Catch-net quality is U-shaped: perfect mechanical nets (bad-states-unrepresentable)
and honest documentation are both good; the dip between — linting /
provably-imperfect best-effort nets — is a design footgun. Documentation as a
goalpost is SUPERIOR to imperfect mechanical nets.

### rul-at-most-family-names  (2026-07-11; TYPED — "I like the naming. Let's bake
that moving forward")
Ratified: **`cmd__disturbs()`** (no `only` — at-most per MATCHED INVOCATION-SHAPE,
gradual across shapes) · **`kind__disturbance_reaches_only()`** ·
**`kind__state_stored_only_in()`** (earns `only` most — its key consumer reads the
emission's negative space). The general rule: **`only` in a role name =
complete-by-contract, totalistic-survey-before-authoring; absence =
arm-incremental.** The chain teaching-sentence: a command DISTURBS cells; that
DISTURBANCE REACHES ONLY what the kind-owner enumerated; a kind's STATE is STORED
ONLY IN the substrates its owner declared. Rides the corpus-respell brief.
Unruled remainder: `predict`/`resolve` menus (delivered in-chat 2026-07-11, low
urgency); wrapper-member names were task 5's (since ruled).

### rul-simulate-env-user-authored  (task 5; 2026-07-11; TYPED direction)
The ρ-transform surface is a USER-AUTHORED executable env-closure — never
engine-generated ("I extremely-strongly-prefer 'user writes' to 'engine
generates'"). The member-name question was resolved by
rul-predict-absorbs-wrapper-modeling: the name `simulate_env` DIED; the authored
body and its contract survive whole inside `cmd__predict()`.

### rul-predict-absorbs-wrapper-modeling  (task 5; 2026-07-11; TYPED — "Merge
acked. merge-riders: acked")
ONE modeling member: **`cmd__predict()`** — "the best read-only sh model of your
command." Wrapper-ness is DETECTED, never declared: a body whose command-position
`"$@"` runs its argument-slot is a peeling wrapper by tautology; the argparse path
to that `"$@"` is the peel, env-idioms along it are ρ-claims, printf lines are
output-claims, marked lines are facts. The axes/emission member survives separately.
Merge-riders ACKED as build obligations: per-channel claim/decline vocabulary
(delegation = faithful claim · printf = asserted output · explicit return = rc claim
· redirect-to-null = per-channel decline ⇒ ⊤ · `return 2` = whole-shape decline) ·
line-level attribution (the `24S` §4a chain's first link cites the LINE) ·
opt-downs-additive. Full design: `notes/273` §2.

### rul-only-oracle-bytes-ship — ratification DEFERRED  (2026-07-11; TYPED:
"continuing to defer on the repair; I'm strongly suspicious it's debt, but I want
to triple-check that hard law in a new session")
The repair stays DRAFTED (task #14 owns the fresh-session re-derivation of the
structural-vouch hard law first). The `24J`/`24C`/LIVING_STATUS debt-confirmations
stand. Full draft: `notes/273` §6.

### rul-lend-map  (task 5; 2026-07-11; TYPED — task-5 closing ruling)
The wrapper's dimension member is **`cmd__lend_map()`**: a function from the site's
argv to fixed strings, one entry per dimension — empty result for a PRESENT key =
full lend (the colon-line, `:   : user`); contents = mapped lend; a MISSING key = ⊤,
walls, hint-tier nudge (the **enumerate-every-dimension law**;
absent-key-means-full-lend explicitly REJECTED); terminal `"$@"` = the peel
boundary; no `only` (arm-incremental). Rider (the human's safety-inversion
correction, conceded): ALL entry-types are dangerous-when-wrong;
believed-no-overlap is safe for the transport consumer and dangerous for the
kill-traffic consumer, and vice versa — the inversion the ternary relation and
never-derive-separation encode. Full design: `notes/273` §3–§4.

## Rulings (task-6 sitting — bodies in `notes/274`; compressed here 2026-07-12)

### rul-evaler-vocabulary  (2026-07-11; TYPED)
The code-evaluating-head family is **eval'er** (né carrier; slug `evaler`).
"Carrier" retires from prose; né-subscript progressively.

### rul-evaler-floor-fixed-set  (2026-07-11; TYPED direction; refined by the
synthesis)
An engine-owned, dialect-versioned grounding floor for the reentry primitive is
acceptable (symbol-grounding hackles LOW — the set falls out of the product's own
nature, we are the *dash* orchestrator). Refined by the synthesis to its logical
floor: ONE token we own. The human's own swap-in-the-host-shell strawman was killed
by its author (probe-apply divergence); any future scheme must pass probe-apply
fidelity first.

### rul-evaler-merge-no-structure-member  (2026-07-11; TYPED 2026-07-12 via the
blanket ack, as refined by the self-attack round)
No separate structure-member: eval'er-ness is DETECTED inside `cmd__predict()`
(delegation into the blessed reentry primitive). The decider (human): a split is
justified only by genuine argparse control-flow divergence; the cohort audit found
every divergence branch-terminal and ρ-confined. The self-attack round's law:
**split members by ANSWER-TYPE, never by claim-strength or purpose** (dissolves the
dual-peel tension retroactively). Bounded downside: a genuinely-forked future tool
costs body-ergonomics only. Full record: `274` §0/§7 + git history.

### rul-env-claim-inversion  (2026-07-11; TYPED 2026-07-12 — blanket ack + the
env-riders close-ack; `274` §12 riders r1–r6 bind as build obligations)
A predict body's ρ-claim grammar, every rung a runnable sh idiom: bare `"$@"` =
claims NOTHING (⊤; never "claims-isolation" — derived separation barred) ·
`VAR=x "$@"` = per-variable claim, rest ⊤ · `env "$@"` = full ambient passthrough
(the positive spelling; the `env` syllable IS the claim — cleanup-vandalism narrows,
fails safe) · `env -i VAR=x … "$@"` = exactly-these. The ρ-side twin of
lend_map's enumerate-every-dimension law: silence = floor everywhere; ignorance
mints ⊤, every believable claim is a typed pointable line (the razor argument).
Identity wrappers owe one syllable (`env "$@"`) or lose env value-flow — `273` §8's
bare line gains the annotation. Full grammar + re-scored translation-gap: `274` §2 +
git history.

### rul-evaler-delegation-actual-command  (2026-07-12; the eval-exclusion half
TYPED — "You've killed eval, fairly")
Authored delegation in an eval'er predict body is an ACTUAL COMMAND, never `eval`
(context record wrong on every axis: fresh options, export-only env, positional
binding; token-collision with future transparent-context eval modeling; off-ramp
leakage). `eval` may reappear only as ENGINE-LOWERING vocabulary, never authored
spelling. The default-head half was superseded by the synthesis below.

### rul-dorc-prefix-head-synthesis  (2026-07-12; TYPED — human-strawmanned; blanket
ack + four typed shaping dispositions; formal stamp at task-6 close)
Three spellings, one design (full table + grades: `274` §1/§8): **bare `sh`** =
escape — analysis DESCENDS for hints only, licenses NOTHING (descend-don't-license;
enforcement tier = TYPESYSTEM per rider-invited-rooms-typing: invited rooms may mint
licenses, hint-only rooms may not — typed direction). **`dorc:sh`** = the mark,
prefix-position on the real head: full analysis license; probe-ship rewrites to
`dorc-sh`, resolved by a host-constructed per-run shim on PATH; strip =
prefix-erasure to bare `sh` (grammar-valid, world-invalid: fails loud-127 under
stock shells). **`dorc-sh` typed directly** = the runtime object: pinned execution
environment, NO analysis license, composes transitively via PATH;
**rul-row-three-documented-dangle (TYPED)**: strip does NOT touch it
(half-strip is worse than no-strip). **rul-no-nested-annotation (TYPED)**: no
nested `dorc:sh` — annotation-syntax in opaque blobs is a plan-time
parse-failure-tier error. Consequence: the rul24-totalistic-munge carve shrinks to
prefix-erasure + the shebang-runner rewrite; no in-body name-rewriting anywhere.
The shim's DST story is walked (`274` §5: host-independent shipped text;
session-establishment materialization; run-id naming; smoke-test → one
shimless-degrade decision; failure lattice drains to the ≥2 sink).
thread-command-word-dorcisms (banked light): the human leans toward REVISITING old
syntax toward command-word spellings (127-loud beats tail-position corruption);
parks at the entity-algebra grammar sitting.

### direction-evaler-probe-shipping-split  (2026-07-12; direction-tier;
task-14-coupled)
Scoped to the probe lane (`24S:imp-7` settles apply forever). The exec boundary is
where substitution dies ⇒ bare-ship and model-participation are mutually exclusive
per-site. Bare lane = proof-gated (whole-read-only payloads ship as the REAL line;
one fewer divergence class — evaluator-identity — never an unperturbed measurement,
per the human's nits). Transform lane = task-14-gated; HOIST stands DOUBTED (the
human's block-integrity counter-shape banked: prefer moving blocks whole under
their executor; scatter-hoist last, per-fact, license-gated — flagged to the
block-context deep round). Full menu + corrections: `274` §5 + git history.

### direction-observable-transforms-gradient  (2026-07-12; human-voiced; DRAFTED
candidates)
Probe design was always transforms-all-the-way-down, ignorable while the set was
small and unobservable; observable transforms add (1) teachability and (2) a trust
gradient to licensure — transform choices can manufacture author-failures wearing
the author's name. Candidates: **candidate-probe-body-contract** (the author-facing
execution contract as transform-admission criterion:
observationally-equivalent-within-the-contract, verified by transform-equivalence
differentials) and **candidate-law-fair-attribution** — **HUMAN-TYPED endorsement
("a critical catch"): registration CLAIMED BY THE HUMAN for the root docs**; errors
unfairly attributed are worse than unattributed; "DX is the CORRECTNESS product";
does not trump lint-never-drives-design. Menu re-rank + v1 upshot (payload-v1
introduces ZERO new observable transforms): `274` §6 + git history.

## Rulings (task-7 sitting — bodies in `notes/275`)

### rul-sin-ordering  (2026-07-12; TYPED — the razor's severity scale, sharpening
candidate-law-fair-attribution into an ordered ladder)
Worst first: (1) **pope-sin** — MIS-attributed error (Dorc names a human and a
fix-here; they comply, and Dorc was wrong). (2) **cardinal-sin** — UNattributed
error (Dorc-did-it, flat — including chosen designs that convert a Truth About The
World into some human's problem that didn't exist pre-Dorc; traveled-wrong-elision
where everybody chose correctly locally; 233-class). (3) **mild-sin** — attributed
error we could have made easier and failed to (insufficient warnings; value left on
the table). (4) **not-sin** — our genuine best was delivered and a HUMAN failed
against it. Consumers: every fold/elision failure-mode analysis slots its modes into
this scale; fair-attribution's could-a-competent-contract-reader-have-avoided-it
test governs the pope-sin/not-sin boundary.

### rul-measurement-is-authorship  (2026-07-12; TYPED — doctrine; CLAUDE.md
sibling-registration next to the 'skip' warning CLAIMED BY THE HUMAN, queued)
The engine's only knowledge of the world is (1) the CFG and (2) API-endpoint
observables of AUTHORED oracle-functions. "Measurement" as a category distinct from
authorship is borderline misleading: probes structurally cannot measure anything
except the output of authored bodies — measurement IS authorship ("we teach authors
how to measure, and then vouchsafe those measurements to us"; we never know
tool-rcs, we know PREDICT-rcs). Companion typed context: `predict` was NAMED because
stdout-oracles were visibly coming; ALL observables are prediction-in-fact, rc
included, just less-obviously-hard. Consequence: the conductor's three-provenance
system-taxonomy is RETRACTED; the registry is the claim-chain (which authored
surface, which line, which idiom).

### rul-orthogonality-counterexample-test  (2026-07-12; TYPED — method doctrine,
general beyond this sitting)
Lines that mean multiple UNRELATED (or mutually-opposing) things are design
failures; multiple RELATED meanings are not. The test: **write an oracle that will
want to push two of the meanings in two different directions** — if genuinely
orthogonal, ops will exercise it (somebody needs "A && !B" on one line). No
inhabitable counter-example ⇒ the meanings are linked and the objection COLLAPSES
until the example turns up. First run (against the `:?` mark's jobs): the
decorated-output oracle splits fact-backing from value-backing — resolved by
per-CHANNEL backing DERIVATION through recipe dataflow, one mark-meaning, derived
consumers (`275` §2); the dialect-minting/disclosure fusion was not inhabitable ⇒
collapsed.

### rul-composed-bytes-defer-and-floor  (2026-07-12; TYPED — "Ack on all")
Four parts: **defer-the-gate indefinitely** (whether validity ever gates on the
delegation-vs-composed line-idiom: no gate machinery, no warning-only-forever weld;
field evidence decides; the data is tracked regardless) · **warnings struck from
the fork** (maximal line-idiom data into warnings/why, forever ungated) · **the
reversible interim floor** (inv-probe-sourced-values' "probe-provenance" and `273`
§6's "real bytes" both PINNED to the world-spoken reading — delegation-produced,
not merely probe-executed — for knife-tier consumers; loosening later is monotone
value-add, tightening later is verdict-churn; scaffolding, NOT a verdict) ·
**doctrine: oracles-may-lie / judgments-not-facts** (the system has coherence,
calibration, attribution — never a truth/lie axis; an idiom-gate couldn't remove
the deliberate-helpful-lie capability, only uglify it; the vouch-tier's one-line
definition: users trusting each other to know what each other WANT; an oracle's
product is a JUDGMENT — the deliberate lie is the stdlib's founding transaction).
Consumers: capture-wire briefs (the floor) · the render-unpark fence · task-14's
re-read of `273` §6 wording · stdlib quality-bar (teaching, never gate).

### rul-value-prediction-species  (task-7 close; 2026-07-12; TYPED — "full ack,
durable as appropriate", with the human's naming and the clock carve-down)
The capture lane's headline ruling; full design = **`notes/275`**. The species is
the **value-prediction** (name TYPED; "claim" = derived-license tier, "prediction" =
engine-internal shape, less-human-interpreted — and literally what it is: a
forward-looking assertion about an apply-time evaluation): every byte-shaped belief
beyond program text — captured stdout, stored rcs, composed output-predictions,
register-resolved who-am-I values. Representationally the reserved seams producing
(OutClaim seat; cause-tagged ValueOf; inv-one-observable intact); behaviourally a
claim-type (typed fields; validity branches on them). Fields, both DERIVED never
declared: provenance (taint-style weakest-fragment: register / world-spoken /
author-composed / ⊤) and backing (coordinate SETS, per-channel through recipe
dataflow). Three regimes by backing-class: register (analytic) / world-cell
(patrolled) / never-settled (HARD-DEFERRED per the human — shelf entry only, no
tokens until a real book's `date` walls and hurts). The validity table (`275` §4)
enumerates which rules branch on which field; the authored surface is THE EMPTY SET
(the kBURDEN headline). Chronology: freeze-at-binding; patrolled window =
apply-script-start → apply-time binding line. Cross-context transport derived via
backing invariance (discharges `272` §11's expected axis-independence vocabulary
member). Backing-inheritance + three-regimes + transport-chain: direction-tier ack
(hedged), formal ratification rides the entity-algebra note + its adversarial
crosscheck. Routed consumers: `275` §11.

## Direction & open threads

- **rider-value-recipe-reshape-capture-seams (hand-carried 2026-07-12 from a
  sibling-conductor's standing-work review; obligation on the block-rebuild
  value-recipe-reshape brief):** the reshape brief must NAME AND RESERVE (not
  build): (seam-pipeline-order) the **post-probe value re-bind** — the value plane
  runs strictly BEFORE the probe; folding a captured literal back requires a second
  value-flow pass or a fold-time substitution channel (`219` q-3.e/q-4.b: the
  deepest architectural cost in the capture design, easy to bake wrong);
  (seam-literal-provenance) **provenance of probe-sourced literals** — a
  value-plane-visible distinction between source-literal and probe-captured bytes
  (`219` q-4.c); leave the provenance slot open even if v1 routes provenance
  through the site-keyed record lane; (seam-per-channel-backing, added at task-7
  close) per-channel backing derivation requires the fragment-preserving recipes
  (`275` §2). Substance deliberately inlined — the originating list is
  auto-rewritten and off every reading path.
- **rider-entity-algebra-uniqueness-seam (same handoff; obligation on the task-12
  design note):** reserve the **uniqueness-bit** seam in the coordinate/comparison
  representation — no strong update exists (`Kill` accumulates); the standing 231
  fence rules "probably unique" may only DEMOTE, never license — representation
  room only, build nothing.
- **task-3 arc — CLOSED 2026-07-11; durable = `notes/272`.** One line: the
  trichotomy dissolved into **address-derived topology** — the ratified
  `kind__state_stored_only_in()` member (locator + substrate-mark emissions; the
  engine-owned carried-by table + emission-set non-interference; the
  never-derive-separation carve, docker-validated; the addresses-are-not-coordinates
  fence, FOURTH pointer at the parked co-reference mechanism; the differential
  harness as the derivation's other half). Statuses: `272` §12 (only the NAMES are
  typed; carve, fence, substrate marks, relation-spine each still want explicit
  acks — ride the entity-algebra note).
- **task-5 arc — CLOSED 2026-07-11; durable = `notes/273`.** One line: `24S` §2b's
  four-job context-function dissolved — predict absorbed wrapper modeling; the
  dimension member became `cmd__lend_map()`; the engine-built whole-ρ closure died
  into the authored predict body (probe-form composition DRAFTED, task-14-gated);
  the stage-D disjointness dividend is DEAD under never-derive-separation;
  dual-peel disagreement = static incoherence ⇒ fail-fast. Statuses: `273` §12.
- **task-6 arc — CLOSED 2026-07-12; durable = `notes/274`.** One line: pin1's
  declaration dissolved into detection; the head question dissolved into the
  `dorc:sh` prefix-mark synthesis (three spellings; descend-don't-license; per-run
  shim; strip = prefix-erasure). Statuses: `274` §13.
- **task-7 arc — CLOSED 2026-07-12; durable = `notes/275`.** One line: `219`'s
  fork dissolved into the value-prediction species (rul-value-prediction-species
  above); the sitting's threads (sitting-steers, backing-three-regimes,
  delegation-vs-composed-bytes) are absorbed into `275` §§2–8; chronology in git
  history. Standing residue: the composed gate (deferred by ruling) · the
  never-settled shelf · the minting-tax + too-pretty crosscheck earmarks
  (`275` §10).
- **thread-delegation-head — RESOLVED 2026-07-12** by
  rul-dorc-prefix-head-synthesis. The quantifier-audit record (∀→∃ eliminates at
  deployment; visibility ≠ quantification; epistemic labeling is the surviving
  core) lives in `274` §6 + git history.
- **thread-env-cannot-exec-functions (pre-existing landmine; routes to task 14 +
  block-context implementation-planning):** `env` execs binaries, not shell
  functions, so env-headed closure bodies cannot literally exec a
  function-substituted guest under probe-form composition. Candidates on record:
  PATH-materialization of predicts as command-files (human, "kinda messy maybe";
  scaffolding-not-semantics needle-thread) · subshell-eval lowering
  (engine-lowering only; gradient-demoted to last). The per-run shim dissolves it
  for the reentry token specifically.
- **thread-semantics-proliferation (minted as task 15):** the admin selected Dorc
  AND a shell; does the oracle-author get to select DIFFERENTLY? Eval'er heads =
  the intense case (settled by ownership); GNU-vs-BSD grep = the everyday one;
  wants a general stance.
- **watch-machinery-raises-disclaim-burden (human observation, 2026-07-10 — "not a
  nack"):** every layer of machinery supporting the user raises the
  documentation/education bar for disclaiming that the machinery isn't *enough*.
  Standing education posture: warranty only the structural class; the authored
  vouch remains THE contract. Watch wherever new contract-supporting machinery
  lands.
- **observe-backing-widening (drafted; entity-algebra-rebuild seam):** an Observe
  (`:?`) inside a verdict-function body widens that fact's backing to include the
  observed coordinate — needs backing-SETS (a reserved rebuild seam, now also
  consumed by `275`'s per-channel backing sets). Safe direction (only widens
  kill-surface); makes acknowledge-by-observing coherent.
- **ack-stays-dead (conductor opinion on record):** the ACK `~` and POISON
  bare-marks stay dead through the respell as staged (`24P` §2) — zombie no-ops at
  tip. Any future revival is a rider on a measurement line, never a bare
  dislocated mark, and only on field evidence.
- **two-observation-planes (vocabulary, human-checked 2026-07-10):** world-plane
  (coordinate statements; must be authored) vs value-plane (bytes through the
  program's own dataflow; structurally visible). Served its purpose as the task-7
  on-ramp; superseded in detail by `275` §§1–2; kept for citation.
- Conversational-strawman discipline (conductor note-to-self): bare munged
  `tool__role` names per `24M`; the `# dorc-lang/v0.1` marker line is exact-match
  and stands ALONE.
- **Owed at 1-series close (task 12):** the `notes/24P` specimen amendment pass
  (hash introducer at minimum; brace-alternation if firmed); then the
  entity-algebra design note itself (imports listed at task-map item 12).
