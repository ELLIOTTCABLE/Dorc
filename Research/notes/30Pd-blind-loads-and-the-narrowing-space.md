# 30Pd — Blind loads: the floor, the narrowing space, and the spellings explored

> Tier: conductor exploration record (Fable, the 2026-08-22 rubber-duck sitting with the
> human; sibling of `30P`). Nothing here is a ruling unless marked; the welded law it rests on
> is `30P:law-no-unsoundness-below-a-blind-act`. Shell-behaviour claims are marked for the
> floor evaluators (`mise run test:floor` manifests under `posh` ∩ `dash`) and must be
> measured before any hint leans on them. Stated softly on purpose: this is a map of paths,
> not an endorsement of any. Grades +SURE / ~SUSPECT / -GUESS.

## §1 — The floor (agreed in the sitting)

A **blind act** is a line whose effect on the shell Dorc cannot see: a `.` of a file the
controller does not hold, an `eval` of ⊤, a call into a body Dorc cannot splice. It does two
separate kinds of damage, with different cures:

- `damage-shell` — the act ran inside this shell. Below it: cwd, functions, aliases, options,
  positionals, variables, and whether the next line is reached are all ⊤. On alias-expanding
  shells the alias table rewrites how later lines are *read*. Consequence: no cwd-dependent
  decision below (`[ -f ./x ]` must not decide; a relative `.` is not EXACT; slashless `$0` is ⊤).
- `damage-world` — the act ran commands. Below it every world cell may be stale: a total wall
  (no footprint ⇒ no survival). Consequence: no elision below; guards only.

What survives, per the human's consistency doctrine: **guards**. A guard is authored sh
re-measured live; Dorc pins the author's bytes and guarantees only that its own movement and
renaming create no binding ordinary sh would not (`30P:rul-guard-resolves-like-its-mutation`).
A site word meaning something else below the act is the author's footgun, as with a hand-written
guard. Oracle files are immune by contract (no top-level commands); only books can do this.

No engine-side recovery exists, and none is sought (`30P:law-no-unsoundness-below-a-blind-act`).
Recovery is author-spelled, and the sitting's finding is that the spellings are plain defensive
sh the author would want anyway — the off-ramp keeps the safety, Dorc keeps the analysis.

## §2 — Axes, and the narrowed requirement

- `axis-dimension` — which part of the model is ⊤: contents-at-path · cwd · alias table · a
  named function · a variable · options · positionals · `$0` · a world cell · reachability.
- `axis-consumer` — which dimension a later line reads. A relative `.` reads cwd + contents;
  `[ -f ./x ]` reads cwd; a `${0%/*}`-relative `.` reads `$0` only; a guard reads nothing;
  elision reads world cells.
- `axis-source-of-truth` — a narrowing constant comes only from authored text; never host bytes
  (`rul-host-bytes-bounded-before-admission`), never probe output (the capture lane moves values,
  never shell text).
- `axis-path` — the fact must hold on every path reaching the consumer; paths may die, diverge,
  loop-until, or carry their own narrowing.
- `axis-window` — no ⊤-making act of that dimension between narrowing and consumer.
- `axis-speaker` — the admin, in sh; the tool's spec, via its oracle at parity. Nobody else.

**`req-narrowing`** — for a consumer reading dimension D at line L: on every path to L, D was
last set to a controller-known value by an act whose semantics Dorc holds exactly. The act need
not be a check, and the failure path need not die. The `||`'s right operand names the licence the
admin is writing: `|| cmd` is a guard, `|| exit` is an assertion, `|| fallback` is a branch.

## §3 — The mechanism matrix

Each row is plain sh; each names what Dorc must hold exactly to honour it.

- **`mech-never-top`** — spellings the blind act cannot touch. POSIX lookup is
  special-builtins → functions → regular builtins → `PATH`; aliases act at parse time on unquoted
  command words only; `$0` is immutable for the shell's life; assignment words are not command
  words; parameter expansion is not a command.
  ```sh
  . /vendor/blind.sh
  . "${0%/*}/oracles/x.sh"          # pure expansion on an immutable $0: EXACT under absolute
                                    #   anchoring, whatever blind.sh did
  . "$(dirname "$0")/oracles/x.sh"  # not immune: dirname may now be a function, alias, or PATH hit
  ```
- **`mech-contain`** — D never becomes ⊤: `( . blind )`, `x=$( . blind; printf %s "$x" )`,
  and `sh /vendor/blind.sh` (execute, don't source — a child cannot touch this shell). Not a
  container: a pipeline element (POSIX leaves the last element's environment unspecified ⇒ ⊤ by
  `rul-unsure-falls-toward-sh-parity`). Containment cures `damage-shell` only; the wall remains.
- **`mech-reset`** — the decontamination prologue. Sound because quoting defeats aliases and
  special builtins defeat functions; the ORDER is load-bearing:
  ```sh
  . /vendor/blind.sh
  \unset -f unalias cd command pwd    # special builtin, quoted: unhijackable
  \unalias -a                         # now the real builtin: alias table := ∅
  \set --                             # positionals := ∅
  PATH=/usr/sbin:/usr/bin:/sbin:/bin  # assignment word: immune
  cd "${0%/*}" || exit 3              # only now — cd is a REGULAR builtin, function-shadowable above
  ```
  Functions stay ⊤ except those unset by name. Cures `damage-shell` dimension by dimension.
- **`mech-establish`** — D set by a known write: `cat >/etc/app/env <<'EOF' … EOF` or
  `cp ./app.env /etc/app/env`, then `. /etc/app/env`. Contents := book bytes. The write walls
  what is above it (correctly), nothing below.
- **`mech-assert`** — D narrowed by an exact check on every reaching path:
  `cmp -s P ./expect || exit` (contents = authored) · `grep -vFxf ./allowed P && exit`
  (contents ⊆ authored lines) · `[ "$X" = lit ] || exit` (a ⊤ variable becomes a constant —
  the TypeScript-narrowing cell, and the one that composes with read-tools: `X=$(sed …)` gives ⊤,
  the test narrows it). Inside the main shell after a blind act, `[` is trustworthy only after
  `mech-reset` or inside a `$( )` container.
- **`mech-assert-world`** [UNREVIEWED — crosscheck before anyone builds toward it] — re-asserting
  a world cell with a dead failure path licenses elision of later sites on that cell *below a
  wall*: `cmp -s /etc/nginx/nginx.conf ./nginx.conf || exit 4` then `systemctl reload nginx`.
  The check is in-sequence (no staleness), the failure path is dead, the vouch is the admin's own
  stage-1-style guard with `exit` in place of the command. Reads as sound; it is a new licence
  species ("assertion"), hence the flag.
- **`mech-read`** — don't execute it: `ID=$(sed -n 's/^ID=//p' /etc/os-release)`. No havoc,
  no wall, value ⊤ until the capture lane. Values in `os-release` are often double-quoted; the
  author owns the parse. Dorc ships no helper (human-ruled: coverage at parity, never provision).

What no row provides: un-walling the world without re-asserting a cell. Once unknown bytes ran,
the ways down are per-cell (establish or assert), not executing, or the engineer's at-most claim
under the admin's flag. No spelling says "and nothing else happened".

## §4 — Shell-behaviour claims owed to the floor evaluators before any hint leans on them

- `claim-quoting-defeats-alias` — `\unalias -a` is never alias-substituted. +SURE (POSIX
  2.3.1 "unquoted"); measure on both binaries.
- `claim-special-builtins-beat-functions` — a function named `unset`/`set`/`exit`/`.` cannot
  shadow the special builtin. +SURE (POSIX 2.9.1.1 lookup order); measure.
- `claim-cd-is-shadowable` — `cd`, `unalias`, `command`, `pwd`, `[` are regular builtins and a
  function of that name wins. +SURE by spec; measure.
- `claim-dash-expands-alias-on-funcdef-name` — `alias foo=bar; foo() { :; }` defines `bar` on
  dash (the parser substitutes before it sees `(`). ~SUSPECT in detail; +SURE it is not
  guaranteed, which is all the floor needs. Measure on both; a disagreement is itself a finding.
- `claim-keyword-beats-alias-on-dash` — `alias if=…` does not affect `if` in command position.
  ~SUSPECT (from dash's `readtoken` ordering); measure.
- `claim-dollar-zero-immutable` — no builtin changes `$0`; only `exec` replaces the shell. +SURE.
- `claim-pipeline-element-environment-unspecified` — the last element may run in the current
  shell. +SURE (POSIX 2.9.2); therefore ⊤, and not worth measuring (the floor pair agreeing
  would not make the spec promise it).
- `claim-subshell-copy-is-total` — `( )` and `$( )` leak no shell state back. +SURE.

## §5 — The lifted target, pencilled (human, 2026-08-22): `narrow-by-test`

Per-path refinement of a ⊤ value by `test`/`[`/`case` literals, with `|| exit` making the
refinement unconditional below — the capture path of `forfeit-divergence-collapse-to-unknown`,
which the human typed as near/mid-term. Idiom: the README's own thesis. Generality: every ⊤
value from every source; three consumers at once (the load plane — `$LIB`-headed loads become
EXACT by one authored line; oracle argparse — ⊤-argv sites bind an entity; the fold). Size L,
licence-review-tier, serial after the wave-one lanes and before influence carriage. It does
NOT pick which `case` arm runs (that is the probe-sourced value, the capture lane); its corpus
win is mostly assertions. Register: `FORFEITS:forfeit-value-narrowing-by-test`; the scheduled
attention-call is the `r31:kernel-punt-glance` horizon beside the other kernel punts.

Pins (the FORFEITS reds) are NAMED here and UNMINTED — no build was permitted this sitting
(machine resource-constrained); a builder mints them through `internal_tooling::xfail::PINS`
with horizon `Horizon::Scheduled("r31:kernel-punt-glance")` (a new marker: the attention-call
beside the other kernel punts, never a do-it-then date) and one red test each at the shapes
the FORFEITS examples sketch, then `mise run both gate:full-quiet`. Until then the census does
not know them and `rul-forfeits-carry-reds` is owed, not met.

Riders the running load lane can absorb cheaply, pure parity: `${0%/*}` is exact below any
blind act while `$(dirname "$0")` is not; `sh file` contains and `. file` does not; `( )`/`$( )`
contain and a pipeline element does not.

## §6 — Reusing the oracles as designed against the world wall

- `reuse-verdict-as-assertion` — the stripped verdict function is plain sh in the book's
  namespace; `docker__is_converged run … || exit 5` is the engineer's check under the admin's
  dead path, and licenses elision of later sites on that cell below the wall. Fall-to-guard with
  the other fallback. Same review flag as `mech-assert-world`.
- `reuse-disturbs-for-the-blind-act` — **NACKED [human, 2026-08-22]**, together with every
  spelling explored for it: an in-book `dot__disturbs_nothing()` role (the ∅ claim riding a
  name), wrap-and-describe (`load_x() { … }` + `load_x__disturbs_nothing()`), and
  self-describing coordinate lines on stdout. The human's reading: the effort itself proves the
  approach dead — "unmodeled → walls → guards, no meaningful escape; we can only push the author
  towards defensiveness and enabling modeling." Recorded so nobody re-derives it: the family
  name for `.`, the ∅-claim carrier, and the book-function-as-family mechanism are all NOT
  wanted, at least for now.
- `attention-by-dimming` — render-plane only: below a wall, guarded-and-currently-converged
  lines may be dimmed ("at most dimmed, warily" is inside `rul-attention-honesty`), so a region of
  guards reads as one attention unit. Never hides a line that may run.
- Dead, named so nobody tries: an engine-synthesized `( v ) || exit` (`rul-ternary-verdict` —
  the author's bytes always survive); hoisting oracle loads above the blind act (`kBACKFLIPS`);
  any shell-model claim from an oracle (the sin-class argument: a wrong one mis-attributes every
  line below to the wrong author — the top of `271:rul-sin-ordering`).

## §7 — FORFEITS minted from this record

`forfeit-value-narrowing-by-test` · `forfeit-file-content-facts-from-exact-checks` ·
`forfeit-content-establishment-by-known-write` · `forfeit-shell-parity-immunity-model`. The
unreviewed licence species (`mech-assert-world`, `reuse-verdict-as-assertion`) and the
`.`-family spelling are deliberately NOT rows: neither has been declined, and a row records a
declined option.
