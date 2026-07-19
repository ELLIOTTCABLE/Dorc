# 279a — Review working notes (solution N)

Reviewer-authored scratch for the adversarial review of `plans/270`, `plans/271`,
and `notes/272`–`278`. This is evidence capture, not a ruling or design durable.
Root documents and typed human rulings remain authoritative. Findings here may be
withdrawn before the final report in `notes/279b`.

## Pass 1 — authority and package map

- Root priorities: failures in apply must degrade toward run; plans remain whole-book,
  ordered, and attention-honest; probe mutation is an absolute contract; replacement
  must preserve consumed observables; nondeterminism stays behind DI seams.
- Welds most relevant here: `kFAIL`, `kHALVES`, `kWHICHSH`, `kLANG`, and the
  `inv-one-observable` / `inv-probe-sourced-values` / `inv-site-keyed-results` laws.
- Round-27 moves: flat `(kind, entity, selector)+context` coordinates; one ternary
  comparison chokepoint; selector dialects keyed by family; wrapper/eval'er detection
  inside `cmd__predict()`; `cmd__lend_map()`; `dorc:sh`; value-predictions with derived
  provenance and backing sets; a posh∩dash executable language floor.

## Candidate findings after first trace

### 279a-cand-language-floor — binary differential is not a membership specification

`276:192-213` and `278:18-36` place spec-level weight on “runs identically under
posh and dash,” while refusing a written spec. For arbitrary shell text, equivalence
over inputs/world states is not decidable or safely executable by the analyzer. A
finite differential battery is strong calibration, but cannot define which constructs
the parser may accept. `277:241-244` additionally leaves exact quoting simplification
to parser-build time. Candidate consequence: implementation-defined dialect membership
at the highest-lock-in boundary. Check whether Debian Policy or a prior plan supplies a
normative syntactic subset that cures this.

### 279a-cand-pipefail-floor — legal full-dialect text fails the stated strip floor

`276:154-169` / `278:67-81` say bare `set -o pipefail` is legal dorc-lang while
posh 0.14.1 and dash 0.5.12 intentionally lack it; `276:180-187` only *blesses* the
self-gating idiom. But `278:23-25,173-191` says a stripped valid file runs identically
under both binaries, and strip does not rewrite ordinary `set`. Either the guarded
idiom is mandatory, bare pipefail is not legal, or the executable-floor/off-ramp claim
is false. Likely final finding; test exact wording across `276`/`278`.

### 279a-cand-comparison-algebra — relation signature lacks composition and set lifting

`277:111-149` calls a ternary function plus generator table the “formal spine,” but
does not define how simultaneous generator answers combine, whether `same` is closed
transitively, or what happens when both `same` and `provably-disjoint` are derivable.
`277:357-361` and `275:61-79` upgrade backings to coordinate sets without specifying
the all/any lifting for transport and survival. This is not a code detail: a wrong
quantifier licenses under-execution. Search the package and older algebra plans for an
existing meet/closure law before keeping.

### 279a-cand-capture-program — the read-value slice dropped required architecture

`270:156-162` says the `$(hostname)` slice ships a probe and folds stdout back as a
literal. `271:822-835` / `277:362-369` reserve only post-probe rebind and literal
provenance. The source design `219:154-183,197-206,210-231` required, in addition:
promotion of expansion-internal commands to leaves, a new capture disposition,
post-probe analysis, and rendering the binding as a quoted literal; it called this a
3–4 wave program. `275:136-138,223-225` still admits the binding-site hazard, and
artifact-entering substitution is postponed (`275:97-115`). Current code retains
`expansion_internal` exclusion and has no `Capture` site kind. Candidate high finding:
the charter presents a buildable slice while omitting the mechanism that preserves the
binding and downstream value semantics.

### 279a-cand-capture-wire — coordinate framing does not automatically carry stdout

`270:156-162` claims the imported wire's coordinate-last-to-token shape upgrades
capture to a single-line value. The imported contract keeps the inner site grammar
unchanged (`262:112-119`) and grants last-to-token specifically to `coord=` / future
free-content fields (`262:137-142`); current `stdout=` remains whitespace-tokenized in
`spike/crates/cli/src/main.rs:2969-3004,3085-3100`. Round 27 never specifies a stdout
record or ordering rule. Values containing spaces therefore are not covered by the
claimed “single-line” floor. Likely combine with capture-program.

### 279a-cand-wrapper-vouch — arbitrary guest execution sits inside a self-vouched body

`273:60-106` makes wrapper `predict()` terminate in `env ... "$@"`; the body is both
read-only model and shipped stand-in. Shipping it with site argv executes the original
guest, which can mutate. Making it safe requires replacing/materializing the guest's
predict while preserving the wrapper's authored bytes. `273:217-244` acknowledges
probe-form composition is only drafted and task-14-gated; `271:84-92` defers that gate.
Candidate finding should sharpen beyond “known open item”: the strip-only/oracle-bytes
law, env-execs-only-files constraint, and arbitrary-guest substitution may be mutually
inconsistent. Trace `24S` and the hard structural-vouch law before ranking.

## Suspicions already weakened

- A single `context` slot is not necessarily a scalar dimension; it can hold a product
  context record. Do not report absent a representation that actually forces scalarity.
- Probe-execution parity was a hard prerequisite in `219`, but it has since landed in
  `spike/e2e/run.sh` gate 1. Do not claim the old prerequisite is still absent.
- The accepted probe→apply TOCTOU exposure is an explicit standing WONTFIX; do not
  relitigate it through value-predictions. The binding-site and multi-wave mechanics
  remain independently reviewable.

## Pass 2 — adversarial trace results

### 279a-cand-backing-completeness — KEEP, rebuild-blocking

The new value proof silently changes what “backing” means. `275:68-75` derives a
value backing from `:?` lines and insists that each mark says exactly “this read reads
X.” Yet its cross-context proof begins with “the value is a pure function of the state
its backing names” (`275:140-154`). The previous ruling is explicit that a backing is
only a declaration-scope and “carries NO completeness burden”
(`24D:156-169`; the live type repeats this at
`spike/crates/plan/src/survival.rs:398-407`). Positive read disclosure cannot justify
the closed-world dependency premise. A value can depend on an omitted store, ambient
environment, time, or another hidden input while every declared backing transports;
the resulting value can then kill an analysis arm in the wrong context
(`275:97-115`). The package promises no new authored speech-act (`275:200-208`) and
hard-defers never-settled inputs (`275:91-95`), so neither completeness nor hermeticity
is actually supplied. This is not the accepted probe/apply TOCTOU axis.

### 279a-cand-selector-vouch — KEEP, rebuild-blocking

The selector dialect manufactures separation claims out of unrelated positive marks.
A family is name-derived and may contain functions from different authors
(`271:267-274`). A verdict or observe mark mints a selector; the set of all such marks
in the family becomes its dialect; and unequal tokens in that dialect are then treated
as provably disjoint (`277:158-177`). Thus adding `:? K:e#active` can change an existing
`K:e#enabled` comparison from collide to spare. But `:?` is simultaneously specified
as a read-disclosure (`277:262-266`) and a marked line is said to assert exactly one
thing (`277:250-255`; `275:73-75`). Neither author wrote “active and enabled never
overlap.” The package labels the derived answer vouch-tier (`277:128-139`) while
admitting physically overlapping cells are inherent (`277:178-181`). Attribution to
one minting line cannot repair the missing pairwise speech-act. Under the survival
flag, a false spare is silent under-execution.

### 279a-cand-capture-program / capture-wire — KEEP as one finding

The source design explicitly calls capture a 3–4-wave keystone and requires capture
leaf promotion, a new site/disposition, a value-plane←probe back-edge, provenance,
wire carriage, and assignment replacement (`219:154-183,187-206,210-221`). Round 27
schedules a working first slice (`270:156-162`) but its rebuild rider reserves only
pipeline order, provenance, and per-channel backing (`271:822-835`). The current
design then postpones artifact-entering substitution and merely flags that eliding the
assignment unbinds the variable (`275:97-115,136-138`). This leaves no semantics that
both uses probe bytes for downstream analysis and faithfully binds the apply-time
variable: rerunning the original capture can disagree with the planned arm; eliding it
loses the binding.

The wire shortcut is also false as written. The imported record contract leaves the
inner site grammar unchanged and extends last-to-token only to `coord=` and future
free-content fields (`262:112-142`). The incumbent `stdout=` parser splits on
whitespace (`spike/crates/cli/src/main.rs:2968-3012,3084-3100`). The claimed
single-LINE floor therefore silently implements at most single-TOKEN until a new
stdout field and producer contract are designed.

### 279a-cand-language-floor / pipefail-floor — KEEP as one finding

The posh∩dash differential is valuable calibration, not a parser-membership spec.
“Runs identically” has no stated quantifier over inputs, environment, external
commands, or observable channels; safely executing arbitrary candidate shell text is
not a static acceptance procedure (`276:192-213`; `278:18-36`). Debian Policy gives a
human portability target but does not define the Dorc parser's full accepted syntax,
and exact quoted-coordinate rules remain deferred to parser-build time
(`277:241-244`).

The layered “base floor / full dialect” distinction does not cure the off-ramp claim.
Bare `set -o pipefail` is legal full dialect although both pinned floor binaries reject
it; only the guarded spelling is recommended, not required (`276:154-187`;
`278:67-81`). Strip does not rewrite it. Direct `dorc-sh` is likewise legal and
deliberately left dangling after strip (`278:116-131,173-191`). This conflicts with
the root promise that Dorc artifacts remain immediately executable shell
(`DESIGN.md:75-79`) and the maintained story's “every artifact” off-ramp
(`USER_STORY.md:994-995`). The design needs separate, mechanically checkable contracts
for base syntax, executor-only extensions, and post-strip portability.

### 279a-cand-comparison-algebra — KEEP, medium unless subsumed in repair

`277:111-149` specifies the ternary codomain and generator registry, but not the
reduction when generators disagree, `same` closure, or set lifting. That omission
becomes load-bearing because backing changes from one coordinate
(`spike/crates/plan/src/survival.rs:398-425`) to a set (`277:357-361`). Survival needs
all footprint×backing pairs proved disjoint; transport needs every backing dependency
transported. The wrong existential is under-execution. The selector-vouch finding is
one concrete bad composition; even after repairing it, the general reduction and
quantifiers should be part of the promised formal spine before the rebuild bakes an
API around it.

## Pass 2 — withdrawals / no-findings

- **Wrapper guest execution:** the danger is real, but the package does not claim the
  unsafe composition is ready. Probe-form composition remains task-14-gated and the
  `env`-cannot-exec-functions landmine is explicitly routed to implementation planning
  (`271:850-855,871-878`; `273:217-244`). Treating that as a present breach would
  punish an honest fence. Keep it as a checked-and-withdrawn suspicion.
- **Context-slot scalarity:** whole-coordinate relational comparison leaves the slot
  free to carry a product context and explicitly refuses pointwise API lock-in
  (`277:91-109`). No finding.
- **Never-derived separation:** the package correctly catches the safety inversion and
  refuses to infer disjointness merely from differently keyed contexts
  (`272:154-173`). This part holds up.
- **Unknown relation result:** the ternary `unknown` result is the correct shared safe
  bottom for both transport and survival (`277:118-126`). The defect is how positive
  generator answers are minted/composed, not the trichotomy.
- **Probe-exec prerequisite and probe/apply TOCTOU:** probe-exec parity has landed in
  the spike; the remaining probe→apply race is a standing accepted trade. Neither is
  a novel round-27 finding.
