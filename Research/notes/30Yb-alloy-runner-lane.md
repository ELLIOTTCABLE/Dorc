# 30Yb — the Alloy runner lane, and strawman-2 run for real

> Tier: LLM-authored lane report (Opus builder, round 30), pre-step to `30Y`'s build. Windows leg
> only. Confidence marks: `+SURE` observed, `~SUSPECT` inferred, `-GUESS` unchecked.

## § 1-what-was-installed

- `mise.toml` pins `java = "temurin-21.0.12+8.0.LTS"` and `"http:alloy"` 6.2.0 (the newest
  release), the GitHub release asset `org.alloytools.alloy.dist.jar`, by `checksum =
  "sha256:6b8c1cb5…edb78d"`. Both land in mise's install store, outside the tree. A wrong digest
  refuses the install (`+SURE`, tested by flipping one hex digit).
- The digest is trust-on-first-use: the release publishes none, and the Maven Central jar of the
  same coordinate has a different SHA-1 (`f399…` vs `fa67…` here), so it is a different build
  (`~SUSPECT`: repackaged, not tampered; unverified).

## § 2-the-invocation

`mise run alloy -- [--timeout <s>] [--command <name>] [--instances] [--solver <id>]
[--open <module>=<file.als>]... <file.als>...`

The task runs `spike/verify/alloy/AlloyRunner.java` from source on the pinned JDK, jar on the
classpath; it parses each file with `CompUtil`, then runs each command in its own child JVM
(`TranslateAlloyToKodkod.execute_command`), killing it at the cap (default 300s). Output is a JSON
array, one row per command: `module`, `command`, `kind`, `scope` (Alloy's own reading of the
clause), `result` (`sat` · `unsat` · `counterexample` · `no-counterexample` · `timeout` ·
`error`), `wall_ms` (child process, ~0.6–1s is JVM start and parse), `solve_ms`, and `message` /
`instance` where present. A timeout's message says whether translation finished and the CNF size.
Exit 1 on any red, 2 on a usage error. Not wired into `hk.pkl` or any gate.

Why the API driver and not the CLI: the jar's `exec` subcommand exists and reports per command
(`receipt.json`, SAT iff a `solution` array is present), but it writes an output directory per
source, the Windows `java` launcher glob-expands a `-c '*'` argument, and a per-command wall-clock
cap needs a process the runner owns anyway. The driver is one file, compiled by the JDK's source
launcher, with no build step.

## § 3-the-module-resolution-rule

`+SURE`, observed: every `open X`, from any module in the graph, resolves to `X.als` under ONE
root: the root file's directory with the root's own `module a/b` path stripped (so
`books/two_paths_one_inode.als` declaring `module books/two_paths_one_inode` roots at
`build/two-paths-one-inode/`). Opened modules do not resolve relative to themselves. strawman-2's
`assay` and `shared` live elsewhere, so `--open X=<file>` serves a file's text at the spot Alloy
looks (the `CompUtil` loaded-files map), without copying; Alloy's messages then name the spot, not
the source file. Only the ROOT module's commands run: `bookScope` in `shared` runs only when
`shared` is itself given as a root.

## § 4-observed-results

Invocation, from `strawman-2/build/two-paths-one-inode/`:
`mise run alloy -- --open assay=../../harness/assay.als --open shared=../shared/species.als
../shared/species.als laws.als corpus.als books/*.als` (sat4j, 300s cap).

| module | command | kind | result | wall |
|---|---|---|---|---|
| shared | bookScope | run | sat | 1.4s |
| laws | neverWrongWhenAllTrue / `_premise` | check / run | no-cx / sat | 2.6s / 2.0s |
| laws | monotoneInSpeech / `_premise` | check / run | no-cx / sat | 38s / 1.9s |
| laws | strangerSafe / `_premise` | check / run | no-cx / sat | 40s / 1.9s |
| laws | attributionHonest / `_premise` | check / run | no-cx / **unsat** | 5.7s / 2.3s |
| laws | attributionSufficient / `_premise` | check / run | no-cx / **unsat** | 90s / 38s |
| laws | attributionMinimal / `_premise` | check / run | no-cx / **unsat** | 29s / 39s |
| corpus | seatCanKnow | check | **counterexample** | 2.2s |
| two_paths_one_inode | line_3 / line_4 / run | | no-cx / **timeout** / sat | 10s / 300s / 18s |
| siblings_across_filesystems | line_3 / line_4 / run | | no-cx / **timeout** / **timeout** | 11s / 300s / 300s |
| chmod_of_a_bare_word | line_1 / line_2 / run | | no-cx / **timeout** / **timeout** | 10s / 300s / 300s |

Five of the eight books in the spec have no module under `build/`; their `report.json` rows carry
`null`. `line_4` of `two_paths_one_inode` also timed out at 600s under glucose and minisat; its
translation takes ~9s and yields ~2.0M variables and ~7.1M clauses. The other timed-out book
commands had not finished translating at 300s.

## § 5-where-alloy-fought

1. **`before` is reserved** — mechanical. Wrote `sig Line { before: set Line, … }` (harness) and
   `.before` in shared and the books. Alloy: `Syntax error … There are 28 possible tokens that can
   appear here` at the `before` token. Renamed the field `above` everywhere (commit `980947db`).
   The prose in `spec/shared.md` still says `before`.
2. **`seq` bound under a 4-bit Int** — mechanical. `for 12 but 4 Int` alone is accepted and
   silently clamps `seq` to 7; spelling `8 seq` is refused (`With integer bitwidth of 4, you cannot
   have sequence length longer than 7`). Added `, 7 seq` to every `4 Int` clause (`de5a8f2c`).
3. **Ambiguous field over a union** — mechanical. `all d: GuaranteesUniqueName + Root |
   … d.on` and `all d: Resolution + Placement | … d.of` in `corpus.als`: `This name is ambiguous
   due to multiple matches: field … GuaranteesUniqueName <: on / field … Root <: on` (and the same
   for `of`). Split each into two quantifiers, same meaning (`b78a3814`).
4. **Module resolution** — mechanical. `laws.als` → `species` → `open shared`: `File cannot be
   found … two-paths-one-inode\shared.als`. Resolved by the runner's `--open` overlay (§ 3), no
   `.als` edit.
5. **The pinned JDK lost to a global one** — mechanical, runner-side. A bare `java` in the task
   body resolved to `C:\Program Files\Eclipse Adoptium\jdk-17…` ahead of the mise pin on PATH (seen
   in Alloy's native-library log). The task now names the pinned JDK by its install path
   (`d4b75348`).
6. **Book outcome checks and runs do not finish** — mechanical by class (a timeout), `~SUSPECT`
   its cause is the book scope: `exactly 18 Claim` plus `12` for every unowned kind puts `Key`,
   `MReferent`, `Query`, and friends at 12 each under relations such as `resolvedIn` and the
   `^parentIn` closures inside `separated`, quantified per key pair. Every `line_3` (a `Ran`
   outcome) finishes in ~10s; every line with a `Verdict` and a `Query` does not. Left red.
7. **`seatCanKnow` has a counterexample** — design (`~SUSPECT`). In the instance, `True` and
   `Computed` are empty and there are no `Line` atoms (`exactly 0 Line`), so by `shared`'s fact no
   claim is in force and none is `True`; `sortOwner`/`schemeOwner` read `PrimaryOf & True` and
   `Yields & True` and are therefore empty, and the first conjunct fails for Simon's
   `AliasesNothingElse`. The check reads ownership through truth-in-force in a world with no lines
   to put anything in force. Also: the spec's check quantifies `GuaranteesUniqueName +
   GuaranteesUniqueReferent + Root`; the hand-built `corpus.als` omits `GuaranteesUniqueReferent`.
   Left as built.
8. **Three law premises are unsat** — design (`~SUSPECT`). `attributionHonest_premise`,
   `attributionSufficient_premise`, `attributionMinimal_premise` have no instance at `6 but 8
   Shword, 3 Line, 1 Query`, so their checks pass without a witness. Probing in a scratch module
   (not committed): `answer = DISJOINT` with non-empty writes and reads is unsat at the default 6
   `Claim` atoms and sat at 7. `Claim` inherits the default 6 because the clause does not name it;
   a non-vacuous `DISJOINT` needs at least seven claims.
9. **The laws go red once the claim universe fits a DISJOINT** — design, not yet adjudicated.
   Same laws with `, 8 Claim` added (scratch, run killed partway): `neverWrongWhenAllTrue`
   no-counterexample (43s); `monotoneInSpeech`, `strangerSafe`, `attributionHonest`,
   `attributionSufficient` all **counterexample** (7s, 2.6s, 13s, 123s); every premise run then sat;
   `attributionMinimal` not reached. `strangerSafe`'s instance has skolems `S = {Root,
   Placement$0, Placement$1, Operand, GuaranteesUniqueName, ChecksRead}` and `d = MayWrite`
   (speaker `Speaker$0`, outside the five named speakers); the rest of the instance is recoverable
   by rerunning with `--instances --command strangerSafe`.

## § 6-concerns

- `+SURE` the committed law results (all green) are vacuous at the committed scope, per item 8;
  the report's `premise: unsat` column is the evidence, and the scope change of item 9 is a
  judgment that changes what the laws ask, so it is not committed.
- `~SUSPECT` a 300s default cap makes a full strawman run ~30 minutes, dominated by five book
  timeouts; the cap is per command, never per batch.
- `-GUESS` memory was not gated; no child exceeded the default JVM heap in these runs.

## § 7-strawman-3

`30Ya-strawman-3/build/outcomes-past-a-wall/`, one flat directory; `open` resolved with no
overlay, as § 3 predicts (`+SURE`). Invocation from that directory: `mise run alloy -- shared.als
laws.als corpus.als book_*.als` (sat4j, 300s cap; up to three commands ran concurrently on 32
logical cores, so wall-clock is slightly inflated). No command timed out, so no 900s retry and no
CNF size to record; the slowest command took 11.2s.

| module | reds (everything else green as expected) |
|---|---|
| shared | none (`bookScope` sat, 1.5s) |
| laws (10 checks, 10 premises, all 1.7–7.8s) | `attributionByRemovalHonest`, `…WithOneVoice`: counterexample (expected); `monotoneInSpeech`, `strangerSafe`, `survivalRestsOnFootprints`: **counterexample** (unexpected). Every premise sat. |
| corpus (4 checks, ~2.2s) | none |
| 9 books (57 checks, 9 runs, 3.8–11.2s each) | `line_7` of `book_stale_index_survives_on_footprints` and of `book_false_footprint_under_executes`: **counterexample**. Every run sat. |

Where Alloy fought:

1. **Ambiguous field over a union** — mechanical. `corpus.als` `describerSpeaksAlone` quantified
   `all d: Disturbs + Reads + Withholds | … d.verb`: `This name is ambiguous due to multiple
   matches: field … Disturbs <: verb / Reads <: verb / Withholds <: verb`. Split into three
   quantifiers, same meaning (`dcd3d037`). FINDINGS' `fight-field-names-clash-on-reverse-join`
   guarded reverse joins only; a forward join through a union hits the same ambiguity.
2. **`set Claim` at a `set MDecl` parameter** — no fight. A scratch module (not committed) ran
   `answer[l.speech, w, l]` and `by[Line]` (a set at an `l: Line` parameter): both typecheck and
   solve. `~SUSPECT` Alloy does not constrain an argument to its declared parameter bound, so
   `l.speech` would also carry any bare `Claim` atom; `said[l]` intersects with `MDecl` and
   therefore does something, and `by[Line]` binds the whole set as `l`, not a per-line union.
   Neither was compared for equivalence.
3. **`exactly 0 Line`** — no fight; `corpus.als` parses and runs with it.
4. **`monotoneInSpeech` and `strangerSafe` counterexamples** — design. Both instances have writer
   = reader = one line and a smaller speech set with no `Reads` (so `backing` is every key and the
   answer UNKNOWN) against a larger one containing a `Reads` whose `at` and `cells` are empty (so
   `backing` is empty and the answer vacuously DISJOINT). `monotoneInSpeech`: `S = {Withholds}`,
   `S2 = S + {Withholds, Reads}`. `strangerSafe`: `S = {}`, `d` = a tessa `Reads`. The law demands
   `answer[S] in answer[S2].*weaker`; `DISJOINT.*weaker = {DISJOINT}` under the shared `weaker`
   fact, so UNKNOWN for the smaller set fails it. `+SURE` of the instance; `~SUSPECT` the direction
   of `.*weaker` in the law reads opposite to the prose ("more speech never weakens an answer"):
   under the law as written, the smaller set answering DISJOINT where the larger answers UNKNOWN
   passes.
5. **`survivalRestsOnFootprints` counterexample** — design. The survived line's speech is a single
   false `Separate` whose `a` and `b` are the same key; with no `Disturbs` or `Reads` in force,
   footprint and backing are that one key, the self-pair is "separated", the flag is typed, and the
   line elides past a wall with no footprint in its speech. The check has no `allTrue` premise and
   nothing forbids `a = b`.
6. **`line_7` of the two stale-index books** — design, `fight-adversary-forces-key-existence`
   again. Both instances contain no `Key` whose word is `ufw_rules`, so ufw's backing
   (`cells = ufw_rules`) is empty, the hork wall's everything-footprint is vacuously separated from
   it, and the ufw line elides (and survives) instead of guarding. Neither book states `some
   w.ufw_rules`. `book_stale_index_flag_not_typed` does not fail the same way because its flag is
   untyped.
