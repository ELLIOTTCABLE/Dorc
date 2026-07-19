# Round-27 design-package review worklog (sol-n)

Review point: `9431ccb` (`worktree-agent-af67e0c672b0f437e`). This is scratch evidence for
the independent review whose final report is `279b-review-report-sol-n.md`; it is not a
design ruling. The reviewed package (`plans/270`, `plans/271`, notes `272`–`278`) remains
untouched.

## 279a-frame — review protocol and controlling tests

Authority read before the package: root `README.md`, `DESIGN.md`, `IMPLEMENTATION.md`,
`USER_STORY.md`, `KNOBS.md`, `TODO-ADDTL.md`, `Research/README.md`, and
`spike/CLAUDE.md`. `AGENTS.md` was supplied in the review prompt for terminology only.

The review will test the package primarily against these design obligations:

- `279a-test-kFAIL`: probe construction fails toward withhold; apply planning fails toward
  perform. A claim or representation that can silently cross that phase boundary is severe.
- `279a-test-observables`: replacement is observable-preserving, not a synonym for not
  executing; command status/stdout/stderr/effect consumption and repeated reads matter.
- `279a-test-authorship`: authored knowledge remains sh-spelled, attributable, and
  strip-fidelity-preserving; engine inference cannot become an unnamed second oracle.
- `279a-test-two-users`: evaluate each choice separately for the admin writing a scrappy
  book and the engineer publishing an oracle. Convenience for one is not evidence for the
  other.
- `279a-test-locality`: failures should remain attributable and local. In particular,
  survival claims may endanger another author's line only inside the explicitly fenced
  `kSURVIVAL-trusted` corner.
- `279a-test-honesty`: the plan retains original order, never hides executing lines, and
  saves attention only through licensed elision (`rul-attention-honesty`).
- `279a-test-hermeticity`: analyzer-kernel inputs determine outputs; clock, disk, network,
  and randomness remain behind injected seams. Probe-derived values and future re-bind
  machinery must not smuggle ambient state into the kernel.
- `279a-test-lockin`: distinguish a deliberately deferred feature from an omitted seam whose
  later retrofit would re-key the fact/value/provenance domain or invalidate published
  oracle spellings.
- `279a-test-exclusion`: recheck candidate conclusions in reverse propagation, probe/apply,
  admin/engineer, and reliable/unreliable-oracle cells before retaining them.

The package inventory is `270` (round charter), `271` (rulings ledger), `272` (address and
derived topology), `273` (wrapper surface), `274` (evaler and re-entry token), `275` (value
predictions/capture), `276` (dialect, unsafe, churn), `277` (entity algebra), and `278`
(`dorc-lang/v0.1` reference). Findings will be ranked by design consequence, not by the
throwaway spike's code quality. Settled market/corpus-go/no-go questions are out of scope.

## 279a-live — candidate ledger

### 279a-cand-role-capture — unmarked names can silently mint authority

`278:90-96` makes the version marker gate syntax but explicitly not role recognition;
`278:133-151` makes family membership name-only, makes names permanent/unversionable, and
allows the engine vocabulary to grow by adding names. The inherited ruling (`24M:51-61`)
accepts coincidental capture because a captured function is said to fail loudly. That last
premise is false for the safety-bearing case: an ordinary function that happens to return 0,
or that invokes a foreign command, can be valid sh and succeed quietly. Once interpreted as
`__is_converged` or `__predict`, the same bytes become author-vouched probe code and can mint
guard/elision authority. Adding a future role name can therefore retroactively reinterpret an
unmarked, previously ordinary file despite the version marker. The reingest-collision floor
does not cover first capture, and warning is not withholding.

Strongest disconfirmation: the role namespace is intentionally rare and authoring a
role-shaped function is treated as the opt-in (`24M:91-118`). That supports low collision
frequency, not the claimed attribution or fail direction. Retain at highest severity because
the package makes the ambiguity permanent just before the shared-file rebuild.

### 279a-cand-path-shim — PATH composition can execute the wrong reentry object

`274:63-69` defines `dorc-sh` as a PATH-prepended runtime object and makes ordinary PATH
resolution the transitive-composition mechanism. `274:172-182` validates the constructed shim
once, but `274:237-240` assumes an authored PATH scrub necessarily yields 127. It need not: a
scrubbed or reordered PATH may contain a different executable named `dorc-sh`. The introduced
name then runs foreign bytes in a probe body with full reentry trust. This is precisely the
injection shape the older threat model avoids by invoke-by-content-hash rather than PATH
(`plans/102:110-116`, seam reserved at `172-174`). An honest wrapper can set PATH as part of
its modeled semantics, so this is not reducible to an intentionally malicious oracle.

Strongest disconfirmation: materialization, smoke testing, and shimless degradation are
well-designed for absence/noexec. They establish that Dorc's shim works at the session PATH;
they do not establish that every authored PATH transform resolves that object. Retain at
highest severity across probe (possible mutation) and apply (wrong evaluator/semantics).

### 279a-cand-relation-algebra — the alleged formal spine lacks composition rules

`277:94-109` correctly reserves opaque, relational chokepoints and refuses to freeze a
pointwise API. Its formal spine then supplies a ternary codomain, consumer table, and generator
registry (`277:111-149`), but no rule for composing generators/dimensions, resolving multiple
paths, or handling conflicting `same`/`provably-disjoint` evidence other than a few named
contradiction examples. That omission is load-bearing when lend maps, axis invariance,
carried-by rows, selector dialects, and resolver canonicalization all contribute to one
comparison. The text explicitly declines a pointwise model without specifying its relational
replacement. Implementers must invent the evidence order inside the one chokepoint the rebuild
will make foundational.

Strongest disconfirmation: `{same, disjoint, unknown}` and unknown-as-safe-for-both are sound,
and the generator registry is an excellent inventory. Retain as high severity because those
pieces do not determine a result for composed cases, while the note calls the page a spec.

### 279a-cand-executable-floor — two interpreters are an oracle, not a sufficient spec

`276:192-240` says there will never be a written spec for the base dialect: validity means a
stripped file “parses and runs identically” under pinned posh and dash. The phrase does not
define the observations or environments over which runs are identical, and concrete
differential execution cannot define membership or semantics for all texts to a static parser.
The package itself needs above-floor semantics (`276:215-217`) and defers exact quoting until
the parser-facing grammar work. Differential tests are a strong calibration suite; treating
them as the entire normative definition means common acceptance/bugs become language and the
new parser becomes the real, undocumented spec at the highest-lock-in rebuild boundary.

Strongest disconfirmation: version pins, the rejection-rc fence, and the real-binary battery
make the proposal substantially more reproducible than “POSIX-ish.” Retain as high severity;
the gap is not a demand for a full POSIX restatement, only a small normative accepted grammar,
observable equivalence, and Dorc-specific semantic delta.

### 279a-cand-pipefail-offramp — the legal dialect is wider than its stripped floor

The package simultaneously says bare `set -o pipefail` is legal dorc-lang
(`278:67-81`) and chooses `posh 0.14.1` plus `dash 0.5.12` specifically because both reject
it (`278:27-36`). Strip has no pipefail rewrite; the self-gating spelling is blessed, not
required. Consequently a legal marked oracle can strip to bytes that fail the package's own
two-binary validity sentence (`278:16-25`) and fail on the POSIX boxes the welded
`kWHICHSH` rationale promises to serve (`KNOBS:198-203`). Calling pipefail “above the floor”
does not supply an off-ramp transformation or restriction; unlike binds/marks, the above-floor
construct survives stripping.

Strongest disconfirmation: the apply executor handshake fails toward run and non-pipefail
executors are explicitly unsupported. That protects Dorc's apply lane, but it does not repair
the stripped-oracle portability promise. Retain as high severity and high confidence; require
the gated idiom in marked/oracle text, teach strip a semantics-preserving rewrite, or narrow
the off-ramp claim.

### 279a-cand-observe-totality — positive dependency marks are consumed as complete support

`275:61-79` defines value backing as the union of producing reads' observed coordinates;
`275:97-115` lets freshness and transport branch on that set; `275:140-154` assumes the value
is a pure function of the state its backing names. But `277:252-266` makes `:?` positive and
single-cell: a mark says a read reads X, not that X is its complete support. A producing recipe
with one marked X read plus an unmarked Y read can therefore acquire backing `{X}` and transport
or fold past a Y disturbance. The package's absent-mark floor (`275:210-220`) does not cover
partial disclosure, and the role-name convention reserves `only` for complete surveys
(`271:323-329`).

Strongest disconfirmation: the package may intend the containing `predict` vouch to make every
observe set exhaustive. That intent is neither stated by the mark semantics nor expressible for
a multi-input read under the single-cell mark. Retain as high severity: unmarked contributing
reads must force unknown backing, or a real complete-support speech act is needed.

### 279a-cand-unsafe-scope — the selected hatch disappears in books

`276:29-38` closes the unsafe-hatch obligation by declaring bare `sh` to be an opaque,
unlicensed subgraph. But `274:287-295` later clarifies those semantics are oracle-body-only:
in books, bare `sh -c` rides the stdlib sh oracle and decomposes normally. `278:116-131`
nevertheless publishes bare sh as “the escape hatch” and says no second construct will exist.
The general admin-side need that motivated the hatch—an explicit region where Dorc gives up
CFG totality and cannot optimize descendants—therefore remains unmet, while the reference
claims it is discharged.

Strongest disconfirmation: oracle engineers genuinely do get the body-line wall, and an admin
might evade recognition with another evaluator. Neither gives a stable, supported book-level
speech act; retain as high/medium severity because later retrofitting a hatch conflicts with
the permanent “no second construct” ruling.

### 279a-cand-trap — a state wall is not a control-flow model

`276:253-269` permits `trap` by treating registration as a loud opaque wall and defers the
handler's implicit may-run edges. The standing architecture requires unmodeled control flow to
collapse to top and reject (`spike/CLAUDE.md:277-286`). A trap changes future control flow;
invalidating facts at registration does not represent a handler that may run between later
sites or at exit, nor its status/effect observables. Retain provisionally at medium severity:
the safe v1 choices are whole-affected-region rejection or actual implicit edges, not an
ordinary-command wall.

Strongest disconfirmation: an opaque wall prevents facts from simply flowing across the
registration and many EXIT-trap interactions remain behaviorally harmless. This lowers
confidence/severity, but does not close asynchronous handler execution or the explicit
standing top-reject conflict.

## 279a-withdrawn — checked suspicions

- `279a-withdraw-site-keying`: `277:430-434` says “probe keying” moves to the coordinate,
  which initially appeared to breach `inv-site-keyed-results`. The round still preserves a
  site-keyed record lane, and the phrase can reasonably mean fact/probe coordination rather
  than result storage. Ambiguous brief wording is not enough for a finding.
- `279a-withdraw-capture-render`: `275` openly records that eliding a capture assignment can
  unbind a variable and sequences the read-value slice before render work. This is a real hard
  problem, but the package preserves and routes it rather than pretending it is solved.
- `279a-withdraw-sudo-portability`: the `sudo` examples understate policy-dependent run-as and
  environment behavior, but they are explicitly strawmen under oracle-author vouch with
  can't-say degradation. This is likely a stdlib coverage/cost problem, not a defect in the
  package's wrapper mechanism.
- `279a-withdraw-survival-unsoundness`: selector separation remains intentionally unsound in
  the trusted-survival corner. The package makes the speech act explicit, attributes it, and
  flag-gates its consumption. Re-arguing the accepted `kSURVIVAL-trusted` risk would duplicate
  a settled decision; no new bypass was found.
