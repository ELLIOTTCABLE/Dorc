# 26G — adjudication of the four r26 analyzer findings

Input: the hermetic r26 smoke-kit validation (`Research/trial/r26/predictions.md` §4,
`README.md` §4), which reported four engine behaviours. This note re-derives each from the
code, adjudicates it against settled law, and sketches a fix for a kernel planner. Findings
are renumbered here as `fnd-*` slugs; the trial's own labels (`m-2`, `m-3`, README §4 items)
are cross-referenced.

**Remit note.** No engine code was changed. Repro instructions below are prose + file:line;
nothing was committed as a test. All repro runs were read-only analyzer invocations
(`dorc probe` / `plan` / `why` / `lint` / `strip`) — no book, mock, or rendered artifact was
executed, and the frozen kit in `Research/trial/r26/` was not modified (scratch inputs were
written outside the repo).

**Verification status up front.** `fnd-shared-auto-cell-collides`, `fnd-dead-branch-still-invalidates`,
and `fnd-existence-gate-darkens-oracle` are verified to a named code path and reproduced.
`fnd-three-constructs-void-marks` is reproduced behaviourally but its parse-side cause is
**not** pinned — see its §1. Two of the trial's own characterisations turn out to be wrong in
ways that matter; those are called out inline.

---

## fnd-shared-auto-cell-collides — same-command sites share one synthesized cell

Trial label: `predictions.md` §4 m-2.

### 1. Verified root cause

Three separable steps, all confirmed:

**(a) Verdict-body coordinate marks are never read for site keying.** A book site's established
cell comes from `idx.effect_of(provider, verb_key)` — `spike/crates/analysis/src/effect.rs:365` —
which is built from the oracle's `cmd__predict()` marks. The `cmd__is_converged()` body is not
consulted for the coordinate at all. Two fallbacks route to the synthesized cell:
`effect.rs:349-356` (no predict resolved for this provider) and `effect.rs:366-374` (a predict
resolved but the effect-map has no cells for that `(provider, verb)`). Both call
`auto_or_opaque` (`effect.rs:235-248`), which mints `dorc_core::auto_fact` when the provider
bears a verdict function, else `Opaque`.

**(b) The synthesized cell is a per-provider singleton by construction.**
`spike/crates/core/src/lib.rs:677-684` — kind `dorc-auto:<provider>`, entity
`EntityRef::Singleton` always, selector the fixed `converged`. The doc comment at
`core/src/lib.rs:674-675` states the consequence outright: "All of one command's sites share
this one cell (the singleton coarseness §3 prices; more same-cell staleness ⇒ more forced runs,
never fewer)."

**(c) Per-site records are then folded into a fact-keyed map — the meet site.**
`spike/crates/cli/src/main.rs:5656-5761` (`facts_from_sites`). Records are *read* per site,
honouring the site-keyed lane: `RecordKey { site, member }` at `main.rs:5676-5679`. They are
then inserted into `by_fact: BTreeMap<FactKey, Observable>` (`main.rs:5664`) and, on a second
site hitting the same key, merged: `by_fact.insert(check.fact, merge_observable(prior, obs))`
at `main.rs:5754`, under the comment "a CROSS-site conflict: two sites on one cell disagree ⇒
the meet ⊤s the channel." The per-site distinction the records lane preserved is destroyed here.
A second fact-keyed map, `probe_origins` (`main.rs:5779-5784`), is keyed the same way — its own
test is named `probe_origins_keys_measured_receipt_by_fact_with_stream_ordinal` (`main.rs:8233`),
so the fact-keying is deliberate and pinned, not accidental.

**Reproduced.** From `Research/trial/r26/`, `dorc probe --book=smoke-book.sh --oracle-dir oracles`
prints its site coordinates; sites 10 and 11 both read `dorc-auto:cp@converged` and sites 12 and
13 both read `dorc-auto:systemctl@converged`, while sites 2/4/6/8 read distinct
`r26.smoke.PkgState:<pkg>@installed`. That contrast is the whole finding in one output: `dpkg`
is the only kit oracle with a `__predict` body, and it is the only one whose sites get distinct
per-entity cells. `cp` and `systemctl` ship verdict bodies only — each carrying a full authored
coordinate mark that is silently unused for keying.

**Two corrections to the trial's account.**

- The trial calls this "untested territory — no fixture in the corpus has two `is_converged`
  sites of the same command." That is **false**. `spike/crates/cli/tests/context-entry-babby-elides/`
  and `.../context-entry-babby-diverges/` each have two `hork` sites, both keyed
  `dorc-auto:hork@converged` (see their `expected.out`), and `-diverges` runs exactly the
  conflicting-record case (`probe-results.txt`: site 0 `holds`, site 1 `absent`). A fix will
  re-bless these; they are the closest existing pins.
- The trial's generality evidence — "the `kp` oracle carries a full coordinate mark and still
  keys `dorc-auto:kp@converged`" — is a **misread**. The mark in
  `cli/tests/carry-fsview-elides/kp.oracle.sh` is a read-set declaration inside the verdict
  (`: sm.dorc.KernelParam:"$1"`, the fixture's own comment says "verdict reads ONLY the marked
  KernelParam cell"), and that oracle has no predict body either. It is another instance of the
  same cause, not independent evidence that authored *effect* coordinates are ignored.

**One amplification the trial missed, and it is the practically important one.** The collapse
does not need a *conflicting* record. Three read-only runs against the kit (records framed with
the kit's own `frame-records.sh`, which defaults any unlisted site to `effect=cant-tell`):

| records fed | cp guard? |
|---|---|
| sites 10 **and** 11 both `holds` (the committed converged world) | one guard (site 11) |
| site 10 `holds` only (11 defaults to `cant-tell`) | **guard=0** |
| site 11 `holds` only (10 defaults to `cant-tell`) | **guard=0** |
| site 10 `holds`, site 11 `absent` | **guard=0** (systemctl still guards) |

Because `cant-tell` is the default for any site the probe could not answer, a shared cell is
poisoned by a sibling that merely failed to report — not only by one that disagreed. In a real
book that drops many files with `cp`, one unreadable destination de-licenses every other `cp`
in the book.

**Where the trail goes cold.** In the all-`holds` case, both sites' shared cell reads `holds`
(the merge is a no-op when `prior == obs`), both sites hold a site-keyed vouch
(`plan/src/lib.rs:1326-1347` keys `Vouches` by `CfgNodeId`) — yet only the *later* of each pair
guards. `dorc why smoke-book.sh:34` answers "ran RATHER THAN guarded: nothing licensed removing
this line: it mutates with no converged report", i.e. the engine believes the earlier site has
no report at all. I did not isolate which lookup drops it. My leading hypothesis is that the
earlier site's establish fails the survival tier because the later same-provider site
may-aliases the shared auto-kind (`fence-no-disjoint` forces `MayAlias` — `plan/src/survival.rs:1213-1218`),
so the last site of a run is the only one whose fact survives; a competing hypothesis is a
plain last-write-wins in a fact-keyed lookup. **Unverified past this point.** It does not change
the fix direction (both are cured by distinct keys), but a planner should not quote a mechanism
for the tie-break.

### 2. Law verdict

**Defect-vs-settled-law**, on the keying; the *coarseness* itself is priced.

Decisive sentences:

- `oracle-contract` §4 (`spike/docs/reference/oracle-contract.md:197-198`): "Verdict and observe
  marks mint selector tokens into the kind's vocabulary, and **attach facts to the one line that
  measured them**." The `cp` oracle writes `: r26.smoke.File:"$dst"@content` in its verdict and
  the engine attaches the fact to a provider-wide singleton instead of to that line.
- `oracle-contract` §5a (`:286-291`): the verdict's licence is "at this tool's own sites only …
  The vouch is inadmissible everywhere else: it **never becomes a fact**, never informs another
  site's reasoning, never transfers to another tool." The engine implements the own-site licence
  *as* a fact (`CommandEffect::Establishes(auto_fact(…))`) in a cell shared across sites, so one
  site's measurement demonstrably informs another's disposition — the table above is that
  influence.
- `spike/CLAUDE.md` `inv-site-keyed-results` (`:415-418`): "the probe-results lane keys by
  command-site … never by fact / kind:entity / command-family: **two same-command sites must not
  collapse**. (Fact-keyed verdict shapes are a conscious orchestrator+human decision, not a local
  refactor — kSTATE-coupled.)"

The honest reading of that last one: the *records lane* obeys it (`RecordKey { site, member }`);
the derived *fact* lane does not, and `facts_from_sites` is precisely the fact-keyed verdict
shape the parenthetical reserves to an orchestrator+human decision. So the collapse is not a
rogue local regression — it is the reserved decision, taken. Whether it was taken *knowingly*
for the auto-cell is the part a human should rule on.

Against `24L` §3, the singleton coarseness is explicitly priced with a safety argument ("more
forced runs, never fewer"). The direction claim survives — no experiment produced a wrong *yes* —
so this is a **precision** defect, not a soundness one. Read `24L`'s pricing as covering the
markless founding one-liner it was written for; it does not obviously extend to a verdict body
that *did* author a coordinate, which is the case the contract's §4 sentence governs.

### 3. Fix-direction sketch

Key the establish on the verdict body's authored coordinate when it has one, falling back to
the auto-cell only for genuinely markless verdict bodies (the shape `24L` §2 actually describes).
Concretely: the verdict lift already resolves marks per site — `dorc_oracle::verdict`, consumed
by `build_vouches` (`plan/src/lib.rs:1299`) with the site's constant-propagated argv in hand —
so the coordinate exists at a point where the site identity is still known. The work is to make
`analysis::effect` able to see it: today `command_effect` takes `checks: &[PredictSet]`
(`effect.rs:258`) and no verdict set at all, so the classification pass is structurally blind to
verdict marks. That signature, and the `auto_or_opaque` decision it guards, is the seam.

Crates/passes touched: `analysis` (`effect.rs` classification + the `auto_or_opaque` seam),
`oracle` (expose verdict-mark resolution on the same footing as `predict::evaluate`), `cli`
(`facts_from_sites`, whose fact-keyed merge stops being lossy once keys are distinct), `plan`
(`survival`'s `add_auto_kind` registration shrinks to the residual markless cases).

Invariants brushed: `inv-site-keyed-results` (the point — restores it fact-ward),
`identity-declared-never-inferred` (the coordinate must come from the oracle's own marks and
binds; the engine must not parse argv to synthesize an entity), `inv-referent-agnostic` (the new
key is a `KindId` + operand token, never decoded), `fence-unnameable` and `fence-no-entity`
(both are properties of the auto-cell and must survive for the markless path),
`fence-no-disjoint` (`survival.rs:1213-1218`) — an authored coordinate is no longer force-
may-aliased, so sites gain the ability to prove separation they never had.

That last one is the naive-fix hazard: **the auto-kind is currently registered as
always-may-alias precisely so it can never manufacture separation** (`277` §6
never-derive-separation). Promoting verdict-marked sites to real coordinates hands them
`disjoint`'s different-kind `continue`, which is a *licence-granting* path. A fix that only
changes the key, without re-deriving what the authored coordinate is allowed to prove about
separation, converts a precision bug into a soundness bug — wrong yeses, the one direction
`24L` §3's pricing argument currently guarantees against. Any patch that makes more things
elide should be assumed wrong until the separation story is argued independently.

Second naive-fix hazard: a verdict body may carry several marks (a verdict plus observes).
§4 (`:199-203`) allows at most one verdict per line but observes are unrestricted; picking the
wrong mark as the key silently re-points the cell. The selection rule must be "the verdict mark
(`asserts`/`refutes`), never an observe", and a body with no verdict mark stays on the auto-cell.

### 4. Untested territory a fix must pin

- Two same-command sites with **distinct** authored entities: distinct cells, both licensable
  independently. No fixture covers this (the two `hork` sites resolve to one cell today).
- Two same-command sites with the **same** authored entity (`cp a /etc/x` twice): must still
  share a cell, and the merge must still ⊤ on disagreement — the collapse is correct here.
- The sibling-`cant-tell` case from the table above: today it silently de-licenses; after a fix
  it must not, and if a genuine shared cell is involved it must de-license *with a diagnostic*.
- A verdict body with a mark whose value-position does not resolve (`"$3"` on a 2-argv site) —
  must fall back to the auto-cell, not to a garbage key.
- Markless verdict-only oracles keep today's auto-cell behaviour exactly (`24L` §2 regression pin).
- Re-bless `context-entry-babby-elides` / `-diverges` and confirm the diverging pair's
  dispositions become per-site rather than order-dependent.

### 5. Size/risk

**Days, cross-cutting.** Four crates, and it reopens the separation question that
`fence-no-disjoint` currently closes by brute force. Confidence in the root cause: **+SURE**
(directly reproduced, three code sites named). Confidence in the fix shape: **~SUSPECT** — the
keying change is mechanical, the separation consequence is a genuine design question, not an
implementation detail, and I would expect it to need a human ruling before code.

---

## fnd-dead-branch-still-invalidates — the guarded-install ladder caps at one

Trial label: `predictions.md` §4 m-3.

### 1. Verified root cause

The trial's framing — "each line's `||`-right mutator site apparently casts a downstream wall
even where the fold proves that branch dead" — is **half right, and the wrong half is the
actionable one**. The wall law is honoured. What is not honoured is a *second, separate*
mechanism: query-site validity.

`SkipClass::QueryResolvable { fact, valid }` (`analysis/src/effect.rs:759`) carries the bit that
decides whether a guard's probed rc may feed the fold. Its contract, `effect.rs:751-758`:

> `valid` is the rule-query-validity bit (205 §2 / 20A §4 st-3): the guard's probe-time rc is
> fold-usable IFF **NO invalidating command reaches the guard from entry** — invalidating = an
> oracled MUTATOR (any Establish/Kill) or an **Opaque**; NOT invalidating = other Queries or
> blessed-pure builtins.

It is computed at `effect.rs:1441-1446` as `valid: reach.states[i].is_pristine()` — a purely
static, records-blind reachability property, in the `analysis` crate. The fold that proves a
`||`-RHS dead lives in `plan` and needs the probe records, which `analysis` never sees. The
passes run analysis→plan once, and never iterate. So an Opaque mutator on line N statically
"reaches" the guard on line N+1 and invalidates it, regardless of the fact that line N's mutator
is later proven dead and omitted.

`facts_from_sites` is where the invalid bit is cashed: `main.rs:5684-5694` — a
`Query { valid: true }` yields `Predicted::Value(r.rc)`, while `Query { valid: false }` (and any
Establish) yields `Predicted::Top`, "withhold the rc".

**Reproduced, and the discriminating experiment matters.** A three-line ladder
(`wombat query X >/dev/null 2>&1 || <mutator> X`, three distinct entities, all three queries
recorded `holds`), planned read-only against a one-oracle dir:

- when the RHS mutator is **modeled** (a `wombat sync` covered by the same oracle):
  `sites=7 elide=3 omit=3 guard=0 run=1` — **all three lines fold**. The cap does not exist.
- when the RHS mutator is **unmodeled** (`hork sync`, no oracle):
  `sites=7 elide=1 omit=1 guard=0 run=5`, with the engine's own hint reading
  "'hork' (line 4) is unmodeled: it is the first wall".

Note what that hint says: line 3's `hork` is *not* the wall — it was omitted, and an omitted
command correctly casts none. Line 4's `hork` is, even though line 4's own query reported
`holds` and so line 4's `hork` is equally dead. The first line folds because nothing precedes
it; every later line's query was invalidated by the preceding line's Opaque before any records
were consulted.

So the ladder cap is **not** about guarded installs, and not about walls. It is: *one static
Opaque anywhere above a guard withholds that guard's rc, and the fold never re-runs to notice
the Opaque was dead.* One more fixpoint iteration would fold line 4 (line 3's mutator is now
omitted ⇒ line 4's query becomes pristine ⇒ line 4 folds ⇒ …), cascading to the whole ladder —
which is exactly the shape of the all-modeled run above. `Research/notes/26C-fixpoint-semantics-audit-and-revival-plan.md`
is the obviously-relevant prior art and a planner should start there.

This also re-explains the r26 kit's own numbers without appeal to the guarded-install idiom: the
kit's `apt-get` oracle *deliberately declines* `install`, so every ladder line's RHS is Opaque.
The kit measured the Opaque-above-a-guard cap and attributed it to the idiom.

### 2. Law verdict

**Defect-vs-settled-law.**

`USER_STORY.md:299-303`, the decisive passage:

> Because **an elided command casts no wall**: a command that will not run cannot invalidate
> anything, so the wall at line 8 simply is not there on a converged day. The two-minute oracle
> didn't just buy its own line; it bought back **every downstream fact it had been poisoning**.

The engine honours the literal sentence (an omitted command casts no wall — verified above) and
violates the promise the paragraph exists to make. "A command that will not run cannot
invalidate anything" is exactly the rule query-validity breaks: a proven-dead Opaque still
invalidates every guard below it. The stage-3 narrative in `USER_STORY` is the headline user
story for the whole product, and the ladder is the idiom it leans on.

Two mechanisms implement "poisoning" — the wall predicate and the query-validity bit — and the
law was applied to one of them.

### 3. Fix-direction sketch

The shape is a fixpoint: iterate classify→fold until dispositions stop changing, with omitted
sites removed from the invalidating set on each pass. Termination is easy to argue — the omitted
set only grows and is bounded by the site count — and the iteration count is trivially bounded
by ladder depth. A cheaper, less general alternative is a single backward pass that pre-computes
"this Opaque is dead under the current records" before validity is computed, but that re-derives
the fold in `analysis` and duplicates the deadness logic, which is worse.

The real obstacle is the crate split, not the algorithm: validity is computed in `analysis`
(`effect.rs:1441-1446`) which is deliberately records-blind, while deadness is known only in
`plan`/`cli` where records live. A fixpoint means either threading a records-derived
"known-dead sites" set *into* the classification pass, or hoisting validity computation out of
`analysis` into the phased caller. The second respects `inv-superposition` better — validity is
currently documented as "a phase-agnostic fact; the collapse stays in the caller"
(`effect.rs:756-758`), and making it depend on records makes it phase-dependent, which is a
direct tension with that invariant and needs an explicit ruling.

Invariants brushed: `inv-superposition` (above — the sharpest one), `inv-top-reject` (the
iteration must never *shrink* a ⊤-trigger by accident; a site is only removed from the
invalidating set on a positive proof of deadness, never on absence of evidence),
`inv-determinism` (fixpoint iteration order must be deterministic), `toctou-scope`.

Naive-fix hazard: treating "omitted" and "dead" as interchangeable. A site can be omitted for
reasons other than a records-proven dead branch, and any such path feeding the invalidating-set
removal is a soundness hole — it would license folding a guard whose cell really was mutated.
The removal predicate must be the records-proven-dead one specifically, not the disposition.

### 4. Untested territory a fix must pin

- The N-line ladder with all guards `holds` and an unmodeled RHS: today 1 folds, after a fix N
  should. No fixture has N>1 of this shape.
- The same ladder with the *middle* guard reporting `absent`: lines above it must still fold,
  it and everything below must not — the cascade must stop at the right line.
- A ladder whose RHS is modeled (today's passing case) must be unchanged — regression pin.
- An Opaque that is *not* in a dead branch must still invalidate everything below it, unchanged.
  This is the case that keeps the fix honest and it is the one most likely to break.
- Iteration determinism: same inputs, same fixpoint, byte-identical render.
- A mutual/cyclic shape that could iterate forever, to pin termination.

### 5. Size/risk

**Days, cross-cutting**, and architecturally the heaviest of the four — it changes pass
structure, not a decision. Confidence in the root cause: **+SURE** (the modeled-vs-unmodeled RHS
experiment discriminates cleanly and the code comment states the rule verbatim). Confidence that
a fixpoint is the right answer: **~SUSPECT** — it is the obvious shape and `26C` already
contemplates it, but the `inv-superposition` tension is real and may make a human prefer a
narrower rule.

---

## fnd-three-constructs-void-marks — marks go inert with no dorc diagnostic

Trial label: `README.md` §4 item 1.

### 1. Verified root cause — behaviour reproduced, cause NOT pinned

All three constructs reproduce. Method: a minimal oracle with a bind and one verdict mark, run
through `dorc strip`; if the kind token survives the strip, the marks were never recognised.

| oracle body | marks survive strip? | `dorc lint` |
|---|---|---|
| baseline (no offending construct) | no (correct) | 0 errors, 0 warnings |
| `[ -n "$1" ] \|\| return 2` before the marks | **yes — inert** | 0 errors, **1 warning** |
| `case "$1" in -*) return 2 ;; *) ;; esac` | **yes — inert** | 0 errors, **1 warning** |
| backslash continuation whose next line opens with `>/dev/null` | **yes — inert** | 0 errors, **1 warning** |

The single warning in every failing case is
`shellcheck:SC2154 — p is referenced but not assigned`, at the mark line. That is a
**third-party** finding, not dorc's: the bind `p : t.Thing = "$1"` was not recognised as a bind,
so `p` is never assigned, and shellcheck notices the downstream reference. Dorc itself emits
**zero** diagnostics. The signal therefore disappears entirely if shellcheck is absent (the same
run reports `info [checkbashisms:lint-tool-absent]` for a missing tool, so tool absence is
normal and expected), and it would also disappear for any voided body that happens not to
reference a bound variable afterwards. The trial's "silent" characterisation is correct.

**Cause not pinned.** I did not locate the parse-side site that drops the marks. The plausible
region is `spike/crates/oracle/src/predict/parser.rs` (several bare `_ => return None` arms at
`:255`, `:1436`, `:1456`) together with `predict/mark_grammar.rs:127`, any of which would
abandon a statement without recording why — but I did not confirm which one fires for these
three inputs, and a `return None` in a parser helper is not by itself evidence of the silent
drop. **Unverified past this point.** A planner should treat the repro table as the reliable
part and re-derive the parse site; the three inputs above are small enough to step through
directly.

Worth noting for whoever does: the three constructs have no obvious shared shape (a test in
statement position, a glob in a case arm, a continuation into a redirection), which suggests
either three separate drop sites or one lexer-level line/statement-boundary confusion. That
question is the first thing to settle, because it decides whether the fix is one guard or three.

### 2. Law verdict

**Defect-vs-settled-law.**

`spike/CLAUDE.md` `inv-top-reject` (`:399-403`):

> anything unmodeled collapses to ⊤ and is **rejected loudly** (un-probeable ∧ un-elidable),
> **never silently best-effort'd**. Under-modeling is a correctness boundary, not a TODO.
> Shrinking a ⊤-trigger is a deliberate design act, never an accident; bias every parser
> ambiguity toward ⊤-reject-with-diagnostic.

Three parser ambiguities are biased the other way: the file parses, lint passes, and the
annotations are silently discarded. "Bias every parser ambiguity toward ⊤-reject-with-diagnostic"
is as close to a directly-applicable sentence as the corpus offers. The safety direction is
intact (a voided mark loses licence, never gains it — an oracle that models nothing cannot vouch
wrongly), so this is a **diagnostic** defect, not a soundness one. That is also why it is
expensive: it costs authoring time and confidence rather than correctness, and it did cost real
time in r26.

### 3. Fix-direction sketch

The general shape: the mark parser must distinguish "this statement carries no mark" from "this
statement was not understood", and the latter must produce a diagnostic at the offending
statement's span. A cheap and independently valuable backstop, whatever the parse fix: a marked
file (one carrying the `# dorc-lang/v0.2` marker) whose marks do not survive to the lifted
representation is itself a reportable condition — the `strip`-based check the r26 authors ran
by hand, promoted into `dorc lint`. That backstop is attractive because it is cause-agnostic:
it catches the next three constructs nobody has found yet.

Crates: `oracle` (the predict/mark parser and `strip.rs`), `lint` (the surfaced diagnostic).
Invariants brushed: `inv-top-reject` (the point), `kLANG` mirror/parse-permissively-trace-
conservatively (a construct may still be *accepted* byte-exact and merely refuse to resolve —
that is the existing `TopReason::Pipeline` precedent at `predict/eval.rs:125-130`, which is the
model to copy: accept, degrade, and *say so*).

Naive-fix hazard: making these three constructs hard parse errors. They are legal POSIX sh and
the dialect's stated posture is parse-permissively/trace-conservatively; rejecting them outright
would break `strip` round-tripping and, worse, push authors toward contorted sh, which cuts
against the project's "scripts must be re-usable after abandoning Dorc" commitment. The
diagnostic must be a warning that names the voided marks, not a refusal.

### 4. Untested territory a fix must pin

- Each of the three constructs, as its own case: marks either work or produce a named
  diagnostic — never inert-and-silent.
- The negative pin that makes the check meaningful: a legitimately markless oracle must **not**
  warn.
- The bare `*)` case arm, which the trial reports works fine — must stay quiet (it is the
  discriminator against a naive "any case arm" over-trigger).
- A voided-mark file with no post-bind variable reference, so shellcheck stays silent — proves
  the new diagnostic does not lean on the incidental SC2154.
- `dorc strip` round-trip on all three constructs after the fix.

### 5. Size/risk

**Hours-to-days, localized** — one or three parser sites plus a lint surface, no cross-crate
semantics, no effect on what elides. Confidence in the behaviour: **+SURE** (reproduced with a
clean baseline discriminator). Confidence in the location: **-GUESS** — deliberately not
asserted. The lint-level backstop is a **hours**, low-risk, high-value increment that can land
independently of finding the parse sites, and I would sequence it first.

---

## fnd-existence-gate-darkens-oracle — the contract's own gate silently kills the oracle

Trial label: `README.md` §4 item 2.

<!-- /* superceded IN PART by §CORRECTION-orlist-not-command-v (appended 2026-07-27): the
     MECHANISM this section names is wrong. The degrade is caused by the `||` (or-list); NOT by
     `command -v`, which models fine on its own, and NOT by "unmodeled statements" generally.
     Every OBSERVATION below reproduces; only the attribution is wrong. The knock-on is that
     `26H` §5's R-1-model-command-v is a wrongly-posed question. Read the correction first. */ -->

### 1. Verified root cause

Reproduced decisively, and the effect is total. A book `wombat query thing … || wombat sync thing`
with a `wombat__predict` body, planned read-only:

- predict body as written: `dorc probe` reports `# site 1: t.Thing:thing@present` and
  `# site 2: t.Thing:thing@present` — authored coordinates, sites resolve.
- the same body with `command -v wombat >/dev/null 2>&1 || return 2` inserted as its **first**
  statement: **every** site reports `unresolvable-no-probe`. The oracle contributes nothing.
- `dorc lint` on the gated oracle: **0 errors, 0 warnings**.

So a single leading statement silently converts a working oracle into a non-oracle.

Mechanism: the predict tracer walks the body and degrades to `Resolution::Top` on anything it
cannot follow; `command_effect` then routes `Top` to `auto_or_opaque` (`effect.rs:349-356`),
which yields `Opaque` for a provider with no verdict function — the site runs unprobed. The
degrade reasons are a closed enum, `TopReason` (`oracle/src/predict/eval.rs:106-131`), reached
at `eval.rs:428-429` (`MissingAnnotation` when no annotation is found, `NoProbeReached` when the
selected path reached no probe command).

The diagnostic gap is legible directly in that enum: **there is no variant meaning "the body
contained a statement I do not model."** The unmodeled gate is reported, internally, as one of
its *symptoms* — the walk reached no probe / found no annotation — and that reason never reaches
the author anyway. This is why the failure presents as total silence rather than as a pointed
complaint about the gate, and adding the missing variant is most of the fix.

An asymmetry worth recording, and a caution against over-generalising: the same gate in a
**verdict** body did not make the site unresolvable — it still keyed `dorc-auto:wombat@converged`.
I did not establish whether the verdict's *vouch* still fires in that case (my attempt to measure
it needed records I did not construct), so "the gate is only fatal in predict bodies" is
**unverified**; treat the predict-path result as the solid one.

The trial also reports `case $?` in a verdict body as unresolvable (`README.md` §4 item 3, the
reason the systemctl oracle leaves systemd's exit vocabulary untranslated). I did not
independently test that one; it is plausibly the same tracer-degrade path, but it is
**unverified here**.

### 2. Law verdict

**Defect-vs-settled-law**, on two counts, and this is the cleanest of the four.

The doc-vs-engine contradiction is flat and quotable. `oracle-contract` §3
(`spike/docs/reference/oracle-contract.md:103-104`):

> Route every surprise to 2+: missing binaries, unrecognized output, permission oddities.
> `command -v tool >/dev/null 2>&1 || return 2` is **the standard gate**.

The contract prescribes the idiom by name; the engine silently voids any oracle that follows it.
An author doing exactly what the reference document instructs gets a working-looking, lint-clean,
entirely inert oracle. That is the worst available failure mode for the project's authoring
story, and it is worse than the trial's framing suggests — the trial notes the gate "cannot be
used", but the consequence is not a lost gate, it is a lost *oracle*.

And `inv-top-reject` again (`spike/CLAUDE.md:399-403`): the collapse to ⊤ is real and
safety-correct, but it is silent where the invariant demands loud.

Note the two are separable and should be fixed separately: the silence is a defect under
settled law and needs no design input. Whether the engine should *model* `command -v` (making
the gate work) is a dialect-scope question — `command -v` is already the contract's own example
of a Query guard (`effect.rs:745-746` cites `command -v nginx` ⇒ `tool:nginx@present`), which
suggests it is meant to be modeled, but I found no ruling that settles it. **Ambiguous-needs-human**
on that second half.

### 3. Fix-direction sketch

Two independent increments, and they should not be conflated:

**(a) Make it loud** — settled, do it now. Add a `TopReason` variant naming an unmodeled
statement, carry the offending statement's span, and surface it from `dorc lint` and from the
probe-time `site-unresolvable` note, which today names the site but not the cause. The
`site-unresolvable` note already exists and already enumerates sites (visible in every kit
render), so this is largely a matter of giving it a reason string with a span. This alone would
have saved the r26 authoring time and needs no design ruling.

**(b) Make the gate work** — needs a ruling. Model `command -v` in oracle bodies as the
target-state-pure probe it is. `is_target_state_pure_builtin` (`effect.rs:295-297`) is the
existing precedent for "this statement cannot touch target state, so do not let it degrade the
analysis", and the natural home. Scope question for the human: whether this is a one-off for
`command -v` or the beginning of a modeled-statement set (`test`, `[`, `if <command>; then` are
the trial's other casualties, and (a)'s diagnostic will immediately start naming them).

Crates: `oracle` (`predict/eval.rs` reasons and the tracer), `analysis` (`effect.rs` if (b)),
`aid`/`lint`/`cli` (surfacing). Invariants brushed: `inv-top-reject` (the point),
`identity-declared-never-inferred` (modeling `command -v` must not become engine-side argparse of
the *delegated* tool's flags), `kFAIL-perform` / parse-permissively-trace-conservatively.

Naive-fix hazard for (b): quietly widening the modeled-statement set to "whatever the corpus
needs" is exactly the "shrinking a ⊤-trigger by accident" that `inv-top-reject` forbids. Each
addition is a deliberate design act with its own purity argument, and the diagnostic from (a)
should be allowed to accumulate evidence about which statements authors actually reach for
before any of them are modeled. Sequencing (a) strictly before (b) is the recommendation.

### 4. Untested territory a fix must pin

- The contract's literal gate as the first statement of a predict body: today the oracle goes
  dark silently; after (a) it must be named. This case does not exist in the corpus at all —
  every corpus oracle omits the gate, which is why the whole class went unnoticed.
- The same gate in a verdict body (the asymmetry I could not resolve), including whether the
  vouch still fires.
- After (b) if taken: the gate present and the tool present ⇒ identical plan to no gate;
  the gate present and the tool absent ⇒ honest decline.
- Each other unmodeled statement the trial names (`test`, `if <command>; then`, `case $?`)
  gets a named diagnostic, whether or not it is ever modeled.
- The negative pin: a genuinely unmodelable body still degrades to ⊤ and still runs.

### 5. Size/risk

Increment (a): **hours-to-days, localized**, no semantic change — purely additive diagnostics.
Increment (b): **days, cross-cutting**, and gated on a human scope ruling. Confidence in the
root cause: **+SURE** for the predict path (reproduced, total, with a clean before/after), and
the contradiction with `:103-104` is textual and not a matter of interpretation. Confidence in
the verdict-body asymmetry: **-GUESS**, flagged above as unverified.

---

## Kernel hazards for a fix-planner

**haz-two-poisoning-mechanisms-one-law.** "A command that will not run cannot invalidate
anything" (`USER_STORY:299`) is implemented twice: the wall predicate, and the query-validity
bit (`effect.rs:751-758`). The wall obeys it; validity does not. Any future law about elided
commands must be checked against *both* or it will be half-applied again. This is the single
most generalizable lesson of the four findings.

**haz-fixing-keying-changes-fold-inputs.** The findings interact, and in the dangerous
direction. `fnd-shared-auto-cell-collides` and `fnd-dead-branch-still-invalidates` are coupled:
distinct authored coordinates mean more sites resolve, which means more sites classify as
Establish rather than Opaque, which changes the *invalidating set* that query-validity is
computed from (`effect.rs:1441-1446`). Fixing the keying will therefore silently move F2's
numbers — plausibly improving them, since a modeled RHS is exactly what made my ladder fold
completely. **Corollary: do not measure either fix's benefit while the other is in flight, and
do not bless a golden that straddles them.** Land them in separate commits with separate
re-blesses, keying first (it is the input side).

**haz-auto-kind-is-load-bearing-for-safety.** The auto-kind's always-may-alias registration
(`survival.rs:1213-1218`, `add_auto_kind` from `cli/main.rs:1209-1217`) is the fence that stops
the singleton from manufacturing separation (`277` §6). It looks like an implementation detail
and is not. Any change to auto-cell keying must ask, per site, whether that site still needs the
fence — and the answer for markless verdict-only sites is still yes.

**haz-silence-is-the-common-cause.** Three of the four findings (`fnd-three-constructs-void-marks`,
`fnd-existence-gate-darkens-oracle`, and the sibling-`cant-tell` amplification under
`fnd-shared-auto-cell-collides`) are the same defect class: a safety-correct degrade that is
never announced. All three cost authoring time rather than correctness. A planner could
reasonably treat "every ⊤-degrade carries a span and a reason, and lint surfaces them" as one
cross-cutting workstream rather than three fixes — it is cheaper, it lands ahead of the two
hard semantic changes, and it would have caught all three findings during authoring instead of
during validation. **This is my recommended first move**, and it is the only recommendation here
that needs no design ruling.

**haz-safety-direction-holds-everywhere.** No finding produced a wrong *yes*. Every defect
costs elisions or costs authoring time; none licenses an unsafe skip. This is worth stating
plainly because it sets the priority: none of these is an emergency, and a rushed fix to any of
them — particularly the separation consequence in `fnd-shared-auto-cell-collides` — can easily
be *worse* than the defect, by converting a precision loss into a soundness loss. Prefer slow.

**haz-pass-order-is-analysis-then-plan-once.** `analysis` is records-blind by construction;
`plan`/`cli` hold the records. Several natural-looking fixes ("just check whether it is dead")
are impossible in the crate where the decision is currently made. Expect any real fix to either
thread records-derived facts into classification or hoist the decision out — both of which brush
`inv-superposition`'s "the kernel emits phase-agnostic facts; only the phased caller collapses
them" (`spike/CLAUDE.md:411-414`). That invariant is the one most likely to be quietly broken by
a well-meaning patch to `fnd-dead-branch-still-invalidates`.

**haz-trial-claims-need-independent-check.** Two of the trial's characterisations did not
survive re-derivation (the "no fixture has two same-command sites" claim, and the `kp`
coordinate-mark evidence), and one understated its finding (the sibling-`cant-tell` collapse).
The trial's *measurements* were reliable; its *mechanisms* were inferred. Re-verify before
building on any of them, including this note.

---

# §CORRECTION-orlist-not-command-v (appended 2026-07-27, during the W-A build)

Appended, not merged: the original text stands as written and carries an inline superseded-marker
at the affected section. This corrects `fnd-existence-gate-darkens-oracle` §1 — and, by the note's
own `haz-trial-claims-need-independent-check`, it is that hazard firing on this note. The error was
inherited from the r26 trial (`README.md` §4 item 2) and re-stated here without an independent
single-variable check; the observations were reproduced, the *mechanism* was not.

## What is wrong

`fnd-existence-gate-darkens-oracle` attributes the degrade to `command -v` specifically, and
generalises to "any unmodeled statement before a marked line makes the site unresolvable"
(the trial's wording, which the finding adopted). Both are wrong.

## The corrected mechanism

Controlled bisect — one `wombat__predict` body, every line held constant except the leading gate,
planned read-only against the same book:

| gate line under test | site resolution |
|---|---|
| *(no gate)* | RESOLVES |
| `command -v wombat >/dev/null 2>&1 \|\| return 2` | unresolvable |
| `wombat ping >/dev/null 2>&1 \|\| return 2` | **unresolvable** |
| `command -v wombat >/dev/null 2>&1` (same command, no `\|\|`) | **RESOLVES** |
| `if [ "${1-}" = "" ]; then return 2; fi` | RESOLVES |
| `wombat ping >/dev/null 2>&1 && return 2` | RESOLVES |

Rows 3 and 4 are the discriminator: swapping the command while keeping `||` still fails; keeping
`command -v` while dropping `||` succeeds. **The or-list is the cause. `command -v` models fine.**

Why: `oracle/src/predict/lexer.rs:135` lexes `|` as a one-byte `Tok::Pipe`, so `||` arrives as two
adjacent pipe tokens; `parse_command` (`parser.rs`, the `CmdTok::Pipe` arm) raises `pipeline = true`
for each, folding the whole or-list into one accepted, byte-exact-shipping Command; and
`eval.rs`'s `run_stmt` degrades any `cmd.pipeline` to `Flow::Top`. The site then runs unprobed. The
reason was discarded at `analysis/src/effect.rs:347` (`Resolution::Top(_) => None`), which is why
nothing surfaced. `&&` does NOT share the mechanism (row 6) — it is not lexed as a pipe.

## What still stands

Everything observational: the gate as the contract writes it does darken a predict body totally;
`dorc lint` reported 0 errors / 0 warnings; the contract prescribes that exact line at
`oracle-contract.md:103-104`; and the finding's law verdict (a doc-vs-engine contradiction, plus
`inv-top-reject`'s silence violation) is unaffected — only the named construct changes.

## Knock-on for the ruling queue

`26H` §5's **R-1-model-command-v is wrongly posed**. It asks whether the tracer should model
`command -v` (and how far `test`/`[`/`case $?` go). `command -v` already models. The real
ruling-gated question is **or-lists in oracle bodies** — which the verdict-lift scope note in
`oracle/src/verdict.rs` records as a deliberate parser scope-cut, not an oversight — weighed
against fixing the oracle-contract, whose taught existence-gate idiom is currently unusable.
That is a doc-vs-dialect choice for the human, and it is a different, broader question than the
one the queue currently holds.

## Landed under W-A (diagnostics only, no dispositions moved)

The or-list degrade now reports as itself (`TopReason::OrList`) instead of borrowing the pipeline
reason — a mis-attribution outranks silence, and "reached a command pipeline" is what an author
who wrote the contract's own `|| return 2` would have been told. Behaviour is pinned identical:
both shapes still degrade to ⊤, and the accept/degrade decision is untouched.

---

# §FINDING-andand-resolves-a-wrong-coordinate (appended 2026-07-27, during the W-A second half)

DIAGNOSIS ONLY — nothing was fixed, per the dispatch's checkpoint gate. This is a **soundness**
finding (a wrong *yes*), unlike all four original `26G` findings, so it does not inherit
`haz-safety-direction-holds-everywhere`. It was found while discharging the bounded `&&`
investigation the or-list correction handed forward (`§CORRECTION-orlist-not-command-v` row 6
recorded `&&` as "RESOLVES" without asking whether it resolves *correctly*).

## The mechanism

`&` is not a metacharacter in the predict lexer (`oracle/src/predict/lexer.rs` `run`: `b'|'` gets
`Tok::Pipe`, `b'&'` falls through to the `_ => self.word()` arm). So `a && b` lexes as the three
WORDS `a`, `&&`, `b`, and `parse_command` folds them into ONE `Command` whose `words` are
`[a, &&, b]` and whose span covers the whole line. Every statement to the right of `&&` is
therefore invisible to the tracer — while the byte-exact shipped probe still executes it on the
host. The or-list is the mirror image and is safe: `||` lexes as two adjacent `Tok::Pipe`s, raises
`pipeline`, and degrades to `TopReason::OrList` (⊤ ⇒ the site runs).

## The repro (pure static; `dorc_oracle::predict::{lift_predicts, evaluate}`, nothing executed)

```sh
# dorc-lang/v0.2
w__predict() {
   w precheck && shift
   thing : sm.dorc.Thing = "$1"
   w query "$thing" : sm.dorc.Thing:"$thing"@present
}
```

`evaluate(check, ["alpha", "beta"])`, one variable changed per row:

| separator before `shift` | resolution |
|---|---|
| `;` (`w precheck; shift`) | `Resolved{ entity: Operand("beta") }` — the tracer models the shift |
| `&&` | **`Resolved{ entity: Operand("alpha") }`** — the shift is swallowed as a word |
| `\|\|` | `Top(OrList)` — degrades, safe |

`dorc lint` reports nothing on any row.

## Why it has teeth

The `&&` row is not a degrade, it is a WRONG resolution. `Resolved::probe_body` for that row
carries the `w precheck && shift` span, so the shipped probe DOES shift at runtime: the host
measures `sm.dorc.Thing:beta@present` and the engine files the record under
`sm.dorc.Thing:alpha@present`. A converged `holds` about beta then licenses eliding a mutator on
alpha — the priority-1 under-execute, and the exact "a *wrong* `Resolved` is the disaster class"
the evaluator's own doc-comment (`eval.rs`, `Resolution`) says it biases every ambiguity away
from. `inv-top-reject` ("bias every parser ambiguity toward ⊤-reject-with-diagnostic") is violated
in the direction that costs correctness rather than precision.

Per-statement severity of what `&&` hides, since not every swallowed statement is unsafe:
`shift` ⇒ WRONG entity (above). An assignment (`… && verb=modern`) ⇒ the variable stays unbound ⇒
a later use degrades to ⊤ ⇒ safe. `return N` ⇒ the tracer over-resolves a path the host declines,
but the host's rc ≥ 2 reads back `cant-tell` ⇒ safe. So the exposure is narrow but real, and it is
positional-argv-shaped — the one thing the coordinate is built out of.

## Not fixed here, and why the obvious fix is not W-A's

Lexing `&&` as a token so the and-list degrades like the or-list is a ⊤-*grow* (sites that resolve
today would stop resolving), which moves dispositions — outside W-A's "diagnostics only, zero
elision movement" tripwire. It needs a conductor checkpoint and its own re-bless. Scope note for
whoever takes it: a single `&` (background/`a & b`) rides the same `_ => self.word()` arm and wants
the same exclusion-check; the corpus's `&`-bearing text is all inside redirects (`2>&1`), which the
`redirect()` lexer already consumes before the word arm sees it.
