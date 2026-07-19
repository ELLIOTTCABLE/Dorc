# 28B — syntax-respell proposal (phase-A map; the conductor rules, phase-B executes)

AI-authored (Opus builder, `lane-syntax-unification-respell` phase A, 2026-07-19). The
grep-grounded implementation plan for landing `plans/281` Part I (the annotation
mark-grammar) plus the corpus respell, per `plans/280` §2 and `notes/28A` §2. This note is
a PROPOSAL: no engine edits were made this dispatch. Authority: root docs,
`spike/CLAUDE.md`, human-typed rulings, `plans/281` outrank. Companions carrying member
semantics I cross-checked: `273` (lend_map), `272` (store/topology), `plans/27C` (context
entry / `tolerates:`→`safe-across`), `277` §3 (dialect minting).

Confidence: `+SURE` where I read the code; `~SUSPECT`/`-GUESS` where I inferred design
intent or sizing. All line-anchors are HEAD `0825f6e` (`spike/crates/...`).

---

## §1 — Surface map (line-anchored)

The grammar change is NOT localized to the parser: marks are lexed inside oracle bodies,
carried in a shared AST, decoded by six role-keyed consumers, rendered into goldens, and
stripped. Every touch-point below is `+SURE` (read directly).

### §1a — Lex + parse (the mark grammar core)

- **`oracle/src/predict/lexer.rs:110,176-182`** — `at_comment_start` / `skip_comment`.
  Comments are SKIPPED before the parser sees them. The `#:` carrier (`281` §1) is
  therefore INVISIBLE today: the lexer must peek after a comment-start `#` and, iff the
  next byte is `:` (immediate, no space), NOT skip — emit the block as a mark intro. The
  version marker `# dorc-lang/v0.1` (space after `#`, `oracle/src/marker.rs:25`) stays a
  comment — the no-space rule is the whole disambiguator (`281` §1, §R5).
- **`oracle/src/predict/parser.rs`** — the recursive-descent dialect parser:
  - `mark_marker` (1296-1303): recognizes `:` `:!` `:?` → must add `:=` and the five `#:`
    intros (`#:` `#:!` `#:?` `#:=`), per `281` §3.
  - `MarkSigil` / `classify_mark` (1284-1291, 1397-1403): the three-value sigil family →
    grows to carry the head-sugar decode (`!`/`?`/`=`) and the carrier (`:` vs `#:`).
  - `parse_command` (731-871): the leading-`:` handling (789-799) treats a statement-start
    `:` as the sh no-op COMMAND word, expecting a SECOND `:` marker for the trailing mark
    (the old `: : token` shape). The new single-`:` standalone intro (`281` §11) breaks
    this — see §2, `flag-standalone-single-colon-restructure`.
  - `parse_mark` (878-923): consumes marker + one target word + optional `= value`. Must
    become verb-driven (three head rules + verb-loop, `281` §4).
  - `split_mark_target` (1320-1365): splits `kind:entity#selector` on the FIRST `#` →
    switch the selector introducer to `@` (`281` §R4). The `kind:entity` split (first `:`)
    is UNTOUCHED.
  - `is_valid_selector` / `is_brace_alternation` / `brace_tokens` (1374-1392): selector
    charset + `#{a,b}` brace shape → `@{a,b}`, and brace generalizes to a standalone
    payload word (`281` §6).
  - `parse_word_led` (604-656) + `kind_after_colon_is_bare` (669-674): the inline-bind vs
    trailing-mark disambiguation (`name : bare-kind [= value]` = bind; `name :
    kind:entity...` = trailing mark on command `name`). Load-bearing; preserve — see §6
    `flag-inline-bind-vs-head-decode`.
  - `FnRole` (29-68): role menu unchanged; the per-role mark CONSUMERS change.
- **`oracle/src/predict/ast.rs`** — `MarkKind` (113-128) is `{Establish, EstablishInverted,
  Observe}`; `MarkTarget` (133-146) is `{kind, entity, prop(=selector), value}`. Today ALL
  meta-marks (`disturbs`/`lends`/`stored-in`/`invariant`/`tolerates`) ride `Establish` with
  the token in `kind` and a sub-token in `entity`. The new verb plane needs a verb
  discriminant + per-verb payload typing — the central AST reshape (§2).

### §1b — Strip (`oracle/src/strip.rs`)

- `collect_file_strip_edits` (204-257): erases binds → `name=value`, trailing marks →
  gone, bare-colon-host lines → whole-line delete.
- `is_bare_colon_host` (261-263): detects `words == [":"]` (the OLD `: : token` shape). The
  new single-`:` standalone intro means the `:` is the intro, not a separate colon-command
  word — this predicate + the whole-line-delete path must be re-derived (§2, §5).
- `#:` carrier strip is ENTIRELY NEW (`281` §9): delete the comment IFF it parses as a
  valid mark-block, else leave + diagnose. `marker_line_edit` (359-368) already deletes the
  `# dorc-lang/v0.1` line whole; the `#:` path is separate.

### §1c — Meta-mark consumers (read `mark.target.kind`/`.entity` today)

Each decodes the token STRING from a `MarkKind::Establish` mark; each must read the new
verb + typed payload:

- **`oracle/src/carry.rs:51,169-200`** — `INVARIANT_TOKEN="invariant"` matched against
  `mark.target.kind`; axis in `.entity`; `NET_KERNEL_SUBSTRATE="net-kernel"` matched too.
  Respell: `invariant:<axis>` → `undivided-by-transit-across <axis>`; substrate `: net-kernel`
  → `: stored-in net-kernel`. Cross-checked against `272` §2 (`state_stored_only_in`),
  `277` §4e, `27C` §4(a): the member semantics (per-netns caveat, whole-member invariance)
  are UNCHANGED — only the spelling the scanner matches moves.
- **`oracle/src/entry.rs:220,314-334`** — `TOLERATES_TOKEN="tolerates"`; dims in `.entity`;
  `expand_dimension_set` splits the attached `{user,fs-view}`. Respell: `tolerates:<dim>` →
  `safe-across <dim>`; the brace becomes a SPACE-separated payload word `safe-across
  {user,fs-view}` (`281` §6). Cross-checked `27C` §2: the vouch semantics (per-function,
  per-dimension, reachability-scoped, `case`-arm scoping) are UNCHANGED.
- **`oracle/src/wrapper.rs:44-75,748`** — `Dimension::{User,FsView,Netns}` +
  `from_token`; `lend_map` reads `Dimension::from_token(&mark.target.kind)`. Respell: `: user`
  / `: fs-view` (mapped/full lend) → `: lends user` / `: lends fs-view`. Cross-checked
  `273` §3: empty-vs-mapped-lend + enumerate-every-dimension law UNCHANGED.
- **`oracle/src/touches.rs:340-373,448`** — disturbs emission: `mark.target.kind` = the
  kind, `.prop` = selector (brace-expanded, 348-354). Respell: `: <kind>` → `: disturbs
  <kind>`; `#sel` → `@sel`. `parse_coordinate` (448) is the LEGACY stringly `kind:entity`
  form (dynamic host-readback, `:`-split, no `#`) — UNAFFECTED by the `@` switch.
- **`oracle/src/reaches.rs:224`** — `kind__disturbance_reaches_only` emission, reads
  `mark.target.kind`. Respell: `: <kind>` → `: disturbs <kind>` (UNIFIED verb; `281` §5,
  `disturbs` wires both members). Cross-checked `272` §10 + `277`: the two members keep
  distinct NAMES (first-vs-second-order); only the mark VERB unifies.
- **`oracle/src/predict/derive.rs:160-186`** — verdict/observe → `ValueClaim`; reads
  `MarkKind` + `.prop`; rejects `@{a,b}` on verdict/observe via `MarkBraceVerdictSingleCell`
  (184-186). The `Reads` off-head verb (`281` §5) must map to `ValueClaim::Observe`
  alongside `:?`.

### §1d — Render + intern surfaces (goldens)

- **`plan/src/lib.rs:4235-4240`** — the fact-label render: `format!("{kind}:{}#{selector}")`
  and `format!("{kind}#{selector}")`. THE selector-`#`-render; switching to `@` re-blesses
  every `# site N: <label>` golden line (`plan/src/render.rs:128-136,546`).
- **`cli/src/main.rs:2059-2110`** (`intern_coordinate`) + `2038` — interns the opaque
  `kind:entity#selector` fragments; the `#`-split moves to `@`.
- **`cli/src/main.rs:4105`** (`render_coord`) renders `{kind}:{entity}` (no selector) —
  no change needed.
- **`hostsim/src/differential.rs:157`** — `Polarity::Query => ":?"` (the observe sugar,
  survives unchanged).

### §1e — Diagnostics machinery

- **`core/src/diag.rs`** (`DiagCode` enum + `slug` + `registry` + `params_of`) and
  **`core/src/catalog.rs`** (const `CATALOG`): new codes = 1 variant + 1 slug arm + 1
  severity arm + 1 params arm + 1 catalog entry (`22B` §7 friction).
- **`core/tests/diag_tidy.rs`** — `MIGRATED_PAYLOADS` (49-116) + `MIGRATED_SLUGS`
  (121-174): a new code adds one entry to EACH, plus a literal `DiagCode::X(` production
  emit (the `every_catalog_variant_is_constructed` grep, 344-360).
- **`core/tests/catalog_defining_cases.rs`** — `covered()` / `DEFINING_CASE_RATCHET` /
  `unwritten_renders_are_greppable_and_pinned`. THE ratchet conflict lives here (§3).

### §1f — Corpus (respelled text)

214 `.sh` under `e2e/cases/` (books + oracles) + 94 golden sets; crate test string
literals (§4 table); the 7 crate `CLAUDE.md` authored-surface blocks. EXCLUDED this lane
(serial docs pass owns): `spike/docs/`, `spike/skills/`, root docs, `Research/`.

---

## §2 — Grammar implementation plan (`281` Part I into the engine)

### §2a — The intro + head-decode

Intro `:= ( ':' | '#:' ) [ '!' | '?' | '=' ]` then required WS (`281` §3). Legal intros:
`:` `:!` `:?` `:=` `#:` `#:!` `#:?` `#:=`. After `intro WS`, decode the FIRST mark by three
rules (`281` §4), in order:

1. sugar present → payload is the sugar's type, verb is the sugar's verb (`!`=`refutes`,
   `?`=`reads`, `=`=`bind`, omit=`asserts`).
2. no sugar, first token has a period → coordinate; verb = `asserts`.
3. no sugar, first token has NO period → it is a verb word; parse it + its payload.

Keystone (`281` §4 `rul-verbs-dotless-kinds-dotted`): verbs are period-free; a kind has ≥2
periods. Second-slot payloads (axis/dimension/substrate tokens, quoted entities, paths) may
carry periods freely — reached verb-driven, never period-tested. Every subsequent mark on
the block is verb-driven (read a verb, consume the payload its arity+type fix, repeat).

`~SUSPECT` the cleanest AST is `MarkKind` grown to the full verb set — `Asserts`, `Refutes`,
`Reads`, `Bind`, `SafeAcross`, `Disturbs`, `Lends`, `StoredIn`, `UndividedByTransitAcross`
— with `MarkTarget` generalized to a payload enum keyed by verb (coordinate for
asserts/refutes/reads; kind[+@selector] for disturbs; kind for bind; dimension-token for
safe-across/lends; substrate-token for stored-in; axis-token for undivided). This keeps the
consumers reading a TYPED payload rather than re-decoding `kind`/`entity` strings — a
strict-typing win (priority-1/3) over a stringly `verb: String` field. The rename
`prop`→`selector` (`ast.rs:143-145` already flags the debt) rides here.

### §2b — The verb vocabulary table (`281` §5) + member wiring (cross-checked)

| verb | rc | sugar | payload | wires to | member semantics source |
|---|---|---|---|---|---|
| `asserts` | yes | omit | coordinate | verdict `MarkKind::Establish` | `derive.rs`; `277` §3 mints |
| `refutes` | yes | `!` | coordinate | `MarkKind::EstablishInverted` | `derive.rs` |
| `reads` | no | `?` | coordinate | `MarkKind::Observe` (backing-widen) | `derive.rs`; `277` §5 |
| `bind` | no | `=` | kind | `Stmt::Annotation` (value-plane) | `ast.rs` Annotation; `281` §8 |
| `safe-across` | no | — | dimension token | `entry.rs` ToleranceVouch | `27C` §2 |
| `disturbs` | no | — | kind[+@selector] | `touches.rs` AND `reaches.rs` | `272`/`277` §4c |
| `lends` | no | — | dimension token | `wrapper.rs` lend_map | `273` §3 |
| `stored-in` | no | — | substrate token | `carry.rs` scan (net-kernel) | `272` §2 |
| `undivided-by-transit-across` | no | — | axis token | `carry.rs` InvarianceIndex | `277` §4e / `27C` §4(a) |

Two structural rulings I verified against the members (`281` §5, orthogonality test):
`disturbs` is ONE verb for two orders — `cmd__disturbs()` first-order footprint and
`kind__disturbance_reaches_only()` transitive reach — the RECEIVER (`touches.rs` vs
`reaches.rs`) fixes order, not a meaning-flip; both read `mark.target.kind` today, so
unification is a one-word add + a shared verb. `safe-across` and
`undivided-by-transit-across` stay distinct verbs (`entry.rs` vs `carry.rs`): different
licenses (enter-and-execute vs travel-without-entry), the knife-tier invariance verb spelled
unmistakably long (`281` §R6).

### §2c — Selectors, `@`, brace-alternation, continuations, rc-arity

- **`@` selector** (`281` §6, §R4): `coordinate := KIND [':' ENTITY] ['@' SELECTOR]`.
  `split_mark_target` switches the selector split from `#` to `@`; entity-less transitional
  `KIND:@SELECTOR` (né `kind:#sel`, `ast.rs:139-140`). `@` needs no comment-avoidance rule
  (`#` did).
- **brace-alternation** as a general payload combinator (`281` §6): `@{enabled,active}`
  (attached to `@`, the old `#{a,b}` shape) and `safe-across {user,fs-view}` /
  `lends {user,fs-view}` (a SPACE-separated brace payload word — NEW shape; today
  `tolerates:{...}` is attached). Refused for `asserts`/`refutes` payloads by rc-arity (the
  existing `MarkBraceVerdictSingleCell`, `derive.rs:184`). `brace_tokens` generalizes to
  serve both attached (`@{`) and standalone (`{`) forms.
- **continuation lines** (`28A:rul-continuation-attachment`): a mark-only physical line
  accrues to the preceding statement's block IFF the preceding line ended with a
  mark-block; else it stands alone (position-scoped). NEW machinery — today each mark is
  trailing on ONE statement, no cross-line accrual. The parser must track "did the previous
  statement end with a mark-block" and re-intro each continuation `:`/`#:`. `281` §11's
  kind-owner example reads as a continuation but is semantics-neutral for member-collected
  verbs (`stored-in`'s invariance sibling), per `28A` §2.
- **rc-arity** (`281` §7): at most ONE rc-consumer (`asserts`/`refutes`) per BLOCK
  including continuations. A standalone block containing an rc-consumer or `reads` = loud
  diagnostic ⇒ that block drops to ⊤ (no statement to measure/back). Enforced over the
  whole block, not per physical line.

### §2d — Bind, both forms

- **inline** `name : KIND = "$value"` (`ast.rs` Annotation; parser 604-651, 680-726): the
  as-built parse EXISTS and works (dispatched by `parse_word_led` on `name : bare-kind`).
  `281` §8 confirms the inline bind is disambiguated by the `= value` tail and a bare kind.
  Preserve; it is NOT a trailing mark. The value-less nullary/Singleton form
  (`index : pkgindex`) is spike-debt but corpus-live (pkgindex) — keep.
- **trailing** `FOO="bar" := KIND` (sugar) / `... : bind KIND` (word): NEW. Rides the
  assignment, entity = the assigned value via value-flow (`281` §8). Strip reduces every
  form to plain assignment. `#:=` is the safer default for binds (a colon-form `:=` on an
  assignment reads as an unknown command under a raw run, `281` §8).

### §2e — Strip (`281` §9)

- colon-form trailing → erase intro-to-end-of-block (as-built for single-line;
  continuation extends the region).
- colon-form standalone → delete the line. The single-`:` intro means the `:` IS the
  standalone statement — `is_bare_colon_host` must detect the new shape (an intro-`:`
  statement with a mark and no preceding command), not `words==[":"]`.
- `#:` → delete IFF it parses as a valid mark-block; else leave as a plain comment +
  diagnose (`mark-hashcolon-malformed`, §3). NEW path.
- marks erase to NOTHING (as-built: `strip.rs` deletes the region, never leaves a `:`).
  This is already correct and CHANGES the old trailing-`:` rc-clobber story exactly as
  `281` §9 wants — the single-`:` standalone intro strips whole-line, so no null-command
  residue. The marker `# dorc-lang/v0.1` (space) must never be confusable with `#:`.

---

## §3 — Diagnostic mint list (EMPTY prose, `27V:rul-error-authorship-tier`)

New `DiagCode`s the grammar needs (severity `Error` unless noted; injection-surface noted
for the ratchet/defining-case). Prose blocks author as `[unwritten: <slug>]`.

1. **`mark-unknown-verb`** — a period-free head/continuation token that is not a known
   verb (rule-3 miss). Trigger: an oracle mark `... : frobnicate sm.dorc.X`. Injection:
   unit lift of a body with a bogus verb word. Severity Error (loud ⊤-reject,
   `inv-top-reject`). Payload: the token + the verb vocabulary (templatized, like
   `ToleratesUnknownDimension`).
2. **`mark-rc-arity-exceeded`** — two rc-consumers (`asserts`/`refutes`) in one block
   (incl. continuations, `281` §7). Trigger: `cmd : sm.a.B@x refutes sm.a.B@y`. Injection:
   unit lift. Severity Error ⇒ block ⊤.
3. **`mark-standalone-rc-consumer`** — a standalone block carrying an rc-consumer or
   `reads` (no statement to back, `28A:rul-continuation-attachment`). Trigger: a bare
   `: sm.a.B@x` line with no preceding command and no continuation. Injection: unit lift.
   Severity Error ⇒ block ⊤.
4. **`mark-hashcolon-malformed`** — a `#:` comment that looks like a mark but does not
   parse; left as a comment, diagnosed (`281` §9 graceful degradation). Trigger:
   `#: frobnicate`. Injection: strip/lift a malformed `#:` line. Severity Warning
   (`~SUSPECT`; the comment carrier never silently mis-erases, but it is not a hard ⊤).

`~SUSPECT` the existing `PredictOutOfDialect` (the `fail_here`→`lift_failure` sink,
`predict.rs:317-327`) can ABSORB the selector-charset and malformed-target failures it
already carries (parser 897-912), so I do NOT propose a distinct `mark-selector-invalid`
unless the conductor wants finer attribution. `MarkBraceVerdictSingleCell`
(`derive.rs:184`) already covers brace-on-verdict; REUSE, do not re-mint.

### FLAG — `flag-ratchet-forbids-additions` (`28A:rul-ratchet-accepts-new-codes` vs as-built)

`28A` §2 says a newly-minted code "legitimately enters the ratchet with a per-entry
injection-surface note" and, "if the gate literally forbids additions, FLAG UP." The
as-built gate LITERALLY FORBIDS it: `catalog_defining_cases.rs:474-506`
(`ratchet_only_shrinks`) asserts `DEFINING_CASE_RATCHET.len() <= baseline` against
`git show HEAD` — any ratchet GROWTH fails at the commit that adds it. So each new code
CANNOT hide in the ratchet; it MUST ship a `covered()` defining case (3 golden files:
`.machine`/`.terse`/`.prose`), where `.prose` renders the `[unwritten: <slug>]` placeholder
(`aid-unloaded-sibling-oracle` is the precedent, `catalog_defining_cases.rs:203-211`).

A SECOND gate compounds it: `unwritten_renders_are_greppable_and_pinned` (530-557) asserts
`unwritten.len() <= 1`. Minting N new codes with empty prose EXCEEDS the ceiling of 1 — the
builder must bump it to `1 + N` (a "conscious conductor act" per the test's own comment).

**Recommendation** (conductor rules): new codes ship as `covered()` cases with
`[unwritten:]` prose + bump the unwritten ceiling; the ratchet is NOT grown. This
contradicts `28A:rul-ratchet-accepts-new-codes` as written — I flag, I do not silently
deviate. `MIGRATED_PAYLOADS`/`MIGRATED_SLUGS` (`diag_tidy.rs`) also grow one entry per code
(mechanical).

---

## §4 — Respell sweep inventory (this tree; approximate hit-counts + file lists)

Counts are `~SUSPECT` (regex over mixed comment/label/mark text; order-of-magnitude
reliable, exact totals to be recomputed mechanically in phase B). Scope: `spike/` code,
fixtures, unit-test literals, `e2e/cases/`, and the crate `CLAUDE.md` authored-surface
blocks. Note `:?`/`:!` sugars survive unchanged (only `#`→`@` inside their targets); the
churn is selectors, token-lines, verb-words, and the two renamed vouch/invariance spellings.

| `281` grep-map row (+ implied) | current corpus spelling | new spelling | ~hits / where |
|---|---|---|---|
| `#` selector → `@` | `...Service:"$svc"#enabled` | `...@enabled` | ~292 e2e (marks + rendered labels); ~162 crate `.rs`; 5 crate CLAUDE.md |
| `: :` double-colon line → single `:` | `:  : invariant:user` | `: undivided-...` | ~33 `.sh`/`.out` lines (mostly the invariance/full-lend lines) |
| positive verdict (head) | `... : sm.a.B@x` | UNCHANGED (head `:` = asserts) | head marks stay `:` |
| `:!` complement | `:! sm.a.B@x` | UNCHANGED sugar (only `#`→`@`) | ~66 e2e `.sh` |
| `:?` observe | `:? sm.a.B@x` | UNCHANGED sugar (only `#`→`@`) | ~17 e2e `.sh` |
| bare-token lend emissions | `printf ... : user` / `: fs-view` | `: lends user` / `: lends fs-view` | ~31 `.sh` (wrapper corpus) |
| disturbs kind claim lines | `printf ... : sm.dorc.Package` | `: disturbs sm.dorc.Package` | ~17 `.sh` (disturbs/reaches bodies) |
| `reaches` verb (unify) | `printf ... : sm.dorc.Service` | `: disturbs sm.dorc.Service` | (subset of the 17) |
| `tolerates:` → `safe-across` | `:  : tolerates:user` | `: safe-across user` | 53 total: 5 e2e, 7 crate `.rs` (entry.rs tests), catalog/diag |
| `invariant:<axis>` → `undivided-by-transit-across` | `:  : invariant:user` | `: undivided-by-transit-across user` | 75 total: 6 e2e, 15 crate `.rs` (carry.rs tests) |
| store emission | `printf ... : fs` / `: kernel` / `: net-kernel` | `: stored-in fs` etc. | ~3 `.sh` + carry.rs/272 test literals |
| whole-kind disturbs + strip → emptied case-arm | `update) : sm.dorc.PkgIndex ;;`-shape | `: disturbs sm.dorc.PkgIndex` | see §6 `flag-emptied-case-arm` |
| `: :` `KIND:@SELECTOR` transitional | `kind:#sel` | `kind:@sel` | rare (transitional) |

Heaviest `.rs` test-literal files (from `grep -rc`): `plan/src/survival.rs` (21),
`analysis/src/effect.rs` (20), `plan/tests/render_corpus.rs` (19),
`oracle/src/predict/derive.rs` (15), `oracle/src/predict/parser.rs` (13),
`oracle/src/lib.rs` (12), `plan/src/lib.rs` (11), then `sweep/`, `touches.rs`, `hostsim/`,
`coverage/`, `core/` (`diag.rs`, `catalog.rs`, `coord.rs`, defining-cases). The two crate
`CLAUDE.md`s carrying authored-surface mark spellings: `oracle/CLAUDE.md:45-46`
(`Service:"$2"#enabled`/`#active`), `analysis/CLAUDE.md:46` (`Package:nginx#installed`),
`syntax/CLAUDE.md:40` (inline-bind example). Charter §2 folds the seven crate CLAUDE.md
authored-surface blocks into this lane.

`~SUSPECT` the SINGLE biggest golden churn is the render-side `@`: `plan/src/lib.rs:4238-4240`
changes one format string, and every `# site N: <label>` line across the 94 `e2e` golden
sets + `render_corpus.rs` re-blesses. That is mechanical (a render flip), not per-file hand
editing.

---

## §5 — Cutover plan (`28A:rul-respell-atomic-cutover`; NO shipped dual-parse)

### Commit ladder (additive-first, e2e stays green until the cutover)

1. **AST reshape** — grow `MarkKind` to the verb set + typed payload enum; rename
   `prop`→`selector`. Update the six consumers to read the typed payload behind
   compatibility shims that still accept the old string decode. Unit tests only; e2e
   untouched.
2. **Diagnostic mints** — add the 4 `DiagCode`s (variant/slug/registry/params/catalog
   `[unwritten:]` + `MIGRATED_*` + covered() cases + goldens + unwritten-ceiling bump).
   Green in isolation.
3. **Lexer `#:` recognition** + parser verb-loop + head-decode + continuation + rc-arity,
   BEHIND acceptance of BOTH old and new spellings during the ladder ONLY IF that keeps e2e
   green cheaply; otherwise keep the parser on the OLD grammar and land the machinery
   unwired. `~SUSPECT` the parser restructure is large enough that steps 1-3 land the
   scaffolding but the parser stays old-grammar until step 4.
4. **THE cutover commit** — flip the parser to the new grammar + the render `@` +
   `split_mark_target`/`intern_coordinate` `@`-split + strip's `#:`/standalone shape +
   the mechanical corpus respell (all `.sh` fixtures, `.rs` test literals, crate CLAUDE.md)
   + regenerated goldens, all in ONE commit, four gates + foreground e2e green at that
   commit. No dual-parse ships.

### Golden regeneration mechanics (WORKING-STATE only in this worktree)

- e2e: `BLESS=1 sh e2e/run.sh` re-blesses all 94 `expected.out`/`.ran`/`expected-*` from
  the freshly-built `target/debug/dorc`. `spike/CLAUDE.md` warns BLESS is EXCLUSIVE and
  authoritative bless is the conductor's at lane close — my bless is WORKING-STATE
  (`28A:rul-respell-atomic-cutover`).
- defining cases: `DORC_DEFINING_BLESS=1` regenerates `tests/defining_cases/<slug>.{machine,
  terse,prose}` (bless-free unit capture, `catalog_defining_cases.rs:406-413`).
- `render_corpus.rs` twin: re-bless via its own path.

### Which goldens churn, roughly

- ~94 e2e golden sets (the `@` render + any respelled oracle bytes in the probe echo).
- `render_corpus.rs` (~19 label sites).
- 4 new defining-case triples + any respelled existing prose examples in `catalog.rs`
  (the `sm ` example strings mention `tolerates:`/`#installed` — those update per charter
  §2's "spelling-mentions inside `sm `-prose update").

### Risks + rollback points

- **R1** parser restructure regressions (standalone/continuation/rc-arity) — mitigate by
  landing the AST + consumers (steps 1-2) first, so step-4 is parser+respell only; rollback
  = revert the single cutover commit.
- **R2** `@`-render vs strip byte-floor — the `@` is a mark-only spelling; strip must still
  reach floor POSIX (`@`-selectors live only in marks, erased whole). Verify the two-binary
  floor (`276:rul-spec-two-binary-floor`) on stripped output post-cutover.
- **R3** golden over/under-bless hiding a real elision regression — `spike/CLAUDE.md`:
  "Bless cannot prove an elision RIGHT — review by eye." The conductor's case-by-case diff
  at lane close is the backstop.

---

## §6 — Ambiguity / flag list

- **`flag-ratchet-forbids-additions`** (§3) — `28A:rul-ratchet-accepts-new-codes` is
  contradicted by the as-built `ratchet_only_shrinks` + `unwritten<=1` gates. Needs a
  conductor ruling: covered-cases-with-unwritten-prose + ceiling bump (my recommendation),
  never a ratchet grow.
- **`flag-standalone-single-colon-restructure`** — `281` §11's single-`:` standalone intro
  (`: undivided-by-transit-across fs-view`) breaks the as-built `parse_command` leading-`:`
  handling (parser 789-799) and `strip.rs::is_bare_colon_host` (which keys on `words==[":"]`,
  the old `: : token` shape). This is the largest structural restructure; the `:` must
  double as intro AND sh no-op. `~SUSPECT` this is where phase B spends the most care.
- **`flag-inline-bind-vs-head-decode`** — the inline bind `pkg : sm.dorc.Package = "$1"`
  has a period-bearing kind; head-decode rule 2 would read it as an `asserts` coordinate.
  The as-built `parse_word_led` pre-check (`name : bare-kind [= value]`, keyed on the kind
  having no INNER `:`) must run BEFORE head-decode, since `sm.dorc.Package` is bare
  (no inner `:`) — preserve that dispatch order. `281` §8 confirms the `= value` tail
  disambiguates; verify the nullary Singleton form survives.
- **`flag-brace-shape-divergence`** — `@{a,b}` is attached (old `#{a,b}` shape) but
  `safe-across {user,fs-view}` / `lends {a,b}` is a SPACE-separated payload word — two
  brace shapes under one "general combinator." Today `tolerates:{...}` is attached
  (`entry.rs:318` `expand_dimension_set`). The dimension-verb payload path must accept the
  standalone-brace word; `brace_tokens` generalizes.
- **`flag-emptied-case-arm`** — a whole-kind disturbs claim as a standalone `:`-hosted
  mark inside a `case` arm (`update) : disturbs sm.dorc.PkgIndex ;;`) strips to an EMPTY
  arm (`update) ;;`). Verify the strip leaves valid POSIX under posh∩dash (an empty case
  arm is legal, `~SUSPECT` confirmed but untested), and that an INLINE (same-line)
  `:`-mark-in-arm strips correctly (the whole-line-delete path assumes the mark is on its
  own line — an inline arm mark needs the region-delete path, not line-delete).
- **`flag-reads-verb-dual-spelling`** — `reads` (off-head word) and `:?` (sugar) both map
  to `MarkKind::Observe`; `derive.rs` must treat them identically (backing-widening,
  `277` §5). No conflict expected; note for the builder.
- **No wire-slug (DiagCode) rename proposed.** Existing slugs stay (wire permanence,
  `280` §2). The 4 new slugs are additions, not renames. If the conductor wants
  `tolerates-unknown-dimension` → a `safe-across`-worded slug, that is a WIRE question I
  flag, never resolve (the wire token is permanent once published; only the `sm `-prose
  mentions update).
- **No `281`-vs-member conflict found.** I cross-checked every verb's member semantics
  against `273`/`272`/`27C`/`277`: the spellings move, the member contracts (per-netns
  caveat, enumerate-every-dimension, per-function/per-dimension vouch, first-vs-second-order
  disturbs, backing-widening) are UNCHANGED. `281` §5's `disturbs` unification matches the
  as-built shared `mark.target.kind` read in both `touches.rs` and `reaches.rs`.

---

## §7 — Sizing + proposed phase-B checkpoint structure

`~SUSPECT` this is a LARGE lane — the parser/AST restructure is the bulk, the respell is
mechanical-but-wide. Proposed checkpoint structure (each a green-gate boundary):

- **CP-A (AST + consumers)**: grow `MarkKind`/`MarkTarget` to typed verbs; migrate the six
  consumers (`carry`/`entry`/`wrapper`/`touches`/`reaches`/`derive`) to the typed payload;
  unit tests. e2e untouched (parser still old-grammar). — biggest correctness surface.
- **CP-B (diagnostics)**: 4 codes end-to-end (variant→catalog→`MIGRATED_*`→covered() +
  goldens + unwritten-ceiling bump). Resolve `flag-ratchet-forbids-additions` per the
  conductor's CP-A/B ruling FIRST.
- **CP-C (lexer + parser)**: `#:` recognition; verb-loop + head-decode + continuation +
  rc-arity; the standalone single-`:` restructure; the inline-bind dispatch preserved.
  Unit tests; e2e still on old spellings if feasible, else this merges into CP-D.
- **CP-D (THE cutover)**: parser flip + render `@` + intern `@` + strip `#:`/standalone +
  mechanical corpus respell (`.sh` + `.rs` literals + crate CLAUDE.md) + golden re-bless,
  ONE commit, four gates + foreground e2e green. No dual-parse.

Between CP-B and CP-C is the natural conductor-review checkpoint
(`27U:map-then-execute-split`): the grammar machinery is landed and unit-green, the
corpus is untouched, so a grounding/license error is caught before the wide respell.

The conductor rules on this note; I stop here (phase A). Phase B (me, continued) executes
after the ruling.
