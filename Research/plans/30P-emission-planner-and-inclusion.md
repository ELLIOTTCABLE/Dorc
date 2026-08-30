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
list [A-webpack-module-concatenation-2026]. Dorc *could* constrain its input — it is merely
expensive — and the human's posture note [TYPED 2026-08-22] is the one to hold: the loader
edge is the SOFTEST place in the whole posture, the rock-bottom origin of the
gradual-enhancement curve, where input admission should be maximal and output value is at its
minimum — so it is the best place to trade power for acceptance, bounded by two rules: no
slippery slope, and every carve-out must teach the user where to head next. Many `KNOBS`
welds are soft barriers of osmosis exactly here; each is relitigable, carefully and
boundedly, and HUMAN-ONLY.

Book `.` lines decide which definitions exist where, so the planner is only as good as the
load model. Three principles replace the idiom-by-idiom approach: an unknown source is a
point havoc, not a poison; an operand resolves only over what the controller already knows,
through shell semantics alone (set-valued only for a glob under an EXACT head); and a
plain-sh file that is sourced is *included*. How the plane stays correct — what may be
rewritten, what the controller may evaluate speculatively, and what each phase verifies at
standup — is `the-load-plane-stays-correct`.

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
- **`rul-emission-is-the-umbrella-name`** [TYPED 2026-08-22] — the field has three words at
  different joints: linkers say **layout / placement / mapping** for deciding where units go
  [B-gnu-ld-linker-scripts-2026] [A-lld-linker-script-2026]; **emission** names writing the
  bytes out; bundlers say **concatenation / scope hoisting / chunking** with **bailout** for a
  named refusal. Dorc's vocabulary, ruled: **`emission`** is the umbrella (the planner, the
  forms, and the attendant miscellany — "emission planner" stands); **`placement`** is reserved
  for SEMANTIC arrangement, the sh-parity engine concerns (where a definition may go and under
  what name); **`layout`** is reserved for textual-emission generics and their engine (weft —
  e.g. the README's `# because:` commentary mode). No linker word covers the naming half;
  that is mangling plus resolution.

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

**`law-no-unsoundness-below-a-blind-act`** [HUMAN, typed in substance 2026-08-22 — "we're not
bending that for ergonomics"; "I am not rushing to introduce unsoundness here"]. Read this before
anything below it. A *blind act* is a line whose effect on the shell Dorc cannot see: a `.` of a
file the controller does not hold, an `eval` of ⊤, a call into a body Dorc cannot splice. Below
one, Dorc's model of the shell is ⊤ and Dorc claims NOTHING from it, at any tier, under any flag:

- no cwd-dependent decision — a relative `.` is not EXACT, `[ -f ./x ]` does not decide,
  slashless `$0` is ⊤; no definition loaded below it carries authority; no load below it is
  re-pointed or pasted (a rewrite of a reference whose resolution is unknown changes which file
  the host loads — explicitness alone never licenses a rewrite);
- no elision below it — the act ran commands, so it is a total wall (no footprint, no survival);
- nothing shipped on a guess — a copy of a file Dorc cannot prove the author referenced is engine
  selection, the thing `rul-load-head-is-exact-or-havoc` struck [LEAN, human 2026-08-22, "I'd
  probably lean no … referential agnosticism"; one typed line to the running conductor settles
  `30Q:ask-ship-explicit-targets-below-a-clobber`];
- no engine-side recovery of any kind — not a syntactic check of host bytes, not an oracle's
  claim about the shell model (a wrong one mis-attributes every line below to the wrong author,
  the top of `271:rul-sin-ordering`), not a rewrite into a subshell (`kBACKFLIPS`).

What survives: guards, because a guard is authored sh re-measured live and Dorc guarantees only
its own movement (`rul-guard-resolves-like-its-mutation`); and the author's own remedies, which
are plain defensive sh — contain it, reset it, assert it, establish it, or read it instead of
executing it — catalogued in `notes/30Pd` with their exact payoffs. Recovery is author-spelled,
attributable, and keeps the off-ramp; the hint names the cheapest remedy for the shape seen.
Why the bought unsoundness does not extend here: that one is an imperfect model of the WORLD,
fenced three ways and committee-shaped; this would be an imperfect model of the SHELL, which
nobody but POSIX may speak for, single-speaker, with the whole suffix as blast radius.

**`principle-unknown-source-is-a-point-havoc`** [ACKED; r30 — NARROWED 2026-08-22 by the law
above: a point of total unknowledge for the shell model; the re-bind clause describes which
BYTES Dorc pins, never a runtime binding it trusts] — an unresolvable `.` means
"anything may have happened here": every binding — function, alias, variable, cwd, shell
option, positional, and whether execution continues at all — is unknown AT THAT LINE; a later
unconditional definition in the same frame re-binds its own NAME by last-wins. CAVEAT
[conductor, 2026-08-22, from sh semantics — a ruling to take, not taken]: that re-bind is
guaranteed only while the alias set is known; on alias-expanding shells (dash) an alias minted
by the unknown file can rewrite the definition's name word itself, so below an unknown `.` even
the runtime binding is never a proof. What that costs [human, same day]: nothing at guard tier —
Dorc pins the author's BYTES and a guard re-measures live under `rul-guard-resolves-like-its-mutation`
(a site word that means something else below the havoc is the author's footgun, as with a
hand-written guard); ELISION below a blind load is out regardless, because the load is a total
wall, and every cwd-dependent decision (`[ -f ]`, relative `.`, slashless `$0`) is ⊤. This is the
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
hork tune web                # guard-tier at most: the havoc is a total wall, so never elide;
                             #   the guard re-measures live on the pinned bytes (the word `hork`
                             #   meaning something else here is the author's footgun)
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
- **`rul-load-head-is-exact-or-havoc`** [TYPED 2026-08-22, soundness-first re-cut of the
  earlier `rul-partly-dynamic-operand-is-a-set`] — the one criterion: Dorc derives authority
  from a sourced file (lifted vouches; bindings below the line; edits rendered into a copy)
  ONLY when it is certain the host's `.` loads exactly those bytes. Certainty has one source:
  the operand is EXACT — a pure function of controller-held inputs under the
  snapshot-is-the-program model — and emission (re-say or cwd-parity mirroring) makes the
  host agree. The re-say is the emission half of EXACT, never a second source of certainty.
  Every dynamic head (`$OPS_LIB`, `${LIB:-./lib}` — an env read, `$(find_config)`, an
  absolute host path) is a point-havoc: host influence accepted AS UNKNOWN, no authority
  claimed, hint toward an EXACT spelling. Two cells, no third: loaded-not-shipped files have
  exactly two honest user stories — host-owned content (`/etc/os-release`, a secrets file;
  host-specific BY DESIGN, never shipped, a controller digest meaningless) and a library
  already on every host (if it must byte-match the controller copy, shipping it IS the
  verification) — and neither wants verification machinery. STRUCK, by name: the
  snapshot-suffix SET with a resolving singleton (an engine selection forcing the host to
  load a file the author never named — `30Pb:fnd-possible-singleton-is-not-exact-selection`,
  confirmed unsound); the POSSIBLE ship-and-wall state (sound, but the singleton does no work
  that `mirrored-tree` does not); and the runtime-verified candidate (establish a digest on
  the controller, check at the `.` line, stop on mismatch — sound in isolation, killed by the
  other-phase cell: the probe has no book namespace so it cannot evaluate the operand as the
  apply's `.` will, and evaluating at apply makes the match/stop a LATE decision, which
  `rul-divergence-proceed`'s front-loading forbids). The set machinery survives only for
  globs with an EXACT head. Boondoggle fence [ACKED]: Dorc ekes analysis value out of static
  truth up to where it becomes intractable or dominates, never turning it into compilation
  output.
- **`model-symbolic-dollar-zero`** [ACKED 2026-08-22] — Dorc never reads `$0` from a shell.
  In analysis `$0` is the AS-GIVEN book path (never realpath'd — sh-parity under symlinks)
  with two live spellings: slash-bearing, and slashless with cwd = the load cwd (which is
  what the world means by slashless invocation, and what `dirname` returns `.` for).
  Authored expressions over it evaluate per spelling under ordinary POSIX expansion
  semantics — parameter expansion IS shell, not tool modelling. A load is EXACT iff every
  live spelling resolves to one snapshot file. **`rul-dead-spelling-is-not-unsound`**: a
  spelling under which the `.` is fatal (`${0%/*}` of a slashless `$0` is the whole word ⇒
  `book.sh/helpers.sh` ⇒ fatal) is DEAD, not unsound — nothing below it runs, so nothing
  below it can under-execute; it earns an off-ramp lint ("dies under `sh book.sh`"), never a
  refusal, and Dorc never TEACHES it (stewardship: no design route may push authors toward
  off-ramp-destroying shell, `KNOBS:kLANG`). **`rul-dorc-invokes-in-a-modelled-live-spelling`**:
  Dorc invokes what it ships in a spelling the analysis modelled as live (`sh ./plan.sh` from
  the generation root); this asserts nothing about sh's `$0`, only how Dorc spells its own
  invocations, and it is load-bearing because under cwd-parity the plan keeps the author's
  `${0%/*}` verbatim. Single-stream (`$0` = `sh`) has no mirrored tree and its `.` lines are
  pasted or refused, so it is outside this. Engineer-side caveat for every hint: `$0` inside a
  SOURCED file is the main script's path, never the sourced file's (sh has no `__FILE__`) —
  oracle files use a book-set root (`30I` §2.2), and the `$0` hints are admin-only.
- **`rul-static-predict-sites-loads`** [ACKED 2026-08-22 "seems like a winner"; unparks
  `ask-authored-pure-predict-may-site-loads` in its STATIC form] — the sanctioned path to a
  command substitution in a load operand. The engine never learns what `dirname` does; a
  stdlib author spells it in sh and declines the edges:

  ```sh
  dirname__predict() {
     [ $# -eq 1 ] || { printf 'predicts none unmodeled-arity\n' >>"${DREP_V1:-/dev/null}"; return 1; }
     case $1 in
     */ | //*) printf 'predicts none unmodeled-shape\n' >>"${DREP_V1:-/dev/null}"; return 1 ;;
     /[!/]*)  printf '/\n';             printf 'predicts stdout\n' >>"${DREP_V1:-/dev/null}" ;;
     */*)     printf '%s\n' "${1%/*}";  printf 'predicts stdout\n' >>"${DREP_V1:-/dev/null}" ;;
     *)       printf '.\n';             printf 'predicts stdout\n' >>"${DREP_V1:-/dev/null}" ;;
     esac
  }
  ```

  (DREP spelling per `30D:rul-predict-status-keeps-every-value` /
  `rul-predict-channel-defaults`: no status is a decline; Stdout is declined by default and
  must be POSITIVELY claimed for the load plane to consume it — a predict with unclaimed
  stdout yields ⊤ ⇒ havoc; the positive stream record is the COMPLETION witness and
  therefore TRAILS the modelled bytes, `30D:reject-partial-stream-without-completion`; edge
  arms STRAWMAN.) A predict is STATICALLY EVALUABLE when its
  reached body lies wholly in the pure decidable set and its argv is controller-known (the
  symbolic `$0` counts); the engine evaluates it at plan time, ON THE CONTROLLER,
  SPECULATIVELY, once per modelled invocation shape — the static half of the split the
  kind-owner roles already have (`disturbance_reaches`'s static line vs its `dpkg -L`
  line), no host, no capture lane, no runtime `$0`. Obligations: (a) runtime confirmation
  is owed at probe STANDUP (`mech-two-standups`), never through `30D`'s per-record OOB lane —
  `reject-missing-expected-confirmation` does not apply to a statically-consumed record;
  (b) this is a NAMED widening of `funcenv-reads-source-literal-plane-only`: the load plane
  now reads a value-prediction (`value-predictions`), a new trust edge at vouch tier — a
  wrong static predict resolves the wrong file and bites OTHER lines, attributed to the
  stdlib author; priced acceptable because `dirname` is POSIX-specified and the oracle IS
  the spec; (c) the `why`-chain cites the predict (claimed/derived); (d) a book function
  shadowing the word resolves to the book (existing frame law), the stdlib predict does not
  apply; (e) the decidable set grows BY NAME, license-review-tier each (`case`-over-known-
  string; substitution of a static predict; later subshell-scoped cwd for `$(cd … && pwd)`)
  — the fence is `dec-decidable-set-v0`'s, no slope; GNU-isms (`readlink -f`) decline ⇒ ⊤
  ⇒ havoc + hint, the fence working through ordinary declines. Outcome on the three real
  idioms: `. "$(dirname "$0")/helpers.sh"` ⇒ EXACT in every spelling (nobody pushed off the
  idiom the world already considers best); the `case $0 in */*) … *) here=. ;; esac` form ⇒
  EXACT; bare `${0%/*}` ⇒ EXACT-or-dead, accepted with the lint. This SUBSUMES the
  byte-shape-recognition carve (which would have violated the typed
  `rul-no-tool-modelling-in-the-load-plane` with an unattributed engine reading) and the
  capture-lane form of the parked ask. RULED: the author's `$(dirname "$0")` line stays
  VERBATIM and Dorc mirrors so it lands (`rul-rewrite-permission-is-derived`, below);
  soundness = POSIX conformance on inputs the oracle accepted, checked at probe standup.
  Sequencing: the computed-`.` parse-tier refusal is repaired to post-analysis first
  (`rul-floor-valid-text-never-parse-fails`).
- **Slashless operands** — `. helpers.sh` is a PATH search, not a cwd lookup; the cwd search
  was deliberately removed from the standard over trojan-horse concerns
  [A-posix-dot-builtin-2018], and the atlas measured it fatal with the cwd off PATH. PATH is a
  host read ⇒ unresolvable by the axis; a lint hint ("write `./helpers.sh`"). The same applies
  to a no-match glob's literal pattern when it carries no slash.
- **Globs** (EXACT head only; a dynamic head is havoc like any other) — SET-valued over the
  snapshot, ORDER-UNKNOWN (collation is the target's; the rail
  pins `LC_ALL=C` and measured ASCII order, which proves nothing about the host): members
  defining one name with different bytes WITHHOLD; a sole-member name is live. No-match
  sources the literal pattern, which the atlas measured as FATAL — a failed `.` ends the
  script even as the left operand of `||` (special-builtin semantics) — so everything
  downstream is unreachable, not merely unbound. Builds after `lane-loop-propagation`.
- **`ask-authored-pure-predict-may-site-loads`** — UNPARKED in its static form as
  `rul-static-predict-sites-loads` (above); the capture-lane (runtime) form stays declined.
  Probe-sourced loads: NACKED [TYPED; permanent law].
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
   refuse with the form named. **The set's membership is deliberately UNWELDED** [TYPED
   2026-08-22]: it stays variable and may grow as the spike discovers new scary exceptions in
   the modelling; what is ruled is that a set exists, that paste happens only inside it, and
   that top-level `return` is in it (**`rul-paste-excludes-non-subshell-return`** [TYPED "fine
   by me"; the standard settles it]). As currently understood: the `.`
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

## the-load-plane-stays-correct — rewrite permission, the blessed lift, and the two standups

The 2026-08-22 ruling that closed `choice-verbatim-or-re-say`, widened on the human's lead into
the correctness posture of the whole load plane. The lift mechanism is general in principle, but
`dirname` is its only entry and a second consumer is its own sitting; this section is about
loading.

- **`rul-rewrite-permission-is-derived`** [TYPED 2026-08-22] — Dorc may rewrite a book's `.`
  line only when the reference names its target EXPLICITLY (a literal operand; a literal-assigned
  book-set root) AND Dorc is already authorized to transform that target (a dorc-lang file it
  strips or bundles). Permission to generate or move content carries the implicit permission to
  re-point explicit references to it; nothing further is implied — a non-explicit reference would
  be rewritten on Dorc's reading rather than the author's, and an untransformed target grants
  nothing. Certainty is a separate axis: EXACT governs AUTHORITY (bindings below, vouches,
  shipping); explicitness governs REWRITING.

  ```sh
  . ./lib/foo.dorc.sh               # explicit + transformed ⇒ may re-point (bundle) or paste
  . "$(dirname "$0")/foo.dorc.sh"   # EXACT, not explicit ⇒ verbatim; the stripped copy is mirrored
                                    #   at the authored relative path so the author's own line lands
  . "$OPS_LIB/foo.sh"               # havoc ⇒ verbatim, nothing shipped, nothing claimed, hint
  ```

  Per form: tree-mirror ships every EXACT target (stripped if dorc-lang) at its authored path;
  tree-bundle additionally re-points LITERAL dorc-lang imports
  (`30Ng:rul-bundle-at-dorc-lang-boundaries` binds literal operands only); single-stream pastes
  an EXACT literal `.` under the exclusion set (a literal `.` below a `cd`/havoc is NOT pasted
  and NOT re-pointed: either rewrite changes which file the host loads, so explicitness alone
  never licenses a rewrite — the resolution must be EXACT too), and an EXACT-via-`$0` `.` — which cannot stay verbatim
  there, `$0` being `sh` — pastes the EXACT-resolved file or refuses the form; a havoc `.` stays
  verbatim in every form and never refuses one.
- **`mech-blessed-lift-to-literal`** [ACKED 2026-08-22] — the engine evaluates a predict at
  plan time, ON THE CONTROLLER, SPECULATIVELY — abstract evaluation of oracle sh feeding probe
  planning, for books not targeted at the controller — iff the reached arm is `P-static`
  (every construct in the decidable set). It may VERIFY that evaluation host-side iff the
  reached call-graph — transitively through frame-resolved function callees, splice-budgeted —
  is `P-blessed` (every command-position callee cheap-and-read-only, blessed BY NAME: the
  decidable-set entry is the blessing, never a class) and the family tool itself is blessed
  (engine-granted by name, override-immune). `P-static ⇒ P-blessed`, not the converse. The
  blessing is the engine's owned residue (`rul-probe-mutation-ownership-split`), not an
  inference about authored commands (`structural-vouch-only` untouched). A user may redefine a
  stdlib predict: the lift and the blessing survive exactly while the reached call-graph stays
  inside the sets; otherwise the load is a point havoc with a line-precise hint. No per-line
  guard of any kind exists: an EXACT `.` has, by definition, nothing above it that can move its
  resolution — a `cd`, a `PATH` assignment, a shadowing definition of the tool, a havoc `.`
  above each make it non-EXACT at analysis
  (`ANALYZER-NEEDS:an-load-exactness-reads-binding-state`); that is an obligation on the
  analysis, not on a guard.
- **`mech-two-standups`** [ACKED; stop-on-contradiction TYPED 2026-08-22] — PROBE standup (no
  tree yet): for every blessed-predict use, a three-way check on each modelled invocation shape
  (the abstract evaluation's input→output set under the funcenv at the site — the live `$0`
  spellings are one source of plurality, not the set): engine-static(body) = host(body) =
  host(tool). Any disagreement is an analytic contradiction ⇒ the phase is Refused for that
  host, before consent: we probe what the analysis found WANTING; we fact-check what it found
  FOR-SURE. One shipped artifact — the check at the top, abort on mismatch, and the probe may
  inline the verified literal below (its own `$0` is single-valued) — never a second exchange
  (`rul-repeated-probing-reviewed-before-design`). APPLY standup (tree shipped): artifact
  integrity — cwd parity, `$0` slash-bearing, every shipped file present and byte-matching its
  manifest; mismatch ⇒ stop before the first book line. The apply never inlines; the author's
  line runs the real tool as many times as written.
- **`rul-standup-is-tunnel-negotiation`** [TYPED 2026-08-22] — both standups are engine-owned
  integrity and fail-fast tooling on the capability-handshake tier
  (`ANALYZER-NEEDS:an-host-capability-handshake`), not plan lines: they spend no user
  attention, and Dorc reserves arbitrarily rich negotiation there at both phases. The blessed
  set is a ruled strict core of the language whose host-side checking is
  equivalent-modulo-perf-and-safety to "does this path exist on the host".
- Open, none blocking: the host spelling of the checks (scaffolding may lean on tools; start
  conservative); whether single-stream should hint when `$0` is used outside a pasted `.` (the
  author's cwd-independence silently downgrades there); a predicted value consumed OUTSIDE the
  load plane (`here=$(dirname "$0")` feeding a `cp` argv) is its own sitting — the load plane
  reads it, nothing else does yet; the exclusion set's `if`-body `.` cell (-GUESS pasteable).

## scheduling-truth — horizons are attention-calls

[TYPED 2026-08-22] An xfail horizon names a round past which *forgetting* the item is
unacceptable — a scheduled attention-call at which it is scheduled or punted again — never
the round it completes in. The `r31:*` horizons stand as minted; no re-horizon act; no LLM
effort on them. What IS r30 (kernel quiescence at its close): point-havoc and `${0%/*}` in
`lane-load-plane-precision` (havoc first; the alias cell read — it did not materialise for
the sourced-alias-then-book-funcdef shape); `lane-loop-propagation`; the planner lane;
`lane-influence-carriage` (stays; critical for reasons outside the conductor's horizon).
Inclusion, RULED (`ask-inclusion-in-r30` [ACKED 2026-08-22, "ack on your scheduling and
plan"]): `mech-acquire-and-ship-plain-sh` is r30, a small slice across the two lanes already
touching those files, after the runtime-dead suspicion is confirmed; `mech-splice-plain-sh-for-analysis`
and `mech-paste-plain-sh-single-stream` are punted behind one FORFEITS row
(`forfeit-plain-sh-inclusion-analysis`). **`rul-forfeits-carry-reds`** [TYPED 2026-08-22]:
from here on every FORFEITS row carries attendant xfails and/or e2e cases, so what is being
forfeited is encoded in sh and stays RED — this row already has them
(`p-x-book-code-source-is-inclusion` · `load31-punted-load-shapes`); the rule goes into
FORFEITS's header at its next edit.

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
| `p-x-load-operand-dirname-of-dollar-zero` · `…-cd-pwd-of-dollar-zero` | r31:book-load-acceptance | trigger names `ask-dollar-zero-command-substitution-path` (now `rul-static-predict-sites-loads`'s) |
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

1. `ask-computed-dot-degrades-to-a-wall` — subsumed by `rul-floor-valid-text-never-parse-fails`.
2. The single-stream exclusion set's membership beyond `return` — deliberately unwelded;
   grows during the spike (not a ruling to wait for).

Ruled 2026-08-22 and recorded above: `choice-verbatim-or-re-say` → verbatim, by
`rul-rewrite-permission-is-derived` + `mech-blessed-lift-to-literal` + `mech-two-standups` +
`rul-standup-is-tunnel-negotiation` (`the-load-plane-stays-correct`) ·
`rul-load-head-is-exact-or-havoc` (supersedes
`rul-partly-dynamic-operand-is-a-set`; singleton/POSSIBLE/verified-candidate struck) ·
`model-symbolic-dollar-zero` + `rul-dead-spelling-is-not-unsound` +
`rul-dorc-invokes-in-a-modelled-live-spelling` · `rul-static-predict-sites-loads` (the
sanctioned `dirname` path; unparks the static half of `ask-authored-pure-predict-may-site-loads`)
· `ask-inclusion-in-r30` (acquire-and-ship in r30; splice + paste forfeited with reds)
· `rul-paste-excludes-non-subshell-return` (top-level `return` excluded; set unwelded) ·
`rul-emission-is-the-umbrella-name` (emission ⊃ placement, layout) · `rul-forfeits-carry-reds`
· `rul-guard-resolves-like-its-mutation` (Dorc's own bindings only; the `command`-pairing
clause nacked) · `rul-munged-name-lifts-over-opaque-load` (the squat hole acked, narrowly) ·
`option-strict-defensive-emission` (lean; machinery stays general).

## parse-never-fails-on-floor-text

**`rul-floor-valid-text-never-parse-fails`** [TYPED 2026-08-22] — no text valid under the
floor (`posh` ∩ `dash`) is a PARSE violation, ever. Cross-shell semantic disagreement and
outside-the-dialect constructs are POST-analysis, PRE-network outcomes — a single pass still
yields diagnosis and aid that a parse crash-out throws away. The ruling is "build the necessary
MACHINERY" (it handles the worst case whatever the UX does), with the UX direction
CONSERVATIVE to start: the rejection/output stays NARROW — reject-but-stay-agile — so that a
later softening of the reject is a UX change, never a kernel rebuild. The UX decision is the
human's; only implementation details are the builder's. General truth, not specific to
parsing: machinery lands in r30 where possible, rulings stay narrow. For the computed `.`
(`fnd-computed-dot-is-a-whole-book-refusal`, a parse-tier defect to repair): fail-fast
post-analysis pre-network for now, or whatever is easiest if that is hard [TYPED lean]. This
subsumes `ask-computed-dot-degrades-to-a-wall`.

## review-adjudication-inputs — the conductor's stance on `notes/30Pb` (findings, not rulings; the human decides)

The opaque review (`30Pb`, nine findings; its quarantined half `30Pa` unread by the
conductor by law) weighed by the conductor's own estimation. DISAGREEMENTS first — those are
the human's highest-priority items, because the reviewers hold constraints the conductor
cannot see.

- **`30Pb:fnd-unknown-source-recovery-is-domain-specific` — PARTIAL DISAGREEMENT, the one
  that matters.** Agree that an unknown `.` havocs more than function bindings, and that the
  cheap domains should be modelled at v0: variables (⊤ already) · cwd (⊤ after ⇒ every later
  relative `.` is unresolvable) · shell options/errexit (⊤ ⇒ the CFG's exit edges go
  conservative) · positional parameters (⊤) · termination (May-reach, which is the safe
  direction for both elision and guards) · aliases and traps (the unenumerable tier, as
  today). DISAGREE with the last clause — "a vouch or guard is available only when the
  body's complete resolution environment is known to agree between measurement and
  execution": an unknown file that redefines a tool as a function is the same cell as the
  host's PATH resolving the tool to anything, which the oracle contract already places on
  the admin's side of the horizon (`rul-probe-mutation-ownership-split`; `an-host-as-adversary`
  — a hostile host can make any guard lie regardless). Adopting the clause would make
  nearly every verdict body unlicensable below any unknown source and hollow out
  point-havoc's value for the shape real books have. **RESOLVED by a narrowed review pass
  (human-relayed 2026-08-22):** the reviewer acks the core argument — static elision already
  stops at the wall, and an apply-time guard intentionally remeasures in the post-source
  environment, so probe and apply resolution need not agree. The review's first narrowing
  (pair-and-refuse a check spelled `command hork` against a site whose bare `hork` may be a
  function) was NACKED by the human: a verdict body may check through `command hork`,
  `not_hork`, or a file test — one class, referent-agnostic — and special-casing one spelling
  of it is surprising, uneven-floor behaviour. The ruling as it stands:
  **`rul-guard-resolves-like-its-mutation`** [TYPED 2026-08-22; reviewer's sentence,
  human-adopted] — *Dorc never interprets or compares what a convergence vouch checks. It must
  only ensure that its own movement or renaming does not create a combination of bindings that
  ordinary shell resolution would never have produced.* Concretely: (a) Dorc's own emitted
  definitions bind as pinned (`pinned-definitions-are-the-artifact's-binding`) — structurally
  above any definition vector, digest-defended below one (next bullet); (b) a guard runs in
  exactly the environment its mutation would — same frame, immediately preceding, body
  verbatim, no emission tier relocating the glue or rewriting any command word inside a body.
  What the body resolves *within* that environment is the author's judgment, and the engine
  never pairs, refuses, or constrains by the spelling of a command word. Footguns that exist
  in ordinary shell are not Dorc's to fix where fixing them would surprise or unlevel the
  floor (a lint may name them; lints do not drive design). Ordinary unknown-source effects
  remain ANALYSIS uncertainty (unsure ⇒ run); wrong-host, malformed, or lost-integrity
  evidence remains REFUSAL (`rul-integrity-failure-withholds-mutation`).
- **`rul-munged-name-lifts-over-opaque-load`** [TYPED 2026-08-22] — the acked hole
  (`gap-book-shadows-dorc-role-name`): Dorc's preamble hoists above the book, so a Dorc-munged
  name (`<name>_h<digest>`) lifted across an unresolvable `.` can be overridden by an explicit,
  intentional squatter of that digest name in the opaque file — sh is last-wins, and the digest
  is a probabilistic defence, not a structural one. It IS explicitly a danger. Ruled: Dorc still
  lifts over in exactly that case. Scope is exactly "Dorc-munged name, lifted across an opaque
  load" and no further — adjacent or non-matching mechanisms get their own examination sitting;
  the human's grounds are unenumerated here. Tripwire for re-examination: any eventual
  intra-host-influence-vector (human's spelling), thus far explicitly excluded.
- **`option-strict-defensive-emission`** [TYPED lean 2026-08-22; TBD, not built] — the
  engine already holds the machinery to emit perfectly per site (the probe lane re-emits per
  site; the planner enumerates `in-place`); choosing idiomatic, hoisted, attention-preserving
  emission by default is a product-level taste-and-UX ruling, not a capability limit. At
  explicit admin request — probably a broader 'strict'-flavoured mode toggling several options
  of this flavour — Dorc may emit maximally-defensively and close even the gap above. Keep the
  machinery general; never bake the default into it.
- **The controller-expectation / host-check pattern** [human-pointed 2026-08-22; a seed,
  not a ruling; the machinery is wanted, the product decision is not] — where host state
  enters a decision, Dorc already has one shape for it, and `30P`'s host-state questions
  (target-runtime sources; unknown-source effects; the exact artifact set) should mirror it
  for consistency: ESTABLISH a world and a set of expectations on the controller; CHECK on
  the host that the expectations match; and let the host's influence be strictly a boolean
  continue / do-not-continue — the host may make Dorc STOP ("mismatch, stop"), never change
  what Dorc does if it continues. Built instances: the per-host capability handshake
  (`an-host-capability-handshake`; the pipefail handshake, `276:rul-pipefail-emit-never`)
  and the admission trichotomy (`rul-admission-is-a-closed-outcome`: Refused returns before
  any plan carrying mutation authority). Needs its own UX-level and opaque review later.
- **`30Pb:fnd-possible-singleton-is-not-exact-selection` — AGREE; then SUPERSEDED by the
  human's soundness-first re-cut.** The finding was right that a singleton suffix-match says
  what the snapshot holds, not what `$LIB` means. The three-state amendment it prompted
  (POSSIBLE / EXACT / ENGINE-SELECTED) is withdrawn: POSSIBLE did no work `mirrored-tree`
  does not, and the model collapses to `rul-load-head-is-exact-or-havoc` — EXACT or havoc,
  nothing between. The only witness for EXACT-ness is authored text the controller can
  evaluate: a literal, a book-set root (`30I` §2.1), `$0` under `model-symbolic-dollar-zero`,
  or a statically-evaluable stdlib predict (`rul-static-predict-sites-loads`).
- **`30Pb:fnd-controller-source-and-target-source-are-distinct` — AGREE on the distinction;
  mild disagreement on "never by path syntax".** The class IS selected by an authored act
  today: a path relative to the load cwd is a controller-side candidate (acquired from the
  snapshot); an absolute path is target-runtime (never read from the controller — reading the
  controller's `/etc/os-release` as the target's would be the defect). That is spelling, not a
  second language, and it is `30I`'s existing cwd-parity rule. ACKED as a lean/gloss
  [human 2026-08-22, not a hard ruling]: each absolute-path cell is AUTHOR SPEECH — that is
  what "absolute" means — and Dorc can neither fix nor meaningfully interpret one. The only
  cells that matter now (pre-lint; a "shellcheck with more world-knowledge" scope-creep is
  out of `30P`'s scope) are those where Dorc MUTATES.
- **`30Pb:fnd-emission-legality-covers-all-shell-state` — AGREE; sharpens tier one.** The
  hoist condition "no book CALL above the `.` names a bound name" must read "no book
  OBSERVATION OR MUTATION" — `command -v helper`, `type`, `unset -f`, `alias`, and variable
  reads count. The decidable-set machinery already sees `command -v <unit-defined-fn>`; the
  value plane enumerates variable reads. File-level assignments hoist only when no book
  statement observes the name; otherwise the whole closure stays in place. OPEN, for the
  planner lane to confirm: whether today's preamble hoist of oracle constants can already
  shadow a same-named book variable (a latent hole if so).
- **`30Pb:fnd-emitted-names-need-freshness-and-hygiene` — AGREE; mostly already law.** The
  BARE-IF-SINGLETON census over emitted ∪ book top-level names is the injectivity proof for
  functions; add detect-and-lengthen for digest collisions (cheap). The header-only boundary
  is exactly `rul-munge-oracle-names-only`'s as-built scope: definitions whose every reference
  is engine-emitted (role functions invoked by guard/probe scaffolding); helpers referenced
  from authored bodies need alpha-rename (reserved) or stay in place / withhold — which is
  what `forfeit-helper-plurality-withhold` does today. State the boundary in the plan.
- **`30Pb:fnd-reviewed-artifact-is-one-exact-set` — AGREE; 30P RESERVES, does not own.** One
  immutable artifact-set identity (paths, bytes, provenance, policy, target cwd, generation)
  that apply consumes and rejects on mismatch is the approval/apply design's
  (`rul-attribution-is-controller-minted`'s "saved approval" re-entry trigger). 30P reserves
  a mandatory identity projection on `ArtifactSet`.
- **`30Pb:fnd-path-spelling-resolution-and-content-identity-differ` — AGREE; largely typed
  already** (`LoadAccount` occurrences keep spelling + locus; canonical keys; snapshot sha).
  Builder check owed: `core::loadpath`'s lexical `..` handling versus the real open under
  symlinks/case-folding.
- **`30Pb:fnd-glob-order-needs-whole-program-meet` — AGREE; my collision-only rule was too
  narrow.** Order-unknown means a universal meet over every order of the members' load
  programs (members can `unset -f`, assign, `cd`, `exit`); closed for dorc-lang members
  (the load-program vocabulary), opaque for plain-sh members ⇒ wall. A constraint on the
  deferred glob work.
- **`30Pb:fnd-dot-source-remains-an-execution-frame` — AGREE.** The splice-as-body-called-once
  is an implementation technique; a first-class source frame keeps `return`, errexit context,
  positionals, traps, and termination observable. `30L`'s execution/region identity split has
  room for a `SourceFrame` identity.

## ledger-updates-owed

None held by this plan. Register and steering state is `30O:register-and-steering-debt`
(authoritative). The two refusal texts this plan names (`30Ng:rul-piped-stdout-carries-a-full-plan`;
the single-stream refusal) reach the prose queue through the ordinary `[unwritten:]` path
when their codes are minted; `mise run prose:census` is the instrument.
