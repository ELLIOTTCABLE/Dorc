# 294 — Oracle loading and resolution: the build ledger

Builder ledger for `plans/28K` (the function-environment pass), branch `ai/r28-oracle-loading`,
based at `ai/main` @ `7068224d`. Append-only; one section per stage as it lands. Confidence marks
per `spike/CLAUDE.md`. This is the *what strained* record — the design authority is `28K` itself
and the code is its own documentation.

Stage order, as ruled: A load-inert · B source-file identity · C-v0 function-environment domain
(literal source targets) · D environment drives every lift · E cross-unit shadow refusal ·
**[conductor checkpoint]** · F pin-by-definition-bytes · G two-kind fixture respell ·
H differential load-order battery · C2 value-flow source targets (budget permitting).

## Stage A — the marked-file load-inertness gate (LANDED)

`28K` §2a `rul-marked-file-is-load-inert`, implementing this crate's long-standing but unbuilt
`oracle/CLAUDE.md declarations-only-files` law. Marker-gated; a marked file's top level must hold
only function definitions and bare assignments whose values expand statically.

### fnd-two-parsers-disagree-on-funcdefs (carry into stage C — load-bearing)

`dorc_syntax::parse` and the dialect parser (`predict::lift_predicts`) do not agree on what a
function definition is, and the disagreement is SILENT. Measured, on the `munge-name-invalid`
fixture's `中pkg__predict() { :; }`:

- the dialect parser lifts it as a role funcdef (which is why the charclass refusal fires at all);
- `dorc_syntax::parse` yields THREE top-level items — a bare `Simple` command, an empty
  `Subshell`, and a `Group` — and emits **zero** diagnostics.

+SURE of the measurement (probed directly). This matters well beyond stage A: `28K` §2's function
environment reads `dorc_syntax::parse`'s `FuncDef` nodes, while every role lift reads the dialect
parser. Where the two disagree about what is defined, the environment and the lifts disagree about
which oracle answers a site — and today that disagreement produces no diagnostic at all.

Containment, ~SUSPECT but well-supported: the divergence is confined to names the sh lexer will not
take as a NAME, and `reserved.rs`'s charclass refusal already rejects exactly that class at Error
severity, so the two parsers agree on every name that can legally ship. Stage C should pin that
containment with a test rather than assume it, and `syntax/CLAUDE.md top-reject-here` arguably owes
a loud `Unsupported` where a funcdef header fails to lex — flagged, not fixed (out of lane).

### dec-one-diagnostic-per-file-not-per-item

First cut minted one diagnostic per offending top-level item. Measured against the real corpus that
was wrong: `unmodeled-wall-inventory` (a book wearing an `oracle.sh` filename) went 4 → 9 errors and
buried its own pinned info line, and `munge-name-invalid` went 1 → 4, the three extras all being
consequences of the single unlexable funcname above.

Collapsed to at most ONE mint per file, spanned at the FIRST offending item. The argument is the
mis-attribution test, not aesthetics: load-inertness is one claim about one file with one
remediation ("make this file definitions-only"), so N mints are a correlated cascade
(`AGENTS.md` fail-fast: only root-cause is reported) and N-1 of them point the author at the wrong
thing (`271:rul-sin-ordering` ranks mis-attribution worst). Cost, accepted: an author fixing the
first offender re-runs to find the next. If a count is ever wanted for prose, a payload field is a
cheap addition — the payload is deliberately empty today.

### Corpus churn (all four re-blessed, each +1 line, every prior pin byte-identical)

`mark-rc-arity-exceeded` · `mark-standalone-rc-consumer` · `unmodeled-wall-inventory` ·
`munge-name-invalid`. The first three were forecast by the CLI-round survey; `munge-name-invalid`
was not, and it is the one that surfaced `fnd-two-parsers-disagree-on-funcdefs`.

The two `mark-*` cases put their mark at top level *structurally*: `lint_mark_subset` only runs for
a file containing no `__`, so wrapping the mark in a funcdef to make the file legal would stop the
case firing at all. Re-blessing was the only option, and both errors firing is honest.

### res-load-inert-conservatism

Refused-for-now, each cheap to relax and expensive to re-tighten
(`271:rul-posix-in-spirit-defaults`): `export`/`readonly` (commands by AST shape); the operator
forms of parameter expansion (`${x:-y}`). The second has a real reason beyond conservatism — the
lexer collapses EVERY operator form to one opaque `ParamComplex` and discards the body, so
`${x:-$(hostname)}` is indistinguishable from `${x:-lit}` at this layer, and accepting the shape
would accept a hidden command substitution. Relaxing it needs the lexer to retain the body first.

### res-syntax-owes-a-loud-unsupported-on-an-unlexable-funcdef-header

OUT of this lane, routed as a follow-on; recorded here with the repro so it survives to fold.
`syntax/CLAUDE.md top-reject-here` requires anything unmodeled to become an explicit `Unsupported`
node plus a loud `Error`. A funcdef header the lexer cannot take as a NAME does neither. Repro,
measured: `dorc_syntax::parse("# dorc-lang/v0.2\n\u{4e2d}pkg__predict() { :; }\n")` yields three
top-level items — `Simple{words:[..]}`, `Subshell{}`, `Group{}` — and an EMPTY diagnostic list. The
silent three-way garble is the bug; that it currently fails safe is luck, not design.

### res-oracle-claudemd-bullet-now-implemented

`oracle/CLAUDE.md declarations-only-files` reads as unimplemented law; it is now enforced. The
bullet is not wrong, so it is left alone — a conductor-tier sharpening, flagged rather than taken.

## Stage B — one `SourceFileId` space over every input (LANDED)

`28K` §2a Provenance. `core::OracleFileId` → `SourceFileId`, widened from "which loaded oracle" to
"which loaded input, book or oracle". Rename in place, no alias
(`rul-strawman-formats-no-compat`). Zero golden churn, and that was designed rather than lucky.

### dec-load-order-is-the-id-order

CLI-named oracles keep ids `0..n`; the book takes `n`. Two things fall out, and both are why the
order is not arbitrary. Every id already minted keeps its value, so admitting the book to the space
moves no threaded span and no golden — the alternative (book first) would silently re-point every
threaded oracle span at the wrong file, a failure no golden can show. And the order IS `28K` §2's
ambient-prefix order (CLI files "before line 1", the book's text after), so an id COMPARISON is a
load-order comparison — which is exactly the primitive the cross-unit shadow refusal needs in
stage E.

### dec-lift-lanes-keep-the-oracle-only-vectors

The combined table feeds the REPORTING seats only (`emit_static_decline_notes` /
`emit_survival_attribution` / `emit_guard_attribution` / `emit_why_report`). The lift and ship
lanes deliberately keep the oracle-only vectors: they zip per-file lifted sets POSITIONALLY, and
the book is not one of those. Appending the book there would have "worked" via zip truncation —
an implicit dependency on a silent length mismatch, which is the shape of bug this codebase is
built to refuse.

### res-book-span-consumers-arrive-in-stages-d-to-f

The book's id has exactly one consumer today (a test). That is honest rather than speculative: the
real consumers are the shadow-refusal citation (E) and pinned-definition attribution (F), both
net-new code. No `book_source_id` helper was minted for a caller that does not exist yet.

## Stage G prelude — the ambient-bind precedence bug (fix LANDED; one face open)

`28K` §7 justifies the two-kind respell on "one body may mint cells of many kinds; marks are
per-line, so nothing ties one function to one kind". The engine contradicted that. Found while
attempting §7's "mechanical fixture edit", which is why the respell's stated COST was wrong.

### fnd-ambient-bind-outranked-the-marks-own-coordinate

An inline bind (`pkg : sm.dorc.Package = "$1"`) set the kind for everything reached after it, and
a mark's own spelled coordinate was consulted only as a fallback. Measured, same binary, on a
two-line book (`apt-get update` / `apt-get install -y nginx`):

| oracle shape | sites |
|---|---|
| original PAIR (`package` + `pkgindex`) | 2 |
| naive merge (update arm added to a shared-bind body) | 0 |

Two corrections to the first reading of that table, both from bisection, both worth keeping
because each cost a wrong hypothesis. `install` was never broken — the zero was `apt-get update`
being the book's FIRST command, so an unmodeled `update` poison-walls every downstream site. And
the original fixture collapses to 0 identically if `pkgindex.oracle.sh` is merely DROPPED, which is
what proved the fault was un-modelled-update rather than the merge itself.

The lift stays SILENT throughout: `dorc lint` reports clean while the body models nothing. That is
the `26G` silence-is-the-common-cause class one layer below its backstop — the funcdef lifts, so
`unlifted_role_fns` sees nothing to report. The precedence fix removes THIS instance, not the class.

### dec-precedence-fix-in-two-commits

Split so the flip could claim corpus-invisibility as its own property. Commit 1 respelled the last
undotted bind kinds (`pkg : package` → `sm.dorc.Package`) and moved the five expectations that
encoded the pre-respell world — verified green under the OLD precedence, since once bind-kind and
mark-kind agree, precedence is moot. Commit 2 flipped precedence in `derive::push_effect` and
removed the `self.annotation.is_none()` gate on the evaluator's singleton re-point (the same bug's
second face: a reached bind suppressed an entity-less coordinate, so a singleton inherited both the
bind's kind AND its operand). Commit 2 moved ZERO tests.

Verified for the flip: no coordinate form parses kindless — a dotless payload lexes as a VERB
(`281` §4's keystone) — so `ctx.kind` now survives only for the documented nullary-singleton path.

### fnd-legacy-short-kinds-in-binds

Counterexample to `oracle/CLAUDE.md grammar-is-v0.2`'s "the r28 cutover retired the old spellings
corpus-wide; nothing left to convert": bind-POSITION kinds were missed. `spike/fixtures/package.oracle.sh`
still bound the undotted `package` while its marks already spelled `sm.dorc.Package`, and five unit
expectations asserted the undotted answer. They passed only because the bind outranked the mark.
(That file's markerless-yet-dialect inconsistency, `293` §7, is a SEPARATE sibling item and was
deliberately left alone.)

### fnd-canonical-arm-local-shape-sidesteps-the-residue

One face survives the fix: a bind ANYWHERE later on the path clobbers an established singleton
re-point (`Stmt::Annotation` writes unconditionally), and a shared-bind-before-case body still fails
for a not-yet-isolated reason. But the CANONICAL authoring idiom — verb-`case` first, binds
ARM-LOCAL, as USER_STORY's `foobar` and the oracle-contract worked minimum both spell it — is
correct on the landed code:

| book | canonical merged body | original pair |
|---|---|---|
| `apt-get update` | 1 site | 1 site |
| `apt-get install -y nginx` | 1 site | 1 site |
| both | 2 sites, `sm.dorc.PkgIndex@fresh` + `sm.dorc.Package:nginx@installed` | identical |

By-value `Ctx` recursion isolates each arm: update's arm carries no bind and no operand guard, so
no ambient kind exists on its path. The awkward shape was the shared bind, not the multi-kind body.
NB the arms must use the `if` form, never a bare `[ … ] || return 2` in statement position —
`26G` F3 records that voiding a whole funcdef.

### tbl-ambient-annotation-sites (the fold item's specification; READING ONLY, no fixes)

The principle each site is judged against: *a bind is an entity-identity channel — it resolves
entity REFERENCES and kind-tags them for the book-site back-map. It is never a kind authority and
never a cell authority. A mark's own coordinate is authoritative for its cell in full (kind,
entity-arity, selector); ambient state may only fill what a coordinate genuinely lacks.*

Complete enumeration — there is no third holder (`parser.rs`'s `annotation_*` are PARSE-shape
helpers, not ambient state; `payload.rs`/`predict.rs` hits are test names).

| # | site | kind | verdict against the principle |
|---|---|---|---|
| 1 | `derive.rs:88` `kind: None` init | write | OK — no ambient kind until one is authored |
| 2 | `derive.rs:116` `Stmt::Annotation ⇒ ctx.kind = Some(..)` | write | SUSPECT — records a bind as a kind authority at all; harmless only because site 4 now demotes it to a fallback |
| 3 | `derive.rs:134-136` `arm_ctx = ctx.clone()` | propagate | OK — by-value recursion isolates arms; this is what makes the canonical arm-local shape work |
| 4 | `derive.rs:190` `.or_else(ctx.kind)` | read | FIXED — mark's coordinate first, ambient only when a coordinate parses kindless (no form does) |
| 5 | `eval.rs:601` singleton re-point | write | FIXED — was gated on `annotation.is_none()`, so a bind suppressed an entity-less coordinate |
| 6 | `eval.rs:716-717` `if annotation.is_none() { annotation = (anno.kind, entity) }` | write | OK, and NOT last-wins — the bind is FIRST-wins, so it cannot overwrite an established re-point |
| 7 | `eval.rs:740` `match self.annotation` in `finish` | read | OK — reads whatever the path settled on |

### fnd-unresolved-bind-value-tops-the-whole-check (NON-precedence; exit criterion tripped)

`eval.rs:709-711`, in `run_annotation`:

```rust
Some(value) => match self.resolve(value) {
    Ok(text) => ResolvedEntity::Operand(text),
    Err(_) => return Flow::Top(TopReason::UnresolvedAnnotationValue),
},
```

`resolve` runs `UnsetPolicy::Unresolved`, so an unset positional is non-concrete ⇒ `Err`. For a
NULLARY verb (`apt-get update`) a shared bind `pkg : … = "$1"` therefore tops the ENTIRE check —
even when the path's cell was already fully determined by an entity-less coordinate, and even
though site 6 means the bind never overwrote anything. This is not a precedence question and no
amount of precedence work reaches it; the sweep's exit criterion applies and the sweep-and-fix
routes to fold unstarted.

Corrects the earlier `Stmt::Annotation`-clobber reading: the m1-vs-m2 measurement (case-only body
resolves `update`; case + ONE trailing bind does not) was real, but the mechanism was this, not an
overwrite. Site 6 forbids the overwrite outright.

### res-tripwire-had-no-target

The commissioned loud-conservative tripwire was specified against the clobber shape. Site 6 shows
that shape cannot occur, so the tripwire has nothing to guard and was not built. The genuine
under-modeled shape is `fnd-unresolved-bind-value-tops-the-whole-check`, which already fails
LOUD-conservative by construction (`Flow::Top` ⇒ decline ⇒ the site runs).

### fnd-dialect-tests-admit-only-string-comparison

CORRECTS an earlier over-claim of mine that `[ … ] || return 2` in statement position voids a
funcdef. It does not. The real constraint is the test GRAMMAR: only `=`/`!=` string comparison is
in dialect. Measured, one file, `dorc lint`:

| spelling | outcome |
|---|---|
| `command -v tool >/dev/null 2>&1 \|\| return 2` (oracle-contract §3's standard gate) | CLEAN |
| `[ "${2-}" = "" ] \|\| return 2` (`oracle/CLAUDE.md` R2-MULTIOP's arity gate) | CLEAN |
| `[ -n "$1" ] \|\| return 2` | VOIDS the funcdef |
| `if [ -z "$1" ]; then return 2; fi` | VOIDS the funcdef |

So there is NO doc-vs-engine conflict: every gate the docs actually prescribe lifts cleanly. What
is out of dialect is the unary file/string test family (`-n`, `-z`, and by extension `-f`, `-x`),
which is ordinary defensive sh an author will reach for unprompted. It fails LOUDLY — both
`predict-out-of-dialect` (naming the operator) and the `26G` unlifted backstop (naming the funcdef
it took down) fire — so this is a dialect-reach question for fold, not a silence bug.

### res-shared-bind-multi-kind-precision-limit

NAMED, not chased: a shared-bind-before-case body with multi-kind arms still resolves nothing, and
the residual mechanism is unproven (suspected: the operand guard going undecidable for a nullary
verb). Fixtures are authored in the canonical shape instead, so no corpus case depends on it.

### res-multi-book-concatenation-still-violates-lineno-identity

Untouched and out of scope: `read_books` still concatenates multiple books newline-joined and keeps
only the first path, so a multi-book unit's line numbers are offsets into the concatenation
(`AID-NEEDS:law-lineno-identity`). Pre-existing, orthogonal to `28K`, and every corpus case is
single-book. The per-book offset map belongs to the CLI-inputs round.
