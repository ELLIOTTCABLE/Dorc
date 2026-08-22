# 30P — Emission, placement, and inclusion: the planner, the stream forms, and how book code is loaded

> Tier: LLM-authored plan (Fable conductor, from the 2026-08-22 design sitting with the
> human; third pass the same day, after the floor atlas was minted and the graded prior-art
> round landed). Subordinate to root docs, `spike/CLAUDE.md`, and `KNOBS.md` (`kBACKFLIPS`
> is the registry entry this plan details). Grades: **[TYPED]** human typed it · **[ACKED]**
> substance confirmed in dialogue · **[PROPOSED]** conductor-derived, awaiting ratification ·
> **[SIDENOTE]** a floated curiosity, explicitly not a plan. Prior-art citations are
> bracketed graded slugs from `.claude/research/emission-and-inclusion-prior-art/`
> (`sources.json`; all `graded-by: subagent`, conductor-read at the findings tier).
> Plainer language than most of the corpus, on the human's instruction.
>
> What this plan owns: the one emission planner (`28Q:pin-emission-planner-universal`), the
> plan's emission FORMS (single-stream versus multipart) and what may be done to a book to
> reach one, and the principles under which a book's `.` lines are loaded — one topic,
> because each answers "where do these bytes go in the plan, and what may we change to put
> them there". It supersedes nothing: `30I` stays the loading/bundling design, `30Ng` the
> stream/form rulings record, `30L` the elision-region design. Sequencing lives in
> `notes/30O`; this plan feeds it.

## the-design-in-one-screen

Dorc writes out an apply script. Oracle function definitions must sit in that file above
their first use, under a name, and without colliding with the book's own names. That
placement-and-naming question is asked in three places today (the preamble hoist, the
bundle inliner, the front-lift ladder) and the ruling is that it is ONE component — the
**emission planner** — answering it once, for probe and apply alike, with only taste
differing between the two. `kFLATTEN` is the probe-time view of that same component.

The planner never rewrites a book. Its only moves are (a) copy bytes verbatim to another
place where copying is provably identical, and (b) rename an *oracle's* definition in the
shipped artifact. When neither is proven it falls back to the most conservative form, and if
the admin asked for a form the book cannot take — single-stream, most often — Dorc refuses
before touching the network, names the line, and names the form that works
(`KNOBS:kBACKFLIPS`, welded). The prior-art round found nothing that contradicts that weld;
every shipped compile-to-fit success bought it by constraining its *input* — a codebase
written for the amalgamation [A-sqlite-mksqlite3c-2026], a closed world with declared externs
[A-closure-advanced-compilation-2025], an ES-module-only subset with an enumerated bailout
list [A-webpack-module-concatenation-2026] — and Dorc cannot constrain its input: the book is
whatever the admin already wrote.

Book `.` lines decide which definitions exist where, so the planner is only as good as the
load model. Three principles replace the idiom-by-idiom approach: an unknown source is a
point havoc, not a poison; an operand resolves only over what the controller already knows,
through shell semantics alone (with a set-valued answer for partly-dynamic operands); and a
plain-sh file that is sourced is *included*.

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
the word `kFLATTEN` already uses — same behaviour, probe-time view. `lift` is NOT a placement
word: "the lift" is the static lift of oracle text into the engine.)

- **`rul-legality-is-code-motions-objective-is-ours`** [TYPED 2026-08-22] — the planner
  borrows code motion's *legality* (a definition must dominate every use; nothing moves across
  a point where its meaning would change; death at the paren) but NOT its objective. Lazy code
  motion places "as late as possible"; Dorc hoists *farther* than latest-possible because the
  attention wall is a contender among the constraints: the non-mutative oracle ocean goes to
  the front, munged as needed, the mutative book last after a divider (`30Ng`). Emission order
  is governed by sh semantics AND attention economics AND taste.
- **`rul-planner-apply-side-first`** [ACKED] — built apply-side now, probe-mode seam
  reserved (a policy toggle on the same component). The probe mode's first consumer —
  per-segment environments for a composed pipe with same-named, differing-bytes helpers
  (`p-x-intra-compound-plurality`) — adds one placement value, never a mechanism.
- **`rul-front-lift-is-the-planners-first-consumer`** [ACKED] — `30Ng:bundle-front-lift-ladder`
  IS the planner's placement question asked of bundles: `tier-hoist-as-is` (née lift-as-is),
  `tier-hoist-munged` (née lift-and-munge), `tier-in-place-rewritten` (née
  positional-with-rewrite), decline. The ladder's shape is shipped praxis: a CLOSED
  enumerated table of named conditions, each mapped to a conservative fallback, plus a
  user-facing disclosure of *which* condition fired at which unit
  [A-webpack-module-concatenation-2026] [A-parcel-scope-hoisting-2024]. Dorc's disclosure is
  a `dorc why` reason arm per tier [PROPOSED]. The tier-one condition "no book call above
  the `.` names a bundle-bound name" is the invariant every scope-merging tool states: a
  merged local must never shadow a global another unit reads, so renaming is a *necessity*,
  not a taste [B-rollup-renaming-necessity-2021] — and in sh every function binding is global.
- Second consumer: `p-x-placement-tuning-pair` (hoist the many-use helper; sink the
  once-used collider) — needs only the `sink` value.
- **Munged names are a product cost, not cosmetics**: Chromium abandoned unity builds
  because the renaming concatenation forced made the code worse, and declined an automated
  renamer three times [A-chromium-jumbo-removal-2019]. Dorc's munge is tractable where
  Chromium's was not only because it is scoped to oracle-custody definitions and gated by
  `rul-happy-path-is-a-closed-set` — authored names by default, munge only on proven need,
  the `30A` zero-munge ratchet holding it.
- Alpha-rename (renaming CALLS inside oracle bodies) stays reserved
  (`d-alpha-rename-equivalence`): it is *hygienic* renaming — rename bound occurrences
  without capturing free ones — which is why it is the harder half; the non-closed-set version
  of it in production is regex-splitting source and guessing which occurrences are references
  [A-maven-shade-simplerelocator-2026]. Every tier above needs only the header-only rename
  (`28R:rul-munge-oracle-names-only`), whose dedup is content-addressed hash-consing
  (`28R:rul-instantiation-hash-dedup`; `26C:disc-hashcons-want-identity`).
- Planner decisions settle from authored-before-contact inputs and enter `Plan::decided` like
  `ImportEdit`; never re-derived at render time, never fed back into analysis
  (`the-render-decides-nothing`; `30Ng:attn-render-refusal-feeds-the-spine` is not a
  prerequisite — a hoist under a proven closed set changes no resolution).
- **`ask-planner-is-the-right-name`** [OPEN, human's] — the field has three words at
  different joints: linkers say **layout / placement / mapping** for deciding where units go,
  parameterised by the user's script [B-gnu-ld-linker-scripts-2026] [A-lld-linker-script-2026];
  **emission** names writing the bytes out, downstream; bundlers say **concatenation / scope
  hoisting / chunking** with **bailout** for a named refusal. `layout` is reserved here (weft
  owns text layout, and emission may *apply* layout — the README's `# because:` commentary
  mode — so layout is a subconcern of emission). Conductor's candidate: **placement planner**
  (the component owns the decision; "emission" is its output). No linker word covers the
  naming half; that is mangling plus resolution.

## the-stream-forms — verbatim-or-refuse, and the uneven floor

**`KNOBS:kBACKFLIPS`** [WELDED, human-typed 2026-08-22] — the enumerated edit classes, the
whole list; a fourth is a human act:

1. **Import re-say in GENERATED plans** (`30Ng:rul-bundle-at-dorc-lang-boundaries`) — the one
   rewrite of book bytes that exists. **Caveat [TYPED]:** a tool in the kit, not a panacea;
   re-saying a `$0`-relative import is surprising, over-magic behaviour with taste failures,
   never an instant "yes". The field's fourth answer to self-location under relocation is the
   same move under another name — make the runtime's self-location TRUE in the new home so
   unmodified code resolves [A-pyinstaller-runtime-information-2026]; its first-class-variable
   answers all leak at exactly the seam where definitions move (Perl's `FindBin`, pre-v3
   `$PSScriptRoot`, CMake's dynamically-scoped `CMAKE_CURRENT_LIST_DIR`)
   [B-perl-findbin-2023] [B-powershell-automatic-variables-2026] [A-cmake-current-list-dir-2026].
2. **Header-only renames of ORACLE-custody definitions** in shipped artifacts
   (`28R:rul-munge-oracle-names-only`). Book bytes, never.
3. **Byte-verbatim relocation under a floor-measured identity set** — paste in place at the
   `floor30-inline-dot-boundary` shape; the hoist ladder; the plain-sh paste set below.

Refused by name, permanently: generated loader functions (floor-REFUTED —
`30I:pin-one-file-root-bundle`; independently corroborated by a shipped bash tool that
documents the scope leak of sourcing inside a `load` function [A-bats-writing-tests-2026]);
any rewrite of a book's control flow, positional parameters, `return`/`exit`, or scoping;
synthesized dispatch.

**`rul-floor-is-uneven-across-forms`** [TYPED 2026-08-22] — most books single-stream fine;
multipart accepts strictly more book code, and the MOST a construct may cost its author is
single-stream, never support. One sentence of self-awareness the prior art earns: refusing
the form is the *unusual* posture. The nearest domain sibling met the identical constraint —
the payload must be pipeable into an interpreter's stdin — and answered it by walking back
textual inclusion in favour of an archive plus a generated loader [A-ansible-module-architecture-2026].
Dorc's floor refutes that loader for sh on measured grounds, so the refusal is defensible;
it is not obvious, and its cost is that the refused set is discovered by users rather than
by the tool author — Chromium's thread is what that feels like [A-chromium-jumbo-removal-2019].

Product-surface consequences: under `30Ng:rul-piped-stdout-carries-a-full-plan`, a piped
`dorc plan` on such a book REFUSES pre-network, naming the line and `--artifact-dir`; and
single-stream is also the no-writable-directory transport floor (`KNOBS:kBOOT`), so the same
books cannot reach those hosts. The refusal text is human prose (the prose queue).

## the-load-principles — three rules, no idioms

**`principle-unknown-source-is-a-point-havoc`** [ACKED; r30] — an unresolvable `.` means
"anything may have been defined here". Every function binding becomes unknown AT THAT LINE;
a later unconditional definition in the same frame re-binds by last-wins. This is the
unknown-callee kill-all transfer the engine already uses one plane down (`16P:T8`, the
ambient gate) applied to the definition plane; today's whole-unit poison is its
flow-insensitive approximation. Nothing else changes: the unknown `.` stays a definition
vector for the planner (nothing hoists across it; defensive renaming stays on) and a wall
for execution — three consumers, three answers. Covers `/etc/os-release`,
`"$HOME/.profile"`, `"$(find_config)"` identically. Posture check against the two shipped
shell resolvers: both refuse until the human declares something (`--fake`/`--keep`,
`# shellcheck source=`) [A-resholve-manpage-2023] [A-shellcheck-directive-wiki-2026]; ShellCheck
also continues past an unfollowable source but then treats it as defining *nothing*
[A-shellcheck-checker-source-props-2026] — acceptable in a linter, under-execution in Dorc.
Point-havoc is the third posture and the only sound one here. Trap for linker intuition:
ELF interposition is first-wins; sh is last-wins.

```sh
. /etc/os-release            # unknown to Dorc: every name is now "maybe" …
hork__is_converged() { … }   # … but THIS is the last definition of this name: live
hork tune web                # licensable
```

**`principle-load-operands-evaluate-over-controller-known-inputs`** — a `.` operand resolves
iff its value is a pure function of what Dorc holds before host contact: program text, the
book's own path, the modeled cwd, the authored snapshot — through a CLOSED allowlist of pure
SHELL operations (`dec-decidable-set-v0`'s precedent: closed, growing by name, each widening
license-review-tier). Excluded by the axis: the target's filesystem, environment, PATH, or a
command's output.

- **`rul-no-tool-modelling-in-the-load-plane`** [TYPED] — evaluating a COMMAND's output inside
  the engine is tool-modelling (`identity-declared-never-inferred`); withdrawn. The rule
  forecloses one *implementation*, not the capability — see the set-valued answer below.
  `${0%/*}` is parameter expansion, modelled, r30. The engine resolves it from the authored
  book path it owns, never from a shell's `$0` (the rail measured `$0`'s shape is
  platform-bound; `gap-dollar-zero-shape-is-platform-bound`). Traps measured: `${0%/*}` of a
  no-slash word is the whole word; of a book at `/` it is the empty string, never "cwd".
- **`rul-partly-dynamic-operand-is-a-set`** [PROPOSED — the prior-art round's finding; a
  human ruling] — an operand with an unknown head and a literal tail
  (`"$(dirname "$0")/helpers.sh"`, `"$LIB/x.sh"` with `$LIB` unknown) is SET-valued over the
  snapshot: every member whose path ends in the literal tail is a candidate; a singleton
  resolves, with the generated plan's import re-said to it (edit class 1, caveat and all);
  plural withholds; empty is unresolvable. This is the machinery the glob ruling already
  builds, reached from the computed-operand side [A-webpack-dependency-management-2026]
  [A-vite-features-glob-dynamic-import-2026]; it models no command, blesses no byte-shape,
  and never invokes `rul-small-allowlists-are-high-cost-minimal-count`. ShellCheck's
  strip-one-dynamic-segment is the same idea without the plurality guard — unsound in
  general, which a linter can afford and Dorc cannot [A-shellcheck-checker-source-props-2026].
  Soundness rests on the re-say: the runtime loads exactly what analysis chose. The
  byte-shape recognition of `$(dirname "$0")` (GCC's include-guard recognition is the
  template for pricing one: enumerated preconditions, off the moment the shape is not
  literal source [A-gcc-cpp-guard-macros-2026]) stays the fallback if this is declined.
  Closes `ask-dollar-zero-command-substitution-path` as a false dichotomy: four options
  existed, not two.
- **Slashless operands** — `. helpers.sh` is a PATH search, not a cwd lookup; the cwd search
  was deliberately removed from the standard over trojan-horse concerns
  [A-posix-dot-builtin-2018], and the atlas measured it fatal with the cwd off PATH. PATH is a
  host read ⇒ unresolvable by the axis; a lint hint ("write `./helpers.sh`"). The same applies
  to a no-match glob's literal pattern when it carries no slash.
- **Globs** — SET-valued over the snapshot, ORDER-UNKNOWN (collation is the target's; the rail
  pins `LC_ALL=C` and measured ASCII order, which proves nothing about the host): members
  defining one name with different bytes WITHHOLD; a sole-member name is live. No-match
  sources the literal pattern, which the atlas measured as FATAL — a failed `.` ends the
  script even as the left operand of `||` (special-builtin semantics) — so everything
  downstream is unreachable, not merely unbound. Builds after `lane-loop-propagation`.
- **`ask-authored-pure-predict-may-site-loads`** [OPEN, parked] — the only principled path to
  evaluating a command's output for a load: an oracle's authored `predict`, through the
  capture lane, under a purity vouch and a widening of `funcenv-reads-source-literal-plane-only`.
  A human ruling; never a lane's. Probe-sourced loads: NACKED [TYPED; permanent law].
- **[SIDENOTE, human 2026-08-22 — not an ack, not a plan]** `kOOB` is mostly discharged. For
  BOOK code only, a world is imaginable where Dorc quietly honours ShellCheck's
  `# shellcheck source=` directive if present — never taught. Priced by the field: a
  comment-carried directive dies wherever the script is generated rather than authored
  [B-hadolint-sc1090-directive-2019]. A curiosity with its weight class; it dies free.

**`principle-book-code-source-is-inclusion`** — a resolvable `.` of an ORDINARY (non-dorc-lang)
sh file is textual inclusion: `.` opens no scope, keeps `$0` (measured), keeps the positional
parameters — by the standard's *silence*: operand-passing `. f a b` is a KornShell extension
[A-posix-dot-builtin-2018], and the atlas measured dash ignoring operands while posh honours
them, so `.` with operands is outside the base dialect. A top-level `return` in the file
behaves like a function's. Three tiers, one rule, no rewriting:

1. **Analysis** — splice the file at the `.` site under whatever branch it sits in (the
   call-splice precedent, `seam-interproc`); funcdefs are definitions (unconditional ⇒ live;
   under an undecidable guard ⇒ `May`); commands are sites; nested `.` recurses under the
   splice budgets; lines keep their own line-space. Measured exact on: `shift`/`set --`,
   `unset -f`, `cd`, `trap … EXIT` all persist to the caller — and a dependency's `cd` moves the
   coordinate every later relative `.` resolves against (the owed cwd-state work).
2. **Multipart emission** — the file ships beside its plan; an existence guard runs FOR REAL
   on the host against the shipped tree, so Dorc needs only "maybe" for the bindings. The
   file gets its own `<name>.plan.sh` when it has mutative sites worth editing
   (`30Ng:rul-bundle-at-dorc-lang-boundaries`'s two-files shape). The precision upgrade —
   deciding `[ -f ./optional.sh ]` TRUE because Dorc ships it — is the four-seat agreement and
   comes after.
3. **Single-stream emission** — byte-verbatim paste under a closed exclusion set; otherwise
   refuse with the form named. The set, now RULABLE rather than merely measurable
   (**`rul-paste-excludes-non-subshell-return`** [PROPOSED; the standard settles it]): the `.`
   is whole-line/top-level/redirect-free (measured — and this condition is doing errexit work
   too: under `set -e` the `||` exemption does not reach inside a sourced file, while a pasted
   `{ …; } || x` WOULD exempt it, so the `||`-operand form must never paste); and the file
   contains no `return` outside a subshell — POSIX says `return` outside a function or dot
   script is UNSPECIFIED, naming the System V (error) versus KornShell (exit) split
   [A-posix-return-builtin-2018]; the atlas measured that `return` inside `{ }` and `if` leaves
   the file while `( )` contains it. Top-level `return` is the same construct JS bundlers name
   as their hard bailout, and their fix is the function wrapper Dorc's floor refuted
   [A-parcel-scope-hoisting-2024]. A `return`-guarded library therefore makes a book
   unpipeable, by the uneven-floor ruling, with a hint: "use an `if` include-guard".

Sizing [PROPOSED]: analysis M + emission M. Split by mechanic for the r30 decision:
`mech-acquire-and-ship-plain-sh` (read the sourced plain-sh file, record it as a load
occurrence, mirror it beside the plan, analyze NOTHING in it — the site walls as today; small
and local to `lane-load-plane-precision` + the planner lane's `Selection`) ·
`mech-splice-plain-sh-for-analysis` (the multi-file splice; disjoint from every r30 lane) ·
`mech-paste-plain-sh-single-stream` (the exclusion-set paste; after the first two). ~SUSPECT,
to confirm before ruling: today a plain-sh `.` is not acquired, so nothing ships, so the
generated plan carries a dangling `.` that the atlas measured as fatal — `dorc apply` on the
most common multi-file book shape would die at that line on the host.

## scheduling-truth — horizons are attention-calls

[TYPED 2026-08-22] An xfail horizon names a round past which *forgetting* the item is
unacceptable — a scheduled attention-call at which it is scheduled or punted again — never
the round it completes in. The `r31:*` horizons stand as minted; no re-horizon act; no LLM
effort on them. What IS r30 (kernel quiescence at its close): point-havoc and `${0%/*}` in
`lane-load-plane-precision` (havoc first; the alias cell read — it did not materialise for
the sourced-alias-then-book-funcdef shape); `lane-loop-propagation`; the planner lane;
`lane-influence-carriage` (stays; critical for reasons outside the conductor's horizon).
Pending the human (`ask-inclusion-in-r30`): the inclusion mechanics above — the human's lean
is punt-behind-one-FORFEITS-row unless a part is local to lanes already touching those files.

## the-atlas — durable floor measurements

Twenty-four `floor30-atlas-*` manifests, authored by the xfail lane's builder, minted by the
conductor under `posh 0.14.1` ∩ `dash 0.5.12` (2026-08-22; a second mint the same day was
byte-identical — `emitted-is-measure-once-ground-truth` holding). Twenty-two agree and are
minted; four DISAGREE and carry the measured divergence verbatim in their headers, opted out
of the floor gate (the harness has no resting place for a disagreement —
`gap-disagreement-has-no-resting-place`; an XFAIL marker cannot work there, it XPASSes).
The divergences: `dot-with-operands` (dash ignores, posh honours — and reports a phantom
third positional) · `local-at-toplevel-of-sourced-file` (dash aborts the file, posh accepts)
— both outside the base dialect · `readonly-set-in-sourced-file` and
`nested-dot-resolves-against-cwd` — status-only divergences of a rejected construct, which
`fence-rejection-rc` already excludes; the mechanics themselves agree (`.` resolves against
the cwd, never the sourcing file: `30I:rul-dot-resolves-as-sh` confirmed). Harness gaps, all
reported, none worked around, NO tooling work scheduled [TYPED]: `gap-early-exit-capture` ·
`gap-no-subdirectory-fixtures` · `gap-stdin-invoked-book` · `gap-dollar-zero-shape-is-platform-bound`
· `gap-locale-dependent-glob-order` · `gap-atlas-mode` (footnote: per-shell recorded texts
would make a divergence a committed, re-checked artifact — `work-atlas-divergence-manifests`,
unscheduled) · `gap-pinned-shell-matrix`. Steering lag: `gate:full-quiet` DOES route
`test:floor` when floor paths are staged; `spike/CLAUDE.md` says the lane is off in every
default gate.

## what-landed — the executable specification

Lane `ai/r30-lane-load-xfails`, folded (pins `6785fada`; scanner fix `5148fe84`; atlas
`da12ef00` + `fdd87fc3`): seventeen live pins, census coherent, horizons as minted.

| pin | horizon | target |
|---|---|---|
| `p-x-unknown-source-is-a-point-havoc` | end-of-r30 | later unconditional role definition is live below an unresolvable `.` |
| `p-x-load-operand-param-expansion-of-dollar-zero` | end-of-r30 | `. "${0%/*}/helpers.dorc.sh"` acquires and binds |
| `p-x-load-operand-dirname-of-dollar-zero` · `…-cd-pwd-of-dollar-zero` | r31:book-load-acceptance | trigger names `ask-dollar-zero-command-substitution-path` (now `rul-partly-dynamic-operand-is-a-set`'s) |
| `p-x-glob-load-acquires-members` · `…-members-are-order-unknown` · `…-no-match-aborts` | r31:book-load-acceptance | the set-valued operand, its withhold, its (fatal) failure |
| `p-x-book-code-source-is-inclusion` | r31:book-load-acceptance | unconditional plain-sh `. ./helpers.sh` binds; the guarded cell holds `May` |

Whole-product XFAIL cases: `load30-point-havoc-and-script-relative` (expected to pass before
the end of r30) · `load31-punted-load-shapes`.

## findings-this-sitting

- **`fnd-computed-dot-is-a-whole-book-refusal`** — an inline `$(…)` in a `.` operand is a
  PARSE-tier refusal (`syntax-unsupported`, exit 10): the book is rejected outright. Harsher
  than ShellCheck, which continues past an unfollowable source
  [A-shellcheck-checker-source-props-2026]. See `ask-computed-dot-degrades-to-a-wall`.
- **`fnd-squat-warning-contradicts-in-book-lift`** — `reserved-namespace-squat` still fires on
  a book-defined `hork__is_converged` with prose false since the r28 in-book lift; prose is
  the human's, the lint wants re-scoping.
- **`fnd-corpus-was-silent-on-this-topics-prior-art`** — the project's research held two
  anchors (`16P:T8`; `26C`/`28R` hash-consing) and was otherwise silent; the vendor dive it
  deferred (`24T:vendor-dives-deferred (née §9)`) is discharged by the round banked at
  `.claude/research/emission-and-inclusion-prior-art/` (30 sources: 21 A · 8 B · 1 C).
  Available on request from that round, not yet gathered: ELF `$ORIGIN`/`patchelf`,
  `-fno-semantic-interposition`, Ruby `require_relative`, Node `import.meta.dirname`, busybox
  `.`, Lua `package.path`.
- **`fnd-no-shell-native-predecessor`** — the shell ecosystem reaches one-standalone-script by
  GENERATING from a non-sh config [C-bashly-readme-2026]; Dorc's relocation of *authored* sh
  has no shell predecessor and borrows from linkers and bundlers by necessity.

## open-rulings — complete list for this topic

1. `rul-partly-dynamic-operand-is-a-set` — ratify or decline (the `$0` question's resolution;
   [PROPOSED ratify: no allowlist, no tool-modelling, sound under the re-say, withholds on
   plurality; the re-say caveat applies]).
2. `ask-computed-dot-degrades-to-a-wall` — [PROPOSED yes; ShellCheck precedent].
3. `ask-inclusion-in-r30` — [PROPOSED: `mech-acquire-and-ship-plain-sh` in r30 as a small
   slice across the two lanes already touching those files, after confirming the
   runtime-dead suspicion; the splice and the paste punted behind one FORFEITS row,
   `forfeit-plain-sh-inclusion-analysis`].
4. `rul-paste-excludes-non-subshell-return` — [PROPOSED ratify now; the standard decides it].
5. `ask-planner-is-the-right-name` — [PROPOSED `placement planner`].
6. `ask-authored-pure-predict-may-site-loads` — parked, unscheduled.

## ledger-updates-owed (held until the human's rulings above)

`notes/30O` re-cut (the planner lane absorbing the hoist ladder; the load slices; inclusion
per item 3; no "droppable" lanes) · `FORFEITS:forfeit-book-dynamic-load-analysis` rewritten
from four idioms to the three principles (it cites four renamed pin slugs) + the
inclusion row if item 3 punts · `cli/CLAUDE.md` harness-contract lines (no-subdirectory
fixtures; platform-bound `$0`; the floor lane IS routed) · `spike/CLAUDE.md
floor-differential-lane-opt-in`'s "off in every default gate" sentence · the two
stale-prose findings onto the human queue · the prose queue gains the two refusal texts.
