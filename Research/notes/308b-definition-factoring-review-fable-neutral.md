# 308b — Neutral review of the landed definition-factoring stage (`28Q` §1, stage-i)

> Tier: independent review record (Fable-class, neutral lane). Reviewed range:
> `feb2305f..083efd8a` at tip `083efd8a377b77832df98ffef48f6c5bf36359ba`, static reading only
> (nothing executed; no gate run by this reviewer). Independence rider: per the review brief,
> this lane read NONE of `Research/notes/30*` (the lane's own `307`/`307c` records included)
> nor `28R*`; every conclusion below is derived from the root docs, `spike/CLAUDE.md`, the
> crate law files, `Research/plans/28Q`, and the code/fixtures at the tip. Where a landed
> comment cites `307c`-slugs, I saw the citation, not the note.
>
> Criteria: root `README`/`DESIGN`/`IMPLEMENTATION`/`USER_STORY`, `spike/CLAUDE.md` + crate
> `CLAUDE.md`s, and `28Q` §1/§7/§8 (`stage-i-definition-factoring`). Weighting per
> `271:rul-sin-ordering` and the execution-priority order: a wrongly-minted license outranks
> everything.

## §0 — Verdict in one screen

The conversion itself is well-built and lands what §1 ruled: derived rows keyed by producing
file, one resolution rule (`core::definition::answering_file`, `core/src/definition.rs:143`)
consumed at every role-lane seat I could enumerate, the agreement veto / `live_source` /
never-live-withdrawal triple retired, `never_live` re-scoped to the one whole-unit fold that
genuinely has no frame (`build_dialect`, `oracle/src/lib.rs:540`) with the direction argued
correctly, the two drivers unified, and a fixture discipline (floor30 differential manifests
minted BEFORE the conversion — commit `1cdd8020` is an ancestor of the range base — plus the
plurality census and the join census) that is genuinely stronger than a golden gate. I found
no wrong-elision route minted by this range: every direction I exclusion-checked (withdrawal
of `disturbs` sets, the never-live dialect fold, the `NoOpinion` plural withhold, the fact
merge) fails toward run/collide.

What I did find: the **wrapped-entry lane's own index folds sit outside the conversion** and
outside the withdrawal edge, in ways the range's own hardened law text now (over-)claims are
closed — including one first-file-wins consent fold that licenses probe-time context entry
from the wrong author's mark in a constructible blessed-plural world (F1, F2). And the
plan's own EXPECTED-TO-FLIP diagnostic cell did not flip, its in-tree text and the plan's
hypothesis are now stale, and the falsification is visible only in quarantined-to-me notes,
if anywhere (F3).

## §1 — Findings, ordered by importance

### F1 — fnd-wrapped-entry-folds-outside-the-conversion (code-tier; consent/license plane)

The stage's law texts state total coverage: `analysis/CLAUDE.md
visibility-is-full-positional` — "every SITE-KEYED consuming act (verdict, predict-at-site,
**probe-ship**, vouch, guard eligibility) answers only from the definition live AT the
site" — and `oracle/CLAUDE.md the-frame-lookup-is-the-only-resolution-seat`. The wrapped
lane's index construction does not obey either. In
`cli/src/survival.rs::build_wrapper_index` (`spike/crates/cli/src/survival.rs:928–981`),
three per-provider folds are whole-unit and **first-file-wins** (`or_insert` over load
order):

- `wrappers.entry(word).or_insert(WrapperModel { … })` (survival.rs:965) — the wrapper's
  peel/ρ/lend model;
- `enter_defs.entry(p).or_insert((fname, stripped))` (survival.rs:963) — the **entry-form
  bytes that really execute on the host** to enter a context;
- `tolerance.entry(p).or_insert(vouch)` (survival.rs:979) — the `safe-across` consent
  vouch, consumed at `decide_entry` (survival.rs:735).

First-wins is the order `28K` §6 rejected as *worse than* last-wins (the retired
wrapped-vouch seat's own recorded sin), and none of the three consults
`LiveDefinitions`. Meanwhile the bodies these folds govern ARE frame-resolved
(`resolve_inner_check` → `ship_predict_body`/`ship_verdict_body` → `shipping_source`,
world.rs:742). So the composed probe can pair a frame-correct inner body with a
whole-unit-first author's consent mark, entry form, and lend model.

Concrete world (all-legal, uncontested — the blessing makes it reachable):

```sh
# base.oracle.sh              (loaded first)
hork__is_converged() {
   : safe-across user
   hork query -- "$2"
}
# book.sh
unset -f hork__is_converged            # the 28K §1 blessing — no contest, no withdrawal
hork__is_converged() {
   grep -q good /etc/hork.conf         # THIS author typed no safe-across mark
}
sudo hork install wombat
```

At the site, the frame names the book's body and `resolve_inner_check` ships it (correct).
But `decide_entry`'s `tolerated` comes from `tolerance.get(hork)` = the FIRST verdict body
lifted = base.oracle.sh's `safe-across user` (survival.rs:975–981 iterates `verdict_sets`
in load order). Result: the probe **enters the user-shifted context and executes the book
author's body there, on the strength of the base author's per-function consent mark** — a
consent chimera. `plans/27C` (both-sides consent: "author's per-function per-dimension mark
× admin's dial") is in `28Q` §6's preserved-invariant wall; this composed behavior violates
it. The dual construction with two blessed `sudo__predict`/`sudo__enter` definitions puts
the FIRST author's entry-form bytes on the host at a frame that names the second's.

Weighting, honestly: this is not under-execution (no elision is minted; the failure class
is wrong-author-consented probe-time execution, and every body involved still sits under
the authored no-mutation contract). +SURE the folds are first-wins and frame-blind (code
above); +SURE the tolerance divergence is reachable via the blessed-override construction;
~SUSPECT the divergence predates this range in narrower worlds (pre-conversion, the
whole-unit verdict winner was LAST-wins while tolerance was already first-wins, so the two
could already disagree for a book-last override) — but stage-i both (a) widened the
answering plural population these folds now disagree across (subshell/regional idioms are
its own product, `frame30-*`), and (b) landed the law text that claims the seat inventory
is closed. Either the wrapped-entry folds convert (frame-keyed, matching the shipped body),
or the law bullets gain a named carve the way `vocabulary-acts-stay-ambient` names its
exception — silent divergence between "the body that ships" and "the consent/entry model
that licenses it" is precisely the chimera class this stage exists to make unspellable.

### F2 — fnd-wrapper-predict-re-lift-bypasses-the-withdrawal-edge (code-vs-landed-law)

`cli/CLAUDE.md withdrawal-is-applied-once-never-consulted` was hardened in this range to:
"NO SEAT SITS OUTSIDE THE EDGE. Every lifted vector routes through it"
(`spike/crates/cli/CLAUDE.md:119`). That claim is falsified by the same function:
`build_wrapper_index` **re-lifts predicts from raw source text**
(`lift_predicts(interner, src)`, survival.rs:943) instead of taking the withdrawn `checks`
vector the binary already holds — the exact "a seat that re-lifts is a seat that will
disagree" failure `oracle/CLAUDE.md` records. Consequence: a CONTESTED wrapper family (two
files defining `sudo__predict`, no blessing — the family whose licenses
`ContestedFamilies` withholds and whose sites must be "indistinguishable from one nobody
described") is invisible to every ordinary seat, yet still peels here, still builds a
`WrapperModel` from the first file, and can still reach `EntryDecision::Enter` — i.e. a
withheld family still licenses probe-time context entry and in-context execution of its
first definition's entry bytes. (The tolerance half at survival.rs:975 DOES take the
withdrawn sets, so the two halves of one bundle sit on opposite sides of the edge.)

+SURE of the code shape; ~SUSPECT the re-lift predates this range (the range diff does not
touch it) — but the range's own carve-closure text is what turns a known-shape residue into
a documented-law contradiction. Fix is cheap (hand `build_wrapper_index` the withdrawn
`checks`), or the law text must name the residue instead of denying it.

### F3 — fnd-expected-flip-cell-did-not-flip (plan-tier + record currency)

`28Q` §1 bound stage-i to test a hypothesis: "`tc-wrapped-lane-drops-a-case-bodied-in-book-
verdict` was measured, never diagnosed — the oracle-only-vector reading is a HYPOTHESIS, so
the case-bodied in-book wrapped fixture rides stage-i as an EXPECTED-TO-FLIP cell (the
asserted cause gets tested, not trusted)." At the tip, the cell
(`spike/crates/cli/tests/pin30-wrapped-case-bodied-in-book-verdict.loom`, untouched by the
range) still commits `site:0 unresolvable-no-probe` / `sites=0`: **the flip did not
happen**, even though stage-i delivered everything the hypothesis said was missing (the
WhyWorld re-lift, source-wide vectors, the unified `build_wrapped_vouches` seat).

The hypothesis is in fact falsifiable from the corpus alone, without running anything:
`pin28-wrapped-vouch-answers-at-a-live-site.loom` is ALSO a case of an in-book verdict at a
wrapped site — same book placement, same `: safe-across user`, same sudo oracle
("only the verdict body differs between the two cases", pin30's own header) — and it ships
its check and elides. The discriminator is therefore the verdict body's SHAPE
(delegation-bodied answers; `case`-bodied drops), not vector membership. +SURE of that
much; -GUESS at the precise declining seat (somewhere in the wrapped lane that traces or
classifies the case-armed body; I could not pin it statically without executing).

Three consequences to report:

1. The behavior is conservative (the site runs) — value-loss, not a license bug.
2. The plan text (`28Q` §1, last bullet) now carries a falsified hypothesis un-amended, and
   `plans/` are ahistorical-and-rewritten by law (root `AGENTS.md` doc-maintenance); the
   fixture's own line 8 ("EXPECTED-TO-FLIP at stage-i…answers nothing there today") is now
   stale at a tip where stage-i has landed — the exact record-drift class that bit stage-0
   (`307:fnd-stage-zero-is-not-built`, cited by the in-range CLAUDE.md edits themselves).
3. If a diagnosis was banked, it lives only in the lane notes this review is fenced from;
   nothing durable-and-unquarantined (plan, fixture, crate law) records the falsification.

### F4 — fnd-auto-cell-is-the-one-unkeyed-row (design-tier watch; law-text over-travel)

`28Q` §1.1 says "Every derived row … is keyed by the DefinitionId that produced it," and the
landed law says the chimera "cannot be SPELLED." One row-family is exempt and nothing names
the exemption: the auto-cell. `verdict_cell_or_auto`
(`spike/crates/analysis/src/effect.rs:271–293`) resolves the ANSWERING definition by frame
(correct), but the markless floor then mints `dorc_core::auto_fact(provider)` — a
**per-provider singleton FactKey with no definition in its identity**. Post-conversion, two
frames' different bodies (e.g. `frame30-subshell-body-answers-inside-only.loom`: the book's
regional body at site 0, the oracle's at site 1) legitimately mint the SAME
`dorc-auto:foobar@converged` cell. Pre-conversion this was one-author-per-provider by
construction; now it is a genuinely shared coordinate between authors' judgments.

I traced the containment and it holds today — this is a watch-item, not a hole:

- the fact-keyed observe seat meets cross-site disagreement to Unknown/⊤
  (`cli/src/results.rs:852` `facts_from_sites`, `:1037` `merge_observable`) — agreement is
  the only survivable overlap, and each agreeing site was measured by its own
  frame-resolved body (the loom's probe artifact shows both bodies shipped, site-keyed);
- flag-off, a running same-cell site is a TOTAL wall (`build_plan_walled` docs); flag-on,
  same-cell is never provably-disjoint, so survival collides; `fence-no-disjoint` keeps
  auto kinds may-touch (main.rs:1527–1542, now deliberately file-blind — direction argued
  in-code, correctly).

Residue worth the entry: (a) `probe_origins` joins two records' origins on the one fact, so
a why-chain for the oracle-answered site can cite the book-body's record — mis-attribution-
adjacent, aid-plane only today; (b) the named future decision "fact-keyed verdict shapes"
(`inv-site-keyed-results`, kSTATE-coupled) would re-open this seam with two-author cells
already in the world. Recommend one sentence in `core/CLAUDE.md
auto-cell-is-the-markless-floor` naming the multi-author-cell fact, so the
"chimera unrepresentable" claim doesn't travel further than the mechanism does.

### F5 — fnd-frame-differential-is-file-granular (test-coverage note)

The engine-agreement half of the differential
(`cli/tests/definition_frames.rs::the_engine_names_the_definition_the_shells_ran`, :677)
compares `live.source_before(site, ROLE)` — a FILE id — against the shells' emitted token.
The frame machinery's identity is definition-granular (file × span), and the one place the
distinction bites (two definitions of one role in ONE file across frames) is exactly where
a file-granular comparison is blind. The funcenv unit tests do pin span-granularity
(`the_frame_lookup_names_the_definition_live_at_each_site`, funcenv.rs), and
`DefinitionProvenance::Ambiguous` withholds the within-file-redefinition population at the
row seat, so nothing unsound hides here — but no committed shell-measured cell exercises a
within-one-file two-frame world (e.g. one file sourced at top level AND re-sourced in a
subshell after edit is impossible; the honest cell is a file holding top-level +
`unset -f` + redefine). Cheap to add to the floor30 battery if the corpus ever grows the
idiom; until then the differential's blindness is bounded by the Ambiguous withhold.
~SUSPECT low value today; noting it keeps the census honest about what "the shells' own
answers" covers.

### F6 — fnd-hash-munge-reachability-claim-still-untested (plan-tier)

`28Q` §1 claims "bitem1's built-but-unreachable hash-munge becomes reachable exactly as its
ledger predicted (two frames, two live bodies, two munged names)". No landed cell exercises
an engine-emitted munged name, and my static walk suggests the claim may still be vacuous
at stage-i: a guard's runtime binding tracks the frame *through the book's own bytes* (an
in-book/regional definition precedes its sites in the rendered artifact; a subshell
re-source re-executes at apply), and preamble copies are needed only for bodies NOT in the
artifact's text — which, with oracles being ambient-prefix (one winner per name at every
book frame), collapses back to at most one preamble body per name. The frame30 apply
artifacts confirm the pattern: the preamble carries the oracle body once, the regional body
rides the book bytes, and runtime binding is correct without any munge
(`frame30-a-regional-decline-is-a-decline.loom:101–134`). -GUESS there is some reachable
munge world I have not constructed (the `pinned-definitions` machinery is welcome belt-and-
braces regardless); but the plan's "becomes reachable" should either gain its witness cell
or lose the claim. (`floor30-sibling-frames-hold-three-bodies` measures the shells, not the
engine's render, so it is not that witness.)

### F7 — nits (each verified at tip)

- `spike/crates/analysis/src/funcenv.rs:191` — doc-comment still points at the deleted
  `LiveDefinitions::answers_at`, and :196 still says "`reserved.rs` **refuses** at Error
  severity" — the exact wording the in-range commit `bd3ce686` corrected elsewhere to
  marked-not-refused (`307:fnd-reserved-name-error-does-not-refuse` as cited by
  `definition_frames.rs:203`). Broken intra-doc link + a claim the range itself disproved.
- `oracle/src/lib.rs:492` — the dialect-minting scan keys `binds_somewhere(i, raw_provider)`
  and `s.get(raw_provider)` by the provider AS THE DECLARING FILE spells it, where
  `verdict.rs::from_sets` normalizes through `map_provider_name` for exactly the
  two-spellings-one-provider case. Behavior-preserving versus the retired
  `dorc_oracle_live_source` (same keying), and funcnames being munged POSIX NAMEs makes
  divergence hard to author — but the fold now carries the normalization asymmetry in one
  more place. Worth a line when the dialect fold is next touched.
- Hint/sweep lanes (`survival_diagnostics`, `sweep/drive.rs:518`
  `touches_answering_source`): under `unsolved()` + `NoOpinion`, plural `disturbs`
  candidates now WITHHOLD where they used to first-match — deliberate, documented in-code,
  aid-plane only. Fine; recording it as the one visible behavior change outside the
  license plane.
- `WhyWorld` still models no wrapped sites (`world.rs:134` `let peeled = BTreeMap::new()`),
  so a why report over a wrapped book explains a narrower world than the run — pre-existing,
  outside this range's scope, but adjacent to the driver-unification story the range
  otherwise closed; worth a named residue line in `cli/CLAUDE.md
  one-definition-table-two-drivers` rather than silence.

## §2 — Checked and cleared (the exclusion-check ledger)

- **Withdrawal now covers the `disturbs` sets** (`survival::{lift_touches_sets,
  pair_touches_sets}`, survival.rs:15/37; both drivers): direction verified — removal of
  an at-most claim ⇒ no footprint ⇒ TOTAL wall (`build_survival_footprints`'s None arm),
  never an empty-footprint vacuous spare; `⊤ never encoded as ∅` holds at this seat.
- **`never_live` → dialect-only** (funcenv.rs:1035dd, world.rs:708, oracle lib:492):
  winner preservation argued and test-pinned
  (`a_never_live_definition_mints_no_dialect_tokens`, both halves from one fixture);
  the danger direction (larger/shifted dialect SPARES MORE) is stated at every consumer;
  dead rows left in the index are unreachable by resolution and I found no other
  reader of the effect map that iterates rows unfiltered (`build_dialect` filters by
  `dialect_minting_source`; `effect_of`/`widening_of` are file-keyed).
- **Seat inventory** (the six-plus): effect-lift rows, `VerdictIndex` rows,
  `effect.rs` predict + verdict lanes (:358/:271 via `answering`, :340), the three cli
  ship seats (world.rs:765/:813, main.rs `ship_predict_stage`), `build_vouches_from_sets`
  and `build_wrapped_vouches` (plan/lib.rs:1538/1693), and — beyond the plan's list — the
  survival footprint lane's three scans (`resolve_touches_footprint`,
  `touches_defining_span`, `ship_touches_body` → `touches_answering_source` →
  `shipping_source`) and the sweep mirror. `live_source` retains exactly ONE production
  caller (the dialect fold); `answers_at` is gone. The forward-scan the footprint lane
  retired was a REAL pre-existing wrong-elision route (a wrong body's emission can NARROW
  an at-most claim; narrower spares more) — the both-load-orders test
  (`the_footprint_answers_from_the_definition_the_frame_names`) is the right pin, and
  fixing it inside this stage was correct scope.
- **`NoOpinion` plural-withhold** (core/definition.rs:153): retires load-order-as-trust-
  adjudicator with an order-SYMMETRIC pin
  (`competing_definitions_without_an_environment_withhold_in_either_order`) plus the
  sole-answers control that keeps it a statement about plurality; direction safe (Opaque ⇒
  MustRun; an aid-note is what's lost). `Ambiguous` answers at no frame under both arms.
- **Driver unification**: binary and `WhyWorld` now build the same source-wide world,
  mint contested/never-live from the same two calls, and withdraw identically
  (world.rs:112–230 vs main.rs:841–925); `definition_table` iterates `oracle_paths`
  only, so the book is never double-registered (world.rs:643 bound; book sited at
  `source_refs.len()-1` in both drivers — the interim one-past shape retired with an
  honest account of why it had been safe).
- **Fixture discipline vs the plan's staging**: the five floor30 differential manifests
  landed BEFORE the conversion (commit `1cdd8020`, an ancestor of `feb2305f`) — the
  measure-first order §8 demanded — and cover the demanded cells (blessed-override,
  subshell re-source in/out + nesting + removal, helper collision, deep-stack
  binds-at-invocation; sibling-frames for three-body plurality). The golden plural-idiom
  cells (`frame30-*`) arrived after the behavior, per the human's pin-the-future lean. The
  plurality census (`every_reachable_plural_family_is_an_enumerated_plural_idiom`) is
  two-way (stale-entry detection) and withholding-aware; the join census
  (`every_lifted_role_row_joins_to_a_parsed_definition`) demands a live specimen of its
  own exception class before it counts — both are better instruments than the vacuous
  byte gate they compensate for, and the `contested.rs` census note pinning the
  withdrawal's load-bearing five cases closes the "retire it as dead code" trap in
  advance.
- **The helper WITHHOLD floor**: `a_contested_helper_closure_withholds_the_role_body`
  asserts the refusal at the consuming seat (`closure_for` erring on the role's own
  body), matching §1's ruled floor; the snapshot-transplant emission correctly deferred
  to its own stage.
- **`the-frozen-set-includes-the-function-environment`**: the conversion adds no funcenv
  entry into the fixpoint loop (dead_predicts and the contested fact are minted once,
  pre-loop, in both drivers); the funcenv floor's `folded_edges = ∅` break survives with
  its rationale correctly GENERALIZED (every environment answer is now winner-shifting).
- **`vocabulary-acts-stay-ambient`**: kind-owner trio lanes untouched (resolvers/reaches
  still ambient, oracle-only), and the census deliberately excludes them from the join.
- **Byte-identity evidence**: no committed golden or transcript changed in the range
  (diffstat: only new cells, new tests, code, and law files) — consistent with
  `syn-single-frame-byte-identical` over the corpus. I could not re-run the gate
  (static-only mandate), so both-legs green is taken from the range's own discipline,
  not re-verified here; likewise the checker-lane riders (`certifier` + sparing
  re-derivation) are noted as riding `gate` rather than re-executed. NB the sparing
  reference re-derivation cannot see a wrong FRAME answer (it re-checks the compare, not
  footprint resolution) — consistent with, and one more reason for, the
  "funcenv is license-review-tier forever" rider the range stamped at every seat.
- **Stage-0 honesty**: the range correctly re-marked `rul-verdict-primacy-at-the-ship-seat`
  as RULED-not-built in all three law files it had drifted into; nothing in this range
  builds toward stage-0 while claiming otherwise.

## §3 — Not verified by this reviewer (stated plainly)

Execution-dependent claims: suite green on both legs; the pin30 non-flip's precise
declining seat; whether `mise run gate`/`bless` were run as law requires (commit hygiene
suggests yes; the hook system enforces the label/trailer half mechanically). The two
untracked `guard26-*` dirs present in the working tree predate this review's branch switch,
are not part of the reviewed range, and were ignored.
