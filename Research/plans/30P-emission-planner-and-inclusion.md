# 30P — Emission, placement, and inclusion: the planner, the stream forms, and how book code is loaded

> Tier: LLM-authored plan (Fable conductor, from the 2026-08-22 design sitting with the
> human; session `r30-conductor-3`; second refinement pass the same day, a third owed
> after the prior-art round). Subordinate to root docs, `spike/CLAUDE.md`, and `KNOBS.md`
> (`kBACKFLIPS` is the registry entry this plan details). Grades: **[TYPED]** human typed
> it · **[ACKED]** substance confirmed in dialogue · **[PROPOSED]** conductor-derived,
> awaiting ratification · **[SIDENOTE]** a floated curiosity, explicitly not a plan.
> Written in plainer language than most of the corpus, on the human's instruction;
> strawmen are illustrative, never spellings.
>
> What this plan owns: the one emission planner (`28Q:pin-emission-planner-universal`), the
> plan's emission FORMS (single-stream versus multipart) and what may be done to a book to
> reach one, and the principles under which a book's `.` lines are loaded — one topic,
> because each answers "where do these bytes go in the plan, and what may we change to put
> them there". It supersedes nothing: `30I` stays the loading/bundling design, `30Ng` the
> stream/form rulings record, `30L` the elision-region design; this plan is the
> emission-and-inclusion layer they all touch, assembled once. Sequencing and lane sizing
> live in `notes/30O`; this plan feeds it. The prior-art round protecting it is
> `.claude/research/emission-and-inclusion-prior-art/` (in flight at this pass).

## the-design-in-one-screen

Dorc writes out an apply script. Oracle function definitions must sit in that file above
their first use, under a name, and without colliding with the book's own names. That
placement-and-naming question is asked in three places today (the preamble hoist, the
bundle inliner, the front-lift ladder) and the ruling is that it is ONE component — the
**emission planner** — answering it once, for probe and apply alike, with only taste
differing between the two. `kFLATTEN` is the probe-time view of that same component, not
a sibling mechanism.

The planner never rewrites a book. Its only moves are (a) copy bytes verbatim to another
place where copying is provably identical, and (b) rename an *oracle's* definition in the
shipped artifact. When neither is proven it falls back to the most conservative form:
definitions stay where the author put them, under renamed names, and if the admin asked
for a form the book cannot take — single-stream, most often — Dorc refuses before touching
the network, names the line, and names the form that works. That posture is welded:
`KNOBS:kBACKFLIPS`.

Book `.` lines decide which definitions exist where, so the planner is only as good as the
load model. Three principles replace the idiom-by-idiom approach: an unknown source is a
point havoc, not a poison; an operand resolves only over what the controller already knows,
through shell semantics alone; and a plain-sh file that is sourced is *included* — analyzed
as if pasted, shipped beside its plan, and pasted into a single stream only under a
floor-measured identity set.

## the-emission-planner — one component, two modes, a closed vocabulary

`28Q:pin-emission-planner-universal` [human-ruled DIRECTION, 2026-08-16] asked for one
abstract planner over the sh truths (definition visibility · death at the paren · the
errexit exemption of an `||`-left), supporting placement × naming:

```text
placement:  hoist (to the top) | in-place (where the author put it) | sink (into the subshell that uses it)
naming:     authored | munged (`name_h<digest>`, header-only)
mode:       probe (verbose-tolerant, no book namespace)  |  apply (idiomatic-first, attention-priced)
```

(`hoist`/`in-place`/`sink`, née top-lift/adjacent/in-paren-colocated. `hoist` is exactly
the word `kFLATTEN` already uses — same behaviour, probe-time view — so it is kept, not
avoided. `lift` is NOT a placement word: "the lift" is the static lift of oracle text into
the engine, a different thing; `front-lift` reads as `hoist` from here on.)

- **`rul-legality-is-code-motions-objective-is-ours`** [TYPED 2026-08-22, the LCM nit] —
  the planner borrows code motion's *legality* (a definition must dominate every use;
  nothing moves across a point where its meaning would change; death at the paren), but
  NOT its objective. Lazy code motion places "as late as possible"; Dorc deliberately
  hoists *farther* than latest-possible, because the attention wall is a contender among
  the constraints: the non-mutative oracle ocean goes to the front, munged as needed, and
  the mutative book sits last after a divider (`30Ng` "the single-stream review surface").
  Emission order is governed by sh semantics AND attention economics AND taste.
- **`rul-planner-apply-side-first`** [ACKED 2026-08-22] — built apply-side now, with the
  probe-mode seam reserved (a policy toggle on the same component, never a second
  planner). The probe mode's first consumer — per-segment environments for a composed
  pipe whose two participants carry same-named, differing-bytes helpers
  (`p-x-intra-compound-plurality`) — adds one placement value ("inside this pipe stage's
  subshell") to the same vocabulary.
- **`rul-front-lift-is-the-planners-first-consumer`** [ACKED 2026-08-22] —
  `30Ng:bundle-front-lift-ladder` IS the planner's placement question asked of bundles:
  `tier-hoist-as-is` (née lift-as-is) = hoist + authored name; `tier-hoist-munged` (née
  lift-and-munge) = hoist + munged name; `tier-in-place-rewritten` (née
  positional-with-rewrite) = in-place + the import re-said; decline = the refused set.
  A separate ladder mechanism would have been a third placement engine — the debt `28Q`'s
  no-more-piecemeal order exists to prevent.
- Second consumer: `p-x-placement-tuning-pair` (hoist the many-use helper; sink the
  once-used collider) — it needs only the `sink` value.
- Alpha-rename (renaming CALLS inside oracle bodies, not just the header) stays reserved
  (`d-alpha-rename-equivalence`). In the field's terms it is *hygienic* renaming — rename
  bound occurrences without capturing free ones — which is why it is the harder half;
  every tier above needs only the header-only rename that exists
  (`28R:rul-munge-oracle-names-only`), whose dedup identity is content-addressed
  hash-consing (`28R:rul-instantiation-hash-dedup`, grounded in
  `26C:disc-hashcons-want-identity`: Filliâtre/Conchon, salsa, git).
- Planner decisions settle from authored-before-contact inputs and enter `Plan::decided`
  like `ImportEdit` does; never re-derived at render time, never fed back into analysis
  (`the-render-decides-nothing`; the `30Ng:attn-render-refusal-feeds-the-spine` sitting is
  NOT a prerequisite — a hoist under a proven closed set changes no resolution).
- **`ask-planner-is-the-right-name`** [OPEN] — the human asks what the industry calls a
  phase that runs after all analysis and optimisation, is parameterised only by the user's
  requests, and decides where definitions go and under what names. Conductor's candidate:
  *layout* (compilers' code layout; linkers' section layout, whose user-facing
  parameterisation is literally the linker script; bundlers' chunk layout). The prior-art
  round's item 4 answers with citations; rename then, once.

## the-stream-forms — verbatim-or-refuse, and the uneven floor

**`KNOBS:kBACKFLIPS`** [WELDED, human-typed 2026-08-22] — Dorc never performs
source-to-source transformation to make an emission form possible. The enumerated edit
classes, the whole list; a fourth is a human act:

1. **Import re-say in GENERATED plans** (`30Ng:rul-bundle-at-dorc-lang-boundaries`) — the
   one rewrite of book bytes that exists, because a generated plan is a durable but not an
   off-ramp durable. **Caveat [TYPED 2026-08-22]:** this is a tool in the kit, not a
   panacea — re-saying a `$0`-relative import in particular is surprising, over-magic
   behaviour toward the user, with taste failures and only slightly-narrow danger; it is
   never an instant "yes, that".
2. **Header-only renames of ORACLE-custody definitions** in shipped artifacts
   (`28R:rul-munge-oracle-names-only`). Book bytes, never.
3. **Byte-verbatim relocation under a floor-measured identity set** — paste in place at the
   `floor30-inline-dot-boundary` shape (a `.` that is the whole of its line, top-level,
   redirect-free); the hoist ladder; the plain-sh paste set this plan owes (below).

Refused by name, permanently: generated loader functions (floor-REFUTED —
`30I:pin-one-file-root-bundle`, the `floor30-dot-loader-function-errexit` manifest: inside a
function, `set -e` and `set --` behave differently than under `.`); any rewrite of a book's
control flow, positional parameters, `return`/`exit`, or scoping; synthesized dispatch.

**`rul-floor-is-uneven-across-forms`** [TYPED 2026-08-22] — "we cannot single-stream
*everything*" (for-all, not for-some: most books single-stream fine). Multipart output
accepts strictly more book code than single-stream, and the MOST a construct may cost its
author is single-stream — never support. Test-then-source
(`[ -r ./local.sh ] && . ./local.sh`) and `return`-guarded library files are the motivating
shapes: ordinary, must work, and may honestly make a book unpipeable.

Product-surface consequences, stated so they are discovered here and not in the field:

- Under `30Ng:rul-piped-stdout-carries-a-full-plan`, a piped `dorc plan` on such a book
  REFUSES pre-network, naming the line and `--artifact-dir`. The refusal text is human prose
  (the prose queue); the behaviour is this plan's.
- Single-stream is also the transport floor for hosts with no writable directory
  (`KNOBS:kBOOT`), so the same books cannot reach those hosts. Same refusal, same honesty.

## the-load-principles — three rules, no idioms

The sibling's 2026-08-22 xfails named four idioms (`$(dirname "$0")`, a source glob, an
existence-guarded file, `/etc/os-release`). Idioms are an open set; blessing one invites the
next. The cut is on axes [ACKED 2026-08-22]:

**`principle-unknown-source-is-a-point-havoc`** — an unresolvable `.` means "anything may
have been defined here", which is sh's own meaning. Every function binding becomes unknown
AT THAT LINE; a later unconditional definition in the same frame re-binds by last-wins.
This is the unknown-callee kill-all transfer the engine already uses one plane down —
`16P:T8` the ambient gate ("an un-oracled command is Opaque ⇒ ⊤ ⇒ poisons all downstream
ambient-ness") — applied to the definition plane; today's whole-unit poison is its
flow-insensitive approximation. Nothing else changes: the unknown `.` stays a definition
vector for the planner (nothing hoists across it; defensive renaming stays on) and a wall
for execution. Three consumers, three answers — binding precision, placement, execution;
conflating them is how a precision gain becomes running someone else's body. Covers
`/etc/os-release`, `"$HOME/.profile"`, `"$(find_config)"` identically, with no name list and
no file parsing. **Scheduled r30** [TYPED 2026-08-22: "proper havoc in r30"].

```sh
. /etc/os-release            # unknown to Dorc: every name is now "maybe" …
hork__is_converged() { … }   # … but THIS is the last definition of this name: live
hork tune web                # licensable
```

One trap for anyone importing linker intuition: ELF symbol interposition picks the FIRST
definition in load order; sh picks the LAST. Last-wins is not a bug to fix.

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
  expansion — shell semantics, which `rul-unsure-falls-toward-sh-parity` says we model —
  and is the r30 slice. Honest rider: `${0%/*}` is the rarer spelling in the wild and has
  its own gotcha (`${0%/*}` of a bare `script.sh` is `script.sh`), so the hint asks people
  to edit their books; that is gradual enhancement working, and the edit is plain POSIX.
- **`rul-small-allowlists-are-high-cost-minimal-count`** [TYPED 2026-08-22] — small
  allowlists and cheating a little on referential inequality are not welded out; they are a
  high price, paid only for high value, at minimal count, and only where no pure-sh
  alternative exists. dorc-lang authors may be asked to be unergonomic; book authors may
  need Dorc to bend a bit — but there is still an analysis bar and a
  go-fix-your-code line, just an astronomically higher one than in dorc-lang.
- **`ask-dollar-zero-command-substitution-path`** [OPEN, human's] — the one
  idiom-recognition "in the running": `$(dirname "$0")` and
  `$(cd "$(dirname "$0")" && pwd)` as fixed byte-shapes meaning "the book's directory",
  valid only as a `.` operand, guarded by the frame model's answer that no function named
  `dirname` may be bound there. Designable, count-of-two. Punted while `${0%/*}` exists;
  re-priced against `fnd-computed-dot-is-a-whole-book-refusal` (below) and against the
  prior-art round's finding on how shipped products answered the self-location problem
  (recognition vs modelling vs a first-class variable vs a directive).
- **`ask-authored-pure-predict-may-site-loads`** [OPEN, parked, unscheduled] — the only
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
  as its second population source.
- Every `$0`-relative import in a GENERATED plan must be re-said (edit class 1, with its
  caveat): the real ssh executor pipes the plan on stdin, so on the host `$0` is the
  shell's name and `$(dirname "$0")` is `.`.
- **[SIDENOTE, human 2026-08-22 — not an ack, not a plan]** `kOOB` is mostly discharged: the
  risky out-of-band surface is already out of band, PLT-first and DX-fronted (the `:`
  marks). For BOOK code only, a world is imaginable where Dorc quietly accepts ShellCheck's
  `# shellcheck source=` directive verbatim — never taught, merely honoured if present,
  since authors are already trained to unroll dynamic loads for it. Recorded as a
  curiosity with its weight class; it dies free.

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
   set; otherwise refuse with the form named. The set as currently understood (the atlas
   proves or amends it): the `.` is whole-line/top-level/redirect-free (measured); the file
   has no top-level `return` (the one construct whose meaning bare paste changes — `return`
   inside a brace group or `if` at the file's top level included; `exit`, errexit, traps,
   `shift`/`set --`, nested `.` are paste-identical). No prior art pastes a file that can
   `return` — bundlers' modules cannot, the preprocessor has no control flow — which is why
   the atlas measures it rather than arguing it. A `return`-guarded library therefore makes a
   book unpipeable, by the uneven-floor ruling, with a hint: "use an `if` include-guard if
   you want single-stream plans".

Sizing [PROPOSED]: analysis M (splice precedent) + emission M (two-files plumbing, paste
under the set, one floor manifest). Still walls, by rule, and should: `. "$HOME/x"` and
`. "$(find_config)"` — the file is not in the snapshot, so nothing could be shipped for it.

## scheduling-truth — there is no r31

[TYPED 2026-08-22] r31 is not a plan. End-of-r30 is kernel quiescence; whatever is not in
r30 is not "later" but *unscheduled, behind other work* (multi-target, first-blooding, and
the rest of the critical queue that is holding on the kernel rewrite). Every "r31" horizon
in the census and in this plan's first pass therefore mis-states the deal. Consequences,
for `notes/30O`'s next pass and the human's ruling:

- Point-havoc and the `${0%/*}` slice are r30, in `lane-load-plane-precision`, as that
  lane's first commit-series (the atlas's alias cell read first).
- Book-code inclusion (analysis + multipart) is either an r30 lane (L) or unscheduled. The
  conductor's lean [PROPOSED]: r30, because the elephant (`. ./helpers.sh` of plain sh walls
  today) is the single largest coverage hole for real books and first-blooding will hit it
  on day one; the single-stream paste tier waits on the atlas and may trail.
- Globs: after loop-propagation if time; else unscheduled, honestly.
- The census pins currently horizoned `r31:book-load-acceptance` are re-horizoned by the
  builder once the human rules: `end-of-r30` for what lands, `Unscheduled { why }` for the
  rest — never a round that does not exist.

## the-atlas — durable floor measurements before any mechanism

The project's floor discipline (`spike/CLAUDE.md` floor-differential-lane-opt-in ·
emitted-is-measure-once-ground-truth · `30A` d1) is the instrument: a sentinel-emitting
script, run under `posh 0.14.1` ∩ `dash 0.5.12`, agreement with each other and with the
committed bytes required, minted once through `bless:floor` (orchestrator, WSL).
Twenty-four narrow manifests (`floor30-atlas-*`) were authored by the xfail lane's builder
and minted by the conductor 2026-08-22. First results:

- Twenty agree and are minted.
- Four DISAGREE between the binaries — which is the atlas's answer for those mechanics, and
  the harness has no resting place for a disagreement (`gap-disagreement-has-no-resting-place`).
  Interim resting place: the four cases are committed XFAIL with the measured divergence
  verbatim in their headers. Two read so far: `readonly-set-in-sourced-file` — the ONLY
  divergence is the rejected assignment's status (dash 2, posh 1), which
  `fence-rejection-rc` already says no rule may depend on; `local-at-toplevel-of-sourced-file`
  — a real divergence (dash aborts the file at `local` with status 2; posh accepts it and
  the value survives), so `local` outside a function is outside the base dialect and the
  paste question for it is moot. `dot-with-operands` and `nested-dot-resolves-against-cwd`
  are pending the builder's full read.
- Harness gaps, all reported, none worked around: `gap-early-exit-capture` (stdout only; no
  rc, no stderr) · `gap-no-subdirectory-fixtures` · `gap-stdin-invoked-book` (`$0` always
  carries slashes under the rail) · `gap-locale-dependent-glob-order` (`LC_ALL=C` pinned) ·
  `gap-atlas-mode` (for bash/zsh/busybox later, disagreement must be recordable data) ·
  `gap-pinned-shell-matrix` (versioned shells across distributions; the containerized
  `livetest` kit is the natural home).

## what-landed-today — the executable specification

Lane `ai/r30-lane-load-xfails`, folded at `6785fada` (pins) and continuing (atlas):
seventeen live pins, census coherent. The book-load pins, re-cut from four idioms to the
three principles (bodies kept where they were right); horizons as first minted, pending
the scheduling ruling above:

| pin | horizon (as minted) | target |
|---|---|---|
| `p-x-unknown-source-is-a-point-havoc` | end-of-r30 | later unconditional role definition is live below an unresolvable `.` (companions: the names-bound-only-before half is GREEN today; the opener-and-wall anti-regression is green) |
| `p-x-load-operand-param-expansion-of-dollar-zero` | end-of-r30 | `. "${0%/*}/helpers.dorc.sh"` acquires and binds |
| `p-x-load-operand-dirname-of-dollar-zero` · `…-cd-pwd-of-dollar-zero` | r31:book-load-acceptance → re-horizon | trigger names `ask-dollar-zero-command-substitution-path` |
| `p-x-glob-load-acquires-members` · `…-members-are-order-unknown` · `…-no-match-aborts` | r31:book-load-acceptance → re-horizon | the set-valued operand, its withhold, its failure |
| `p-x-book-code-source-is-inclusion` | r31:book-load-acceptance → re-horizon | unconditional plain-sh `. ./helpers.sh` binds; the guarded cell holds `May` |

Two whole-product cases, XFAIL-marked (the e2e XFAIL form carries no horizon — a harness
limit): `load30-point-havoc-and-script-relative` (expected to pass before the end of r30)
and `load31-punted-load-shapes` (rename owed with the re-horizon: there is no r31).

## findings-this-sitting

- **`fnd-computed-dot-is-a-whole-book-refusal`** — an inline `$(…)` in a `.` operand is a
  PARSE-tier refusal today (`syntax-unsupported`, exit 10): a book containing
  `. "$(dirname "$0")/helpers.sh"` is not walled from that line down, it is rejected
  outright. The current price of punting `dirname` is "Dorc will not analyze your book" —
  a harsher cliff than any other in this design, and harsher than ShellCheck's per-line
  `SC1090`. See `ask-computed-dot-degrades-to-a-wall`.
- **`fnd-squat-warning-contradicts-in-book-lift`** — `reserved-namespace-squat` still fires
  on a book-defined `hork__is_converged` and says Dorc "treats it as ordinary shell and
  runs it verbatim", false since the r28 in-book lift
  (`cli/CLAUDE.md the-book-is-a-definition-source`; USER_STORY stage 3). Prose is the
  human's; the lint's firing condition wants re-scoping to "a loaded oracle also defines
  this name".
- **`fnd-corpus-silent-on-this-topics-prior-art`** — the project's own research holds two
  strong anchors for this plan (`16P:T8`; `26C`/`28R` hash-consing) and is otherwise silent:
  no bundler, linker, ShellCheck-source, LCM-mechanics, hygiene, `dlopen`, or Bazel-glob
  material, and the vendor dive the corpus itself deferred
  (`24T:vendor-dives-deferred (née §9)`: CoLiS, ShellCheck source interior, mvdan-sh/oils)
  was never done. The prior-art round in flight discharges it.
- `xfail::call_sites` was a literal scan that missed a rustfmt-wrapped call and mistook a
  doc comment for a call site — fixed in the lane (`5148fe84`).

## open-rulings — complete list for this topic

1. `ask-dollar-zero-command-substitution-path` — punt stands; re-price after the
   prior-art round and `fnd-computed-dot-is-a-whole-book-refusal`.
2. `ask-computed-dot-degrades-to-a-wall` — should the parse-tier refusal become an
   unresolvable-load wall so the rest of the book still analyzes? [PROPOSED yes.]
3. `ask-inclusion-in-r30` — book-code inclusion as an r30 L lane, or unscheduled
   (`scheduling-truth`). [PROPOSED r30.]
4. `ask-planner-is-the-right-name` — pending the prior-art round's item 4.
5. The plain-sh paste exclusion set — ratify after the atlas's `return` cells are read.
6. `ask-alias-from-unknown-source` — after the atlas's alias cell: non-issue, or the
   unenumerable tier's documented limitation.
7. `ask-authored-pure-predict-may-site-loads` — parked, unscheduled.
8. The census re-horizon (`r31:…` → `end-of-r30` / `Unscheduled`) — a builder act after
   items 1–3.

## ledger-updates-owed (held until the human's review of this plan)

`notes/30O` re-cut around this plan (the planner lane absorbing the hoist ladder; the load
slices; inclusion's placement per item 3; the schedule without an r31) ·
`FORFEITS:forfeit-book-dynamic-load-analysis` rewritten from four idioms to the three
principles (it cites four renamed pin slugs — silent drift today; "r31" wording too) ·
`cli/CLAUDE.md artifact-forms-derive-from-one-structure`'s "front-lift waits on a licence"
sentence retired at the planner lane's fold · the human items above onto the queue · the
prose queue gains the two refusal texts · the `24T` deferred-dive item closed by the
research round.
