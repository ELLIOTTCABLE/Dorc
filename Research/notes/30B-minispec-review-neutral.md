# 30B — minispec + dorc-verify: neutral review

> Tier: LLM-authored review note (Fable-class, neutral posture, no stake in the artifact).
> Subordinate to root docs, `spike/CLAUDE.md`, `minispec/CLAUDE.md` and `notes/301` (THE spec).
> Read-only pass at `d31378e8`: no product code touched, no gate run, no lane executed.
> Confidence per the house discipline (+SURE / ~SUSPECT / -GUESS / --WONDER).
>
> METHOD LIMIT, stated first because several findings hang off it: this review is STATIC
> READING ONLY. `lake build`, `cargo kani`, `verify:check`, `verify:report` and
> `verify:translate` were NOT run (the brief forbids it, and three of the five are
> Linux/WSL-only besides). Where a claim depends on what a tool would do, it is marked
> ~SUSPECT and the mechanism is spelled out so the human can settle it in one command.

## §0 — The verdict, compressed

The hard part is genuinely well done, and the soft part is where the rot is.

**Well done:** the derived-definitions linkage is real, not decorative
(`spike/verify/aeneas/src/lib.rs:21-25` `#[path]`-includes the shipping `sorted.rs` and
`lattice.rs`, and `minispec/Generated/Funs.lean:706-737` is a source-line-cited translation of
the actual `Flat::join` body). The three law statements are faithful readings of that derived
body, their hypotheses are honest and satisfied by the real production instantiations, and the
non-vacuity witnesses do real work. The badge design (independent set, typed absence, "a tier
that cannot look says so") is the right shape, and the Kani lane's by-name resolution against
`cargo kani list` is the right mechanism.

**The rot:** the two badges the corpus actually claims — `elaborated` and `interrogated` ×3 —
are checkable ONLY in an opt-in Linux/WSL lane, and that lane is currently
~SUSPECT-broken-and-green-anyway: the corpus root module imports a unit that was deleted
(`fnd-root-module-names-a-deleted-unit`), and the lane's own staging step never removes stale
files from a shared cache root (`fnd-lean-staging-never-removes`), so the deleted file is
~SUSPECT still sitting in the build root satisfying the import. On top of that the promote
generator does not exist, the committed report is stale in two ways and structurally
un-gate-holdable, the axiom census undercounts, and `proved` is a text-grep. Net: the ceremony's
advertised two-way refusal is, as built, unarmed for exactly the badges in play.

None of this is a soundness hole in Dorc. It is a halo hole in the instrument whose entire
stated purpose is preventing halos (`301:post-halo-is-the-hazard`).

## §1 — Faithfulness: does each law bind the algebra it names?

### `fnd-derived-linkage-is-genuinely-real` (+SURE) — the good news, first and unhedged

The linkage is not decorative, and the design choice that makes it real is the right one.
`spike/verify/aeneas/src/lib.rs:17-25` holds no algebra of its own; it re-aliases the crate
(`extern crate self as dorc_core`) and `#[path]`-includes `crates/core/src/sorted.rs` and
`crates/analysis/src/lattice.rs` verbatim. So the bytes charon compiles ARE the bytes that ship.
The emission carries source spans back to them (`minispec/Generated/Funs.lean:707`:
`Source: 'src/../../../crates/analysis/src/lattice.rs', lines 147:4-159:5`).

I checked the translation by hand against the Rust. `minispec/Generated/Funs.lean:709-737` is a
faithful case-for-case rendering of `spike/crates/analysis/src/lattice.rs:147-159`, including the
two details that matter: the `Elem/Elem` arm really does consult the equality dictionary and then
the clone (`Funs.lean:724-728`), and every arm that Rust spells `x.clone()` really does route
through the derived `Flat` clone rather than being optimized into an identity
(`Funs.lean:716-717`, `721-722`, `734-735`). The three statements name
`lattice.Flat.Insts.GeneratedLatticeLattice.join`, which is exactly the function the derived
instance installs as `join` (`Funs.lean:761-762`). A law over it is a law over the shipping body.

### `fnd-battery-never-instantiates-its-own-law` (+SURE) — the sharpest structural gap

Nothing mechanically connects a unit's `def <Slug> : Prop` to the evidence that earns its badges.

Concretely, in `minispec/Minispec/JoinIsCommutative.lean`: the law is `def JoinIsCommutative`
(lines 29-35); the anti-vacuity probe (lines 39-46) and all six battery `example`s (lines 49-72)
call `join u32Clone u32Eq …` directly and never mention `JoinIsCommutative`. Same shape in
`JoinIsIdempotent.lean:27-50` and `JoinIsAssociative.lean:32-84`.

Consequence: replace the `def` body with `join cl eqi a b = join cl eqi a b` and every badge is
unchanged — `elaborated` still earns (it elaborates), `interrogated` still earns (the battery is
green and the named probe exists), the report renders identically. The statement is reviewed by
humans only. That is a legitimate design position, but `301` §5 words `interrogated` as evidence
about the LAW ("the in-unit instance battery is green AND non-vacuous"), which it is not: it is
evidence about the SEAT FUNCTION, sitting in the same file as the law.

Cheap repair, and it costs one line per unit: state at least one battery entry as an
instantiation of the law's own `Prop` (e.g. a `theorem JoinIsCommutative_at_u32` obtained by
applying `JoinIsCommutative` at `T := U32` with the two lawfulness proofs already sitting in
`TrustedBase.lean:50-51`). Then a statement edit that decoupled the law from the seat would break
the file.

### `fnd-hypothesis-strengthening-is-mechanically-invisible` (+SURE)

`301` §1 names the in-unit battery as "the standing anti-laundering device: concrete witnesses
evaluate through whatever the vocabulary actually means, wherever it lives." As built, the
witnesses do NOT evaluate through the vocabulary: `LawfulClone` / `LawfulEq`
(`minispec/Minispec/Vocabulary/TrustedBase.lean:32-41`) appear in the `def … : Prop`s and in the
two U32 ceremony proofs, and NOWHERE in any battery entry.

So the laundering move the device exists to stop is available. Strengthening a hypothesis
narrows every theorem that carries it; add `∧ (∀ x : T, ∃ n : Aeneas.Std.U32, True)`-shaped
noise, or in the limit a T-restricting conjunct, and: `u32Clone_lawful` / `u32Eq_lawful` stay
provable (U32 is precisely the witness such a narrowing preserves), every battery stays green,
`elaborated` and `interrogated` stay earned, and the report is byte-identical. Only a human
reading the `Vocabulary/` diff catches it — which is exactly what "edits are ceremony"
(`minispec/CLAUDE.md:42-44`) asks for, so the ACCESS law covers this; the MECHANICAL claim in
`301` §1 does not, and should be softened rather than believed.

### `fnd-no-generated-freshness-check-exists` (+SURE that none exists; ~SUSPECT this instance is benign)

`minispec/CLAUDE.md:50-52` and `301` §3 rest the derived-definitions integrity on "committed so a
regeneration diff is a reviewable drift alarm." A diff is an alarm only if somebody regenerates.
Nothing in the tree compares committed `Generated/` against the sources it came from: the cheap
gate walks `Generated/` for holes and axioms only (`spike/verify/src/check.rs:69-79`), and there
is no digest, no mtime check, no recorded source revision.

At `d31378e8` the sources HAVE moved since the last regeneration: `minispec/Generated/` was last
written by `b9d91fec` (2026-08-14 22:36), while `crates/core/src/sorted.rs` and
`crates/analysis/src/lattice.rs` were both edited by `e114b597`/`afa8df9e`/`ddf60300`
(2026-08-15). I checked the diff: all 260 added lines are inside `#[cfg(kani)] mod kani_support`
(`spike/crates/analysis/src/lattice.rs:348-349`, `spike/crates/core/src/sorted.rs:336-337`),
which charon compiles without `--cfg kani`, so ~SUSPECT a regeneration would be byte-identical
and this particular drift is benign. The point is that nobody can tell that from the repo — and
the only instrument that could is Linux/WSL-only.

Cheapest real fix, and it rides the cheap gate on both legs: hash the `#[path]`-included source
files at translate time, commit the digest beside `Generated/`, compare in
`check::run`. `301` §5's own principle ("evidence is recomputed at whatever tier runs it — never
a hand-updated cache") is satisfied: a digest is not a verdict, it is an identity.

### `fnd-translation-unit-has-no-cheap-rot-check` (+SURE)

The Kani lane opens with a toolchain-less `cargo check` of its detached harness crate precisely
because nothing else compiles it (`spike/verify/src/kani.rs:273-297`, `rot_check`) — good, and
`spike/CLAUDE.md` calls it "the only compile standing between a `core`/`analysis` signature
change and a silently-rotted battery."

The aeneas translation unit has the same detachment (`spike/verify/aeneas/Cargo.toml:24`, empty
`[workspace]`) and NO equivalent. `aeneas:check` exists (`spike/verify/aeneas/mise.toml:62-65`)
but has no root wrapper and rides no gate. So a `lattice.rs` edit that breaks the shim's
arrangement — a new `use` of a third crate, a name the `extern crate self as dorc_core` aliasing
cannot resolve — rots invisibly, and rots the artifact the LAWS ARE STATED OVER rather than a
harness battery. This is the more load-bearing of the two units and it is the one without the
guard.

### `fnd-seat-citation-ignores-everything-between` (+SURE)

`spike/verify/src/seat.rs:28-59` splits the citation and uses only the FIRST segment after the
crate as the module file and the LAST as the function name (`seat.rs:33-36`). For
`dorc_analysis::lattice::Lattice::join` the `Lattice::` segment is discarded entirely:
`dorc_analysis::lattice::AnythingWhatsoever::join` resolves identically, and a nested module
path (`a::b::c::fn`) cannot resolve at all. `declares_fn` (`seat.rs:74-81`) matches the trait
SIGNATURE at `spike/crates/analysis/src/lattice.rs:39` and cannot distinguish it from any impl's
definition.

The check still earns its keep as advertised (it goes red when a rename moves `fn join` out of
`lattice.rs`), but "toolchain-resolved pairing over string-matching" is the Kani lane's property,
not this one's. The seat citation is a filename+function grep, and its doc-comment
(`seat.rs:5-8`) should say so.

### `fnd-verified-boundary-overreads-its-own-census` (+SURE)

`minispec/REPORT.md:59-63` renders "The verified boundary — the subsets of the analysis engine
opted into Lean verification" as one entry: `dorc_analysis::lattice::Lattice::join`. That is a
TRAIT method with six implementors in `lattice.rs` (`Powerset`, `Flat`, `Product`, `MapL`, `May`,
`Must`). The evidence is `Flat`-only, and it is statements-plus-instances, not proofs.

A reader of the one artifact aimed at non-proof-literate reviewers is told the join seat is
inside the verified boundary. The honest census entry is the implementation, not the trait —
`dorc_analysis::lattice::Flat::join`, or the trait method qualified by implementor. Note this
interacts with `fnd-seat-citation-ignores-everything-between`: the resolver cannot currently tell
the difference, so fixing the render means fixing the resolver's segment handling too.

## §2 — Honesty of hypotheses, and non-vacuity

### `fnd-trusted-base-is-visible-and-discharged` (+SURE) — this part is honest

`TrustedBase.lean` is the good half of the artifact. The two hypotheses are named, carry their
own prose (`TrustedBase.lean:3-22`), state exactly what they assume (`clone` is the identity and
total; `eq` decides propositional equality and is total — `lines 32-41`), explain WHY the
assumption is needed (the translation models `T: Clone + Eq` as records of opaque functions), and
are discharged outright for the battery ground (`u32Clone_lawful` / `u32Eq_lawful`,
`lines 50-51`). Every unit's prose points at the file (e.g.
`JoinIsCommutative.lean:17-20`). This is fine print rendered as headline; I have no complaint
about the framing.

I also checked whether the hypotheses are SATISFIED by the real production instantiations, which
is the question that decides whether the theorems bind anything. `Flat<T>` is instantiated in
production at exactly two types: `Flat<String>` (`spike/crates/analysis/src/value.rs:218`) and
`Flat<Binding>` (`spike/crates/analysis/src/funcenv.rs:289`). `Binding` is
`#[derive(Debug, Clone, Copy, PartialEq, Eq)]` (`funcenv.rs:279`) and `String`'s are the
standard ones — both have identity clones and propositional-equality `Eq`. +SURE the hypotheses
hold for both. The conditional theorems therefore have real content at the real call sites.

### `fnd-unsatisfiable-hypothesis-would-earn-every-badge` (+SURE about the mechanism; forward risk, not a live defect)

The batteries are ALWAYS at U32 (`TrustedBase.lean:46-48`, shared by every unit), never at a
law's actual production instantiation. So the machinery cannot distinguish a law whose hypothesis
is satisfied in production from one whose hypothesis is FALSE there: both elaborate, both have a
green U32 battery, both earn `interrogated`, and a proof of the conditional would earn `proved`
while binding nothing that runs.

This is not hypothetical in this codebase. The Eq-excluding pattern is already house practice —
`spike/CLAUDE.md`'s `collapse-mints-narrative` has narrative records "Eq-EXCLUDED from lattice
equality" for fixpoint termination, and `300:fnd-reach-lattice-outside-scope` records
`analysis::effect::Reach` holding a raw `BTreeSet` behind "a hand-written cause-excluding `Eq`".
`LawfulEq` is FALSE for any such type. The first enrichment law stated over a seat whose
production element type carries a deliberately-partial `Eq` will be vacuous-in-practice and
badge-green, silently.

Mitigation to consider when enrichment is chartered: require each law to name the production
instantiation(s) its hypotheses are claimed to hold at, and battery it THERE rather than only at
U32. A `Flat<Binding>` battery is as cheap as the U32 one.

### `fnd-error-channel-claim-overstates-what-is-shown` (+SURE)

`JoinIsCommutative.lean:9-10` claims the law covers "the error behaviour: neither order can fail
where the other succeeds", and `JoinIsAssociative.lean:13-16` says the monadic comparison means
"nothing about `Result` is assumed away."

Under the hypotheses actually carried, the derived `join` cannot fail AT ALL: the only
`Result`-producing calls in the body are `clone` and `eq` (`Funs.lean:716-735`), and
`LawfulClone`/`LawfulEq` assert both always return `ok`. So the error channel is not
under-constrained, it is unreachable — the hypotheses assume it away, which is the opposite of
what the prose says. The monadic SHAPE is honest (the statements do not project out of `Result`);
the claim that the laws say something about failure is not. Two words of prose, and worth fixing
because this file's whole job is to be the honest English.

### `fnd-nonvacuity-probe-is-a-naming-convention` (+SURE)

`spike/verify/src/unit.rs:161` computes `has_nonvacuity_probe` as
`text.contains(&format!("theorem {slug}_nonvacuous"))`. `Statement::Stated` is likewise
`text.contains(&format!("def {slug} : Prop"))` (`unit.rs:156`). So "the anti-vacuity probe" is,
mechanically, a theorem with the right NAME that the Lean build accepts;
`theorem JoinIsCommutative_nonvacuous : True := trivial` earns `interrogated` in full.

The actual probes are real — I checked all three. `JoinIsIdempotent_nonvacuous`
(`JoinIsIdempotent.lean:37-40`) is a genuine instance at the arm that consults both dictionaries;
`JoinIsAssociative_nonvacuous` (`JoinIsAssociative.lean:47-60`) really does drive the two
groupings through DIFFERENT intermediates (`Elem 1` left, `⊤` right) to the same answer, which is
the non-trivial family. `JoinIsCommutative_nonvacuous` (`lines 39-46`) is the weakest of the
three: it is two distinct elements joining to ⊤ in both orders, which the six battery examples
below it already cover; it witnesses non-degeneracy but adds nothing over the battery.

The point is not that the probes are bad — they are good — but that their quality is a human
property, and `evidence.rs:148-166`'s doc-comment ("a green battery with no positive witness is
green vacuously") describes a guarantee the name-check cannot deliver. Under
`301:law-spec-touch-frontier-human-only` that may be an acceptable division of labour; it should
be WRITTEN as one.

## §3 — Badge integrity and the ceremony

### `fnd-root-module-names-a-deleted-unit` (+SURE about the file; ~SUSPECT about the build)

`minispec/Minispec.lean:13` imports `Minispec.LeqIsReflexive`. That file does not exist and has
not existed since `b0d709aa`, which renamed it to `JoinIsAssociative.lean`. `Minispec.lean` has
not been touched since `bad20a65` (the stub skeleton). The same file therefore also does NOT
import `Minispec.JoinIsAssociative` — the third of the three catalogued laws.

~SUSPECT (high) that `lake build` fails on a clean cache: `minispec/lakefile.toml:2` lists
`Minispec` in `defaultTargets`, `lakefile.toml:22`'s glob `Minispec.+` includes the root module
itself, and an import with no corresponding file is a build error, not a warning. I could not run
lake to confirm.

Two secondary observations. First, the file's own docstring ("Importing every unit is what makes
`lake build` a whole-corpus check rather than a per-file one, so a unit that stops elaborating
cannot hide behind a neighbour", `Minispec.lean:4-6`) is contradicted by
`lakefile.toml:18-19`'s comment ("`Minispec.+` takes the root and every submodule, so a new unit
needs no edit here — one less thing between an author and a law"). Both cannot be the mechanism.
The glob is the real one, so `JoinIsAssociative` does still elaborate; the redundant hand-list is
what drifted, and the fix is to delete the import list rather than repair it. Second, the ONE
cheap check that would have caught this — units on disk versus units the root module imports — is
absent from `check.rs`, which checks units against the CATALOGUE in both directions
(`check.rs:40-52`) but never against `Minispec.lean`.

### `fnd-lean-staging-never-removes-stale-files` (+SURE about the code; ~SUSPECT this is why the lane read green)

`spike/verify/src/pipeline.rs:208-231`: `stage` does `create_dir_all(build_root)` and then
`copy_tree`, which creates directories and copies files and NEVER removes anything the source no
longer has. The build root defaults to a SINGLE shared path,
`$XDG_CACHE_HOME/dorc-minispec-lean` (`spike/verify/src/lib.rs:50-61`), shared across every
worktree on the box.

Two consequences, and the second is the one that bites:

1. Cross-worktree contamination. Worktree A's staging leaves its files in the root; worktree B
   stages over them and builds a UNION of the two corpora. For the one lane that computes
   `elaborated` and `interrogated`, in a repo whose whole fleet lives in
   `.claude/worktrees/*`, that is a live channel.
2. A deleted or renamed unit keeps satisfying imports forever. `LeqIsReflexive.lean` was staged
   into that root by the green build recorded in `304` §5 (measured at the stub stage, when the
   file still existed and the root import was correct), and nothing has removed it since. So
   ~SUSPECT the "lake green" claim banked in `300` §2 after the trio was authored was measured
   over a tree containing a file the repository does not contain — which is exactly how
   `fnd-root-module-names-a-deleted-unit` survived a green lane.

Fix is three lines: remove the build root's `Minispec/` and `Generated/` before copying (or
`remove_dir_all` the staged sources, keeping `.lake`), and key the default root by worktree.
The `.lake` skip in `copy_tree:221` shows the author already knew which subtree must survive.

### `fnd-earned-badges-are-unchecked-on-every-gate-that-runs` (+SURE)

At `Tier::Cheap`, `elaborated` and `interrogated` return `Evidence::NotAtThisTier`
(`spike/verify/src/evidence.rs:67-82`, `94-99`), and `NotAtThisTier` agrees with EVERY
expectation by construction (`spike/verify/src/badge.rs:139-149`). That is the right design — a
tier that did not look must not confirm — but it means the only two badges the catalogue actually
claims Earned (`spike/verify/src/catalogue_lock.rs:21-28`, `36-43`, `51-58`) are compared against
nothing on any gate that runs by default.

The cheap gate DOES ride the ordinary suite — via `spike/verify/tests/corpus_is_coherent.rs:13`
rather than via the `verify:check` task, which appears in no `gate:*` run list (`mise.toml:246`,
`361`); the task's own description claims it "rides `gate:full-quiet`", which is true only
through that test, and worth a word in the description. What rides is coherence, slugs, seats,
bindings, holes. What does not ride is the two Earned claims.

The lane that would check them is `verify:lean` / `verify:report --with-lean`: opt-in, Linux/WSL
only (`pipeline.rs:149-155` refuses on Windows outright), gigabytes of dependency store, and — per
the two findings above — ~SUSPECT currently green over a stale staged tree. So the composite
state at `d31378e8` is: six Earned claims standing, zero of them checkable by anything a Windows
developer or the ordinary gate can run, and the one checking lane ~SUSPECT unable to see the
corpus's actual breakage.

### `fnd-promote-generator-does-not-exist` (+SURE)

`spike/verify/src/catalogue_lock.rs:1-8` opens `@generated by dorc-verify promote — do not edit
by hand` and asserts "Every row's badge expectation was COMPUTED from evidence at promote time."
There is no `promote` subcommand: `spike/verify/src/bin/dorc-verify.rs:19-33` dispatches
`check`, `report`, `materialize`, `lean-build`, `kani`, and nothing else. The file is hand-edited
(`300` §2 records this honestly as `fnd-promote-subcommand-missing`).

So `301` §5's "The promote act IS the ceremony; there is no other" is currently: a hand-edit to a
file whose header forbids hand-edits, asserting a provenance nothing produced. The claim in the
header is the part I would fix first — it is the only place a reviewer learns what the numbers
mean, and it is false. Either build the generator or restate the header as "hand-promoted; review
is the git diff, and the with-engines lane is what checks it."

### `fnd-proved-is-a-text-grep-at-every-tier` (+SURE)

`Badge::Proved` is the one badge declared not to need an external engine
(`spike/verify/src/badge.rs:68-69`), and `evidence::proved` (`evidence.rs:131-146`) is: the
claimed file reads, it contains the string `theorem <Slug>_holds`, and `contains_hole` is false.
No tier consults the Lean build for it — not even `WithEngines`.

So `proved: Earned` can stand, on both platform legs and on the ordinary gate, for a proof file
that does not compile. The doc-comment concedes the shape ("Its Lean-checkedness rides
`elaborated`'s build", `evidence.rs:62-65`) but that is a DIFFERENT badge, itself
`NotAtThisTier` on every gate that runs, so the ride does not happen. Of the six badges this is
the one a reader will weigh most heavily, and it is the one with the weakest evidence path.
Minimal repair: make `Proved` require the Lean verdict (`needs_external_engine → true`), keeping
the cheap tier's text check as a REFUSAL-only signal (a claimed proof that is missing or holed
can still fail cheaply; only "earned" needs the engine).

No live exposure today — all three rows carry `proof: None` and `Minispec/Proofs/` holds only
`README.lean` — so this is a trap laid for the first proof, not a current lie.

### `fnd-committed-report-is-stale-and-cannot-be-gate-held` (+SURE)

`minispec/REPORT.md` is stale in two independent ways, and both defeat the drift alarm
`dorc-verify.rs:141-155` was built to be:

1. It embeds ABSOLUTE paths from a DIFFERENT worktree — `REPORT.md:14`, `30`, `46` all read
   `C:\Users\ec\Sync\Code\Dorc\.claude\worktrees\r30-conduct\minispec\…`. Source:
   `spike/verify/src/report.rs:56` renders `u.path.display()`, and `unit::load_all` builds those
   paths from `repo_root()` (`unit.rs:95`). The render is therefore worktree-dependent, so the
   staleness comparison FAILS in every worktree except the one that wrote it. The design intent
   ("the report is the drift alarm", `dorc-verify.rs:141-142`) is structurally unrealizable until
   the path is made repo-relative — `seat.rs:89-93` already has the `relative()` helper, and
   `binding.rs:92-97` its twin.
2. The `pinned` rows read `absent(seam-kani-pairing-unbuilt: no paired harness)`
   (`REPORT.md:22`, `38`, `54`). The current code cannot produce that string: at cheap tier
   `pinned` is `NotAtThisTier` → `not-recomputed-here` (`evidence.rs:83-86`), and the only
   `pinned` absence string in the code is a bare `"no paired harness"`
   (`evidence.rs:117`). `seam-kani-pairing-unbuilt` survives nowhere in `spike/` — the Kani lane
   closed it (`300a` §3). So the committed report predates the tier refactor.

Both mean `mise run verify:report` refuses today. Nothing notices, because `verify:report` rides
no gate and no test asserts the report is current — the ONE derived artifact in this system that
`301` §5's organizing principle ("everything else is derived and gate-checked") does not
gate-check. Once the paths are relative, a five-line test in `corpus_is_coherent.rs` closes it
permanently.

(Minor, mentioned because it is a committed artifact: those lines also publish the human's home
directory path. Not a secret; still noise that a repo-relative render removes for free.)

### `fnd-pinned-reports-absence-of-an-existing-harness` (+SURE)

All three rows carry `harness: None` (`catalogue_lock.rs:19`, `34`, `49`), which renders as "no
paired harness". But the harnesses exist and are the obvious pairings:
`flat_obeys_the_binary_laws` (`spike/verify/kani/src/harness/lattice_laws.rs:124-127`) proves
commutativity and idempotence over `Flat<u8>`'s whole domain via
`merges_commute_of` (`lattice_laws.rs:67-70`) and `one_value_laws_of` (`lines 54-63`), and
`flat_is_associative` (`lines 130-132`) proves associativity.

The direction is safe (understatement, not ambition) and `todo` is an honest expectation, but the
EVIDENCE string is factually wrong about the world: a reader is told no harness exists. Given the
lane's pairing machinery is built and resolves by name against `cargo kani list`
(`kani.rs:199-208`, `446-463` — a good mechanism, and its test at `kani.rs:482-505` pins that a
commented-out `#[kani::proof]` does NOT resolve), three promotes appear to be sitting there for
free. That is a human call, not mine; but the string should distinguish "the catalogue cites no
harness" from "no harness exists".

### Smaller integrity notes

- `fnd-undeclared-binding-direction-untested` (+SURE) — `binder_refuses.rs` pins `Unaccepted`
  (`lines 67-83`) and `Missing` (`lines 86-102`) but not `Undeclared`, which
  `binding.rs:5-7` names as the failure that actually happens ("evidence that quietly walked
  away"). `binding.rs:145-153` implements it; nothing exercises it.
- `fnd-proofs-directory-is-uncensused` (+SURE) — the hole census covers `Generated/`
  (`check.rs:69-79`), `Minispec/*.lean` (`check.rs:91-97`) and `Minispec/Vocabulary/*.lean`
  (`check.rs:59-67`), but never `Minispec/Proofs/`: it is reached only via a catalogue row's
  `proof` path (`evidence.rs:142`). A holed proof file that no row cites is invisible to the
  cheap gate; `lake build` would emit `declaration uses 'sorry'` into `Built.dependency_holes`
  (`pipeline.rs:165-170`), which — see below — does not reach the report either. No live risk
  (only `README.lean` is there), one-line fix.
- `fnd-unit-walk-is-single-level` (+SURE) — `unit::load_all` (`unit.rs:94-107`) reads only the
  top level of `minispec/Minispec/`. A unit filed in a new subdirectory would be built by the
  lake glob and be invisible to the binder, including to the "not in the catalogue" check.
- `fnd-bare-harness-names-may-collide` (~SUSPECT, latent) — `kani.rs:468-475` keys citations on
  the LAST path segment, so two same-named `#[kani::proof]` fns in different harness modules
  would alias into one. I checked: no duplicates exist across the three modules today.

## §4 — Coverage honesty: is what minispec does NOT cover visible?

Inside the corpus, yes, and creditably. `minispec/CLAUDE.md:23-30` states the remit as "the
ABSOLUTE MINIMUM provable surface … basic, zero-controversy mathematics with no Dorc design
content, chosen so the process, praxis, gates, and habits get built on terrain that cannot
generate design emergencies", and says enrichment is a standalone human-led item. The units' prose
names its hypotheses. `301` §0's `post-halo-is-the-hazard` names the failure mode outright. The
badge set renders absence with the same weight as presence, and `Expectation::Excepted(reason)`
exists for typed non-coverage (`badge.rs:92-94`) even though nothing uses it yet.

The gap is that ALMOST NONE of that reaches `minispec/REPORT.md` — the one generated artifact
whose stated audience is "reviewers who are not proof-literate"
(`spike/verify/src/lib.rs:5-7`).

### `fnd-report-has-no-badge-legend` (+SURE)

`REPORT.md` names six badges and marks them earned/todo with never a word about what any of them
MEANS. The definitions live in Rust doc-comments (`badge.rs:13-33`) and in `301` §5 — neither of
which the stated audience reads. A reader sees `elaborated: earned` and `interrogated: earned`
for three laws; the truth is "the statement typechecks, and twelve concrete U32 examples
evaluate as expected." Those are very different impressions, and the gap between them is
precisely the halo. A generated legend is a dozen lines of `report.rs` and it is the single
highest-value fix in this review.

### `fnd-report-omits-its-own-trusted-base` (+SURE)

`REPORT.md:65-69` is titled "The trusted base" and lists two numbers: `Generated/` proof holes,
and external axioms. It omits:

- **The statement-level hypotheses.** Every one of the three laws is conditional on
  `LawfulClone` and `LawfulEq`. That is the trusted base OF THE CLAIMS, and it is the entry a
  reviewer most needs. It appears nowhere in the report.
- **The dependency-closure holes.** `pipeline.rs:175-183`'s own doc-comment says the number is
  "lifted into the report rather than left in scrollback" — it is not. `report::Census`
  carries only `{holes, axioms}` (`report.rs:133-140`); the count is `println!`ed by the CLI in
  the with-lean lane only (`dorc-verify.rs:94-102`) and the committed report is cheap-tier by
  policy (`dorc-verify.rs:61-71`). `300` §2 records four such holes in aeneas's own Lean library.
  Anything proved through a holed declaration is not proved, and the number is invisible in the
  artifact that exists to make such things visible.

### `fnd-axiom-census-undercounts-by-three` (+SURE, verified by hand)

`pipeline.rs:100-104` counts axioms as lines whose trimmed form `starts_with("axiom ")` — with a
trailing space. aeneas wraps long axiom names onto the following line, leaving `  axiom` alone on
its own line, which does not match. Measured on the committed tree: `FunsExternal.lean` has 14
axioms of which 11 match, plus 2 in `TypesExternal.lean` — 16 real, 13 reported. The three missed
are `FunsExternal.lean:103`, `126`, `133` (`SortedSet::into_iter`, and the `SortedMap::iter`
closure's `call_once` / `call_mut`).

The error is in the unsafe direction: the report understates the trusted base. It is also the
single number the report offers as its integrity measure ("a green build proves nothing without
this number", `REPORT.md:67` — the same sentence applies to getting the number right). Fix: count
`axiom` as a whole word at line start, or count `@[rust_fun]`/declaration heads.

### `fnd-hole-number-counts-files-not-holes` (+SURE)

`pipeline.rs:96-99` increments `holes` once per FILE that contains any hole, but the report
renders it as "`minispec/Generated/` proof holes: **0**" (`REPORT.md:67`) and the refusal text
says "carries {holes} proof hole(s)" (`check.rs:73-77`). Currently 0, so nothing is wrong today;
the moment it is nonzero the number is wrong and reads low. Either count occurrences or rename
the field to what it measures.

### `fnd-fence-pointer-explains-half-the-axioms` (+SURE)

`REPORT.md:68` sends a reader to `spike/verify/aeneas/Cargo.toml` "where every entry carries its
class and its reason." That fence lists five `opaque` patterns, all EXITS
(`aeneas/Cargo.toml:47-53`), which account for seven of the sixteen axioms. The other nine —
`core::fmt::Formatter::debug_tuple_field2_finish`, the tuple `Debug`/`PartialEq` impls,
`Vec::remove`, `Vec::is_empty`, `MaybeUninit`, and the `SortedMap::iter` closure type and its two
call impls (`FunsExternal.lean:23-65`, `126-137`; `TypesExternal.lean:21-25`) — are aeneas's own
unmodelled std, present in no fence and carrying no class or reason anywhere in the tree. The
sentence is false for the majority of the entries it claims to cover.

Worth adding, because it is the good news the report also fails to tell: NONE of the sixteen
axioms is reachable from the three current laws. `Flat`'s join touches only the derived `Flat`
clone and the `T` dictionaries. So the report simultaneously overstates the trusted base's
documentation and overstates its relevance — a per-law axiom dependency (or one sentence saying
"no catalogued law currently depends on any of these") would be strictly more informative than
the global count.

### `fnd-report-does-not-state-the-remit` (+SURE)

`REPORT.md` never says that the corpus is deliberately three claims of textbook lattice algebra
with zero Dorc design content, nor that no law is proved, beyond three per-law `proved | todo`
cells a reader must aggregate themselves. `minispec/CLAUDE.md:23-30` says it well; the report is
where a non-proof-literate reviewer will actually look. One generated paragraph, derived from
data already in hand (`LAWS.len()`, how many rows carry a proof, how many carry a binding), and
the halo largely closes.

## §5 — Ranked repair list (my read of cost versus halo-reduction)

1. `Minispec.lean:13` — delete the hand-written import list (the lakefile glob is the real
   mechanism) or repair it; add the units-versus-root-module check to `check::run`.
2. `pipeline::stage` — clear the staged sources before copying, and key the build root per
   worktree. Until this lands, no `verify:lean` result should be trusted, including the ones
   already banked.
3. `report.rs:56` — render the statement path repo-relative; then gate-hold report currency with
   a test in `corpus_is_coherent.rs`. These two are one change: the second is impossible without
   the first.
4. `report.rs` — add a generated badge legend, the remit paragraph, the two statement-level
   hypotheses, and the dependency-hole count.
5. `pipeline.rs:100-104` — fix the axiom count; correct or narrow the fence pointer.
6. `evidence.rs` — make `Proved` engine-gated for "earned"; distinguish "catalogue cites no
   harness" from "no harness exists".
7. Per-unit — instantiate each law's own `Prop` at least once in its battery, so the statement
   and the evidence are mechanically coupled.
8. Add a cheap `cargo check` rot-guard for `spike/verify/aeneas/src/lib.rs`, mirroring
   `kani::rot_check`; add a source digest beside `Generated/`.
9. Soften the three claims that outrun their mechanisms: `301` §1's anti-laundering role for the
   batteries, `catalogue_lock.rs`'s `@generated` header, and the units' `Result`-error prose.

Items 1-3 are the ones I would not ship another badge claim without.

## §6 — What I could not verify, explicitly

- Whether `lake build` fails at `d31378e8` (the mechanism is spelled out in
  `fnd-root-module-names-a-deleted-unit`; one `mise run verify:lean` from a CLEARED build root
  settles it, and clearing the root is the load-bearing half of the test).
- Whether the committed `Generated/` regenerates byte-identically (one `verify:translate`).
- Whether `verify:check` is green (I read the checks; I did not run them).
- Whether the three Kani `Flat` harnesses are green at HEAD.
- Whether `dorc-verify report` refuses (I am +SURE the committed text cannot be reproduced by
  the current code path; I did not execute it).
- Anything about Verso, mutation testing, reach instrumentation, or the assertion-subset
  machinery: all named seams, all unbuilt, all honestly rendered as `todo`.

## §7 — The parts I would keep unchanged

Stated because a review that lists only defects mis-scales them.

- The `#[path]`-include translation unit. It is the difference between a law about the code and a
  law about a transcription of the code, and it was the right call.
- `Evidence::NotAtThisTier` and its "agrees with everything" rule (`badge.rs:116-149`), with the
  test that pins it beside the cases that DO refuse
  (`corpus_is_coherent.rs:60-74`). Getting this right is what keeps the cheap gate from becoming
  a rubber stamp for the expensive one, and it is right.
- The badges-as-an-independent-set decision, and typed absence rendered with the weight of
  presence.
- The Kani lane's by-name resolution against the toolchain's own harness list, and its refusal
  trichotomy (`dorc-verify.rs:216-263`): green / finding / could-not-run, with an over-budget
  harness counted as UNJUDGED rather than passed.
- `binder_refuses.rs`'s posture — refusals tested off throwaway fixtures, never off the real
  corpus, with the reason stated in the module doc.
- `TrustedBase.lean` as a piece of technical writing. It says what it assumes, why it must
  assume it, and proves it where it can.
- The access split itself (`301:law-spec-touch-frontier-human-only`). Several findings above
  reduce to "the mechanism is a naming convention and the content is human-reviewed" — which is
  exactly what an external acceptance surface the worker cannot write to is FOR. The
  recommendation is to say so in the artifacts, not to mechanize it.
