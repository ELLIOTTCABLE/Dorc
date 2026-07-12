# 278 — dorc-lang v0.1 reference (one page)

AI-authored (Opus scribe, the task-12 authoring batch, 2026-07-12). Status: **DRAFT —
awaiting conductor review + human delta-acks.**

Authority: root docs and human-TYPED rulings outrank this page. **This page assembles; it
never rules.** Every factual line traces to an already-typed ruling cited inline; on any
conflict between this page and a cited ruling, the ruling wins. Assembled per `276`'s
close ("scribe-work, unscheduled, cheap at any time") from the `271`-accreted authored
additions plus the `276` floor weld. Naming discipline per `270` §1: hyphenated full-word
slugs; outside-document refs as `docID:slug`; research errands cited as
`kwhichsh-gcd/turnNN`.

---

## §1 — The base-dialect floor (the posh∩dash weld)

No language spec is written for this tier, mid-spike or ever. The specification is one
sentence, executable — quoted verbatim from `276:rul-spec-two-binary-floor` (typed
2026-07-12, "It's a solid floor, and I like it"; kWHICHSH **welded** to
`kWHICHSH-minimum-lcd` on it):

> **A valid dorc-lang v0.1 base-dialect text is a stripped file that parses and runs
> identically under `posh <vP>` and `dash <vD>`; where the two disagree, the construct is
> outside the dialect.**

**Version pins** (`276:rul-spec-two-binary-floor`, ruled conductor 2026-07-12 under
delegated authority "decide for us"; empirics `kwhichsh-gcd/turn02`): **`posh 0.14.1` ∩
`dash 0.5.12`.** posh 0.14.1 = the shell in current Debian stable (Trixie), `/bin/sh`-policy
shell across three releases. dash 0.5.12 = the newest official release that still *lacks*
`set -o pipefail` (pipefail enters dash at **0.5.13**, not 0.5.12 — corrected by real-binary
diff of `options.h` + built-binary behavior). *Pipefail-notch coherence:* the floor pin
(0.5.12, last pre-pipefail dash) and DESIGN's executor lean (dash ≥ 0.5.13, first with
pipefail) sit exactly astride the pipefail notch — "pipefail lives above the floor" made
literal in version numbers. `dash-pin-tension` (recorded, free veto): 0.5.11.5 is
battery-identical on all 12 checks, so the conductor may ratify down to it at zero dialect
cost if "gently older" should outrank "newest-lacking."

**`fence-rejection-rc`** (`276:rul-spec-two-binary-floor`, promoted from `kwhichsh-gcd/turn02`):
the sentence's "parses and runs identically" is scoped to **accepted** constructs only —
dash exits 2 where posh exits 1 on *rejected* ones, with divergent error text — so no
dialect rule may ever depend on the exit code or error text of a rejected construct.

**Inheritance:** Debian Policy §10.4 is a 25-year institutionally-maintained definition of
"good portable shell with `local`"; `checkbashisms` is a free linter; posh is the
enforcement binary; maintainer-scripts prove the floor livable
(`276:rul-spec-two-binary-floor`). The human's framing on full-ack: "our target is *write
good portable shell*, which is what I wanted it to turn out as all along"
(`276:rul-base-dialect-ruling-list`).

### Care-set (`276:rul-base-dialect-ruling-list`, HUMAN full-ack 2026-07-12)

| shell | disposition |
|---|---|
| **ksh93** | **OUT** — no `local` at all (`typeset` scopes only in `function f{}`); ksh93-membership and the `local` keystone are one decision. |
| **zsh** | **IN via discipline** — macOS login shell, can't drop; quote-as-law covers the word-split inversion; honest residual = NOMATCH glob-abort ⇒ one quality-bar line: avoid bare globs in oracle bodies. |
| **mksh** | **free-rider** — has `local`; not targeted; revisit only if an Android target surfaces. |
| **posh / yash** | **CI differential-test targets, not members** (care-set membership = whose daily-driven shells we serve; posh's spec/enforcement-binary role in the floor weld is a separate, compatible question). |

### Dialect rulings (`276:rul-base-dialect-ruling-list`)

| rule | content |
|---|---|
| `local` keystone | the dialect is "POSIX + `local`" (reaffirms r23-h3). |
| `local x=$(cmd)` | **permitted**; analyzer treats it rc-opaque and hints under `set -e` (SC2155 masking is real in every care-set shell; the only portable fix is declare-then-assign — `local -r` is itself a bashism). |
| printf-doctrine | never `echo` with flags/escapes. |
| quote-as-law | quoting is law, not style — the one rule that makes the dialect survive the consumer cohort. |
| function form | `f()` only; `function f{}` rejected. |
| bash-family ban | `${x/…}` `${x^^}` `${x:off:len}` `[[ ]]` `==` `<<<` `&>` `|&` — the exact set a bash-habituated author reaches for. |
| `test -a` / `-o` | accept-run / **emit-never** (POSIX-2024 removed them; Debian Policy still mandates support — they run everywhere, we and the stdlib never write them). |
| `$'…'` | **OUT-for-now** — the only permit-candidate resting on an unverified version floor; `printf` covers the use-cases; cheap to admit later, expensive to retract. |

### The pipefail four-lanes (`276:rul-pipefail-four-lanes`, typed strong-ack 2026-07-12)

- **-dialect: IN.** `set -o pipefail` is legal dorc-lang (POSIX-2024); the analyzer models
  pipeline-rc first-class because pipeline rc is verdict-load-bearing (off ⇒ wrong verdicts,
  unsafe; on ⇒ lost elisions, safe — the verdict lane *wants* pipefail).
- **-support-envelope.** The EXECUTOR story is unruled; **non-pipefail executors are an
  explicitly unsupported class** — carved now so no obligation to support {no shipped
  executor} ∧ {pipefail-less ancient host sh} can accrete.
- **-guard-handshake.** Apply-lane availability is a per-host handshake fact (session-start
  known-answer probe), never a version database; absent ⇒ the check body is unshippable
  there ⇒ guard declines ⇒ site runs (fail-toward-run).
- **-strip-idiom.** The blessed paste/stripped spelling is the self-gating
  `(set -o pipefail 2>/dev/null) && set -o pipefail` — floor-safe bytes, errexit-safe (left
  of `&&` is `set -e`-exempt); on ancient shells it degrades to the consumer's ambient
  laxness (the no-worse-than-bare floor).

---

## §2 — The authored additions above the floor

Everything below is dorc-lang text *above* the portable-sh floor; all of it is what strip
(§3) erases or rewrites to reach the floor.

### The `# dorc-lang/v0.1` marker — TYPED (`24M:rul24M-version-comment` shape; `24C:rul24-marker-v0.1` exact spelling, human-typed; `KNOBS:kOOB` sanctioned exception)
Exact-match, stands alone, within the first ~10 lines. **Gates syntax only** — binds,
marks, any non-POSIX construct — and never `__role` name-recognition, which is recognized
in unmarked files too and is a permanent, unversionable surface (`24M:rul24M-version-comment`).
The sole sanctioned comment-parse (`KNOBS:kOOB`, a closed set of one). Per
`276:rul-verdicts-never-stable` the `# dorc-lang/vN` marker gates **language-warts only** —
never a promise to reproduce semantic warts.

### Inline entity binds — SOFT/provisional (`271:rul-binds-entity-only-provisional`, ~GUESS both sides)
Binds name **entities**, never cells; facts about cells attach via marks on
probing/emitting commands. Door left open on counterexample. The inline-bind spelling is
the `KNOBS:kTYANNOT-inline` pole (annotate directly on a command argument).

### Trailing marks — verdict `:` and observe `:?` — in-use (`271` worked minimum; `KNOBS:kTYANNOT`)
A trailing mark rides the end of a statement. `:` = a **verdict** mark;
`:?` = an **observe** mark. Both mint selector tokens on a runnable measurement line
(`271:rul-selector-disjointness-dialect-scoped`); claims/`disturbs` never mint. An Observe
inside a verdict-function body widens that fact's backing to include the observed coordinate
(`271:observe-backing-widening`, drafted). Exact charsets/quoting defer to §6.

### The `#` selector introducer — TYPED, PERMANENT (`271:rul-selector-introducer-hash`)
`#` introduces a selector on the flat three-place coordinate `(kind, entity, selector)`:
`sm.dorc.Service:"$svc"#enabled`. Quoting supported where charsets collide. The bare
selector-less form permanently means "true / occupied / whole-entity"
(`271:rul-coordinate-shape-flat-three-place`).

### The `dorc:sh` / `dorc-sh` / bare-`sh` reentry trio — TYPED (`271:rul-dorc-prefix-head-synthesis`; `274` §1/§8; formal stamp at task-6 close)

| spelling | meaning | analysis | strip |
|---|---|---|---|
| `sh -c '…'` (bare) | the host's real sh; the escape hatch | DESCENDS for hints only, licenses NOTHING | untouched |
| `dorc:sh -c '…'` | the dialect's reentry; "dorc may do as it pleases" | full analysis license | prefix-erased → bare `sh` |
| `dorc-sh …` (typed directly) | the runtime object (pinned evaluator); composes transitively via PATH | NO analysis license | untouched — documented-dangle |

Descend-don't-license enforcement tier = TYPESYSTEM, not test-pin
(`271:rider-invited-rooms-typing`, typed direction). `dorc:sh` is grammar-valid /
world-invalid (colon is an ordinary word char; fails loud-127 under stock shells). No nested
`dorc:sh` — annotation-syntax in opaque blobs is a plan-time parse-failure-tier error
(`271:rul-no-nested-annotation`, TYPED). Row three is untouched by strip
(`271:rul-row-three-documented-dangle`, TYPED; "half-strip is worse than no-strip"). The
bare-`sh` head IS the long-owed `unsafe` escape hatch — discharged by identification, no
second construct will ever exist (`276:rul-unsafe-is-bare-sh`).

### `__role` name-semantics — families, closed vocabulary, extension by-new-name-only
A **family** (`271:rul-family`, TYPED) is the set of non-overlapping/non-contradicting
`__role` functions describing one description-target — two species: a COMMAND (all
`systemctl__*`) or a KIND (all `sm_dorc_Package__*`). Membership is **name-derived only**,
never file, never author (`271:rul-family`; `24M:rul24M-bare-dorcism-names` — dots die, no
prefix, bare munged POSIX NAMEs). The per-species role vocabulary is engine-owned,
closed-at-a-version, extends **by new name only** (`271:rul-family`). In generic discussion
role-functions are written WITH their keying class (`271:rul-class-prefixed-role-names`,
TYPED). Names are a permanent, unversionable compat surface (`24M:rul24M-version-comment`).

| role | status |
|---|---|
| `cmd__predict()` — the one read-only modeling member; wrapper-ness detected, never declared | TYPED (`271:rul-predict-absorbs-wrapper-modeling`) |
| `cmd__is_converged()` — the canonical verdict function | in-use (`271` worked minimum; `24M`) |
| `cmd__disturbs()` — né `touches()`; no `only` (at-most per matched invocation-shape) | TYPED (`271:rul-touches-becomes-disturbs`, `rul-at-most-family-names`) |
| `cmd__lend_map()` — the wrapper's dimension member; enumerate-every-dimension law | TYPED (`271:rul-lend-map`) |
| `kind__resolve()` — kind-owner resolver | name in-use (`24M`, `272` §1); **menu unratified** (`271:rul-at-most-family-names`, "predict/resolve menus … unruled remainder") |
| `kind__disturbance_reaches_only()` | TYPED (`271:rul-at-most-family-names`) |
| `kind__state_stored_only_in()` — earns `only` most (consumer reads its negative space) | TYPED (`271:rul-at-most-family-names`) |

The `only`-in-a-name convention: `only` = complete-by-contract, totalistic-survey-before-authoring;
absence = arm-incremental (`271:rul-at-most-family-names`).

### The rc verdict partition — TYPED (`271:rul-rc-partition-stands` / `rul-zero-one-inversion-pair`)
`0` = named sense holds · `1` = complement · **`≥2` = flat sink** (meaningless / error /
warn). The verdict-bearing statuses are exactly the inversion-pair **{0, 1}** — the only
statuses that can ever carry a verdict, hence ever license a skip; `≥2` is never inverted
and can never license (stays semantically flat forever).

### The ρ-claim env-idiom ladder — TYPED (`271:rul-env-claim-inversion`; `274` §2)
Every rung a runnable sh idiom; silence = floor (ignorance mints ⊤; every believable claim
is a typed pointable line):

- bare `"$@"` = claims NOTHING (⊤; never "claims-isolation" — derived separation is barred)
- `VAR=x "$@"` = per-variable claim, rest ⊤
- `env "$@"` = full ambient passthrough (the `env` syllable IS the positive claim)
- `env -i VAR=x … "$@"` = exactly-these

---

## §3 — Strip semantics / the off-ramp

`dorc strip` erases the authored additions to reach floor-legal portable sh:

- **binds + marks** erased whole-statement, leaving the underlying command as the
  last status-affecting statement (`KNOBS:kTYANNOT`).
- **`dorc:` prefix-erasure** — `dorc:sh` → bare `sh` (`271:rul-dorc-prefix-head-synthesis`;
  `274` §1).
- **the shebang-runner rewrite** — the only surviving name-touching rewrite.
- **NO in-body name rewriting** — because names are already bare munged POSIX NAMEs, the
  `rul24-totalistic-munge` carve shrinks to prefix-erasure + the shebang rewrite
  (`271:rul-dorc-prefix-head-synthesis`; `24M:rul24M-bare-dorcism-names`).
- **row-three dangle**: `dorc-sh` typed directly is **untouched** by strip
  (`271:rul-row-three-documented-dangle`; fails loud-127 post-uninstall by the author's
  documented buy-in).

**The executable off-ramp test**: strip-then-run-under-both-binaries — the stripped file
parsing and running identically under `posh 0.14.1` and `dash 0.5.12` IS F-OFFRAMP as a
command (`276:rul-spec-two-binary-floor`).

---

## §4 — The stability ledger (`276:rul-verdicts-never-stable`, typed emphatic)

- **syntax = marker-gated** — the `# dorc-lang/vN` marker keeps the syntax parsable and
  agile through redesign.
- **`__role` names = permanent** — the unversionable compat surface (`24M`).
- **verdicts = unstable-and-improving, disowned** — Dorc does NOT promise cross-version
  verdict stability for `dorc plan`; the core mode explicitly gets better without notice.

Named consequences (both "bank both, we'll roll with them"):

- **plan-as-API** — the named failure-mode: treating plan output as a cross-version-stable
  interface (canonical nightmare: admins CI-gating on plan shape).
- **verdict-pinning** — the named, **disowned** someday-feature ("someday we may offer
  `dorc <scaryfeature>`; we don't now"); human sizing on record: a Hard Problem "on the
  order of ~all of the rest of Dorc combined."
- Rider: `dorc plan --exit-code` inherits verdict-churn — its contract must gate
  **divergence-of-world**, never plan shape (obligation travels with whoever builds it).

---

## §5 — Explicitly outside the dialect

- **The banned bash-family constructs** (verbatim, `276:rul-base-dialect-ruling-list`):
  `${x/…}` `${x^^}` `${x:off:len}` `[[ ]]` `==` `<<<` `&>` `|&`.
- **emit-nevers**: `test -a` / `-o` — accept-run / emit-never (`276:rul-base-dialect-ruling-list`).
- **`$'…'`** — OUT-for-now (`276:rul-base-dialect-ruling-list`).
- **`function f{}`** — rejected; `f()` only (`276:rul-base-dialect-ruling-list`).
- **`echo` with flags/escapes** — never; printf-doctrine (`276:rul-base-dialect-ruling-list`).
- **authored `eval`** — never an authored spelling; delegation in an eval'er predict body is
  an ACTUAL COMMAND, never `eval`; `eval` may reappear only as engine-lowering vocabulary
  (`271:rul-evaler-delegation-actual-command`, "You've killed eval, fairly").
- **Scope carve**: this reference binds **ORACLE / marked dialect text only**;
  book-acceptance is a separate open question (a tabled value-ladder, not a parse-bit)
  (`276:rul-kwhichsh-oracle-scoped`).

---

## §6 — Where the grammar is unsettled (owned by the entity-algebra note)

Not invented or guessed here. Each defers to **the entity-algebra design note
(`notes/277`, authored 2026-07-12 — its §4 carries the exact-grammar proposals,
awaiting the human's delta pass)**:

- **entity / selector charsets** (per-position) — `277`.
- **brace-alternation multi-cell marks** (`: sm.dorc.Service#{enabled,active}`, direction
  only per `271:rul-emission-selector-on-mark`) — exact grammar owed to `277`.
- **axis-tokens in third position** (the ingredient-borne invariance mark spelling; the ONE
  deliberate kOOB reading) — `277`.
- **quoting rules** where selector/entity charsets collide (`271:rul-selector-introducer-hash`
  notes "quoting supported", grammar unspecified) — `277`.
