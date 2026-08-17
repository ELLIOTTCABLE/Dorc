# 30C — The closure/custody work order (`28Q` §8 stage-ii, phase 3: closures and sourcing)

The executable spec for the `lane-custody-closures` phase-3 build, written against the code as it
stands at `73359e0f` so a cold successor executes from here and re-derives nothing. Phases 1–2
(the custody-composite repair, definition-grade row keying, the `Measurement` separation) are
LANDED and are this document's floor, not its subject.

Grades: `+SURE` measured in this tree · `~SUSPECT` read but unmeasured · `-GUESS` inference ·
`--WONDER` open question. Slugs are minted here and referenced elsewhere as `30C:<slug>`.

## §1 — The ruled contract (the ground; nothing below may contradict it)

Human-typed 2026-08-17, `307:§ack-implementation-open`, restated verbatim-in-substance:

- `rul-only-oracle-sourcing-mints-speakers` — the ONLY speaker-minting merge is an explicit
  **top-level `.`** in a **marked (dorc-lang) file**, of a file that is itself
  **dorc-lang/oracle-code with NO top-level commands**. Already law at
  `oracle/CLAUDE.md rul-speaker-minting-is-oracle-sourcing-only`.
- `rul-inertness-is-contract-never-engine-fact` — load-inertness is HYGIENE/CONTRACT. The license
  grounds on the MARKER plus the no-top-level-commands CONTRACT; refusals ATTRIBUTE to the
  contract; no diagnostic, doc-comment, narrative, or code comment anywhere may claim an engine
  PROOF of inertness. This is the modifier on every `28Q` §9 pin 1/2/9/10 ack.
- `rul-cli-coloading-composes-nothing` — naming several files on one command line is INGESTION.
  It composes no custody, no family, no speaker — permanently, until the human redesigns ingestion.
- `rul-book-sourcing-mints-no-speaker` — a book's `.` mints no speaker. Its only book-side value
  is UN-WALLING, and that lands ONLY together with `28K:res-book-ships-its-load-closure` (the
  sourced closure materializes at apply) plus the executing e2e cell.

Two rulings from `28Q` §9 pin 10, closed [TYPED 2026-08-17]:

- membership stays **granular to the needed licensure/state/behaviour** — no global
  propagate-everything rule is built; each consumer asks its own question with its own
  conservatism.
- diamond loading is **effectively allowed**, keyed to RESOLUTION-identity (bytes × what the
  file's own sourcing resolved to); differing bytes refuse.

## §2 — The custody model (the one design decision this order makes)

`dec-custody-is-containment-not-equivalence`. Custody is an ASYMMETRIC containment relation over
the include-tree, never an equivalence class over files.

    custody(F) = {F} ∪ transitive descendants of F along SPEAKER-MINTING edges

An edge `P → C` is speaker-minting iff: `P` is a marked file that is **not the book**; the `.`/
`source` sits at `P`'s TOP LEVEL; `P`'s target word is a source-literal path the driver resolved;
and `C` satisfies the contract (marked, no top-level commands).

The licensure question is exactly one, asked at exactly one seat:

    does the VOUCHER's custody reach the RESOLVED declaration's file?

Why containment and not a scalar unit-id compared with `==` (`28Q` §2, the `28R` review's finding):
a scalar merges siblings. Entry `E` sourcing strangers `A` and `B` would give `A` and `B` one
unit-id and dissolve the fence. Containment gives the ruled shape directly — an ANCESTOR edge takes
custody of what it sources, SIBLING/COUSIN edges fence (`28M` §10
`dir-ownership-is-transitive-inclusion`), and the two-strangers case leaves `A` and `B` mutually
fenced beneath `E`'s custody of the composition.

`fnd-book-edges-are-not-speaker-edges` [+SURE, from `rul-book-sourcing-mints-no-speaker`]: because
book `.`-edges are excluded from the relation, the two-strangers-under-a-BOOK case never even
reaches the fence — the book contributes no edges, so `A` and `B` are their own roots by
construction. The `E`-is-an-oracle case is the one the containment shape is load-bearing for.

`dec-definitioncustody-carries-the-relation-not-a-root`. `core::DefinitionCustody` today is a
STAMP: measured, nothing in the workspace compares two of them for a decision (`+SURE` — the only
readers are `plan/src/lib.rs:542` wrapping it into `LicenseCustody::Vouched`, and the mint sites at
`1747`/`1875`; the sole DECISION seat is `HelperIndex::resolve`'s `chosen.file != asker`). So the
re-key keeps the newtype's *shape* and moves the DECISION behind a core-sited containment
predicate. `28M` bitem3's seam promise ("internals change, consumers still only compare") holds in
substance with zero consumer churn; the deviation from its letter is that the compare is
asymmetric (`reaches`), which containment forces. Recorded as a deviation in §8.

## §3 — Enumerated seats (file:line at `73359e0f`)

**Load/dialect surface**
- `spike/crates/oracle/src/load_inert.rs:59` `item_is_load_inert` — admits `FuncDef` and wordless
  `Simple` only; `. ./other.sh` is refused (pinned negatively at `:204`). THE amendment seat.
- `spike/crates/oracle/src/load_inert.rs:31` `lint_load_inert` — the marked-file refusal; its
  diagnostic is `DiagCode::OracleFileNotLoadInert`. Refusal attribution re-words to the CONTRACT
  (`rul-inertness-is-contract-never-engine-fact`); prose is `[unwritten:]`-tier and
  conductor/human-owned (`error-authorship-tier`) — builders mint structure, never words.

**Closure/custody**
- `spike/crates/oracle/src/closure.rs:223` `HelperIndex::build(srcs, book)` — indexes a source only
  when its whole top level passes `item_is_load_inert`; `book` is threaded explicitly.
- `spike/crates/oracle/src/closure.rs:403` `HelperIndex::resolve` — `chosen.file != asker` at
  `:426`. THE custody predicate.
- `spike/crates/oracle/src/closure.rs:335` `closure_for(file, body)` — the walk; `contributing`
  drives per-file constant capture.
- `spike/crates/oracle/src/closure.rs:505` `agree` — the load-edge collision report; differing
  bytes refuse. The diamond rule re-keys its identity to bytes × resolved sourcing.
- `spike/crates/core/src/lib.rs:127` `DefinitionCustody` + `:129` `of_defining_file`.
- `spike/crates/core/src/definition.rs:86` `DefinitionId::custody` — the ONE crossing.
- `spike/crates/analysis/src/funcenv.rs:599` `custody_of_source_index` — the index→custody crossing.

**The 15 `HelperIndex` construction/consumption sites** (every one must receive the closure data or
provably not need it):
- `cli/src/main.rs:735` (build, book = last), `:2596`, `:2633`, `:2731`, `:4823` (build, `None`)
- `cli/src/survival.rs:391`, `:1142`, `:1356`, `:1404`, `:1921`
- `cli/src/world.rs:170` (build), `:880`, `:921`
- `plan/src/lib.rs:1694`, `:5769`, `:5821`
- `coverage/src/lib.rs:561`, `sweep/src/drive.rs:165`

**Sourcing resolution (existing, book-side)**
- `analysis/src/funcenv.rs:1195` `load_sites` — splits `.` sites into resolvable/unresolvable
  against `DefinitionTable::definitions_of_path`; target must be a `SourceLiteralPlane` literal.
- `analysis/src/funcenv.rs:1143` `sourced_definitions`; `:164` `set_loadable` keys by the CLI path
  STRING exactly.
- `cli/src/world.rs:665` `definition_table` — the ONE reader of role funcdefs; `set_loadable(path)`
  per CLI oracle path.
- `cli/src/main.rs:2232` `source_table` — oracles in CLI order, then the book LAST.
- `cli/src/main.rs:335` `resolve_oracle_paths` — CLI positional + `--oracle-dir` glob. THE seat a
  sourced-file read must extend (I/O at the edge only, `io-at-edges-only`).

`fnd-sourced-files-are-not-read-today` [+SURE]: nothing reads a file named by a `.` that was not
also named on the CLI. `definitions_of_path` matches the literal target against CLI path strings,
so today's `. ./x.oracle.sh` only resolves when `x.oracle.sh` was co-loaded. Phase 3's oracle-side
sourcing therefore needs a real CLI-edge LOADER (read, contract-check, append to the source vector,
record the edge), not merely a predicate change.

## §4 — Build items, in landing order (each independently green + committed)

`ord-fixtures-precede-consumers` — floor-differential cells land BEFORE the engine behaviour that
consumes them (`28Q` §8 stage-i's measure-first precedent); ordinary golden cells for new behaviour
land AFTER it (the human's lean: pin the future, never pin the hole).

1. **`b1-floor-fixtures-for-sourcing`** — the ground-truth manifests, `floor30-*` pattern
   (`spike/crates/cli/tests/floor30-helper-collision-across-frames.loom` is the template: a
   sentinel-manifest book, `expected.emitted` = the shells' own bytes, minted ONCE via
   `mise run bless:floor -- <case>` on the WSL leg, never churned). Cells:
   - `floor30-oracle-sources-its-helpers` — a marked entrypoints file `.`-ing a helpers file;
     the role body reaches a helper declared there and the base file's own declarations still
     contribute (the two conjuncts `p-x-blessed-toplevel-source` asserts).
   - `floor30-sibling-subtrees-stay-separate` — one entry sourcing two files; each sourced file's
     own names, and the fact that neither sees the other's.
   - `floor30-diamond-resolves-once` — two entries sourcing one identical helpers file; sh binds
     one body and re-sourcing is idempotent.
   Gate: measured once, byte-stable on re-mint (`emitted-is-measure-once-ground-truth`).

2. **`b2-load-inert-admits-top-level-dot`** — `item_is_load_inert` admits a top-level `.`/`source`
   whose target word is statically resolvable; refusal for a contract-violating target attributes
   to the CONTRACT. Greening trigger for `p-x-blessed-toplevel-source`. Both conjuncts of that pin
   must green together — the whole-file refusal is what costs the file its own declarations.

3. **`b3-the-sourced-file-loader`** — the CLI-edge read: from each marked oracle's top-level `.`
   sites, resolve the target relative to the sourcing file's directory, read it, contract-check it
   (marked + no top-level commands), append it to the source vector, and record the edge. Failure
   to read or to satisfy the contract REFUSES with contract attribution and the closure does not
   form (the site's licensure falls back to today's answer — the safe direction).
   `--WONDER`: whether a sourced file also joins the `--oracle-dir` ambient prefix or only the
   closure. Lean: it joins the SOURCE VECTOR (so its definitions are visible exactly as sh would
   bind them after the `.`), because `rul-unsure-falls-toward-sh-parity`.

4. **`b4-custody-closure-computation`** — the include-tree → `custody(F)` relation, built at ONE
   seat from the recorded edges, threaded to `HelperIndex`. Diamond dedup keys to
   RESOLUTION-identity (bytes × what the file's own sourcing resolved to); differing bytes refuse
   through the existing `agree` path.

5. **`b5-rekey-the-custody-predicate`** — `HelperIndex::resolve`'s `chosen.file != asker` becomes
   `!custody(asker).contains(chosen.file)`; `DefinitionCustody` internals follow per §2.
   Single-closure world (today's whole corpus) stays byte-identical: with no oracle-side `.`,
   `custody(F) = {F}` and the predicate is the phase-1 one, term for term.

6. **`b6-blessing-keying-family-rooted`** — `pin-blessing-keying`'s acked lean:
   family-rooted-within-the-closure (reachable from THIS family's predict members), not
   closure-global. Bites only where one closure hosts families of divergent care.

7. **`b7-repin-the-discarded-package`** — `pin28-helper-package-entrypoints-discarded.loom` re-pins
   via a thin entrypoints ORACLE that SOURCES the helpers (the spelling the ruling leaves); the
   in-book entrypoint form stays as an honest DECLINE cell. The in-book spelling is PERMANENTLY
   de-licensed — a book mints no speaker even with `.`.

8. **`b8-book-side-unwalling`** — ONLY as a unit with `28K:res-book-ships-its-load-closure` (closure
   materialization at apply) AND the executing e2e cell: the original book under its sourced tree ·
   the artifact run from an isolated cwd · a missing sourced file failing honestly. If the runtime
   half cannot land, b8 does not land — a book's `.` keeps walling and
   `FORFEITS:forfeit-book-sourcing-walls` stands unrewritten. Landing it rewrites that row
   (rewrite-don't-annotate).

9. **`b9-xfail-sweep`** — after EACH landing, re-check `p-x-blessed-toplevel-source`,
   `p-x-definition-grade-keying`, `p-x-helper-unset-f-across-files`, `p-x-regional-helper`
   (`internal-tooling/src/xfail.rs` `PINS`; the last three carry horizon `r31:closure-custody`).
   A pin that greens is promoted and its registry row removed; a pin that does not is
   RE-HORIZONED with `Horizon::Deferred{was, now, why}` — never weakened, never deleted.
   `mise run xfail:census` renders what is owed.

## §5 — Gates

- `syn-single-frame-byte-identical` / single-CLOSURE byte-identity: today's whole corpus produces
  byte-identical output except cells named in the drift enumeration (§6). This is the migration
  gate on EVERY commit.
- `empty-world-byte-identical`: no oracles loaded ⇒ byte-identical. Unconditional.
- New floor cells: measured once, byte-stable on re-mint; disagreement BETWEEN binaries is the
  floor's verdict and is never blessed away.
- Checker gates (`notes/300` §2): certifier + sparing re-derivation green over the full corpus.
- `mise run gate:quick-quiet` in the hot loop; `mise run both gate:full-quiet` + `mise run
  bless:dry` FOREGROUND at close. Windows leg FIRST (`preflight-bounds-before-spend`: the WSL
  build cache inflates `vmmemWSL` and the Windows RAM probe reads it as pressure).
- `mise run bless:floor` is WSL/*nix only (git's Windows userland has no `posh`) and rides the
  trial filter — scope it to the named case, always.

## §6 — Expected golden drift (the enumeration; anything outside it is a STOP)

- `pin28-helper-package-entrypoints-discarded.loom` — re-pinned to the sourcing spelling (b7).
- `pin28-helper-package-entrypoints-lift.loom` — the co-loaded package re-spells as a sourcing
  package; the lift returns through custody rather than co-loading.
- The new `floor30-*` cells (b1) and the new decline/e2e cells (b7/b8) — additions, not drift.
- `crates/aid/src/catalog_lock.rs` — only if a code is minted; through the promote ceremony
  (`two-bless-paths-split-by-directory`: promote FIRST, rebuild, THEN the e2e bless).

## §7 — Fences (hands off; a Spine lane runs concurrently)

`plan::records`/whylog/spine surfaces · `core::influence` · `core::spine` · `crates/aid` catalog and
arrangement beyond decline-reason needs. Out of scope with no rulings: the committee fence's
permanence (build-as-spiked stands, marked unratified) · verdict-word enrollment · any
multihost/book-to-host mapping · `LIVING_STATUS.md` (conductor's fold).

Verified-core proximity: this lane does NOT touch `core::sorted`, the lattice/solver, the
certifier, the sparing reference model, or `minispec/`. If a signature change ripples into
`spike/verify/`, that is the alarm working — report, never widen a fence's allow-list.

## §8 — Deviations recorded at authoring time

- `dev-docid-is-30c-not-30f` — the brief expected `30F`; the `Research/**/30*` glob shows `30A`
  and `30B` are the highest taken, so the lowest free letter is `30C`.
- `dev-prelude-a-was-already-landed` — `28Q` §1 item 1's file-grade tripwire retired at
  `c34e996a` (predecessor's last doc commit). Residual: the item still explains itself by contrast
  with the retired shape, which brushes plans-are-ahistorical; trimmed to plain current truth.
- `dev-custody-compare-is-asymmetric` — §2's containment shape, against `28M` bitem3's "consumers
  still only compare" letter. Zero consumer churn, so the promise holds in substance.
