# 30Y — assay: design-tier models in Alloy

> Tier: LLM-authored plan (Fable conductor, from the 2026-09-27 sittings; round 30). Subordinate
> to the root docs and `spike/CLAUDE.md`; sibling of `notes/301` (minispec and dorc-verify, the
> code-tier instrument this extends downward to the design tier) and of `notes/30X` (the testing
> architecture the correspondence half will eventually ride). Its ledger is `notes/30Ya`; its
> evidence base is `.claude/research/design-model-mechanisation-prior-art/` (four fronts, every
> source graded; the worked strawman is `strawman-2/` there, on the older harness). Grades:
> **[TYPED]** the human typed it · **[ACKED]** confirmed in dialogue · **[CONDUCTOR]**
> conductor-derived, unratified. Every name here is STRAWMAN and renames freely
> (`rul-strawman-formats-no-compat`). This document is about the tooling only: what assay
> reads, what it builds, what it runs, and what it reports. How a spec document is written is
> not here.

## § 0-what-assay-buys

**assay** is a compiler and checker that reads a literate specification document, builds Alloy 6
modules from its fences, runs them, and holds the verdicts in a lock. It is a generic solver
over sh-spelled lines mapped to abstract claims about those lines; it knows shell, and it knows
nothing about what any claim means. It exists for two payoffs, both typed by the human [TYPED]:

- **`buy-regression-before-a-human-reads-it`** — a design statement made precisely once, and a
  later rewrite, composition, or assumption that reads it wrong, is caught mechanically, by a
  gate an LLM runs before showing a claim to a human. Of the two defect populations the three
  crosscheck rounds over the identity model found, text-drift under folds and rewrites is the
  one that does not narrow with rounds and scales with edit volume; that is the population this
  instrument is for.
- **`buy-one-input-across-checkers`** — the unit that pins a behavioural claim is a few lines of
  sh with an expected verdict per line: a book. The same book, unrewritten, is the design tier's
  test today and the product's own input at the field tier later, so "the design says elide and
  the product says guard" is a diff over one file, never a translation between two.

What falls out once those are built, each observed in the prior art and each priced by the
strawman: a refuted shape (the `311u` register, the `GOTCHAS` list) becomes a run that re-runs on
every edit; the cost of a rewrite is countable as changed lock rows; and the strongest tool the
panels lacked, "no answer changed" after a rewrite, becomes a lock diff of zero.

Posture [TYPED]: tracked, not proved. Bounded model-finding over hand-written models of the
prose, at the prose's own altitude. Markdown in, Alloy out; JSON in, JSON out; a nonzero exit is
how it speaks. Never user-facing. The design tier's instrument the way Kani and minispec are the
algebra tier's; the seam between them is the book, not a translation (§ 1.3).

## § 1-the-instrument-alloy-six

### § 1.1-why-alloy

Alloy fits a pre-code relational model better than anything else surveyed [CONDUCTOR, front 3
of the research round]: the identity model's statements are sets, relations, multiplicities,
and universally quantified laws, which is Alloy's whole language; the small-scope hypothesis
holds empirically here, since every panel witness to date fits in four to six atoms; a check
takes seconds; a counterexample is a concrete small world a human can read; Alloy 6 adds a
temporal mode (`var` sigs, `after`, `always`) the chronology work will want (§ 4); and the
distribution is one jar on a JVM, with a headless `exec` subcommand and a documented Markdown
mode that runs fenced `alloy` blocks straight out of a `.md` file, so a person can open a spec
in the Analyzer without assay in the loop.

### § 1.2-limitations-stated-plainly

- **Bounded.** "No counterexample at scope 6" is not a theorem. The lock stores the scope beside
  every result and the report never says "proved". The subset-quantified laws are the expensive
  ones; the strawman's guess is a ceiling near scope five or six for them, to be measured.
- **Fixed atoms fix scope minima.** A `one sig` claim atom is an atom in every universe that
  opens its module, so a check over the actual claims needs a scope at least the corpus's size,
  and a law meant to hold for all speech must not open the corpus at all. The module split in
  § 2.3 exists for this reason [CONDUCTOR].
- **Higher-order quantification only where Alloy skolemizes it.** A `check` with a top-level
  `all S: set Claim` negates to an existential Alloy can solve; nesting a set quantifier under
  another quantifier does not. A law written outside that shape gets Alloy's own error, passed
  through.
- **No vacuity check built in.** A `check` whose premise is unsatisfiable passes. § 2.5 pairs
  checks with satisfiable twins by name, and a book `run` that finds no instance is a failure.
- **Integers are bounded.** They appear as argv positions and as key values in positional
  catalogs (`ufw insert 1`, rowids; 311 § 2.9), and Alloy's integers wrap at the bitwidth the
  scope sets. The bitwidth is a scope like any other; a spec that counts sets it and knows it.
- **The model is the prose's model, not the code's.** Nothing here reads Rust. The tie to the
  implementation is the book (§ 1.3), and until the product's kernel reaches the design a book
  pins, that book's field-tier verdict is a recorded, qualified red (§ 3.1).

### § 1.3-the-seam-is-the-book-not-a-translation

Every found tie between a model-finder and a prover shares a statement only when the finder runs
inside the prover's logic or both read one language; nothing found ties an Alloy-family model to
Kani, CBMC, or Rust [front 3]. The project's proof tier consumes definitions derived from the
shipping code (`301`), so a design-tier model and a code-tier law never share a statement, and
forcing them to would put the incorrectness at the seam. The shared unit is instead the one the
referent projects converge on: an input with a stored expected verdict, run unchanged against
each tier [front 4]. Here that is the sh book. The design tier reads it into Alloy; the field
tier will run it on a host; the lock records both verdicts side by side.

## § 2-what-assay-builds

### § 2.1-the-input

A spec document is Markdown with two fence kinds assay reads and nothing else it interprets:

- `alloy` fences: the model, verbatim. Whatever sigs, facts, functions, predicates, and commands
  the spec wants; claim atoms as `one sig <name> extends <some sig under Claim>`.
- `sh` fences: books and load files. A book is valid sh: each concrete line is followed by two
  comment forms, `#}` and `#=`. A load file is an `sh` fence of `.` lines headed by a file name,
  which books source (the degenerate form of a future in which claims ride lines of oracle sh;
  § 4).

The two comment forms, precisely:

- **`#}` binds the immediately preceding logical line and nothing else.** Its components are
  shell-lexed and matched one-to-one with the line's words. A bare component *names* the literal
  under it; a braced component `{flavour}` puts the literal under it in a *class*. Any
  component may be either, the command word included. The literal is the join key: one literal
  is one atom, so a word named on one line and classed on another is one atom carrying both.
- **`#=` is line-scoped Alloy, lifted verbatim** across consecutive `#=` lines, with `this`
  bound to the line's atom. A `#=` that mentions `this` makes the line a `Line` and a `run`; one
  that does not is a fact about the world, and its line generates no atom. Free names in a `#=`
  are atoms assay knows: the map's names, claim atoms, and names the `#=` introduces, which assay
  mints. A trailing `for` clause on a `#=` is the run's scope (§ 2.4).

### § 2.2-the-harness-and-the-unit

Assay owns exactly the sh-side structure, as one Alloy module every generated module opens
[ACKED 2026-09-27]:

```alloy
module assay
sig Shword { class: set Class }
sig Class {}
sig Claim {}
sig Line { before: set Line, speech: set Claim, cmd: one Shword, argv: seq Shword }
```

- A `Shword` is one atom per distinct literal shell word, or per name a `#=` introduces.
- A `Class` is an opaque category of strings [TYPED]: arbitrary, never parsed, never meaningful
  to assay. It exists so two claims can agree about two different literals as one kind of text
  without anyone writing a parsing rule; where two claims can share the literal itself, they
  just do. Membership is only ever stated, by a braced map component or by a spec's fact. A
  braced word is a join node, not a quantifier: a book stays concrete, and a universal
  statement over a class is a `check` in the spec.
- A `Claim` is what a `.` line loads, by file stem. Assay never reads a claim's fields.
- **`unit-is-the-logical-line`** [ACKED] — a `Line` is one complete command as sh reads it,
  continuations joined, because that is where a comment attaches. Alloy holds four things about
  it: its order among the book's lines, the claims in force at it, and its decomposed words as
  `cmd` and `argv`. Decomposed structure is assay's to grow (parts and the operators between
  them, redirects, a substitution tree) so that spec text never grows string-splitting; literals
  are never exposed. At v0 assay hands the map line to the existing syntax crate's lexer, takes
  the first word as `cmd` and the rest as `argv`, and polices nothing [TYPED: least work].

Beside the harness, assay opens the tree-global spec module (STRAWMAN `spec/shared`) into every
module it generates, so no document declares the truly global things [ACKED]. What that module
holds, from `Speaker` and the claim species' base through the answer order, the laws, the
verdict subsets, and how a resolution is derived, is spec-tier content and not this document's;
anything less than tree-global is opened explicitly by the spec that wants it.

### § 2.3-the-generated-modules

One spec document becomes one directory of Alloy modules [CONDUCTOR, STRAWMAN layout]:

- `species.als` — opens `assay` and `shared`; every `alloy` fence line that is not a claim atom.
- `claims.als` — opens `species`; every `one sig … extends <sig under Claim>` atom, and one
  named set per load file, so a book's loads are set expressions.
- `laws.als` — opens `species` only, so the claim universe is free; every spec-authored `check`
  that carries a scope clause.
- `corpus.als` — opens `claims`; every spec-authored `check` written without a scope clause,
  run at exact bounds over the actual claims.
- `books/<name>.als` — opens `claims`; one per book fence (§ 2.4).

The species-versus-claims split is syntactic (a `one sig` whose ancestry reaches `Claim`); its
purpose is the scope-minimum limitation of § 1.2. The scope-clause rule for laws versus corpus
checks is STRAWMAN [CONDUCTOR]: it keeps the document valid Alloy, so the Analyzer's Markdown
mode still opens it, and it reads naturally, since a law states its scope and a corpus check's
scope is the corpus.

### § 2.4-books-lines-speech-and-runs

A book fence compiles as follows.

- Every distinct literal on a map line, every braced class, and every name a `#=` introduces
  becomes a `one sig` atom. Each braced component adds a class membership fact for its literal.
- Each line whose `#=` mentions `this` is a `Line` atom with `cmd`, `argv`, and `before` (the
  lines above it). Each `#=` without `this` is emitted as a fact, verbatim.
- **`mech-speech-is-per-line-data`** [ACKED] — each line's `speech` is the set of claims the
  `.` lines above it load, in order, a later load displacing an earlier one that answers the
  same thing (by file stem, at v0). It is data the book module states, computed today by that
  rule and by the analyzer's own load model later; nothing in a spec learns how it was assembled.
  Claims are atomic [TYPED]: a load brings a set of them and there is no partial override to
  model, since an oracle either handles an input shape or declines it whole.
- Each `Line` becomes a `run` asserting its `#=` body with `this` substituted, at exact bounds
  on `Shword`, `Class`, `Line`, and `Claim`, and at the default scope for everything else unless
  the `#=` ends in a `for` clause, which is passed through as written. A run that finds no
  instance is red.

Assay derives no claim and recognises no verdict. Default resolutions, the truth of loaded
speech, query pairs, and the meaning of `Elided` are all facts and functions in the shared
module or the spec, over the relations assay supplies.

### § 2.5-the-checks-assay-runs

1. **Laws**: every `check` in `laws.als`, over the free claim universe, at the scope it states.
2. **Corpus checks**: every `check` in `corpus.als`, at exact bounds over the actual claims.
3. **Vacuity twins**: a `check X` is paired by name with a `run X_premise` in the same module,
   which must be satisfiable; a check with no twin is recorded `premise: absent`, never green.
4. **Books**: every line's `run` (§ 2.4).

### § 2.6-lints

Solver-free, and only what compilation needs:

- one literal carries at most one bare name across the tree, and one name covers at most one
  literal (the join key of § 2.1);
- every free name in a `#=` is an atom assay knows;
- every `.` line resolves to a load file or a claim atom;
- every generated module parses under Alloy (Alloy's own parser; assay reimplements none of it).

### § 2.7-the-lock-the-report-and-the-exit-code

- **The lock** [ACKED, with the human's nack of a generated index in the `SLUGS.md` style]: one
  committed JSON file per spec beside it (STRAWMAN `<spec>.lock.json`), one row per command:
  module, name, kind, scope, result (`sat` · `unsat` · `counterexample` · `no-counterexample`
  · `premise: absent`), and a hash of the command's text. `assay --check` recomputes and exits
  nonzero on a mismatch in either direction; `assay --write` rewrites it, and the commit that
  carries it is the ceremony, as with `301`'s catalogue lock. Not merged with that lock in this
  experiment.
- **The report** is the same rows plus the lint results and the generated file paths, as JSON,
  to stdout. No prose, no suspicion, no ranking [TYPED]. A person or model who wants the
  counterexample opens the generated `.als` in the Analyzer or runs `alloy exec` on it.
- **Exit codes** [CONDUCTOR]: `0` green and lock matches · `1` a red command or a lock mismatch
  · `2` a lint refusal · `3` the runner failed (jar, JVM, timeout).

### § 2.8-two-examples

A species with its meaning fact and two claims under it, as written in a spec's `alloy` fence
(the species base `MDecl extends Claim` comes from the shared module):

```alloy
sig Yields extends MDecl { of: one Class, under: one MScheme, to: lone MScheme }

one sig tessa__a_slash_separated_path_names_one_entry_per_component extends Yields {} { speaker = tessa  of = slash_path  under = Path  to = Entry }
one sig tessa__a_bare_word_is_not_a_path extends Yields {} { speaker = tessa  of = bare_word  under = Path  no to }
```

The first line lands in `species.als`; the two atoms land in `claims.als`. A `check` written
with a scope lands in `laws.als` and runs over a free universe of claims; one written without
lands in `corpus.als` and runs over exactly these.

A book, as written in an `sh` fence. The `stat` line names two literals; the `chmod` lines put
the same literals in a class; the fixture states only what is true of the world:

```sh
# siblings_across_filesystems.sh
. ./tessa_fs.sh
. ./simon_fs.sh
. ./stdlib_boot.sh
. ./carl_chmod.sh

   stat -c '%i %d' /srv/a/shared /var/lib/other
#} stat -c '%i %d' a_path d_path
#= a_path.reaches = inode_x.reaches and d_path.reaches = inode_z.reaches
#= inode_x.worldParent = fs_1 and inode_z.worldParent = fs_2 and fs_1.worldParent = boot_1 and fs_2.worldParent = boot_1

   chmod g-w /srv/a/shared
#} chmod g-w {slash_path}
#= this in Ran

   chmod g+w /var/lib/other
#} chmod g+w {slash_path}
#= this in Elided
```

Its generated module, abridged:

```alloy
module books/siblings_across_filesystems
open claims

one sig stat, c_flag, fmt_i_d, a_path, d_path, chmod, g_minus_w, g_plus_w, inode_x, inode_z, fs_1, fs_2, boot_1 extends Shword {}
fact { a_path.class = slash_path  d_path.class = slash_path }
fact { a_path.reaches = inode_x.reaches and d_path.reaches = inode_z.reaches
       inode_x.worldParent = fs_1 and inode_z.worldParent = fs_2 and fs_1.worldParent = boot_1 and fs_2.worldParent = boot_1 }

one sig line_3, line_4 extends Line {}
fact { line_3.cmd = chmod  line_3.argv = 0->g_minus_w + 1->a_path  no line_3.before }
fact { line_4.cmd = chmod  line_4.argv = 0->g_plus_w + 1->d_path   line_4.before = line_3 }
fact { line_3.speech = tessa_fs + simon_fs + stdlib_boot + carl_chmod  line_4.speech = line_3.speech }

run line_3 { line_3 in Ran }    for 4 but exactly 13 Shword, exactly 1 Class, exactly 2 Line, exactly 19 Claim
run line_4 { line_4 in Elided } for 4 but exactly 13 Shword, exactly 1 Class, exactly 2 Line, exactly 19 Claim
```

Everything that makes `line_4` elide is the spec's: Carl's claim that chmod reads its operand
under `Path` when the operand is a `slash_path`, Tessa's claim that a slash path under `Path`
yields an entry, the shared module's derivation of the resolution from those, and its definition
of `Elided`. Hand the same lines a bare word (`#} chmod g-w {bare_word}`) and Tessa's second
claim declines, and the same site guards. Assay saw two literals, one class, two lines, and a
set of claims.

## § 3-implementation-sketch

Light on purpose; the builder has latitude on everything not marked.

- **Runner.** A mise-managed JDK and a pinned Alloy 6.2 distribution jar fetched to a cache, never
  vendored into the tree (SyncThing is live above the repo). Drive `exec` with preferences on the
  command line, capture the XML per command, parse results and scopes; one process per command,
  with the wall-clock and memory gating the Kani lane already uses. Windows and WSL alike.
- **Compiler.** Rust, as a subcommand of `crates/internal-tooling` (which already reads the
  corpus for `slugs` and `docids`); a sibling crate under `spike/verify/` if it outgrows that. A
  Markdown fence lexer; the syntax crate's lexer for map lines; `#=` lifted verbatim; the five
  modules of § 2.3; the lints of § 2.6. Assay never parses Alloy.
- **Lock and report.** JSON in, JSON out; `--check` writes nothing. Gate placement: lints in the
  pre-commit hk step, path-filtered to spec files; lock recomputation in `gate:full-quiet`,
  path-filtered the same way; larger scopes in an opt-in lane.
- **First experiment** [ACKED: minimal harness first, then back to 311 to use it]: re-express
  the strawman-2 document on this harness, in place, as the tool's own fixture, and replace its
  expected report with an observed one. The bites to expect are the experiment's first findings:
  the subset-quantified laws' cost at scope six; `attributionSufficient` under redundant speech;
  the default scope for spec-owned sigs in book runs. The strawman is not promoted to the spec
  tier [TYPED nack]; turning 311 into a specification is separate, clean-context, product-focused
  frontier work.

### § 3.1-later-and-maybe

Not in the first cut; listed so nothing here is mistaken for forgotten.

- **The correspondence compiler.** A book fence is already a runnable sh file. When the
  product's kernel reaches the design a spec pins, the same file compiles to an e2e expectation
  (a loom session under the harness binary, the expected transcript derived from the spec's
  verdict subsets), and assay learns the one piece of Dorc knowledge it will need to map
  between tools, the outcome vocabulary [TYPED: eventually]. Until then and wherever the product
  disagrees, the lock stores the field-tier verdict as qualified intent in goblint's manner
  [ACKED]: `UNKNOWN!` for an unsoundness, `UNKNOWN` for intended imprecision, `TODO` for
  precision owed; nothing regenerates a stored verdict from product output.
- **Derived columns and views.** The rested-on set per spared line and the danger column per
  claim (the inverse of `restsOn` across all books) are evaluable against instances; a
  `--eval` of named expressions per run, and views over the lock, once the lock is boring.
- **The mutation lane** [TYPED: negative verification and mutation testing are a required
  guard]: line-drop mutants over the `alloy` fences with the laws and books re-run; a mutant no
  check or book kills is reported. Opt-in and slow.
- Larger scopes nightly; unification with the catalogue lock; conditional loads (`if [ -f
  local.sh ]; then . ./local.sh; fi`) as two speech sets per later line.

## § 4-latitude-kept-open

What the first cut must not weld shut, each priced at one sentence [ACKED 2026-09-27 unless
marked]:

- Speech is per line and is data (§ 2.4); nothing derives it inside Alloy; the laws stay
  quantified over arbitrary claim sets. An admin's load displacing a foreign author's claim at
  every later line then needs no carve-out anywhere.
- The truth stratum is the spec's and is not welded static. Alloy 6's `var` sigs and step scopes
  are the route for the lifecycle species (311 § 3.3), for the two standups as two instants, and
  for time-of-check against time-of-use; the runner accepts temporal commands from day one
  because that is a flag, not a design.
- Fixture facts are sited (a `#=` without `this` sits on a line), which is the hook the temporal
  work needs; assay hoists them into timeless facts today and stops doing so then.
- `Line` grows structure, never literals: parts and operators, redirects, substitutions, and a
  bounded word-set form (`{flavour} rest... a_bar`, both ends bound) when a spec needs them.
- A class may gain a corpus-level "for every inhabitant seen so far" check [CONDUCTOR, from the
  human's lean]; a book line stays concrete.
- A load file is the v0 of claims that ride lines of oracle sh; when that arrives, `#}` on an
  oracle's own line is what classes an argument position, and assay's `speech` rule follows the
  analyzer's load model instead of file stems.
- Assay eventually owns the outcome vocabulary, and nothing else Dorc-shaped, for mapping
  between tools [TYPED].

## § 5-meta-test-cases

The twelve elements the ledger banked as stress cases for the system, in short: forward from
design, 311 § 3.2 two-tops, § 3.5 committee law, § 1.1 no-referent-in-engine, § 3.3 lifecycle;
backward from code, the exhaustive-consumer-map Kani harness, the minispec join laws and their
Kani twins, the sparing-reference disjointness unit test, the DST permutation pin; lateral and
value, the pi-webhost book with its package oracle, `GOTCHAS:a-host-is-not-a-partition`, `28M`'s
withhold-only wall, `30D`'s decision table. The full set with rationale is `30Ya` "Meta-test
cases". None is built in the first cut; they are the acceptance list once 311 is a spec.

## § 6-what-the-research-round-found

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
  consumers is triaged, not auto-resolved [A-riscv-arch-test-readme-2026]; the shell lineage's
  unit is a script plus expected output run unchanged against a semantics and seven shells
  [A-greenberg-smoosh-executable-posix-semantics-2020].

Corpus-internal findings that shaped the design: the three crosscheck rounds over 311 split into
genuine model defects (narrowing per round) and text-drift regressions (not narrowing; 76
commits in 11 days); one bug class recurred three times (a universal over an empty set read as
safe); `311u` is a regression suite in prose nothing re-runs; every panel witness fits four to six
atoms. The scope map of what else in the corpus is worth modelling, by cluster, is `30Ya` "Scope
map".
