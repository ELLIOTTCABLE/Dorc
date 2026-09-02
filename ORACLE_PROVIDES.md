ORACLE_PROVIDES: the map of what an oracle hands Dorc
======================================================

> AI-authored, AI-maintained, UNAUDITED (no human has reviewed it line by line). Current as of the
> r30 designs (2026-09-02). A MAP, not the contract: the contract an author reads is
> `spike/docs/reference/oracle-contract.md`; the law an engine builder obeys is `spike/CLAUDE.md` (its
> "The authored surface" and "Language & off-ramp law" sections); the design-of-record for each shape
> is the document its entry points at. Use it like `Research/README.md`: grep the slug, follow the
> pointer, never take a paraphrase here over the document it names.
>
> Update when a shape is minted, retired, or re-cut; STATUS words are re-graded when a design lands or
> a lane folds. STATUS: `built` (crate steering says so) · `ruled, unbuilt` (a typed or design-of-record
> ruling exists, no lane has landed it) · `proposal` (a design exists; rulings owed) · `open` (no
> design-of-record). Steering wins on "built", the design wins on "intended"; entries say both.
> Every entry carries a gradual-degradation sketch (the founding obligation: each partial-effort
> increase buys value, none removes previously-won value; "not yet sketched" is honest). Trust-shapes
> use `271:rul-sin-ordering`: pope-sin (mis-attributed) > cardinal-sin (unattributed under-execution)
> > mild-sin (attributed, could have helped more) > not-sin. All sh is illustration, never spelling.

The frame: a book line Dorc cannot read (`foobar sync-certs "$CERTS"`) plus an author who knows the
tool. Everything authored is sh: role-function bodies whose NAMES wire them up; marks and binds
(marker-gated, strip-only); `printf` records into the report lane; load-time idioms (`.`, include
guards); the `dorc:` head token. Everything else — value predictions, wrapper-ness, keying, backing
sets, families — is DERIVED and never declared. Entries run in dependency order.

### provides-decoding — "here is how to read my command's invocations"
- WHAT: the argparse — ordinary control-flow (`verb="$1"; shift; case "$verb" in …`) a site's resolved
  argv flows THROUGH; it defines reached-path scoping for every other shape and type-checks the vouch
  (declines included). The engine parses no tool argv, ever.
- DESIGN: `spike/CLAUDE.md` identity-declared-never-inferred · rul-argv-flows-bytes-do-not ·
  `271:rul-only-oracle-bytes-ship`; quality bar `spike/crates/oracle/CLAUDE.md` (R2-MULTIOP). STATUS: built.
- TRUST: near self-defeating when wrong (resolves nothing ⇒ ⊤ ⇒ run: not-sin); the live bite is the
  half-checked multi-operand yes (cardinal-sin at the author's own site).
- DEGRADES: no decoder ⇒ wall + run; each decoded arm buys exactly its paths. Monotone.

### provides-vocabulary — "these are the named pieces of world-state my tool deals in"
- WHAT: kinds (reverse-DNS, ≥2 dots; no registry), entities, `@selector` cells — coordinates
  `KIND:ENTITY@SELECTOR` in marks; binds `dest : org.foob.Certs = "$1"` (or trailing
  `dest="$1" #:= org.foob.Certs`) name entities, never cells. A family's selector DIALECT (for survival
  sparing) is derived from its marks and gated: selectors join selector-distinct survival only once one
  genuine `family__predict` exists (`30J` P2); before that they still key facts and widen backings.
- DESIGN: `277` §1 · `plans/281` §6/§8 · `271:rul-coordinate-shape-flat-three-place` ·
  `271:rul-binds-entity-only-provisional` (SOFT) · `24M` · `plans/30J` (§12: the dialect a comparison
  reads is the closure live at the backing's frame) · `plans/30W` §1 (index-kinds; ruling owed).
- STATUS: built (coordinates, binds, `@`); `ruled, unbuilt` (P2 — as-built `oracle::build_dialect` mints
  one last-wins set per family; build on `30J` §10's trigger); `proposal` (index-kinds, `30W` §10).
- TRUST: identity claims; within-kind incoherence is the kind-owner's (provides-kind-resolution).
- DEGRADES: no kinds ⇒ the markless floor (`plans/24L` auto-cell); each kind buys composition, each
  selector buys precision. Monotone.

### provides-reading — "here is how to measure that state, read-only" (+ the inertness self-vouch)
- WHAT: the recipes inside any role body (`dpkg-query -W "$pkg"`), shipped strip-only into the probe lane
  and, byte-identical, into apply guards. Writing sh in an oracle body at all vouches its probe-position
  inertness — the structural self-vouch, the only source of probe safety.
- DESIGN: `spike/CLAUDE.md` structural-vouch-only · rul-probe-mutation-ownership-split (`plans/27C` §3) ·
  rul-unprovable-rides-the-vouch · rul-no-mutating-guards · `KNOBS:kVOLATILES` · `23O`. STATUS: built.
- TRUST: the sharpest non-license liability; a mutating probe is contract collapse (no gradient).
  Enforced structurally + by the falsification-first effect check (never a completeness gate).
- DEGRADES: no recipe ⇒ cannot elide (safety intact); shallow recipes buy shallow verdicts. Monotone.

### provides-environment-witness — "the book-environment variables my body consumes, or shuts out"
- WHAT: positive speech only, in the body's sh: a PIN consumes a variable faithfully (the probe replays
  the book's value for exactly it); a SEVER is `env -i`-class hygiene provably keeping ambient variables
  from the check. A site below a book env-delta (`export AWS_PROFILE=prod`; a leading assignment-prefix)
  is probeable iff every delta variable is witnessed one of the two ways, else guard/run. No author ever
  enumerates a sensitivity list; the construct computes the complement.
- DESIGN: `plans/30S` (ruled 2026-08-24; §2 model, §3 refused routes) · `271:rul-env-claim-inversion` ·
  `ANALYZER-NEEDS:an-env-identity-carriage`. STATUS: `ruled, unbuilt` (reds
  `p-x-prefix-assignment-splits-fact-identity` +3; e2e `env30-book-exports-meet-oracle-envelopes`);
  as-built the ρ-fold keys by variable NAME only.
- TRUST: a false severance measures the wrong world under the right name — pope-sin. PATH/IFS/ENV-class
  deltas are engine-owned and wall dispatch; loader/locale classes are platform-oracle speech.
- DEGRADES: silence ⇒ sites below any env-delta withhold (today's containment is the empty stdlib); each
  pin/sever buys its sites. Monotone. `30S:seq-stdlib-gates-on-env-identity` gates aws/kubectl/psql-class families.

### provides-binding — "this measurement MEANS that cell"
- WHAT: the trailing mark on a runnable line — `cmd : KIND:ENT@sel` (`asserts`), `:!` (`refutes`), `:?`
  (`reads`) — one exit status onto one cell (rc-arity: at most one verdict per line; reads and meta marks
  are free; a mark-block may hold several and spill onto `:`/`#:` continuation lines). Backing = own cell ∪
  the line's `reads`. OBLIGATION: a verdict body marks EVERY cell its status reads (`cmp a b` marks both),
  else the fact can be wrongly spared.
- DESIGN: `plans/281` §3–§7 (THE grammar; carriers `:`/`#:`) · `277` §5 (backing sets) ·
  `30T:req-verdict-marks-every-read-cell` (+ `comp-backing-detector`).
- STATUS: built (marks, carriers, backings); `ruled, unbuilt` (the mark-BLOCK — as-built one mark per
  physical line; the trailing bind form; the read-vs-marks detector).
- TRUST: descriptive; an under-marked read is the cardinal-sin path the detector warns on; over-marking only
  widens invalidation (not-sin).
- DEGRADES: unmarked ⇒ anonymous fact, maximally conservative; each mark buys its cell. Monotone.

### provides-behavior — "here is what running it would DO" (three sub-shapes; code cites them by number)
1. **predicted observables** (`cmd__predict`), per channel: Status is the function's ordinary aggregate exit
   status, every value claimed (`return 2` predicts 2; no reserved decline); Stdout/Stderr default DECLINED,
   claimed only by an authored, statically-recognized, executed record trailing the bytes:
   `printf 'predicts stdout\n' >>"${DREP_V1:-/dev/null}"` (comma-set `rc,stdout,no-stderr`; `none` declines
   all = the whole-shape refusal). A peeling body (`"$@"` in command position) IS the wrapper declaration.
   DESIGN: `notes/30D` (+`30Da`) · `271:rul-predict-absorbs-wrapper-modeling` · `273`. STATUS: `ruled,
   unbuilt` (the channel algebra; sequenced with the stdlib revival by `30J` §10); built: status-only
   predicts, per-channel coverage blocking, peel detection.
2. **at-least effects** — establish/kill topology DERIVED from predict bodies' marks (never a declared
   marker). DESIGN: `ANALYZER-NEEDS:an-effect-class`. STATUS: built.
3. **at-most footprint** (`cmd__disturbs`, né touches) — per MATCHED shape, entities on stdout typed
   `: disturbs KIND[@sel]`; emission = the claim, none = no claim = wall; a body asking the host (`dpkg -L`)
   ends every completing path with `printf 'disturbs nothing-else\n' >>"${DREP_V1:-/dev/null}"` (mandatory:
   exit-0 truncation would under-claim). Entry-mutating verbs (`mv`/`rm`/`ln`) decline entirely at v0.
   DESIGN: `24A` §1b · `271:rul-touches-becomes-disturbs` · `plans/30U` §5 ·
   `ANALYZER-NEEDS:an-atmost-completion-signal` · `30T` §6. STATUS: built (`oracle::touches`,
   `plan::survival`); `ruled, unbuilt` (the completion record).
- CONSUMED BY: 1 → composed probes, lifted hand-guards, branch folds; 2 → kills, walls; 3 → survival past
  running walls under `--risk-faultless-skips`.
- TRUST: 1 self-checking (coverage blocks; parity gates); 2 description; 3 the traveled completeness claim —
  a wrong footprint silently under-executes SOMEONE ELSE'S line (cardinal-sin, flag-consented, attributed).
- DEGRADES: each sub-shape independently optional (`24A:rul24-threefunc-monotonic`). Monotone.

### provides-convergence — "for this invocation, taken whole: is the desired state true?"
- WHAT: `cmd__is_converged()`: 0 = the named sense holds; 1 = complement; ≥2 = cannot say (flat forever;
  runs). Declining is control-flow (`*) return 2 ;;`). The markless verdict-only oracle is the typeless floor.
- DESIGN: `plans/239` · `23O` §2 · `spike/CLAUDE.md` rul-rc-partition · rul-vouch-is-verdict-authoring ·
  `plans/24L`. STATUS: built. CONSUMED BY (license-free): drift display, hints; (licensed): see next.
- TRUST: an ANSWER carrying the adequacy gap (converged ≠ no-op; `an-adequacy-bite`): a wrong yes
  under-executes the author's OWN tool's line (attributed cardinal-sin).
- DEGRADES: absent ⇒ no verdict (facts may still display); each arm buys its verdicts. Monotone.

### provides-license — "I accept the blame for acting on my answer" (the ladder)
- WHAT: zero measurement content. Rungs over one answer: 0 display-only · 1 in-position (the guard
  `( f args ) || bytes`) · 2 carried (the line leaves the plan; past walls only under the admin's flag).
  RULED default: authoring the verdict member IS rungs 1+2, permanently; any opt-DOWN spelling is additive.
- DESIGN: `24M:rul24M-rungs-default` · `KNOBS:kCONTRACT-RUNGS` · `24D` §4 · `core::claim::Rung` (always
  `Both`); vetos PARKED (`notes/23M`). STATUS: built (the default); `open` (opt-down spellings, human-reserved).
- TRUST: pure liability; enforced by attribution (every elision/guard names its licensor) plus the admin's flag.
- DEGRADES: each rung trades author-liability for admin-value; declining a rung never removes lower value.
  Monotone iff rung-selection stays per-path — a constraint on any spelling that lands.

### provides-report-lane-records — "here is my out-of-band speech" (née provides-margins)
- WHAT: verb-led records, `printf '<verb> …\n' >>"${DREP_V1:-/dev/null}"` (total off-Dorc; the sink VALUE
  is engine-supplied, the NAME carries the format version). Verbs: `decline <class> <tail>` (classes
  `unsound`/`unmodeled`/`interactive`/`hazard`; aid-only) · `predicts <channel-set>` / `predicts none …`
  (load-bearing, sub-shape 1) · `disturbs nothing-else` (load-bearing: at-most completion witness and
  finished-definition act). Heads are closed grammar; tails are free and decision-inert.
- DESIGN: `27W` · `notes/30D` §3/§7 · `plans/30U` §4 · `spike/CLAUDE.md` decline-class-emission (LOAD-BEARING
  by design, 2026-08-23). STATUS: built (`decline`; live per-attempt capture); `ruled, unbuilt` (`predicts`,
  `nothing-else`).
- TRUST: a misclass misdirects attention only (mild-sin); the load-bearing verbs carry their shapes' knives.
  Only the literal fixed form mints authority; `echo`, a composed format, or an aliased sink stay feedback.
- DEGRADES: silence is legal everywhere; classing is enhancement; a missing load-bearing record withholds
  authority and never widens it. Monotone.

### provides-context-vouch — "this body stays read-only when executed shifted along DIM"
- WHAT: the standalone mark `: safe-across user` (`{user,fs-view}`), per function, path-scoped — read-only
  BY DESIGN, not by privilege-starvation; claims nothing about answers or other functions.
- DESIGN: `plans/27C` §2 · `plans/281` §5 · `spike/CLAUDE.md` context-entry-probing. STATUS: built
  (`oracle::entry`; e2e `context-entry-*`). TRUST: a false vouch is a probe break in an entered context
  (attributed to the three consents). DEGRADES: no vouch ⇒ never entered ⇒ guard/run. Monotone.

### provides-wrapper-map — "here is what my wrapper does to each context dimension"
- WHAT: `cmd__lend_map()`: peel as the tool does, end in `"$@"`; per dimension a valued line
  (`printf '%s\n' "$target" : lends user`) maps, bare `: lends fs-view` passes through, a MISSING dimension
  walls. Wrapper-ness is detected from the predict's peel; the two peels must agree (plan-time fail-fast).
- DESIGN: `273` · `271:rul-lend-map` · `271:rul-env-claim-inversion` (bare `"$@"` claims nothing; `env -i
  VAR=x "$@"` exactly-these) · `plans/24S` §0. STATUS: built.
- TRUST: a wrong lend value mis-keys facts (measured-wrong-world; capped by entry siting).
- DEGRADES: no map ⇒ every wrapped site opaque; each dimension buys its sites. Monotone.

### provides-context-entry — "here is how to get INTO the context my wrapper denotes"
- WHAT: `cmd__enter()` (name provisional): pre-entry sh ending in `"$@"` (`sudo -n "$@"`); non-interactive by
  construction; siting verified or declined (≥2); self-effects are the author's vouched residue.
- DESIGN: `plans/27C` §3 (reuse-never-acquire; the ternary dial). STATUS: built (e2e `context-entry-babby-*`).
- TRUST: a mis-sited entry yields confident wrong verdicts — the worst probe object (attributed).
- DEGRADES: no entry form ⇒ contexts never entered ⇒ guards at apply; each verified entry buys measurement.

### provides-kind-resolution — "these two names are one entity of my kind"
- WHAT: `kind__resolve()`: prints the canonical name, falls through to the input; one per kind per world.
- DESIGN: `24F`/`24G` · USER_STORY stage 6 · `core/CLAUDE.md` canonical-coord-continuity. STATUS: built.
- TRUST: a wrong merge over-verifies (not-sin); a wrong split re-opens the silent skip (cardinal-sin).
- DEGRADES: no resolver ⇒ plain name comparison; can't-answer ⇒ may-alias ⇒ run. Monotone.

### provides-kind-entailment — "disturbing an entity of my kind drags these other cells along"
- WHAT: `kind__disturbance_reaches()` (née disturbance_reaches_only), rung 1: invoked per footprint cell of
  its kind (`$1` = entity), emits entailed cells (`printf '%s\n' "$1" : disturbs sm.dorc.Service`; host arms
  `dpkg -L -- "$1" : disturbs sm.dorc.File`). Emissions only WIDEN footprints; never backings.
- DESIGN: `plans/30U` §3 · `24G` · `271:rul-at-most-family-names` · `ANALYZER-NEEDS:an-kind-reach`.
  STATUS: built (the widening half; e2e `strawman24-reach-*`, `pin28-reach-*`); `ruled, unbuilt` (the
  rename respell; the `unrelated` compare answer).
- TRUST: over-breadth walls; a dying host arm walls total; rung 1 alone cannot under-execute (not-sin).
- DEGRADES: silence ⇒ cross-kind pairs collide; each arm closes a hole. Monotone by construction.

### provides-finished-definition — "…and nothing else, in any vocabulary"
- WHAT: rung 2: `printf 'disturbs nothing-else\n' >>"${DREP_V1:-/dev/null}"` in tail position on a reached
  path of `disturbance_reaches`; content = the completeness claim, arrival = the execution witness; exactly
  one per completing path (zero = rung 1; two = refuse the footprint). The ONLY licensor of cross-kind sparing.
- DESIGN: `plans/30U` §1/§4 (`rul-cross-kind-sparing-needs-a-finished-definition`; `inv-30U-unary-never-pairwise`).
  STATUS: `ruled, unbuilt` (`30U` §10 pending a successor rewrite; USER_STORY stage 7 carries the FIXME).
- TRUST: the one dangerous sentence in a kind-owner's file — written early, it under-executes someone else's
  line (cardinal-sin, flag-consented, cited by file:line).
- DEGRADES: absent ⇒ cross-kind facts guard on drifted days (withheld, never wrong); the record buys survival
  shape by shape. Monotone; the incentive-inversion pin guards that a definition never reduces sparing.

### provides-store-topology — "where my kind's state lives, and which context axes cannot divide it"
- WHAT: `kind__state_stored_only_in()`: `printf '/proc/sys\n' : stored-in kernel` lines (the `only` contract
  on the emission set) plus whole-member `: undivided-by-transit-across fs-view` invariance lines. Invariance
  × the engine's read-set-closure proof = the unflagged cross-substrate carry; identity axes ride the flag.
  Owed re-cut (`30W`): per-arm `state_stored_in` + a `stored nothing-else` sentinel; index-kinds
  (Boot/Machine/LoginSession/Host) as axis targets; a keyed/invariant/⊤ trichotomy per (kind × selector × index).
- DESIGN: `272` (§12) · `277` §4e · `271:rul-invariance-speech-act` · `plans/27C` §4 · `plans/30W` §3/§4/§10
  (`rule-only-decomposes-everywhere`, `rule-incarnation-invariance-passes-the-razor` — owed).
- STATUS: built (the invariance line + pure-predicate carry, `oracle::carry`; e2e `carry-fsview-*`); the
  `stored-in` emissions parse and are inert (`an-store-topology`; the `30U` §7 collide-only consumer unbuilt);
  `proposal` (the `30W` decomposition and index-kinds).
- TRUST: a false invariance lets one world answer for another (cardinal-sin, attributed to the line);
  contradictions with the member's own emissions refuse at plan time.
- DEGRADES: silence ⇒ facts never cross contexts (only quieter); each line buys its axis. Monotone.

### provides-filesystem-binding — "this shell-routed write disturbs at most this file"
- WHAT: a File-kind role member (name NOT yet minted) receiving a structural LOCATOR (target word + cwd-state
  + mode) from a redirect/heredoc, answering with a whole-entity at-most claim (`sm.dorc.File:<path>`,
  selector-less) or declining; object and filesystem type measured inside the shipped body (a regular file on
  an allowlisted persistent fs binds; FIFOs, devices, procfs, network fs, unknowns decline); ends every
  completing path with the completion record; ships on the derived-footprint rails.
- DESIGN: `plans/30T` (2026-08-29; §0 rules, §3.2 contract, §10 `comp-fs-binder-member`) · `30U` §7.
  STATUS: `ruled, unbuilt` (none of `30T` §10 built; `comp-routing-locator` precedes it; the name is open).
- TRUST: the sub-shape-3 knife (cardinal-sin, flag-consented, attributed to the binder's arm).
- DEGRADES: no binder ⇒ every shell-routed write is a total wall; each answered locator narrows it. Monotone.

### provides-identity-tier — "same means THIS relation per selector; these regions overlap or not"
- WHAT: three owner declarations, all measured in the denoted context, per-aspect, perishable under namespace
  mutations: a per-selector relation mapping (contents → inode identity; existence → directory-entry
  identity); a referent-transparent declaration (`boot_id`, `machine-id`: entity inequality separates);
  and `kind__overlaps()` (region × region/entity; rc 0 overlaps, 1 provably disjoint, ≥2 unknown; the lazy
  answer is unknown by name-bias, the disjoint arm deliberate).
- DESIGN: `plans/30T` §6 (spellings unminted) · `plans/30W` §1–§3 (STRAWMAN `overlaps`;
  `rule-separation-from-identity-not-name`, `rule-two-consumer-name-bias` — owed) · `30T:comp-identity-tier`.
- STATUS: `proposal`. TRUST: a wrong "same" transports across two real worlds; a wrong disjoint spares past a
  real write (both cardinal-capable, attributed). DEGRADES: absent ⇒ same-kind path-distinct pairs collide
  (the v0 floor); the overlaps/unknown arms are safety-neutral; only the disjoint arm spends trust.

### provides-package-loading — "my package's dependencies, custody, and load hygiene"
- WHAT: a marked file's top level is load-inert (definitions, known-value assignments, exact `.` operands, the
  include-guard idiom `if [ "${x_loaded-}" != 'org.x/v1' ]; then . "$ROOT/org.x/entry.oracle.sh"; fi`,
  `unset -f`, subshell loading; never top-level commands). Custody = a file plus what its top-level `.` lines
  pull in: a vouch composes only helpers under the voucher's own custody; co-loading and a book's `.` compose
  nothing; ambient helpers stay legal sh and SUSPEND the vouch (never refuse). `command -v` guards are
  supported sh that withhold custody until their load model lands. Two live definitions of one family's member
  in a frame withhold that family's licenses unless the admin blesses the replacement.
- DESIGN: `plans/30I` §2.2/§3.4/§15 · `plans/28K` §1/§4 · `plans/28M` · `plans/30P` (the load principles;
  `law-no-unsoundness-below-a-blind-act`) · `notes/30Ic` · `spike/crates/oracle/CLAUDE.md`
  rul-vouch-reaches-own-custody-only. STATUS: built (custody closures, exact and guarded-source speaker
  edges, `unset -f`, subshell loading, bundles); `open` (`30I:pin-command-v-load-model`; `30I` §4.2).
- TRUST: a mis-composed custody blames the voucher for a helper they never selected (pope-sin) — hence every
  non-exact reach suspends rather than composes.
- DEGRADES: no `.` lines ⇒ each file answers alone; each exact source act buys its closure. Monotone;
  visibility never becomes refusal.

### provides-payload-declaration — "this input of mine is sh, running <somewhere>, with <properties>"
- WHAT: the `dorc:sh -c '…'` head token (full analysis invitation; strip erases the prefix; bare `sh -c` is
  the permanent escape hatch, hints only; no nested annotation inside opaque payloads) — and, beyond it, an
  authored speech-act naming an operand/stdin/file as code with a dialect, a fidelity normalizer (an authored
  function pre-applied to the bytes), an execution world, and custody.
- DESIGN: `274` · `271:rul-dorc-prefix-head-synthesis` · `276:rul-unsafe-is-bare-sh` · `plans/24T`
  (proposal-tier) · `notes/26M` (`q-payload-declaration-speech-act`; carrier-geometry survey first).
  STATUS: `ruled, unbuilt` (`dorc:sh`; e2e `payload-*-walls` pin the hatch) · `open` (the general act;
  ROADMAP `cand-payload-declaration-speech-act`).
- TRUST: a false fidelity claim analyzes bytes the host never runs (detectable by reconstruction differential);
  mis-siting an expansion's world is cardinal-capable (`26M` holes).
- DEGRADES: bare `sh -c` ⇒ opaque wall; `dorc:sh` buys analysis of held bytes; not sketched further.

----

Not shapes (grep bait, so nobody re-mints them): **value predictions** are DERIVED, never authored (`275`,
§6 NOT RATIFIED; `271:rul-value-prediction-species`; read `26B`/`26C` under `30D`) — the authored surface
is the empty set · **families, wrapper-ness, keying, backing sets** derive from names, peels, stores,
co-location · **cost hints** stay parked (`KNOBS:kPROBING`) · **version/context pins** (MH2) are deferred
(`270` §4; ROADMAP `design-mh2-version-layer`).

Tombstones (each replaced, none dropped): `provides-margins` → provides-report-lane-records · the `UNK`
refusal idiom → `decline <class>` (`27W`) · the `~` vouch-mark → verdict authoring IS the vouch (`24A` §1c) ·
`is_diverged` → the `:!` sense · `touches()` → `disturbs()`; `reaches()` → `disturbance_reaches()` (`271`;
`30U`) · the `#` selector → `@` (`plans/281` §R4) · `.prop` cells → `@selector` · `return 2` as a predict
decline → `predicts none` (`30D`) · the reserved grounding-bridges seat → provides-finished-definition +
provides-identity-tier · vetos → PARKED (`23M`), unpark-bar unchanged.
