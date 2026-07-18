# 27M — block-context lane-context-entry (né `24S` W2) landing + residue

AI-authored (Opus builder, r27 lane-context-entry session, 2026-07-17). Records what landed for
`270:block-context`'s SECOND lane (`27J` §2.2), the PRIMARY entry lane of `plans/27C`. Authority:
root docs + `spike/CLAUDE.md` rulings + `271`/`273`/`27C`/`27J`/`27K`/`27D` outrank this. Companions:
`27K` (the peel model this builds on), `27C` (the whole spec, consumed + freshly amended mid-run),
`27J` §2.2 (the lane brief).

## Branch / base

- Branch `ai/r27-context-entry`, based on `ai/spike3-r27` @ **`2e3ded8`** (the brief's stated base;
  verified at step-zero). Six granular `(AI …)` commits (below). NOT folded — the conductor folds.
- Two mid-run human REDIRECTS + one WELD landed as chat messages and were built to (below); the
  revised `27C` §3 text "lands in the corpus with the human's next commit" — this note is built to
  the RULED CELLS, which outrank the pre-revision `27C` §3 text still in the worktree copy.

## Scope FENCE honored (hard)

PRIMARY lane only (`27C` §0.1): entry, dial, vouch, segments, degrade, disclosure, in-context
guards machinery, facts-born-in-context. The `27C` §0.2 FALLBACK lane (pure-predicate carry;
read-set closure; invariance-line consumption; ANY cross-context fact travel without entry) is
BUILT NONE — it is `lane-fallback-carry`, a later lane (`27D` block-close). Every cross-dimension
boundary without {entry × dial × vouch} ⇒ wall.

## Commits (oldest→newest)

1. `45c6ed0` (new API) — **FactKey context slot** + `core::escalation` (dial/capability vocab).
2. `0b98b82` (new) — **`oracle::entry`**: entry-form + tolerance-vouch + composition + consent models.
3. `1a83d5f` (new) — hostsim context/capability injection + the `VAR=x "$@"` ρ rung.
4. `0ac0415` (new) — §6 mined-idiom lints + the authority-disclosure line.
5. `245b917` (re) — **composition rework** per the HUMAN-ACKED `27C` §3 rulings (redirect #1).
6. `e607574` (new) — CLI dial/capability flags + escalation disclosure + fold-entry coherence
   (redirect #2) + five DiagCodes registered.
   (+ the e2e fixtures + this landing note commit.)

## 1. The FactKey / context-keying DECISION (report-ask #2 — the load-bearing surface)

**`FactKey` gains a fourth field `context: Context`** (`core/src/lib.rs`), default
`Context::HostDefault`. `core::Context` gained a `Wrapped(ContextKey)` variant; `ContextKey(Symbol)`
is a `Copy` newtype over an interned canonical string. Rationale + properties:

- **No collision** (non-negotiable): two facts naming the SAME `(kind, entity, selector)` in
  DIFFERENT wrapper-denoted worlds carry different `context` ⇒ they are UNEQUAL ⇒ a
  `BTreeSet<FactKey>` (the host store, the probe-results lane) holds them as distinct entries; a
  wrapped measurement can never alias/overwrite the ambient one. Pinned:
  `core::tests::wrapped_and_ambient_same_cell_never_collide`.
- **No accidental transport** (non-negotiable): `Coord::of_fact` now reads `fact.context`, and
  `compare` checks the context axis FIRST — a cross-context pair answers `Relation::Unknown` (the
  safe bottom for BOTH consumers), so a wrapped fact neither transports to nor spares an ambient
  one (`never-derive-separation`). Pinned:
  `coord::tests::compare_cross_context_is_unknown_never_transports_never_spares`.
- **rung-0 byte-stable**: every unwrapped fact (EVERY fact in the wrapper-free corpus) defaults
  `HostDefault`, a constant across a wrapper-free run ⇒ it partitions nothing; the erasability
  digest renders `HostDefault` as the empty string ⇒ byte-identical to the three-place key. The
  ~56 construction sites migrated mechanically (compiler-guided; every literal broke loudly).
- **`Copy` preserved** (load-bearing): `Context`/`ContextKey` are `Copy`+`Ord`+`Hash`, so `FactKey`
  stays `Copy` — no representation change across the 56 sites.
- **The room-tag seat** (`27L` `tc-room-tag-on-fact-vs-factkey`, RESOLVED here as owed): the room
  tag joins by the IDENTICAL additive-`Copy`-field-on-`FactKey` pattern this variant proves out
  (an additive `room: RoomKey`, default = unqualified, rung-0 stable). The seat is the demonstrated
  pattern; building the room field stays the payload-reentry lane's (kept a separate `tc-*`).

The **`ContextKey` canonical** is the folded per-dimension NORMAL FORM (see §3), NOT the chain
syntax — so batching AND fact-keying consume ONE key, and the ruling-4 order-sensitivity is exact.

## 2. Entry/segment composition SHAPES (report-ask #3)

- **`EntryForm`** (`cmd__enter`, `FnRole::Enter` + `lift_enters`): the ONE licensed seat for real
  context entry. Structural detection (`inv-referent-agnostic`): a terminal command whose LAST word
  is `"$@"` with a NON-EMPTY head (the exact complement of a transparent peel). `head` = display
  text (the disclosure line); `self_effect_span` = where the AUTHOR's vouched entry residue
  attributes (see §5). A bare `"$@"` (empty head) is a transparent peel, not an entry form.
- **Composition** (`compose_chain` over `[ChainLink]`, outermost-first): reworked per the redirect —
  see §3. Produces `ComposedContext` with `per_dimension` shifts + a `rho_tag`; `walls()` =
  ⊤ dimensions (degrade), `crossed()` = shifted dimensions (consent gates on these), `to_context()`
  mints `HostDefault` (identity chain) or `Wrapped(canonical)`.
- **Probe emission per-(host,context)**: the DESIGN is `ComposedContext::canonical()` = the batching
  key (one entered segment per composed context). The book-side probe-EMISSION wiring (shipping
  `sudo__enter pipx__predict …` composed on `271:rul-only-oracle-bytes-ship`) is NOT wired into the
  pipeline this lane — see §9 (deferred). The env-exec landmine
  (`271:thread-env-cannot-exec-functions`) meets this: `sudo -n <shell-function>` cannot run a
  function; the `274` §5 per-run PATH shim is the recorded dissolution the emission will reuse.
  Flagged `tc-entry-reentry-token` (§8).

## 3. Composition rework (redirect #1, HUMAN-ACKED `27C` §3 rulings) — what reworked

The composition is NOT a commutative agree-or-Top meet (my first cut was). Reworked:

- **`27C:rul-dimension-owned-compose-ops`**: each dimension fixes its op ONCE, engine-internal
  (`dimension_op`): user/netns = **absolute overwrite** (inner wins, caller-independent —
  `sudo -u root … sudo -u bob` ⇒ bob); fs-view = **caller-relative** (`chroot /mnt` then
  `chroot /t` ⇒ `/mnt/t`, paths nest via `join_path`). Wrapper authors emit single-step strings;
  the engine applies the op to opaque values (`inv-referent-agnostic`).
- **`27C:rul-top-absorbs-absolute-maps`**: ⊤ STICKY — no overwrite-rescue through an inner absolute
  map OR inner full lend (pinned both ways). Once ⊤, stays ⊤.
- **ρ threading (ruling 3)**: `RhoAccum` folds each link's `RhoClaim` into the normal-form key —
  a scrub (`ExactlyThese`) resets the base, overrides accrue, `Nothing` is the identity (so `nice`
  folds away). The mapped VALUES are expected ρ-resolved by the caller under the composed ρ
  (cross-link ρ-threading); an unresolvable `sudo -u "$VAR"` ⇒ ⊤-value ⇒ walls.
- **Canonical = folded NORMAL FORM, never chain syntax (ruling 4)**: nice-permutations AND
  cross-dimension permutations share ONE key; only genuine fold differences (path nesting,
  scrub-reorder `env A=1 sudo` ≠ `sudo env A=1`) key apart. Re-keyed the fact plane + batching to
  the normal form (my first cut keyed by chain order — re-keyed per ruling 4's explicit instruction).
- DST pins (ruling 6): all landed — nested permutations incl the shared-key `nice`; chroot-in-chroot
  path; ⊤-in-middle poisons through inner FULL and inner ABSOLUTE; unresolvable `sudo -u "$VAR"`
  walls.

**Was already conformant**: ⊤-sticky propagation (my meet had Top absorbing). **Required rework**:
per-dimension ops, ρ-threading, the normal-form key.

## 4. Dial × capability CONSENT-TRACE (report-ask #4, as implemented)

Two ORTHOGONAL axes (`27C:rul-two-axis-escalation-consent`, HUMAN-TYPED): `core::escalation`
(`Capability` = Root/NonRootNopasswd/Degraded; `EscalationDial` = NoEscalation/VouchedOnly/AnyProbe).
`decide_entry(has_entry_form, capability, dial, crossed, walls, tolerated)` traces the cells:

| dial \\ capability | Root | NonRootNopasswd | Degraded |
|---|---|---|---|
| **NoEscalation** | DialForbids | DialForbids (or NoCapability if crossed unreachable) | NoCapability |
| **VouchedOnly** (default) | Enter iff every crossed dim vouched, else Unvouched | user-dim: same; fs/netns crossed: NoCapability | NoCapability |
| **AnyProbe** | Enter (vouch overridden) | user Enter; fs/netns NoCapability | NoCapability |

Capability bounds BEFORE the dial (a shift the connection can't effect walls regardless);
`has_entry_form=false` ⇒ NoEntryForm; a ⊤ (walled) dimension ⇒ TopDimension (checked first).
`capability_permits(cap, dim)`: Root=all; NonRootNopasswd=user only; Degraded=none. All cells
pinned (`entry::tests`). CLI flags: `--no-probe-escalation` / `--probe-escalation` (default) /
`--escalate-any-probe` + `--probe-capability=root|nopasswd|degraded` (the host-fact stand-in;
hostsim-injected in DST; the probe NEVER self-acquires).

The **`tolerates:` vouch** (`ToleranceVouch`, `lift_tolerance` over the `is_converged` body):
per-function, per-dimension, reachability-scoped (top-level unconditional; case-arm scoped per-verb
via `tolerated_on_path(verb)`); brace-alternation `tolerates:{user,fs-view}`; loud on an unknown
dimension. NB the PARSEABLE spelling is the colon-line `:   : tolerates:<dim>` (no-op `:` + a
`: target` mark, exactly like the corpus `:   : invariant:user`), NOT the `27C` §2 STRAWMAN
shorthand `: tolerates:<dim>` — recorded so the stdlib brief spells it parseably.

## 5. Entry self-effects carve (report-ask #5) — WELDED, first-class

Per the mid-run WELD (`27C:rul-probe-mutation-ownership-split`, HUMAN-TYPED): the carve is no longer
excisable and is NOT an engine-decided acceptable-effect class. Entry self-effects (sudo's auth-log
line, timestamp refresh) are the entry-form AUTHOR's VOUCHED residue (`authoring-is-vouching`),
attributed to the entry command's line. Landed as `EntryForm::self_effect_span` — the attribution
point, the same authored-claim chain as every other vouch. The ENGINE's side (choosing to escalate,
the dial, entry selection, the disclosure line) is the owned/judgment tier: no hard-line machinery,
disclosed via `emit_escalation_policy`.

## 6. Degrade ladder + lints + disclosure + coherence

- **Degrade ladder** (`EntryDegrade`): NoCapability(dim) · DialForbids · Unvouched(dim) ·
  TopDimension(dim) · NoEntryForm · RuntimeEntryFailure. Every rung ⇒ can't-say ⇒ guard/run; the
  runtime rungs (entry refused / impossible / rc 127 / decline) land through the record rc-partition
  (0=holds, 1=absent, else=cant-tell⇒Unknown⇒run) — named for the disclosure only. Each pinned.
- **Fold-entry coherence** (`27C:rul-fold-entry-coherence-failfast`, redirect #2 — narrowed):
  `check_entry_coherence` fires ONLY on STATIC sh-structure — an argparsing entry form must consume
  the same leading args the lend-fold did; a trivial re-pass (`sudo -n "$@"`, 0 shifts) delegates to
  the real tool and NEVER false-fails; control-flow bodies conservatively skip. NO
  semantic-effectiveness check (tool-semantics, the traversal vouch owns it — a wrong one is the
  attributed `hole-bad-oracle-blast`). Wired into the CLI pre-network fail-fast (dual-peel pattern,
  third instance): `wrapper-entry-incoherent`, `EXIT_WRAPPER_INCOHERENT=11`.
- **§6 mined-idiom lints** (recognize-never-license): `reads_identity` (id/whoami, $USER/$HOME/
  $LOGNAME); corroboration BOTH directions — `corroborate_tolerance_over_identity` (a tolerance
  mark over visible identity-dependence ⇒ "are you sure?" Warning) and `hint_heavy_context_no_vouch`
  (heavy context-handling with no mark ⇒ the one-line adoption hint Note). `adoption_hint` suggests
  the parseable spelling.
- **Authority disclosure** (`27C:render-authority-disclosure`): `authority_disclosure` (per-context
  render) + `emit_escalation_policy` (the CLI's consent-legibility line naming the posture ×
  capability × entry-capable wrappers loaded). SCOPE: the POLICY in effect, not a per-book-SITE
  tally (the site tally needs the deferred book-side emission).

## 7. hostsim + rung-0

- **Context-qualified verdict injection** falls out of `FactKey.context` (a `Context::Wrapped` fact
  is a distinct key answered independently — the babby-sudo two-world pin
  `context_qualified_verdict_injection_answers_worlds_independently`). **Capability-cell injection**:
  `Host::with_capability`/`capability()` (root/NOPASSWD/degraded).
- **Rung-0**: the wrapper-free corpus stays BYTE-STABLE (82 pre-existing e2e cases unchanged); 857
  unit tests green; two NEW e2e cases (below).

## 8. tc-\* flags carried forward (NEVER resolved here)

- **`tc-entry-reentry-token`**: the entry composition meets the env-exec landmine
  (`271:thread-env-cannot-exec-functions`) — `<entry> <shell-function>` can't run a function; the
  `274` §5 per-run PATH shim is the recorded dissolution the book-side emission will reuse. Flagged
  UP (my entry composition's own answer to the landmine is the shim, but the emission that needs it
  is deferred).
- **`tc-rho-nothing-as-key-identity`**: the ρ normal-form key treats `RhoClaim::Nothing` (bare
  `"$@"`) as identity (so `nice` folds away, ruling 4). A wrapper that genuinely transforms env but
  claims `Nothing` (e.g. `env -u FOO "$@"` ⇒ unrecognized ⇒ Nothing) would key as env-identity — a
  potential under-distinction in the DEFERRED ρ-value-flow territory. The load-bearing dimensions
  (user/fs-view/netns) always key distinctly; flagged for the ρ-value-flow lane.
- **`tc-context-slot-on-coord-not-factkey`** (from `27G`/`27K`): RESOLVED — FactKey now carries
  context (see §1). Reported resolved, not carried.
- `inv-superposition`: nothing needed flagging UP (the entry models are phase-agnostic data).

## 9. Modeling gaps / churn-avoidance disclosures (`ru-26`) — the DEFERRED integration

- **The book-side entry-composed-probe-elision is NOT wired into the pipeline.** Consistent with
  `27K`/`27L`'s models-first pattern (both landed "models-only, zero new trust"): a wrapped book site
  still WALLS (byte-stable). What landed is the full ENGINE MODEL (entry-form, composition, consent,
  degrade, coherence, facts-born-in-context keying, hostsim injection) + the CLI ADMIN SURFACE (dial,
  capability, disclosure, fail-fast). What is DEFERRED: peeling a wrapped BOOK site in classify/value,
  emitting the entry-composed probe (`sudo__enter pipx__predict …` via the per-run shim), threading
  the composed `Context` into the shipped `FactKey`, and reading back the context-qualified verdict to
  elide. This is the big cross-pipeline integration (classify + value + `compile_probe` + the CLI
  round-trip + hostsim), the natural next dispatch. The `27C` §8 babby-sudo "vouched ⇒ enters ⇒ correct
  per-context answers" story is thus demonstrated at the MODEL + hostsim tier, not yet end-to-end e2e.
- The `VAR=x "$@"` ρ rung is modeled only for the unambiguous top-level case (assigns-then-bare-peel);
  the parser fold (prefix-vs-statement) stays deferred (`27K` gap; corpus-churn risk).
- The fold-entry coherence covers the linear shift-count case; flag-strip `while`-loop forms skip
  conservatively (a narrow-scope disclosure).

## 10. e2e tally (report-ask #7 — CONFIRMED at handback)

Verbatim, from the builder's final foreground run: `all 84 e2e round-trips passed
(ap-2 dash -n + apply/probe exec gates, redirect sandbox, ordered run-set, stderr
floor, argv-echo differential, dual-rail license judge, why-lens emission)`. 82
pre-existing cases BYTE-STABLE; 857 unit tests, 0 fail; fmt/clippy/deny/typos clean.

Two new cases (corpus idiom, inert mocks under `PATH=mocks-only`):

- `wrapper-entry-form-coheres` — a sudo oracle authoring predict + lend_map + the `__enter` form;
  all lift clean and cohere (the trivial re-pass entry delegates ⇒ no fold-entry divergence); the
  escalation-policy disclosure names the entry-capable wrapper; the wrapped site STILL walls
  (byte-stable, zero new trust).
- `wrapper-entry-incoherent-fails-fast` — a wrapper whose `__enter` shifts 2 leading args while its
  `__lend_map` fold consumed 1 ⇒ static incoherence ⇒ `DORC_EXIT=11` + `wrapper-entry-incoherent`
  (declared in `expected-diagnostics`); the artifact still ships.

## 11. Redirects acknowledged (report-ask)

- **WELD** (`27C:rul-probe-mutation-ownership-split`): built first-class, attribution wired to the
  entry-form's line (§5). No rework of prior code — the `self_effect_span` was authored to this from
  the start.
- **Redirect #1** (composition rulings): REQUIRED REWORK — the per-dimension ops, ρ-threading, and
  normal-form key (commit `245b917`). ⊤-sticky was already conformant.
- **Redirect #2** (`27C:rul-fold-entry-coherence-failfast`): built fresh to the narrow static scope
  (two triggers, no semantic-effectiveness); nothing to narrow (I had not built the proposed version).
