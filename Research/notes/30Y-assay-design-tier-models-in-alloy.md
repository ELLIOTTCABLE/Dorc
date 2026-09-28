# 30Y — assay: design-tier models in Alloy

> Tier: LLM-authored plan (Fable conductor, from the 2026-09-27 sittings; round 30). Subordinate
> to the root docs and `spike/CLAUDE.md`; sibling of `notes/301` (minispec and dorc-verify, the
> code-tier instrument this extends downward to the design tier) and of `notes/30X` (the testing
> architecture the correspondence half will eventually ride). Its ledger is `notes/30Ya`; its
> evidence base is `.claude/research/design-model-mechanisation-prior-art/` (four fronts, every
> source graded; the worked strawman is `strawman-2/` there). Grades: **[TYPED]** the human typed
> it · **[ACKED]** confirmed in dialogue · **[CONDUCTOR]** conductor-derived, unratified. Names
> marked STRAWMAN rename freely (`rul-strawman-formats-no-compat`). This document is about the
> tooling only: what assay reads, what it builds, what it runs, and what it reports. How a spec
> document is written (where normativity sits, what a fence may contain, the spec tier's
> editing posture) is not here.

## § 0 — what-assay-buys

The name: an assay is the test that tells you what an alloy is made of. **assay** (STRAWMAN) is
a compiler and checker that reads a literate specification document, builds Alloy 6 modules from
its fences, runs them, and holds the verdicts in a lock. It exists for two payoffs, both typed by
the human [TYPED]:

- **`buy-regression-before-a-human-reads-it`** — a design statement made precisely once, and a
  later rewrite, composition, or assumption that reads it wrong, is caught mechanically. The
  check runs as a gate an LLM invokes before showing a claim to a human, as a cheaper sibling of
  a full adversarial review for that one class of error. The three crosscheck rounds over the
  identity model found two populations of defect, and the second, text-drift under folds and
  rewrites, is the one that does not narrow with rounds and scales with edit volume. That is
  the population this instrument is for.
- **`buy-one-input-across-checkers`** — the unit that pins a behavioural claim is a few lines of
  sh with an expected verdict per line: a book. The same book, unrewritten, is the design tier's
  test today and the product's own input at the field tier later, so "the design says elide and
  the product says guard" is a diff over one file, never a translation between two.

What falls out for free once those two are built, each observed in the prior art and each
priced by the strawman: attribution is computed, never authored, so a survival's "rested on"
set is a lock column and a wrong one is a finding; a refuted shape (the `311u` register, the
`GOTCHAS` list) becomes a run that re-runs on every edit instead of a paragraph nobody re-reads;
the cost of a rewrite is countable as changed lock rows per claim; and the strongest tool the
panels lacked, "no answer changed" after a rewrite, becomes a lock diff of zero.

Posture [TYPED]: tracked, not proved. Bounded model-finding over hand-written models of the
prose, at the prose's own altitude (objects, relations, laws; no spellings, no UX, no code).
Never user-facing, never marketed. The instrument for the design tier the way Kani and minispec
are the instruments for the algebra tier; the seam between them is the book, not a translation
(§ 1.3).

## § 1 — the-instrument-alloy-six

### § 1.1 why-alloy

Alloy fits a pre-code relational model better than anything else surveyed [CONDUCTOR, front 3
of the research round]: the identity model's statements are sets, relations, multiplicities,
and universally quantified laws, which is Alloy's whole language; the small-scope hypothesis
holds empirically here, since every panel witness to date fits in four to six atoms (two
directory entries and one inode, two filesystems on one disk, one stranger's key); a check takes
seconds; a counterexample is a concrete small world a human can read; Alloy 6 adds a temporal
mode (`var` sigs, `after`, `always`) that the chronology work will want (§ 4); and the
distribution is one jar on a JVM, runnable headless on every platform this project builds on,
with a documented Markdown mode that runs fenced `alloy` blocks straight out of a `.md` file.
Alloy has been translated into provers five times, each a one-off, and the mismatch lands every
time on exactly the features 311 leans on (bounded integers, finiteness, multiplicities), so no
structural tie from the model to Kani or Lean is attempted (§ 1.3).

### § 1.2 limitations-stated-plainly

- **Bounded.** "No counterexample at scope 6" is not a theorem. The report prints the scope
  beside every verdict, never the word "proved"; the lock stores the scope with the result; a
  larger scope runs in an opt-in lane. The subset-quantified laws (§ 2.4) are the expensive
  ones; the strawman's guess is a ceiling near scope five or six for them, to be measured.
- **Fixed atoms fix scope minima.** A `one sig` claim atom is an atom in every universe that
  opens its module, so a check opening the claim corpus needs a scope at least the corpus's
  size, and a law meant to hold for all speech must not open the corpus at all. The module split
  in § 2.2 exists for this reason [CONDUCTOR].
- **Higher-order quantification only where Alloy skolemizes it.** A `check` with top-level
  `all S: set MDecl` negates to an existential Alloy can solve; nesting a set quantifier under
  another quantifier does not. The six generic laws are written in the solvable shape; a
  spec-authored law that is not gets Alloy's own error, surfaced verbatim.
- **No vacuity check built in.** A `check` whose premise is unsatisfiable passes. Every law gets
  a satisfiable twin (§ 2.4), and a book `run` that finds no instance is a failure, never a pass
  (the class of bug the panels hit three times, a universal over an empty set read as safe).
- **Integers unused.** Nothing in the design tier counts; the bitwidth stays at Alloy's default
  and no model may depend on it. A model that needs to count has left this instrument's remit.
- **The model is the prose's model, not the code's.** Nothing here reads Rust. The tie to the
  implementation is the book (§ 1.3), and until the product's kernel reaches the design a book
  pins, that book's field-tier verdict is a recorded, qualified red, not a lie (§ 3.4).

### § 1.3 the-seam-is-the-book-not-a-translation

Every found tie between a model-finder and a prover shares a statement only when the finder runs
inside the prover's logic or both read one language; nothing found ties an Alloy-family model to
Kani, CBMC, or Rust [front 3]. The project's proof tier consumes definitions derived from the
shipping code (`301`), so a design-tier model and a code-tier law never share a statement, and
any attempt to make them would put the incorrectness at the seam. Instead the shared unit is the
one the referent projects converge on: an input with a stored expected verdict, run unchanged
against each tier [front 4]. Here that is the sh book. The design tier reads it into Alloy; the
field tier will run it on a host; the lock records both verdicts side by side and disagreement is
triaged, not auto-resolved (§ 3.4).

## § 2 — what-assay-builds

### § 2.1 the-input

A spec document is Markdown with two fence kinds assay reads and nothing else it interprets:

- `alloy` fences: the model, verbatim. Species sigs with the facts that say what a claim of that
  species means when true; claim atoms as `one sig <speaker>__<long_name> extends <Species>`
  with their fields; the spec's own functions, predicates, and checks.
- `sh` fences: books and oracle sets. A book is valid sh: each concrete line is followed by two
  comment forms assay lifts, `#}` binding the line's shell words to global names (shell-lexed;
  a `{word}` is a class) and `#=` carrying line-scoped Alloy lifted verbatim across consecutive
  `#=` lines with `this` bound to the site the line generates. A line whose `#=` holds no verdict
  form is a fixture line, a measurement at that point in the book, and binds no `this`. A `.`
  line loads speech. An oracle set is an `sh` fence of `.` lines headed by its file name, sourced
  by books.

The tool-owned side is two things. A **harness** module holding what is true of every spec by
construction: `Speaker`; `MDecl` with its `speaker`; `True in MDecl`; `Ans` with its declared
`weaker` order and the `Safe` and `Spares` subsets. And the **generic laws** (§ 2.4). Between the
harness and a spec sits a **shared layer** the generic laws call by name and the harness does not
define: `Site` (with `verb`, `arg`, `before`, and `speech`), `Query` (`w`, `r`), `answer[S, q]`,
`wrong[S, q]`, `restsOn[S, q]`, `by[s]`, and the three verdict subsets `Ran`, `Elided`, `Guarded`
of `Site`. The strawman spec supplies this layer by hand. Whether it stays in each spec, moves
to a module most specs open, or joins the harness is unruled [TYPED: a meaningful distinction,
not ruled]; what is ruled is that a spec must specify what it talks about, so assay checks the
signature by name and arity and refuses a spec that lacks a member.

### § 2.2 the-generated-modules

One spec document becomes one directory of Alloy modules [CONDUCTOR, STRAWMAN layout]:

- `species.als` — opens the harness; every `alloy` fence line that is not a claim atom: species
  sigs, meaning facts, the shared layer as the spec wrote it, the spec's functions and predicates.
- `claims.als` — opens `species`; every `one sig … extends <Species>` atom, and one named set per
  oracle-set fence (`fun tessa_fs: set MDecl { … }`), so a book's loads are set expressions.
- `laws.als` — opens `species` only, so the claim universe is free; the six generic laws plus
  every spec-authored `check` that carries a scope clause (§ 2.4).
- `corpus.als` — opens `claims`; every spec-authored `check` written without a scope clause,
  run at exact bounds over the actual claims (§ 2.4).
- `books/<name>.als` — opens `claims`; one per book fence (§ 2.3).

The split is syntactic: a `one sig` extending a species is a claim atom; everything else is
species. Its purpose is the scope-minimum limitation of § 1.2 and the distinction between a law
("for all speech, the engine is never wrong when the speech is true") and a corpus check ("every
closure claim in *this* spec is made by the owner of its sort"), which are different questions
over different universes.

### § 2.3 books-sites-speech-and-derivation

A book fence compiles as follows.

- Each concrete line with a verdict form in its `#=` is a `Site`; its `verb` is the mapped
  command word; its `arg` is the last key-typed name on the `#}` line (a generator convention,
  soft); `before` is the set of sites above it; `Converged` holds every site whose verdict form is
  `Elided` or `Guarded`. A `Query` is generated per (earlier non-converged site, converged site)
  pair. Fixture lines generate no site.
- **`mech-speech-is-per-site-data`** [ACKED 2026-09-27] — each site's `speech` is the set of
  claims in force at that line: the oracle sets and single claims the `.` lines above it load, in
  order, last definition winning by file stem where two loads answer the same thing, plus every
  derived claim (below) whose measurement line is above it. It is data the book module states,
  computed today by that rule and by the analyzer's own load model later; the laws and every
  shared-layer definition take a site's set as a parameter and never learn how it was assembled.
  Claims are atomic [TYPED]: a load brings a set of them, a later load may bring a set that
  displaces some, and there is no partial override to model, since an oracle either handles an
  input shape or declines it whole. A load under a condition (`if [ -f local.sh ]; then
  . ./local.sh; fi`) is two speech sets for every later site; each verdict is checked under both.
- **`mech-two-strata`** — the truth stratum, `reaches` and `worldParent` on `MKey`, is named by no
  claim and read only by `wrong`; a fixture line's `#=` constrains it. The knowledge stratum is
  what the loaded speech says. Static claims are true by default where in force at some site and
  false where in force at none (so an unloaded claim never constrains a book's world into
  unsatisfiability); a book denies a claim on a fixture line. Derived claims, `Resolution
  { of, to: lone }` and `Placement { of, within }`, have their truth computed against the truth
  stratum, never assumed: a resolution with no target is a decline and is always true.
- **`mech-defaults-are-derived-deviations-are-written`** [ACKED] — the generator derives every
  default resolution and placement from the static scheme claims composed with the fixture
  facts, under the hypothesis that the lookup is correct, with the speaker derived as the owner
  of the key's scheme. A hand-written `Resolution` or `Placement` on a `#=` line is a deviation
  (a decline, a wrong lookup) that suppresses the default for that key; the report lists every
  written deviation and every suppressed default.
- Each site becomes a `run` at exact bounds (`exactly N MKey`, `exactly M Site`, …) asserting its
  verdict form conjoined with every other `#=` statement on its line (an attribution intent such
  as `… not in by[this]`, a `not wrong[…]` guard). A run that finds no instance is a red.
- Verdict forms are a closed set the shared layer defines once from the consumer map: `this in
  Ran`, `this in Elided`, `this in Guarded`. The correspondence compiler (§ 3.4) recognises
  exactly these and nothing else.

### § 2.4 the-checks-assay-runs

1. **Generic laws** (tool-owned, over `laws.als`, the free universe): never wrong when all
   in-force speech is true; monotone in speech (more true speech never weakens an answer along
   `weaker`); stranger-safe (a speaker the set does not contain cannot weaken it); attribution
   honest (a wrong answer rests on some false claim); attribution sufficient (the rested-on set
   alone yields the answer); attribution minimal (each rested-on claim changes the answer when
   removed). Each is parametric in a speech set `S`, so per-site speech changes none of them.
2. **Spec laws**: a spec-authored `check` with a scope clause, run over the free universe.
3. **Corpus checks**: a spec-authored `check` without a scope clause, run over the exact claim
   corpus (the seat rule "every claim's speaker owns the seat that can know it" is the standing
   example). The scope-clause rule is STRAWMAN [CONDUCTOR]; it keeps the document valid Alloy so
   the Analyzer's Markdown mode still opens it.
4. **Vacuity twins**: every generic law ships with a `run` of its premise at the same scope,
   which must be satisfiable. A spec law is expected to carry a `run` named `<law>_premise`;
   absent one, the report marks the law `vacuity: unchecked` rather than green.
5. **Books**: every site's `run` (§ 2.3).
6. **Derived columns**: `by[s]` for every spared site (the rested-on claims, computed); the
   **danger** column per claim, the inverse of `restsOn` across all books mapped to the wrong
   answer a false claim would cause, never authored.
7. **Mutation lane** (opt-in, slow) [TYPED: negative verification and mutation testing are a
   required guard]: line-drop mutants over the `alloy` fences (each meaning fact, each claim
   atom, each shared-layer fact removed in turn) with the laws and books re-run; a mutant no
   check or book kills is reported as an unwitnessed line. This is what makes "every fence line
   is load-bearing" a measured statement.

### § 2.5 lints

Cheap, solver-free, and the part that rides the ordinary gate. Four families [CONDUCTOR; the
eleven of the strawman report, regrouped]:

- **Name coherence**: a bare word on a `#}` line lifts a global name required to inhabit the
  same value in that word position everywhere; a `{class}` word gathers instances; the same
  spelling cannot be both; each concrete line inhabits its `#}` line word for word; an
  introduced instance name (`inode_x`) is introduced once per book.
- **Scope**: every free name in a `#=` line is one the line owns (its map names, the instances
  it introduces, claim atoms, `this`); `this` never appears on a fixture line; no `#` in a
  fence except the two lifted forms; no comment syntax inside an `alloy` fence.
- **Contract**: the shared-layer signature is present with the right arities (§ 2.1); every
  claim atom's name prefix equals its `speaker`; every derived claim's speaker equals the
  owner of its key's scheme; every `.` load resolves to an oracle set or a claim; every verdict
  form is one of the three.
- **Model hygiene**: a scheme yields or is primary, never both (the fact 311 § 1.3 states; its
  absence admitted a stranger's primary claim on `Path` that broke monotonicity in the
  strawman); every `check` has a vacuity twin or a recorded `unchecked`.

### § 2.6 the-lock-and-the-report

- **The lock** [ACKED, with the human's nack of a generated index in the `SLUGS.md` style]: per
  spec, one committed file beside it (STRAWMAN: `<spec>.lock.json`) holding, per check, the
  scope and result; per book site, the expected form, the computed answer, the rested-on set, and
  the intents; per claim, the danger column; per law, the vacuity status; and the pinned Alloy
  build and solver. The gate recomputes and refuses a mismatch in either direction: a verdict that
  moved is a finding (a regression, or a repair that must be acknowledged), and a verdict that
  appeared is ambition nobody committed to. The lock moves only by a human commit, exactly as
  `301`'s catalogue lock does. It is not merged into that lock in this experiment; whether the
  two become one artifact is a later question with the correspondence half.
- **The report** is tool output for machines first (JSON), with views generated on demand
  (`assay report`, `assay danger <claim>`, `assay why <book>:<line>`) rather than a committed
  prose index. The residue query is one line: laws marked `unchecked`, sites whose only
  discharge is the recorded qualification of § 3.4, mutants unkilled.
- **Failure output** names the triage: which of *book*, *model* (a meaning fact or the shared
  layer), or *prose* (a claim atom whose long name promises something its species does not
  encode) the reader should suspect, in that order, with the counterexample rendered as a table
  of the relations over named atoms (claim ids and book instance names, so a corpus or book
  counterexample reads like the spec; a free-universe counterexample is rendered by species and
  speaker).

### § 2.7 ergonomics-for-an-editing-model

The loop an LLM runs: edit the spec; `mise run assay` (STRAWMAN task); read the lock diff and any
counterexample; either repair the model, retire a verdict deliberately in the same commit, or
stop and show the human. The by-law reading [TYPED lean]: where prose and model disagree, the
model wins, and the disagreement is a design question to burn down into more mechanical
clarity, by adding constraints to the model and tuning the prose, never by softening a check.
Every name is long and correctness-sensitive, since a name is the one intent-carrier the code
cannot hold, and the lints make a mismatched name a red rather than a note.

### § 2.8 two-examples

A species with its meaning fact and two claims under it, as written in a spec's `alloy` fence:

```alloy
sig GuaranteesUniqueName extends MDecl { on: one MScheme }
fact { all d: GuaranteesUniqueName & True, a, b: MKey |
   a.scheme = d.on and b.scheme = d.on and a.worldParent = b.worldParent and a.reaches = b.reaches implies a = b }

one sig tessa__a_file_has_one_inode_in_its_filesystem extends GuaranteesUniqueName {} { speaker = tessa  on = Inode }
one sig simon__a_filesystem_has_one_device_number_in_its_boot extends GuaranteesUniqueName {} { speaker = simon  on = DeviceNumber }
```

The first two lines land in `species.als`; the two atoms land in `claims.als`. `laws.als` opens
`species` and checks, over a free universe of claims, `monotoneInSpeech for 6 but 8 MKey`; a
corpus check such as `seatCanKnow` (written without a scope clause) lands in `corpus.als`, opens
`claims`, and runs at `exactly` the corpus's size.

A book, as written in an `sh` fence, with one load between two sites to show the per-site
speech:

```sh
# siblings_across_filesystems.sh
. ./tessa_fs.sh
. ./simon_fs.sh
. ./stdlib_boot.sh
. ./carl_chmod.sh

    stat -c '%i %d' /srv/a/shared /var/lib/other
#}  stat -c '%i %d' a_path d_path
#=  a_path.reaches = inode_x.reaches and d_path.reaches = inode_z.reaches
#=  inode_x.worldParent = fs_1 and inode_z.worldParent = fs_2

    cat /proc/sys/kernel/random/boot_id
#}  cat /proc/sys/kernel/random/boot_id
#=  fs_1.worldParent = boot_1 and fs_2.worldParent = boot_1

    chmod g-w /srv/a/shared
#}  chmod g-w a_path
#=  this in Ran

. ./alice_nfs_overrides.sh

    chmod g+w /var/lib/other
#}  chmod g+w d_path
#=  this in Guarded
#=  no (speaker.simon & by[this])
```

Its generated module, abridged:

```alloy
module books/siblings_across_filesystems
open claims

one sig a_path, d_path, inode_x, inode_z, fs_1, fs_2, boot_1 extends MKey {}
fact { a_path.reaches = inode_x.reaches and d_path.reaches = inode_z.reaches
       inode_x.worldParent = fs_1 and inode_z.worldParent = fs_2 }
fact { fs_1.worldParent = boot_1 and fs_2.worldParent = boot_1 }

-- derived defaults, one per key the fixture lines bound; speaker = the key's scheme owner
one sig tessa__a_path_resolves_to_inode_x extends Resolution {} { speaker = tessa  of = a_path  to = inode_x }
one sig tessa__inode_x_is_placed_in_fs_1 extends Placement {} { speaker = tessa  of = inode_x  within = fs_1 }
-- … d_path, inode_z, fs_1, fs_2 likewise
fun derived_1: set MDecl { tessa__a_path_resolves_to_inode_x + … }
fun derived_2: set MDecl { simon__fs_1_is_placed_in_boot_1 + simon__fs_2_is_placed_in_boot_1 }

one sig site_3, site_5 extends Site {}
fact { site_3.verb = chmod  site_3.arg = a_path  no site_3.before  site_3 not in Converged }
fact { site_5.verb = chmod  site_5.arg = d_path  site_5.before = site_3  site_5 in Converged }
fact { site_3.speech = tessa_fs + simon_fs + stdlib_boot + carl_chmod + derived_1 + derived_2 }
fact { site_5.speech = site_3.speech - simon__an_ext4_filesystem_exposes_its_inodes_through_no_other_filesystem + alice_nfs_overrides }
one sig q_5 extends Query {} { w = site_3  r = site_5 }
fact { Static & True = Static & Site.speech }

run site_3 { site_3 in Ran }
   for exactly 7 MKey, exactly 2 Site, exactly 1 Query
run site_5 { site_5 in Guarded and no (speaker.simon & by[site_5]) }
   for exactly 7 MKey, exactly 2 Site, exactly 1 Query
```

Alice's load displaces Simon's closure at every later line (by file stem, in this cut), so the
site that spared under `simon_fs` alone now guards, and the intent on its line says nothing of
Simon's is rested on. The lock row for that site records the expected form, the computed answer,
and the rested-on set; the danger column for Simon's closure loses this book and keeps the
others.

## § 3 — implementation-sketch

Light on purpose; the builder has latitude on everything not marked.

- **Runner.** A mise-managed JDK and a pinned Alloy 6.2 distribution jar fetched to a cache, never
  vendored into the tree (SyncThing is live above the repo; a jar in-tree syncs forever). Drive
  the CLI's `exec` subcommand with preferences passed on the command line, capture the XML
  instance per command, parse results and scopes. If the pinned build's CLI proves too thin, a
  shim over the same jar's Java API is the fallback; either way one process per command, with
  the wall-clock and memory gating the Kani lane already uses. Runs on Windows and WSL alike.
- **Compiler.** Rust. Lean: a subcommand of `crates/internal-tooling`, which already reads the
  corpus for `slugs` and `docids`; a sibling crate under `spike/verify/` if it outgrows that. A
  Markdown fence lexer; the sh word lexer for `#}` lines (the existing syntax crate's lexer where
  it serves); `#=` lifted verbatim; the five-module generator of § 2.2; the derivation of § 2.3;
  the lints of § 2.5. Alloy itself parses the Alloy; assay never reimplements that.
- **Lock and report.** JSON in, JSON out, `--check` compares and writes nothing. Gate placement:
  lints in the pre-commit hk step, path-filtered to spec files; lock recomputation in
  `gate:full-quiet`, path-filtered the same way; the mutation lane and larger scopes in an
  opt-in `assay:deep` (STRAWMAN names).
- **First experiment** [ACKED: minimal harness first, then back to 311 to use it]: point assay at
  the strawman-2 spec where it sits, and replace its expected report with an observed one. Three
  bites are anticipated and are the experiment's first findings either way: the scope minima
  once the laws open the corpus (or do not, after the split); the cost of the subset-quantified
  laws at scope six; `attributionSufficient` under redundant speech, since `restsOn` is
  single-removal. The strawman is not promoted to the spec tier [TYPED nack]; turning 311 into a
  specification is separate, clean-context, product-focused frontier work.
- **Not in the first cut**: the correspondence compiler (§ 3.4), temporal models (§ 4), any
  unification with the catalogue lock, and any view beyond the three listed.

### § 3.4 the-correspondence-half-later

A book fence is already a runnable sh file. When the product's kernel reaches the design a spec
pins, the same file compiles to an e2e expectation: a loom session running the book under the
harness binary, with the expected transcript derived from the three verdict forms and nothing
else. Until then, and wherever the product disagrees afterward, the lock stores the field-tier
verdict as qualified intent in goblint's manner [ACKED]: `UNKNOWN!` for an unsoundness (the
product spared where the design collides), `UNKNOWN` for intended imprecision (the product guards
where the design elides, by decision), `TODO` for precision owed. Disagreement is triaged five
ways (implementation, book, record, model, prose) and the failure output says which it suspects;
nothing regenerates a stored verdict from product output.

## § 4 — latitude-kept-open

What the first cut must not weld shut, each priced at one sentence [ACKED 2026-09-27 unless
marked]:

- Speech is per site and is data (§ 2.3); nothing derives it inside Alloy; the laws stay
  quantified over arbitrary sets. The admin seat, an admin's load displacing a foreign author's
  claim at every later line, is then the ordinary ownership rule applied to the in-force set at
  that line, with attribution naming the admin, and needs no carve-out.
- The truth stratum is not welded static. Alloy 6's `var` sigs and step scopes are the intended
  route for the lifecycle species (311 § 3.3, a resolution leaving `True` after a write), for
  the two standups as two instants, and for time-of-check against time-of-use; the runner
  accepts temporal commands from day one because that is a flag, not a design.
- Fixture facts are sited (a measurement is at its line), which is the hook the temporal work
  needs; the generator hoists them into timeless facts today and stops doing so then.
- `Site` may grow fields (a vantage, an entry chain) without the verdict forms or the `#=`
  vocabulary moving; `.` lines may become sites of their own when the load plane's claims enter.
- A claim may acquire a matched-shape parameter when 311's per-shape warrants are modelled; the
  displacement key "what a claim answers to" is then that pair, and it is the generator's, not
  the model's [CONDUCTOR].

## § 5 — meta-test-cases

The twelve elements the ledger banked as stress cases for the system, in short: forward from
design, 311 § 3.2 two-tops, § 3.5 committee law, § 1.1 no-referent-in-engine, § 3.3 lifecycle;
backward from code, the exhaustive-consumer-map Kani harness, the minispec join laws and their
Kani twins, the sparing-reference disjointness unit test, the DST permutation pin; lateral and
value, the pi-webhost book with its package oracle, `GOTCHAS:a-host-is-not-a-partition`, `28M`'s
withhold-only wall, `30D`'s decision table. Each was chosen to exercise a different mechanism;
the full set with rationale is `30Ya` "Meta-test cases". None is built in the first cut; they
are the acceptance list for the instrument once 311 is a spec.

## § 6 — what-the-research-round-found

Four fronts, banked as `turn01`–`turn04` in the evidence base with every source archived and
graded; the load-bearing findings, condensed (grades are the subagents' unless the ledger says
the conductor re-read the copy):

- **Production Alloy use** (turn01): one standalone model per concern is the norm; no
  multi-year model set exists; the only mechanical model-to-code tie found is tag-level string
  equality run on every PR [B-gadget-silo-ci-alloy-jobs-2026]; one project fails CI on a vacuous
  `run` [C-emilia-protocol-run-alloy-doc-2026]; litmus suites are stored SAT/UNSAT goldens
  [A-khronos-vulkan-alloy-litmus-makefile-2019]; models as regression records of real bugs
  [B-nixbot-scheduler-alloy-model-2026] [B-aws-cdk-iam-policy-merging-alloy-2022].
- **Single-artifact prose and checks** (turn02): no maintained format is both analysable and
  carries typed evidence for uncheckable claims [A-ernst-dependability-case-language-2015]; the
  nearest is an evidence-guarded Alloy model with a dated, attributed expert-evidence kind
  [A-pernsteiner-safety-case-pluggable-checkers-2016]; standards bodies converge on a stable
  in-prose id per sentence, a separate check keyed by it, a register of ids declared uncheckable,
  and a drift rule retiring an id on semantic change [A-khronos-vulkan-vuid-style-guide-2026]
  [A-riscv-normative-rules-tagging-2026]; requirements tooling adds a per-item declaration of
  coverage owed and a typed discharge kind with a CI-diffed snapshot
  [A-openfasttrace-writing-a-specification-2026] [A-awslabs-duvet-annotations-2026]; literate
  containers run the fences and ignore the prose [B-alloy-docs-markdown-2023]; links rot and
  human link vetting is wrong about a quarter of the time
  [A-cleland-huang-traceability-trends-2014].
- **Ties between instruments** (turn03): a finder and a prover share a statement only inside
  one logic or one language [A-blanchette-nitpick-kodkod-isabelle-2010]
  [A-konnov-tla-trifecta-tlc-apalache-tlaps-2022]; a bounded check is a theorem only under a
  proved encoding [A-lean-bvdecide-api-docs-2026]; the five Alloy-to-prover translations each
  lose bounded integers, finiteness, and multiplicities [A-krings-alloy-to-b-translation-2018]
  [B-chen-lforge-forge-in-lean-2024]; design specs and conformance specs pull apart
  [A-davis-extreme-modelling-mongodb-2020]; trace validation is the strongest production seam
  [A-cirstea-tla-trace-validation-2024]; seams fail at interfaces, never inside verified code
  [A-fonseca-verified-distsys-bug-study-2017].
- **The shared unit across tiers** (turn04): where a proof tier consumes examples it extracts an
  executable and runs them under the ordinary runner; theorems never consume the suite
  [A-wasmcert-coq-readme-2026] [A-armstrong-sail-isa-semantics-2019]; stored verdicts carry
  per-implementation qualifications [A-oils-var-op-test-spec-file-2026]
  [A-goblint-developer-testing-guide-2026]; regenerating verdicts from output is
  characterisation, not specification [A-tree-sitter-writing-tests-2026]; disagreement across
  consumers is triaged five ways [A-riscv-arch-test-readme-2026]; the shell lineage's unit is
  a script plus expected output run unchanged against a semantics and seven shells
  [A-greenberg-smoosh-executable-posix-semantics-2020].

Corpus-internal findings that shaped the design: the three crosscheck rounds over 311 split into
genuine model defects (narrowing per round) and text-drift regressions (not narrowing; 76
commits in 11 days); one bug class recurred three times (a universal over an empty set read as
safe); `311u` is a regression suite in prose nothing re-runs; every panel witness fits four to six
atoms. The scope map of what else in the corpus is worth modelling, by cluster, is `30Ya` "Scope
map".
