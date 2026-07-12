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

Sibling deliverables: the **entity-algebra design note** (the
`24C:rul-selector-pre-stdlib` obligation, tasks 1–2 below) is its own document when
authored; large arcs get their own comprehensive durables — **`notes/272`**
(address-derived topology, task 3) and **`notes/273`** (the wrapper surface, task 5) —
while this ledger carries the rulings and the arc-pointers, never the design bodies.

## Session task map (mirrors the conductor task list)

1. adj-entity-algebra, spelling half — the authored entity/selector mark grammar.
   CLOSED 2026-07-10: all six sub-rulings typed or dictated (see "The entity-algebra
   spelling direction" below); mechanical close-out rides task 12.
2. adj-entity-algebra, seams half — the two `24S:A7` reservations. CLOSED 2026-07-10
   (three typed acks + the "family" minting; see the seam rulings below).
3. adj-trichotomy-spelling (né `24S:A3`) — CLOSED 2026-07-11: dissolved into the
   address-derived-topology design; comprehensive durable = **`notes/272`** (naming
   typed; component statuses in its §12 table); this ledger keeps only the rulings.
4. adj-axis-vocabulary (né `24S:A1`) — ratify v1 = {user, fs-view} + ρ,
   versioned-closed, engine-owned. CLOSED. (Amended 2026-07-11 by
   rul-networking-unpunt: netns ahead of full fs-view.)
5. wrapper context-function spelling (né `24S` §2b) — CLOSED 2026-07-11: the
   surface redesigned wholesale; comprehensive durable = **`notes/273`** (statuses
   in its §12); this ledger keeps only the rulings + the arc-pointer below.
6. eval'er declaration spelling (né carrier; renamed by rul-evaler-vocabulary) —
   `24T` pin1 (which-arg-is-code, stdin shapes, argv-binding, the child-shell context
   record). IN PROGRESS 2026-07-11: merge-vs-structure-member analysis drafted,
   awaiting ack (rul-evaler-merge-no-structure-member).
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
13. containers-lens re-skim (optional; riding the entity-algebra crosscheck).
14. triple-check the structural-vouch hard law in a fresh session; then adjudicate the
    deferred rul-only-oracle-bytes-ship (the `24J` raw-ship repair).
15. semantics-proliferation stance (minted mid-task-6): whose interpretation of a
    tool's language/flags does an author answer for — eval'er heads the intense case,
    GNU-vs-BSD grep the everyday one.

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

## Rulings (task-3/task-5 sitting, continued)

### rul-touches-becomes-disturbs  (task 3/5 sitting; 2026-07-11; TYPED — "'touches ->
disturbs' is a winner")
The per-TOOL mutation-claim role renames `touches()` → **`disturbs()`** (liability
register: recruits paranoid completeness, the omission-dangerous direction). Rides the
corpus-respell churn window. Other family names NOT settled: `is_converged` keep
(typed, earlier); `predict`/`resolve`/`reaches` proposals delivered, unruled; the
store-member name DEFERRED pending the substrate-mark contract ruling (name follows
contract — see thread below). The naming-proposal menu itself is in-chat, 2026-07-11.
~~Partially superseded same date~~ — the reaches/store deferrals resolved by
**rul-at-most-family-names** below.

### rul-class-prefixed-role-names  (same exchange; 2026-07-11; TYPED)
In all generic discussion and documentation, role-functions are written WITH their
keying class: `cmd__disturbs()`, `kind__reaches()`, `kind__stores()` — never the bare
role name. The human: saying `cmd__touches()` and `kind__lives_in()` by default "would
have significantly helped without any naming change." Binds this ledger, the design
note, briefs, and doc prose forward.

### rul-networking-unpunt  (task 3/5 sitting; 2026-07-11; TYPED — amends
`rul-axis-vocabulary-v1`)
Containers make networking/network-naming core 2026 single-host ops; the human's
internal network-punt (spike + security + complexity) is lifted for the AXIS tier:
**netns re-enters the axis roadmap explicitly, sequenced AHEAD of full fs-view**
("way more pressing than the full complexity of bind-mounts/overlayfs").
Single-host-single-universe still governs spike3 analysis (the bare-metal host stays
inviolate). Conductor calibration riders (in-chat, un-nacked): the netns axis is
naming/scoping ONLY — no network access, `24S:imp-5` no-cross-host untouched; it is
mechanically cheap (`ip netns exec NAME` yields argv-named axis values exactly like
`sudo -u USER`; the net-kernel substrate has no aliasing ladder — no symlink/bind/
copy-up analog; cross-namespace state is disjoint-by-construction); it serves the
ip/nft/sysctl tool class, while docker-MANAGED networking stays behind the endpoint
substrate, opaque beyond the socket. Consumers: the substrate carried-by table
(net-kernel rows), `270:block-context` sequencing, the entity-algebra design note's
context-slot examples.

### rul-lint-never-drives-design  (2026-07-11; TYPED, standing — human-voiced;
durable home beyond this ledger is the human's call, spike/CLAUDE- or KNOBS-adjacent)
Lints are never a rescue from a bad design and are never discussed during design:
either re-design, or absorb the limitation as a core, frontloaded, README-class
breakage/constraint. Lints are built at the END of design, from that frontloaded
list, to make documented limitations sting less — never built early so the breakage
can be omitted from the README. Compliance note: the conductor's prior lint-flavored
design arguments in this sitting (binding-smell lint as adjudicability support; the
file-entity nudge; "a lint writes itself") are STRUCK as design justifications;
candidate D's case was re-audited and stands without them (native spelling; off-ramp;
pointable post-hoc attribution; differential testability as stdlib-CI verification;
substrate future-proofing).

### rul-net-quality-u-curve  (2026-07-11; TYPED, standing — human-voiced; same
placement note)
For a product designer, catch-net quality is U-shaped: perfect mechanical nets
(bad-states-unrepresentable) and honest documentation are both good; the dip between
them — linting / provably-imperfect best-effort mechanical nets — is a design
footgun. Documentation as a goalpost is SUPERIOR to imperfect mechanical nets: it
forces design-by-principle and honest communication. Supersedes the conductor's
"documentation-tier = weakest net" framing (retracted); the at-spool decline-clause
is re-graded from weak-apology to the design-tier artifact, with the differential
harness positioned as our own stdlib-CI verification, not the product's rescue.

### rul-at-most-family-names  (task 3/5 sitting; 2026-07-11; TYPED — "I like the
naming. Let's bake that moving forward")
The at-most claim family's names, ratified: **`cmd__disturbs()`** (né touches; NO
`only` — the contract is at-most per MATCHED INVOCATION-SHAPE, gradual across shapes
per USER_STORY stage 5, so the human's reservation rule excludes the marker) ·
**`kind__disturbance_reaches_only()`** (né reaches; trailing `only` — leading
`only_reaches` misparses as "merely reaches") · **`kind__state_stored_only_in()`**
(the task-3 member; earns `only` most — its key consumer reads the emission's
negative space, so a partial answer is meaningless, not partially-trustworthy). The
general rule rides with it: **`only` in a role name = complete-by-contract,
totalistic-survey-before-authoring; its absence = arm-incremental,
gradual-enhancement-friendly.** The chain teaching-sentence: a command DISTURBS
cells; that DISTURBANCE REACHES ONLY what the kind-owner enumerated; a kind's STATE
is STORED ONLY IN the substrates its owner declared. Consumers: the corpus-respell
brief (`270:block-rebuild` — NEW RIDER: these renames ride the same churn window) ·
all future documentation (human directive) · the entity-algebra design note ·
`notes/272` (uses them throughout). Unruled remainder: `predict`/`resolve` (menus
delivered in-chat 2026-07-11, low urgency); wrapper-member names (task 5, owed the
same read-aloud + only-rule treatment).

### rul-simulate-env-user-authored  (task 5; 2026-07-11; TYPED direction — "I
extremely-strongly-prefer 'user writes' to 'engine generates' for the
'probe-env-closure', that's a fucking fantastic idea"; ratification explicitly
CONTINGENT on the same-turn nit investigations, "suspiciously pretty")
The ρ-transform surface is a USER-AUTHORED executable env-closure — never an
engine-generated replication recipe. Name direction (his): **`cmd__simulate_env()`** —
simulation-framing frontloads the never-perfect-fidelity limitation in the title and
rhymes with `cmd__predict()`; conductor grammatical variant `cmd__simulates_env()`
offered (third-person, matching `cmd__disturbs()`). NOT yet settled: separate member
vs folding INTO `cmd__predict()` (the nit-predict-merge disposition, task-5 thread).
**→ RESOLVED by rul-predict-absorbs-wrapper-modeling below (2026-07-11): the
member-name `simulate_env`/`simulate` DIES; the authored body and its contract
survive whole, as the wrapper-shaped `cmd__predict()`.**

### rul-predict-absorbs-wrapper-modeling  (task 5; 2026-07-11; TYPED — "Merge
acked. merge-riders: acked")
ONE modeling member: **`cmd__predict()`** — "the best read-only sh model of your
command." Wrapper-ness is DETECTED, never declared by member-kind: a body whose
command-position `"$@"` runs its argument-slot is a peeling wrapper by tautology
(a wrapper IS a command whose behavior-model contains its argument-slot in command
position); the argparse path to that `"$@"` is the peel, the env-idioms along it
are the ρ-claims, conditional printf lines are output-claims, marked lines are
facts. No separate simulate member exists. The axes/emission member SURVIVES
separately (world-plane coordinate claims are not prediction; it is su's only
authorable member). **The three merge-riders are ACKED as build obligations
(typed, same message):** per-channel-decline vocabulary (delegation = faithful
claim · printf = asserted output · explicit return = rc claim · redirect-to-null
= per-channel decline ⇒ ⊤ · `return 2` = whole-shape decline) · line-level
attribution (the `24S` §4a chain's first link cites the LINE within the body) ·
opt-downs-additive (rul24M-rungs-default pattern; kCONTRACT-RUNGS unmoved).
Consumers: `270:block-context` wrapper stages (reframed as predictor-support +
the axes member); task-6 carriers (working frame: a carrier is most naturally
predict-with-a-code-operand); the stdlib wrapper-oracle briefs.

### rul-only-oracle-bytes-ship — ratification DEFERRED (2026-07-11; TYPED:
"continuing to defer on the repair; I'm strongly suspicious it's debt, but I
want to triple-check that hard law in a new session")
The repair stays DRAFTED, not ruled. The human wants the underlying hard law
itself (the round-20 structural-vouch ruling + the round-23 strip-predict
correction chain) re-derived in a fresh session before ratifying — not trusted
from this sitting's reading. Minted as top-level conductor task #14; the
`24J`/`24C`/LIVING_STATUS corrections stand as debt-CONFIRMATION (that part he
acked by directing the minting), only the repair-direction ratification waits.

### rul-lend-map  (task 5; 2026-07-11; TYPED — task-5 closing ruling)
The wrapper's dimension member is **`cmd__lend_map()`**: a function from the site's
argv to fixed strings, one entry per dimension — an empty result for a PRESENT key =
full lend (spelled as the colon-line, `:   : user`); contents = mapped lend (the
value-emission, dimension in the mark); a MISSING key = ⊤, walls, hint-tier nudge
(the enumerate-every-dimension law — the absent-key-means-full-lend reading is
explicitly REJECTED); terminal `"$@"` = the peel boundary; no `only` in the name
(arm-incremental member). Rider recorded (the human's safety-inversion correction,
conductor-conceded): ALL entry-types feed transport and are dangerous-when-wrong;
believed-no-overlap is safe for the transport/elision consumer and dangerous for the
kill-traffic/survival consumer, and vice versa — the inversion the ternary
`compare()` relation and the never-derive-separation carve exist to encode. Full
design, strawmen, rationale: `notes/273` §3–§4.

### rul-evaler-vocabulary  (task 6; 2026-07-11; TYPED — "'Carriers' is a very weird
word… Let's stick to eval'ers")
The code-evaluating-head family is named **eval'er** (né carrier, `24T` pin1's
"licensed-code-carriers"; slug-form `evaler`). "Carrier" retires from prose going
forward; corpus citations get the né-subscript treatment progressively, no bulk edit.
Naming dividend recorded under the merge ruling below: if the merge holds, the family
mints NO new member name at all.

### rul-evaler-floor-fixed-set  (task 6; 2026-07-11; TYPED direction — "I wouldn't
mind a baked-in list of blessed-eval-alikes")
Symbol-grounding hackles are LOW here, unlike kinds: an engine-owned, dialect-versioned
blessed eval'er head-set is acceptable as the grounding floor for the reentry
primitive. The human's framing (typed): the set is effectively FIXED by the product's
own nature — we are the *dash* orchestrator; a foreign-shell head (`bash -c`) means the
runtime evaluates a language we cannot parse, and growing warts-replicated per-shell
parsers is a permanent no. The blessed set therefore FALLS OUT of the
semantics-proliferation problem (task 15), not from an awareness/habit/taste-driven
bless list. Rider, banked so it isn't re-derived: the human's own opposite-direction
strawman — allow NO real shell names; the user types `dorc` (or `eval`) and the host
side swaps in the demonstrably-present running shell — was killed BY ITS AUTHOR in the
same message (probe-apply divergence); any future scheme in this space must pass
probe-apply fidelity first.

### rul-evaler-merge-no-structure-member  (task 6; 2026-07-11; DRAFTED-AWAITING-ACK —
the human's decider, the conductor's audit)
The human's settling decider (typed, with a stated lean to env-unknown-idiom): a
separate structure-member is justified only if the two members' argparses would
genuinely DIVERGE in control-flow ("entire branches filled out in one licensure may be
a flat refusal in another"); if they are basically-always duplicated code, avoid the
duplication and add some sort of annotation. Conductor's cohort audit (in-chat,
2026-07-11): across the plausible v1 eval'er cohort — sh/dash (the floor itself),
bash/busybox-sh (authored delegation), su/runuser (the straddle), fish/csh
(both-members-refuse), composed sites like `sudo sh -c` / `find -exec sh -c`
(divergence across TOOLS, never within one) — NO member pair diverges structurally;
every difference confines to the TERMINAL ACTION of individual branches, and always on
exactly the ρ dimension (su's login branch: env unclaimable, structure knowable).
Drafted ruling: **no structure member; eval'er-ness rides `cmd__predict()` detection**
(delegation into the blessed reentry primitive), and the su-class divergence is spelled
as ~~a **partial ρ-decline annotation on the delegation line** —
`dash -c "$code" "$@"   : env-unclaimed` — a third engine-owned closed mark
vocabulary~~ — **SUPERSEDED same date by rul-env-claim-inversion below (the human's
counter-proposal): no mark, no third vocabulary; the distinction rides the env-idiom
grammar itself.** Consequences per consumer stand as stated: value-plane tracing ⇒
payload env-references go ⊤ (argv-literal payloads analyze whole — the field-trial
`su - postgres -c` line lands); probe-form shipping (task-14-gated) ⇒ closure
unbuildable ⇒ participant unshippable ⇒ can't-say ⇒ run.
Consistency note: the predict×lend_map dual-peel duplication stands unaffected
(different answer-TYPES; the refined principle: never mint a member whose body would be
a branch-terminal-only clone of a sibling's — express branch-terminal differences as
in-body claim vocabulary instead).

### rul-env-claim-inversion  (task 6; 2026-07-11; DRAFTED-AWAITING-ACK — the human's
counter-proposal, conductor-endorsed and sharpened)
The ρ-claim reading of a predict body INVERTS from the earlier draft: **bare
command-position delegation claims NOTHING about ρ** (env-⊤: unshippable closure,
no env value-flow believed through the body), and the passthrough claim is spelled
POSITIVELY as **`env "$@"`** (the human's "vaguely plausible made-up-spelling for
'copy my env as it is'" — in fact a real idiom: no-op-adjacent, forces PATH-lookup,
reads as "hand off to the environment as-is"). The full per-rung claim grammar, every
rung a runnable sh idiom:
  - bare `"$@"` (or `dash -c "$code" "$@"`) — ρ-claim: NOTHING (⊤). The su login
    branch is spelled as exactly what a naive author writes.
  - `VAR=x "$@"` — per-variable claim: VAR known, rest ⊤ (deliberately under-claims
    the real passthrough; per-variable per `24S` §2b).
  - `env VAR=x "$@"` — full passthrough with named overrides.
  - `env "$@"` — full passthrough, unmodified.
  - `env -i VAR=x … "$@"` — exactly-these (the scrub-base, unchanged from `273` §2).
Sharpening (conductor; wants the human's eye): bare means **claims-nothing (⊤)**,
NEVER "claims-isolation" — a positive not-shared fact would be derived separation
(the `272` §4 never-derive-separation carve reaches here; both readings coincide for
tracing and shipping anyway, so the safe one costs nothing today).
Why this beats the mark, recorded: (1) UNIFORMITY — it is the ρ-side twin of
rul-lend-map's enumerate-every-dimension law (absence-means-full-lend was rejected
there; the earlier bare-means-passthrough draft quietly reinstated it on the ρ side —
the inversion removes the inconsistency); silence=floor everywhere. (2) No new mark
vocabulary; the engine-owned closed sets stay two; the queued kOOB reading shrinks.
(3) ~~NO translation-gap~~ — **RE-SCORED same date under the human's challenge** (the
in-chat walk-through): the gap is REDISTRIBUTED AND DEMOTED, not eliminated.
Contradiction-tier: neither scheme ever had one — both readings are sound WEAKENINGS
of the bytes' behavior, and the conductor's original pricing of the mark overstated
this axis too. Reader-divergence-tier: BOTH schemes carry one — the mark concentrated
a nastier annotation-cancels-code flavor on rare lines; the inversion spreads a milder
analyzer-believes-less-than-code flavor across every bare body (which is the
analyzer's universal posture toward all sh), and mints one genuinely new cost:
`env "$@"` MEANS more to the analyzer than it does at runtime — annotation-by-idiom,
the product's founding move (the admin's own `dpkg -s … ||` guard is precisely a real
idiom whose presence carries analyzer meaning), where the mark was
annotation-by-annotation, the thing the kOOB redline exists to ration. The
LOAD-BEARING win is therefore the omission-direction/razor argument, sharpened: under
bare-means-passthrough, an ignorant author's naive body mints a positive,
possibly-false claim SPELLED BY AN ABSENCE — no line says the false thing;
razor-failing; the exact omission-shape rul-lend-map's enumerate-law killed — while
under the inversion, ignorance mints ⊤ and every believable claim is a typed,
pointable line. (4) Cargo-cult razor alignment: the
`env` syllable is never silently-superfluous — it IS the claim; and a shell-literate
reviewer who "cleans up" the redundant-looking `env` NARROWS the claim (passthrough→⊤)
— cleanup-vandalism fails safe. Lane framing (the human's probe-vs-apply question,
answered): a predict body's env-spelling has exactly ONE executing consumer — the
probe-lane closure; it never exists in the apply artifact (`24S:imp-7`), so the
runnable-fidelity bar is probe-stand-in-tier — but the lanes do NOT lower the stakes
of the CLAIM: a wrong env-claim reaches apply decisions through analysis (wrong
value-flow → wrong-CONVERGED → wrong elision), same knife-tier as a wrong check body.
Costs, priced: identity wrappers owe one syllable (`env "$@"`) or lose env value-flow
— `273` §8's bare-`"$@"`-predict line needs a superseding annotation on ack; stdlib
cost trivial. Guest-less `env "$@"` executes as `env` (prints environ) — argparse
declines guest-less shapes anyway.

## Direction & open threads

- **thread-env-cannot-exec-functions (flagged during task 6, 2026-07-11; PRE-EXISTING
  landmine, not new debt):** `env` execs binaries via PATH — it cannot invoke a shell
  FUNCTION. Probe-form composition (`273` §6, task-14-gated) substitutes participants
  with `__predict` FUNCTIONS, so any env-headed closure body (`env -i … "$@"` — already
  present in the ACKED `273` §2 sudo strawman, independent of the claim-inversion)
  cannot literally exec a function-substituted guest. The shipped form needs a
  reconciliation (engine-composed closure text; prefix-assignment forms — noting POSIX
  leaves prefix-assign-on-function persistence murky, dash/bash diverge). Route: task
  14 + block-context implementation-planning. Discovered while auditing
  rul-env-claim-inversion; applies to the pre-inversion draft equally.
  **Candidate on record (human, 2026-07-11; self-described "kinda messy maybe"):**
  materialize participating `__predict` functions to disk as command-files at
  probe-ship time + prepend a probe-directory PATH element to the shipped compound.
  Pros: exec-uniformity — env-headed closures just work; functions become real
  commands. Cons, named: inherits the kCOMMS writable-fs residual (stripped/Windows
  targets, `plans/142`); PATH must be woven through EVERY closure rung, including
  authored `env -i … PATH=…` scrub-bodies (engine-rewritten PATH text brushes the
  task-14 question directly); fs-writes ride the DST transport seam; cleanup
  obligations on the host. Possible needle-thread, recorded: PATH-materialization is
  SCAFFOLDING-not-semantics — the semantic content stays authored oracle bytes; the
  engine contributes only plumbing — which may satisfy whatever task-14's
  fresh-session law-check ratifies. Alternative shapes (subshell + export-loop
  emulation of the closure around a function CALL, no disk) converge on the same
  task-14 adjudication.

- **thread-semantics-proliferation (minted as task 15; human-raised mid-task-6,
  explicitly raised-not-answered):** the admin has selected Dorc AND a shell (identity
  unknown to us, presumed dorc-sh-compatible-enough by the fact Dorc runs at all); does
  the oracle-author get to select DIFFERENTLY? Most intense at eval'er heads (whose
  payload-dialect do we honor?); the everyday form is portability footguns (GNU vs BSD
  grep flag semantics). Wants a general stance "besides just eval." The eval'er-local
  fragment is already directed (rul-evaler-floor-fixed-set); the general stance is
  task 15's.

- **task-3 arc — CLOSED 2026-07-11; the comprehensive design durable is
  `notes/272`** (minted at the human's direction: "large enough to deserve a separate
  document"; this ledger keeps rulings plus this pointer). The arc in one line: the
  trichotomy question dissolved into **address-derived topology** — the ratified
  `kind__state_stored_only_in()` member (locator + substrate-mark emissions;
  derivation = the engine-owned carried-by table + emission-set non-interference over
  the blessed who-am-I labeling; the never-derive-separation carve, docker-validated;
  the addresses-are-not-coordinates fence, registered as the FOURTH pointer at the
  parked co-reference mechanism; the differential harness as the derivation's
  load-bearing other half). The full attack-round record — the greatest-hits mint,
  the collapse-analysis, the microtypesystem/relation+generators framing, the
  five-strawmen survival round (in-tree: `notes/27x-strawmen-topology/`), the
  too-small attack and its substrate repair — lives in `272` §§0–11, with
  per-component ratification status in `272` §12 (only the NAMES are typed; the
  carve, fence, substrate marks, and relation-spine each still want an explicit ack).
  Residuals routed: task-7 gains a second customer (the axis-independence
  value-bound for dynamic locator arms); task-8 re-reads its adjudicability
  condition against derived-not-declared; task-12 imports the one-page formal spec,
  the fence, the substrate-slot reservation, and the keyed/partitioned vocabulary
  decision; task-13 carries the containers lens; the corpus-respell brief gains the
  rename rider (rul-at-most-family-names).

- **task-5 arc — CLOSED 2026-07-11; the comprehensive design durable is
  `notes/273`** (the `notes/272` precedent: extensive redesign deserves its own
  document; this ledger keeps rulings plus this pointer; the dialogue chronology
  lives in this file's git history). The arc in one line: `24S` §2b's four-job
  context-function dissolved — **`cmd__predict()` absorbed wrapper modeling**
  (wrapper-ness detected by command-position `"$@"`, never declared by member-kind),
  the dimension member became **`cmd__lend_map()`** (enumerate-every-dimension;
  full/mapped lends; guest/dimensions vocabulary), the engine-built whole-ρ closure
  (`24S` §6b) died into the authored predict body (probe-form composition, DRAFTED,
  gated on task-14's fresh-session law-check), the `24S` §2a stage-D disjointness
  dividend was confirmed dead under never-derive-separation, and dual-peel
  disagreement was ruled static incoherence ⇒ fail-fast. Typed rulings stay above
  (rul-predict-absorbs-wrapper-modeling · rul-lend-map ·
  rul-simulate-env-user-authored · the deferred rul-only-oracle-bytes-ship);
  mechanism, license anatomy, output-prediction direction, fences, warts, and
  vocabulary live in `273` (per-component statuses in its §12). Residuals routed:
  task-6 frame (carrier = predict-with-a-code-operand) · task-7's four customers ·
  task-8's inversion sharpening · task-14 (the §6 gate) · block-context and stdlib
  briefs (`273` §11).

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
