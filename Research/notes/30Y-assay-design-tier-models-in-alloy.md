# 30Y — assay: design-tier models in Alloy

> Tier: LLM-authored plan (Fable conductor, from the 2026-09-27 sittings; round 30). Subordinate
> to the root docs and `spike/CLAUDE.md`; sibling of `notes/301` (minispec and dorc-verify, the
> code-tier instrument this extends downward to the design tier) and of `notes/30X` (the testing
> architecture the correspondence half will eventually ride). Its ledger is `notes/30Ya`, and its
> build ledgers are `notes/30Yc` (with the chafe register), `notes/30Yf` (the lock's
> semantics, the key, and the tiers), and `notes/30Yg` (progress reporting and the official
> tier's cap); its evidence base is `.claude/research/design-model-mechanisation-prior-art/` (four fronts, every
> source graded); the worked fixtures are `notes/30Ya-strawman-2/` and `notes/30Ya-strawman-3/`,
> both run (`notes/30Yb`). Grades: **[TYPED]** the human typed
> it · **[ACKED]** confirmed in dialogue · **[CONDUCTOR]** conductor-derived, unratified. Every
> name here is STRAWMAN and renames freely (`rul-strawman-formats-no-compat`). This document is
> about the tooling only: what assay reads, what it builds, what it runs, and what it reports.
> How a spec document is written is not here.

## § 0-what-assay-buys

**assay** is a compiler and checker that reads a literate specification document, builds Alloy 6
modules from its fences, runs them, and holds the verdicts in a lock. It is a generic solver
over sh-spelled lines mapped to abstract claims about those lines; it knows shell, and it knows
nothing about what any claim means. Its purposes, in order [TYPED]:

- **`buy-rigor-by-adversary`** — first and above the rest: make it hard for the design process to
  write a wishy-washy line into a spec. Every statement a spec makes is checked as a `check`,
  never confirmed by a `run`: the solver is set against the statement, fills every gap the spec
  left with the world that breaks it, and hands that world back. The counterexample is the
  sentence the author forgot to write. A `run` is used for one thing only, to prove a set of
  facts is satisfiable so that a check cannot pass by contradiction.
- **`buy-regression-before-a-human-reads-it`** — a design statement made precisely once, and a
  later rewrite, composition, or assumption that reads it wrong, is caught mechanically, by a
  gate an LLM runs before showing a claim to a human. Of the two defect populations the three
  crosscheck rounds over the identity model found, text-drift under folds and rewrites is the
  one that does not narrow with rounds and scales with edit volume.
- **`buy-one-input-across-checkers`** — the unit that pins a behavioural claim is a few lines of
  sh with an expected verdict per line: a book. The same book, unrewritten, is the design tier's
  test today and the product's own input at the field tier later, so "the design says elide and
  the product says guard" is a diff over one file, never a translation between two.

What falls out once those are built: a refuted shape (the `311u` register, the `GOTCHAS` list)
becomes a check that re-runs on every edit; the cost of a rewrite is countable as changed lock
rows; and the strongest tool the panels lacked, "no answer changed" after a rewrite, becomes a
lock diff of zero.

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
distribution is one jar on a JVM. The jar carries a headless `exec` subcommand and a documented
Markdown mode that runs fenced `alloy` blocks straight out of a `.md` file, so a person can open a
spec or a generated module in the Analyzer, or run it from a shell, without assay in the loop.
Assay itself uses neither: it drives the jar's Java API through a small adapter of its own (§ 3),
because `exec` writes an output directory per source and offers no per-command cap.

### § 1.2-limitations-stated-plainly

- **Bounded.** "No counterexample at scope 6" is not a theorem. The lock stores the scope beside
  every result and the report never says "proved". The subset-quantified laws are the expensive
  ones; the strawman's guess is a ceiling near scope five or six for them, to be measured.
- **Fixed atoms fix scope minima.** A `one sig` claim atom is an atom in every universe that
  opens its module, so a check over the actual claims needs a scope at least the corpus's size,
  and a law meant to hold for all speech must not open the corpus at all. The module split in
  § 2.3 exists for this reason [CONDUCTOR].
- **Default scopes are small.** Alloy's default of three atoms per sig does not fit a book once
  keys are first-class things, and Alloy has no document-wide scope, only a clause per command.
  The ceiling for every kind assay does not own is the spec's to spell (§ 2.4). A red for want
  of scope is a red; the report says which command and what scope it ran at, and nothing more.
- **Higher-order quantification only where Alloy skolemizes it.** A `check` with a top-level
  `all S: set Claim` negates to an existential Alloy can solve; nesting a set quantifier under
  another quantifier does not. A law written outside that shape gets Alloy's own error, passed
  through.
- **No vacuity check built in.** A `check` whose premise is unsatisfiable passes. § 2.5 pairs
  checks with satisfiable twins by name, and every book carries a satisfiability run.
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
  shell-lexed and matched one-to-one with the line's words. A bare component is the *name* of
  the literal under it, and when the two are equal the literal is named after itself; a braced
  component `{flavour}` puts the literal under it in a *class* and leaves it named after itself.
  Every name that is not already an Alloy identifier becomes one under a deterministic munge of
  its bytes into Alloy's identifier alphabet, injective over the names it munges and readable for
  common shell punctuation; a munged spelling that meets an authored identifier is the join-key
  refusal below [TYPED 2026-09-28: a word defaults to itself; no ceremony forces a name to be
  minted, and every word trivially has one]. Any component may be either form, the command word included.
  The literal is the join key: one literal is one atom, so a word named on one line and classed
  on another is one atom carrying both; a name covering two literals, munged or authored, is a
  refusal.
- **`#=` is line-scoped Alloy, lifted verbatim** across consecutive `#=` lines, with `this`
  bound to the line's atom. What a `#=` statement *is* decides where it goes [ACKED 2026-09-27]:
  a **declaration** (`one sig …`) is a claim or a world object, emitted at module level, and if
  it is a claim it is in force at that line and every line after; a **formula on a line that
  never mentions `this`** is a fact about the world, and that line generates no atom; a
  **formula on a line that mentions `this`** is that line's *outcome*, the statement the
  adversary attacks (§ 2.4). World facts never need `this`, since the world does not know about
  lines and every name is global; `this` appears in exactly three places, a claim about this
  line, a world record about this line (what it wrote, which the truth stratum keys by line),
  and the decision about this line. A declaration binds `this` whether it is a claim or a world
  object. Free names in a `#=` are atoms assay knows: the map's names,
  claim atoms, and names the `#=` introduces, which assay mints. A trailing `for` clause on an
  outcome is its command's scope.

### § 2.2-the-harness-and-the-unit

Assay owns exactly the sh-side structure, as one Alloy module every generated module opens
[ACKED 2026-09-27]:

```alloy
module assay
sig Shword { class: set Class }
sig Class {}
abstract sig Claim {}
sig Line { above: set Line, speech: set Claim, cmd: one Shword, argv: seq Shword }
```

- Names: no harness field is an Alloy 6 reserved word (`before` is one, a past-time operator,
  hence `above`), and no sig that opens the harness reuses a harness field name, since Alloy
  refuses a join through a name two sigs share, forward over a union and reverse alike.
  `Claim` is abstract so that no atom exists outside the claim base the shared tier declares;
  otherwise the adversary mints bare claims in every speech set.

- A `Shword` is one atom per distinct literal shell word, or per name a `#=` introduces.
- A `Class` is an opaque category of strings [TYPED]: arbitrary, never parsed, never meaningful
  to assay. It exists so two claims can agree about two different literals as one kind of text
  without anyone writing a parsing rule; where two claims can share the literal itself, they
  just do. Membership is only ever stated, by a braced map component or by a spec's fact. A
  braced word is a join node, not a quantifier: a book stays concrete, and a universal
  statement over a class is a `check` in the spec.
- A `Claim` is what a `.` line loads, by file stem, or what a `#=` declares. Assay never reads a
  claim's fields, and it is the spec's business that a claim in force cannot be flipped by the
  solver (the strawman does it with one fact: a claim not otherwise computed is true exactly
  where some line has it in force). That fact is what lets a measurement, "this line's check said
  yes", be a claim declared on the line rather than a switch the adversary can throw.
- **`unit-is-the-logical-line`** [ACKED] — a `Line` is one complete command as sh reads it,
  continuations joined, because that is where a comment attaches. Alloy holds four things about
  it: its order among the book's lines, the claims in force at it, and its decomposed words as
  `cmd` and `argv`. Decomposed structure is assay's to grow (parts and the operators between
  them, redirects, a substitution tree) so that spec text never grows string-splitting; literals
  are never exposed. At v0 assay hands the map line to the existing syntax crate's lexer, takes
  the first word as `cmd` and the rest as `argv`, and polices nothing [TYPED: least work].

Beside the harness, the spec tier owns two shared halves, and assay places them without reading
either [TYPED 2026-09-28, "keeps assay a thin preprocessor"]. The **prepend** half (STRAWMAN
`specs/shared.md`) is a module assay opens beneath every document, so no document declares the
truly global things [ACKED]: what every specification talks about, from `Speaker` and the claim
species' base through the answer order, the verdict subsets, and the truth default. The
**append** half (STRAWMAN `specs/shared-laws.md`) is what every specification must satisfy: the
generic laws, stated over names such as `answer`, `wrong`, and `support` that each document
defines. Alloy resolves names only downward through `open`, so a law that names a function the
document defines can be shared only by concatenation: assay splices the append half, as text,
after the document's own definitions into its laws module (§ 2.3). A document may use either
half or both. The append half's laws name things each document defines (an `answer`, a
`wrong`, a `support`; the names are the spec tier's own convention, chosen by whoever writes
that half), and Alloy, not assay, refuses a document that leaves one undefined, in that
document alone; the scope each generic law runs at is the append half's to spell, once, for
every document. Nothing assay owns appears in either half; everything either half says is
spec-tier content and not this document's, and anything less than tree-global is opened
explicitly by the spec that wants it.

### § 2.3-the-generated-modules

One spec document becomes one directory of Alloy modules [CONDUCTOR, STRAWMAN layout]:

- `species.als` — opens `assay` and `shared`; every `alloy` fence line that is not a claim atom.
- `words.als` — opens `shared`; one atom per distinct literal on any map line in the document,
  per braced class, and per name a `#=` line or a claim atom's body introduces, with the class
  memberships the map lines state and no others. An introduced name is an identifier that no
  declaration in the document, the shared halves, or the harness binds, that is not an Alloy
  keyword, and that is not a quantifier's or `let`'s bound name; a cell a claim names
  (`cells = pkg_index`) is the common case. Nothing else declares a literal. A misspelled
  declared name therefore becomes a word and fails in Alloy as a type error; a misspelled
  word becomes a second word, visible only in the report's word table.
- `claims.als` — opens `species` and `words`; every `one sig … extends <sig under Claim>` atom
  from an `alloy` fence, and one named set per load file, so a book's loads are set expressions.
- `laws.als` — opens `species` only, so the claim universe is free; every spec-authored `check`
  that carries a scope clause, and every spec-authored `run` that does other than a corpus
  check's twin (a premise twin of a law written without its own clause takes its law's), followed by the shared append half spliced
  verbatim (§ 2.2).
- `book_corpus.als` — the corpus book: a generated book of one line, the null command `:`
  (a real sh word nothing describes, so no claim matches it and no species fact fires; its
  word is minted by assay under a name of assay's own, STRAWMAN `assay_colon`), whose
  speech is every claim atom the document declares. Every spec-authored `check` written without
  a scope clause is an outcome of that line, so it runs at exact bounds over the actual claims
  with all of them in force, through the same mechanism as any book (§ 2.4); truth-in-force is
  defined there, which a lineless universe cannot offer [ACKED 2026-09-28]. A corpus check's
  premise twin lands here beside it. The module ends with the corpus `every_line` conjunction
  (below) and the inhabitation run `run book_corpus {}`.
- `book_<name>.als` — opens `claims`; one per book fence (§ 2.4): the line checks `line_<n>`, the
  book's `every_line` conjunction, and the book's run, named after the book.
- `assay.als` and `shared.als` — the harness and the tree-global module, written into the same
  directory. Every module is a root Alloy runs on its own, and Alloy resolves every `open`,
  from any module in the graph, against the root file's directory alone (an opened module
  never resolves relative to itself); one flat directory per document, module name equal to
  file name, is the layout that resolves without a models path.

The species-versus-claims split is syntactic (a `one sig` whose ancestry reaches `Claim`); its
purpose is the scope-minimum limitation of § 1.2. The scope-clause rule for laws versus corpus
checks is STRAWMAN [CONDUCTOR]: it keeps the document valid Alloy, so the Analyzer's Markdown
mode still opens it, and it reads naturally, since a law states its scope and a corpus check's
scope is the corpus. An unscoped `run` is a corpus outcome only as a premise twin: one not named
`<check>_premise` after a check of the same document is a refusal, since as an outcome its
meaning would invert from "some world has this" to "every world has this" [the 311 arc asked].

**The emitted text** [ACKED: "ack comment-strip"; CONDUCTOR `30Yf:key-hashes-what-the-jvm-parses`].
Every generated module is comment-free and carries one item per line: each paragraph's tokens on
one line, one space wherever the source separates two tokens and none where it does not (so
`this/A` and `1..3` stay the programs they are), string literals verbatim. What the tokenizer
cannot split with certainty (an unterminated string or block comment, a quote glued to an
identifier, a control byte) is a refusal, never a guess. The line map is a sidecar,
`assay-map.json` in the same directory: for every emitted line of every module, the spec file and
line each column span came from, so an Alloy message, a lint, or a counterexample walks back to
the document [TYPED 2026-09-28: a line-mapping culture]. The key (§ 2.7) hashes exactly the
bytes Alloy parses, and those bytes carry no comment, so rewording a comment or reflowing a
paragraph moves no key; a normalisation that went wrong shows as a parse error or a moved
verdict, never as a silent key match. A compile removes the modules the previous map listed and
this compile no longer emits, and nothing else in the directory.

**`every_line`, the whole-book conjunction** [ACKED, `30Yf:book-conjunction-entails-the-lines`].
Each book with two or more line checks that carry no `for` of their own gets one more check,
`every_line`, whose body is those checks' bodies verbatim, premises included, each in a block and
joined by `and`, at the book's own scope clause. Validity distributes over conjunction at one
scope over one set of facts, so a green `every_line` makes every member green (§ 2.5), and a red
one leaves the line checks to find the first red line. A line with its own `for` stays outside it
and runs alone. The corpus book gets the same conjunction over its corpus checks written inline
(`check X { … }`, not naming an assertion, no `expect`). Its members share one module and one
scope clause by construction, which is the whole of what the entailment rests on
[CONDUCTOR `30Yf:book-conjunction-assay-invariants`]. Two commands of one label in one module
are a refusal: the lock's row is `(module, name)`, and Alloy accepts the duplicate.

### § 2.4-books-lines-speech-and-outcomes

A book fence compiles as follows.

- Literals, classes, introduced names, and class memberships are `words.als`'s, shared by every
  book in the document; a book module declares none.
- Each line whose `#=` holds an outcome is a `Line` atom with `cmd`, `argv`, and `above` (the
  lines above it). Every `#=` declaration is emitted at module level. Every `#=` world fact is
  emitted as a fact, verbatim.
- **`mech-speech-is-per-line-data`** [ACKED] — each line's `speech` is the union of the claims
  the `.` lines above it load plus every claim declared on a line at or above it. A later load
  displacing an earlier one is the analyzer's load model's to define and is not modelled at v0;
  claims are a set. It is data the book module states, computed today by that rule and by the
  analyzer's own load model later; nothing in a spec learns how it was assembled. Claims are atomic [TYPED]: a load brings
  a set of them and there is no partial override to model, since an oracle either handles an
  input shape or declines it whole.
- **`mech-outcomes-are-checked`** [ACKED 2026-09-27] — each `Line` becomes a `check` whose
  conclusion is its outcome with `this` substituted and whose premises are the outcomes of every
  line above it, so the first red names the line and an earlier decision is never the adversary's
  to revisit. Each book also becomes one `run` asserting every outcome together, so no check
  passes because the facts contradict. A check with a counterexample is red; a book run with no
  instance is red.
- **`mech-scope-is-spelled-in-alloy`** [ACKED 2026-09-27] — assay sizes exactly the four kinds it
  owns (`Shword`, `Class`, `Line`, `Claim`) on every command it generates. Every other kind's
  ceiling is the spec's, since assay cannot know how large a key or a referent may need to be,
  and Alloy offers no document-wide scope, only a clause per command. So the spec spells it as a
  command with an agreed name and an empty body whose only content is its scope clause, and
  assay copies that clause onto the commands it generates (book outcomes, book runs, corpus
  checks): `run bookScope {} for 12 but 4 Int` in `specs/shared` is every book's default; a
  document's own `run bookScope {}` overrides it for every book in that document;
  `run bookScope_<book> {}` in the owning document overrides one book; a trailing `for` on an
  outcome line overrides one command. Most specific wins. A scope is paid per command, so a
  large book's ceiling costs only that book's commands and never the small ones beside it. The
  ceiling caps one world, not a total across books; the headroom above what a fixture names is
  what the adversary builds counterexamples from, so the shared number is the common case plus
  headroom, and the rare large book takes the escape. Wherever a clause sets `Int`, assay also
  sets `seq`: Alloy clamps a sequence's length to the largest integer the bitwidth admits, and
  clamps it silently, so an unspelled `seq` bound is a hidden cap on `argv`, never an error.

Assay derives no claim and recognises no verdict. What makes a line converged, how a resolution
is derived, and what `Elided` means are all facts and functions in the shared module or the
spec, over the relations assay supplies.

### § 2.5-the-checks-assay-runs

1. **Laws**: every `check` in `laws.als`, over the free claim universe, at the scope it states.
2. **Corpus checks**: the outcomes of the corpus book's one line, at exact bounds over the
   actual claims, every claim in force.
3. **Vacuity twins**: a `check X` is paired by name with a `run X_premise` in the same module,
   which must be satisfiable; a check with no twin is reported `premise: absent`, never green. A
   book line's check, and a corpus check without a twin, has its book's run as its witness, and
   the report gives that run's result as the check's premise (§ 2.7).
4. **Book outcomes**: every line's `check` (§ 2.4).
5. **Book satisfiability**: every book's `run`, the corpus book's included.
6. **Conjunctions**: each module's `every_line` (§ 2.3).

**How a pass answers a row.** `--check` and `--write` compile, parse every module once in one
adapter child per document (§ 3), and answer each command by the first of these that applies:

- `platform-fail` when Alloy refused, on this platform, a library module the jar bundles (the
  Windows jar and `util/natural`); nothing about the model is learned.
- `unsupported-here` for a `1.. steps` command, which needs a complete temporal checker the jar
  does not ship.
- **cached**: in the hot and gate tiers, a row whose committed result is definite and whose key
  is unchanged is the committed row, unsolved [ACKED `30Yf:key-composition`].
- `deferred`: in the hot tier, a row whose last result was a timeout or an out-of-memory, or whose
  recorded translation is larger than the deferral threshold (§ 3), is listed and never green;
  the hot loop never produces a timeout by design [TYPED: "a vertical pyramidal slice"].
- **entailed**: in the hot and gate tiers, a member of an `every_line` that completed
  `no-counterexample` is `no-counterexample` without a solve. A conjunction that is red, timed
  out, or errored entails nothing, and its lines are answered one by one.
- `not-run` when the batch's time is spent before the command starts.
- **replayed**: in the hot and gate tiers, a row whose last solve found an instance (a
  counterexample, or a run's witness) evaluates that instance against the current command and
  facts before solving; still satisfying, it is a genuine counterexample or witness at once
  [TYPED: "fantastic, hard ack"]. The instance is used only under
  `30Yf:replay-guard-declarations-byte-identical` [CONDUCTOR]: Alloy's command formula carries
  the explicit facts and the claim and none of the declaration constraints (signature facts,
  multiplicities, abstractness, subset parents), and its instance reader enforces none of them,
  so an instance is replayed only when every paragraph of the loaded closure other than a
  `fact`, an `assert`, or a command is byte-identical to the closure it came from and the scope
  string is identical. Then a `fact`, a claim, or a world-fact edit replays, and a `pred`, `fun`,
  `sig`, `enum`, or `open` edit re-solves. Three belts on the instance itself: the same bitwidth;
  every `exactly N S` bound met by the atoms of `S` and its descendants (instance XML lists an
  atom under its most specific signature only); every signature and field it names still
  declared. Temporal commands are included, on the builder's verification that instance XML
  round-trips a lasso trace and that the evaluator reads the whole trace (`30Yf` § 6).
- **fresh**: otherwise the command is solved, under the tier's caps.

**Targeted runs** [TYPED: "a vertical pyramidal slice"; `30Yf:run-targeted-is-a-slice-not-a-block`].
`--module <m>` runs every command of one generated module. `--only [<module>.]<command>` runs the
command, its `<command>_premise` twin, and, in a book module, the book's run (in `book_corpus`,
`book_corpus`), which is what the green needs beside it to mean anything; in `laws` a law without
a twin is already `premise: absent`. Without a module, `--only` names the one module holding
that command and refuses when none or several do. A targeted run checks and never writes: named
without a mode it is a `--check`, and `--write` with a target is a usage error.

**Tiers** [ACKED; the numbers are § 3's]. `--hot` is the author's loop: cached rows, deferral,
entailment, replay, and a small per-command budget. `--gate`, the default and the completion
gate's, is the same without deferral and with a larger budget. `--official` is the from-scratch
belt: keys ignored, no replay, no entailment, every line solved on its own beside its
conjunction, at the project's ceiling budget and heap, several children in parallel; a
conjunction whose verdict disagrees with its lines' own (green against a red line, or red
against all-green lines) is a finding against the construction and exits 1 [CONDUCTOR, ruled at
the lane's report].

### § 2.6-lints

Solver-free except the last, and only what compilation needs; each is a named refusal in the
report (exit 2):

- `join-key-coherence`: one literal carries at most one name across the document, and one name
  covers at most one literal, munged names included (the join key of § 2.1);
- `map-word-count`: a `#}` line has as many components as its command has words;
- `map-line-follows-command`: a `#}` line follows its command directly, every command has one,
  and a `#=` line follows a mapped command;
- `fence-header`: an `sh` fence opens with `# <identifier>.sh`;
- `no-stray-comments`: an `sh` fence carries no other `#` line;
- `load-stem-resolves`: every `.` line resolves to a load file or a claim atom;
- `this-on-atomless-line`: a declaration mentions `this` only on a line that has an outcome;
- `parent-is-a-known-sig`: every `extends` or `in` parent is a sig declared somewhere assay reads
  (or `univ`, `Int`, `String`);
- `unscoped-run-is-a-premise-twin`: an unscoped `run` is named `<check>_premise` after a check of
  the document (§ 2.3);
- `label-is-unique-in-module`: no two commands of one label in one generated module (§ 2.3);
- `tokenizer-is-certain`: every emitted paragraph tokenizes with certainty (§ 2.3);
- `alloy-parses`: every generated module parses and typechecks under Alloy's own parser, through
  the adapter (assay reimplements none of it); one finding per distinct message, naming the
  modules it stopped and walked back through the line map to the spec line.

A free name in a `#=` that nothing declares is not a lint: it is minted as a word (§ 2.3).

### § 2.7-the-lock-the-report-and-the-exit-code

- **The lock** [ACKED, with the human's nack of a generated index in the `SLUGS.md` style]: one
  committed JSON file per document beside it, `<stem>.lock.json` for `<stem>.assay.md`: an array
  whose first element is `{"schema": 2}` and then one row per command, one per line, so its git
  diff is a row diff. It is compared in both directions; the commit that carries it is the
  ceremony, as with `301`'s catalogue lock, and rows are never hand-edited. Not merged with that
  lock in this experiment.
- **Results** [ACKED `30Yf:lock-definite-versus-unmeasured`]. A *definite* result, `sat`,
  `unsat`, `counterexample`, or `no-counterexample`, is a fact about the model at that scope that
  no budget changes. An *unmeasurement* says nothing about the model and is recorded as its own
  kind [TYPED: a platform failure is separate from a timeout, with the same semantics]:
  `timeout` (with `phase`, `translating` or `solving`), `out-of-memory`, `platform-fail`,
  `unsupported-here`, `not-run` (the batch's time ran out before it started), and `deferred`
  (the hot tier's, § 2.5). `error` is neither [CONDUCTOR `30Yf:lock-oom-and-error-rows`]: a
  deterministic refusal (a missing scope is Alloy's error at solve time), never skipped, never
  carried across a changed key.
- **Columns**: `module`, `name`, `kind`, `scope` (Alloy's own rendering of the clause),
  `result`, and where they apply: `phase`; `budget`, the per-command CPU cap in seconds the row
  was measured under (a command cut short by the batch clip records the budget it actually had);
  `heap`, in megabytes, on a timeout or an out-of-memory; `size`, the translation's primary
  variables, variables, and clauses, on a fresh solve and on a timeout past translation (a
  replayed or entailed row carries none); `key` (below); `platform` (`windows`, `linux`, or
  `macos`: recorded, never keyed, since verdicts are platform-independent and parse failures are
  not [CONDUCTOR `30Yf:key-platform-recorded-not-keyed`]); `message` on an error or a platform
  failure, with the out directory's paths made relative. No timing: wall-clock and solve time
  are report columns. The premise is not stored either: the report derives it from the twin's
  row, or the book's run's (§ 2.5).
- **The key** [ACKED `30Yf:key-composition`]: SHA-256 over the loaded closure exactly as Alloy
  loaded it (every module, bundled `util/*` included, each with its command paragraphs removed
  textually), the command's own generated text, its scope as Alloy renders it, the jar's pinned
  digest, the adapter source's digest, and the effective options the adapter reports having used
  (solver, symmetry, skolem depth, overflow, unrolls, decompose mode and threads,
  partial-instance inference, core minimisation and granularity), never a flag vector. A solve
  that reports other options than its key was computed under is an `error`. Stripping commands
  is sound because a command is a query: it binds no name, adds no constraint another command
  inherits, declares no atom, and opens no module; the premises a book line or a conjunction
  embeds are in the command's own text, which the key hashes
  [`30Yf:key-why-stripping-commands-is-sound`].
- **Matching** [ACKED `30Yf:lock-asymmetric-match`]: each computed row against the committed row
  of the same `(module, name)`. A definite result matches only its equal, at any budget, and an
  equal `size` across a changed key proves nothing; an unmeasurement never matches a definite
  result on a changed key, which unlocks the row as unmeasured rather than leaving it passing
  [TYPED: "unlocks as 'unmeasured', not stays-passing"].

  | committed row | computed | standing | exit |
  | --- | --- | --- | --- |
  | any | definite, equal to the committed result | `green`, or `accepted-red` | 0 |
  | any | definite, another result (an unmeasurement or `error` committed included) | `mismatch-moved` | 1 |
  | none | definite or `error` | `mismatch-new` | 1 |
  | `error`, same key and message | `error` | `accepted-error` | 0 |
  | anything else | `error` | `mismatch-moved` | 1 |
  | definite, same key | an unmeasurement | `carried` | 0 |
  | `platform-fail` | `platform-fail` | `accepted-unmeasured` | 0 |
  | an early unmeasurement, `not-run`, or `deferred`, same key | an unmeasurement | `owed` | 4 |
  | a full timeout or out-of-memory, or `unsupported-here`, same key | an unmeasurement | `accepted-unmeasured` | 0 |
  | none, or any other with a changed or unknown key | an unmeasurement | `unmeasured` (`deferred` if deferred) | 4 (a deferral: 0) |

  A committed row the whole run no longer produces is `gone`, and a missing or unreadable
  lock is a mismatch; both exit 1.
- **Early and full** [TYPED, refined; `30Yf:lock-early-versus-full-timeout`]: a timeout below the
  ceiling budget, or an out-of-memory below the ceiling heap (the official tier's, § 3), is
  *early* and owes exactly one thing, a run at the ceiling; at or above it the row is *full* and
  owes analysis (`size` and `phase` say whether it is translation-bound, the specification's to
  restructure, or solve-bound, the solver's or a change of claim) or acceptance. Full is relative
  to the current ceiling, so a raised ceiling demotes accepted rows to early and they owe their
  run again with no special case.
- **Accepted residue**, listed by the report under its own headings and never a green
  (`30Z:pos-halo-is-the-hazard`): accepted reds, accepted unmeasurements (full timeouts and
  out-of-memories, platform failures, unsupported commands), and accepted errors (an `error` on
  the same key with the same message).
- **Writing** [ACKED `30Yf:lock-tier-invariant`: a lower tier never makes the lock worse].
  `--write` at the hot or gate tier writes every row it computed definite or `error`; for a row
  it computed as an unmeasurement it keeps the committed row, or writes none. `--write
  --official` writes every computed row except that an unmeasurement never replaces a committed
  definite row whose key is unchanged, since that verdict is known and the row is carried with
  its old key and result; on a changed key or a new row the official tier writes the
  unmeasurement, so a full timeout on new text is recordable for acceptance. Rows the document no
  longer has are dropped. `--write` prints the rows it changed on stderr (`+` new, `~` moved,
  `-` gone) and exits 0 once the lock is written, or 1 on the official tier's construction
  finding (§ 2.5). A targeted run never writes.
- **A schema-1 lock** (no `schema` element) is read for its results alone: its `budget`, `heap`,
  `size`, and `key` read as unknown, and its `premise` and `hash` columns are ignored. No such row
  matches a key, so nothing is cached, carried, or owed from it: every row re-solves, a definite
  result matches by equality, and an unmeasurement against it is `unmeasured` until a run records
  a definite result or an official tier records the row. Any `--write` renders schema 2.
- **The report** is one JSON object per document on stdout: the spec, the out directory, the
  generated files, the word table, the class table, the corpus and books summaries, the lint
  results, and on `--check` or `--write` every command's row with its `premise`, `provenance`
  (`fresh`, `cached`, `entailed`, `replayed`, or `unrun`), `standing`, `wall_ms`, `solve_ms`, and
  a `note` (on a timeout, how far translation got), then the lock's summary: the tier, the
  status (`matches`, `mismatch`, `unmeasured`, `missing`, `unreadable`, `written`), and the rows
  under each standing, the `gone` rows, and the official tier's `construction_disagreements`.
  `--staged` adds `key_diff`. No prose, no suspicion, no ranking [TYPED]. A person or model who
  wants the counterexample opens the generated `.als` in the Analyzer, or runs `mise run alloy
  -- --instances --command <name> <module.als>` on it.
- **Progress** is stderr's, loud by default like any tool [TYPED 2026-09-29: agents and hooks
  pass `--quiet`, and an agent that forgets is a praxis defect, not the tool's]. A `--check` or
  `--write` prints a start line before the survey; a plan line after it (tier, commands,
  modules, children, the per-command caps, the batch cap, and how many rows the committed lock
  last recorded as a timeout or an out-of-memory); for each fresh solve its start (the budget it
  has, the row's last result and size), its translation's end (clauses, primary variables), a
  still-alive line every five minutes (wall time waited; CPU against the budget once the child
  has started; translating, or solving with the clause count), and its result with the count of
  rows answered so far over the total; and an end line on every exit path, with counts by
  provenance and by result. Every line carries the time since the pass began, so the last line
  on a dead terminal says how long it ran; durations render as `41.3s`, `12m08s`, `2h14m05s`.
  Rows answered without a solve print nothing and advance the count. Nothing predicts: no
  estimate, no fraction of work done, only effort against the caps [TYPED: a cap-derived bound
  reads as a prediction]. `--parse`, `--staged`, and a bare compile print none of it. `--quiet`
  silences progress and never a finding: the preflight line, the diff, the key-diff warning, and
  errors stay. `--help` lists the flags and the trailing caps.
- **Exit codes**: `0` every row green or accepted residue [TYPED 2026-09-28: a set of reds fully
  acked by the lock is a pass] · `1` a mismatch (a moved result, a new row, a row gone, a
  missing lock) or the official tier's construction finding · `2` a lint refusal, compile or
  parse, or a usage error · `3` the solver could not run (no JVM, a refused adapter compile,
  preflight's refusal) · `4` no mismatch, but rows went unmeasured or are owed a run at the
  ceiling, distinct from 1 because the reader's next act differs: run the official tier, where 1
  says fix the model [CONDUCTOR, ruled at the breakpoint]; rows the hot tier defers do not count
  toward it · `75` the heavy-work lock is held elsewhere: contention, not a failure; do other
  work. Over several documents the most severe wins, in the order 0, 4, 1, 3, 75, 2.

### § 2.8-two-examples

A species with its meaning fact and two claims under it, as written in a spec's `alloy` fence
(the species base `MDecl extends Claim` comes from the shared module):

```alloy
sig Yields extends MDecl { of: one Class, under: one MScheme, to: lone MScheme }

one sig tessa__a_slash_separated_path_names_the_inode_of_its_last_entry extends Yields {} { speaker = tessa  of = slash_path  under = Path  to = Inode }
one sig tessa__a_bare_word_is_not_a_path extends Yields {} { speaker = tessa  of = bare_word  under = Path  no to }
```

The first line lands in `species.als`; the two atoms land in `claims.als`. A `check` written
with a scope lands in `laws.als` and runs over a free universe of claims; one written without
becomes an outcome of the corpus book's null line and runs over exactly these, all in force.

A book, as written in an `sh` fence. The `stat` line names two literals and states the world by
name; the `chmod` lines put the same literals in a class; the second `chmod` line declares the
measurement its verdict rests on, as a claim, and then states its outcome:

```sh
# siblings_across_filesystems.sh
. ./tessa_fs.sh
. ./simon_fs.sh
. ./stdlib_boot.sh
. ./carl_chmod.sh

   stat -c '%i %d' /srv/a/shared /var/lib/other
#} stat dash_c fmt_i_d a_path d_path
#= w.inode_x.scheme = Inode and w.inode_z.scheme = Inode
#= w.a_path.reaches = w.inode_x.reaches and w.d_path.reaches = w.inode_z.reaches
#= w.inode_x.worldParent = w.fs_1 and w.inode_z.worldParent = w.fs_2 and w.fs_1.worldParent = w.boot_1 and w.fs_2.worldParent = w.boot_1

   chmod g-w /srv/a/shared
#} chmod g-w {slash_path}
#= this in Ran

   chmod g+w /var/lib/other
#} chmod g+w {slash_path}
#= one sig carl__the_file_at_d_path_has_the_mode extends Verdict {} { of = this }
#= this in Elided
```

`w.a_path` is the spec being explicit that a key is a first-class thing with a word, not a
string. The document's words module and the book's generated module, abridged:

```alloy
module words
open shared
one sig stat extends Shword {}
one sig dash_c extends Shword {}
one sig fmt_i_d extends Shword {}
one sig a_path extends Shword {}
one sig d_path extends Shword {}
one sig inode_x extends Shword {}
…
one sig chmod extends Shword {}
one sig g_dash_w extends Shword {}
one sig g_plus_w extends Shword {}
…
one sig slash_path extends Class {}
one sig bare_word extends Class {}
fact { class = a_path->slash_path + d_path->slash_path + … }
```

```alloy
module book_siblings_across_filesystems
open claims
fact { w.inode_x.scheme = Inode and w.inode_z.scheme = Inode }
fact { w.a_path.reaches = w.inode_x.reaches and w.d_path.reaches = w.inode_z.reaches }
fact { w.inode_x.worldParent = w.fs_1 and w.inode_z.worldParent = w.fs_2 and w.fs_1.worldParent = w.boot_1 and w.fs_2.worldParent = w.boot_1 }
one sig line_2, line_3 extends Line {}
one sig carl__the_file_at_d_path_has_the_mode extends Verdict {} { of = line_3 }
fact { line_2.cmd = chmod line_2.argv = 0->g_dash_w + 1->a_path no line_2.above }
fact { line_3.cmd = chmod line_3.argv = 0->g_plus_w + 1->d_path line_3.above = line_2 }
fact { line_2.speech = tessa_fs + simon_fs + stdlib_boot + carl_chmod }
fact { line_3.speech = line_2.speech + carl__the_file_at_d_path_has_the_mode }
check line_2 { (line_2 in Ran) } for 12 but 4 Int, 7 seq, exactly 20 Shword, exactly 2 Class, exactly 2 Line, exactly 17 Claim
check line_3 { (line_2 in Ran) implies (line_3 in Elided) } for 12 but 4 Int, 7 seq, exactly 20 Shword, exactly 2 Class, exactly 2 Line, exactly 17 Claim
check every_line { { (line_2 in Ran) } and { (line_2 in Ran) implies (line_3 in Elided) } } for 12 but 4 Int, 7 seq, exactly 20 Shword, exactly 2 Class, exactly 2 Line, exactly 17 Claim
run siblings_across_filesystems { (line_2 in Ran) and (line_3 in Elided) } for 12 but 4 Int, 7 seq, exactly 20 Shword, exactly 2 Class, exactly 2 Line, exactly 17 Claim
```

The `stat` line is the book's first command and states only world facts, so it is `line_1` and
has no atom; the two `chmod` lines are `line_2` and `line_3`. The `12 but 4 Int` is copied from
the shared module's `bookScope`, the `7 seq` follows from the `4 Int`, and the exact bounds are
assay's. Everything that makes `line_3` elide is the spec's: Carl's claim that chmod reads its
operand under `Path`, Tessa's claim that a `slash_path` under `Path` yields an inode, the spec's
derivation of the resolution from those, Carl's verdict claim on the line, and the shared
module's definitions of `Converged` and `Elided`. The adversary may not flip the verdict claim,
because a claim in force is true; it may not revisit line 2, because that is a premise; it may
only hunt for a world the fixture admits in which the questions about line 3 do not all spare.
Hand the same lines a bare word (`#} chmod g-w {bare_word}`) and Tessa's second claim declines,
and the same site is forced to guard. Assay saw literals, two classes, two lines, a declaration,
and a set of claims. The worked version of this, with all its books, is `notes/30Ya-strawman-2/`.

## § 3-implementation-sketch

The builder has latitude on everything not marked.

- **Architecture** [ACKED `30Yf:arch-mostly-rust-tiny-java`]: everything decidable from text and
  JSON is assay's Rust (keys, the lock and its asymmetry, tiers and budgets, ordering and
  deferral, the conjunction, the replay policy and its guards, reporting); the JVM keeps four
  verbs. Nothing reimplements Alloy.
- **The toolchain.** A mise-managed JDK and the Alloy 6.2 distribution jar, pinned by version and
  digest into mise's own store, never vendored into the tree (SyncThing is live above the repo).
  Their paths come from `mise where` (or `ALLOY_JAVA_HOME` and `ALLOY_JAR`), never from `PATH`,
  since a machine-global JDK earlier on `PATH` would win over the pin; they are cached in
  `<target>/alloy/jvm-paths.json`, keyed by the two pin lines of the root `mise.toml`, so a warm
  invocation spawns no `mise`.
- **The adapter**, `spike/verify/alloy/AlloyAdapter.java`: one Java class speaking JSON lines on
  stdin and stdout, four verbs. `parse` takes a root module and returns the closure Alloy loaded
  (each module's path and text), the command list (label, kind, `expect`, scope, bitwidth,
  unbounded steps, whether Alloy synthesized it), the signatures and their fields, and the
  effective options; `parse-only` parses and returns the module name; `solve` runs one command
  and returns the result, the translation size, the options it used, and on request the
  instance as XML or text; `eval` reads a stored instance and evaluates a command's formula
  conjoined with every reachable fact. A root parses once and stays cached in the child. The
  child emits a tick with its own CPU time every second, since Rust's standard library reads no
  child's CPU portably, and a `translated` event when translation ends; it halts itself when its
  parent is gone, and reports an out-of-memory as its own reply. It is compiled once by the pinned
  `javac` into `<target>/alloy/adapter-<the first 16 hex digits of its source's SHA-256>/`, so an
  edit to the source compiles a fresh class and the key moves with it. Kodkod's native-library
  probes are kept off stderr.
- **One child per document** [TYPED soft ack: "if it chafes, drop it";
  `30Yf:arch-adapter-loop-is-the-daemon`]: an assay pass spawns one adapter JVM per document,
  parses each module once, and runs its commands in sequence. Rust enforces every budget: a
  command's CPU from the child's own ticks since its start, its wall-clock from the moment the
  request was sent, polled every 200 ms; a command over either is killed with the child, and the
  next call respawns it, losing only the parse cache. The phase of a timeout is `solving` once a
  `translated` event arrived, `translating` otherwise. The child JVM runs with the tier's heap
  (`-Xmx`), processor count (`-XX:ActiveProcessorCount`), and `-XX:+ExitOnOutOfMemoryError`. No
  daemon outlives the invocation, and there is no socket.
- **The runner**, `mise run alloy`: a Rust subcommand over one adapter child for all its files,
  keeping the runner's command line, row shape, and exits: it parses each root module, runs each
  command, and prints one JSON row per command; a module living outside the root's directory is
  served with `--open` at the spot Alloy resolves `open` to, rather than copied; `--command`
  refuses a name that matches nothing (exit 2); a command carrying `expect` is judged by
  agreement with it and its row carries the `expect`, never inside `scope`; `--parse-only`
  parses alone. It is bounded on every axis a solver can exhaust [TYPED 2026-09-28: a command
  that never returns puts an LLM to sleep for hours]: a wall-clock cap per command (default
  120 s), a CPU cap (default the wall cap), a heap cap (2048 MB), a processor count (2), and a
  batch cap (540 s, under the 600-second ceiling an agent harness puts on a foreground command)
  after which commands not yet started are `not-run`; the runner's batch cap stops new starts
  only. The runner kills the live child as it exits. An out-of-memory is an `error` row there.
  It takes a directory as well as files, and skips a module whose only command is the `Default`
  Alloy synthesizes for a command-less module. It preflights in-process, before the lock, for
  disk and for the memory its own `--heap` implies (the heap plus the 512 MB a child costs beside
  it; one table in `preflight.rs` stays the home of every bound), and runs under the
  repository's global heavy-work lock (`internal-tooling exclusive`),
  a file in the user's cache directory that names its holder, so a second heavy task on the same
  machine is refused with the holder's name (exit 75) and told to do other work rather than wait.
- **The tiers' caps** [ACKED; the ceiling CONDUCTOR, ruled at the breakpoint]:

  | tier | CPU per command | wall per command | heap | batch | children |
  | --- | --- | --- | --- | --- | --- |
  | `--hot` | 120 s | 240 s | 2048 MB | 540 s | 1 |
  | `--gate` (default) | 600 s | 1200 s | 2048 MB | 540 s | 1 |
  | `--official` | 1800 s, the ceiling | 3600 s | 4096 MB, the ceiling | 8 h | 1 to 4 |

  Each child may use two processors. The official tier runs as many children as the machine's
  available RAM holds at the heap plus 512 MB each, at least one and at most four, each taking
  whole modules from one queue; the pass preflights for one child's worth before it starts. The
  batch cap is a hard bound on wall time [CONDUCTOR, ruled at
  the lane's report]: a command starts only with its wall cap clipped to what the batch has left,
  one cut short by the clip is a `timeout` at the budget it had (which reads as early and owed),
  and `not-run` is for a command never started, so a started command never outlives the batch.
  The official tier's cap exists because a from-scratch pass over a grown specification is
  otherwise unbounded in hours [TYPED 2026-09-29: "clearly the official tier needs some kind of
  cap"]; eight hours is a conductor's figure. The tier is not resumable, on purpose: the
  per-command ceiling is the shared law that makes a timeout a design-meaningful residue rather
  than a reason to churn the Alloy, and the batch cap means only "stop and move on". A capped
  `--write --official` writes what it measured, records the rows it never started as `not-run`
  where the key changed or no row existed, keeps a committed definite row otherwise, and exits 0;
  the next `--check` reads those rows as owed, and the route to them is a longer
  `--batch-timeout`. Caps after assay's own `--` override the tier: `--cpu` (alone, it sets the
  wall cap to twice itself), `--timeout` (the wall cap; alone, it sets the CPU cap to itself too),
  `--heap`, `--procs`, `--batch-timeout`. Assay always solves with sat4j.
- **Ordering and deferral.** Modules run cheapest first, by the smallest translation size the
  lock records for any of their commands (a module with none recorded runs last); within a
  module the `every_line` conjunction runs first, then its commands by recorded size. The hot
  tier's deferral threshold is 2,000,000 clauses, a starting figure, not a measured one.
- **The Rust**, under `spike/crates/internal-tooling/src/`: `alloy_jvm.rs` (the JVM paths and
  their cache, the jar's and the adapter's digests, the platform name, directory arguments);
  `alloy_jvm/adapter.rs` (the child, its four verbs, budget enforcement, the compiled class);
  `alloy_jvm/runner.rs` (`mise run alloy`); `assay.rs` (the command line, the compiler, the
  module layout, exit severities); `assay/sh.rs` (`sh` fences and map lines); `assay/alloy.rs`
  (the tokenizer, heads and binders, the munge, command stripping); `assay/emit.rs` (the
  one-item-per-line rendering and the line map); `assay/book.rs` (the conjunction);
  `assay/key.rs` (the key); `assay/replay.rs` (the replay guard, the stored instances under
  `<out>/instances/<module>/<label>.xml`, and the fit belts); `assay/tier.rs` (tiers, caps, the
  ceiling, the deferral threshold); `assay/lock.rs` (the lock's rows, schemas, matching, and
  writing); `assay/drive.rs` (the survey, slices, ordering, and the per-row decision of § 2.5);
  `assay/pass.rs` (one document's pass, the report, the diff, the exit). `json.rs` and
  `sha256.rs` are the crate's own, standard library only.
- **Gate placement.** Pre-commit, hk's `assay` step over staged `specs/**/*.assay.md` and
  `specs/**/*.lock.json`, runs `--staged` [TYPED: parse and key-diff belong there]: it compiles
  the staged bytes into `<stem>.staged/`, parses every module through a fresh adapter child at a
  1024 MB heap, and diffs the keys against the staged lock, warning on stderr how many rows the
  commit leaves unmeasured; it never solves, takes no heavy-work lock, and exits 0 unless a lint
  refuses. Builder completion, hk's `assay-lock` step (profile `slow`), runs `--check --quiet` at
  the gate tier over the same paths. `--parse` alone is Alloy's parse lint in one child, no
  heavy-work lock. `--check` and `--write` preflight in-process for the tier's heap plus one
  child's overhead, take the heavy-work lock, and exit 75 when another task holds it. The
  official tier's standup in CI (runner choice) is separate work.
- **Platforms** [TYPED `30Yf:arch-three-platforms`]: Windows, Linux, and macOS must be
  supportable; WSL is secondary. sat4j is pure Java and the default, and no design depends on a
  bundled native: minisat and glucose ship for Windows and Linux, the core-producing minisat and
  plingeling for Linux only, macOS and Apple Silicon unverified. A faster solver, if one ever
  earns its place, is an external DIMACS solver pinned per platform through mise like the jar and
  the JDK, driven by a small factory in the adapter; 6.2 has no generic external factory.
  Whether a timeout is translation-bound or solve-bound is the `phase` column's to say per row
  before any solver is chosen [CONDUCTOR `30Yf:arch-solver-choice-is-a-measurement`].
- **Compiler.** Rust, as a subcommand of `crates/internal-tooling` (which already reads the
  corpus for `slugs` and `docids`); a sibling crate under `spike/verify/` if it outgrows that. A
  Markdown fence lexer; the syntax crate's lexer for map lines; `#=` lifted verbatim, sorted into
  declaration, fact, or outcome by shape; the modules of § 2.3; the lints of § 2.6. Assay
  never parses Alloy beyond recognising declaration heads and binders: a `sig`'s name, parent,
  and field names; a `fun`, `pred`, `check`, or `run` name and its scope clause; a
  quantifier's or `let`'s bound names. That is what classifying declarations and minting
  words (§ 2.3) need, and nothing else is read. The line map in `assay-map.json` walks every
  emitted column back to the spec file and line it came from [TYPED 2026-09-28: a line-mapping
  culture, kept from the start]. `--check` never writes the lock; generated modules, their line
  map, and stored instances live under `<target>/alloy/<stem>/`, never committed.
- **Fixtures** [TYPED 2026-09-28]: assay's own fixture is a meaningless, Dorc-agnostic document
  under `internal-tooling`'s tests, compiled byte-for-byte against committed expected modules and
  their `assay-map.json`, with negative documents for the join-key, stray-run, and
  duplicate-label refusals; no committed test runs a JVM. The two strawmen,
  `notes/30Ya-strawman-3` (the outcome algebra past a wall; every command finishes in seconds at
  the shared ceiling of twelve) and `notes/30Ya-strawman-2` (the identity model's first cut;
  its per-line checks reach millions of clauses at the same ceiling, through the transitive
  closures in its separation predicate), are frozen exploration artifacts, never fixtures: their
  content about Dorc is apparently normative and partly wrong. Strawman-3 was compiled and run
  once by hand as the compiler's smoke (`notes/30Yd`: every verdict as the hand build's);
  strawman-2 compiles, and its frozen shared half's laws error under Alloy (`notes/30Ye`). Neither is
  promoted to the spec tier [TYPED nack]; turning 311 into a specification is separate,
  clean-context, product-focused frontier work.

### § 3.1-later-and-maybe

Not built; listed so nothing here is mistaken for forgotten. What is excluded outright as unsound
or risky (scope escalation, any size-equality pass, shared translation between a check and its
twin, slice reuse of unsat results) is `30Yf` § 11's.

- **The correspondence compiler.** A book fence is already a runnable sh file. When the
  product's kernel reaches the design a spec pins, the same file compiles to an e2e expectation
  (a loom session under the harness binary, the expected transcript derived from each line's
  outcome), and assay learns the one piece of Dorc knowledge it will need to map between tools,
  the outcome vocabulary [TYPED: eventually]. Until then and wherever the product disagrees, the
  lock stores the field-tier verdict as qualified intent in goblint's manner [ACKED]:
  `UNKNOWN!` for an unsoundness, `UNKNOWN` for intended imprecision, `TODO` for precision owed;
  nothing regenerates a stored verdict from product output.
- **Derived columns and views.** The rested-on set per spared line and the danger column per
  claim (the inverse of `restsOn` across all books) are evaluable against instances; a
  `--eval` of named expressions per command, and views over the lock, once the lock is boring.
- **The mutation lane** [TYPED: negative verification and mutation testing are a required
  guard]: line-drop mutants over the `alloy` fences with the laws and books re-run; a mutant no
  check or book kills is reported. Opt-in and slow.
- The official tier on a schedule or in CI, and larger scopes there; a deterministic budget unit
  (a SAT conflict limit, in the manner of Lean's heartbeats) refining `budget` so that
  "unmeasured at K conflicts" is machine-independent; unification with the catalogue lock;
  conditional loads (`if [ -f local.sh ]; then . ./local.sh; fi`) as two speech sets per later
  line.

## § 4-latitude-kept-open

What the first cut must not weld shut, each priced at one sentence [ACKED 2026-09-27 unless
marked]:

- Speech is per line and is data (§ 2.4); nothing derives it inside Alloy; the laws stay
  quantified over arbitrary claim sets. An admin's load displacing a foreign author's claim at
  every later line then needs no carve-out anywhere.
- A measurement is a claim, so a verdict claim's truth is by the default only until a spec
  models the state the check measures; then it is computed like a resolution's, and a lying
  verdict function becomes representable and attributable with no change to assay.
- The truth stratum is the spec's and is not welded static. Alloy 6's `var` sigs and step scopes
  are the route for the lifecycle species (311 § 3.3), for the two standups as two instants, and
  for time-of-check against time-of-use; the runner accepts temporal commands from day one
  because that is a flag, not a design.
- Fixture facts are sited (a world-fact `#=` sits on a line), which is the hook the temporal
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
