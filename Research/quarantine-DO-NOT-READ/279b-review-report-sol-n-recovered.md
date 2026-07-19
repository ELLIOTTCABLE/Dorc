# 279b — design-package review report (sol-n)

Review point: `9431ccb` (`worktree-agent-af67e0c672b0f437e`). Scope: the round-27
package (`Research/plans/270`, `Research/plans/271`, and `Research/notes/272` through
`278`) as the design gate for the rebuild. The reviewed files were not edited.

## Conclusion

The package has a strong safety center, but it is not ready to serve as the rebuild's
complete design gate. I would stop the gate on findings `279b-fd1` through `279b-fd5`.
Two can introduce execution authority without the intended author speech act; one leaves
the rebuild to invent the foundational comparison algebra; one lets partial dependency
disclosure carry a completeness license; and one directly contradicts the promised
stripped-oracle floor. The remaining findings can be resolved in the same language/block
briefs without reopening the package's sound decisions.

Severity means consequence if built, not estimated frequency. Confidence grades the
finding from the repository evidence, not confidence in a particular repair.

## Findings — most severe first

### 279b-fd1 — The PATH shim can become an attacker- or environment-selected evaluator

**Severity: CRITICAL. Confidence: HIGH.**

The runtime design rewrites trusted `dorc:sh` to the ordinary command name `dorc-sh`,
places a constructed shim at the front of PATH, and deliberately uses normal PATH lookup
for nested/transitive composition (`Research/notes/274-evaler-surface-and-reentry-token.md:38-44`,
`Research/notes/274-evaler-surface-and-reentry-token.md:63-69`). Its session smoke test proves that Dorc's shim is executable under the session's
initial PATH (`Research/notes/274-evaler-surface-and-reentry-token.md:172-182`). It does not
prove that a later authored environment transform still resolves that object.

The package notices `env -i PATH=…` but concludes that removing the prepend makes marked
delegation fail 127 and therefore fail safely
(`Research/notes/274-evaler-surface-and-reentry-token.md:235-240`). That conclusion is
false whenever the replacement PATH already contains another `dorc-sh`:

```sh
tool__predict() {
    env -i PATH=/opt/vendor/bin "$@"
}
```

If the analyzed guest begins with `dorc:sh`, the transformed probe begins with
`dorc-sh`; `/opt/vendor/bin/dorc-sh` can run instead of Dorc's pinned evaluator. This is
not limited to a dishonest oracle. An honest wrapper model can change PATH because the real
wrapper does. The engine then introduces a generic command name into that changed resolution
environment. In probe, the wrong executable can mutate; in apply, it can interpret the
payload differently. Both are Dorc-created, unattributable failures, contrary to the
plan-nonmutation promise (`IMPLEMENTATION.md:192-204`) and the rule that unattributable errors
are engine faults (`IMPLEMENTATION.md:470-481`, `IMPLEMENTATION.md:508-512`).

The older threat analysis already names this exact class: PATH invocation is injectable,
whereas invoke-by-content-hash is not
(`Research/plans/102-dorc-threat-model.md:110-116`), and it reserves the anti-injection seam
at `Research/plans/102-dorc-threat-model.md:172-174`. The new package silently crosses that
seam in the unsafe direction.

**Gate consequence:** do not make a PATH-resolved bare name the identity of a trusted reentry
object. Row-2 rewriting needs an unambiguous capability (for example, a securely materialized
absolute path carried explicitly). If row 3 retains PATH composition, its secure-directory,
ownership, path-rewrite, and collision behavior must be specified separately; session-start
smoke testing is not sufficient.

### 279b-fd2 — Unmarked role-name recognition can silently turn ordinary sh into probe authority

**Severity: CRITICAL. Confidence: HIGH.**

The reference makes the marker gate syntax only and explicitly recognizes `__role` names in
unmarked files (`Research/notes/278-dorc-lang-v0-1-reference.md:90-96`). Family membership is
derived only from names—never file or author—and the engine may extend the closed vocabulary
by adding new role names, while all such names are permanent and unversionable
(`Research/notes/278-dorc-lang-v0-1-reference.md:133-151`). Thus neither present-day plain sh
nor future role additions have an unambiguous opt-in boundary.

The inherited ruling accepts coincidental capture because `__` is rare and says a captured
function will misbehave loudly (`Research/notes/24M-language-rulings-and-remaining-ledger.md:51-61`).
That premise does not hold for the safety-bearing cases. Ordinary sh such as

```sh
backup__predict() { backup "$@"; }
```

can succeed quietly. Under the package's name-only recognition, the same function is no longer
an opaque helper: it is author-vouched modeling code eligible to run during probe and feed
licenses. A similarly captured `__is_converged` can quietly return 0. In addition, a future
engine adding a role name can reinterpret a previously ordinary unmarked file even though the
file has no language-version marker selecting that vocabulary.

The existing warning does not repair the design. The current spike explicitly leaves book
squats ordinary and emits only a warning
(`spike/crates/oracle/src/reserved.rs:261-295`); the package's name-only/shared-file semantics
remove that disambiguation. More importantly, the round itself rules that lints cannot rescue
a bad correctness boundary (`Research/plans/271-block-settle-rulings-ledger.md:310-315`). The
claim that an unmarked role function is an authored opt-in
(`Research/notes/24M-language-rulings-and-remaining-ledger.md:108-116`) is circular when the
only evidence of authorship is an ambiguous name.

**Gate consequence:** role authority needs an unambiguous, versioned admission condition.
Marker-gating recognition is the already-present candidate and preserves plain-sh gradual
enhancement. A fail-closed collision rule could also work, but a warning-and-continue rule
cannot authorize probe execution or elision. Resolve this before role names become a published
unversionable surface.

### 279b-fd3 — The “formal spine” lists evidence producers but does not define their composition

**Severity: HIGH. Confidence: HIGH.**

The package is right to put selector and whole-coordinate comparison behind opaque relational
chokepoints and to avoid baking pointwise comparison into callers
(`Research/notes/277-entity-algebra-design.md:91-109`). It then calls one page the formal
relation spec: a ternary codomain, a consumer table, and a registry of generators
(`Research/notes/277-entity-algebra-design.md:111-149`). Those are necessary pieces, but they
do not determine the comparison.

No rule says how evidence from resolver canonicalization, selector dialect, context-axis
invariance, carried-by rows, wrapper lends, and re-keying combines. In particular, the spec
does not define:

- how per-axis claims form one whole-context judgment after pointwise decomposition has been
  deliberately excluded from the API;
- how multiple lend/canonicalization paths compose;
- the precedence of `same`, `provably-disjoint`, and `unknown` evidence; or
- the general response to conflicting evidence beyond three named contradiction examples.

This is not an academic completeness request. Wrapper transport already requires a lend entry
composed with the kind's store derivation
(`Research/notes/273-wrapper-surface-redesign.md:198-203`). An implementation deciding whether
an alice-side fact speaks for a root-side site must combine user mapping, the relevant storage
axes, entity resolution, and selector relation. The current text tells each component what it
may say but never defines the evidence algebra that yields the single answer.

The rulings ledger accurately records that only the ternary/safety inversion was acknowledged;
the generator registry and one-page spec were left unacknowledged and “yolo'd” into this
review (`Research/plans/271-block-settle-rulings-ledger.md:752-767`). That is precisely the
part the entity-algebra rebuild would freeze into its most central API
(`Research/plans/270-round27-charter.md:95-103`).

**Gate consequence:** specify a small, deterministic evidence-composition algebra before the
re-key. It should cover multi-axis and multi-boundary examples, make conflict/refusal monotone,
and preserve the excellent consumer rule that `unknown` licenses neither transport nor
survival. Keeping the chokepoint opaque is good API design; leaving its semantics to the
implementer is not.

### 279b-fd4 — Positive observe marks are used as if they completely describe a value

**Severity: HIGH. Confidence: HIGH.**

The value design defines a channel's backing as the union of coordinates named by producing
reads' `:?` marks, then allows a value to fold when that set remains fresh and to transport
when that set is invariant (`Research/notes/275-value-predictions-and-the-capture-lane.md:61-79`,
`Research/notes/275-value-predictions-and-the-capture-lane.md:97-115`). The transport proof's first premise is stronger: “the value is a pure function of
the state its backing names” (`Research/notes/275-value-predictions-and-the-capture-lane.md:140-154`).
Nothing in the authored surface establishes that completeness.

An observe mark is deliberately positive and single-cell: it says this read reads X, and the
mark itself asserts exactly one thing
(`Research/notes/277-entity-algebra-design.md:252-266`). A producing arm may also read Y:

```sh
choice__predict() {
    choice status "$1"   :? sm.example.SharedState:"$1"
}
```

Suppose `choice status` truthfully reads the named shared state but also consults an unmarked
per-user preference. Its stdout remains world-spoken, and under the stated union rule its
backing contains only `SharedState`. Freshness can then survive a disturbance to the preference,
or the value can transport from alice to root because the named cell is invariant. Either can
fold the wrong question/value into control flow and under-execute without entering the
flag-gated survival corner. No authored line is false: “reads SharedState” is true; “reads only
SharedState” was never said.

The package acknowledges that a wrong backing is knife-tier and says absent marks cost only
value (`Research/notes/275-value-predictions-and-the-capture-lane.md:210-220`), but it does not
handle the partial-mark case. Nor can totality be inferred from the role name: the round's own
naming rule reserves `only` for complete-by-contract surveys, while roles without it are
arm-incremental (`Research/plans/271-block-settle-rulings-ledger.md:323-329`). “Honesty of the
mark” cannot strengthen “reads X” into “is a pure function only of X.”

**Gate consequence:** every contributing read without complete backing evidence must make the
derived backing unknown, or the language needs an explicit complete-support contract that can
express all inputs. Positive observe marks may safely widen an already-known backing; their
negative space cannot license freshness or cross-context transport. Resolve this before the
fragment recipe/backing-set representation is frozen.

### 279b-fd5 — Legal pipefail text violates the package's own executable off-ramp

**Severity: HIGH. Confidence: HIGH.**

The floor selects `posh 0.14.1` and `dash 0.5.12` specifically because both reject bare
`set -o pipefail` (`Research/notes/278-dorc-lang-v0-1-reference.md:16-36`). The same reference
then declares `set -o pipefail` legal dorc-lang, while presenting the self-gating spelling as
only the blessed idiom (`Research/notes/278-dorc-lang-v0-1-reference.md:67-81`). Strip neither
erases nor rewrites pipefail: the reference's exhaustive strip list contains bind/mark
erasure, `dorc:` erasure, and shebang rewriting, then defines strip-and-run-under-both as the
off-ramp test (`Research/notes/278-dorc-lang-v0-1-reference.md:175-191`).

Therefore a legal marked oracle can strip to bytes that fail the package's validity test. The
apply-time evaluator handshake does not help a consumer who has stripped Dorc away. Calling
pipefail “above the floor” names the contradiction but supplies neither a transformation nor a
restriction. This also contradicts the welded rationale that stripped oracles run on any
POSIX box regardless of the consumer's shell (`KNOBS.md:198-203`).

**Gate consequence:** choose one coherent rule: require/normalize the self-gating idiom in
marked oracle text; add a demonstrably semantics-preserving strip rewrite; or narrow the
off-ramp promise. Merely declaring no-pipefail Dorc executors unsupported protects the apply
lane, not the standalone artifact.

### 279b-fd6 — Two differential binaries cannot carry all the weight assigned to a language spec

**Severity: HIGH. Confidence: MEDIUM-HIGH.**

The package says no written base-language specification will exist “mid-spike or ever”; a valid
text is one that parses and runs identically under pinned posh and dash
(`Research/notes/276-language-sitting-kwhichsh-unsafe-churn.md:192-213`). Pins, real-binary
tests, and the rejected-construct rc fence make this an excellent calibration oracle
(`Research/notes/276-language-sitting-kwhichsh-unsafe-churn.md:215-237`). They do not make it
an implementable static-language definition.

“Runs identically” does not define which observables are compared, which environment and
inputs quantify the comparison, or how nontermination and external commands are treated. A
finite differential run cannot decide language membership for arbitrary text, and agreement
can preserve common bugs or undefined behavior. The new parser must still choose an accepted
grammar, parse tree, error recovery, and semantics for constructs it models. Without a small
normative definition, those choices become the real spec by accident—the exact surface the
root design calls the second-most-critical component and the highest-lock-in one
(`DESIGN.md:325-337`). Finding `279b-fd5` is one concrete result of the missing boundary, but
the problem remains after pipefail is repaired.

**Gate consequence:** retain the two-binary suite, but demote it from sole definition to
calibration. The rebuild needs a compact normative accepted grammar plus a definition of
observable equivalence and the Dorc-specific semantic delta. This need not restate POSIX or
be formal-methods-heavy; it only needs to adjudicate parser/analyzer disagreements without
making the implementation self-authorizing.

### 279b-fd7 — The declared unsafe hatch is not an unsafe hatch for books

**Severity: MEDIUM. Confidence: HIGH.**

The language sitting closes the long-owed Rust-`unsafe` equivalent by identifying bare `sh`
as an opaque, permanently unlicensed subgraph, including multi-line heredoc regions
(`Research/notes/276-language-sitting-kwhichsh-unsafe-churn.md:29-38`). The evaler note later
clarifies that the three-spelling table applies only in oracle bodies. In books, bare `sh -c`
is a normal site that rides the stdlib sh oracle and decomposes classically
(`Research/notes/274-evaler-surface-and-reentry-token.md:284-295`). The reference nevertheless
publishes bare sh as “the escape hatch” and rules out any second construct
(`Research/notes/278-dorc-lang-v0-1-reference.md:116-131`).

The oracle engineer has a body-line wall. The admin does not have the promised supported
speech act, “do not reason inside this region,” precisely because the stdlib recognition that
provides ordinary-book value removes it. Reaching for an obscure evaluator name is not an
equivalent contract and is vulnerable to future stdlib growth.

**Gate consequence:** either scope the unsafe obligation explicitly to oracle bodies and leave
the book need open, or provide a book-level idiomatic sh spelling whose descendants cannot
license transformations. The permanent “no second construct” ruling should not be baked while
the two users receive different semantics from the alleged one construct.

## What holds up

- `279b-ok1-ternary`: the split between `same`, `provably-disjoint`, and `unknown`, with
  distinct consumers and `unknown` safe for both, is a real repair to the wrapper safety
  inversion (`Research/notes/273-wrapper-surface-redesign.md:171-203`,
  `Research/notes/277-entity-algebra-design.md:116-149`). Finding `279b-fd3` asks for its
  composition semantics, not a different codomain.
- `279b-ok2-positive-authority`: explicit kind-owner invariance replacing inference from
  negative space is sound, and the separation carve correctly refuses to turn address
  inequality into referent inequality
  (`Research/notes/272-address-derived-topology.md:139-169`). The consolidated fences preserve
  keying as license-free and keep the survival flag from manufacturing separation
  (`Research/notes/277-entity-algebra-design.md:378-397`).
- `279b-ok3-values`: the value-prediction design correctly separates fragment provenance from
  per-channel backing sets and identifies question-staleness independently of answer-staleness
  (`Research/notes/275-value-predictions-and-the-capture-lane.md:61-79`,
  `Research/notes/275-value-predictions-and-the-capture-lane.md:117-138`). The
  binding-line hazard and post-probe rebind cost are recorded rather than hidden.
- `279b-ok4-DST`: the host-interaction inventory, single materialization event, deterministic
  run-id naming, simulator event, and fail-to-run lattice for shim availability are thoughtful
  and DST-shaped (`Research/notes/274-evaler-surface-and-reentry-token.md:172-182`). Finding
  `279b-fd1` is specifically about executable identity after later PATH transforms, not about
  those mechanisms.

## Suspicions checked and withdrawn

- `279b-wd1-site-keying`: `Research/notes/277-entity-algebra-design.md:430-434` says “probe
  keying” moves to the coordinate, initially appearing to violate site-keyed probe results.
  The ledger still routes probe-sourced values through the site-keyed record lane
  (`Research/plans/271-block-settle-rulings-ledger.md:829-834`). The phrase can mean
  coordination/fact keying rather than result storage; ambiguity alone is not a finding.
- `279b-wd2-capture-render`: eliding a capture assignment can unbind a variable, and a folded
  value needs a second value pass or fold-time substitution. The package explicitly records
  both hazards (`Research/notes/275-value-predictions-and-the-capture-lane.md:117-138`,
  `Research/plans/271-block-settle-rulings-ledger.md:822-834`) and reserves the rebuild seams.
  This is difficult deferred work, not a concealed design gap.
- `279b-wd3-sudo-portability`: the `sudo` predictor strawman underweights sudoers-dependent
  run-as and environment behavior. It also explicitly declines login-shell shapes, calls the
  body a strawman, asks for read-only query arms, and keeps it author-vouched
  (`Research/notes/273-wrapper-surface-redesign.md:60-106`). This may reduce stdlib coverage,
  but does not by itself invalidate the wrapper mechanism.
- `279b-wd4-survival`: selector-dialect separation is intentionally risky in the trusted
  survival corner. The package now requires a positive minting speech act, attributes the
  claim, and allows `provably-disjoint` to affect survival only behind the survival flag
  (`Research/notes/277-entity-algebra-design.md:120-149`,
  `Research/notes/277-entity-algebra-design.md:156-180`). No new path around those
  gates was found; re-arguing the accepted `kSURVIVAL-trusted` choice would duplicate a settled
  decision.
- `279b-wd5-reference-staleness`: `278` still says DRAFT/awaiting delta review and labels the
  grammar unsettled (`Research/notes/278-dorc-lang-v0-1-reference.md:1-12`,
  `Research/notes/278-dorc-lang-v0-1-reference.md:232-244`) even
  though `277` records the delta pass as settled
  (`Research/notes/277-entity-algebra-design.md:439-445`). This should be cleaned before using
  the reference as an implementer handoff, but it is editorial state drift rather than an
  additional design failure.
- `279b-wd6-trap`: a registration-only wall initially looked weaker than the standing rule
  that unmodeled control flow collapses to top (`spike/CLAUDE.md:277-286`), especially because
  handler bodies have implicit later edges. The package acknowledges those edges, defers their
  modeling, and requires a loud wall (`Research/notes/276-language-sitting-kwhichsh-unsafe-churn.md:253-269`).
  I could not demonstrate a distinct wrong-elision path that survives stand-in rc reproduction
  and is not merely the general timing sensitivity of any replacement. The v1 wording deserves
  a fixture, as the package already says; the evidence did not justify a design finding.
