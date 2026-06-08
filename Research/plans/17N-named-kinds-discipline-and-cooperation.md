# 17N — Named kinds: the discipline, and the cross-oracle channel (round-17 K1+K2 reunion)

> **What this is.** The reunion `175` (K1, the cross-oracle kind-channel) and `17H` (K2, the minimal
> type-discipline) each forward-pointed to and were firewalled from doing: one coherent story for how Dorc
> spells, analyzes, and reconciles a *named kind*. It folds in the foundational relational frame
> (`094`/`095`), the two-axis kill reasoning (`17B`), the open decisions the spike owes (`16Q`, `151`), and
> the human's round-17 corrections (recorded inline).
>
> **The one-object thesis (why a reunion is needed).** A kind is **one object seen from two sides** — K2's
> *type* (a declared name carrying a state-model) and K1's *identity-anchor* (the thing two independently-
> authored oracles must agree names the same thing). The Seam (below) is where they fuse.
>
> **Structure.** Part I = *spell a kind and analyze it* (single-oracle; K2-heavy). Part II = *cooperating —
> reconcile kinds across authors* (cross-oracle; K1-heavy). The Seam fuses them. A single-oracle analyzer
> task can stop after Part I; a cross-oracle / `kOOB` / coordination task can skip to Part II + Seam.
>
> AI-generated; confidence-marked (+SURE / ~SUSPECT / -GUESS / --WONDER). Leans are marked; the still-open
> decisions are listed at the end. Trust repo-root `DESIGN`/`KNOBS`/`README`/`IMPLEMENTATION` over this.
> **Round-17 adversarial-crosscheck findings (all of them — folded, fought, deferred): `notes/17O`.**

> ### Provisional spelling ruling (BOUNDED to this paragraph — do not thread through the body)
> **Whether Dorc has a type-system at all is UNSETTLED** (`kTYANNOT`); everything here is downstream of that.
> This round provisionally concludes there is likely **no satisfactory sh-native spelling** for the hard
> type-claims an oracle must make — "the 2nd arg of `frobctl --frock` is a `Wombat`"; "…and it transitions to
> `Wombat#frocked` for the rest of the script." That is **bad news** (lowers differentiation from
> YAML/Ansible; raises the cost of our best-effort downsides).
>
> Provisional lean, *heavily* strawman: a minimal **TS/Flow-style inline type-annotation** (env-vars too
> DSL-y / half-assed). Illustrative, NOT locked:
> ```sh
> frobctl.check() {
>    local w : com.frobber.Wombat{defrocked,frocked} = "$1"   # …then e.g.  0) return 0 : "$w" is Wombat#frocked
> }
> ```
> **The huge caveat that keeps this provisional (round-17 crosscheck, verified live — `17O` F-OFFRAMP):**
> this spelling is **not a behavioral no-op, and it breaks the off-ramp weld.** Under stock `dash`, `local w
> : Foo = "$1"` aborts (`local: :: bad variable name`); under `bash` it silently leaves `w` empty and returns
> 0 (corruption, not a crash); the dotted name fails `dash -n`; un-stripped `{a,b}` brace-expands under bash
> but not dash. So the off-ramp — DESIGN's "absolutely trivial" `ssh host 'dash -s' <script` — would require
> *running Dorc's stripper first*, and that "stripper" is really a correctness-critical source-to-source
> transpiler (it must delete `: T` **and** collapse the `=` spacing). 17N itself treats "breaks the off-ramp"
> as a **welded kill** (kill-8, HM) — so this spelling fails 17N's own bar.
>
> **The real choice is a knob — `kTYANNOT`:** *annotate inline, directly on a command argument* (ergonomic,
> intuitive, significant-meaning-in-place — but **off-ramp-hostile**: sh has no inline-comment form) **vs.**
> *pull every typed value out of argument-position and annotate it on an end-of-line `# …` comment*
> (shellcheck-style: ugly, verbose, restructures the script — but **off-ramp-free**, the comment is inert
> under any shell). A genuinely horrible trade; the eol-comment plan-B is the only known off-ramp-clean
> carrier of the hard claim, and it is also DX-vomit. See `kTYANNOT`.
>
> On `dq-entity-algebra` (§4): the earlier "lower-stakes, leave fuzzy" is **retracted** — the crosscheck
> showed the ≥enum floor + 16Q's `q1-precision` gate force the structured-vs-flat call *now* (`17O`
> F-ALGEBRA); the current lean is a small **recursive, JSON-adjacent** structure (§4), still strawman. The
> body below maps the **sh-native** options (the floor + the ideal where achievable: blessed vocabulary,
> idiomatic guards); this paragraph is the escape-hatch for the irreducible *declared* claims sh cannot carry
> — exactly what `kTYANNOT` is unsettled about.

---

## 0. The frame everyone inherits (true going in)

- **F0 · relay + adjudicate, never declare or infer (the stance — `095`).** The "symbol-grounding problem"
  is the *motivating* chicken-and-egg (DESIGN "Inference limitations"); Dorc sidesteps it by staying
  **referent-agnostic**. The *declaring* is the user's (both the oracle author and the consumer). Dorc does
  exactly two things: it **relays** (carries those claims and reports them back to the human) and it
  **adjudicates** — it acts (lifts/replaces a command) only when all claimants about a piece of state
  *agree* on its type and that agreed type licenses the lift. It never declares and never guesses a
  referent. To enforce "foos don't bar," Dorc "need not know what foos/bars *are* in the real world, only
  keep the promise to the person who does."
- **F1 · the named kind, 3-place, lift-not-run (A1; universal — `175` Result A).** Identity binds to a
  **named kind**, never a shared argument-token; the relation is **3-place** `(kind, provider, verb) →
  effect`. The mainstream convergent design — Debian virtual-packages [A-debian-policy-relationships-2024],
  `update-alternatives` [B-update-alternatives-man-2024], RPM `Provides`, Ansible `package:`
  [B-ansible-package-module-2024], Puppet RAL [B-puppet-ral-resource-2024], K8s `kind`, Terraform
  resource-type. The novelty was never the shape, only the *spelling*. The oracle is plain sh the analyzer
  **lifts statically**, never sources or runs.
- **F2 · the ≥1-anchor floor (`094` g5 / `095`).** Tell Dorc *one* anchor per kind and guard-structure
  *propagates hints* across the script (subject to F3); anything with no reachable anchor is **⊤ ⇒ run**
  (the safe floor). The chicken-and-egg is the *proof the floor is non-empty*, not a wall. (Earlier draft
  claimed command-shape made this floor nearly vacuous — **retracted**; see F7.)
- **F3 · co-reference is two questions, both harder than they look.** The shared-token frame splits:
  - **the type question** — *what category is an operand?* (`nginx` is a `package`.) See F7 for the only
    coordination-free lead, heavily hedged; otherwise declared.
  - **the instance question** — *which cell does it name, and is it the same cell as that other operand?*
    Structure gives at most a **may-grade hint**, never a guarantee. Same-string-in-two-places is *not*
    same-intended-instance, and a guard's operand is *not* its body's operand: `if ! dpkg -s
    conflicting_package; then apt-get install something` probes one cell and establishes a *different* one.
    So co-reference ≠ the probe/establish *linkage*. (This downgrades `094` g1 — §3.)
  Across two oracles a shared token may also be a coincidental name-collision. ⇒ the **named kind** is what
  bridges *different commands/tools onto one cell* (Part II) — the part that genuinely needs cross-author
  declaration. The residual hard kernel ("which cell, same cell?") is entity-uniqueness/identity = **SF-1**,
  welded-undecidable: identity is **declared**, never inferred; the **probe confirms a cell's *state*, not
  its identity**.
- **F4 · the `kOOB` redline + X3 (`151` X3).** Config is spelled in sh / library-code — no YAML, frontmatter,
  pragma, comment-parsing. The redline is *configuration form, not metadata transport*: an analyzer lifting
  author-written declarations from oracle ASTs into an **engine-internal index keyed by leaf-id** does not
  breach it. Hard constraint: a **1-place sh function-name cannot carry the 3-place relation** (sourcing
  apt's then brew's oracle *clobbers*); the handle must be a **lifted datum**, not a function name. *(Where
  sh can't carry the claim at all, the top-paragraph annotation escape-hatch is the provisional fallback.)*
- **F5 · optimizer, not type-checker *for plain sh*; a graded right to reject only on the annotation layer.**
  Dorc never rejects working sh: unknown ⇒ ⊤ ⇒ run. The reject-power is a **trust-spectrum**, not a switch:
  it does **not** infer-upward from partial context to a "reject," and does **not** reject on
  insufficient-context-to-infer (both stay ⊤-run — the low end). But once the user writes an *annotation* they
  are Doing A New, Dorc-specific Thing, so a denial there is **deferred off the day-1 path** (a gradual
  gradient, not a cliff): Dorc may reject on **ill-formedness** (a parse/lint concern), and ~SUSPECT on
  **typing conflicts** (two oracles asserting incompatible types for one handle — the same "it's-a-new-thing,
  the error is earned" argument). What it must never do is reject *plain working sh*, or pick a *correlation's*
  safety-direction (spine-1). (`17O` F-REJECT; fuzzy, much else first.) Forgiving lineage still governs
  (success/gradual/soft/pluggable typing, typestate, small effects, occurrence typing); welded out
  (`kVERIFY`): HM, dependent types, Cousot-Galois.
- **F6 · kill-safety is TWO-AXIS (`17B` — load-bearing throughout).**
  - **axis-depth** — *how hard the analysis thinks*. Kill ⇒ less certainty ⇒ run ⇒ **over-run**
    (`IMPLEMENTATION` priorities 2–3). **Kill-floor holds; judge on value.**
  - **axis-fidelity / coordination** — *what states exist; when one oracle's state-test discharges
    another's dependency*. Kill = coarsening ⇒ confident-wrong sameness ⇒ over-correlation ⇒
    **under-execute (priority-1, worst)**. Failure is **non-directional**, so "simpler = safer" is false.
    **Kill-floor does NOT hold; judge on the fidelity-floor + MUST-grade-to-correlate (§9 / Seam).**
- **F7 · command-shape — a hedged design-note, not a foundation.** --WONDER there *may* be a way to
  establish *some* type-equality from a command's binary + identical static argv-shape (two `hork --sniggle
  X` / `hork --sniggle Y` *might* share a *type*, not an instance) **without cross-author coordination**.
  But every command parses args differently (the argparse zoo); this is untrodden; and if deriving it
  requires the oracle to declare *how* the command parses args, it collapses to the cost of just declaring
  the type — **no net win**. Deliberately under-sold: a lead for the next spike, not a floor, and it does
  **not** vacate F2. (Open: how "same binary" + "same flag-grammar" are even decided — ties to `16Q`
  `q1-flaggrammar`, `151` "footprint isn't a one-liner".)
- **F8 · the analyzer must handle real scripts, deeply (welded).** Constant-propagation, interprocedural
  analysis, variables, and heredocs are **welded must-handle-correctly** — a core goal is that writing
  *better*-structured scripts never sheds Dorc value (DESIGN: make the happy-path the easy-path). The
  **sole** punted construct is `eval` (and related dynamic spellings; shell values are not first-class
  procedures, so there is genuinely nothing higher-order to chase — see kill-4). The cheap-on-scruffy-input
  optimizer is **known-insufficient** for the high-value careful-oracle audience (`151` X4), so full
  flow-analysis is required, not optional; the next spike builds it.

---

# PART I — Spelling a kind, and the discipline that analyzes it (single-oracle)

## 1. What a kind *is* (the operational definition)

- **A *type* (kind) is a namespace/category of intercomparable shared-state cells; an *instance* is a
  channel — one cell.** The file at `/etc/nginx/nginx.conf` is one instance; everything anywhere (any
  script, any analysis-unit) that reads or writes that path observes the same global state, so they
  coordinate *through* that shared cell. The type (`filesystem-entry`) groups cells that are meaningfully
  comparable — "do two paths reach the same file?" is a real question beyond string-equality — and that
  share probeable/mutable states (is-a-file, is-a-directory). **Naming the type buys intercomparability +
  shared structure/states.**
- **The type-system is declared over *pointers*.** The string a type is attached to (static `"nginx"` or a
  runtime `$path`) is a *pointer*, one indirection from the cell. The type describes the **pointed-at
  channel** — not the string's text, not the cell's contents. So Dorc types keys by *what they resolve to*.
- **…but not *all* types are indirect (the minority direct class).** A genuine class of intercomparable
  values *is* the annotated stringy value itself, no shared-state indirection: at least **direct-string**
  (content-comparable), **symbolic/interned key** (minted-at-use, only meaningfully comparable to *itself*,
  structureless, incomparable to all other strings — a nonce/gensym), and **the args-array** (`$@` — POSIX
  has only the one array, but it is still a structured type). Whether these are one type or several, and how
  subtyping relates them to the indirect majority, is left open. (Most types are pointers-to-channels; a few
  are direct.)
- **inc-8 · nominal, not structural** [contrast [B-typescript-design-goals-2020]]. A kind is a *declared
  name*, not inferred-from-structure — structure-inference *is* grounding (F0), which Dorc refuses (hence
  kill-6).
- **inc-S · a kind carries a ≥enum state-model** [B-aldrich-typestate-oriented-2009]: states + transitions +
  optional state-specific data. The **fidelity-floor is set by the world** (`17B` F5): the model must
  distinguish ≥ the entity's real *mutation-gating* states. A systemd unit forces ≥3 (installed / enabled /
  active), so **boolean is below-floor** — the degenerate default. Below the floor ⇒ over-correlation ⇒
  under-execute (F6).
- **inc-7 · the effect-map ≡ a typestate transition-table** [A-lucassen-gifford-effect-systems-popl-1988] +
  [B-aldrich-typestate-oriented-2009]. `(provider, verb) → {establish, kill}` *is* "this verb transitions
  the kind's state." One mechanism, two readings — build it once.
- **inc-10 · open/closed-state framing** [B-garrigue-polymorphic-variants-1998]: may a provider add a state;
  width-subtyping = handle a subset. Take the framing; the tag-reuse footgun (same tag, different meaning)
  is the Seam.

## 2. The forgiving discipline — what to keep, what to kill

**spine-1 · the safety-direction is the ORACLE's, not the tool's (`17H` B; the keystone of Part I).**
Forgiving type systems that *bake* a safety-direction can only do so because they have a **uniform free
runtime backstop**: Dialyzer "never cries wolf" because Erlang is runtime-type-safe
[A-lindahl-sagonas-success-typings-2006]; gradual typing's add-direction safety is enforced by runtime casts
[A-siek-refined-criteria-gradual-typing-2015], the very machinery Takikawa's performance "death" kills
[A-takikawa-sound-gradual-typing-dead-2016]. **Dorc has no uniform free backstop** (re-run = `over-execute`,
priority-2, non-uniform) and Bracha says *don't let any layer depend on the type system's correctness*
anyway [B-bracha-pluggable-types-2004] (= **tenet-0**). ⇒ Dorc's machinery must **not pick** a correlation's
safety-direction; the **oracle declares it**. Absence ⇒ no correlation ⇒ ⊤ ⇒ run — the floor, not the tool
choosing.

**The include / kill map** (`[axis]` per F6; prior-art bracketed; one-line why):

INCLUDE — *postures & properties of the forgiving lineage:*
- **inc-1 · success-typing polarity** `[posture]` [A-lindahl-sagonas-success-typings-2006] — report only
  *definite* clashes ("never cry wolf"); unknown ⇒ ⊤. The optimizer-not-checker stance.
- **inc-2 · the gradual guarantee as the no-cliff law** `[law/test]`
  [A-siek-refined-criteria-gradual-typing-2015] — adding/removing precision must not break a working program.
  **Asymmetric for Dorc:** *remove* is a real guarantee (drop an oracle ⇒ floor ⇒ still runs = off-ramp);
  *add* (a wrong oracle caught loudly) is best-effort only — cast-free, so a wrong oracle is uncaught (the
  Seam's hazard). Usable as a differential test.
- **inc-3 · pluggable / no-runtime-effect** `[contract]` [B-bracha-pluggable-types-2004] — the
  oracle-is-a-behavioral-no-op contract (DESIGN "Contract & DX").
- **inc-4 · soft-typing's static/dynamic split** `[division]` [B-bracha-pluggable-types-2004] —
  can't-prove-safe ⇒ insert a runtime check, don't reject = ⊤-run + probe.
- **inc-5 · the stub-corpus governance model** `[governance]` [B-definitelytyped-governance-2020] +
  [B-pep-561-distributing-type-info-2017] — decoupled community stubs, **machine-enforced not
  author-trusted**, **tiered by impact** (= `effort-allocation`), with a **declared precedence order**.
  Real-world proof the oracle-bootstrap loop scales (→ §9 opt-4).
- **inc-6 · occurrence-typing guard-lifting** `[spine]` [A-tobin-hochstadt-logical-types-2010] — a guard's
  test refines the kind's *state* per-branch (the narrowing spine, §3). **Note (post-`094`-g1):** narrowing
  is licensed once the operand's kind/instance is *known*; the guard does not establish the instance-link
  itself (F3).
- **inc-S / inc-7 / inc-8 / inc-10** — carried from §1.
- **inc-9 · coherence as a CONTRACT** `[fidelity]` [A-wadler-blott-ad-hoc-polymorphism-1989] — kind = class,
  oracle = instance; instances must *agree* for meaning to be well-defined. Dorc can't *enforce* it (never
  rejects plain sh) ⇒ a kind-owner-declared contract + best-effort CI lint. **The Seam — Part II.**

KILL — *degrades to the floor / welded out:*
- **kill-1 · NOT a kill — re-cast as the may/must certainty mechanic** `[depth]`
  [A-lindahl-sagonas-success-typings-2006]. The forbidden thing is narrower than "inference": it is
  **corpus-distributive guessing** (across *unrelated* programs) used as a *skip-license* (`094` g6 / `095`
  f27). That is at most a **linter hint**, never a probe/apply skip — *and even "never" the human holds
  unsold (no counterexample either way; flagged, not asserted).* What is **kept**: certainty-tracked
  derivation routed by setting — three levels (no-info / some-but-not-enough / enough-to-be-sure) × three
  settings (linter / probe-compiler / apply), each with its own threshold. Crucially, **coherence over the
  single program-unit being actively compiled/applied is not corpus-mining** — it is wanted and can reach
  must-grade. (This is the MUST/MAY mechanic, `096`, not a kill.)
- **kill-2 · consistency-relation cast/blame machinery** `[depth]`
  [A-siek-refined-criteria-gradual-typing-2015] — keep the static `⋆`-consistent idea (= ⊤); kill
  cast-insertion + blame (we never check or cast).
- **kill-3 · the sound-gradual *performance death*** `[depth]` [A-takikawa-sound-gradual-typing-dead-2016] —
  the death *is* mandatory boundary-check cost; Dorc is cast-free, so it's absent **by design**.
- **kill-4 · Koka *row-polymorphism* (keep the floor; drop the flat-map gloss)** `[depth]`. **Keep the
  floor:** Dorc has **no higher-order effects** — shell values aren't first-class procedures, so there is
  nothing for effect-polymorphism to range over (the sole caveat is `eval`, punted, F8). So kill the row
  apparatus. **But don't oversimplify:** the effect model is *not* a flat 2-element `{establish,kill}` map —
  it's flow-sensitive typestate (the same wombat is `defrocked` before a check, `frocked` after), which we
  **keep** (inc-S + inc-6).
- **kill-5 · typestate *enforcement*** `[depth]` [B-aldrich-typestate-oriented-2009] — rejecting operations
  invalid in the current state is dropped (never reject). **Keep the state-distinctions (fidelity), kill the
  protocol-enforcement (depth)** (`17B` re-split). Its precondition (uniqueness) is SF-1 → the Seam.
- **kill-6 · structural typing for kind-identity** `[fidelity-via-identity]`
  [A-siek-refined-criteria-gradual-typing-2015] §5.5 + [B-typescript-design-goals-2020] — structure-tests
  break the gradual guarantee; structure-inference = grounding (F0). Stay nominal (inc-8).
- **kill-7 · *unbounded* subtyping lattices** `[depth]` [A-lindahl-sagonas-success-typings-2006] — kill
  unbounded (`kCONTEXT` flat-domain), **keep** the bounded any/none + finite-union-widened lattice (where the
  ≥enum states sit). *Not a blanket kill.*
- **kill-8 · HM / full inference** `[welded]` [B-bracha-pluggable-types-2004] +
  [A-lindahl-sagonas-success-typings-2006] — restrictive + rewrite-hostile (breaks the off-ramp); welded out.
- **kill-9 · dependent types / Cousot-Galois full soundness** `[welded]`
  [B-livshits-soundiness-manifesto-2015] — no realistic whole-program analysis achieves full soundness.

POSTURE: **pos-1 soundiness** [B-livshits-soundiness-manifesto-2015] — sound core, *documentedly* unsound on
the hard subset. **pos-2 TypeScript's unsound-by-design non-goals** [B-typescript-design-goals-2020] —
erasable, productivity-over-correctness (diverge: TS structural, Dorc nominal).

## 3. Narrowing — how the analyzer reads *one* oracle's kinds

- **The idempotency guard carries hints, not links (`094` g1, downgraded this round).** `094` g1 read "the
  guard's shared literal argument *is* the entity-link." The human's counterexample downgrades it: `if !
  dpkg -s conflicting_package; then apt-get install something` is ordinary, and a guard's operand need not
  be its body's operand — so guard-structure does **not** establish that the body acts on the probed
  instance, and a shared token (even a shared variable) is a **may-grade hint**, not a must-grade link.
  Occurrence-typing narrowing (inc-6) is real but operates *downstream* of a known kind/instance; it refines
  *state*, it does not manufacture the *identity*.
- **Fact-centric, never command-centric (`16Q` §4).** The anchor probes a **fact** ("does `kind:entity`
  hold?", e.g. `dpkg -s nginx`), never dry-runs the mutator. A command-centric `mycmd.check() { mycmd
  --dry-run "$@"; }` makes the named-kind index decorative in the skip path.
- **The forced self-vouch (DESIGN; `16Q`).** No analysis decides whether `mycmd --dry-run` is probe-safe, so
  an oracle **vouches for its own command's inertness by existing** (the accepted-unprovable axiom). A static
  first layer can still certify the probe touches no *modeled* mutation; the oracle's own declared command is
  accepted + sandbox-contained, never refused.

**Illustration — the *compiled probe* (grounds the round-2 probe-model discussion; `17O` R2-PROBEGATE).** The
probe phase is not the book; it is *compiled from* the book's CFG — each potentially-mutating command is
replaced by an oracle-vouched read-only check (the oracle *intercepts*: `id__check` ships and replaces `id`)
or omitted; independent checks are dispatched **concurrently** (what "make them read-only" buys); output is
out-of-band (per-leaf freeform → scratch files; gating verdicts on a *separate* lane — the GitHub-CVE lesson,
P6). Dorc lifts the oracle *bodies* + (only where a probe is valid/inert solely under a prior guard) a minimal
CFG fragment — never the book's *contents*, so it never inherits the book's `trap`s (`17O` R2-TRAP):
```sh
D=${DORC_SCRATCH:?}; V=${DORC_VERDICT:?}                  # scratch dir + verdict lane (separate channels)
id__check() { command id "$@"; }                          # oracle interceptor, shipped + replaces `id`
emit() { printf '%s\trc=%d\n' "$1" "$2" >>"$V"; }         # verdict off freeform; %d, never %n
getent group app                >"$D"/p1.out 2>&1; emit p1 "$?"   # independent ⇒ dispatched concurrently
id__check -nG deploy            >"$D"/p2.out 2>&1; emit p2 "$?"
systemctl is-active --quiet app >"$D"/p3.out 2>&1; emit p3 "$?"
```
A probe-gated branch is resolved by *running the read-only probe for real* (so Dorc, unlike Ansible
check-mode, is not blind past a result-gated guard); a gate on a *mutation's* result is the run-delta residue
(§4 / `17O` R2-CHANGEDELTA). Full strawman + the CFG-preserved variant:
`notes/17x-strawmen/adversarial/compiled-probe.straw.sh`.

## 4. `dq-entity-algebra` — the shape of *one* kind's state (NOT deferrable; `17O` F-ALGEBRA)

- **Floor (settled):** **≥enum** (boolean is the degenerate default; `17B` F5). Below ⇒ under-execute.
- **Not deferrable (round-17 crosscheck correction; the earlier "leave fuzzy" is retracted).** 16Q §3 gates
  the next spike's keystone (`q1-precision`) on settling flat-vs-structured *on paper first* ("wrong shape
  re-keys every transfer function and the substrate"), and the ≥enum floor's own canonical example forces
  structured: a systemd unit's `#enabled` and `#active` are **independently** mutation-gating (`enable --now`
  writes both; `is-active`→true must not discharge an unmet `#enabled`), which a flat key or a single
  exclusive enum cannot hold. The deferral was empty across the design-range.
- **Current lean (strawman): a small recursive, JSON-adjacent entity structure.** Keys with values; a
  **present key with an omitted value is boolean `true`** (`wombat{defrocked,frocked}` = both hold); an
  inline `!`-pun for false (brevity); a value may be any tracked type — a **direct** type (string/key), a
  **stringy handle to another kind**, or a **nested struct**. Bias: **under-coordination, over-completeness**
  — authors name what they know, consumers match only the depth they need, and *absent ≠ asserted-false*
  (the carry-vs-compare split, C6, made structural). The *shape* (recursive · default-true-presence ·
  matchable-at-depth) is the lean; the exact grammar is open.
- **The strong-update dependency hands off to the Seam.** Strong (overwrite) vs weak (accumulate) is gated by
  entity-**uniqueness** [A-foster-flow-sensitive-qualifiers-2002]; the structured key is precisely what lets a
  transition overwrite `#active` without touching `#enabled`. Over-coarsening manufactures false uniqueness ⇒
  unlicensed strong-update = over-correlation. Uniqueness is SF-1, undecidable — the *identity* half (which
  cell) is cross-oracle (Part II + Seam), not Part I's.

---

# PART II — Cooperating: reconciling kinds across independent authors (cross-oracle)

## 5. The problem restated cross-oracle, and the two regimes

Two oracles A, B authored **independently** must converge on one **opaque kind-handle + state sub-data** so a
fact B *probes* discharges a precondition A *declares* — Dorc routing it, understanding **none** of the
semantics. This is F3's hard case: different commands/tools touching the **same cell** can't be bridged by
command-shape (F7) or by token co-reference (different programs, different slots, possible name-collision), so
identity must ride a **declared named kind**.

**Two regimes (`175` Result B):**
- **Within one authority** — in-band, self-paying, many consumers: Debian `Provides:`, systemd `Alias=`,
  `update-alternatives`.
- **Across authorities** ("apt's nginx ≡ brew's nginx") — **no in-band self-paying form exists**; the world
  uses a central curated index (repology; CISA's funded authority [A-cisa-software-id-ecosystem-2023];
  `owl:sameAs`, famously *misused* [A-halpin-owl-sameas-2010]).
- **The poison (`175` §3):** *packages* drag cross-*manager* real-world equivalence onto the clean channel.
  Per-token identity is cheap; **grouping into a named kind is the unsolved part**, answered by a thin agreed
  anchor, not inference. (If a future feature needs cross-provider *sameness*, not just per-provider naming,
  `owl:sameAs`/CISA's "grouping is unsolved" bite harder — unsettled.)

## 6. The channel mechanism — how two oracles converge on one handle (`175` Result C; PLT-free)

- **C1 · identifier ⊥ matching, consumer-driven.** Separate the *identifier* from the *match rule*; the
  **consumer** picks match-depth (BCP-47 tag vs RFC-4647 range [B-w3c-language-tags-2024]; InChI full vs
  layer-match [C-inchi-wikipedia-2026]; Pact contract-by-example [B-pact-cdc-docs-2022]). A's precondition is
  a *pattern* matching B's handle at the depth A needs.
- **C2 · reverse-DNS = the most sh-native handle, X3-clobber-free** [C-reverse-dns-notation-wikipedia-2026].
  Root the handle in *existing DNS*, not a new registry; universal convergence (Java, UTI, D-Bus, Flatpak,
  AT-proto). A plain string → `net.example.wombat` as a lifted datum, zero registry (F4-legal). *(If carried
  in the annotation escape-hatch rather than a bare sh datum, the reverse-DNS shape still applies.)*
- **C3 · dimensionality minimal-but-extensible** — flat + optional layers, never a mandatory schema (= §4 /
  top paragraph).
- **C4 · cross-party matching *forces* a thin coherence standard.** InChI needed a "standard layer-set"
  before independent ids matched — the *compared* dimensions must be agreed; private sub-data may ride. **=
  inc-9 (coherence) in the field — the Seam.**
- **C5 · self-describing reduces but never removes the anchor** (multihash [B-multihash-multiformats-2024];
  dynamic UTIs [B-houghton-utis-2012]) — the ≥1 anchor is irreducible (F2).
- **C6 · carry-vs-compare** (semver `+build` carried-but-ignored [B-semver-spec-2013]) → a handle may carry
  provenance it doesn't match on (the "in-band-but-ignored" half of the §4 lean).

## 7. The blessed-vocabulary shortcut — Result D (`175` §5)

The purest "spelled in sh": for a **blessed, bounded** vocabulary, a command execution *names the kind
itself*. `getent <database> <key>` [B-getent-man-2024] — arg-1 *is* the kind (NSS: `passwd`/`group`/`hosts`/…),
read-only, rc = 3-outcome fact. The kind is read off a line the author *already writes*, zero extra spelling.
Pattern is universal (positional kind-first / `-t` flag / arg-is-namespace-member; full catalog `notes/174`).

- **Bootstrap implication — tempered (human).** "Bless ~40 commands → hundreds of kinds" is **overstated**.
  A blessed probe helps *only* if it is in the **non-mutable** subset **and** doubles as a runnable
  correctness-guard. A command that is slow, or mutative, and adds no guard-value is **strictly worse** than
  not probing (we'd run it only to expose a non-negotiated type). `getent passwd <user>` is the model case
  (cheap, read-only, and checking a user exists before use is a real correctness win); commands like `sysctl`
  or `sc query` are **uncertain on name alone** (possibly expensive or mutative) and must be verified
  per-command before blessing. The bootstrap is the *non-mutable, guard-valuable* slice, not a raw count.
- **A new, orthogonal axis — the command-*wrapper* (deferred).** `sudo`/`ssh host CMD`/`docker exec C CMD`/
  `flock`/`chroot` [B-flock-man-2024] run a *nested, analyzable* command in a named **context** that is a
  *separate per-kind namespace* (the same `apt-get install nginx` is a different cell on host-A vs host-B vs
  inside a container) — and `ssh`/`docker exec` *are Dorc's own execution model*. **Deferred** (the
  wrapper-context `dq`): whether execution-context becomes a first-class kind, designed once across analyzer
  and transport (`kCOMMS`), is left to a later round.
- **THE BOUND:** all blessed/bounded. The arbitrary/opaque kind still needs the declared handle (§6 / top
  paragraph). Command-execution beats co-reference **only via blessing**.

## 8. The empirical check — strawmen on real provisioning code (`175` §6; `notes/17x-strawmen/`)

Six real, commit-pinned provisioning scripts; guards extracted by grep without reading mutative bodies.
- **The spine is real.** Every book's idempotent core *is* the getent-pattern kinds (`user`/`group`/`service`/
  `tool`/`file`), probed by the Result-D commands; a careful author (`consul`) writes the whole oracle for
  free, and the kinds are uniform → cross-script-correlatable.
- **The working example — cross-author instance-bridging that token co-reference can't reach.**
  `enginescript-redis`'s unguarded `usermod -aG redis www-data` depends on `user:www-data`, created by a
  *separate* web-stack script. No shared token in *this* script → co-reference is blind. But `getent passwd
  www-data` (a blessed kind) **discharges it across the script boundary**, and `id -nG www-data | grep -qw
  redis` **elides the usermod**. Reachable *only* by a declared/blessed named kind bridging two authors onto
  one cell — the concrete payoff of Part II over Part I.
- **The honest long tail.** Config *content*, whole-script *sentinels*, *packages*, unguarded *group-
  membership* + *ufw ports* stay un-liftable; hardening scripts worst-case (~2 of ~9 author-guarded);
  port-probe-ability is provider-dependent (clean `firewall-cmd --query-port`, fragile `ufw status | grep`).

## 9. The coordination contract — who declares what (`17H` E / `17B`)

When several oracles touch one multi-state entity, *who declares what?* Four author-obligation options, by
coordination cost:
- **opt-1 · monolithic** — one oracle owns the kind + all states. Zero coordination; doesn't compose.
- **opt-2 · RAL-enum** — kind-owner declares the state-vocabulary; providers slot in per `(state, provider)`
  (Puppet). Composes; **risk = shared *name*, divergent *meaning*** ("active" = systemd-active vs HTTP-up) ⇒
  over-correlation. The contract must pin selector *meaning*, not just name (the Seam).
- **opt-3 · per-facet + cross-facet invalidation** — separate oracles per facet; one declares "writing facet
  X invalidates facet Y" (cert-write must *kill* the loaded-facet). Most expressive, highest coordination.
- **opt-4 · precedence / resolution order** — a declared precedence over overlapping sources
  ([B-pep-561-distributing-type-info-2017]). Resolves *which oracle wins*.
- **The unifying rule (the lean): MUST-grade-to-correlate (`17B` F3).** A discharge is licensed **only** by
  an explicit, structurally-anchored declaration that two operations touch the same `(kind, selector)`;
  absence/ambiguity ⇒ **no-match ⇒ run**. Engler's MUST/MAY (`096`) applied to the *correlation itself*; it
  restores directionality to F6's non-directional fidelity failure (failures forced toward over-run, safe).
  Coherence (inc-9) is what makes opt-2/opt-4 sound. **Open (`oq-2`):** which option, or per-kind (a `mode`)?

---

# THE SEAM — where Part I and Part II fuse (`17H` spine-2 / fw-4)

**The one un-dodgeable obligation: cross-referent MEANING-agreement.** Four hazards gathered separately are
**one object**:

> coherence — instances of a class must agree [A-wadler-blott-ad-hoc-polymorphism-1989]
> ≡ polymorphic-variant *same-tag-different-meaning* [B-garrigue-polymorphic-variants-1998]
> ≡ DefinitelyTyped's *shared-name / divergent-meaning* [B-definitelytyped-governance-2020]
> ≡ the *over-correlation* that causes under-execute (`17B` / F6 fidelity axis)
> ≡ InChI's "standard layer-set" needed before independent ids match (`175` C4).

All five: **agreement on what a kind's name/state MEANS is load-bearing, and NOT covered by ⊤-on-conflict** —
silent same-name-different-meaning *never conflicts*. The boolean "unsure ⇒ false ⇒ run" default protects
*within-oracle* uncertainty; it does nothing against *cross-oracle* over-correlation (`17B` F4).

**This is exactly the K1↔K2 seam, and it is one object** — where K2's coordination semantics (Part I: what
states exist, when one state-test discharges another's dependency — inc-9, §9) meets K1's grounding of what a
name/state *means* (Part II: the named-kind anchor, the pinned selector-meaning — §5/§6/C4). The named kind is
**K1's identity-anchor + K2's type, fused**, because the discipline's most dangerous operation is licensed by
an *identity* claim:

- **fw-1 / fw-2 · the strong-update keystone collapses to identity.** A strong update (the only precise
  update) requires proving two mutations target the **same unique cell** (SF-1,
  [A-foster-flow-sensitive-qualifiers-2002]); typestate linearity (kill-5) needs the same uniqueness. Under
  F0 the analyzer holds only opaque pointers — so the one operation whose misfire makes the floor unsound has
  a welded-undecidable precondition, and it is *identity* (the instance question, F3), not discipline. The
  resolution is the **declared anchor**, not analysis; structure gives hints (may), the probe confirms
  *state* (not identity).
- **fw-3 · the verdict channel must not bake a phase (SF-3, superposition).** The 3-valued verdict
  (holds / fails / unknown) crosses **both** phases, but `kFAIL` is welded *phase-keyed* (probe ⇒ withhold,
  apply ⇒ perform). A single lattice can't carry two opposite fail-orientations; the engine emits
  **phase/orientation-agnostic facts** and the phased caller collapses them. The discipline must not bake a
  phase default.

**The directionality fix is one principle on both sides:** spine-1 (the *oracle* declares the safety-
direction) and MUST-grade-to-correlate (§9: only an explicit anchored declaration discharges) are the same
rule — *correlation, like the anchor, is licensed by declaration; absence defaults to run.* Coherence (inc-9)
makes it sound; Dorc can't enforce it (never rejects plain sh), so it is a **kind-owner-declared contract +
best-effort CI lint** (inc-5's machine-enforced-not-author-trusted model), never a checked property.

---

# The lean, in one paragraph

Build the **dumbest forgiving discipline**: **nominal kinds (types = namespaces over pointers-to-shared-state,
plus a minority of direct value-types) carrying a ≥enum state-model read as a flow-sensitive
effect-map/typestate, narrowed by occurrence-typing guards, everything unanchored ⇒ ⊤ ⇒ run** (soundiness).
Keep the *postures/properties* of the forgiving lineage (success-typing polarity; the gradual guarantee as a
no-cliff test; pluggable no-runtime-effect; soft-typing's static/dynamic split; the stub governance model;
the may/must certainty mechanic routed by setting); kill the heavy machinery (casts/blame; corpus-distributive
skip-licensing; Koka row-polymorphism; typestate enforcement; HM; dependent types) — because Dorc omits
runtime enforcement, so the safety-direction comes from the **oracle, not the tool** (spine-1). The least-
burdensome cross-oracle channel is a **handle author-declared and lifted into the analyzer index** (reverse-DNS
datum, or — where sh cannot carry the claim — the top-paragraph annotation escape-hatch; never a function-name,
F4), with optional selectors (C3), consumer-driven matching (C1), a thin coherence standard for the *compared*
states (C4), carried provenance allowed (C6), and the irreducible ≥1 anchor (F2). **The bound:** for the
arbitrary/opaque kind, no inference-only spelling exists — the kind is irreducibly author-declared; **blessing
buys a bounded vocabulary; co-reference buys a may-grade hint; the open-ended kind costs one declaration.** Not
magic — exactly as DESIGN predicted.

---

# Candidate sh-spellings — *illustrative, NOT recommendations* (`175` §12)

> Strawmen to argue with and throw away. Confidence low unless a documented cost is flagged. Running example:
> `frobctl` manages "wombats" (defrocked/frocked/wet); oracle B wraps it; oracle A wraps `zonk`,
> non-mutative only on a defrocked wombat.

- **P1 — handle as a lifted datum, not a function name** (F4): `oracle_kind='net.frobnitz.wombat'`.
- **P2 (hazard documented) — probe captures the tool's own rc separately.** `cmd | grep -q` conflates "no
  match" with "tool failed". `wombat_is() { command -v frobctl >/dev/null 2>&1 || return 2; _s=$(frobctl
  status -- "$2" 2>/dev/null) || return 2; [ "$_s" = "$1" ]; }`
- **P3 — states as optional selectors** (`net.frobnitz.wombat#defrocked`); minimal-but-extensible.
- **P4 — DSL-smell, rejected.** `wombat_is defrocked "$w" || frobctl defrock -- "$w"` — "infer polarity from
  the guard, declare only the residue" reads as a bespoke mini-DSL; the human rejects it. Kept only as a
  documented-bad strawman.
- **P5 — A's precondition is a guard naming the same handle:** `zonk_check() { oracle_requires
  'net.frobnitz.wombat#defrocked' "$1"; }`
- **P8 (highest-value — *lead here*) — read where the kind is *already* grounded by idiom.** The most basic:
  filesystem test/mutator operators — `[ -f "$x" ]`, `[ -d "$x" ]`, `rm`/`cp`/`mv` — reveal a runtime value
  is a *filename/path* with no new spelling at all. Then OS metadata: `systemctl is-enabled` / `dpkg -s` /
  `pkg-config --exists` / `id -u`. No new handle if the system already wrote one.
- **P9 (Result D) — for a blessed, bounded vocabulary, read the kind off the command.** `getent passwd "$u"
  # arg-1 'passwd' = the kind; rc = found/absent/unknown; read-only`. Handle-free for the NSS kinds; does not
  generalize to an opaque kind (§7 bound).
- *(Where none of these can carry the claim — the `Wombat`-transition case — the top-paragraph inline-annotation
  escape-hatch is the provisional fallback.)*

**Documented-cost anti-patterns:** verdict mixed into freeform stdout (GitHub paid an injection CVE, moved to
environment files [B-github-actions-setoutput-deprecation-2022]; `kCOMMS`) · `cmd | grep -q` (P2) ·
over-coarsening a kind below its ≥enum mutation-gating floor (`is-enabled` discharging `is-active`, `17B`).
**Narrow note, not a blanket ban:** a **dotted name carrying the 3-place `(kind,provider,verb)` relation**
clobbers across providers (X3, F4) — but using `frobctl.check()` merely as a *probe function name* is **not**
ruled out by this (the "fails `dash -n`" objection is moot once Dorc owns its parser and may accept
annotations); the X3 clobber is specifically about a 1-place name standing in for the 3-place relation.

---

# Open decisions this feeds

- **`dq-kOOB` — provisional lean recorded (bounded, top paragraph): an inline-annotation escape-hatch for the
  claims sh can't carry.** `KNOBS kOOB` is `directional`; this is the round's pressure on it. Still a human
  ruling — kept to its corner, not threaded through the body.
- **`kTYANNOT` (new knob — added to `KNOBS.md` this round) — `kTYANNOT-inline` ↔ `kTYANNOT-eol-comment`.** The
  off-ramp forces the choice (inline-ergonomic-but-off-ramp-hostile ↔ eol-comment-clean-but-DX-vomit);
  downstream of whether a type-system exists at all. Definition in `KNOBS.md`; see top paragraph + `17O`
  F-OFFRAMP.
- **`dq-entity-algebra` — NOT deferrable; recursive-structure lean (§4 / top paragraph; `17O` F-ALGEBRA).**
  ≥enum floor settled; flat-vs-structured forced *structured* by the floor + the 16Q `q1-precision` gate; the
  lean is a recursive JSON-adjacent struct (present-key=true, `!`-false, nested/handle values).
- **`dq-substrate` — deferred to the next spike (`16Q` §3).** IFDS/IDE vs Datalog vs worklist; the human is
  underqualified to settle on paper and will experiment; the provenance/why-tree query model is the
  undercounted coupling. Many F8 must-handles will likely *force* the choice.
- **The wrapper-context `dq` — deferred (§7).** Execution-context as a first-class kind (host/user/container/
  lock), designed once across analyzer + transport.
- **`oq-1`/`oq-2`/`oq-3` (`17B`).** Is MUST-grade-to-correlate *the* resolution? Which coordination option, or
  a `mode`? Is selector-meaning-pinning a firewall hand-off or K2's to bound?
- **NEW: strict-mode vs yolo-mode (surfaced this round, unsettled).** A possible relaxation lever to get
  "reasonable" skipping on generic/oracle-less everyday scripts in ~80% of cases. **Asymmetric cost (note):**
  relaxing the over-execution bound (priority-2, `kFAIL-perform`) is a softer trade; relaxing the
  no-undeclared-probe default (`kFAIL-withhold`, *welded*, the "plan stage doesn't mutate" promise) is a
  redline breach, not a dial. The human dislikes the lever (another hit to the value-prop); recorded, not
  adopted.

# What this reunites / feeds

`175` (K1) + `17H` (K2), reunited as **one named-kind object**. Feeds: **DESIGN "Inference limitations"** (the
`wombat`/`hork` passage dead-ending on "(UNSETTLED, CONTINUE)") — the settled continuation to carry (per `16Q`
§4, *shape not spelling*; the human's edit) is "an author-declared, **fact-centric**, reverse-DNS-named-or-
blessed-probed **named-kind** anchor; 3-place, never a 1-place naming convention; co-reference is only a
may-grade hint; *blessing* buys a bounded vocabulary" — with the open caveat that the irreducible declared
claim may need the inline-annotation escape-hatch. Also: **`effort-allocation`** (bless the *non-mutable,
guard-valuable* getent-pattern slice) · **`kCOMMS`** (verdict channel off freeform output; P-anti-pattern) ·
**`kELISION`** (the strong-update/discharge predicate the Seam licenses) · the next spike's `q1-precision` +
F8 flow-analysis build (`16Q` §2).
