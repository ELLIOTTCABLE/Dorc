# spike/crates/oracle — CLAUDE.md

Lifts an oracle's sh **statically** (never sources or runs it) into the *kind-index*: the 3-place relation
`(kind, provider, verb) → effect` (`an-kind-index`) plus a read-only `FactProbe` per kind (`an-fact-probe`).
Read `spike/CLAUDE.md` (invariants) and `Research/plans/191-spike2-keystone-charter.md` (the charter) first.
This crate is the home of the `kTYANNOT-inline` experiment (`ch-shape-anno`) and the stdlib-oracle quality bar.

## What this crate is — and its place in the keystone

The fact-centric pivot is settled and built (`16P` DP-1 / T7): an oracle declares a **named kind**, a
**read-only fact-probe** (three-outcome: holds / absent / can't-tell — `an-probe-shape`), and an **effect-map**
`(provider, verb) → (kind, polarity)` (`an-effect-polarity`). A book's bare `apt-get install -y nginx`
resolves through the effect-map to a fact; the kind name is the *only* cross-oracle anchor (apt's `package` ≡
yum's `package`), never decoded for meaning (`inv-referent-agnostic`, `an-named-kind`). Never probe by
dry-running the mutator — `an-fact-centric` (the command-centric strawman `mycmd.check(){ mycmd --dry-run; }`
made the named-kind index decorative in the elision path; `16P` DP-1).

+SURE, traced in `src/lib.rs`: today this crate is the **skeleton of `17N` Part-I with none of its
type-discipline**. `Polarity{Establish, Kill}` is a *binary bit*, `FactProbe` is a flat verbatim body, the
index keys a flat `(kind, provider, verb)`. The charter's keystone is to re-key this: `Polarity` becomes a
**typestate transition** on a kind's ≥enum state-model (`inc-7` — "the effect-map ≡ a typestate
transition-table"), and the kind grows a structured, per-selector entity-algebra (`an-entity-shape`,
`an-per-entity-selector`). `ap-1` is explicit: **build that re-key before more type-machinery** — it
invalidates everything downstream keyed on the flat pair, and is *the spike*. The poison-wall demonstrator
(`apt-get update` poisoning `apt-get install nginx` to non-elidable) is killed only by structured cells:
`update` establishes `package-index#fresh`, `install` establishes `package:nginx#installed` — different cells,
no poison (`16Q §1`, charter §3).

## Spike-2 work in this crate

- **The `kTYANNOT-inline` experiment (`ch-shape-anno`).** Lift an oracle's declaration of a kind, its ≥enum
  states, and its kind-typed fields. The charter rules the *inline TS/Flow-style* annotation strawman for this
  spike (`local w : com.frobber.Wombat{defrocked,frocked} = "$1"`; `17N` top paragraph). **Known debt, accept
  it:** this is *not* a behavioral no-op and **breaks the off-ramp weld** (`17O` F-OFFRAMP, verified live: stock
  `dash` aborts `local: :: bad variable name`; `bash` silently leaves `w` empty rc 0; dotted name fails
  `dash -n`). Do **not** build the correctness-critical strip/transpile pass (`ch-shape-anno`). The parser is a
  disposable test front-end — massage inputs past it; arbitrary shell-input is a non-goal.
- **The recursive kind-typed entity-algebra (`ch-entity-algebra`, `an-entity-shape`).** A kind's fields are
  typed by the named-kind namespace *itself* — a `service`'s field can be typed `file` or `user` (Wombat-style
  handle-to-kind, *reusing* types established elsewhere, not a bespoke value-type system). Shape lean (`17N §4`):
  present-key = `true`, `!`-pun for false, values = direct types / kind-handles / nested structs, **absent ≠
  asserted-false** (the carry-vs-compare split, C6). This is `ch-priority` #5 — **the first thing allowed to
  give** when the going gets hard; simplify its shape before abandoning the keystone, but `an-finite-domain`
  (depth-bound the recursion — `seam-finite`) stays a floor or the solver hangs.
- **`an-strong-weak-update` / `an-entity-uniqueness` hand-off.** The structured key is precisely what lets a
  transition overwrite `#active` without touching `#enabled`. Strong (overwrite) vs weak (accumulate) is gated
  by uniqueness; the *identity* half (which cell) is welded-undecidable (SF-1) — **declared, never inferred**;
  the probe confirms a cell's *state*, not its identity (`17N` F3 / fw-1).
- **`an-cross-oracle-coherence` is a contract, not enforced** (`inc-9`). Two oracles grounding one kind must
  *agree* on what its name/states MEAN (the Seam). Dorc can't enforce it (never rejects plain sh) ⇒ best-effort
  CI lint, never a checked property. The handle is a **lifted datum** (reverse-DNS string, `175` C2), never a
  function name — a 1-place sh function-name cannot carry the 3-place relation (it clobbers; `an-oob-config-redline`
  X3 / F4).

## The stdlib-oracle quality bar — `17O` regression class (NOT engine holes)

"Blessing" is **not a separate magic mechanism — it is a stdlib oracle shipped day-1** (human disposition,
`17O`). So *ship good ones*: these are kept as regression tests against "good, battle-tested sh," not as gaps in
the engine. `fixtures/package.oracle.sh` is the model (captures the tool's own rc; `case` over `${Status}`, not
a pipe-into-grep) — but note it does **not** yet do the executable-file check below, so it is not a finished
exemplar of R2-SHADOW.

- **`R2-SHADOW`** — `command -v X` must confirm X resolves to an executable *file*, not a function/alias/builtin.
  Verified live: `docker(){ :; }; command -v docker` → rc 0, no binary — and Dorc's *own* function-helper /
  sourced-oracle idiom is exactly what shadows it. Fails **unsafe** (reports installed ⇒ elide the install ⇒
  priority-1 under-execute).
- **`R2-IDCACHE`** — group membership via `getent group … | <field 4>`, never `id -nG` (`id` reads a
  stale resolver cache — nscd/sssd; member-removed-but-cache-warm ⇒ wrong elision, on the `17N §8` flagship
  example). Also forbid same-session re-probe of a mutated cell (TOCTOU).
- **`R2-ORTRUE`** — the lifter must **refuse to treat an errexit-masked rc as a probe verdict**. `svc_up(){
  systemctl is-active --quiet "$1" || true; }` always reports holds (rc forced to 0 — verified). The admin wrote
  `|| true` for apply-time errexit-survival; the lift-as-probe contract assumes rc means something. A lifted
  guard's rc is a verdict only if the analyzer can prove it isn't masked (`|| true` / `|| :` / `; true`).
- **Per-database hermeticity (`F-GETENT-HOSTS`).** `getent passwd`/`group` are file-backed (fine); `getent
  hosts`/`ahosts` route through nsswitch → live DNS = non-hermetic, a buried network side-channel behind a
  "read-only" probe. **Read-only ≠ hermetic** — disqualifies it from licensing an elision (`kVOLATILES`).
- **The ≥enum floor (`F-BLESSED`).** Blessing is *not* "free read-off-the-command": an honest `service` probe is
  **two** commands (`is-enabled` *and* `is-active` — discharging `enable --now` needs both), and some kinds
  (group-membership) have no single clean blessed probe. Below the floor ⇒ over-correlation ⇒ under-execute.
  **Now enforced structurally (task-P/find-1):** a probe is declared per `(kind, selector)`
  (`oracle_probe_<kind>_<selector>`); a multi-selector kind shipping only the kind-default `oracle_probe_<kind>`
  is UN-PROBEABLE (its sites run). So `service` MUST ship `oracle_probe_service_enabled` (is-enabled) AND
  `oracle_probe_service_active` (is-active) to elide either — a single is-active body can no longer silently
  discharge `#enabled` (`KindIndex::resolve_probe`).
- **`R2-MULTIOP` — the single-operand guard.** A check binding one operand (`pkg : package = "$1"`) MUST gate its
  probe on there being NO second operand: `if [ "$2" = "" ]; then probe "$pkg"; fi`. WITHOUT it, a multi-target
  command (`apt-get install nginx curl`) resolves to entity=`nginx` ALONE and ships a probe for nginx only — so
  a host with nginx-but-not-curl elides the whole install and **never installs curl** (a priority-1 under-execute;
  20I §3 / 208 strain-W3, pinned in `tests/check.rs::naive_oracle_without_operand_guard_drops_trailing_operands_known_hazard`).
  The engine cannot supply this (it parses nothing — `inv-referent-agnostic`); the guard is the oracle's job, and
  the `[ "$2" = "" ]` form degrades the multi-operand argv to ⊤ ⇒ run (the safe direction).

## Honor (cite the slug when you rely on one)

- `inv-referent-agnostic` (W4) — the kind name is the cross-oracle anchor; never decode an `OpaqueToken`'s text.
- `inv-no-throw` (dn-7) — a malformed lift → a `Diagnostic`, never a panic; the consumer treats an **absent
  effect as ⊤ ⇒ run**, never a silent wrong-elision. (Already the posture in `lift`: non-literal anchor /
  missing probe / top-level mutator / malformed effect all fail-soft.)
- `inv-superposition` — this crate emits phase-/orientation-agnostic facts (`Polarity`/the future typestate
  transition); it must **never** bake a phase or fold `May`/`Must`. Collapse happens in the phased *caller*
  (`prove_replaceable` etc.), not here.
- `inv-must-may` — a `Grade::May` (mined/distributional) never authorizes elision; only `Must` (idiom-implied or
  oracle-declared) does. Co-reference of a shared token is at most a may-grade *hint*, never a link (`17N` F3).
- `inv-top-reject` — an oracle file is declarations only; a top-level mutator or unmodeled construct is
  ⊤-rejected loudly (`NON_DECL_CONSTRUCT` / `TOP_LEVEL_MUTATOR` in `lift_one`).

## A tension to surface, not resolve

`ch-shape-anno` (inline annotation) and `inv-no-throw`/the off-ramp weld pull against each other in *this*
crate. The lifter must stay total and fail-soft on malformed input (`inv-no-throw`); but the inline annotation
form is *itself* malformed sh under stock shells (`17O` F-OFFRAMP), so the parser-massaging that lets it through
(`ch-shape-anno`) is exactly the thing that erases the "is this even well-formed?" signal a fail-soft lifter
would otherwise surface. ~SUSPECT this collides with `F-REJECT`'s trust-spectrum: once a user writes an
annotation they are Doing A Dorc-Specific Thing, so the lifter *may* reject on ill-formedness (and ~SUSPECT on
typing-conflicts) — but that reject-power lives behind a `kTYANNOT` knob that is **unsettled at the design level**
(inline-ergonomic-but-off-ramp-hostile ↔ eol-comment-clean-but-DX-vomit). For the spike: bake the inline shape,
push it, and **record where the lift's malformed-input handling and the annotation-massaging fight** — that
friction is `notes/19x` deliverable, not a thing to design away here (`ch-wrong`).
