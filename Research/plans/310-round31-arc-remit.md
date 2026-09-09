# 310 — r31 arc remit: the language-and-kernel build, cut into lanes

> human: ON HOLD: This document spawned design-sittings that have substantially
> reworked the core language. This will need to be completely rewritten after the
> 311+-series model-redesign stabilizes. /=

> AI-authored (Fable, the 2026-09-02 planning sitting; human present, ruling in chat).
> Implementation-focused and AHISTORICAL: this is the shape of the build — lanes, order,
> dependencies, merge points, and where the arc can be cut short — never the design. Every
> unit points at its design-of-record; the only content written out in full is where this
> remit SPLITS a design along a seam its document did not have (§4). Rulings the human owes
> are not steps of this plan: where a lane is gated on one, the gate is cited by slug and
> the lane waits. Root docs, `spike/CLAUDE.md`, the stamped designs, and `ROADMAP.md` (the
> direction-level cut, the owed rulings) outrank this file. Sizing words (small / medium /
> large) are relative to each other, never to wall-clock; the conductor sizes dynamically.

## §0 — the arc in one screen

r31 builds the language and kernel pieces that r30 designed and did not build, chosen so
that a *pivot book* — one file that stands up a machine from the controller and then
converges it over `ssh` — is analyzable end to end, correctly, in the plain-argv form of
`ssh host cmd args`. Everything else designed around that (payload forms, the identity
tier, selector-granular survival) stays out or held. The arc has four build lanes:

1. **`lane-tracers-and-records`** — the oracle tracers learn the line shapes the docs
   already teach: the taught decline idiom, `local`, the `predicts` channel records, the
   `disturbs nothing-else` record, and the read-versus-marks detector. This lane is also the
   gate before anyone hand-authors a real oracle.
2. **`lane-edge-local-exec-and-load`** — local-exec becomes a supported product mode
   (no host means the controller is the target), the delivery shape of `26N` §2 replaces
   the stdin pipe, and the load-plane rulings build once typed.
3. **`lane-kernel-index-rekey`** — the coordinate's context slot becomes a product over
   index-kinds; book-side environment identity (`30S`) rides that slot; Host is an entered
   index; transit verbs re-key; cross-kind comparisons answer `unrelated` and collide
   until a finished definition says otherwise. The largest lane and the one every ruling
   gate sits in front of.
4. **`lane-file-semantics-binder`** — the redirect locator, the filesystem binder member,
   and its claim consumption, so a heredoc write claims "at most this file" instead of
   walling the rest of the book. The second red-line seam: the arc is coherent with or
   without it.

The arc opens only after the test-architecture rebuild (`notes/30X`) has folded — every
lane's looms and reds are authored in the rebuilt shape, and lane 2 consumes the harness
seam it introduces. Conductors are serial; at most two builders run at once, always in
disjoint crate sets (§2).

## §1 — the lanes

Each unit: what it is (one line), where its design lives, what it touches, its acceptance
pointer. Order within a lane is build order.

### `lane-tracers-and-records` — `oracle/` (the predict and verdict tracers, the reaches
and touches lanes), the report-lane intake in `plan::records`. Entry condition: none.

- `unit-taught-decline-idiom` (small; FIRST) — the USER_STORY stage-3 shape
  `[ "$2" = "" ] || return 2` must decline exactly as the `if …; fi` spelling does; today it
  renders Run with no diagnostic. Design: the shape is human-authored teaching
  (`USER_STORY.md` stage 3; `oracle/CLAUDE.md` R2-MULTIOP), so the tracer is wrong, not the
  idiom. Acceptance: one loom pinning both spellings to the same disposition, plus the
  diagnostic when a decline is reached.
- `unit-local-leaves-the-deny-list` (small) — the ruled dialect is POSIX + `local`
  (`spike/CLAUDE.md` dialect-quality-law) while `notes/26J`'s tracer deny-list ⊤-degrades
  it. Model `local` in both tracers as a frame-scoped binding (it is `export`-shaped in the
  tracer's own terms, without the environment half); the rest of the deny-list stands.
  Acceptance: a loom whose verdict body uses `local` lifts and elides.
- `unit-predict-channel-records` (medium) — `notes/30D` built: Status keeps every value
  (`return 2` predicts 2), the closed `predicts <channel-set>` record recognized statically
  in the predict tracer and confirmed at runtime through the owned DREP capture, the
  contracted-input refusals, the fifteen `30D` §9 acceptance obligations as reds. Replaces
  the as-built `return 2`-as-decline in place (`rul-strawman-formats-no-compat`); every
  fixture and doc that spells the old form respells in the same commits.
- `unit-nothing-else-record-recognition` (medium) — `plans/30U` §4: the `nothing-else`
  argument in the report-lane grammar, the tracer's static-recognition arm in
  `disturbance_reaches` and dynamic `disturbs` bodies, tail-position/exactly-one enforcement,
  the mandatory-witness check for dynamic `disturbs` bodies, and the corpus-wide respell
  `disturbance_reaches_only` → `disturbance_reaches` (one commit, no compat). This unit
  PRODUCES a finished-status witness per (kind, reached shape) and consumes nothing —
  see §4a for why the consumer is in lane 3.
- `unit-backing-detector` (small) — `30T:comp-backing-detector`, the falsification-first
  read-versus-marks warning on verdict bodies; general across kinds, never a gate.

Unblocks: hand-authoring of the trial oracles (the human's, `notes/27Q` §2 preconditions);
lane 3's `unit-unrelated-and-settle-gate` consumer; lane 4's binder member (which is
another tracer role and another record producer).

### `lane-edge-local-exec-and-load` — `cli/` (argv, `transport_edge`, `artifact`),
`transport/`, `analysis::funcenv`. Entry condition: `notes/30X` folded (lane 2's first unit
consumes the harness seam).

- `unit-local-exec-mode` (medium) — the product mode `TODO.md` owes and `26K` §0b names
  as the pivot prerequisite. As-built: `transport::LocalDriver` exists and is reached only
  through a debug-only environment pin that `30X` retires into the harness seams
  (`30X:inv-fixture-state-never-typeable-into-main`). Build: an invocation with no host
  denotes the controller (`28Q` §3: "the controller is a context available-at-probe by
  definition"; the spelling authority is the human's cli-refresh note in `TODO.md` — no
  host prefix means local, `--book` goes; do not invent flags beyond that); the local
  driver is a first-class product driver selected by that invocation, not by environment;
  the artifact ships to the local shell over the same wrapper and routes the ssh driver
  uses (`unit-delivery-shape`; `260` §5 driver 2, ≥95% shared). Acceptance: plan and apply
  looms round-tripping a
  book on the controller under the rebuilt harness; the receipt records the controller as
  the target with no host name fabricated (`271:rul-sin-ordering` top cell).
- `unit-delivery-shape` (medium) — `26N:rul-delivery-shape-file-backed-default`, whole: the
  constant wrapper; the file-backed route (an exclusive scratch at a controller-literal root,
  the nonce-delimited stream materialised by a `while read -r` loop, per-file `cksum`, cd to
  the root, `sh /abs/<the book's filename>`); the in-memory route as the `auto` floor; the
  `capabilities=` marker key; host-side self-selection in one exchange; cleanup by manifest.
  The `260` §5 invocation line and driver 2 change with it, and the sim driver scripts both
  routes. Lands BEFORE lane 3's `unit-host-index-and-entry`, whose entry-composed probes
  would otherwise consume their own artifact. Acceptance: the existing marker/transport
  looms under both routes; a DST sometimes-assert per route.
- `unit-artifact-injectivity` (small) — `30T:comp-artifact-injectivity`, the apply-standup
  distinctness check; independent of everything, homes here because it is the artifact
  lane.
- `unit-load-model-builds` (small each; the first RULED, the rest GATED) — the builds behind
  the load-model rulings: the `$0` authority spelling (`26N:rul-dollar-zero-authority-spelling`,
  RULED 2026-09-03 — `30P:model-symbolic-dollar-zero` carries the amended text; the `cd`
  operand rides the same `$0` evaluation; `load30-point-havoc-and-script-relative` targets
  GUARD, never elision, and its empty-run-set expectation is re-read against
  `30P:law-no-unsoundness-below-a-blind-act` — `26N:fnd-blind-act-fixture-is-guard-at-most`)
  together with the unresolvable-load refusal (`26N:rul-unresolvable-book-load-refuses`: one
  pre-network code for a computed or relative-below-a-clobber book-custody `.`, naming the
  load line, the clobbering line, and the `${0%/*}` remedies; `cli/CLAUDE.md`'s ¬EXACT ship
  rule gains that pointer); the hoist ACTION's T2 tier (`tc-hoisted-dot-line-spelling`,
  `tc-t2-is-narrower-than-the-ladder-says`; `30Ng` §7's ladder); and whatever
  `30I:pin-command-v-load-model` rules. The gated ones wait on their ruling and build in
  isolation; none blocks the other units.
- `unit-acquired-source-weld` (small; GATED on the ruling `ROADMAP` names) — reconcile
  `30R`'s ordered role-carrying source vector with `30I`'s occurrence account
  (`an-static-load-occurrence-account`); one representation, no adapter.

Unblocks: the personal-target path (`26M` §sitting-tier-triage: local-exec → hand oracles
→ the book runs single-shot).

### `lane-kernel-index-rekey` — `core::coord` (the context slot, `ContextKey`, the compare
chokepoint), `analysis::effect`/`value` (site keying, the ρ-fold), `plan::settle`/`world`
(walls, re-keying, the survival gate), `oracle::entry`/`carry` (the Host entry).
Entry condition: the six `30W` §10 rulings typed (`ROADMAP`, sittings the human owes) —
the whole lane waits; `unit-env-identity` alone may start before them (§4b).

- `unit-env-identity` (medium) — `plans/30S` whole: the ρ-fold carries VALUES into
  `ContextKey`; a bare assignment-prefix routes through the same peel the `env` spelling
  takes; an ambient `export` between sites fences Must-transport; a site below an
  unwitnessed delta withholds with the two-remedy hint; the engine-owned resolution class
  (PATH/IFS/ENV) walls dispatch; measured deltas ride the why-chain. Acceptance: the four
  `r31` pins (`p-x-prefix-assignment-splits-fact-identity`,
  `p-x-env-wrapper-context-carries-values`, `p-x-ambient-export-fences-fact-transport`,
  `p-x-unwitnessed-env-delta-withholds-probe`) and the `env30-book-exports-meet-oracle-envelopes`
  loom green. Built on the existing two-variant slot; the next unit widens it (§4b).
- `unit-context-slot-audit` (small; read-only; the conductor's) — `30W` item 1's
  precondition: enumerate every reader and writer of `core::coord::Context` and
  `ContextKey` (compare, `FactKey`, probe keying, records, the ρ-fold, carry, survival
  witnesses, receipts) before the re-key is briefed. Output: the touch-list the next unit's
  brief carries; nothing else.
- `unit-context-slot-product` (large; retrofit-hostile) — `30W` item 1: the slot becomes
  a product over referenced index-kinds; `HostDefault` becomes the empty product; the
  wrapper-entered key and the value-carried env key become entries in it. Every reader on
  the audit's list converts in one lane; no adapter (`rul-strawman-formats-no-compat`).
  The compare chokepoint stays the one seat (`core/CLAUDE.md` relational-compare-chokepoint).
  One seat RESERVED, nothing built: a per-context measured-supply projection keyed by the new
  key, beside a per-chunk requirement set (`26N` §4.4) and the per-leaf stream demands of
  `26O:5-the-routing-planner` — the capability system's representation, built under
  `ROADMAP`'s `arc-host-capabilities`.
- `unit-host-index-and-entry` (medium; GATED additionally on `28Q:pin-ssh-entry-shape` and on
  lane 2's `unit-delivery-shape` having landed; the entered context it stands up carries the
  per-context scaffold of `26O:4-the-per-context-scaffold` — its own scratch, sink, frames
  written at minting, a closing count — so an entry-composed remote probe never drops its
  records: the `sudo` case rides this unit's shim carrier, the `ssh` case waits on the pin) —
  `30W` items 2 and 3 for Host only: Host as an entered index whose value is the wrapper's
  mapped lend; the Host binder minimal (destination string is the entity; no merging —
  `FORFEITS:forfeit-no-host-merging` stands; host-key continuity at the transport is the
  identity witness already built, `260` §5); the one re-keying rule (a disturbance of an
  index-value cell re-keys that index downstream) and expected-sever at the settle/wall
  seat. The argv-form fidelity carve rides here: an `ssh` entry form accepts a remainder of
  simple words as identity across the remote re-parse and declines anything else with a
  hint. Acceptance: under an inert `ssh` mock that execs its remainder locally (the `sudo`
  fixture shape), a book's `ssh host tool args` sites probe in the Host world, elide when
  converged, and a running transit above them re-keys them to guard/run; the
  `until ssh … true; do sleep N; done` shape neither elides nor walls (one red, the
  `26K:sit-wall-transparent-delay-loops` corollary as `28Q` §3 states it — no bespoke
  machinery, a pure body under a modeled `sleep`).
- `unit-measured-index-kinds-and-witness` (medium) — `30W` items 2 (Boot / Machine
  identity reads as ordinary stdlib predicts carried as witnesses) and 4 (the witness
  captured at probe, asserted at apply standup and after every fired index disturbance;
  mismatch is an integrity outcome — withhold — never a verdict input). Rides
  `unit-env-identity`'s envelope rails.
- `unit-unrelated-and-settle-gate` (small) — `plans/30U` §7: the compare answer gains
  `unrelated`; cross-kind pairs answer it before canonicalization
  (`core/CLAUDE.md` kind-fence-movable); the settle gate spares a cross-kind pair only
  through the footprint kind's finished-status witness from lane 1's recognition unit; the
  store-declaration collide-only consumer; the incentive-inversion regression pin. Lands
  BEFORE lane 1's recognition if the schedules cross (§4a).
- `unit-invariance-line-kind-targets` (small) — `30W` item 7's half that survival on
  patch day needs: `undivided-by-transit-across` accepts an index-KIND coordinate
  (`sm.dorc.Boot`) beside the closed axis tokens, sugar preserved. Item 6's store-member
  decomposition (`state_stored_in` + `stored nothing-else`) is NOT in this arc — it is
  gated on `rule-only-decomposes-everywhere` and is stdlib-arc-shaped.

Unblocks: pivot books in the argv form; lane 4.

### `lane-file-semantics-binder` — `syntax`/`analysis` (the locator), `oracle` (the binder
role, the derived-footprint rails), `plan::settle`/`world` (claim consumption), the
render's reasons. Entry condition: lane 3's `unit-unrelated-and-settle-gate` and lane 1's
`unit-nothing-else-record-recognition` landed (a binder claim is an at-most claim and needs
both the gate and the completion record); the binder member's NAME minted (the one open
spelling `ROADMAP` names — a provisional name is fine under
`rul-strawman-formats-no-compat`, but mint it before the brief, not in it).

- `unit-routing-locator` (small) — `30T:comp-routing-locator`: the parse yields
  (target word, cwd-state, mode); routing state is EXACT-or-havoc
  (`30T:inv-routing-is-exact-or-havoc`); everything downstream consumes it.
- `unit-fs-binder-member` (medium) — `30T:comp-fs-binder-member`: role recognition, arm
  trace, claim mint, ship-and-readback on the derived-footprint rails; the authored fs
  stdlib file is NOT this unit (stdlib arc), the contract it is written against is.
- `unit-binder-claim-consumption` (medium) — `30T:comp-claim-consumption`: binder claims
  enter footprint unions at the settle/wall seat; render reasons name the binder; cross-kind
  gating is `30U`'s machinery consumed, never re-derived.

Unblocks: the write-if-changed idiom keeping the rest of the book (under the flag); the fs
stdlib's authorship having feedback (`30T` §10 external couplings).

## §2 — dependencies, parallelism, merge order

```
30X folds ──┬─► lane-tracers-and-records  (oracle/, plan::records)          ─┐
            │                                                               │
            ├─► lane-edge-local-exec-and-load (cli/, transport/, funcenv)   ─┤  disjoint crates:
            │                                                               │  merge in any order
            └─► lane-kernel: unit-env-identity (analysis::value, core::coord)┘
                       │
   six 30W rulings ────┤ (the gate; nothing below it starts before they are typed)
                       ▼
                unit-context-slot-audit → unit-context-slot-product
                       │
   pin-ssh-entry-shape ┤
                       ▼
                unit-host-index-and-entry → unit-measured-index-kinds-and-witness
                       │
                unit-unrelated-and-settle-gate → unit-invariance-line-kind-targets
                       │                                 ▲
                       │            lane 1's nothing-else recognition (landed by now)
                       ▼
                lane-file-semantics-binder (locator → member → consumption)
```

- Lanes 1 and 2 are crate-disjoint from each other and from lane 3's first unit; up to
  two of the three may be in flight at once, each in its own worktree, folding by rebase
  onto `ai/main` in whatever order they finish. Lane 1 touches `plan::records` intake
  (the report grammar) and lane 3 touches `plan::settle`; those files do not overlap, but
  both lanes add reds under `crates/cli/tests/` — fold one before blessing the other.
- Lane 3 is strictly serial and holds the branch alone from `unit-context-slot-product`
  onward: the re-key touches every crate, and a parallel builder on any of them would
  merge into a moved coordinate representation. Lanes 1 and 2 must be folded before it
  starts.
- Lane 4 is serial after lane 3 and after lane 1's recognition unit; it may share the
  conductor's worktree with lane 3's tail.
- Merge-point rule for the whole arc: every fold re-runs `mise run both gate:full-quiet`
  on the folded result before the next lane dispatches; `gate:arc` once, from the populated
  branch, at close (`spike/CLAUDE.md` four-rung-gate-ladder).

## §3 — the two seams (where the arc can be cut, without restructuring)

- **`seam-payload-forms`** — everything above is the plain-argv form of a remote line.
  The heredoc form (`ssh h <<'EOF' … EOF`) and the sibling-file form (`ssh h sh -s <./x.sh`)
  both reduce to the payload-declaration speech-act (`notes/26M`, the standing focus; its
  carrier-geometry survey and holes are the human's sitting, `ROADMAP`). Neither is in a
  lane. If the survey lands mid-arc and its yield is small, the heredoc form's analysis
  half (a sub-parse of held bytes in the Host world, rendered whole-or-nothing per `26M`'s
  v1 posture) slots after lane 3 as its own unit; the sibling-file form pulls
  `FORFEITS:forfeit-plain-sh-inclusion-analysis` in with it and is a later arc.
- **`seam-survival-last-mile`** — lane 4 is the drifted-day product for config-writing
  tails. Cutting the arc after lane 3 leaves the language coherent (cross-kind collides
  correctly; finished definitions exist and are consumed; the Host index and re-keying
  work) and forfeits only the binder's value, which `30T` §9 already registers with reds.

## §4 — splits this remit makes that the designs did not (explained, since they are new)

### §4a — `30U` is cut along the producer/consumer seam, consumer first

`30U` §10 lists its units in one flat sketch. This remit puts the RECOGNITION half (the
record's grammar, the tracer arm, tail-position/exactly-one, the respell) in lane 1 and the
CONSUMPTION half (`unrelated`, the settle gate, the store collide consumer) in lane 3,
because they live in different crates and different builders' heads: recognition is
tracer work over authored line shapes; consumption is comparison and settlement law.

The order is fixed by safety, not convenience. As built, cross-kind pairs are spared on
nobody's speech (`30U` §2's "free rider"). Landing the CONSUMER first, with no kind finished
anywhere, makes every cross-kind pair collide — the safe direction, and the only change in
behaviour is under the flag. Landing the recognition first would produce witnesses nothing
reads (inert, harmless) — acceptable, but it means a window where the record is taught and
does nothing, which is the docs-ahead-of-build condition this arc exists to close. So: if
the two lanes' schedules cross, the gate lands first; if they do not, either order is
sound and the gate is never the one waiting.

The handoff type — a finished-status witness keyed by (kind, reached shape) — is minted in
`core` by whichever unit lands first and is the only shared surface. Neither unit re-lifts
or re-traces the other's material (`oracle/CLAUDE.md` the-frame-lookup-is-the-only-resolution-seat).

### §4b — `30S` builds on the existing slot, before `30W` widens it

`30S`'s own `open-env-epoch-seat-unification` leaves unresolved whether ambient deltas ride
the coordinate's context slot or a separate scope slot, and rules only that ONE slot family
serves transits, local-exec scopes, and env-epochs — settled by whichever sitting builds
first. This remit settles it by construction: `30S`'s value-carriage is a change to
`ContextKey`'s CONTENTS (values beside names), which the as-built `Wrapped(ContextKey)`
variant already holds; it does not need the product-over-index-kinds shape. So
`unit-env-identity` builds first, ungated, on the two-variant slot, and
`unit-context-slot-product` later moves the same `ContextKey` into one entry of the
product. The churn is bounded to the enum around the key, never the key's composition, and
the four `30S` pins green once and stay green across the re-key — which is also the re-key's
own regression net for that entry.

### §4c — `30T`'s independent units are pulled forward; its chain stays whole and last

`30T` §10 marks two units independent of the chain (`comp-backing-detector`,
`comp-artifact-injectivity`). They go to lanes 1 and 2 respectively, where the crates they
touch already have a builder. The chain itself (locator → member → consumption) is NOT
split across lanes even though its three units touch three crates: it is one feature with
one acceptance story, it sits behind the arc's second seam, and spreading it over three
builders would make the seam un-cuttable. `comp-identity-tier`, `comp-channel-relative-speech`,
and `comp-content-establishment` are out of the arc (`ROADMAP`).

### §4d — `30D` is in; `30J`'s P2 is out, against `30J` §10's trial clause

`30J` §10 says the family-vocabulary build moves ahead of any hand-authored real-oracle
trial. This remit rules P2 out of r31 anyway, and the reasoning should be visible: P2
changes only SELECTOR-DISTINCT, same-entity sparing (which tokens one family's marks mint
for survival), and the survival this arc buys for pivot books is cross-kind and
cross-index — `30U`'s finished definitions and `30W`'s invariance lines — where P2 is not
consulted. The hazard `30J` names (a trial under P0 trains authors toward fake predict
marks) is a hazard for a trial that evaluates selector-granular authoring; the pivot trial
evaluates the argv form, the Host world, and transit re-keying with a handful of the
human's own scrappy oracles. P0's collide-by-default stays the conservative floor. `30D`'s
half (`unit-predict-channel-records`) is in for the opposite reason: the docs and
`ORACLE_PROVIDES` already teach `predicts none`, so every oracle authored before it lands
churns when it does.

### §4e — `30L` is not a unit here

`ROADMAP`'s r31 section carried `cand-elision-regions` as unbuilt; the tree says otherwise
(`plan::region`, `plan::settle`'s universal meet, the route-aware in-loop floor, the
re-sized splice budgets — `ANALYZER-NEEDS` §A rows, all B). The one named residue,
`30L:pin-book-argv-value-plane`, is a winner-shifting value-plane widening that `30L` itself
says gets its own ruling; it is not in this arc.

## §5 — what a conductor verifies at close (pointers, not prose)

- `mise run xfail:census`: the four `r31` `30S` pins green; `load30-point-havoc-and-script-relative`
  green at GUARD (the ruling is typed; its elision expectation was wrong —
  `26N:fnd-blind-act-fixture-is-guard-at-most`), never re-horizoned (`LIVING_STATUS`
  standing truths).
- The `30D` §9 obligations, the `30U` §10 reds, the `30T` §9 reds for the chain if lane 4
  ran, and the lane-3 acceptance looms above, all in the rebuilt loom shape.
- `empty-world-byte-identical` pinned in every lane touching `plan` or `core`.
- Register currency: `ANALYZER-NEEDS` rows `an-env-identity-carriage`, `an-kind-reach`,
  `an-compare-chokepoint`, `an-coordinate-context-slot`, `an-atmost-completion-signal`
  re-graded to what landed; `ORACLE_PROVIDES` STATUS words likewise; `FORFEITS` rows whose
  capture landed rewritten or removed; crate `CLAUDE.md`s for every crate a lane touched.
