# 30Y — assay: design-tier models in Alloy

> Tier: LLM-authored plan (Fable conductor, from the 2026-09-27 sittings; round 30). Subordinate
> to the root docs and `spike/CLAUDE.md`; sibling of `notes/301` (minispec and dorc-verify, the
> code-tier instrument this extends downward to the design tier) and of `notes/30X` (the testing
> architecture the correspondence half will eventually ride). Its ledger is `notes/30Ya`; its
> evidence base is `.claude/research/design-model-mechanisation-prior-art/` (four fronts, every
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
`spec/shared.md`) is a module assay opens beneath every document, so no document declares the
truly global things [ACKED]: what every specification talks about, from `Speaker` and the claim
species' base through the answer order, the verdict subsets, and the truth default. The
**append** half (STRAWMAN `spec/shared-laws.md`) is what every specification must satisfy: the
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
  that carries a scope clause, followed by the shared append half spliced verbatim (§ 2.2).
- `book_corpus.als` — the corpus book: a generated book of one line, the null command `:`
  (a real sh word nothing describes, so no claim matches it and no species fact fires; its
  word is minted by assay under a name of assay's own, STRAWMAN `assay_colon`), whose
  speech is every claim atom the document declares. Every spec-authored `check` written without
  a scope clause is an outcome of that line, so it runs at exact bounds over the actual claims
  with all of them in force, through the same mechanism as any book (§ 2.4); truth-in-force is
  defined there, which a lineless universe cannot offer [ACKED 2026-09-28].
- `book_<name>.als` — opens `claims`; one per book fence (§ 2.4).
- `assay.als` and `shared.als` — the harness and the tree-global module, written into the same
  directory. Every module is a root Alloy runs on its own, and Alloy resolves every `open`,
  from any module in the graph, against the root file's directory alone (an opened module
  never resolves relative to itself); one flat directory per document, module name equal to
  file name, is the layout that resolves without a models path.

The species-versus-claims split is syntactic (a `one sig` whose ancestry reaches `Claim`); its
purpose is the scope-minimum limitation of § 1.2. The scope-clause rule for laws versus corpus
checks is STRAWMAN [CONDUCTOR]: it keeps the document valid Alloy, so the Analyzer's Markdown
mode still opens it, and it reads naturally, since a law states its scope and a corpus check's
scope is the corpus.

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
  checks): `run bookScope {} for 12 but 4 Int` in `spec/shared` is every book's default; a
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
   which must be satisfiable; a check with no twin is recorded `premise: absent`, never green. A
   book line's check has no twin of its own: its book's run is its witness, and the lock records
   that run's result as the line's premise.
4. **Book outcomes**: every line's `check` (§ 2.4).
5. **Book satisfiability**: every book's `run`.

### § 2.6-lints

Solver-free, and only what compilation needs:

- one literal carries at most one name across the tree, and one name covers at most one literal,
  munged names included (the join key of § 2.1);
- every free name in a `#=` is an atom assay knows;
- every `.` line resolves to a load file or a claim atom;
- every generated module parses under Alloy (Alloy's own parser; assay reimplements none of it).

### § 2.7-the-lock-the-report-and-the-exit-code

- **The lock** [ACKED, with the human's nack of a generated index in the `SLUGS.md` style]: one
  committed JSON file per spec beside it (STRAWMAN `<spec>.lock.json`), one row per command:
  module, name, kind, scope, result (`sat` · `unsat` · `counterexample` · `no-counterexample`
  · `timeout` · `not-run` · `error`), the premise twin's result beside its check (or `absent`),
  and a hash of the command's text. A timeout is a result, not a runner failure. Wall-clock, and on a
  timeout whether translation finished and how large the problem was (what decides between a
  ceiling too high and an encoding too costly), are report columns beside the row and never
  enter the lock, which is compared in both directions. `assay --check` recomputes and exits
  nonzero on a mismatch in either direction; `assay --write` rewrites it, and the commit that
  carries it is the ceremony, as with `301`'s catalogue lock. Not merged with that lock in this
  experiment.
- **The report** is the same rows plus the lint results and the generated file paths, as JSON,
  to stdout. No prose, no suspicion, no ranking [TYPED]. A person or model who wants the
  counterexample opens the generated `.als` in the Analyzer or runs `alloy exec` on it.
- **Exit codes**: `0` the computed rows match the committed lock row for row, result and premise
  included, whether those results are green or red [TYPED 2026-09-28: a set of reds fully acked
  by the lock is a pass] · `1` a mismatch in either direction, which is what a NEW red is · `2` a
  lint refusal · `3` the runner failed (jar, JVM, or the heavy-work lock held elsewhere). The
  report lists the reds the lock accepts as its residue, so a reader sees them without opening
  the lock; committing the lock is the ceremony that accepts a red.

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

one sig stat, dash_c, fmt_i_d, cat, proc_boot_id, chmod, g_minus_w, g_plus_w, a_path, d_path, inode_x, inode_z, fs_1, fs_2, boot_1, … extends Shword {}
one sig slash_path, bare_word extends Class {}
fact { class = a_path->slash_path + d_path->slash_path + … }
```

```alloy
module book_siblings_across_filesystems
open claims

fact { w.inode_x.scheme = Inode and w.inode_z.scheme = Inode }
fact { w.a_path.reaches = w.inode_x.reaches and w.d_path.reaches = w.inode_z.reaches }
fact { w.inode_x.worldParent = w.fs_1 and w.inode_z.worldParent = w.fs_2 and w.fs_1.worldParent = w.boot_1 and w.fs_2.worldParent = w.boot_1 }

one sig line_3, line_4 extends Line {}
one sig carl__the_file_at_d_path_has_the_mode extends Verdict {} { of = line_4 }
fact { line_3.cmd = chmod  line_3.argv = 0->g_minus_w + 1->a_path  no line_3.above }
fact { line_4.cmd = chmod  line_4.argv = 0->g_plus_w + 1->d_path   line_4.above = line_3 }
fact { line_3.speech = tessa_fs + simon_fs + stdlib_boot + carl_chmod }
fact { line_4.speech = line_3.speech + carl__the_file_at_d_path_has_the_mode }

check line_3 { line_3 in Ran }                            for 12 but 4 Int, exactly 20 Shword, exactly 2 Class, exactly 2 Line, exactly 17 Claim
check line_4 { line_3 in Ran implies line_4 in Elided }   for 12 but 4 Int, exactly 20 Shword, exactly 2 Class, exactly 2 Line, exactly 17 Claim
run siblings_across_filesystems { line_3 in Ran and line_4 in Elided } for 12 but 4 Int, exactly 20 Shword, exactly 2 Class, exactly 2 Line, exactly 17 Claim
```

The `12 but 4 Int` is copied from the shared module's `bookScope`; the exact bounds are assay's.
Everything that makes `line_4` elide is the spec's: Carl's claim that chmod reads its operand
under `Path`, Tessa's claim that a `slash_path` under `Path` yields an inode, the spec's
derivation of the resolution from those, Carl's verdict claim on the line, and the shared
module's definitions of `Converged` and `Elided`. The adversary may not flip the verdict claim,
because a claim in force is true; it may not revisit line 3, because that is a premise; it may
only hunt for a world the fixture admits in which the questions about line 4 do not all spare.
Hand the same lines a bare word (`#} chmod g-w {bare_word}`) and Tessa's second claim declines,
and the same site is forced to guard. Assay saw literals, two classes, two lines, a declaration,
and a set of claims. The worked version of this, with all its books, is `notes/30Ya-strawman-2/`.

## § 3-implementation-sketch

Light on purpose; the builder has latitude on everything not marked.

- **Runner.** A mise-managed JDK and the Alloy 6.2 distribution jar, pinned by version and
  digest into mise's own store, never vendored into the tree (SyncThing is live above the repo).
  The runner is one Java source file under `spike/verify/alloy/`, run from source by the pinned
  JDK as `mise run alloy`: it parses each root module through Alloy's own `CompUtil`, runs each
  command in a child JVM of its own under a wall-clock cap, and prints one JSON row per command
  in the lock's shape (§ 2.7); a module living outside the root's directory is served at the
  spot Alloy resolves `open` to, rather than copied. The jar's `exec` subcommand is not the
  route: it writes a directory per source file and offers no per-command cap. The JDK is named
  by its install path, since a machine-global one earlier on `PATH` wins over the pin. Windows
  and WSL alike. The runner is bounded on every axis a solver can exhaust [TYPED 2026-09-28: a
  command that never returns puts an LLM to sleep for hours]: a wall-clock cap per command, a
  CPU-time cap per command, a heap cap on the child JVM, a processor count the child may use,
  and a cap on the whole batch after which remaining commands are reported as not run; every
  cap is a flag with a default, and the defaults (120 seconds per command, 540 per batch) sit
  under the 600-second ceiling an agent harness puts on a foreground command, so a harness kill
  never orphans a solver; a shutdown hook destroys whichever child is live when the runner
  exits. The runner takes a directory as well as files, and skips a module whose only command is
  the `Default` Alloy synthesizes for a command-less module (recognisable by its unknown source
  position). The task rides `mise run preflight alloy` for disk and RAM, and
  runs under the repository's global heavy-work lock (`internal-tooling exclusive`), a file in
  the user's cache directory that names its holder, so a second heavy task on the same machine
  is refused with the holder's name and told to do other work rather than wait.
- **Compiler.** Rust, as a subcommand of `crates/internal-tooling` (which already reads the
  corpus for `slugs` and `docids`); a sibling crate under `spike/verify/` if it outgrows that. A
  Markdown fence lexer; the syntax crate's lexer for map lines; `#=` lifted verbatim, sorted into
  declaration, fact, or outcome by shape; the six modules of § 2.3; the lints of § 2.6. Assay
  never parses Alloy beyond recognising declaration heads and binders: a `sig`'s name, parent,
  and field names; a `fun`, `pred`, `check`, or `run` name and its scope clause; a
  quantifier's or `let`'s bound names. That is what classifying declarations and minting
  words (§ 2.3) need, and nothing else is read. Generated files carry a `--` comment per
  emitted item naming the source file and line it came from, so an Alloy message can be
  walked back to the spec [TYPED 2026-09-28: a line-mapping culture, kept from the start].
- **Lock and report.** JSON in, JSON out; `--check` writes nothing. Gate placement: lints in the
  pre-commit hk step, path-filtered to spec files; lock recomputation in `gate:full-quiet`,
  path-filtered the same way; larger scopes in an opt-in lane.
- **Fixtures** [TYPED 2026-09-28]: assay's own fixture is a meaningless, Dorc-agnostic document
  under `internal-tooling`'s tests, compiled byte-for-byte against committed expected modules,
  with one negative document for a lint refusal; no committed test runs a JVM. The two strawmen,
  `notes/30Ya-strawman-3` (the outcome algebra past a wall; every command finishes in seconds at
  the shared ceiling of twelve) and `notes/30Ya-strawman-2` (the identity model's first cut;
  its per-line checks reach millions of clauses at the same ceiling, through the transitive
  closures in its separation predicate), are frozen exploration artifacts, never fixtures: their
  content about Dorc is apparently normative and partly wrong. Strawman-3 was compiled and run
  once by hand as the compiler's smoke (`notes/30Yd`: every verdict as the hand build's);
  strawman-2 refuses under the identifier rule for map components and stays as it is. Neither is
  promoted to the spec tier [TYPED nack]; turning 311 into a specification is separate,
  clean-context, product-focused frontier work.

### § 3.1-later-and-maybe

Not in the first cut; listed so nothing here is mistaken for forgotten.

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
- Larger scopes nightly; unification with the catalogue lock; conditional loads (`if [ -f
  local.sh ]; then . ./local.sh; fi`) as two speech sets per later line.

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
