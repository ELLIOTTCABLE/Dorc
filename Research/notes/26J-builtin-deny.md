# 26J — the state-mutating-builtin deny-list

Closes `26I:fnd-state-builtins-silently-mis-key`. Posture is DENY, not model: a state-mutating
builtin at a plain-command head inside an oracle role body degrades the trace to ⊤, loudly, with a
named reason. Denial loses value only, never safety — the site runs, which is what it did before
any oracle existed.

Scope: the ORACLE lane's two tracers (`predict/eval.rs`, `verdict.rs`). The book lane is a
separate seat and is adjudicated at the end (`§book-lane`).

## The axis, and why it is not the existing one

`analysis::effect::is_target_state_pure_builtin` already lists `set cd export unset shift read
readonly local : true false echo printf test [` — and that list is CORRECT and must not change.
It answers **does this command mutate the managed remote-host state?** `set -- alpha` does not; it
is target-state-pure, and classifying it `Pure` so it does not poison downstream ambient-ness is
right.

This deny-list answers a different question: **does executing this head diverge the TRACER's own
model from the bytes the tracer is about to ship?** The predict/verdict tracers carry
`positionals` and `vars` and walk a statement sequence they assume the host will walk too. A head
that rebinds those, or that makes the walked sequence not the executed one, produces a *wrong*
`Resolved` — the disaster class, not a degrade.

Two orthogonal lists, two names, one cross-reference each. `effect.rs`'s "share this ONE list,
never a parallel notion of 'inert command'" doc-comment binds the *inertness* axis; this is not
that axis, and a head can be honestly on both lists (`set` is: target-state-pure AND
tracer-diverging).

## The list

### deny-now — tracer-state divergence (produces a WRONG `Resolved`)

| head | mechanism |
|---|---|
| `set` | `set -- alpha` rewrites the positionals wholesale. `26I`'s demonstrated case. |
| `unset` | unbinds a var the tracer resolved; host probes the empty string. Demonstrated. |
| `eval` | arbitrary rebinding. Also discharges the sub-finding that the dialect's authored-`eval`-never law was unenforced at lift while `dorc lint` flagged it — two parsers, opposite verdicts, and the licensing one was the permissive one. Demonstrated. |
| `export` | the `NAME=v` form binds a var the tracer never sees; a later use resolves stale. |
| `readonly` | as `export`. |
| `local` | as `export` (not POSIX, universal in dash/bash — an oracle body will meet it). |
| `read` | rebinds from stdin. `26I` reasoned this "safe by degrade"; that holds only for a NEVER-bound var. `pkg : X = "$1"; read pkg; probe "$pkg"` is the same wrong-coordinate shape. |
| `getopts` | consumes args and writes `OPTIND`/`OPTARG`/its named var — the positional-mutation class by definition. |
| `.` / `source` | sources arbitrary code that can rebind anything. |
| `exec` | replaces the shell image; every later statement is dead, but the tracer walks on and records their spans as probe bodies. |
| `exit` | as `exec`, for reachability. |
| `break` / `continue` | the tracer's `run_while` re-evaluates its test and keeps iterating; sh's `break` leaves the loop. A flag-strip loop then over-`shift`s in the model relative to the host ⇒ wrong entity. `continue` diverges the same way by skipping the rest of the body the tracer still walks. |
| `shift` (COMMAND head only) | keyword-`shift` is modeled and untouched (below). This arm is reachable ONLY by the quoted `'shift'` form, which `at_keyword` declines to recognize and sh still runs — `26I`'s named pathological door, closed at zero cost. |

### deny-now — execution-environment divergence (coordinate right, claim weakened)

The tracer's claim is "these shipped bytes measure this cell". These heads keep the coordinate but
undermine the claim. All have zero corpus use, so the value loss is exactly zero today.

| head | mechanism |
|---|---|
| `cd` | later relative paths in the shipped body resolve against a different cwd than the analyzer assumed. |
| `trap` | installs a handler running arbitrary code at exit/signal. Walled book-side already (`trap_at_tip_walls_and_is_never_silently_pure`); unwalled oracle-side until now. |
| `umask` | changes creation modes for anything the body touches. |
| `ulimit` | can fail a probe for a reason unrelated to world state. |
| `alias` / `unalias` | change command resolution for every later statement (dash expands aliases in sourced files, and an oracle IS sourced). |
| `hash` | changes the PATH lookup cache the later statements resolve through. |

### already-modeled — leave alone

| head | why |
|---|---|
| `shift` (keyword) | a first-class `Stmt::Shift`; `run_shift` applies it to the positionals. 460 in-role corpus uses. Denying it would delete every flag-strip loop in the corpus. |
| `return` | modeled decline — `TopReason::Declined` / `Decline::Return`, and `is_return_head` already matches BOTH quoting forms. |
| `:` | **the mark carrier**, not an inert no-op. 26 in-role uses carrying `state_stored_only_in`, `lend_map`, `safe-across`. Denying it deletes those subsystems. Genuinely inert as sh. |
| `true` / `false` | inert fixed-rc; `Decline::Inert` verdict-side, `is_rc_forging_head` gate-side. |
| `command` | the oracle-contract's own existence gate (`command -v tool >/dev/null 2>&1 || return 2`) and it models fine — `26G:§CORRECTION-orlist-not-command-v` established the or-list, not `command -v`, was the culprit. Denying it would re-break the contract's taught idiom. |
| `test` / `[` / `printf` / `echo` | modeled. |

### genuinely-inert — deliberately NOT denied

`wait` · `times` · `jobs` · `pwd` · `type` · `getconf`. None rebinds a tracer-modeled var or
positional, none changes command resolution, none alters an environment a later modeled statement
depends on. Listed so the omission reads as a decision rather than an oversight: the deny-list is
"heads that diverge the tracer", not "all builtins", and padding it with inert heads would be
breadth with no mechanism behind it.

`newgrp` is not on any list: dash has no such builtin — it is an external binary, outside a
builtin deny-list's remit.

## Where the deny hooks

Both seats are at the *recognition edge* — the plain-command arm each tracer already funnels
through — never inside the kernel. Both copy the shape the `Pipeline` / `OrList` / `AndList`
degrades already use: parse-permissively (the body still lifts and `strip` still erases its
bytes), trace-conservatively (⊤ here).

- `predict/eval.rs` `run_stmt`, the `Stmt::Command` arm, checked BEFORE the probe span is
  recorded — beside the existing `cmd.pipeline` and `is_return_head` guards. Yields
  `TopReason::StateMutatingBuiltin`, which rides the existing `site-unresolvable` reason channel
  (`effect.rs` `degrade` → `unresolvable_causes` → the PASSTHROUGH `detail`) with no new plumbing.
- `verdict.rs` `run_command`, before `decline_idiom`, so a denied head can never set
  `reached_command`. Yields `VerdictTop::StateMutatingBuiltin` ⇒ never `Vouched`.

The other oracle lanes need no change and were checked: `touches.rs` / `reaches.rs` accept only a
`printf` head and escalate everything else (allow-list, already closed); `carry.rs`'s
`PURE_BUILTINS` is likewise an allow-list, and every denied head already rejects there as
`UnmarkedExternalCommand`.

## What verification established

`26I`'s finding reproduced independently before the fix (the note's own
`haz-trial-claims-need-independent-check` discharged): with the hook disabled, `evaluate` over its
evidence body and argv `["install"]` returns `Resolved { kind: "sm.dorc.X", entity:
Operand("install") }` — and its `probe_body` carries the `set -- alpha` span itself, so the
mis-keying statement was being SHIPPED into the probe. Whole-product, the same case renders a
probe that reports `site 0 effect=holds`: a converged holds filed against a cell the shipped body
never measured. `+SURE`, the finding is exactly as written.

After: the site is `unresolvable-no-probe`, nothing ships, nothing keys, and the apply runs the
command. Both are pinned — `the_evidence_case_can_no_longer_ship_a_wrong_coordinate` at the unit
tier and `deny26-state-builtin-mis-keys` at the whole-product tier.

Resolutions only DISAPPEAR: the deny is a pure ⊤-grow at a single arm, reachable only by a head on
the table, and the corpus has no in-role occurrence of any of them. Both gate legs moved zero
goldens.

## needs-human residue

- `authoring-surface-stays-silent` — `dorc lint` still reports 0 errors / 0 warnings on an oracle
  whose predict body the deny has made entirely dead. The book-side `site-unresolvable` note now
  carries the named cause, but the author gets nothing at their own file, which is
  `26G:haz-silence-is-the-common-cause` still firing. An `oracle-state-mutating-builtin` lint at
  the validate seat would fix it AND would be argv-independent (it would catch a head on a case
  arm this argv never selects, which the trace-time ⊤ structurally cannot). Deliberately NOT built
  here, because it needs a ruling the deny itself did not: **which roles it should scan**. The
  deny binds the two TRACED roles (predict, verdict); `__resolve` is host-run strip-only and its
  body legitimately carries constructs neither tracer models, so a lint that scans role bodies
  textually would over-fire on exactly the role the and-or work already carved out.
- `verdict-degrades-have-no-channel` — a verdict-lane ⊤ reaches no surface at all. Both consumers
  (`plan/src/lib.rs:1399`, `:1500`) test `!matches!(…, Vouched)` and discard the reason;
  `classify_decline` narrates a genuine decline but a ⊤ falls past it. Pre-existing and general —
  it swallows `AndOrList`, `BudgetExceeded`, and `NonConcreteWord` identically — so this lane
  neither caused nor worsened it, but the new variant inherits the silence. Building the
  verdict-side twin of `unresolvable_causes` is its own errand.
- `book-side-set-subform-deny` — see `§book-lane`. A `set` whose first operand is not an option
  (`set --`, `set alpha`) is the positional-rebinding form; `set -e` / `set -o pipefail` must keep
  modeling. Whether the book dialect refuses the former is a scope call.
- `inert-column-is-relative-not-absolute` — the table is closed against the state a tracer carries
  TODAY. If the tracers ever model cwd, IFS, or the function table, the inert column must be
  re-adjudicated: those heads are inert *relative to what is modeled*, not absolutely.

## §book-lane — adjudicated, and mostly already closed

The book lane is NOT the same hole, and needs no deny of its own for the lvalue family:
`analysis::value::transfer_lvalue_builtin` already models `unset` · `read` · `export` ·
`readonly` · `local` · `getopts`, clobbering the named vars and havoc-ing all of them on a dynamic
or unexpected-flag operand. `syntax/src/parser.rs` already refuses `eval` outright
(`UnsupportedReason::DynamicExecution`) and refuses `.`/`source`/`unset`/`printf -v`/`test -v` in
their dynamic sub-forms. `cfg.rs` walls `trap` and terminates paths at `exit`/`return`.

The one book-side gap found: `set` is modeled ONLY for its errexit toggle (`errexit_toggle`), so
`set --` book-side rebinds positionals nothing models. Its reachability is narrower than the
oracle-side twin — it needs a `set --` inside a spliced funcdef body whose `$N` overlay
(`positional_argv`) the rebind invalidates. **needs-human**, and deliberately not fixed here:
a book-side `set` deny cannot be wholesale (`set -eu` opens most books in the corpus and its
errexit modeling is load-bearing), so it would have to be a sub-form deny in the shape
`syntax/parser.rs` uses for `unset`/`printf` — which is a book-dialect scope decision, not the
oracle-lane side-lane this note closes.

