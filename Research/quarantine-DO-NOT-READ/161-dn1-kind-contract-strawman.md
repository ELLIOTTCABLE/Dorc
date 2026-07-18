# 161 — dn-1 strawman: the oracle kind / anchor / verdict contract

> **Status (2026-06-05): spike ph-1, the hinge.** 151 found that four rounds
> converge on one unwritten artifact — the sh idiom by which an oracle (a) NAMES
> the kind its predicate serves, (b) ANCHORS a sound skip, (c) reports its
> VERDICT — and defer it four times (DESIGN:199, 090:377, 099 §9, 09A:102). This
> note picks a buildable strawman so the `oracle`/`analysis` crates have a
> concrete contract. It is a **strawman for discussion** (like `Research/strawmen/`),
> not a commitment; the spike's job is to surface where it breaks. Confidence-marked.

## 1. The constraints (why this is hard, restated)
- **c-named-kind (W4 / X3):** cross-oracle identity (apt's `package` ≡ yum's
  `package`) must bind to a *named kind*, never a shared arg-token (token-equality
  is name-collision-prone across oracles). And it is a **3-place relation** (kind,
  provider, equivalence) — X3 proved no sh **naming convention** can carry it: a
  1-place namespace (`pkg__probe`) clobbers when two providers coexist.
- **c-in-band (kOOB):** spelled in sh, lifted by analysis — no YAML/frontmatter/
  pragma/comment-parsing. 151's de-risk: an analyzer-internal index *lifted from
  author sh declarations* does NOT violate kOOB (the redline is user-*config*
  form, not metadata transport).
- **c-offramp / no-op:** the oracle file should be ordinary sh — run/source-able,
  behaviourally inert if Dorc vanishes. X4 found the prior strawman's dotted
  verb-ladder (`apt-get.check`) **fails `dash -n`** ("Bad function name": `.` and
  `-` are illegal in POSIX function names). So the idiom must be dash-clean.
- **c-not-author-disciplined (X4):** the soundness-critical *frame* (kind anchor,
  verdict channel) must be machine-extractable, not riding on hand-rolled
  `case`/`grep` arg-parsing (ufw `.`-regex false-converge; apt-get `-o` leak).
  The irreducible *predicate body* (how to read `apt-get --simulate` output) is
  author knowledge we cannot machine-verify — but we can contain its blast radius.

## 2. The decision (spike strawman): blessed-anchor, statically lifted per-file
An oracle is a **plain sh file** that sets a few **blessed-name constants** and
defines a **blessed-name probe function**. Dorc lifts these *statically, per
file* (constant-folding / tainting — exactly the AST analysis the engine does
anyway) into an **analyzer-internal kind-index**. It never sources oracles into a
live shell, so the naming-convention clobber that killed `pkg__probe` cannot
occur — each file yields exactly one `ProviderDecl`.

```sh
#!/bin/sh
# oracle: apt-get  — serves kind `package` on Debian-likes.   [STRAWMAN]
# Plain sh: assignments + one function. dash-clean (passes `dash -n`). Inert if
# Dorc vanishes (the constants are unused; oracle_probe is just a function).

oracle_for=apt-get      # provider binding: a bare `apt-get …` in a book binds here
oracle_kind=package     # the NAMED kind (cross-oracle identity; W4). literal-only.
oracle_verb=install     # the sub-verb whose operands are the entity tokens

# probe: the read-only "would `apt-get install $@` change anything?" predicate.
# Dorc ships THIS BODY as the probe (Tier-B knowledge). Convention: exit 0 =
# converged (no change), 1 = diverged (would change). "unknown" is NOT an exit
# code here — it is the analyzer's/runner's verdict when the probe can't be
# lifted or can't run (see §4), which avoids the strawman's rc-overload bug
# (grep's empty-stream-vs-error conflation).
oracle_probe() {
   apt-get --simulate "$@" | grep -qE '^(Inst|Conf|Remv) ' && return 1
   return 0
}
```

What Dorc lifts → an internal `ProviderDecl { provider: "apt-get", kind:
"package", verb: "install", probe: <AST of oracle_probe body> }`. Any non-literal
anchor (`oracle_kind="$x"`, conditional reassignment, missing) ⇒ the file is **not
a valid oracle** (⊤ / diagnostic), never a guessed one.

### How a book binds (the transmutation)
Book (admin, pure sh) writes a bare mutator:
```sh
apt-get install -y nginx
```
Probe phase: Dorc resolves `apt-get` → the `ProviderDecl`, and projects the
**probe** `oracle_probe install -y nginx` (read-only). Verdict `Converged` ⇒ the
apply-phase elides the mutator; `Diverged`/`Unknown` ⇒ apply runs the *original*
`apt-get install -y nginx` unchanged. The oracle is a behavioural **no-op**: it
adds a probe, never alters apply (DESIGN "oracles surface, don't change").

The already-guarded book idiom needs *no* oracle for the skip itself — Tier-A
recognises `command -v nginx || apt-get install -y nginx` structurally (the
`command -v` probe narrows). The oracle's effect-decl is still what *links* the
guard's `nginx` to the package entity for cross-statement reasoning.

## 3. The entity-extraction problem (do NOT pretend it's a one-liner — X3)
`oracle_verb=install` + "the operands of `install`" is the entity rule. But
extracting operands means **stripping flags**, and flag grammars are per-command
(`apt-get -o Foo=bar`, `-t target`, `--option=…`). X3: "each command's footprint
is a mini-parser." Spike stance:
- Dorc owns a **generic POSIX-ish operand/flag splitter** (longopt `--x[=y]`,
  clustered shortopts, `--` terminator).
- It is **⊤-conservative**: an *unknown* flag-that-might-take-an-argument, or any
  non-literal operand (`apt-get install $PKGS`), ⇒ entity-set is ⊤ ⇒ the leaf is
  un-skippable / the probe is withheld. This **surfaces the apt-get `-o` hazard
  by construction**: `-o` is unknown-shaped ⇒ ⊤ ⇒ no false-skip (vs the strawman's
  silent leak).
- A provider may later *declare* its flag grammar to recover precision — a clean
  future `kBURDEN` lever, explicitly out of spike scope.

## 4. The verdict channel (three-valued, runner-produced)
`core::Verdict { Converged, Diverged, Unknown }`. The oracle's `oracle_probe`
emits only converged/diverged (exit 0/1). **`Unknown` is produced by the
analyzer/runner**, never by exit-code overloading, in these cases: the bound args
don't statically lift (⊤ entity, §3); the probe references undeclared-inert ops
(can't prove `kFAIL-withhold`); the host is unreachable / the probe errors or
times out (`vm-unreachable-is-unknown`). `Unknown` folds conservatively (apply
runs). This cleanly separates *oracle-quality bugs in the predicate body* (which
no frame can prevent) from the *frame* (which we machine-enforce).

## 5. Cross-oracle identity / equivalence
Two files both setting `oracle_kind=package` are **coherent providers of one
kind** — equivalence-by-named-kind, automatic, no shared token. *Entity*
equivalence across providers (apt `nginx` ≡ brew `nginx`, or the `libssl-dev` ≡
`openssl` alias zoo, 091's m×n) is **harder and deferred**: the spike treats
`(kind, entity-string)` as the identity within a provider and ⊤s/​hints any
cross-provider entity claim. Surfaces the alias problem without faking it.

## 6. Rust shape (the `oracle` crate + `core` additions)
```rust
// core (added now): named-kind + provider ids (interned, referent-agnostic).
pub struct KindId(Symbol);       // e.g. "package" — a name, never decoded for meaning
pub struct ProviderId(Symbol);   // e.g. "apt-get"

// oracle crate:
pub struct ProviderDecl {
    pub provider: ProviderId,
    pub kind: KindId,
    pub verb: Option<Symbol>,        // the sub-verb whose operands are entities
    pub probe: ProbeBody,            // lifted AST of oracle_probe (shipped as sh)
    pub anchor_span: Span,           // provenance of the declaration
}
// The analyzer-internal index (the dn-1 artifact). NOT a namespace; a relation.
pub struct KindIndex {
    by_provider: BTreeMap<ProviderId, ProviderDecl>,  // bare-command resolution
    by_kind: BTreeMap<KindId, Vec<ProviderId>>,       // coherence / equivalence
}
```
`BTreeMap` (not `HashMap`) so iteration for output is deterministic
(`inv-determinism`). Lifting = a pass over a parsed oracle file that
constant-folds the three anchors and captures the `oracle_probe` body; non-literal
⇒ `Carrier`-diagnostic, no decl.

## 7. Alternatives considered
- **alt-naming-convention** (`apt_get__probe__package`): rejected — X3's 3-place-
  in-1-place clobber, plus dash-name mangling of `apt-get`.
- **alt-registration-calls** (`dorc_oracle apt-get package`): liftable too, but
  the funcs must exist as no-op shims for the off-ramp, and richer decls degrade
  into DSL-sentence args (`dorc_establishes apt-get install : package installed`)
  — *more* DSL-y than constants, and the kOOB "stay short of a DSL" steer cuts
  against it. Held as the fallback if constants prove too weak.
- **alt-infer-from-body** (no explicit kind): impossible — the chicken-and-egg
  (094 g3/g4); kind cannot be inferred without grounding. This is *why* dn-1
  exists.

## 8. Surfaced design problems (the actual ph-1 output)
- **find-1 (kOOB borderline):** are blessed constants "config spelled in sh"
  (forbidden) or "sh we lift" (allowed)? I read them as the latter (151's de-risk
  — analyzer-internal lift of author declarations), but a purist could object
  that `oracle_kind=package` is YAML-in-disguise. **The redline needs the human's
  ruling.** (Lean: allowed; it's an assignment the analyzer taints, not a sidecar.)
- **find-2 (oracle vs book recognition):** a file is "an oracle" iff it sets the
  blessed anchors. No location/extension marker. Is content-recognition enough, or
  is an explicit oracle/​book distinction wanted? (Lean: content is enough for the
  spike.)
- **find-3 (entity-extraction is a mini-parser, X3):** the generic flag-splitter +
  ⊤-on-unknown is sound but coarse; precision needs per-provider flag grammars (a
  real, non-trivial authoring burden — contradicts "oracles are dumb one-liners").
- **find-4 (predicate-body soundness is uncontainable by the frame):** ufw-regex /
  apt-get-`-o`-class bugs live in `oracle_probe`'s body; the frame can't prevent
  them. The *separate* `kFAIL-withhold` enforcement (prove the probe only calls
  declared-inert ops, or sandbox it) is the real defense — and it is NOT the same
  mechanism as the verdict channel. Build them separately.
- **find-5 (off-ramp is "inert", not "useful"):** the oracle file is *inert* sh if
  Dorc vanishes, but `oracle_probe` isn't independently *useful* (you'd never call
  it by hand). The off-ramp claim holds in the weak sense (no breakage), not the
  strong sense (drop-in utility). Honest framing for DESIGN.
- **find-6 (verb model is too thin):** `oracle_verb=install` assumes one
  entity-bearing sub-verb; real tools have many (`apt-get install/remove/purge`)
  with different effects (establish vs kill). The spike will need
  `oracle_verb`→effect-polarity, not a single verb. Flagged for the effect-decl.
