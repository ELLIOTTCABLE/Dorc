# 19H — The value analysis the engine needs, and the shape of `check()` contract-lifting

> What this is. A forward design synthesis (reference-quality, not a strain-log) distilling the round-19
> finding into two things take-3 must get right. Round-19 validated the output side of the elision engine
> — one coherent observable-tuple, the apply fold, the kind/entity/selector cell-model + ambient gate —
> by feeding it injected stand-in values, then drove into the wall: the input side, a real value-flow
> analysis (and the command-keyed `check()` it lifts and runs), is unbuilt, and is the actual core of the
> tool. §1 specifies what that analysis needs; §2 the shape of `check()` contract-lifting, with examples;
> §3 the carry-forward; §4 the open questions, several of which a prior-art run should settle before any
> rebuild.
>
> AI-authored, confidence-marked (+SURE / ~SUSPECT / -GUESS / --WONDER). Trust the root
> `README`/`DESIGN`/`IMPLEMENTATION`/`KNOBS`/`AGENTS` and the human rulings (19A §5, find-3, the round-19
> conversation) over this. Continues `19F`/`19G`. An adversarial-crosscheck on the first draft sharpened
> §1.3/§2.3/§4 (the soundness floor is apply-direction only; the probe direction and propagation-correctness
> are separate guarantees). Slugs it leans on: `16C` (the value-synthesis refutation — narrower than "no
> value plane"), `19A §5` (the settled probe→observables→abstract-interpret model), `ch-shape-anno`/`kTYANNOT`
> (the inline annotation), `seam-interproc`/`seam-finite`, `inv-referent-agnostic`/SF-1/`an-entity-uniqueness`
> (identity declared, not inferred), `17N` (named kinds), `17O` (the stdlib-oracle quality bar),
> `dq-reflexive-probe-inertness`/`16P` DP-4 (probe-inertness is not contract-enforceable). The algorithmic
> and substrate claims here are deliberately held open (§4) pending a prior-art pass.

---

## 0. The finding

The settled model — the probe runs the command-keyed read-only `check()`s, returns concrete observables,
and the apply phase abstract-interprets the script over those values and omits what cannot run (`19A §5`)
— rests on the engine actually *tracking values*. It tracks them twice: before the probe, to work out
which command touches which entity and to compile the right read-only check; and after, to flow the
probe's results through the script's constructs and discover what is safe to elide. That value-tracking
is a real value-flow analysis, and round-19 deferred all of it — injecting the observables, inferring the
entities with a flag-strip. `16C` refused analyzer-side value *synthesis* (computing what a command would
emit at runtime); it did not, and could not, refuse value propagation (following a value the script
already names from where it is bound to where it is used). That distinction is this doc's own sharpening,
not `16C`'s wording: `16P` records the whole value plane as deliberately deferred — a charter-blessed
scoping, not an error. The forward point is narrower: propagation is decidable, it is not the thing `16C`
refuted, and this goal (eliding on real inputs) is the one that cannot afford the deferral, since it is
exactly what resolving entities and producing observables both require.

---

## 1. The value analysis — what the engine needs

### 1.1 It is a real value-flow analysis, over every file involved

It propagates values the program already names — through assignments, variables, function parameters and
positional arguments, and the ordinary control constructs — wherever they flow, across the files that
participate (books and oracles alike). It does not need to be cutting-edge, but it needs to be real:
constant propagation plus argument/parameter propagation is the floor, not the ceiling. A book that
writes `pkg=nginx; apt-get install -y "$pkg"` must see `nginx` reach the install the same way an oracle's
`check()` sees `nginx` reach its annotation — the moment you name a value, naming it cannot blind the
analysis, or positional parameters die on the first `x=$1`.

Two notes that the round-19 framing got wrong and this corrects. First, there is no useful book/oracle
distinction at this layer: they are the same machinery and largely the same thing — "oracle" and "book"
are design-shorthand for *intent* (a published, correctness-heavy provider vs. a scrappy local play), not
an implementational boundary, so the value analysis treats them uniformly. Second, the line we draw is
not a *category* exclusion ("variables are out, only literals are in") — it is a *reach* limit (see §1.3):
we follow what we can, and stop where it gets too hard, and stopping is always safe.

### 1.2 Why it is necessary — two engine capabilities, both value-flow

These are the parts that make the rest of the engine work; round-19 had to stand in for each precisely
because the analysis was absent.

- Entity-resolution, before the probe. The cell-model (kind/entity/selector — the poison-wall fix,
  `19G §1`) needs each command resolved to a cell. That means following the script's own value (`nginx`,
  however it arrived at the command) into the oracle's `check()`, through the `check()`'s argparse, to a
  kind-annotation. Absent the analysis, the engine *infers* identity by flag-stripping argv — the find-3
  stand-in, which breaks the welded "identity is declared, never inferred" (SF-1 / `an-entity-uniqueness`
  / `17N F3`), breaches `inv-referent-agnostic`, and mis-reads no-verb commands.

- Observable-flow, after the probe. Shipping a read-only check is pointless unless its result can then be
  *flowed into the script's constructs* to license an elision. The post-probe abstract-interpretation has
  to carry the probe-derived state through `&&`/`||`/`if`/`case`/assignments — the richer it is, the more
  elision candidates exist; the more constructs it cannot follow, the fewer. The round-19 fold did this
  for a bare exit code over `&&`/`||`/`if`/`!`; the real thing flows dependency-state through whatever the
  book actually wrote. The fold is the seed of this analysis, not the whole of it.

One mechanism serves both: the command-keyed `check()` (§2) is what the pre-probe pass lifts (to resolve
the entity and pick the body to ship) and what the post-probe pass consumes (its result is the observable
flowed into the fold). So the value analysis is not two features bolted together; it is one analysis with
a pre- and post-probe face.

### 1.3 The boundary is a control — the apply-direction floor, and what it does not cover

One invariant is load-bearing, and it covers the apply direction: wherever the analysis stops — a
genuinely dynamic value it cannot resolve, or a construct past whatever complexity we choose to follow —
it degrades to ⊤, and ⊤ means the command runs, its arguments go unparsed, nothing is elided
(`kFAIL-perform`). For that direction, soundness comes from the degrade-to-run and holds independent of
how powerful or precise the analysis is: correctness is fixed (⊤ ⇒ run), coverage is a dial — cheap now,
richer later — at no cost to correctness.

Two things that floor does not cover, and take-3 must guarantee by other means (the correction an
adversarial-crosscheck surfaced on the first draft of this doc):

- The probe direction (`kFAIL-withhold`: never mutate in a read-only pass). The `check()` body is the
  read-only probe we ship and run; if the analysis mis-resolves an entity or lifts the wrong body, the
  degrade never fires — the engine confidently ships a wrong-entity or mutating probe. Probe-inertness is
  a separate obligation: the reflexive-inertness check (point the effect-analyzer at the lifted probe body
  and refuse to ship one it cannot prove inert — `dq-reflexive-probe-inertness`) plus the sandbox the
  contract frame provably cannot replace (`16P` DP-4). A conservative value analysis does not make a probe
  read-only.
- Propagation-correctness. The floor protects against the analysis degrading; it does nothing against the
  analysis being confidently wrong — a value mis-tracked through `shift`/`$@`, or two distinct operands
  aliased to one cell, is a wrong resolution licensing a wrong elision, and the degrade never triggers
  because nothing reports the error. The analysis must be correct where it acts, not merely conservative
  where it gives up — the harder of the two guarantees, and the one §4 holds open.

The dial itself we control, and it can sit differently per input: "past this nesting / this construct, do
not parse arguments and do not elide" is a legitimate, tunable stop. ~SUSPECT we want it set more
generously for oracles (authored, expected to stay liftable) than for scrappy books — but since they share
the machinery, that is a threshold, not a different analysis.

### 1.4 What it must at least reach (minimum-necessary, not a closed set)

A non-exhaustive list of the low-hanging fruit the analysis has to clear to be useful at all. More is
better; none of these is a ceiling, and listing them is not meant to exclude anything absent from the
list.

- Constant propagation through assignment and variable use (`pkg=nginx; … "$pkg"`).
- Propagation through function parameters and positional arguments, including the re-binding the argparse
  idiom relies on (`$@`, `$1`, `shift`) — the bridge from a call to a `check()`'s body.
- Flow through the common control constructs already in the apply fold's reach (`case`, `if`, `while`,
  `&&`/`||`, `!`), now carrying values rather than only a bare exit code.
- The inline kind-annotation as the grounding anchor (`pkg : com.debian.apt.Package = "$1"`) — the one
  non-sh-native token in the whole scheme, and the locus of the `kTYANNOT`/`kOOB` debt (§4).
- Cross-file flow: a book calling a command an oracle defines, and `. /path`-style helper sourcing, so a
  value can travel from a book through an oracle's `check()` and back (`seam-interproc`).

### 1.5 How it relates to what round-19 validated

The apply fold is the first, smallest instance of this analysis: it already abstract-interprets
`&&`/`||`/`if`/`!`, only over a bare exit code and only on injected inputs. Take-3 widens its domain from
"an exit code or ⊤" to "the values the script names or ⊤," and feeds it real probe results instead of
fixtures. The cell-model is this analysis's output (the resolved entity, the annotation's kind, the verb's
selector), and the one observable-tuple is the shape the `check()` populates per channel. None of that is
thrown away; it is layered onto a real input side instead of stand-ins.

Whether that widened analysis rides the existing worklist substrate unchanged, or wants something else, is
an open question for the prior-art run (§4), not a claim this doc should make.

---

## 2. The shape of `check()` contract-lifting (examples)

The unifying object is the oracle's command-keyed, full-args `check()` (`19A §5` C-1/C-4): one sh function
per command-family that argparses the command the way the real tool does, inline-annotates which value is
which kind, and is itself the read-only probe body. The engine lifts it (to resolve the entity and pick
what to ship) and ships+runs it (to get the observable). Examples build from simplest to hardest. The
value flowing in may originate in the book, not just in literal argv.

### 2.1 Anatomy — `apt-get`, with the value arriving from the book

```sh
# book:
pkg=nginx
apt-get install -y "$pkg"

# oracle:
apt_get__check() {                          # command-keyed: lifts/ships for `apt-get …`
   while [ "${1#-}" != "$1" ]; do           # skip leading options the way apt-get really does
      case $1 in -t|-o) shift 2 ;; *) shift ;; esac
   done
   verb=$1; shift                           # install / purge / update / …
   pkg : com.debian.apt.Package = "$1"      # inline-declare: THIS value is a Package, on THIS path
   dpkg-query -W "$pkg"                     # the read-only fact-probe (not a dry-run of apt-get)
}
```

<!-- /* defect noted 2026-06-10 (round-20 build, notes/204 strain-1): this example is internally
inconsistent — the book line's argv is [install, -y, nginx] (flag AFTER the verb), but the
while-loop strips only LEADING flags, so as written the walkthrough below is wrong and the
annotation would bind entity=`-y`. The dialect evaluator faithfully reproduces whatever the
check's own argparse does (engine-side flag-guessing would be the find-3 sin), so corrected
fixture oracles strip post-verb flags too (verb=$1; shift; then strip). Both orderings are
pinned in oracle/tests/check.rs. ALSO (task-P/find-3, 20I §3): this single-operand annotation
must gate its probe on `if [ "$2" = "" ]; then dpkg-query -W "$pkg"; fi` — without that guard a
multi-target `apt-get install nginx curl` binds entity=nginx ALONE and ships a probe for nginx
only, silently dropping curl (a priority-1 under-execute; the naive-drop is pinned in
oracle/tests/check.rs::naive_oracle_without_operand_guard_drops_trailing_operands_known_hazard,
and the guard is an oracle-quality-bar line `R2-MULTIOP` in oracle/CLAUDE.md). */ -->

What the engine does:

- In the book, propagate `pkg` ⇒ the install's argv is `[install, -y, nginx]` — the book's own value-flow,
  same machinery as the oracle's.
- Bind those into the `check()`'s `$@`, then follow the argparse: the `while` consumes `-y` (it strips to
  `y` under `${1#-}`), stops at `install`; `verb=install`, `shift`, `$1=nginx`.
- The annotation binds `nginx : com.debian.apt.Package` on this path. The engine never decided `-y` was a
  flag — the oracle's own argparse did, and the analysis traced the value through it.
- Pre-probe output: the cell `package:nginx#installed` (kind from the annotation, selector from the verb,
  §2.5), fed to the cell-model and ambient gate.
- Post-probe output: ship `apt_get__check install -y nginx` (full argv, C-1); the host runs the
  `dpkg-query` body; its rc is the observable the fold then flows.

The round-19 stand-ins this removes: the engine flag-stripping `-y` itself (find-3), and the rc being
injected (the masking). Both collapse into the one `check()`.

### 2.2 The read-only guard — the idempotency idiom (the most common shape)

```sh
# book:  command -v nginx || apt-get install nginx
command__check() {
   case $1 in -v) shift ;; esac
   tool : org.freedesktop.Tool = "$1"
   command -v -- "$tool" >/dev/null         # R2-SHADOW: an executable file, not a fn/alias/builtin
}
```

The guard is already read-only, so the probe simply runs it and reads the rc: present ⇒ 0, absent ⇒ 1. The
post-probe pass flows that into the `||`: `0 || install` ⇒ install omitted, `1 || install` ⇒ install runs.
Nothing about the install's own rc is needed — the guard's probed rc decides. This is the canonical
`dpkg -s || install` idiom, and the smallest end-to-end demonstration of the whole loop: lift a guard,
ship it, run it, flow the result, elide.

### 2.3 The mutator with no read-only self-probe (`useradd`) — and a fork

```sh
# book:  useradd deploy || mkdir /srv/app
useradd__check() {
   # (option handling per useradd's real grammar)
   user : org.openldap.PosixAccount = "$1"  # `useradd <name>` → $1 is the User; no verb
   getent passwd "$user"                    # the read-only fact-probe (not useradd itself)
}
```

Note the shape first: `useradd <name>` has no verb — the entity is the bare first operand, exactly a case
the current engine cannot resolve (it reads verb=word-1, so `deploy` becomes the verb; the round-19
fixtures fake it by baking the username in as the verb). So this is not only an illustration of the model
— no-verb resolution is one of the things the value analysis must newly enable. With that flagged: the
check's read-only body (`getent passwd deploy`) probes the fact — does the user exist — and yields its own
rc. But the `||` consumes `useradd`'s rc, and `useradd` is a mutator: it is not run in a read-only probe.
So a fork take-3 should decide deliberately, not bake in:

- ~SUSPECT lean (per the human's "no defaults, no values; only what the probe gives us"): an un-probeable
  mutator is ⊤, so it runs. `useradd deploy` runs, returns its real rc at apply-time, and `|| mkdir` fires
  on its own. Convergence-elision still applies where a mutator's status is not branch-consumed (a bare
  `useradd deploy`, converged and ambient, can be replaced by a no-op whose dead status nobody reads).
  Under this reading there is no converged-rc declaration, which drops round-19's `(exit 9)` value-
  preserving substitution. That is not a claim the substitution was wrong — it is built, tested, and the
  fold direction under it is right (`19E`); it is the deliberate cost of the no-values ruling: you give up
  eliding a converged branch-consumed mutator and let it run. No safety is traded — the
  undeclared-rc-blocks-elision gate still prevents the under-execute the `(exit 9)` machinery also prevents
  — only that one elision is lost.
- The alternative is an oracle declaration of `fact-state → observable` ("when the user exists, `useradd`
  yields 9"), enabling a value-preserving elision of the mutator. Richer, but it is the kind of declared
  value the human is currently rejecting.

Either way, +SURE: a mutator's rc is never inferred or defaulted; it is a genuinely probed value or it is
⊤ ⇒ run.

### 2.4 Cross-oracle identity — many commands, one named kind (`17N §5`)

```sh
apt_get__check() { … pkg : com.debian.apt.Package = "$1"; dpkg-query -W "$pkg"; }
dnf__check()     { … pkg : com.debian.apt.Package = "$2"; rpm -q "$pkg"; }   # different command, same kind
```

The named kind (the reverse-DNS handle, a lifted datum — `175 C2` / `17N`) is the coordination vocabulary:
two unrelated commands resolve to the same cell, so an apt-establish and a later dnf-query coordinate even
though the engine stays referent-agnostic about the commands. A shared arg-token could not bridge them
(`17N §5`); the kind does. The per-command argparse differs (`$1` vs `$2`, different grammars) — which is
exactly why identity is command-keyed (only the oracle knows its tool) while the kind is cross-oracle
(`19A §5`'s three layers: command-keyed invocation, named-kind identity, fact-converged license). ~SUSPECT
residual (`inc-9`): two oracles annotating one kind must agree on its meaning, and Dorc cannot enforce
that — a CI-lint, never a checked property.

### 2.5 The selector comes from the verb

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

The annotation gives kind and entity (`service:nginx`); the verb selects the cell (`#enabled` vs
`#active`). `enable` and `start` touch different cells of the same entity, and neither discharges the
other — the round-19 selector regression, made to fall out of the `check()`'s own `case $verb`. The `≥enum`
floor (`17O F-BLESSED`) is why a service probe is genuinely two read-only commands; the contract-lifting
must let one `check()` carry several selector-probes, not collapse a service to one bit.

### 2.6 The quality bar the lifting must preserve (`17O`)

Because the `check()` body is the read-only probe, the `17O` regression class is contract, not engine
holes, and the lifting must not paper over them: `R2-SHADOW` (`command -v` confirms an executable file,
not a shadowing function); `R2-IDCACHE` (group membership via `getent group` field-4, never the stale
`id` cache); `R2-ORTRUE` (refuse to read an errexit-masked rc — `… || true` forces 0 — as a verdict);
`F-GETENT-HOSTS` (`getent hosts` routes to live DNS, non-hermetic — read-only is not hermetic). These are
why the `check()` is authored, not generated.

### 2.7 The static ⟷ dynamic duality

One `check()` body is read twice. The pre-probe (static) read takes the argparse and annotation and yields
the cell, plus which read-only body to ship. The post-probe (dynamic) read is the shipped body run on the
host, yielding the concrete observable the fold flows. Round-19 built consumers of the dynamic half but
fed them fixtures, because the static half — resolve the entity, pick the body, run it — was the stand-in.

---

## 3. What take-3 carries forward from round-19

Designs the spike validated (against injected inputs and the find-3 stand-in for identity — real, but
stand-in-fed), to layer onto the value analysis rather than re-derive:

- The one observable-tuple over channels (effect / status / stdout / stderr) — the right shape for "the
  check produces a per-channel value or ⊤".
- The apply fold — abstract-interpretation over probed values; rc opaque; ⊤ ⇒ run — as the seed of the
  post-probe value analysis (widen its domain, feed it real results).
- The cell-model (kind / entity / selector) plus the reaching-defs ambient gate — the poison-wall fix; the
  keystone output. Note its entity coordinate is exactly the half still riding the find-3 stand-in
  (selector/kind discrimination is validated; entity is resolved only because the flag-strip happens to
  pick the literal operands in the fixtures).
- The deterministic host simulator and the e2e corpus, plus the executable (`dash -n` and, for some cases,
  real mocked exec) acceptance gate — the measuring infrastructure the next step extracts and de-crufts
  into the take-3 stick.
- The `17O` quality bar (§2.6), kept as regression contract.

---

## 4. Open questions (several for a prior-art run, before any rebuild)

- The algorithm and substrate. How rich the value-flow needs to be, how it terminates and how precise it
  is, and whether it rides the existing monotone worklist or wants a different engine — undetermined here.
  The prior-art run should ground this. The only thing this doc asserts firmly is the apply-direction
  degrade-to-⊤ floor (§1.3) — and even that is one of three guarantees: the probe-direction inertness and
  the propagation-correctness it names are separate, and harder, and not provided by the floor.
- `fork-mutator-rc` (§2.3): an un-probeable mutator as ⊤ ⇒ run (current lean) vs. an oracle-declared
  `fact-state → observable`. Decides whether any "declared value" exists at all.
- `fork-annotation-spelling` (`ch-shape-anno` / `kTYANNOT`): the inline `pkg : Kind = "$1"` form breaks the
  off-ramp under stock dash (`17O F-OFFRAMP`) and wants a strip/transpile pass; the eol-comment
  alternative re-opens `kOOB`'s no-comment-config. The one non-sh-native token in the scheme; its spelling
  is the live `kTYANNOT` decision.
- Where the complexity dial sits (§1.3), and whether oracles and books really can share one threshold —
  worth confirming, though the machinery is shared by construction.

---

*The round-19 spike's durable product is this finding plus the `notes/19*` record. The immediate next step
(per the human) is to extract the round-19 corpus, strip the cruft that only fills in for the stand-in
build (injected-rc fixtures, identity-inference-dependent cases, masking tests), and turn it into the
clean acceptance measuring-stick the take-3 rewrite is graded against.*
