# 20S — task-L2: member-precision (the in-loop members-elision value). Strain + decisions.

> Round-21 (take-3 continued). Charter: `209` brk-1(b) (the PRECISION slice the L1 structure
> note `20M` §5 recorded as "the floor the member-elision slice later lifts"), executed as
> task-L2 = Members-valued loop vars + per-member fact-families + the all-or-nothing in-loop
> license + the per-member probe wire. AI-authored, confidence-marked. Companions: `20M`
> (L1 structure), `20O` find-6 (the preconditions this fixes/confirms), `20E` (the Query
> firewall this reuses), `20C` (the record grammar this extends). HEAD before task: `e130c22`.

## §0 Headline

+SURE (traced + tested + e2e-exec-gated): an in-loop install whose argv references a
Members-bound for-var now ELIDES per-member when every member is converged — the brk-1(b)
payoff. `for pkg in nginx curl; do apt-get install -y "$pkg"; done`, both converged ⇒ the
body substitutes to `true` and the loop iterates twice over `true`; run-set EMPTY
(`loop-members-all-converged-elides`). The elision is all-or-nothing and self-reach-gated:
one diverged member runs the whole leaf (`loop-members-partial-runs`); a pre-loop writer of a
member cell runs it despite both-converged bait (`loop-member-external-writer-runs`); a body
reassignment of the for-var degrades it to the existing ⊤ floor (`loop-var-body-reassign-tops`).
Gates green ×2 (66 e2e zero-xfail; fmt/clippy-D/test/typos). The in-loop render-floor LIFTS
only for this exact shape; in-loop Queries, multi-leaf/nested interactions, and partial-member
elision stay floored/refused.

## §1 The Members representation, and where it is read (item-1)

+SURE (the chosen mechanism): Members is a SEPARATE side-channel
(`ValueFlow::member_argv: BTreeMap<CfgNodeId, Vec<Vec<ValueOf>>>`), NOT a new lattice element.
The dataflow `ValueEnv` stays `MapL<String, Flat<String>>` exactly as before — the for-var
still binds the Flat JOIN (⊤ for >1 distinct) in the main `argv` map, so EVERYTHING ELSE about
the value-plane is unchanged (the brief's hard constraint). Members never flows through the
general transfer; it is computed by a dedicated post-solve pass (`Prep::members_pass`) that:
- finds each `for` LoopHead whose list is Members-eligible, and
- for each body site whose argv REFERENCES the for-var, substitutes each concrete member into
  the site's incoming state and re-resolves the words (reusing `resolve_site_words`, the
  extracted core of `site_argv`) ⇒ one concrete argv per member.

WHERE IT IS READ: ONLY at in-loop command-sites, by two consumers — `effect::member_family`
(classify, to build the fact-family) and `effect::self_reach_holds`'s caller. Nothing else
reads it; the Flat ⊤ serves every other consumer. So the "Members never flows through general
transfer" contract holds structurally (it is not IN the transfer).

Eligibility is STRICT (every ambiguity ⇒ ineligible ⇒ the existing ⊤ degrade), `eligible_members`:
- ev-1 single-level only: a body containing a nested loop ⇒ ineligible; a `for` head that is
  itself `in_loop_body` (an inner loop) ⇒ ineligible. BOTH directions of a nested pair refused
  (`members_nested_loop_degrades_to_none`). This is the item-3 "multi-leaf interactions stay
  floored" boundary, drawn at the value-plane.
- ev-2 every list word a single concrete (post-F1: no glob/tilde ⇒ ineligible; a split-literal
  list composes into MORE members — `members_split_literal_list_composes`).
- ev-3 no body reassignment of the for-var: an assignment, an lvalue-builtin
  (`read`/`unset`/`export`/`readonly`/`local`/`getopts`) naming it, or a ⊤ (unsupported) region
  inside the body ⇒ ineligible (`members_body_reassign_var_degrades_to_none`,
  `members_body_read_clobbers_var_degrades_to_none`). The brief's "Members never flows through
  general transfer; reassignment ⇒ degrade to the Flat join".

DUPLICATES KEPT (no dedup): `for x in a a` yields a two-element family of the same cell — dash
iterates them, and dedup would mis-model the iteration count. Flagged in the brief as the thing
to verify; KEPT (`members_keeps_duplicates_no_dedup`, `members_family_keeps_duplicate_cells`).
+SURE no place dedups: the value pass pushes per-word fields in order; classify maps 1:1.

## §2 The fact-family + all-or-nothing resolution (item-2)

`SkipClass::EstablishMembers { members: Vec<FactKey>, self_reached: bool }` — the resolved
per-member cells (list order, dups kept) + the self-reach bit (§3). `effect::member_family`
resolves it: each per-member argv runs through `command_effect` (the oracle's own `check()`,
identically to a straight-line command); EVERY member must yield `[Establishes(fact)]` or the
WHOLE site is `None` ⇒ falls to the single-cell path ⇒ the in-loop floor runs it
(`members_family_all_or_nothing_one_member_unresolvable_tops`: `for p in nginx "a b"; do
apt-get install -y $p; done` — the `a b` member splits to two operands, the check refuses,
the whole site tops). NO partial family is ever constructed.

THE REACHING-DEFS CONSEQUENCE (load-bearing, +SURE after tracing): a resolved Members site
GENS its per-member cells into `Reach`, NOT Opaque. Pre-L2, the body install's Flat argv was ⊤
⇒ `command_effect` ⇒ `[Opaque]` ⇒ it joined `Reach::Top` and poisoned everything downstream
across the back-edge. Now `classify`'s `effects[site]` for a member-family is the per-member
`Establishes` cells, so a post-loop install of a DISTINCT package stays EstablishAmbient
(`members_family_gens_member_cells_not_opaque_post_loop_stays_clean`), while a post-loop install
of a loop-member cell is correctly EstablishWritten (`members_family_poisons_post_loop_same_cell`).
This is also what keeps the Members site's OWN in-state pristine-of-others for §3.

## §3 The all-or-nothing in-loop license — the subtle core (item-3)

`ReplaceLicense::prove_members_replaceable` mints the license iff (implemented EXACTLY, every
ambiguity ⇒ REFUSE):
- (a) every member's host verdict is `Converged` (a single Diverged/Unknown ⇒ refuse). The
  family is all-or-nothing; partial-member elision (rewriting the list to the diverged members)
  is the recorded LATER direction, not this.
- (b) `self_reached` (the engine bit, §3.1).
- (c) `consumption_ok(consumed, status)` — the SHARED gate, reused verbatim. The in-loop leaf's
  status is ⊤ (fork-mutator-rc: a mutator's rc has no sanctioned source), so a CONSUMED status
  (errexit-region, or a post-loop `$?` reading the body) blocks; a consumed Stdout/Stderr or an
  if-guard render-floor blocks too.
- (d) per-member-resolvable: SUBSUMED — a member with no probe record arrives `Unknown` ⇒ not
  Converged ⇒ (a) refuses. No separate check needed.

CODE HOME of each conjunct: (a) + (b) + the `Grade::Must`/`Converged` derivation in
`prove_members_replaceable` (plan/lib.rs); (c) in the shared `consumption_ok`; the apply-side
plumbing (observe each member, assemble the verdicts, ⊤ status) in `members_disposition`
(plan/lib.rs), called from `build_plan` BEFORE `disposition_for` (so the in-loop floor in
`disposition_for` is bypassed for the Members shape, and still stands for every other in-loop
leaf). The stand-in is always `StandIn::True` (the body → `true`, observable-preserving given
(a)+(c)). Loop structure NEVER rewritten.

### §3.1 self-reach (item-3(b)) — why the naive cell-set check is WRONG, and the fix

THE TRAP (caught by `members_self_reach_broken_by_pre_loop_writer`): self-reach asks "are the
ONLY writers reaching this site its own per-member establishes?" My first implementation checked
"is the site's `Reach` in-state ⊆ the family's cells?". That is WRONG: a pre-loop `apt-get purge
curl` writes `package:curl#installed` — the VERY cell the loop's curl-member establishes — so the
in-state IS a subset of the family by cell-identity, and the naive check passed `self_reached:
true` (the test failed loudly, demonstrated end-to-end). A cell-set lattice cannot distinguish
"my own establish reached" from "a DIFFERENT writer wrote the same cell".

THE FIX (+SURE, traced): `self_reach_holds` re-solves the reaching-defs with THIS site's gen
SUPPRESSED (`reach_transfer`'s `suppress` arg), then checks the site's in-state is pristine
(empty, not ⊤). With the self-establish removed, the back-edge carries ONLY other writers' cells
to the in-state; pristine ⟺ no pre-loop writer, no in-loop sibling, no Opaque reached it ⟺ only
self. Costs N small extra solves (N = Members sites, ≤ a handful/book; perf is network-dominated,
`spike/CLAUDE.md`). Pinned both directions: pre-loop writer (`..._by_pre_loop_writer`), in-loop
Opaque sibling (`..._by_opaque_in_body`), clean (the `_classifies_as_..._family` self_reached:
true).

RATIONALE preserved at the license site (the fixed-point argument the brief mandated): the
elision's own effect removes the body's writes, so under the elision the resting probe stays
authoritative (elide-all is self-consistent); ANY non-self writer breaks that argument ⇒ refuse.

## §4 The record-grammar extension + the wire (item-4)

`ProbeCheck` gains `member: Option<u32>`. `render::probe::site_key` formats the record key: `N`
(single-fact) or `N.M` (member M of leaf N). The grammar becomes
`site <leafid>[.<member-idx>] effect=… rc=…` (documented in the artifact header's existing
`<leafid>` slot, kept byte-stable — the `.M` is an EXTENSION of the leafid token, same posture
20C/20H took for reserved keys). `compile_probe`'s `push_member_checks` ships ONE check per
member, all-or-nothing on the probe body (a member with no declared probe ⇒ the whole site
unresolvable — `can't-probe ⇒ can't-elide`). The cli re-keys: `RecordKey { site, member }`,
`parse_site_key` parses `N.M`, `facts_from_sites` looks up `(check.site, check.member)`. The
existing same-cell conservative merge (`merge_observable`) is untouched and still applies if two
member checks resolve to the same cell.

The `run.sh` awk touch (minimal, POSIX): gate-1's site-id extraction + grammar regex accept an
optional `.M` (`[0-9][0-9.]*`), and the set-compare uses lexical `sort` (a `.M` key is not a
plain int, so `sort -n` would mis-order; lexical equality of the two sets still holds).
`norm_parity.awk` needed NO change (it keys on `$2` as a string — `0.0` works as-is).

PARITY (the load-bearing gate-1 check): the per-package dpkg-query shims reproduce the authored
member records exactly — `loop-members-partial-runs`'s shim exits 1 for `curl`, 0 for `nginx`,
so the probe emits `site 0.0 effect=holds` + `site 0.1 effect=absent`, matching the fixture
(rc firewalled off establish-class records per `norm_parity`).

## §5 item-6 preconditions (fixed FIRST)

- **item-6a (FIXED + pinned): post-`while` `$?` marked the CONDITION, not the BODY.** dash
  (verified): post-loop `$?` = the body's last command rc (loop ran ≥1) or 0 (ran 0) — NEVER the
  condition's. Pinned dash facts: `n=0; while [ $n -lt 2 ]; do n=$((n+1)); (exit 7); done; echo
  $?` ⇒ 7 (body-last, not the `[ ]` false rc); `while false; do …; done` ⇒ 0; `for x in a b; do
  (exit 5); done` ⇒ 5; empty `for` ⇒ 0. The bug: a `while`'s only exit edge is `cond_exit →
  merge`, so the backward `$?`-predecessor walk stopped at the condition command (the `cond_exit`
  IS the condition's last `Command`) and never reached the body. FIX: `lower_while` records
  `while_exit_to_body[merge] = body_exit`; `mark_dollar_question_predecessors`, when its walk
  reaches such a `merge`, also seeds the body-exit. The condition keeps its mark (harmless — it
  is `StatusRenderFloor`-blocked unconditionally anyway). `for` verified-correct and LEFT (its
  `head → merge` exit + `head`'s back-edge pred already reaches the body — proven by a temp probe
  during dev, then pinned: `consumed_post_for_dollar_question_marks_body`,
  `consumed_post_while_dollar_question_marks_body_not_only_condition`). WHY THIS MATTERS for L2:
  without it, a member-loop body whose post-loop `$?` is read would elide while its rc is consumed
  — exactly the `kFAIL-perform` under-execute (item-3(c) relies on the body being marked).
- **item-6b (DONE): in-loop Query sites EXCLUDED from probe compilation.** `compile_probe` skip-
  unresolvables an in-loop `QueryResolvable` site (it stays render-floored this round; probing it
  is wasted remote work, and with item-4 it would ship per-member). An in-loop MEMBERS establish
  is the ONE in-loop shape that ships (per-member) checks.
- **item-6c (CONFIRMED, no action): `done < file` already ⊤-rejects post-F1.** Verified: `while
  read line; do …; done < /etc/pkgs` ⇒ `parse: error[syntax-unsupported]` + `cfg:
  error[cfg-top-node]` (task-F1 / 20O find-3 handles it).

## §6 Render (item-5)

A licensed in-loop Members leaf substitutes via the EXISTING in-situ machinery — the task-F2
scaffolding-shared splice. `for pkg in nginx curl; do apt-get install -y "$pkg"; done` renders
`for pkg in nginx curl; do true; done` (the install span replaced by `true` in-situ, the
`for`/`do`/`done` scaffolding kept). `dash -n`-clean, the loop iterates N times over `true`,
observable-preserving given the license's (a)+(c). No new render path was needed — the body
leaf shares its line with loop scaffolding, so `scaffolding_boundary_lines` + `inline_scaffold_subst`
already covered it. Loop structure never rewritten. Verified end-to-end (the elides case's apply
parses + runs to an empty run-set).

## §7 Every golden delta (corpus)

- gd-1 **`loop-analyzed-body-runs` RE-GROUNDED** (the one tracked non-new `expected.out` change;
  justified line-by-line below). Its L1 premise ("a multi-word loop body is ⊤/Opaque ⇒ poisons
  the curl below it") is FALSE under L2: `for x in a b; do apt-get install -y "$x"; done` is now a
  Members family (the body references `$x`), so it ships per-member checks and gens member cells
  (curl below is no longer poisoned). Rather than let it silently become an elision case (that is
  `loop-members-all-converged-elides`'s job), re-pointed it at the floor-STILL-holds-for-non-Members
  shape: `for x in a b; do apt-get install -y nginx; done` (CONSTANT body, not referencing the
  for-var ⇒ NOT a Members site ⇒ self-establishing ⇒ EstablishWritten via the back-edge ⇒
  un-probeable + floored ⇒ runs every iteration). Run-set: `nginx` twice. book.sh +
  probe-results.txt + expected.ran + expected.out all updated coherently.
- gd-2..5 **the four item-7 cases CREATED** (each exec-gated, probe shims per parity):
  `loop-members-all-converged-elides` (both converged ⇒ body → `true`, run-set EMPTY — the
  payoff); `loop-members-partial-runs` (curl diverged ⇒ whole leaf runs, run-set both installs);
  `loop-member-external-writer-runs` (pre-loop `purge curl` ⇒ self-reach broken ⇒ runs despite
  both-converged bait, run-set purge+both); `loop-var-body-reassign-tops` (body `pkg=evil` ⇒ not
  a Members site ⇒ floored, run-set `install evil` twice).
- The other 61 cases: BLESS-run left every tracked `expected.out` BYTE-IDENTICAL (git shows only
  `loop-analyzed-body-runs` changed among tracked cases; the bless re-wrote all 66 but reproduced
  61 verbatim). Hand-derived run-sets verified preserved (a pre-bless snapshot diff of all 5
  touched cases' `expected.ran` showed zero change).

## §8 Flags (tc-* — surfaced, not resolved)

- tc-l2-member-list-not-rewritten (DESIGN, the recorded LATER direction): the brief's `209`
  brk-1(b) names "rewrite the iteration list to the diverged members" (`for pkg in postgresql`)
  as the render direction. This slice does NOT do that — it elides the WHOLE body (all members)
  or none (item-3 all-or-nothing). Partial-member elision (list-rewriting) is explicitly deferred
  ("member-LIST-rewriting is a recorded later direction, not yours"). So `loop-members-partial-runs`
  runs BOTH installs even though nginx alone is converged — the converged-member work is not
  saved. Flagged as the recorded imprecision, not a bug.
- tc-l2-singleton-member-family (minor, ~SUSPECT benign): a single-word `for f in nginx` loop is
  now a 1-member family (it elides per item-3 when converged — `in_loop_members_single_member_elides_when_converged`),
  REPLACING the L1 floor's RUN for it (`in_loop_establish_runs_even_when_converged` was re-pointed
  at a CONSTANT body, `in_loop_constant_establish_runs_even_when_converged`). +SURE this is correct
  (a single-member loop IS the simplest member-elision), but it is a behavior change from L1 that a
  reader of `20M` §5 might not expect — stated so the next round doesn't read it as a regression.
- tc-l2-nested-refused-coarsely (DESIGN, accepted): a nested loop refuses Members on BOTH the
  outer (body-has-nested-loop) AND the inner (head-is-in_loop_body) loop. An inner single-level
  loop is technically analyzable in isolation, but its members interact with the outer iteration
  (the doubly-nested reasoning item-3 defers), so I refuse it wholesale. Conservative-sound; the
  precision is the deferred multi-leaf direction.
- tc-l2-self-reach-cost (perf, ~SUSPECT moot): N extra reaching-defs solves (one per Members site).
  Network-dominated, so moot for the spike; flagged in case a future round has many member-loops
  and wants a single provenance-carrying solve instead (a `SourcedReach` lattice tracking
  `(cell, source)` would do it in one pass — considered, deferred as over-engineering for now).

## §9 Exclusion-check (the four-by-two, abbreviated)

- reverse direction (backward/Must): `EstablishMembers` + `self_reached` are
  orientation-agnostic engine facts (minted before any phase collapse); the license is the
  apply-phase collapse in `members_disposition`. No backward caller exists; a future one would
  need its own member handling (flagged, not built — same as `20M` §7's note for the floor).
- other phase (probe): a Members site ships per-member probe checks REGARDLESS of `self_reached`
  (the probe must learn each member's convergence; `self_reached` gates only the apply-side
  license). VERIFIED: the probe for `loop-member-external-writer-runs` emits `site 1.0`/`site 1.1`
  even though the apply refuses (self-reach false) — the wire carries the observation, the caller
  collapses (`inv-superposition`).
- other user (lazy admin): a scrappy `for pkg in a b c; do apt-get install "$pkg"; done` now
  elides per-member when converged — the exact precision the admin wanted, with the all-or-nothing
  + self-reach guards keeping it safe. A reassignment or a nested loop degrades to the L1 floor
  (safe).
- other reliability (unreliable oracle): a member with a flaky/unknown rc arrives non-Converged ⇒
  the all-or-nothing (a) refuses the whole leaf ⇒ runs. Strictly safer than the per-member path
  would be if it elided the converged members (it doesn't, this slice).

## §10 What an adversarial crosscheck should attack (the hunt-list — write it yourself)

- hunt-1 (self-reach, priority-1): find a shape where `self_reach_holds`'s suppressed-solve
  concludes pristine but a NON-self writer actually reaches the site at runtime. Attack vectors:
  a writer reached ONLY via the back-edge from a LATER body command (does suppressing only the
  Members site, not the sibling, miss a sibling-via-back-edge writer of a member cell?); a
  member-cell written by a command in a DIFFERENT loop that shares the cell; the interaction
  with `EstablishWritten` siblings. The suppressed-solve removes only the site's OWN gen — verify
  a sibling establish of a member cell is NOT also suppressed (it must show up as non-self).
- hunt-2 (all-or-nothing at resolution): find a member whose per-member argv resolves to a
  single establish of the WRONG cell (a value-plane mis-substitution under the member override) —
  e.g. an interaction between the member override and a SECOND var the body reads, or field-
  splitting under the override. Does `record_member_sites` clone the RIGHT incoming state (the
  site's, not the head's)? Does overriding the for-var perturb another var's resolution?
- hunt-3 (dups + order): `for x in a b a` — three members, cells [a, b, a]. Does the license
  treat the duplicate `a` correctly (two records `site N.0`, `site N.2` both for `a`)? Does a
  conflicting host (N.0 holds, N.2 absent for the same cell `a`) degrade correctly via the cli
  merge? (The merge keys by CELL, so two records for `a` merge — verify the conservative ⊤.)
- hunt-4 (the render): a Members body that is NOT a one-liner (the `do`/body/`done` on separate
  lines), or shares its line with a SIBLING command (`do apt-get install "$pkg"; echo done`), or a
  member whose elision leaves an empty arm. Does the in-situ splice stay `dash -n`-clean? (The
  corpus only exercises the one-liner shape.)
- hunt-5 (item-6a completeness): an `until` loop (not just `while`) post-loop `$?`; a `while` with
  a MULTI-command body (does the walk reach the LAST body command, or an earlier one?); a `while`
  with a multi-command CONDITION; nested `while`/`for` post-loop `$?`. The fix records ONE
  `body_exit` per while-merge — verify a multi-command body's `body_exit` is the LAST command.
- hunt-6 (eligibility holes): a body that reassigns the for-var via a shape `body_reassigns_var`
  misses — a `$(...)`-internal assignment (is it span-contained + caught?), an `eval`, a function
  call that reassigns it (functions are detached — does the call count as a write?), a `for` over
  the SAME var name nested (caught by ev-1, but verify). A missed reassignment ⇒ a wrong member
  family ⇒ a wrong elision (priority-1).
- hunt-7 (gate-1 parity vs the firewall): a member record carrying `rc=` that the `norm_parity`
  strip should remove (establish-class) vs a hypothetical Query member (none exist — Members are
  establishes). Verify no member record's rc leaks into the fold (the firewall is establish-only
  for members).

## §11 Confidence summary

- +SURE: the Members side-channel leaves the value-plane otherwise-unchanged; the family is
  all-or-nothing at resolution; the reaching-defs gens member cells (not Opaque); the self-reach
  suppressed-solve is correct where the naive cell-set check was wrong (demonstrated); item-6a is
  dash-verified and pinned both loop kinds; render reuses the F2 machinery `dash -n`-clean; the
  four cases exec-gate the four quadrants; goldens byte-stable except the one justified re-ground.
- ~SUSPECT: the singleton-member-family behavior change from L1 (correct, but a reader-trap);
  the self-reach N-solve cost (moot for spike); the coarse nested-loop refusal (sound, deferred).
- RESOLVED (was --WONDER): hunt-1 (a sibling-via-back-edge writer of a member cell) — I
  constructed the adversarial case (`for pkg in nginx curl; do apt-get install -y "$pkg";
  apt-get purge -y curl; done`, both reported converged) and the suppressed-solve CATCHES it:
  the sibling purge's gen is NOT suppressed (only the install's own is), so `curl#installed`
  reaches the install's in-state via the back-edge as a non-self writer ⇒ self-reach false ⇒
  the install RUNS. Pinned: `members_in_loop_sibling_writer_runs_despite_both_converged`. hunt-3
  (dups `a b a`) also spot-verified: ships `site 0.0`/`0.1`/`0.2`, the dup `a` a distinct member
  index, the cli merge keys by cell. The remaining hunt-list items (hunt-2/4/5/6/7) are NOT yet
  adversarially constructed — they are the crosscheck's job.
