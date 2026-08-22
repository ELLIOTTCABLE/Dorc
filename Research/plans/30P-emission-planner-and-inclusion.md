# 30P — Emission, placement, and inclusion: the planner, the stream forms, and how book code is loaded

> Tier: LLM-authored plan (Fable conductor, from the 2026-08-22 design sitting with the
> human; session `r30-conductor-3`). Subordinate to root docs, `spike/CLAUDE.md`, and
> `KNOBS.md` (`kBACKFLIPS` is the registry entry this plan details). Grades: **[TYPED]**
> human typed it · **[ACKED]** substance confirmed in dialogue · **[PROPOSED]** conductor-derived,
> awaiting ratification. Written in plainer language than most of the corpus, on the human's
> instruction; strawmen are illustrative, never spellings.
>
> What this plan owns: the one emission planner (`28Q:pin-emission-planner-universal`), the
> plan's emission FORMS (single-stream versus multipart) and what may be done to a book to
> reach one, and the principles under which a book's `.` lines are loaded — the three are one
> topic, because every one of them answers "where do these bytes go in the plan, and what
> may we change to put them there". It supersedes nothing: `30I` stays the loading/bundling
> design, `30Ng` the stream/form rulings record, `30L` the elision-region design; this plan
> is the emission-and-inclusion layer they all touch, assembled in one place for the first
> time. Sequencing and lane sizing live in `notes/30O`; this plan feeds it.

## the-design-in-one-screen

Dorc writes out an apply script. Oracle function definitions must sit in that file above
their first use, under a name, and without colliding with the book's own names. That
placement-and-naming question is asked in three places today (the preamble hoist, the
bundle inliner, the front-lift ladder), and the ruling is that it is ONE component — the
**emission planner** — answering it once, for probe and apply alike, with only taste
differing between the two.

The planner never rewrites a book. Its only moves are (a) copy bytes verbatim to another
place where copying is provably identical, and (b) rename an *oracle's* definition in the
shipped artifact. When neither is proven, it falls back to the most conservative form:
definitions stay where the author put them, under renamed names, and if the admin asked for
a form the book cannot take — single-stream, most often — Dorc refuses before touching the
network, names the line, and names the form that works. That posture is welded:
`KNOBS:kBACKFLIPS`.

Book `.` lines are what decide which definitions exist where, so the planner is only as good
as the load model. Three principles replace the idiom-by-idiom approach: an unknown source
is a point havoc, not a poison; an operand resolves only over what the controller already
knows, through shell semantics alone; and a plain-sh file that is sourced is *included* —
analyzed as if pasted, shipped beside its plan, and pasted into a single stream only under a
floor-measured identity set.

## the-emission-planner — one component, two modes, a closed vocabulary

`28Q:pin-emission-planner-universal` [human-ruled DIRECTION, 2026-08-16] asked for one
abstract planner over the sh truths (definition visibility · death at the paren · the
errexit exemption of an `||`-left), supporting placement × naming:

```text
placement:  top-lift   | adjacent (where the author put it) | colocated inside the paren that uses it
naming:     authored   | munged (`name_h<digest>`, header-only)
mode:       probe (verbose-tolerant, no book namespace)  |  apply (idiomatic-first, attention-priced)
```

Licence above it: `rul-happy-path-is-a-closed-set` — the idiomatic choice (lift, keep the
name) is taken only when the engine has PROVEN the set of names and constructs in play is
closed; otherwise munge-everything, adjacent, which is today's defensive emission.

- **`rul-planner-apply-side-first`** [ACKED 2026-08-22] — the planner is built apply-side
  now, with the probe-mode seam reserved (a policy toggle on the same component, not a
  second planner). The probe mode's first consumer — per-segment environments for a
  composed pipe whose two participants carry same-named, differing-bytes helpers
  (`p-x-intra-compound-plurality`) — waits; it adds one placement value ("inside this pipe
  stage's subshell") to the same vocabulary, never a new mechanism.
- **`rul-front-lift-is-the-planners-first-consumer`** [ACKED 2026-08-22; reverses the
  conductor's earlier lean] — `30Ng:bundle-front-lift-ladder` IS the planner's placement
  question asked of bundles: lift-as-is = top-lift + authored name; lift-and-munge = top-lift
  + munged name; positional-with-rewrite = adjacent; decline = the refused set. Building the
  ladder as its own mechanism would have been a third placement engine — the debt
  `28Q`'s no-more-piecemeal order exists to prevent.
- The second consumer is `p-x-placement-tuning-pair` (top-lift the many-use helper;
  colocate the once-used collider inside its paren): it needs only the third placement
  value.
- Alpha-rename (renaming CALLS inside oracle bodies, not just the header) stays reserved
  (`d-alpha-rename-equivalence`); every tier above needs only the header-only rename that
  exists (`28R:rul-munge-oracle-names-only`).
- Planner decisions are settled from authored-before-contact inputs and enter `Plan::decided`
  like `ImportEdit` does; they are never re-derived at render time and never fed back into
  analysis (`the-render-decides-nothing`; the `30Ng:attn-render-refusal-feeds-the-spine`
  sitting is NOT a prerequisite — a lift under a proven closed set changes no resolution).

## the-stream-forms — verbatim-or-refuse, and the uneven floor

**`KNOBS:kBACKFLIPS`** [WELDED, human-typed 2026-08-22] — Dorc never performs
source-to-source transformation to make an emission form possible. The enumerated edit
classes, the whole list; a fourth is a human act:

1. **Import re-say in GENERATED plans** (`30Ng:rul-bundle-at-dorc-lang-boundaries`) — the one
   rewrite of book bytes that exists, and only because a generated plan is a durable but not
   an off-ramp durable.
2. **Header-only renames of ORACLE-custody definitions** in shipped artifacts
   (`28R:rul-munge-oracle-names-only`). Book bytes, never.
3. **Byte-verbatim relocation under a floor-measured identity set** — paste in place at the
   `floor30-inline-dot-boundary` shape (a `.` that is the whole of its line, top-level,
   redirect-free); the front-lift ladder; the plain-sh paste set this plan owes (below).

Refused by name, permanently: generated loader functions (floor-REFUTED —
`30I:pin-one-file-root-bundle`, the `floor30-dot-loader-function-errexit` manifest: inside a
function, `set -e` and `set --` behave differently than under `.`); any rewrite of a book's
control flow, positional parameters, `return`/`exit`, or scoping; synthesized dispatch.

**`rul-floor-is-uneven-across-forms`** [TYPED 2026-08-22] — not every book admits every
emission form. Multipart output (the plan plus its dependencies as files) accepts strictly
more book code than single-stream. The MOST a construct may cost its author is
single-stream — never support. Test-then-source (`[ -r ./local.sh ] && . ./local.sh`) and
`return`-guarded library files are the motivating shapes: both are ordinary, both must work,
and both may honestly make a book unpipeable.

Product-surface consequences, stated so they are discovered here and not in the field:

- Under `30Ng:rul-piped-stdout-carries-a-full-plan`, a piped `dorc plan` on such a book
  REFUSES pre-network, naming the line and `--artifact-dir`. The refusal text is human
  prose (the prose queue); the behaviour is this plan's.
- Single-stream is also the transport floor for hosts with no writable directory
  (`KNOBS:kBOOT`), so the same books cannot reach those hosts. Same refusal, same honesty.
- Single-stream review is a core product tension, not a bug (`30Ng` "the single-stream
  review surface"): the admin who pipes is asking to see the oracle ocean in-line; Dorc
  lifts it to the front, munged as needed, and puts the mutative book last after a divider.

## the-load-principles — three rules, no idioms

The sibling's 2026-08-22 xfails named four idioms (`$(dirname "$0")`, a source glob, an
existence-guarded file, `/etc/os-release`). Idioms are an open set; blessing one invites the
next. The cut is on axes [ACKED 2026-08-22, the three-principles target]:

**`principle-unknown-source-is-a-point-havoc`** — an unresolvable `.` means "anything may
have been defined here", which is sh's own meaning. Every function binding becomes unknown
AT THAT LINE; a later unconditional definition in the same frame re-binds by last-wins.
Nothing else changes: the unknown `.` stays a definition vector for the planner (nothing
lifts across it; defensive renaming stays on) and a wall for execution. Three consumers,
three answers — binding precision, placement, execution; conflating them is how a precision
gain becomes running someone else's body. Covers `/etc/os-release`, `"$HOME/.profile"`,
`"$(find_config)"` identically, with no name list and no file parsing.

```sh
. /etc/os-release            # unknown to Dorc: every name is now "maybe" …
hork__is_converged() { … }   # … but THIS is the last definition of this name: live
hork tune web                # licensable
```

**`principle-load-operands-evaluate-over-controller-known-inputs`** — a `.` operand
resolves iff its value is a pure function of things Dorc holds before any host contact: the
program text, the book's own path (`$0`), the modeled cwd, and the authored snapshot — through
a CLOSED allowlist of pure SHELL operations (parameter expansion: `${0%/*}`, `${x:-d}` with
`x` known; later `cd`/`pwd` once cwd-state is modeled; globs as snapshot expansion). The
precedent is `dec-decidable-set-v0`: closed, growing by name only, each widening
license-review-tier. Excluded by the axis, not by name: anything reading the target's
filesystem (`readlink -f`), environment (`$HOME`, `$XDG_*`), or a command's output.

- **`rul-no-tool-modelling-in-the-load-plane`** [TYPED 2026-08-22] — evaluating a COMMAND's
  output inside the engine (`$(dirname "$0")`, `$(cd … && pwd)`) is tool-modelling, which
  `identity-declared-never-inferred` forbids; an "allowlist of pure coreutils" is a small
  door into the room the oracle contract welded shut. Withdrawn. `${0%/*}` is parameter
  expansion — shell semantics, which `rul-unsure-falls-toward-sh-parity` says we model — and
  is the r30 slice. Honest rider: `${0%/*}` is the rarer spelling in the wild and has its own
  gotcha (`${0%/*}` of a bare `script.sh` is `script.sh`), so the hint asks people to edit
  their books; that is gradual enhancement working, and the edit is plain POSIX.
- **`ask-dollar-zero-command-substitution-path`** [OPEN, human's] — the one idiom-recognition
  the human has said "might be in the running, in specific shapes" (`$(dirname "$0")` and
  `$(cd "$(dirname "$0")" && pwd)` as fixed byte-shapes meaning "the book's directory", valid
  only as a `.` operand, guarded by the frame model's answer that no function named `dirname`
  may be bound there). Designable, count-of-two, priced as a small-allowlist exception sold
  for high value. Punted while a pure-sh alternative exists
  (`rul-small-allowlists-are-high-cost-minimal-count` [TYPED 2026-08-22]). Raised in urgency by
  `fnd-computed-dot-is-a-whole-book-refusal` (below).
- **`ask-authored-pure-predict-may-site-loads`** [OPEN, parked beyond r31] — the only
  principled middle ground that could ever admit `dirname`: a stdlib oracle whose `predict`
  states its stdout in sh, consumed through the capture lane (`seam-re-bind`, the r26
  revival), under a new vouch species ("this predict is a pure function of its argv") and a
  widening of `funcenv-reads-source-literal-plane-only` whose reason (no host involvement)
  survives but whose letter (program-text grade only) would change. A human ruling; never a
  lane's.
- **Probe-sourced loads: NACKED** [TYPED 2026-08-22; already permanent law] — a host's
  answer never sites a load.
- **Globs** are a SET-valued operand over the snapshot. The target's collation is
  unknowable, so a glob population is ORDER-UNKNOWN: two members defining one name with
  different bytes WITHHOLD (the existing plurality withhold); a name defined by one member
  is live. No-match sources the literal pattern, which fails — decidable from the snapshot,
  surfaced as a diagnostic. Never modelled by locale. Builds after `lane-loop-propagation`
  as its second population source (the loop lane mints the closed-member machinery).
- Every `$0`-relative import in a GENERATED plan is re-said (edit class 1): the real ssh
  executor pipes the plan on stdin, so on the host `$0` is the shell's name and
  `$(dirname "$0")` is `.`. This is not new cost; it is the grant that already exists.

**`principle-book-code-source-is-inclusion`** — a resolvable `.` of an ORDINARY (non-dorc-lang)
sh file is textual inclusion, which is what sh does: `.` opens no scope, keeps `$0`, keeps
the positional parameters (POSIX), and a top-level `return` in the file behaves like a
function's. Three tiers, one rule, no rewriting:

1. **Analysis** — splice the file at the `.` site under whatever branch it sits in (the
   call-splice precedent, `seam-interproc`: a sourced file is a body called once); its
   funcdefs are definitions (unconditional ⇒ live; under an undecidable guard ⇒ `May`); its
   commands are sites; its own `.` lines recurse under the splice budgets. Its lines keep
   their own line-space (`AID:law-lineno-identity`; the `30I` locator DAG already carries
   multi-file loci).
2. **Multipart emission** — the file ships beside its plan; an existence guard runs FOR REAL
   on the host against the shipped tree, so Dorc needs no decision about it — only "maybe"
   for the bindings. The file gets its own `<name>.plan.sh` when it has mutative sites worth
   editing (the two-book-files shape `30Ng:rul-bundle-at-dorc-lang-boundaries` already rules).
   The precision upgrade — deciding `[ -f ./optional.sh ]` TRUE because Dorc ships it, so
   verdict functions defined inside can license — is the four-seat agreement
   (acquisition · analysis · artifact closure · frame recovery) and comes after.
3. **Single-stream emission** — byte-verbatim paste under a closed, floor-measured exclusion
   set; otherwise refuse with the form named. The set, as currently understood (the atlas
   below proves or amends it): the `.` is whole-line/top-level/redirect-free (measured); the
   file has no top-level `return` (the one construct whose meaning bare paste changes —
   `return` inside a brace group or `if` at the file's top level included; `exit`, errexit,
   traps, `shift`/`set --`, nested `.` are paste-identical). A `return`-guarded library
   therefore makes a book unpipeable, by the uneven-floor ruling, with a hint: "use an `if`
   include-guard if you want single-stream plans".

Sizing [PROPOSED]: analysis M (splice precedent) + emission M (two-files plumbing, paste
under the set, one floor manifest). The conductor's earlier L conflated "no lowering exists"
with "no lowering can exist"; the refuted lowering was the function wrapper, and bare paste
under an exclusion set is the same verbatim-relocation class the bundle already uses. First
r31 kernel lane; an L lane if pulled into r30.

Still walls, by rule, and should: shape 7 (`. "$HOME/x"`) and shape 8 (`. "$(find_config)"`)
— the file is not in the snapshot, so nothing could be shipped for it.

## the-atlas — durable floor measurements before any mechanism

The project's floor discipline (`spike/CLAUDE.md` floor-differential-lane-opt-in ·
emitted-is-measure-once-ground-truth · `30A` d1) is the instrument: a sentinel-emitting
script, run under `posh 0.14.1` ∩ `dash 0.5.12`, agreement with each other and with the
committed bytes required, minted once through `bless:floor` (orchestrator, WSL). Eighteen
narrow manifests (`floor30-atlas-*`) are authored by the xfail lane's builder and minted by
the conductor before the inclusion design hardens: positional-parameter edits inside a
sourced file · `. f a b` operands · top-level `return` in a sourced file, in the main script,
inside `{ }`/`if`/`( )`, and when the `.` is inside a function · `exit` and errexit inside a
sourced file · `. missing-file` as a special-builtin failure with errexit off, and whether
`|| true` catches it · `$0`/`${0%/*}` inside a sourced file and under `sh script.sh` /
`./script.sh` / stdin · `local` at a sourced file's top level · a function defined inside a
function · `alias` in a sourced file then a later same-named funcdef (the one hazard this
plan deliberately does not split — measure it) · `unset -f`/`readonly`/`cd`/`trap` persistence
· glob order under two locales · the no-match literal · nested `.` against cwd.

Two harness gaps to close as tooling, not this plan's lanes: **`gap-atlas-mode`** (for
bash/zsh/busybox later, disagreement is data, not failure — a record-only mode over the
same harness) and **`gap-pinned-shell-matrix`** (versioned shells across distributions; the
containerized `livetest` kit is the natural home). Possibly a third, `gap-early-exit-capture`
(rc and partial output when the mechanic ends the script) — the builder reports.

## what-landed-today — the executable specification

Lane `ai/r30-lane-load-xfails`, folded at `6785fada`: seventeen live pins, census coherent.
The book-load pins, re-cut from four idioms to the three principles (bodies kept where they
were right):

| pin | horizon | target |
|---|---|---|
| `p-x-unknown-source-is-a-point-havoc` | end-of-r30 | later unconditional role definition is live below an unresolvable `.` (companions: the names-bound-only-before half is GREEN today; the opener-and-wall anti-regression is green) |
| `p-x-load-operand-param-expansion-of-dollar-zero` | end-of-r30 | `. "${0%/*}/helpers.dorc.sh"` acquires and binds |
| `p-x-load-operand-dirname-of-dollar-zero` | r31:book-load-acceptance | trigger names `ask-dollar-zero-command-substitution-path` |
| `p-x-load-operand-cd-pwd-of-dollar-zero` | r31:book-load-acceptance | same open ruling |
| `p-x-glob-load-acquires-members` · `…-members-are-order-unknown` · `…-no-match-aborts` | r31:book-load-acceptance | the set-valued operand, its withhold, its failure |
| `p-x-book-code-source-is-inclusion` | r31:book-load-acceptance | unconditional plain-sh `. ./helpers.sh` binds; the guarded cell holds `May` |

Two whole-product cases, XFAIL-marked (the e2e XFAIL form carries no horizon — a harness
limit, noted): `load30-point-havoc-and-script-relative` (expected to pass before the end of
r30) and `load31-punted-load-shapes` (the punted shapes in one book).

## findings-this-sitting

- **`fnd-computed-dot-is-a-whole-book-refusal`** — an inline `$(…)` in a `.` operand is a
  PARSE-tier refusal today (`syntax-unsupported`, exit 10): a book containing
  `. "$(dirname "$0")/helpers.sh"` is not walled from that line down, it is rejected
  outright. The current price of punting `dirname` is "Dorc will not analyze your book",
  which is a harsher cliff than the conductor had represented; the hint and the
  `ask-dollar-zero-command-substitution-path` item are correspondingly more urgent. Whether
  the refusal should degrade to an unresolvable-load wall (so the rest of the book still
  analyzes) is a one-line ruling worth taking early.
- **`fnd-squat-warning-contradicts-in-book-lift`** — `reserved-namespace-squat` still fires
  on a book-defined `hork__is_converged` and says Dorc "treats it as ordinary shell and
  runs it verbatim", which has been false since the r28 in-book lift
  (`cli/CLAUDE.md the-book-is-a-definition-source`; USER_STORY stage 3). Stale warning on
  the product's minimal rung: prose is the human's; the lint's firing condition wants
  re-scoping to "a loaded oracle also defines this name".
- `xfail::call_sites` is a literal scan that misses a rustfmt-wrapped call and mistakes a
  doc comment for a call site — a tooling fix in the running lane.

## open-rulings — complete list for this topic

1. `ask-dollar-zero-command-substitution-path` — punt stands; re-price against
   `fnd-computed-dot-is-a-whole-book-refusal`.
2. `ask-computed-dot-degrades-to-a-wall` — should the parse-tier refusal become an
   unresolvable-load wall? [PROPOSED yes: the rest of the book is ordinary sh.]
3. `ask-authored-pure-predict-may-site-loads` — parked beyond r31.
4. The plain-sh paste exclusion set — ratify after the atlas mints (the `return` cell is the
   load-bearing measurement).
5. `ask-alias-from-unknown-source` — after the atlas's alias cell: non-issue, or the
   unenumerable tier's documented limitation.
6. Whether book-code inclusion is pulled into r30 (an L lane) or leads r31.
7. The `floor30-atlas-*` mints (conductor, WSL, one command per case from the builder's
   report).

## ledger-updates-owed (held until the human's review of this plan)

`notes/30O` re-cut around this plan (the planner lane absorbing the front-lift; the load
slices = point-havoc + `${0%/*}`; inclusion re-sized; the schedule) · `FORFEITS:forfeit-book-dynamic-load-analysis`
rewritten from four idioms to the three principles (it cites four renamed pin slugs — silent
drift today) · `cli/CLAUDE.md artifact-forms-derive-from-one-structure`'s "front-lift waits on
a licence" sentence retired at the planner lane's fold · the two human items above onto the
queue · the prose queue gains the two refusal texts.
