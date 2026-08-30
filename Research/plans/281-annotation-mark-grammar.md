# 281 — The annotation mark-grammar (THE spec)

Plan-tier, kept-current: the specification of Dorc's line-annotation surface — the "marks"
an oracle author writes to say things about a line of shell. Part I is the language surface,
written to be implemented from directly; Part II is the design rationale; the closing
grep-map records prior spellings for corpus migration. This document is authoritative on the
mark grammar and its verb vocabulary. It supersedes the worked-minimum grammar of `277` §4
and takes over the grammar `278` §6 deferred to `277`. Authority order: root docs,
`spike/CLAUDE.md` rulings, `plans/271`, and any human-typed ruling outrank this. Companion
specs it does not restate: coordinate/entity algebra `277`; wrapper/context members
`273`/`plans/27C`; store/topology members `272`; base-dialect floor `276`/`278`. Open
cross-cutting reconciliations (kOOB, KNOBS) are listed in §12.

---

# PART I — THE SPECIFICATION

## §1 — Scope, carriers, and the accepted hazard

A **mark** is a strip-only annotation an author writes about **one line of code** (one
statement). Marks are metadata riding the sanctioned trailing-annotation lane (`KNOBS:kOOB`),
never sidecar configuration; `dorc strip` removes them to reach floor-legal POSIX (§9).

Marks are not *meant* to execute — the intended paths never run them (Dorc parses them at
compile time and ships only stripped oracle bytes, `271:rul-only-oracle-bytes-ship`; the
off-ramp runs the stripped file; the `#!/usr/bin/env dorc-sh` shebang is meant to catch a raw
run of an unstripped file). But "meant to" is exact: the shebang is a safety-net, not a wall.
If an unstripped marked file reaches a shell another way — piped to `sh`, `source`d,
stripped-then-forgotten — a mark **can** execute, and when it does the colon form corrupts
the command's argv and a standalone continuation `:` line clobbers the tool-rc to 0 (a
silent always-converged). This is a real, accepted, live hazard, mitigated but never
eliminated. It is the price of the colon form's salience.

There are two carriers, semantically identical, chosen per physical line:

- **colon form** (`:`) — the default. Salient: it lexes and highlights as ordinary shell in
  every parser checked (GitHub, VS Code, tree-sitter-bash). It carries the hazard above.
- **hash-colon form** (`#:`) — the opt-in comment carrier, for authors who value
  comment-inertness over salience. Inert under any shell, on every route; distinguished from
  an ordinary comment and from the version marker by the immediate colon (no space); stripped
  only when it parses as a valid mark-block (§9). Its cost is highlight demotion: the leading
  `#` greys the block in some renderers. An author handling genuinely dangerous state should
  prefer it.

## §2 — The mark-block is an unordered set

A mark-block is a **set** of marks, not a sequence. Order carries no meaning; any mark is
freely reorderable (a core mark loses only its head sugar, §3, when moved off the head). The
governing principle:

> **Annotations annotate the line of code, never each other.** No mark's meaning depends on
> another mark's presence or position.

A fact's backing (§7) is therefore *derived by the engine from co-location on the same line*,
never by one mark referencing another; and no annotation-modifies-annotation construct exists.

## §3 — The intro grammar

```
mark-block := intro WS mark ( WS mark )*
intro      := ( ':' | '#:' ) [ SUGAR ]        # SUGAR immediately follows the colon, no space
SUGAR      := '!' | '?' | '='                 # closed set; head-only; core cell-and-value relations only
WS         := one or more spaces/tabs
```

- The intro is a single `:` (colon form) or `#:` (hash-colon form), optionally followed with
  no intervening space by one sugar character, then a required space. Legal intros:
  `:`  `:!`  `:?`  `:=`  `#:`  `#:!`  `#:?`  `#:=`.
- Sugar is a **head-only lexical shortcut**, reserved for the core cell-and-value relations
  (§5): the verdict pair (omit / `!`), the read (`?`), and the bind (`=`). All meta verbs are
  always spelled as words; a core mark moved off the head is spelled with its word verb too.
- A mark-block either **trails a statement** (binding to that statement's rc or value) or
  **stands alone** (scope per its verbs' member semantics — e.g. a whole-member invariance
  line). It may spill across physical lines: each continuation line re-introduces with its own
  `:` / `#:` and accrues to the same block. Continuation `:` lines are the `:` builtin (inert);
  continuation `#:` lines are comments (inert). "Do not repeat the colon" is scoped *within* a
  physical line (chain `... : verb X verb Y`, never `... : X : Y`); *across* lines you re-intro.

## §4 — Parsing: three head cases, then verb-driven

After `intro WS`, the **first** mark is decoded by three rules, in order:

1. **sugar present** → the payload is of the sugar's type; the verb is the sugar's verb.
2. **no sugar, first token contains a period** → it is a coordinate; the verb is the positive
   verdict, `asserts` (the omitted-verb default).
3. **no sugar, first token contains no period** → it is a verb; parse it and its payload.

Every subsequent mark is **verb-driven**: read a verb, consume exactly the payload its arity
and type fix, repeat. The period test is consulted only for the head token, and only to
separate a verb from a coordinate.

These three head rules apply **per intro line**: because each continuation line re-introduces
(§3), its first token is decoded by the same three rules afresh — a bare-coordinate `:`
continuation reads `asserts`, a `:?` continuation reads `reads` — never as a verb-driven
continuation of the previous line's tail (so a bare-coordinate continuation under a head
verdict legitimately reads `asserts` and trips the §7 rc-arity rule; the correct behavior,
`28A:rul-per-intro-head-decode`).

**Keystone invariant** (`rul-verbs-dotless-kinds-dotted`, the entire disambiguation surface):
verbs are period-free; a coordinate's head (its kind) always contains at least two periods
(reverse-DNS). Nothing else constrains periods — second-slot payloads (dimension/substrate/axis
tokens, quoted entities, paths) may contain periods freely, reached verb-driven.

Lexing follows shell word-splitting and quoting (whitespace separates; quote for embedded
spaces), but Dorc **resolves, it does not expand**: a `"$var"` in a coordinate resolves through
value-flow to entity identity, never through shell expansion.

## §5 — The verb vocabulary

Engine-owned, closed at a version, extends by new name only. Which verbs consume the rc, carry
sugar, and take coordinates vs kinds vs tokens is fixed here.

**Core cell-and-value plane — sugar-eligible:**

| verb | rc-consuming | head sugar | payload | meaning |
|---|---|---|---|---|
| `asserts` | yes | omit | coordinate | rc 0: the command finds the cell present/true |
| `refutes` | yes | `!` | coordinate | rc 0: the command finds the cell absent/false |
| `reads`   | no  | `?` | coordinate | this line reads the cell (discloses backing) |
| `bind`    | no  | `=` | kind       | the value assigned on this line is an entity of this kind (§8) |

At most one rc-consumer (`asserts`/`refutes`) per line (§7). `refutes` is the exit-code's
*sense*-flip — rc 0 witnesses the cell is false — not "divergence" (§R6).

**Meta plane — never sugared, always word verbs.** Grammar here (verb + payload); member
semantics live in the cited specs:

| verb | payload | member / spec / meaning |
|---|---|---|
| `safe-across`                 | dimension token | context vouch, `plans/27C` §2 — the body is read-only when executed shifted along the dimension |
| `disturbs`                    | kind            | **both** `cmd__disturbs()` and `kind__disturbance_reaches()` — one verb; the member fixes first-order footprint vs transitive reach |
| `lends`                       | dimension token | `cmd__lend_map()`, `273` |
| `stored-in`                   | substrate token | `kind__state_stored_only_in()`, `272` — the kind's state lives in this substrate |
| `undivided-by-transit-across` | axis token      | axis-invariance, `277` §4e — the kind's store is not forked by transit across the axis, so a fact travels without entry |

Two structural rulings, both decided by the orthogonality test
(`271:rul-orthogonality-counterexample-test`):

- **`disturbs` and reach are one verb.** They are one relation — disturbance — at two orders
  (a command's direct footprint; a kind's transitive reach). The enclosing member is the
  *receiver* (command vs kind), not a hidden meaning-flip, so one verb loses nothing; there is
  no inhabitable oracle where the two want to point differently. The member name
  (`disturbs` vs `disturbance_reaches`) still carries first-vs-second-order; the
  completeness contract rides the `disturbs nothing-else` report-lane record (`plans/30U`),
  never the name.
- **`safe-across` and `undivided-by-transit-across` are deliberately distinct** (and
  deliberately spelled unlike). They both concern a dimension but are orthogonal relations
  with different licenses: `safe-across` licenses entering-and-executing a body; invariance
  licenses a fact traveling *without* entry. The inhabitable counterexample is the babby-sudo
  case — a check that runs read-only as root yet answers differently as root (safe-across, but
  its store is per-user, so not undivided). One verb would silently merge two licenses; a false
  invariance is the costlier, knife-tier error, so its verb is made unmistakable.

Emission members (`disturbs`, `lends`, `stored-in`) carry their entity/value at runtime as the
line's stdout; the mark types it. That shape and its per-member semantics belong to the specs
above.

## §6 — Coordinates, the `@` selector, brace-alternation

```
coordinate := KIND [ ':' ENTITY ] [ '@' SELECTOR ]
KIND       := reverse-DNS labels, >= 2 periods          # sm.dorc.Service
ENTITY     := bare (letters digits . _ - /) | double-quoted ("$var" via value-flow)
SELECTOR   := POSIX name (letter/underscore, then letters/digits/underscores)
```

- `@` introduces the selector, attached (no space). It needs no comment-avoidance rule.
- A selector-less coordinate means the whole entity, reading as a ⊤-selector at consumers
  (collides with every cell). Entity-less transitional form: `KIND:@SELECTOR`.
- Polarity rides the verb (`asserts`/`refutes`), never the coordinate.
- **Brace-alternation is a general payload combinator**: `@{enabled,active}` expands to one
  coordinate per selector; `safe-across {user,fs-view}` to one mark per dimension. Allowed
  wherever multi-cell is semantically legal; refused only where it would fabricate a multi-cell
  *verdict* (`asserts`/`refutes` payloads), by rc-arity.

## §7 — The one hard composition rule, and backing

> **At most one rc-consuming mark (`asserts` or `refutes`) per line.** A line has one exit
> code; a verdict maps that one code onto one cell's truth. Two independent cells need two
> codes — one code cannot witness "A converged, B diverged", and which cell diverged decides
> what runs. This is rc-arity, the whole content of the old "one cell per line" rule.

`reads` and all meta marks do not consume the rc; any number may accompany the one verdict.
A **backing** of a verdict fact = { its own cell } ∪ { cells named by `reads` marks on the
same line }, per-channel/recipe-granular (`275` §2), derived by the engine from co-location,
order-independent.

## §8 — Bind: two forms

A bind types a **value** as an entity of a kind (value-plane; binds name entities, never cells;
no selector, no `@`; SOFT/provisional, `271:rul-binds-entity-only-provisional`). Two forms:

- **inline** — `name : KIND = "$value"`: types the value in argument-position, disambiguated
  by its `= value` tail. Most readable at the assignment itself.
- **trailing mark** — `FOO="bar"   := sm.dorc.Package` (sugar) or `... : bind sm.dorc.Package`
  (word): rides the assignment, payload a kind, entity being the assigned value (via
  value-flow). `:=` is the assignment mnemonic.

The trailing form is the last construct that had required inline placement, so with it **the
entire annotation surface can ride `#:` comments** if an author wants zero non-comment syntax.
Bind especially wants the comment carrier: a colon-form bind on an *assignment* fails harder
under a raw run than a colon-mark on a command (the shell reads `:=` as an unknown command),
so `#:=` is the safer default for binds. Strip reduces every form to plain assignment. The
division holds: type a value where it lives; describe a line in its mark-block.

## §9 — Strip and the off-ramp

`dorc strip` reaches floor-legal POSIX (`276`/`278`):

- **colon form, trailing a command** → erase from the intro `:` to end-of-block, leaving the
  command as the last status-affecting statement.
- **colon form, standalone line** → delete the line.
- **hash-colon form** → delete the comment **iff it parses as a valid mark-block**; on any
  parse failure, leave it untouched as a plain comment. Marks erase to *nothing*, never to a
  null-command, so no stripped-in `:` can clobber the tool-rc.

Diagnostics diverge by carrier (`KNOBS:kWARN`): a malformed **colon** mark is committed syntax
→ the line's marks drop to ⊤ (run) with a loud plan-time diagnostic (`inv-top-reject`); a
malformed **hash-colon** mark stays a comment but Dorc still diagnoses ("looks like a `#:`
mark, did not parse, treated as a comment") — the comment carrier's graceful degradation is
never silent. The raw-unstripped hazard of §1 remains: marks *can* execute off the supported
paths, which is why `#:` (inert on every route) exists.

## §10 — The salience design-goal

Source-side attention-honesty: `rul-attention-honesty` (a correctness-critical line must never
render demoted) applied to the source surface. A correctness-critical annotation that renders
flat grey to every reader outside the author's editor is lying about its importance. The goal,
and the constraint that makes it achievable:

- **Source salience only from shell tokens that already highlight in our favor** — `"$var"`,
  `{a,b}`, salient-but-shell-legal punctuation (`:`, `@`) — **never from invented syntax.**
  This dissolves the tension between *salient-designed* and *invisible-posix*: salience from
  reused shell constructs is the same thing as looking like shell, not a new language the world
  must adopt. We borrow shell's highlighting; we never ask the world to build ours.
- **Avoid tokens that highlight against us** — chiefly `#` (greys), the reason for `@`.
- **Test against a named battery** (GitHub, VS Code, tree-sitter-bash, a web renderer or two).
  Best-effort, not a hard invariant like the posh∩dash floor — highlighter grammars are
  versioned and drift, so the target degrades gracefully (a token renders plain, never
  poisoned).
- **Two tiers, kept apart:** *passive* highlight (borrowed shell tokens) is this goal; *active*
  loudness (red squiggles, real errors) needs a Dorc LSP (DESIGN §5) and is deferred.

Candidate KNOBS tension: `salient-designed ↔ invisible-posix`, lean =
salience-via-borrowed-shell-tokens (§12).

## §11 — Worked examples

A convergence check saying three things — verdict, a disclosed extra read, a context vouch:

```sh
foobar status --certs-current -- "$dest"   : sm.dorc.Certs:"$dest"@synced
   :? sm.dorc.Policy:"$dest"@current
   : safe-across user
```

Inline (one physical line, same set), then the hash-colon carrier (inert, greyer):

```sh
foobar status --certs-current -- "$dest"   : sm.dorc.Certs:"$dest"@synced reads sm.dorc.Policy:"$dest"@current safe-across user
foobar status --certs-current -- "$dest"   #: sm.dorc.Certs:"$dest"@synced safe-across user
```

A footprint emission, and a brace-alternated multi-cell disturbance:

```sh
printf '%s\n' "$1"   : disturbs sm.dorc.File
apt-get update       : disturbs sm.dorc.PkgIndex@{fetched,valid}
```

A bind, three ways — inline, trailing sugar, trailing sugar in the comment carrier:

```sh
dest : org.foob.Certs = "$1"              # inline: types dest's value where it lives
dest="$1"   := org.foob.Certs             # trailing sugar (colon form)
dest="$1"   #:= org.foob.Certs            # trailing sugar (comment carrier; inert on every route)
```

A kind-owner (fresh kind, fresh domain) declaring where its state lives and that a chroot
cannot fork it — the actor of each mark is the kind's own state:

```sh
sm_dorc_KernelParam__state_stored_only_in() {
   printf '/proc/sys\n'   : stored-in kernel        # state stored-in the kernel substrate
   : undivided-by-transit-across fs-view            # one store a chroot cannot fork
}
```

## §12 — Open items for the conductor (need a human ruling; forward-looking)

- **kOOB second comment-parse.** The `#:` carrier is a second sanctioned comment-parse beyond
  `# dorc-lang/vN`; `24M:rul24M-version-comment` reserved that for a fresh human ruling. It is
  namespace-narrow (only `#:`-immediate, a closed grammar, strip-if-valid) and carries the same
  strip-only marks the colon form does, not sidecar configuration — but it wants the ruling on
  record, and the `KNOBS:kTYANNOT` entry updated (both poles now ship: colon-inline default,
  `#:`-comment offered).
- **KNOBS registration.** Register the `salient-designed ↔ invisible-posix` tension (§10);
  naming is human-authoritative.
- **`@` permanence.** Confirm `@` inherits the permanent-going-forward status the selector
  introducer formerly held (`271:rul-selector-introducer-hash`).
- **Respell timing.** Land the corpus respell (grep-map below) before the stdlib block stamps
  the prior spellings into the seed corpus — same cheap machinery as the earlier `.prop`→`#`
  sweep; goldens churn freely.

---

# PART II — RATIONALE

## §R1 — Why not comments as the primary carrier

Comments are the most shell-native annotation and the only inert, rc-preserving,
strips-to-nothing EOL construct shell offers; a comment-primary design would delete three
hacks (argv corruption, the mid-word-`#` rule, the strip clobber). It loses on one axis, and
that axis wins: **highlighting.** Comments render flat grey by default in every editor without
special tooling — they *remove* attention from correctness-critical annotations, the FlowType
pain of metadata hidden in comments and un-highlighted everywhere but the author's own
configured corner. This is `rul-attention-honesty` on the source surface. The apparent tension
between wanting loud, language-looking syntax and wanting annotations that vanish into shell
dissolves under one constraint: draw salience from reused shell tokens that already highlight,
never from invented syntax — then salient and invisible-as-a-language coincide, and comments
are simply the wrong pole. `#:` is kept anyway, for authors who value inertness over salience.

## §R2 — Why one intro plus sugar

A single intro with a head sugar keeps the frequent core marks terse (`:` / `:!` / `:?` / `:=`)
while routing the open-ended, ever-growing meta set into word verbs, which never exhaust the way
a glyph alphabet would against a vocabulary that extends by new name forever. The period test
carries the only disambiguation, and only at the head; everything after is verb-driven, so
payloads can be anything. The reorderable-set model makes "many things about one line" a flat
list, never an annotations-on-annotations language.

## §R3 — Why rc-arity is the single-cell rule

One exit code witnesses one cell, and per-cell divergence-resolution is the whole point, so two
verdicts on one line is unmeasurable, not merely disallowed. The orthogonality test forces the
split (the inhabitable "A ∧ ¬B on one measurement") and, run the other way, *collapses* the
verdict+observe objection — a line disclosing its own read is linked, not orthogonal, so it may
share the line and is more honest for doing so. Composition never touches the hard rule because
no one wants two verdicts on one line.

## §R4 — Why `@` over `#`

`#` was chosen optimizing lex-safety and a URL-fragment mnemonic. The salience goal adds a
second criterion, highlight-safety, under which `#` fails: it greys the selector and poisons
the rest of the line in GitHub's renderer. `@` lexes identically safely (literal in a word
through posh∩dash), highlights clean, is visually distinctive, and carries an annotation
connotation. A new criterion flipping a prior "permanent" spelling is the marker-gated agility
`278` §4 exists to permit; permanence resumes at `@`.

## §R5 — Why `#:` is offered

The colon form is salient but corrupts under a raw unstripped run; `#:` is a real comment,
inert and rc-safe on every route even unstripped. Offering both lets the author pick the axis —
salience or raw-exec-inertness — with identical semantics. `#:` is terse and mechanically
unambiguous (`# ` is a comment or the version marker; `#:` is a mark-block), strips only when
valid so a broken one is never silently mis-erased, and diagnoses so a broken one is never
silently ignored.

## §R6 — Verb-naming principles

- **Actor-is-command on the core plane.** `asserts`/`refutes`/`reads`/`bind` all read "this
  command _s this cell (or value)." The meta plane is body- or kind-scoped by nature
  (`safe-across` is about the body's execution; `stored-in` and
  `undivided-by-transit-across` are static kind properties, actor = the kind's state), and that
  asymmetry is expected.
- **The verdict pair names presence-vs-absence, not convergence.** `refutes` flips *which
  truth-value rc 0 witnesses*; it does not mean "diverged." A `refutes` check exiting 0 has
  established the cell is false, which for a book wanting that cell absent is itself converged.
  So a converged/diverged pair would mislabel; `asserts`/`refutes` is precise with a clean
  built-in antonym.
- **Unify iff one relation, never iff orthogonal.** `disturbs`/reach unify (same relation, two
  orders); `safe-across`/invariance do not (orthogonal, different licenses). The orthogonality
  test decides both.
- **Name the claim, not the symptom.** Invariance is a structural claim about the store — that
  the axis does not fork it — enforced as "true of the substrate itself, not of your test
  environment" (`277` §4e). A symptom-level name (e.g. "constant-across": the values happen not
  to vary) is hazardous on a knife-tier verb, because it cues the author to verify the
  observational question ("are my two test contexts equal?") instead of the structural one
  ("does this axis divide my store?") — and a false line silently under-executes someone else's
  site. `undivided-by-transit-across` cues the structural question and is deliberately unlike
  `safe-across`, since confusion here is the most costly.
- **Rare and critical spells out; frequent earns brevity.** The higher-frequency `safe-across`
  is short; the rare, knife-tier kind-owner verb is fully spelled, its length an
  attention-grabbing feature.

---

# Prior spellings — grep-map for the corpus respell (value drops after the churn)

One line per item; migrate the corpus mechanically, then this section is history.

| was (corpus / prior) | is (this spec) | note |
|---|---|---|
| `#` selector introducer (`271:rul-selector-introducer-hash`) | `@` | highlight-safety; §R4 |
| `: :` double-colon token line | single `:` / `#:` intro | one-intro grammar; §3 |
| `277` §4 "three fixed mark positions" | unified verb grammar | §3–§5 |
| positive verdict mark (unnamed) | `asserts` | §5 |
| complement verdict `:!` (verb: `complement`) | `refutes` | §5 |
| `converges`/`diverges` (proposed) | `asserts`/`refutes` | symptom vs claim; §R6 |
| reach-emission verb / `reaches` | `disturbs` (unified) | §5 |
| `tolerates:` vouch (`27C`) | `safe-across` | §5 |
| store emission verb / `stores` | `stored-in` | direction fix; §5 |
| `invariant:<axis>` / `constant-across` (`277` §4e) | `undivided-by-transit-across` | claim not symptom; §R6 |
