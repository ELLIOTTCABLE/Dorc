# 218a — door-2/door-4/precedence: the independent clean-context design (verbatim)

> Provenance: produced 2026-06-11 by an independent design subagent (Fable-tier, clean
> context, read-only) launched by the round-21 close orchestrator with the 212 rulings
> pinned as hard constraints and NO sight of note 218 (the orchestrator's own design) —
> the judge-panel/adversarial-prompting pattern from IMPLEMENTATION.md. Preserved
> VERBATIM below (transport escapes de-escaped, nothing else touched) because its
> divergence register (div-1..5), hunt-list (hunt-A..K), and unsettled list (u-1..12)
> are load-bearing inputs for any implementing round. Note 218 §9 records which elements
> the orchestrator adopted into the synthesis and which stay open. NEVER-VOUCH applies:
> this is an AI design artifact, process evidence only, not human-reviewed.
> Implementing rounds read 218 AND this note together; where they disagree and 218 §9
> is silent, the disagreement is OPEN, not resolved.

---

# Independent design: door-2 (declared converged-run) · door-4 (guard-insertion) · the precedence seam

Designed against HEAD `e512cc3`, honoring the 212 rulings as hard constraints. All sh herein is frozen strawman, never to be executed. Confidence markers per claim. Section slugs: `d2-*` (door-2), `d4-*` (door-4), `ps-*` (precedence seam), `hunt-*`, `u-*` (unsettled), `div-*` (deliberate divergences from program docs).

## 0. Constraint base (what this design treats as welded)

The 212 rulings: dq-errexit-2 genuinely open (ALL THREE bare-middle owners live in mechanism, naming, tests); dq-errexit-3 directional (door-4 behind a CLI flag, default `Never` provably-zero-transforms, boundary-(3)/correlated-failure frame, builds LAST, product hard-defers); dq-errexit-1 evidence-driven and untouched here. Plus: inv-probe-sourced-values' reserve clause ("an oracle-declared fact the human has explicitly sanctioned — none currently exist", spike/CLAUDE.md:191) is exactly the slot door-2 fills; weld-5 stands; rc stays opaque to Dorc (the *oracle* declares which value means what — door-2 is this ruling's intended consumer, +SURE); TOCTOU-WONTFIX; order-sacred; "skip" banned; kOOB no-comment-config redline; kTYANNOT acceptable-debt inline for the spike.

One structural observation that shapes everything below (+SURE, traced): door-2 and door-4 split across the engine's existing **value-gate vs trust-gate** boundary. The value machinery (`consumption_ok`, plan/src/lib.rs:540) already accepts a *declared* rc — the `StatusRelaxable` block fires only at `Predicted::Top` (lib.rs:552), and the doc at lib.rs:261-267 explicitly reserves "the fold's declared-rc opt-in, 19A §5". So door-2 needs nearly zero new *value* logic; what is genuinely new is the **trust/intent layer** (who consented to canary-removal) — and that is precisely the precedence seam. Designing them as one fused mechanism would be the mistake; this design keeps three layers: (L1) declaration lift + storage (oracle crate), (L2) value-gate pass-through (plan crate, minimal), (L3) the policy module (all intent/trust questions, one module, hot-swappable).

---

## 1. door-2 — the declared converged-run

### d2-1. What the claim is, precisely

A door-2 declaration is the oracle-author's counterfactual, entity-generic claim:

> "For any entity E such that my probe for `(kind, selector)` reports *holds*, re-running `<provider> <verb> … E` would mutate nothing and would yield rc `N` (and, descriptively, this stdout)."

Two claims fused, and the design keeps them named separately because they fail separately:

- **claim-noop**: converged-per-my-probe ⇒ the re-run is a no-op. This is *sharper than* the probe's existing three-outcome contract — the probe says "state holds"; claim-noop says "AND this verb does nothing then." They can diverge: `dpkg -s nginx` holds, yet `apt-get install nginx` on an *outdated* nginx upgrades it (see hunt-A — this is live on the flagship oracle).
- **claim-rc**: the no-op run's exit status is exactly `N`. Tool-knowledge, the weld-1..5 conjunction's missing arm.

Door-2-static consumes both; door-4 consumes claim-noop and (conditionally) claim-rc's conformance bit (d4-3). Neither door ever consumes a declared stdout *value* (d2-4).

### d2-2. The sh spelling (acceptable-debt inline, kTYANNOT precedent)

A vouched-body function, parallel to `oracle_probe_*`, keyed by **(provider, verb)** — not (kind, selector):

```sh
# oracle file (package.oracle.sh), alongside the existing declarations
oracle_kind=package
oracle_probe_package() { … }                                  # unchanged
oracle_effect apt-get install establish installed             # unchanged

oracle_converged_run_apt_get_install() {
   printf '%s is already the newest version.\n' "$1"
   return 0
}
```

Verbless providers use the bare provider segment (ε-verb parity with `empty_verb`, oracle/src/lib.rs:269):

```sh
oracle_converged_run_useradd() { return 9; }    # non-conforming tool, declared honestly
```

**Why (provider, verb) keying and not (kind, selector)** (+SURE): the claim's subject is a *command family's re-run behavior*, and weld-5's own counterexamples are per-tool (`useradd` rc 9, `mkdir` rc 1 — 20V §1). Two providers establishing the same cell (`apt-get install` and `dpkg -i`, both → `package#installed`, fixture lines 26-30) have independently-true-or-false re-run claims. Keying by kind would let one provider's declaration license another provider's sites — a category error. The (kind, selector) coordinate is *recovered* through the effect-map at bind time (d2-3), so the kind still anchors blame and the probe.

**Funcname resolution without guessing** (+SURE this is total): the suffix after `oracle_converged_run_` is matched against `to_funcname_segment(provider) + "_" + to_funcname_segment(verb)` (and bare provider-segment for ε-verb) for *each `oracle_effect`-declared pair in the same file* — the same anchored-matching trick `bind` already uses for per-selector probes (oracle/src/lib.rs:561-591, `strip_prefix(&kind_seg)`). The `_`↔`-` mangling routes through the one shared home (`check::map_provider_name`, check.rs:56 / `to_funcname_segment`, lib.rs:297), inheriting its known lossiness and its round-trip diagnostic pattern (`tc-perselector-mangle`). An unmatched `oracle_converged_run_*` is a loud lift error, never a guess — `inv-referent-agnostic` holds (we match against declared structure, never decode meaning).

**Body dialect** (the same move as the check dialect, check.rs:13-19 — "the dialect is NOT arbitrary sh"): the body must be a sequence of zero-or-more `printf`/`echo` simple-commands followed by exactly one final `return <integer-literal>`. Anything else (a command substitution, a branch, a real command) ⇒ per-function lift failure ⇒ declaration void ⇒ sites run (`kFAIL-perform`). Rationale: the engine must extract `claim-rc` statically and totally; a constrained dialect does that in ~30 lines of AST-walking with no evaluator. ~SUSPECT this floor is too tight for realistic declared outputs (multi-line text wants a heredoc) — u-3.

**Lifted representation** (new on `KindIndex`, oracle/src/lib.rs:126):

```rust
converged_runs: BTreeMap<(ProviderId, Symbol), ConvergedRunDecl>,

pub struct ConvergedRunDecl {
    pub kind: KindId,            // the file's oracle_kind — the blame anchor (m-2)
    pub selector: SelectorId,    // recovered from the matching Establish effect cell
    pub rc: Rc,                  // claim-rc, from the final `return N`
    pub stdout: Option<OutClaim>,// the printf text, carried for m-6 ONLY — never licenses
    pub body: String,            // verbatim, for the m-4 author-harness + provenance
    pub fingerprint: DeclPin,    // m-3, d2-6
}
```

**Lift-time coherence gates** (each a crate-local `DiagCode` const, the existing oracle-crate pattern at lib.rs:274-284; fail-soft per `inv-no-throw`):

- `oracle-convrun-out-of-dialect` (Error): body violates the dialect ⇒ dropped.
- `oracle-convrun-unanchored` (Error): suffix matches no `oracle_effect`-declared (provider, verb) in this file ⇒ dropped. (Also the typo-catcher.)
- `oracle-convrun-wrong-polarity` (Error): the matched effect cell is `Query` or `Kill`. A Query needs no counterfactual (its substitution is probe-sourced already, task-D2); Kill-convergence ("already absent") is *deferred, not irrelevant* — the exclusion-check rule applies: at HEAD a Kill site is `MustRun` and never probed (plan/src/lib.rs:735-737), so there is no convergence verdict for a declaration to condition on. When Kill-elision lands, this same spelling extends; until then, declare-on-kill is refused loudly so it cannot silently mean nothing.
- `oracle-convrun-multicell` (Error): the (provider, verb) declares >1 effect cell. A multi-cell verb is `MustRun` at HEAD (lib.rs:1042-1045 path), so a declaration on it is dead weight that would silently activate if multi-cell classification ever lands — refuse now, loudly.
- `oracle-convrun-conflict` (Error): a *second* declaration for the same (provider, verb) — within a file or across `-o` files — with a differing `rc`. Disposition: **both dropped**, not first-writer-wins. ~SUSPECT, deliberately stricter than `us-effectmap`'s first-wins for effects (lib.rs:165-191): a declared rc is correctness-load-bearing under errexit, and two declarers disagreeing is itself evidence the claim is rot-prone (the Liquibase-checksum lesson, 222 §4); refusing both degrades to r-2 behavior, the safe floor. Recorded for builder/human reversal (u-7).

### d2-3. Which channels a declaration may sanction — adopt 222 p-1, sharpened

- **rc (Status): sanctioned.** At an eligible site the status fed into the license check becomes `Predicted::Value(decl.rc)` instead of `Predicted::Top`. This flows through the *existing* gate unchanged: `StatusRelaxable ∧ Value(N)` already passes `consumption_ok` (lib.rs:552-553), and the stand-in machinery already reproduces any rc (`StandIn::from_rc`, lib.rs:595 / `render::standin_sh`, render.rs:65). Door-2 changes the *provenance* of a status prediction, not the gate.
- **stdout/stderr: NOT sanctioned for consumption.** A consumed `Stdout`/`Stderr` blocks unconditionally exactly as at HEAD (lib.rs:542-544), declaration or no. This is p-1's fence around the reborn assumptions-problem (222 c-2: declared text feeding control flow is how Chef died). The declared text exists *descriptively* (m-6, d2-5).
- **Effect: never.** Convergence comes from the probe verdict alone; a declaration can never substitute for probing under door-2. (`can't-probe ⇒ can't-elide` is untouched; what door-4's m-c does instead is run the probe body *at apply* — a different mechanism, not a relaxation of this one.)

**div-1 (deliberate divergence from the 20V §4 sketch):** 20V sketches the declared body *becoming* the stand-in ("a vouched body the stand-in becomes"), i.e. the artifact prints "is already the newest version." at apply. This design does NOT ship the printf. The stand-in is the minimal `StandIn::from_rc(decl.rc)` (`true` / `(exit 9)`); the text surfaces only in the provenance comment. Rationale: (a) under p-1 the printed text is never load-bearing, so shipping it buys nothing the gate recognizes; (b) printing imitation tool-output *manufactures evidence of a run that did not happen* — an admin grepping an apply transcript for "newest version" would be deceived in precisely the way the no-fabricated-values weld exists to prevent, even though the bytes are declared rather than engine-invented; (c) byte-exact tool output is the most rot-prone claim-shape in the whole 222 survey (locale/version drift, p-1's LANG note) — emitting it invites trusting it; (d) reusing `StandIn` keeps the render delta zero. Counter-argument recorded for the human: replay-fidelity of the apply transcript (a `tee`'d log resembling a real run) was plausibly part of 20V's intent. ~SUSPECT my side of this is right; it is cheap to flip later (one arm in the disposition layer).

### d2-4. Site eligibility and the mint path (the value half; the trust half is §3)

A site S is door-2-*eligible* iff (all existing machinery, names cited):

1. `class = SkipClass::EstablishAmbient(fact)` — single-cell, ambient (`EstablishWritten` refuses: the probe's verdict is stale by same-run writes, and the declaration is conditioned on converged-*now*, which we then cannot know).
2. `Grade::Must` (inv-must-may, unchanged conjunct).
3. Probe verdict for `fact` = `Converged` (PhasedVerdict lock unchanged; `Unknown`/`Diverged` ⇒ run).
4. No ⊤-successor (`has_top_successor`, lib.rs:1340; 16G hole-5 parity); not in-loop (`cfg.in_loop_body`, the task-L1 floor); not heredoc-refused at render (lib.rs:1790).
5. **Status consumers ⊆ {errexit-implicit} ∪ {StatusInvariant}** — the new consumer-provenance discrimination, see ps-2. An explicit reader (`&&`/`||` operand, `$?`-predecessor, `if`/`elif` guard, loop condition) refuses per the precedence ladder regardless of the value machinery being able to reproduce the rc. (This is deliberately *more* conservative than `consumption_ok` alone would be — the trust gate rides above the value gate.)
6. Consumed `Stdout`/`Stderr` ⇒ refuse (unchanged); `StatusIterated` ⇒ refuse (unchanged).
7. A declaration exists for the site's (provider, verb), unexpired (d2-6), and **decl.rc == 0 when the consumer set includes errexit** — see d2-4a.
8. The policy module resolves the bare middle toward the declaration (ps-3) and the door-level flag admits door-2 (ps-1).

The mint is a new private sibling of `prove_query_replaceable` (lib.rs:341) — `prove_declared_replaceable(fact, decl, verdict, consumed) -> Option<ReplaceLicense>` — minting `via: LicenseVia::DeclaredConvergedRun` (a NEW variant on lib.rs:167's enum; the dashboard's `static-declared` column, 21B §1, keys on exactly this, never blurred with `ConvergedEstablish`). The `Derivation` (lib.rs:197) gains an optional `declaration: Option<DeclRef>` display-only field carrying {kind, provider, verb, rc, fingerprint}. Disposition stays `Replace(license, StandIn::from_rc(decl.rc))` — door-2 IS an elision (a value-preserving replacement), so it needs no new `Disposition` variant; that vocabulary change is door-4's alone (d4-1).

**d2-4a — the fabricated-abort gate (a finding this design adds; not in 20V):** if `decl.rc != 0` and the site is errexit-consumed, the static stand-in `(exit 9)` *faithfully reproduces a crash* — the converged book under `set -e` would genuinely have died at this line (that is the tool's declared converged behavior). Minting it is value-faithful but useless-to-hostile: it converts "elide the work" into "guarantee the abort the real run would have had", and — the unreliable-oracle cell of the exclusion check — if the declaration is *wrong* (tool actually exits 0), the engine has **injected an abort into a healthy book**, a novel-behavior failure adjacent to breaking trust-boundary (3) from the *static* side. Disposition: door-2 refuses when `decl.rc != 0 ∧ errexit-consumed`, with a loud catalogued Note explaining that the book, as written, cannot complete on a converged host (which is itself high-value plan-time information for the admin). Non-zero declarations thus serve as conformance documentation and door-4 gating input (d4-3), and never static-elide in-spike. -GUESS this surprises the human; flagged prominently because 19A §5's `(exit 9)` language pointed the other way — but that language predates the precedence ladder, and the live path for mutator `Value(N)` statuses is dead code at HEAD anyway (+SURE: the cli firewall pins mutator status to ⊤ unconditionally — `ProbeSiteKind::Establish` rc "feeds the fold NOTHING", lib.rs:670-681).

### d2-5. Where the counterfactual surfaces (m-6) — three surfaces, all existing seams

- **Artifact comment** (the human-reads-here surface): extend `render::apply::provenance_comment` (render.rs:291) so a door-2 edit's disclosure carries the declaration text, not just "already converged":
  `true   # dorc: elided [apt-get install -y nginx] (probe: converged; oracle package declares re-run ⇒ rc 0 "nginx is already the newest version.")`
  Mechanically: `SpanEdit` (lib.rs:1513) grows an optional per-edit `why: String`; the group comment joins members' whys. Mind 21E: when `comment_safe` (lib.rs:1724) drops the comment, the OOB surface below still carries it (the standing artifact-over-prose rule, 21E res-3).
- **stderr/verdict lane** (the disclosure floor, 20V §5): one catalogued Note per door-2 elision — `DiagCode("dq-door2-declared-elision")`, registered in `core::diag::CATALOG` (diag.rs:69) with template + constructor per rq-1..rq-3, Note severity (satisfying the all-Note invariant test, diag.rs:223). Template carries: site id, fact label, oracle kind, provider+verb, declared rc, declared text, fingerprint. This is m-2's blame-template embryo: it names the declaring oracle at the moment of use.
- **Dashboard**: 21B's `static-declared` column populates from `LicenseVia::DeclaredConvergedRun`; the `#[expect(unreachable_patterns)]` arms in `coverage` fire at compile time when the variant lands (21B §1's designed tripwire), forcing the attribution update in the same change.

### d2-6. m-1 and m-3 mapping

**m-1 (tri-level support)** maps structurally, not as new syntax (+SURE this is the honest spike floor):

- *none* = no `oracle_converged_run_*` for the (provider, verb) ⇒ doors unavailable ⇒ sites run. Identical posture to Ansible's undeclared-module default (do-nothing-predict-nothing — though inverted into our apply lane: "runs, full stop", 222 §7's deliberate divergence).
- *partial* = the declaration's reach is already narrowed by two structural gates: (i) entity-resolution through the oracle's own `__check` argparse — an invocation the check refuses to annotate (multi-operand via the `[ "$2" = "" ]` guard, fixture line 43; unrecognized flags) resolves no fact, hence no `EstablishAmbient`, hence no door; (ii) per-(provider, verb) keying — undeclared verbs get nothing. So "partial with sh-spelled conditions" is the *composition* of check-narrowing with declaration-presence — conditions are spelled where the author already spells argv understanding. No prose `details:` field exists (it would be comment-config — kOOB).
- *full* = declaration present, check resolves broadly.

The known cost: this couples declaration-scope to probe-scope — an author cannot probe broadly but declare narrowly (u-8).

**m-3 (checksum-pinned declarations)** — the honest in-spike slice is **probe→apply fingerprint continuity**, not a persistent acceptance store (there is nothing persistent to pin against; kSTATE is unsettled and stays so):

- `DeclPin` = a dependency-free FNV-1a (or similar, inline, no crates — inv-no-unsafe/no-deps kernels) over the normalized declaration body + the matched `oracle_effect` line.
- The **probe artifact** gains one header comment-record per declaration consulted at compile-probe time: `# dorc-decl: apt-get install package@fnv:9a3e21 rc=0` (comment lines, dash-n-clean, zero execution surface — render::probe::header parity, render.rs:118).
- The **apply-side caller** (cli) parses these back from the probe-results channel context and compares against the declarations it lifted *now*. Mismatch ⇒ the declaration lapses for this run (r-2 behavior: door-1 only), with a Warning-class crate-local diagnostic `door2-decl-stale-pin` naming both fingerprints. This catches the real race the spike can exhibit: oracle edited between probe-render and apply-build.
- The `validCheckSum`-shaped re-vouch idiom and a host-side tool-version pin both defer (u-3, u-7-adjacent); the seam they would occupy is `DeclPin::matches`, one function inside the policy module.
- 222's FOD warning is honored in the negative: any future `--check`-like re-verifier must not inherit elision — recorded as a design note on the pin type, nothing to build yet.

---

## 2. door-4 — guard-insertion

### d4-1. The license-vocabulary change: a new category, and what keeps weld-5 intact

New `Disposition` variant (lib.rs:615):

```rust
/// door-4: prefix the leaf with the kind's vouched read-only probe, keeping the
/// original mutator as the `||` right operand. NOT a Replace: nothing is
/// reproduced from prediction; the decision is DEFERRED to a live runtime read.
GuardInsert(GuardLicense),
```

`GuardLicense` is a second witness type, *not* `ReplaceLicense`, with the same privacy lock (fields private; only constructor is `prove_guard_insertable` in the policy-gated path — the note-165 L2 pattern):

```rust
pub struct GuardLicense {
    fact: FactKey,
    guard: GuardSource,        // provenance-typed: WHICH bytes, from WHERE (d4-2)
    declaration: DeclRef,      // the sanctioning converged-run declaration
    derivation: GuardDerivation, // mint-policy arm, probe verdict at plan, consumer set
}
```

**The invariants that stop erosion of weld-5**, stated as compile-shaped rules:

- inv-g1 — *no value crosses the boundary*: `GuardLicense` carries **no `StandIn`, no `Predicted<Rc>`, no `Observable`**. The type cannot express "and the rc will be N". A future refactor that wants to fold a guard's "known" outcome into a value must construct a `ReplaceLicense`, which demands probe-provenance — the existing lock. (The temptation door-4 creates is exactly "a guard whose outcome we're sure of is just a fancy stand-in" — make that unspellable.)
- inv-g2 — *the original bytes always survive*: the render edit for a `GuardInsert` is `replacement = guard_prefix + original_span_bytes` — the mutator's own text is re-emitted verbatim inside the replacement (d4-5). There is no code path in which a `GuardInsert` removes the mutator. A `GuardInsert` whose original is dropped is unrepresentable at the edit-constructor (it takes the span and re-reads the source).
- inv-g3 — *guard bytes have one provenance*: `GuardSource` is constructed only from `KindIndex::resolve_probe` output (oracle/src/lib.rs:234) — the structurally-vouched per-(kind, selector) probe body, the identical resolution `compile_probe` uses (lib.rs:886). Never the `__check` argparse (st-2: placeholder check bodies must not ship, 20B §3), never engine-synthesized sh, never declaration-body text. The probe lane and the apply guard ship the *same* trust-object — 222 c-4's "the kind's author owns the kind's read-side" line, satisfied by construction.
- inv-g4 — *mint requires the declaration*: no declaration ⇒ no `GuardLicense`, even though the guard "would re-measure anyway". What the declaration buys is claim-noop (d2-1): the vouch that converged-per-this-probe ⇒ this verb is a no-op — without which the guard suppresses real work (the apt-upgrades-installed-packages case, hunt-A). The probe's three-outcome contract alone does not state claim-noop; HEAD's plain elision already leans on the oracle's *implicit* version of it, but door-4 runs at sites the engine could *not* previously elide, so the explicit vouch is the honest price of the wider reach. (+SURE this is the right reading of 20V's "mintable only when … declares converged-run-equivalence".)

### d4-2. The guard body: exact source, shipped form, output-silencing

The shipped form mirrors the probe artifact's wrapper mechanics — same emitters, same names, one audited home (task-R discipline, render.rs module doc):

```sh
#!/bin/sh
# dorc apply: …                                   ← apply_header (render.rs:246)
# dorc guard preamble (door-4): read-only checks vouched by their oracles
package_installed__check() {
   command -v dpkg-query >/dev/null 2>&1 || return 2
   st=$(dpkg-query -W -f='${Status}' "$1" 2>/dev/null) || return 1
   case $st in
      'install ok installed') return 0 ;;
      *) return 1 ;;
   esac
}

set -e
…
package_installed__check 'nginx' >/dev/null 2>&1 || apt-get install -y nginx   # dorc: guard-inserted (oracle package declares converged re-run rc 0; probe@plan: converged)
…
```

Decisions inside that strawman, each load-bearing:

- **Function preamble, not inline body** (+SURE forced): probe bodies use `return` (function context, an-probe-shape) — at script top level `return` is an error in dash, so inlining a brace group is broken, and rewriting the oracle's body is banned (oracle code is never edited by the engine). The preamble is emitted between the header and the book's first byte — a *block emission*, not a span edit, so the 21E edit model is untouched. One definition per needed `(kind, selector)`, first-seen-deduped, named by `check_fn_name` (lib.rs:757) — byte-identical naming to the probe artifact, so a human diffing probe vs apply sees one vocabulary.
- **Name-collision refusal**: if the book defines any function whose name collides with a needed wrapper (the AST knows every `FuncDef`), door-4 refuses those sites (`door4-name-collision`, runs verbatim) — sh function redefinition is last-writer-wins at runtime and a book-defined `package_installed__check` after the preamble would hijack the guard.
- **Entity binding** rides `render::probe::invocation` (render.rs:164) — `sem::single_quote`, the lone quoting decision (F-QUOTE), so a hostile entity (`x; rm -rf /`) stays one inert argument in the apply lane exactly as in the probe lane. Singleton cells invoke bare.
- **Output-silencing rule**: the redirect is applied **at the call site** — `… __check 'nginx' >/dev/null 2>&1 || …` — never by editing the body. Rationale: body output-discipline is unenforceable (bodies are author sh; inv-no-throw posture), while a call-site redirect is total and composes with any body. The *mutator's* channels are untouched (world-2 must be byte-faithful). The guard's rc passes through the redirect unchanged.
- **Why `||` and not `if`/`&&`** (+SURE, semantics verified by the door-3 work): the left operand of `||` is errexit-exempt (213 §2 d-4 — `lower_condition_region`/`clear_fallible_range`; POSIX exempts AND-OR non-final commands), so a *failing or can't-tell guard can never itself crash the book*: rc 1 and rc 2 both fall through to the real mutator. The three-outcome probe convention degrades exactly right with zero added logic: `holds(0)` ⇒ short-circuit; `absent(1)` ⇒ run; `cant-tell(2)` ⇒ run (the conservative direction). An `if ! …` spelling would be semantically equal but is more bytes and loses the idiomatic check-then-act shape the off-ramp story wants; recorded as rejected.
- **`set -u` hazard** (~SUSPECT, build-verify): the guard body executes under the *book's* shell options. A body that is probe-lane-clean can die under the book's `set -u` (an unset-parameter expansion error in dash is fatal even in errexit-exempt context, ~SUSPECT — must verify in-build). Conservative candidate: wrap the call `( set +u; package_installed__check 'nginx' ) >/dev/null 2>&1 || …` (a subshell fork, ms-class, restores the probe lane's effective environment). Optimistic candidate: bare call + an oracle quality-bar rule (R2-class) "probe bodies must be set-u-clean". Build feedback decides (u-11); the strawman above shows the optimistic form.

### d4-3. The four-world trace as design argument (with one sharpening 20V lacks)

- **world-1 converged ∧ healthy**: guard rc 0, mutator suppressed; the list rc is the guard's *live, re-measured* rc — no claimed value is consumed anywhere; errexit sees 0. Perf claim per 20V: a local read (~ms) replaces apt's lock+resolver seconds — and the read is *host-local at apply*, the network round-trip having been paid by the probe already, so door-4's runtime premium over door-2-static is one local exec, ≈nothing (this kills "door-2 is the fast one" as an argument — see u-4).
- **world-2 diverged-since-probe**: guard rc 1 ⇒ mutator runs. `kFAIL-perform` *by construction* — strictly dominates door-2-static under TOCTOU drift, which would have elided on the stale verdict. (TOCTOU stays WONTFIX as a *program*; door-4 just happens to be immune.)
- **world-3 converged ∧ env-sick** — the canary world, where this design sharpens 20V: door-4's canary suppression is **narrower than door-2's**. A sickness *visible to the probe* (corrupt dpkg db ⇒ `dpkg-query` fails ⇒ rc 1/2) falls through to the real mutator, which then crashes with the tool's own error — canary *restored*. Only sicknesses invisible to the read-side (held lock, full disk on the write path) are suppressed. Door-2-static suppresses all of world-3. This is an argument 222 c-3's "surviving family" claim predicts but 20V does not state; it should appear in the disclosure prose and the dashboard's door semantics. (+SURE for the dpkg example; the general claim is per-oracle.)
- **world-4 lying-check**: probe (and guard — same body) reports holds wrongly ⇒ under-execute. The pre-existing root trust, genuinely unwidened: the identical lie breaks HEAD's plain elision. What door-4 adds is the *correlated-failure surface* (212's frame): the same lying body now misleads at plan AND acts at apply — which is why it sits behind the flag, last, and why every door-4 site's disclosure names the oracle and fingerprint (hunt-K exercises this).

**The conformance gate** (`decl.rc == 0`): for a non-conforming tool (declared rc 9), world-1's *original* behavior under `set -e` is a crash; the guard-transformed book proceeds (guard rc 0). The transform would convert a crashing-on-converged book into a proceeding one — a behavior change beyond canary-removal. Door-4 therefore refuses when `decl.rc != 0` at an errexit-consumed site; the mirror-image rule to d2-4a. Net: non-conforming declarations currently gate *both* doors shut under errexit and exist as documentation — an honest, narrow floor (u-4 notes the cell).

### d4-4. The m-a / m-b / m-c mint-policy seam

`MintPolicy { ProbedConvergedOnly /*m-a*/, DeclaredAlways /*m-b*/, EvenUnprobed /*m-c*/ }`, consumed only inside the policy module's door-choice (ps-1). All three correctness-safe (the guard re-measures; 211 §7); they differ on:

- **m-a** (default per 211 §7): insert only where this run's probe verdict = Converged. Minimal wasted reads, minimal diff; the guard exists to absorb probe→apply drift. Disclosure: "probe says converged; guarded against drift."
- **m-b**: insert wherever a declaration + resolvable probe body exist, verdict regardless. First-apply cost: a wasted read at diverged sites. The under-argued upside this design wants recorded: the artifact is *durable* — an admin re-running the rendered apply (the off-ramp!) gets check-then-act semantics on every later run, Dorc-or-not. m-b is "Dorc as a compiler of idempotency guards", and it is the arm that maximizes the r-4 convergence story (d4-6).
- **m-c**: insert even where no probe verdict exists (probe lane skipped/partial). The guard becomes the *only* convergence mechanism — pure runtime check-then-act, the largest diff and the largest posture shift (apply-lane reads at sites never vouched *for this run*). Kept live in the enum because dq-errexit-3's taxonomy work needs it comparable, not because the spike defaults anywhere near it.

### d4-5. Render mechanics on the span renderer (21E-aware)

The key economy: **insertion is expressible as the existing `SpanEdit`** — `SpanEdit { lo, hi, replacement: format!("{guard_call} >/dev/null 2>&1 || {original}"), original }` where `original` is the span's verbatim bytes (lib.rs:1330's `command_text`). No new edit primitive; `normalise_edits`' disjointness (lib.rs:1527), `group_edits`' line-overlap closure (lib.rs:1577), right-to-left splicing (lib.rs:1672-1679), `comment_safe` + `region_ends_in_quote` (lib.rs:1724/1747) all apply unchanged. Consequences checked against 21E's failure inventory:

- A door-4 replacement *contains newlines* when the original is multi-line (quoted-newline operand): the group machinery already handles multi-line regions; the provenance comment lands after the region's last line; `comment_safe` scans the whole rendered region. The 21E P1 class (adjacent multi-line edits) composes — a door-4 edit abutting a door-2 edit joins one `EditGroup` and splices right-to-left; member disjointness is untouched because door-4 edits the same `[lo, hi)` a Replace would. Mandatory pole case: door-4 insertion on the closing line of a prior multi-line elision (the 21E reproducer reshaped) — hunt-H.
- Heredoc-bearing leaves: refused (d-6 parity, lib.rs:1482), though *technically* an insertion strands nothing (the `<<EOF` operator and body stay attached to the surviving original). Recorded as a future-lift asymmetry: door-4 could legally edit heredoc leaves door-2 cannot — not taken in-spike, uniform refusal is simpler and the corpus population is zero (~SUSPECT).
- The preamble: emitted by a new `render::apply::guard_preamble(defs: &[(name, body)])` emitter with the standard GUARANTEE doc (dash-n-clean iff each body is a brace group — the same precondition as `wrapper_def`, render.rs:148). The artifact with zero `GuardInsert` steps emits no preamble — byte-identity for flag-off is preserved structurally (ps-4).

### d4-6. Re-analysis closure (the fixed point, and the rule that makes it converge)

Feed the transformed book back into Dorc:

```sh
package_installed__check() { …dpkg-query…; }
set -e
package_installed__check 'nginx' >/dev/null 2>&1 || apt-get install -y nginx
```

- The preamble funcdef is a book-defined function; the call is arch-2's `InlineCall` territory (budget permitting). With the H2SaLS-style oracle declaring `dpkg-query`/`dpkg -s` as a Query cell, the inlined guard classifies `QueryResolvable` and the `||` becomes fold-resolvable — i.e. **door-4's output is rung r-4's input**: elision now provable-from-sh, zero declaration trust, the anti-cliff direction 212 demands stay measurable. Without a Query oracle for the probe's own commands, the call is opaque-ish ⇒ the line runs verbatim — safe, merely unimproved.
- **The convergence rule that must ship with door-4** (else the closure diverges): *already-guarded refusal*. At a candidate site, if the site's `AndOr`-left (looking through `InlineCall` body classes post-arch-2) contains a `QueryResolvable` whose `FactKey` equals the site's fact, the policy refuses with `Refuse(AlreadyGuarded)` — this is 20V §3's "already-guarded (door-1)" provenance arm made mechanical. It serves three masters at once: (1) idempotence — re-analyzing a transformed book never double-guards (`check || (check || apt)` never accretes); (2) the admin's own hand-written `dpkg -s nginx || apt-get install -y nginx` never receives a second machine guard (admin-explicit wins in the *guarded* direction); (3) the precedence ladder's arm-2 (ps-3). ~SUSPECT the same-fact test through an inlined call is the fiddliest part of the build (u-2); polarity matters (`check && cmd` is the kill-direction idiom and must NOT match — hunt-C).

---

## 3. The precedence seam

### ps-1. Shape: one module, data + pure functions, one choke point

`crates/plan/src/policy.rs` (pure, no I/O, kernel-clean — DST trivially). The cli constructs the policy from flags and threads it; `build_plan` (lib.rs:1073) gains the policy + a declaration-lookup closure as injected arguments, the same DI shape as `observe` and `compile_probe`'s `probe_body` closure — the kernel never reads flags (`inv-superposition`: the phased caller collapses; the policy IS the caller's collapse rule, packaged).

```rust
pub struct DoorPolicy {
    pub doors: DoorLevel,              // Never | StaticDeclared | InsertGuard   ← the CLI flag
    pub mint: MintPolicy,              // m-a | m-b | m-c (meaningful under InsertGuard)
    pub bare_middle: BareMiddleOwner,  // OracleDefault | EngineGlobal | AdminPerBook  ← ALL THREE LIVE
}

pub enum DoorChoice {
    Refuse(RefuseReason),              // AdminExplicitHandler | AlreadyGuarded | DeclMissing |
                                       // DeclStale | DeclNonConforming | BookSignalForcesRun | …
    Door3Invariant,                    // free, zero trust — predates this module, routed for attribution
    StaticDeclared(DeclRef),           // door-2
    InsertGuard(DeclRef),              // door-4
    NotApplicable,                     // r-0: runs
}

pub fn door_choice(site: &SiteCtx, policy: &DoorPolicy) -> DoorChoice  // THE choke point
```

`SiteCtx` carries only facts the engine already computes or this design adds: the consumer-provenance set (ps-2), the same-fact-guard bit (d4-6), the declaration lookup result + pin status, the probe verdict, ⊤/in-loop/heredoc bits, and the book-level admin signal (ps-3). Hot-swappability = every mid-round ruling lands as one arm or one default in this file; nothing outside it mentions ownership models. The CLI spelling, extending the hand-rolled parser (cli/main.rs:64): `--errexit-doors=never|declared|insert` (default `never`), `--door4-mint=probed|declared|always` (default `probed`), `--bare-middle=oracle|engine|book` (default — see ps-3 coincidence note).

**Level semantics**: `Never` = HEAD behavior, byte-identical (ps-4). `StaticDeclared` = door-2 only. `InsertGuard` = doors 2+4, the door-2/door-4 split per site decided inside `door_choice` (giving m-5's sampled-cross-check a future home as a policy swap, not a mechanism change). **div-2**: 212's flag ruling names door-4; whether door-2-static also hides behind the flag is ambiguous in the relays ("apply phase be a *pure*, direct, elision-only transform" — door-2 IS elision-only, suggesting it could ride unflagged; but "default `Never` must PROVABLY produce zero transforms" reads broader). This design gates *both* behind the one flag with separate levels — the most conservative reading, collapsible later by deleting a level. Flagged for the human rather than settled.

### ps-2. The consumer-provenance split (the one genuine engine change this program needs)

The ladder's first rung — "admin-explicit handler ⇒ refuse" — requires distinguishing *who reads the status*, which HEAD's `Channel::StatusRelaxable` deliberately erases (one mark for `&&`/`||` operands, errexit-region commands, `$?`-predecessors, if/elif guards — core/src/lib.rs:375). Do NOT split the Channel (its *value* semantics are correctly uniform — a known rc reproduces any of those reads); add a parallel per-node consumer-tag set on `Cfg`, sibling to `consumed_observables` (cfg.rs:286):

```rust
pub enum StatusConsumer { ErrexitImplicit, BranchOperand, DollarQ, IfGuard, LoopCond }
pub fn status_consumers(&self, id: CfgNodeId) -> &BTreeSet<StatusConsumer>
```

Each existing mark-site adds one tag insert: `materialise_errexit_edges` (cfg.rs:352) tags `ErrexitImplicit`; `lower_and_or` tags `BranchOperand` (and continues to mark door-3's `StatusInvariant` *instead*, 213 d-2 — a `|| true` left carries neither blocking mark nor `BranchOperand` tag); the `$?`-pred walk tags `DollarQ`; guard/condition lowering tags `IfGuard`/`LoopCond`. The eligibility predicate both doors share: `status_consumers(site) ⊆ {ErrexitImplicit}` (the empty set means the site already elides at HEAD with no door needed — door-2's population is precisely the errexit-only-consumed establishes, the 21B `needs-declaration` rung made mechanical, +SURE this matches the dashboard's lone groupadd site). The range-over-mark imprecision 213 res-3 found (chain over-marks) applies to tags too — the tag granularity is hunt-B's first target.

### ps-3. The ladder, with all three bare-middle owners genuinely live

Order of evaluation inside `door_choice` (first match wins; admin-explicit beats everything in BOTH directions):

1. consumer set ∩ {BranchOperand, DollarQ, IfGuard, LoopCond} ≠ ∅ ⇒ `Refuse(AdminExplicitHandler)`. The admin's own sh marked the rc live; **no declaration overrides it** — deliberately more conservative than the value gate, which *could* reproduce the rc (lib.rs:261-267): a wrong declaration suppressing a written `|| fallback` is the stacked-failure disaster, so trust refuses where value would permit.
2. same-fact guard governs the site (d4-6) ⇒ `Refuse(AlreadyGuarded)` — the admin already wrote it; doors never stack.
3. status consumed only as `StatusInvariant` (`|| true`) ⇒ `Door3Invariant` — the admin's "rc not load-bearing", free, zero trust (mechanism already at HEAD; routed through here purely so attribution sees one ladder).
4. the book-level admin signal, if present, **in both directions**: force-run ⇒ `Refuse(BookSignalForcesRun)`; allow-doors ⇒ proceed to 5 with consent established. Book-granular, still admin-explicit, still above any oracle default.
5. the bare middle, by `BareMiddleOwner`:
   - `OracleDefault`: declaration present ⇒ door per `DoorLevel`/`MintPolicy` (the declaration itself carries default-consent — the engineer answered the intent question tool-by-tool); absent ⇒ `NotApplicable`.
   - `EngineGlobal`: the declaration is *capability only*; consent comes from the run-level setting (operationally: the flag's level). YOLO's territory, bounded and disclosed.
   - `AdminPerBook`: doors fire only under an explicit book signal (arm 4's allow-direction); a bare book with declarations available still runs.
6. `NotApplicable` ⇒ runs (r-0).

**Coincidence to surface honestly** (+SURE it will confuse someone later): in a spike whose only consent surface is the CLI flag, `OracleDefault`-with-flag-on and `EngineGlobal`-with-flag-on behave identically — the flag is simultaneously "doors exist" and "engine-global consent". The three owners remain *distinguishable* in the type, the ladder arm, and the test matrix (a same-book triple of e2e cases, one per owner: under `AdminPerBook` the un-signaled book elides nothing even with flag-on and declarations present; under the other two it doors; under `OracleDefault` with a declaration-less oracle set, nothing fires even with consent). Nothing anywhere assumes oracle-ownership: the default `BareMiddleOwner` for the spike is **EngineGlobal** -GUESS — it is the only owner that adds zero new trust-surface beyond the flag the human already ruled into existence; recorded as trivially swappable (one line in `DoorPolicy::default()`).

**The two admin-per-book sh spellings (kOOB-clean candidates), with failure modes:**

- **spell-env — a recognized top-level assignment, region-traced:**
  ```sh
  #!/bin/sh
  set -eu
  DORC_DOORS=converged-run     # plain sh; assigns an unused var; no-op everywhere else
  …
  ```
  *Why it isn't comment-config*: it executes; it is lifted by the same literal-assignment machinery as `oracle_kind=` (oracle/src/lib.rs:430-460); and — the principled part — its *scope* is derived by control-flow-tracing exactly like `set -e` itself: the cfg's errexit pass already models on/off/⊤ regions per node (cfg.rs:311-318, `errexit_toggle` at 716); a `DORC_DOORS` region rides the identical pass, applying to sites after it in flow order. That is AGENTS.md's "user-configuration comes from (principled, contracted) control-flow-tracing" verbatim. Off-ramp: stock dash assigns a var nobody reads — inert; deletable in one line when offboarding.
  *Failure modes*: (f1) the kOOB judgment itself — a reserved env-name is the mildest dorc-ism, but it is config-spelled-as-sh, and 207 §4b explicitly left "is a recognized env-name config-in-disguise?" for the human — unsettled, surfaced (u-5); (f2) conditional/dynamic assignment (`if x; then DORC_DOORS=…; fi`, `DORC_DOORS=$Y`) ⇒ region ⊤ ⇒ refuse loudly (the `set "$dyn"` precedent, cfg.rs:317); (f3) a typo'd value or name silently no-ops — mitigated by reserving the `DORC_*` name-prefix: any unrecognized `DORC_*` assignment draws a Warning (a namespace dorc-ism, budgeted by DESIGN's "slight dorc-isms you'd omit idiomatically"); (f4) collision with a real tool reading the name — reverse-DNS-ify if pursued seriously; (f5) `export` variants and quoting variants need the lift to accept exactly the literal-assign shapes and ⊤-refuse the rest.

- **spell-helper — a no-op helper defined and invoked:**
  ```sh
  #!/bin/sh
  dorc_doors() { :; }          # one-line prelude; the off-ramp is this definition
  set -eu
  dorc_doors converged-run     # a real command in the CFG; argument = the consent vocabulary
  …
  ```
  *Why it qualifies*: pure sh; the signal is a *call site* in the CFG, so position-sensitivity and conditional-invocation tracing come free (a call under `if` is visible control flow; trace or ⊤-refuse); arguments give an extensible vocabulary without new names; "config is spelled in sh / library-code" — this IS library-code.
  *Failure modes*: (f1) two lines of ceremony, and the definition is mandatory — a book calling `dorc_doors` without defining it **breaks standalone** (command not found), so the off-ramp depends on the admin keeping the def line, a sharper failure than spell-env's inert assignment; (f2) an admin "improving" the body beyond `:` makes the signal an effectful command — the engine reads only the call's literal argv either way, but the config/code line blurs; (f3) same typo-silence as spell-env, same prefix-Warning mitigation; (f4) arch-2 will try to inline it (harmless — body `:` — but the policy lift must key on the *call*, pre-inline).

  Rejected third option, recorded so nobody re-derives: any `set +e`-keyed signal — 20V's named anti-door ("spelled in sh but means the opposite; pursuing it teaches admins to weaken their books").

  My lean: spell-env with region-tracing (~SUSPECT), because it is one inert line riding two existing mechanisms (literal-assign lift + region pass) and has the gentler off-ramp failure mode; spell-helper is the stronger citizen under the "everything is a command" philosophy. Both are presented because the kOOB call is the human's.

### ps-4. Default `Never` and the provably-zero-transforms test shape

Four layers, stated as process evidence, never proof (the never-vouch rule):

1. **Structural**: `DoorLevel::Never` short-circuits `door_choice` before any declaration lookup; the only constructors of `StaticDeclared`/`InsertGuard` choices live behind that match; `GuardLicense`'s sole constructor takes a `DoorChoice::InsertGuard` by value. Flag-off, the new code is unreachable by construction, and the preamble emitter is driven only by the (empty) set of `GuardInsert` steps.
2. **Unit, exhaustive**: the `SiteCtx` discriminant space is small and finite (consumer-subset × decl-present × pin-state × verdict × owner × level) — enumerate it; assert `Never ⇒ choice ∈ {Refuse, Door3Invariant, NotApplicable}` over every cell, all three owners included (the all-three-live obligation lands in this matrix's axes, not in any default).
3. **Pipeline differential (the load-bearing one)**: a harness-level gate in `e2e/run.sh` — every case runs twice, bare and with `--errexit-doors=never`, stdout byte-compared; plus the round's standard evidence, zero churn across all existing goldens (the 213/215/21E precedent). This is the same artifact-level "pure elision-only transform of their code" the human asked for, made mechanical.
4. **Runtime tripwire**: `debug_assert` in `build_plan` — `Never ⇒ no step is GuardInsert ∧ no license via == DeclaredConvergedRun` (the 21E `spliced_count` pattern).

---

## 4. My adversarial hunt-list (what I would attack in a build of this design)

- **hunt-A (HIGHEST — semantics, not code): claim-noop is false for the flagship oracle as naively written.** `apt-get install nginx` against an installed-but-outdated nginx *upgrades it*; `dpkg -s nginx` reports holds. A package oracle that ships `oracle_converged_run_apt_get_install` with the obvious probe vouches a no-op that isn't one — door-4's guard then suppresses real upgrades (and HEAD's plain elision already does! — verify the unwidened-trust claim by constructing the pole at HEAD first). Attack the m-4 author-harness story with exactly this tool; decide what the *honest* package oracle declares (a version-aware probe? declining the declaration? a distinct `#version` selector?). If the canonical oracle can't honestly declare, door-2/4's reachable population collapses and the design must say so.
- **hunt-B: the consumer-provenance tags (ps-2) mis-tagging one site = the disaster class.** A `BranchOperand` site tagged errexit-only lets a declaration suppress a written `|| fallback`. Attack with: `cmd; rc=$?` (DollarQ pred-walk range), `a && b` middles, `! cmd` under `set -e` (negation exemption), chains (`a || b || true` — 213 res-3's over-mark now mis-tags too), `$(cmd)` assignment-statuses (cfg.rs:665 find-6), `cmd &`, case-words, and every shape under both `set -e` regions and toggled-off regions. Construct against dash, not the engine's self-report.
- **hunt-C: the already-guarded test's false matches.** Same-kind-different-entity guards (`check 'curl' || apt-get install -y nginx` must NOT refuse); different-selector guards; the polarity inversion `check && cmd` (kill-direction — must not match); the guard buried in an inlined wrapper (post-arch-2 InlineSite walk); a guard whose fact resolves only through depth-2 positionals (217's correction says those refuse inlining — the test must then conservatively not-match, which double-guards: acceptable? prove the failure is cosmetic).
- **hunt-D: flag-off byte-identity attack.** Hunt any reachable delta with `Never`: preamble emission with zero inserts, Derivation shape changes leaking into flat-render text, diagnostic ordering shifts from the lift's new passes (lift diagnostics print even when doors are off — does a malformed `oracle_converged_run_*` in an oracle file now error a previously-clean case? It should — lift hygiene is flag-independent — but then "zero delta" needs the precise statement: zero *stdout* delta, stderr may gain lift diagnostics for files that carry new declarations).
- **hunt-E: door-2's fabricated-abort cell (d2-4a) from the other side.** I gated `decl.rc != 0`; attack the gate's completeness: a *wrong rc=0 declaration* on a tool whose converged re-run exits non-zero suppresses the canary identically to the accepted trade — fine — but a wrong rc=0 declaration at a site whose status later becomes admin-handled via a path the tags miss (hunt-B compositions) is the stacked failure. Also attack `StandIn::Exit` reachability: assert no path mints a non-zero declared stand-in at all in-spike.
- **hunt-F: pin continuity spoofing/rot.** Apply consuming probe-results whose `# dorc-decl:` records are missing (old probe artifact), duplicated, contradictory, or hand-edited; two oracles declaring the same (provider,verb) where one was edited between phases; fingerprint collision laziness (FNV is non-cryptographic — fine for drift-detection, but say so in the doc and attack the assumption).
- **hunt-G: policy bypass.** Grep-level: any construction of `LicenseVia::DeclaredConvergedRun`/`GuardInsert` outside the policy-gated path; hostsim/tests injecting `Observable.status = Value(n)` for mutator facts (the anti-masking discipline, spike/CLAUDE.md:196 — a test handing the engine a declared rc the lift should produce is masking).
- **hunt-H: render composition with 21E.** Door-4 insertion as a group member with: a preceding multi-line door-2 elision (the P1 shape reshaped); an entity operand containing `'` (single_quote emits `'\''` sequences — `region_ends_in_quote` must scan the *guard prefix* correctly); insertion on a `done; cmd` shared-keyword line (21E res-2); comment-drop paths where the OOB record must still carry the declaration text (verify the lane actually receives it — the artifact-comment and stderr paths are built independently and can drift).
- **hunt-I: the e2e exec-gate semantics of door-4.** The guard's body commands hit the mocks (`expected.ran` gains `ran: dpkg-query …`) — assert the *exact* expected run-sets for all four worlds under mocks: world-1 guard-only; world-2 guard-then-mutator in book order (order-sacred); world-3 simulated by a rigged mock probe (rc 2) ⇒ mutator runs; world-4 (lying mock: holds, while "reality" diverged) ⇒ mutator absent from ran — the under-execute made visible and pinned as the *documented* residual, not a green lie.
- **hunt-J: dashboard attribution honesty.** `static-declared` and `guard-transform` columns populate; 21B's `#[expect(unreachable_patterns)]` tripwires actually fire on the new variants (compile-check, then the runtime bucket); the door-3-vs-door-2 discriminator order (21B's subtlest call) extended: a `|| true` site that ALSO has a declaration must bucket `dead-invariant` (door-3 is free; the declaration was unnecessary) — assert the ladder's arm-3-before-arm-5 ordering is visible in attribution.
- **hunt-K (mandated by 212): the boundary-(3)/correlated-failure attack.** One lying oracle (probe always-holds + declaration), one book, N sites, two phases: enumerate every user-experienceable failure surface at plan and at apply; verify every surface names the oracle, the declaration, the fingerprint, and the site (the m-2 template); then run the same attack with doors=`Never` and demonstrate the failure count returns to HEAD's baseline — the flag's value, demonstrated adversarially.

## 5. Decisions I could NOT settle without build feedback (confidence-marked)

- **u-1** (~SUSPECT buildable cheaply, -GUESS on precision): consumer-provenance tagging granularity — the existing marks are range-based and over-shared (213 res-3); tags may inherit chain over-marks badly enough to need the precise-sub-range work 213 deferred. Build will tell whether errexit-only sites are cleanly separable on the real corpus.
- **u-2** (-GUESS): plumbing the site's (provider, verb) + same-fact-guard bit into `SiteCtx` — whether `classify` grows a parallel output cleanly or this drags a wider re-key. The closure-injection shape (cli resolves, kernel consumes) is the design intent; its ergonomics are unproven.
- **u-3** (~SUSPECT too tight): the declaration body dialect (printf/echo + final literal return). Multi-line declared output wants heredocs; build feedback from writing the 3-5 real H2SaLS-adjacent declarations will say whether the floor holds.
- **u-4** (the big product fork, surface to the human with dashboard numbers): whether door-2-static should exist at all. Door-4-with-m-a dominates it on TOCTOU-immunity and canary-narrowing (d4-3), at ~zero runtime premium (one host-local read); door-2's surviving advantage is purely trust-shaped (no new code runs at apply — boundary-(3) purity, the ruling's own axis). The safer-under-trust-taxonomy door is the less-reliable-under-prior-art door (222 c-3). I designed both behind one flag's levels so the dashboard + four-world cases can argue; I could not, and should not, settle it.
- **u-5** (human's, by definition): spell-env vs spell-helper for admin-per-book — gated on the open kOOB judgment from 207 §4b ("is a recognized env-name config-in-disguise?"). Both are designed; neither is default.
- **u-6** (~SUSPECT Note is wrong): severity of the per-site door disclosures. All-Note keeps the diag-catalog invariant (diag.rs:223) and the gate-3 floor quiet, but door-disclosures are the calibration mechanism (222 c-8) — they may deserve Warning prominence. The catalog test forces the deliberate change either way; the choice needs the human's warning-fatigue read.
- **u-7** (-GUESS): cross-file declaration-conflict disposition (both-dropped vs first-wins, d2-2) — both defensible; mine is stricter than the effects precedent.
- **u-8** (~SUSPECT acceptable in-spike): the m-1 coupling of declaration-scope to check-argparse-scope (probe-narrow ⇒ declare-narrow, inseparably). A real oracle wanting the split will surface in the build or not at all.
- **u-9** (-GUESS low population): in-loop door-4 — per-iteration check-then-act is semantically clean and the floor refusal is conservatism; 21F imp-2 says the loop population is corpus-thin, so lifting it is likely not worth the render risk this round.
- **u-10** (deferred by design): m-5 sampled door-4-under-door-2 — needs door-2 and door-4 coexisting in the field plus a plan-presentation determinism story; the `DoorChoice` seam is its prepared home.
- **u-11** (~SUSPECT real): the `set -u` interaction with inserted guard bodies (d4-2) — bare call vs `( set +u; … )` subshell wrapping; needs dash-verified behavior of unset-expansion errors in errexit-exempt contexts before choosing.
- **u-12** (--WONDER): whether `dq-errexit-1` gains a new cost-species entry from door-4 — the guard's apply-lane read is a p-5-class read-side action at a site the book never spelled (a per-site, serial, tiny instance of the Chef-systemd hazard). It is not canary-shaped and not run-evidence-shaped; possibly ledger-entry material, possibly nothing. Surfaced per the evidence-driven protocol, not asserted.

**Divergence register** (for the synthesizing orchestrator): div-1 — declared body does not ship as the stand-in (against 20V §4's sketch; rationale in d2-3). div-2 — door-2 gated behind the flag alongside door-4 (conservative reading of an ambiguous ruling; ps-1). div-3 — door-2 refuses non-conforming (`rc != 0`) declarations under errexit (d2-4a; new gate, not in 20V). div-4 — door-4's world-3 canary-suppression argued *narrower* than door-2's (d4-3; a strengthening of 20V's "same residue, no worse"). div-5 — declaration keyed (provider, verb), recovering (kind, selector) through the effect-map, where 20V's prose gestures at kind-level declarations (d2-2's argument).
