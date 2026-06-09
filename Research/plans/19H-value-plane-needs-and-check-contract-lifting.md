# 19H — The value-plane the engine needs, and the shape of `check()` contract-lifting

> **What this is.** A forward design synthesis (reference-quality, not a strain-log) distilling the
> round-19 finding into the two things take-3 must get right. Round-19 validated the **output** side of
> the elision engine — one coherent `Observable`, the apply fold, the cell-model + ambient gate — on
> *injected stand-in values*, then drove into the wall: the **input** side (the value-plane + the
> command-keyed `check()` that *produces* those values and *resolves* entities) is unbuilt, and is
> unbuildable on the round-16 "no value-plane, inject the values" foundation. §1 specifies what that
> input side needs; §2 specifies the shape of `check()` contract-lifting with copious examples; §3 the
> carry-forward; §4 the open forks.
>
> AI-authored, confidence-marked (+SURE / ~SUSPECT / -GUESS / --WONDER). Trust the root
> `README`/`DESIGN`/`IMPLEMENTATION`/`KNOBS`/`AGENTS` and the human rulings (19A §5, find-3, the round-19
> conversation) over this. Continues `19F`/`19G`. The slugs it leans on: `16C` (no value-plane), `19A §5`
> (the settled probe→observables→abstract-interpret model), `ch-shape-anno` / `kTYANNOT` (the inline
> annotation), `seam-interproc` / `seam-finite`, `inv-referent-agnostic` / SF-1 / `an-entity-uniqueness`
> (identity declared-never-inferred), `17N` (named kinds), `17O` (the stdlib-oracle quality bar).

---

## 0. The finding, in one sentence

The settled model — **probe runs the command-keyed read-only `check()`s → concrete observables → the
apply phase abstract-interprets the CFG over those values → omits what can't run** (`19A §5`, +SURE
human-ruled) — *requires the engine to flow concrete values* (the book's literal arguments, through the
oracle's `check()`), so a **narrow value-plane is not optional**: it is the single mechanism that both
**resolves entities** (find-3) and **produces observables** (the rc the fold consumes). `16C`'s "no
value-plane; every lock is fact-shaped; inject the values" was the correct scoping for round-16 but is
the *foundational mis-step for this goal* — it forced every downstream piece (entity-resolution,
rc-production) to become a stand-in (the engine's flag-strip; the test-injected rc).

---

## 1. The value-plane — what the engine needs

### 1.1 What it is, and what it is *not*

+SURE. A **narrow, decidable, forward value-analysis** over a **deliberately-constrained oracle-contract
dialect**: it tracks concrete **constants** (literals like `nginx`, `install`, `-y`) and **simple
positional parameters** (`$1`, `$@`, `shift`) as they flow through assignment, parameter-binding, and a
small fixed set of string/control constructs — and ⊤ on everything outside the dialect.

It is **NOT** general analyzer-side value *synthesis* — `16C`'s refutation of that stands; the general
case is undecidable and undesirable. The narrowing is exactly what buys back decidability, on **two**
independent constraints that must both hold:

- **the book side is concrete-or-⊤** — only a *literal* operand resolves; a `$VAR`/`$(…)` operand is ⊤ ⇒
  the command runs (already the spike's posture: `effect::resolve_entity` ⊤s on non-literal). The book
  author gets analysis *only* for literal args; nothing is guessed.
- **the oracle side is the constrained dialect** — the `check()` is written in a liftable subset ("feels
  like sh, works like sh, reliably liftable" — human), *not* arbitrary sh. A `check()` that steps outside
  the dialect is ⊤ ⇒ its command runs. The dialect's boundary **is** the ⊤-boundary.

The domain is small: `Value = ⊥ | Const(bytes) | ⊤` per value-location, plus a positional-parameter vector
(`$1..$n`, mutated by `shift`). ~SUSPECT a couple of derived shapes are needed (a "prefix-stripped" view
for `${1#-}`), but nothing approaching a string solver.

### 1.2 Why it is necessary — the two engine capabilities that *cannot exist* without it

These are the "parts necessary to make the rest of the engine work" — each is a thing round-19 had to
*stand in for* precisely because the value-plane was absent.

- **(need-entity) entity-resolution → the cell-model's input.** The cell-model (`FactKey{kind, entity,
  selector}`, the poison-wall fix, `19G §1`) needs each command resolved to a cell. Without flow-tracking
  the book's `nginx` *through the oracle's argparse* to a kind-annotation, the engine must **infer**
  identity (flag-strip + verb=word-1). That stand-in (`find-3`, annotated in `effect.rs`): (a) violates
  the **welded** "identity is *declared, never inferred*" (SF-1 / `an-entity-uniqueness` / `17N F3`); (b)
  breaches `inv-referent-agnostic` (the engine reading argument *structure*); (c) mis-parses no-verb
  commands (`useradd deploy` → verb=`deploy`). The value-plane is *how the oracle's declaration (the
  annotation) reaches the book's concrete arg* — the only sound bridge.
- **(need-obs) observable-production → the fold's input.** The fold abstract-interprets `&&`/`||`/`if`/`!`
  over concrete rc **values** (`19A §5`: `9 || mkdir` ⇒ mkdir runs, by the shell's own semantics; rc is
  *opaque* to Dorc). Those values must come from *running the read-only `check()`s in the probe* — not
  from a fixture. Round-19 injected them (the cli `rc=N` stdin, the `substitutes_exact_rc` test): the
  masking. The value-plane (statically: which read-only body to ship; dynamically: run it, receive the
  rc) is what makes the rc a *real probed value*.

+SURE the load-bearing realization: **both needs are the same mechanism** — the command-keyed `check()`.
So the value-plane is not two features; it is one analysis serving entity-resolution *and*
observable-production. This is why it cannot be deferred to a "build-2": it *is* the keystone-input.

### 1.3 The necessary capabilities (the minimum the dialect + analysis must model)

Driven by the canonical argparse (`§2.1`). Each `C-vp-*` is a thing the lift must handle or the whole
command degrades to ⊤ ⇒ run.

- **`C-vp-1` literal constants** tracked through the AST (`nginx`, `install`, `-y`). The decidable floor.
- **`C-vp-2` positional-parameter binding (interprocedural).** Book call `apt-get install -y nginx` binds
  the oracle `check()`'s `$@ = [install, -y, nginx]`, `$1 = install`, … This is the **call-edge +
  param-binding** the spike lacks (`lower_funcdef` builds detached bodies, no call-edges — `seam-interproc`).
  *Without it there is no path from the book's constant into the check.* +SURE this is the single biggest
  net-new piece.
- **`C-vp-3` `shift` / `shift N`** — re-binds positionals (`$1` becomes the old `$2`). The spine of any
  argparse. Modeled as a concrete index-advance over the (finite) arg-vector.
- **`C-vp-4` a minimal string-op set for option detection** — `${1#-}` (prefix-strip), `[ "$x" = "$y" ]`
  equality, enough to express "is this an option." The dialect fixes the allowed set; anything else ⇒ ⊤.
- **`C-vp-5` control-flow over values** — `while`, `case $1 in -t|-o) … ;; *) … esac`, `if` —
  abstract-interpreted over the concrete/⊤ values. The apply fold already does this for *rc*; the
  value-plane **generalizes the same machinery to strings/constants** (`§1.5`).
- **`C-vp-6` the inline kind-annotation as the grounding anchor** — `pkg : com.debian.apt.Package = "$1"`
  binds the flowed value to a *named kind* on that control-flow path (`ch-shape-anno`; `17N` reverse-DNS
  handle). This is the **one non-sh-native token** in the whole scheme, and the locus of the `kTYANNOT`/
  `kOOB` debt. The engine lifts it; the value flowing into it becomes a typed entity.
- **`C-vp-7` ⊤-propagation as the safety floor** — a non-literal book arg, or any `check()` construct
  outside the dialect, ⇒ ⊤ ⇒ *can't-resolve* ⇒ the command **runs** (`kFAIL-perform`). No partial best-
  effort guess — that guessing *is* the flag-strip stand-in's sin (`need-entity`). The boundary is loud.

### 1.4 What keeps it decidable (termination + the constrained dialect)

+SURE, and important for the compiler-minded reviewer: this does **not** reintroduce an undecidable
value analysis, because the analysis is **bounded concrete partial-evaluation over a finite arg-list**,
not a fixpoint over an unbounded value lattice:

- the book's args are a **finite list of literals** (or ⊤). Abstract-interpreting the `check()`'s argparse
  (`while`/`shift`/`case`) over a finite concrete arg-vector **terminates** — each `shift` strictly
  advances a bounded index; the `while` consumes a finite list. (`§2.1` runs to completion in 3 iterations
  for `[install, -y, nginx]`.)
- the value domain is **finite-height**: the only `Const` values that ever appear are *substrings of the
  script's own literals* (+ ⊤). No new values are synthesized (that is the `16C` line we do **not**
  cross). So `seam-finite`'s ascending-chain guarantee holds without a hard depth cap — the cap stays as
  the loud backstop.
- the constrained dialect is the *other* half of decidability: the oracle author writes argparse in a
  **liftable subset** by contract. A `check()` that uses an un-modeled construct (`eval`, a dynamic
  `$cmd`, unbounded recursion) lifts to ⊤ — its command runs, never a wrong-resolve. **The dialect must be
  designed deliberately as a first-class artifact** (what is liftable), not discovered ad-hoc.

### 1.5 How it composes with the round-19 engine (the carry-forward, not a rebuild)

+SURE. The value-plane is a *generalization of machinery round-19 already validated*, not a foreign body:

- **the apply fold is a special case.** `plan::fold` already abstract-interprets `&&`/`||`/`if`/`!` over
  `AbstractRc ∈ {Known(rc), Top}` — i.e. a value-plane whose values are *exit codes*. The value-plane
  generalizes the domain to *constants/strings flowing through the check*. Carry the fold's design
  forward; widen its domain. (`19C` is the fold; its algorithm is right, only its *inputs* were injected.)
- **the cell-model is the value-plane's output.** `FactKey{kind, entity, selector}` is exactly
  (annotation-kind, flowed-entity, verb-selector). The poison-wall fix stands; it just needs a *sound*
  entity instead of the flag-strip's inferred one.
- **the worklist substrate hosts it.** Lattice value = `Const | ⊤`; forward dataflow; no IFDS/Datalog
  (`19A §5` / `notes/180`: substrate is not the question). The `Box`/`Product`/`MapL` combinators compose
  a positional-param environment.
- **the static/dynamic split.** The value-plane is the **static** half (resolve the entity for the
  cell-model + ambient gate; decide *which read-only body to ship*). The probe is the **dynamic** half
  (ship the body full-args, run it, receive the concrete observables). The fold consumes the dynamic half.

---

## 2. The shape of `check()` contract-lifting (examples)

The unifying object is the oracle's **command-keyed, full-args `check()`** (`19A §5` C-1/C-4): one sh
function per command-family that (a) argparses the command the way the *real* tool does, (b) inline-
annotates which value is which kind, and (c) *is* the read-only probe body. The engine lifts it statically
(entity-resolution) **and** ships+runs it (observable-production). Examples build from simplest to hardest.

### 2.1 Anatomy — the package mutator-with-read-only-probe (`apt-get`)

```sh
apt_get__check() {                          # command-keyed: this lifts/ships for `apt-get …`
   while [ "${1#-}" != "$1" ]; do           # skip leading options the way apt-get really does
      case $1 in -t|-o) shift 2 ;; *) shift ;; esac
   done
   verb=$1; shift                           # the subcommand: install / purge / update / …
   pkg : com.debian.apt.Package = "$1"      # ← inline-declare: THIS value is a Package, on THIS path
   dpkg-query -W "$pkg"                     # the read-only fact-probe (NOT a dry-run of apt-get)
}
```

What the engine does, on the book line `apt-get install -y nginx`:

- **lift + bind** (`C-vp-2`): `$@ = [install, -y, nginx]`.
- **flow** (`C-vp-3/4/5`): the `while` tests `${1#-}` ≠ `$1` — true for `-y` (strips to `y`), false for
  `install`/`nginx`. So it consumes `-y` (the `*) shift` arm), stops at `install`. `verb = install`,
  `shift`, `$1 = nginx`.
- **anchor** (`C-vp-6`): the annotation binds `pkg = nginx : com.debian.apt.Package`. The engine never
  decided `-y` was a flag — the *oracle's own argparse* did, and the engine traced the constant through it.
- **produce — static**: entity `package:nginx`, kind from the annotation, selector from `verb` (`install`
  → `#installed`; `§2.5`). Cell = `package:nginx#installed`, fed to the cell-model + ambient gate.
- **produce — dynamic**: ship `apt_get__check install -y nginx` (full-args, `C-1`); the host runs it; the
  `dpkg-query -W nginx` body returns the rc. *That* rc is the probed observable.

Contrast the round-19 stand-in: `resolve_entity` flag-strips `-y` *itself* (find-3 breach) and the rc is
*injected* (masking). Both vanish here — the check is the single source.

### 2.2 The read-only guard — the idempotency idiom (the achievable slice)

The simplest and most common shape; +SURE achievable *without the full value-plane's hardest parts*,
because the guard is *already* a read-only command whose rc the probe can just run-and-read.

```sh
command__check() {                          # for the book's own `command -v X` idiom
   case $1 in -v) shift ;; esac
   tool : org.freedesktop.Tool = "$1"
   command -v -- "$tool" >/dev/null         # R2-SHADOW: resolves to an executable file, not a fn/alias
}
```

Book: `command -v nginx || apt-get install nginx`.

- the guard `command -v nginx` lifts (entity `tool:nginx`), ships, **runs** → rc 0 (present) / 1 (absent).
- the fold consumes the *guard's* probed rc: `0 || install` ⇒ install **omitted**; `1 || install` ⇒
  install **runs**. **No declaration of `install`'s rc is needed** — the guard's probed rc drives the fold.
- this is the canonical `dpkg -s || install` / `command -v || install` idiom (DESIGN). It is the
  **read-only-guard slice** (round-19's "R2" capstone option): demonstrable on the existing `hostsim`
  (map the modeled fact-verdict → the guard's rc) and the existing fold, with entity-resolution still on
  the find-3 stand-in. It needs `need-obs` for *guards*, not the harder mutator case below.

### 2.3 The mutator with no read-only self-probe (`useradd`) — and the central model fork

```sh
useradd__check() {
   # (option-skip elided; useradd's real grammar)
   user : org.openldap.PosixAccount = "$1"  # `useradd <name>` → $1 is the User; NO verb
   getent passwd "$user"                    # the READ-ONLY fact-probe (NOT useradd itself)
}
```

The check's read-only body (`getent passwd deploy`) probes the **fact** (does the user exist) → its own
rc. But for `useradd deploy || mkdir /srv/app`, the `||` consumes **useradd's** rc, and useradd is a
mutator — it is *not run* in a read-only probe. So:

- **the fork (flag for take-3, do not pre-decide):** does the oracle *also* declare "when the user exists,
  `useradd` yields rc **9**" (`19A §5` C-4-refined's `fact-state → observables`, enabling a value-
  preserving elision of useradd to `(exit 9)`)? **Or** is an un-probeable mutator simply **⊤ ⇒ run** (the
  human's later "no values, no defaults; only abstract-interpretation over probe-received values")?
- **~SUSPECT lean (per the human's latest):** **⊤ ⇒ run.** `useradd deploy || mkdir` runs `useradd`
  (we have no probed value for it), and at runtime its real rc 9 fires the `|| mkdir`. `mkdir` runs.
  Convergence-elision still applies *only* where the mutator's status is **not branch-consumed** (a bare
  `useradd deploy`, converged + ambient ⇒ replaceable by `true`, its dead status harmless). Under this
  reading there is **no `fact-state → observables` declaration**, no converged-rc, and round-19's
  `substitutes_exact_rc` (`useradd` → `(exit 9)`) was itself the over-reach — the simpler, sounder
  behavior is "it runs."
- the cost of the lean: we lose the *optimization* of eliding a branch-consumed non-conforming establish.
  Per the priority order (never under-execute ≫ avoid unnecessary-execute) that is the correct trade. The
  richer `fact-state → observables` is reservable for later if the value is ever shown to matter.

The +SURE part regardless of the fork: a mutator's rc is **never inferred or defaulted** — it is either
a genuinely-probed value (it is not, here) or ⊤ ⇒ run. The round-19 `rc=0`-default and the injected
`rc=9` are both stand-ins the take-3 model deletes.

### 2.4 Cross-oracle identity — many commands, one named kind (`17N §5`)

```sh
apt_get__check() { … pkg : com.debian.apt.Package = "$1"; dpkg-query -W "$pkg"; }
dnf__check()     { … pkg : com.debian.apt.Package = "$2"; rpm -q "$pkg"; }   # different command, SAME kind
```

- the **named kind** (the reverse-DNS handle `com.debian.apt.Package`, a *lifted datum* — `175 C2` /
  `17N`) is the **coordination vocabulary**: `apt-get install nginx` and a later `dnf … nginx` resolve to
  the *same cell* `package:nginx#installed`, so an apt-establish and a dnf-query coordinate even though
  the engine is referent-agnostic about the *commands*. A shared *arg-token* could not bridge them (`17N
  §5`); the kind does.
- the per-command argparse differs (`$1` vs `$2`, different option grammars) — that is *exactly* why
  identity is command-keyed (only the oracle knows its tool's grammar) while the *kind* is cross-oracle
  (`19A §5`'s three-layer split: command-keyed *invocation*, named-kind *identity*, fact-converged
  *license*).
- ~SUSPECT residual (`inc-9`, contract-not-enforced): two oracles annotating the *same* kind must *agree*
  on what it means; Dorc cannot enforce it (it never rejects plain sh) ⇒ a CI-lint, never a checked
  property.

### 2.5 The selector comes from the verb, not the entity

```sh
systemctl__check() {
   verb=$1; shift
   svc : org.freedesktop.systemd.Unit = "$1"
   case $verb in
      enable) systemctl is-enabled -- "$svc" ;;   # → svc:…#enabled
      start)  systemctl is-active  -- "$svc" ;;   # → svc:…#active
   esac
}
```

- the annotation gives **kind + entity** (`service:nginx`); the **verb** gives the **selector cell**
  (`#enabled` vs `#active`). `systemctl enable nginx` and `systemctl start nginx` touch *different cells of
  the same entity* — neither discharges the other (the round-19 selector regression, `19G`/`193`). The
  `check()` makes this fall out of its own `case $verb`.
- +SURE the `≥enum` floor (`17O F-BLESSED`): an honest `service` probe is *two* read-only commands
  (`is-enabled` **and** `is-active`); discharging `enable --now` needs both. The contract-lifting must let
  one `check()` carry multiple selector-probes (the `case $verb` above), not collapse a service to one bit.

### 2.6 The stdlib-oracle quality bar the lifting must preserve (`17O`)

The `check()` body is the **read-only probe**, so the `17O` regression class is *contract*, not engine
holes — the lifting/shipping must not paper over them:

- **`R2-SHADOW`** — `command -v X` must confirm an executable *file* (`-v -- "$x"` + a file test), not a
  function/alias/builtin; Dorc's own helper-fn idiom is what shadows it. Fails **unsafe** (reports
  installed ⇒ wrong-elide).
- **`R2-IDCACHE`** — group membership via `getent group … | field-4`, never `id -nG` (stale nss cache).
- **`R2-ORTRUE`** — the lifter must **refuse to treat an errexit-masked rc as a verdict** (`… || true` /
  `|| :` forces rc 0). A lifted check whose rc is masked is not a verdict.
- **`F-GETENT-HOSTS`** — `getent hosts`/`ahosts` route through nsswitch ⇒ live DNS = non-hermetic
  (`kVOLATILES`); **read-only ≠ hermetic**, disqualified from licensing elision.

These are why the `check()` is *authored* (the engineer's correctness-heavy work), not generated — they
are the contract DESIGN calls "what makes our product."

### 2.7 The static ⟷ dynamic duality, summarized

One `check()` body is read **twice**:

| phase | reads the `check()` for | yields |
|---|---|---|
| **static** (value-plane, analysis-time) | the argparse + annotation | the cell `kind:entity#selector` (→ cell-model, ambient gate) + *which* read-only body to ship |
| **dynamic** (probe, run-time) | the body, shipped full-args (`C-1`) | concrete observables (rc, stdout) → the apply fold |

The value-plane is the static half; round-19 built the consumers of the dynamic half (the fold) but fed
them injected values because the static half (resolve which body, ship it, run it) was the stand-in.

---

## 3. What take-3 carries forward from round-19 (validated yield)

+SURE these are *designs the spike validated*, to layer on the value-plane + `check()`-lifting rather than
re-derive:

- **the one `Observable`** (`19G §1`, Commit A `f148a31`): the output-tuple over channels
  `{Effect, Status, Stdout, Stderr}` — coherent, and the right shape for "the check predicts a per-channel
  value or ⊤."
- **the apply fold** (`19C`): abstract-interpretation of `&&`/`||`/`if`/`!` over probed values; rc opaque;
  ⊤ ⇒ run. The algorithm is right; it needs *real* inputs.
- **the cell-model + ambient gate** (`193`/`19G`): `FactKey{kind, entity, selector}` + reaching-defs over
  the effect-map — the poison-wall fix. The keystone *output*; the value-plane is its *input*.
- **the DST `hostsim` + the e2e corpus** (43 cases) + the `ap-2` `dash -n`-executable harness — the
  measuring infrastructure (the next step extracts + de-crufts this into the take-3 acceptance stick).
- **the substrate** (worklist + lattice combinators) — hosts the value-plane unchanged (`19A §5` / `180`).
- **the `17O` quality bar** (`§2.6`) — kept as regression contract.

---

## 4. Open forks for take-3 (flag, do not pre-decide — `ch-wrong`)

- **`fork-mutator-rc` (`§2.3`):** un-probeable mutator = ⊤ ⇒ run (the simpler "no-values" reading,
  current lean) vs. an oracle-declared `fact-state → observables` converged-rc (the richer, value-
  preserving elision). Resolves the contract's size. ~SUSPECT ⊤⇒run.
- **`fork-annotation-spelling` (`ch-shape-anno` / `kTYANNOT`):** the `pkg : Kind = "$1"` inline form
  breaks the off-ramp weld under stock dash (`17O F-OFFRAMP`) ⇒ needs a strip/transpile pass; the eol-
  comment alternative re-opens `kOOB`'s no-comment-config. The one non-sh-native token in the scheme; its
  spelling is the live `kTYANNOT` decision.
- **`fork-dialect-boundary`:** the constrained oracle-contract dialect must be *designed* (what `check()`
  constructs are liftable — the `C-vp-*` set is a first cut). This is the artifact that makes the value-
  plane decidable; it deserves its own pass.
- **`fork-interproc-scope` (`seam-interproc`):** call-edges into `check()` bodies — intra-file first, or
  the `. /path` source-following supergraph. `C-vp-2` (param-binding) is the minimum.

---

*The round-19 spike's durable product is this finding + the `notes/19*` record. The immediate next step
(per the human) is to extract the round-19 corpus, strip the cruft that only "fills in" for the stand-in
build (injected-rc fixtures, `resolve_entity`-dependent cases, masking tests), and turn it into the
clean acceptance measuring-stick the take-3 rewrite is graded against.*
