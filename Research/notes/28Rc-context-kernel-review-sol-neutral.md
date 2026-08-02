## Findings

### 1. Definition-only keying cannot represent frame-dependent helper resolution

Severity: high  
Plan sections: §1, especially `syn-definition-factored-indices`; §8 stage-i

+SURE The proposed factoring makes each derived helper closure a property of one `DefinitionId`, computed once:

> “Every derived row … a helper closure — is keyed by the **DefinitionId** that produced it … Computed once, whole-unit”  
> — `Research/plans/28Q-context-kernel-unification.md`, lines 81–84

> “A query at site S = `live_definition(frame(S), name)` → read THAT definition's rows.”  
> — same, lines 85–89

+SURE This is insufficient under the shell-resolution model the plan is trying to follow. A shell function body that calls another function resolves that helper from the live function environment when the call executes; helper binding is not permanently lexical to the caller’s definition. The same role `DefinitionId` can therefore behave differently at two sites without itself being redefined:

```sh
tool__is_converged() { helper "$@"; }

helper() { check_a "$@"; }
tool x

helper() { check_b "$@"; }
tool y
```

+SURE The maintained prior plan recognizes this dependency explicitly:

> “pin-closure captures from the live environment, wherever sourced”  
> — `Research/plans/28M-committee-speech-and-the-custody-price.md`, lines 454–456

+SURE `28K` similarly defines pinned material as the selected definition plus “its closure,” because the executing judgment and its dependencies must travel together:

> “the pinned material is the analysis-resolved *definition's* bytes … plus its closure”  
> — `Research/plans/28K-oracle-loading-and-resolution.md`, lines 229–234

+SURE Consequently, selecting the root role definition by frame and then reading a root-`DefinitionId`-only closure can ship or analyze the wrong helper. This is the exact class of “swap whose judgment executes” failure that `28K` calls pope-sin tier (`28K`, lines 219–228). It can affect probe results and elision, not merely optimization quality.

~SUSPECT The correct key must include the invocation frame, or the helper closure must itself be resolved recursively from that frame and represented as a site-specific composite binding. That weakens §1’s “computed once” and “no index multiplication” claim, although memoization by `(DefinitionId, frame)` may contain the cost.

Confidence: high.

---

### 2. Entry-closure custody is not a well-defined identity when source closures overlap

Severity: high  
Plan sections: §2 `syn-closure-is-the-speaker`; §8 stage-ii

+SURE The plan defines an entry-closure independently for every CLI entry or book:

> “the **entry-closure** — the transitive closure of literal `.`-sourcing reachable from an entry file (a CLI-named positional, or the book)”  
> — `Research/plans/28Q-context-kernel-unification.md`, lines 132–138

+SURE It then proposes to replace singular custody with “closure-membership,” while saying consumers continue to perform simple comparisons:

> “custody becomes closure-membership; consumers still only compare. One type, zero consumer churn.”  
> — same, lines 140–144

+SURE Transitive source closures are sets that may overlap; they do not naturally partition files or definitions. For example, entries `a.oracle.sh` and `b.oracle.sh` can both source `shared.sh`. A definition in `shared.sh` then belongs to both entry-closures. A more complex diamond may put the same definition in several overlapping closures with neither closure containing the other.

+SURE The plan does discuss diamond loading, but only byte deduplication and differing-byte refusal:

> “Diamond loading: byte-identical files dedup … differing bytes refuse”  
> — same, lines 156–160

+SURE Deduplicating bytes does not decide which speaker owns the deduplicated definition. Nor does “the defining file within the closure” choose an entry-closure when that file is within several closures.

+SURE This ambiguity is load-bearing because closure identity controls:

- the committee fence;
- kind-owner single occupancy;
- helper custody and attribution;
- vocabulary-blessing propagation.

+SURE The prior authority describes the committee mechanism as applying only across sibling/cousin inclusion edges (`28M`, lines 530–545), but it does not supply a canonical identity for shared descendants. The proposal turns that directional include-graph relationship into an equality-like custody key without defining the quotient.

~SUSPECT A sound design needs to retain the inclusion DAG and ask a relational question—such as whether all load-bearing definitions share one consenting entry ancestor—rather than pretending every definition has one closure identity. Alternatively, custody could be keyed to a particular load occurrence or root-to-definition path, but that would affect attribution, deduplication, and the claimed “zero consumer churn.”

Confidence: high.

---

### 3. The availability domain is too small for conditional and repeated lifecycle events

Severity: high  
Plan sections: §3 `syn-availability-is-universal`; §8 stage-iii

+SURE The plan gives each context one of four positional statuses:

> “available · arrives-at(p) · departed-at(p) · never”  
> — `Research/plans/28Q-context-kernel-unification.md`, lines 190–195

+SURE It also describes truth as piecewise constant between events and treats begin/end as position-indexed mutations:

> “piecewise-constant truth between events”  
> — same, lines 45–49

> “`useradd alice` at line 6 makes the user-alice context arrives-at(6)”  
> — same, lines 236–240

+SURE Ordinary shell control flow does not provide one total lifecycle trajectory. A creator may occur on one branch but not another, may be under `&&`/`||`, may fail without terminating the book, or may execute repeatedly in a loop. At a CFG join, availability can be “available on some incoming paths and absent on others.” A loop can yield zero, one, or several begin/end transitions at the same syntactic plan positions.

+SURE None of `available`, `arrives-at(p)`, `departed-at(p)`, or `never` represents these may/must distinctions. An interval model over source positions alone also cannot distinguish loop iterations or path histories.

+SURE This omission matters to the claimed downstream theorem:

> “probing enters it if available”  
> — same, lines 215–217

> “Guards in arriving contexts are sound BY CONSTRUCTION”  
> — same, lines 243–245

~SUSPECT The guard fallback itself will often fail safely when entry is impossible, but that does not repair the abstract-domain gap. The analyzer still needs to decide whether to probe, whether a fact exists in the correct incarnation, and whether a later site may elide. A “may be available” state must not be consumed as “definitely available.”

+SURE Established dataflow practice would model availability as a lattice at CFG points—at minimum definite-available, definite-unavailable, and unknown/mixed—while lifecycle histories or incarnation sets handle loops. The plan mentions an existing lattice and universal meets elsewhere, but §3 neither defines the transfer/join operations nor acknowledges that its four statuses are insufficient.

~SUSPECT The plan’s linear formulation may work for the pivot strawman, but it does not yet support the stated universal claim over arbitrary analyzed shell.

Confidence: high.

---

### 4. Stage ii is not independently specified enough to be a build stage

Severity: medium  
Plan sections: §2, §8 stage-ii, §9

+SURE Stage ii includes closure-keyed fencing and blessing propagation:

> “the frame-relative, closure-keyed fence … kind-owner occupancy per closure; blessing-reachability for vocab-minting”  
> — `Research/plans/28Q-context-kernel-unification.md`, lines 381–386

+SURE Both governing choices remain open:

> “`pin-blessing-keying` — family-rooted vs closure-global”  
> — same, lines 403–405

> “The fence's permanence and the sparing-tier composite questions … unresolved”  
> — same, lines 417–419

+SURE The difference between family-rooted and closure-global blessing is semantic, not merely implementation detail. Under closure-global blessing, a helper reached by one family’s higher-grade `predict()` may gain vocabulary-minting authority when used from another family’s lower-grade verdict. Under family-rooted blessing, it may not. The plan itself says this “bites when one closure hosts families of divergent care” (lines 174–177).

+SURE The committee fence is likewise marked unratified in its authority:

> “DELIBERATELY UNRESOLVED … ‘try it that way for the spike, and see how much it sucks’”  
> — `Research/plans/28M-committee-speech-and-the-custody-price.md`, lines 251–261

~SUSPECT It is reasonable to implement a conservative experiment, but that is different from an independently green refactor stage in “THE refactor plan.” The plan should split structural custody infrastructure from policy consumers:

1. build closure/inclusion provenance without changing licenses;
2. settle blessing scope and fence semantics;
3. enable those consumers with explicit behavioral gates.

+SURE As written, stage ii either cannot finish until its human rulings occur, or it silently chooses provisional behavior whose later change can alter sparing, attribution, and oracle-author expectations.

Confidence: high.

---

### 5. Stage iii contains two semantic systems but provides a gate for only the easier one

Severity: medium  
Plan sections: §3, §8 stage-iii, §9–§10

+SURE Availability and incarnation continuity are distinct problems:

- availability asks whether a context can be entered at a position;
- continuity asks whether a fact from one lifetime may speak about another.

+SURE The plan acknowledges that the latter is unresolved:

> “cross-incarnation correlation/equivalence … NOT designed here”  
> — `Research/plans/28Q-context-kernel-unification.md`, lines 197–213

> “nothing here builds toward either pole”  
> — same, lines 411–416

+SURE It also reserves the complete oracle-authored lifecycle surface:

> “How users express these concepts in oracles … is the next design dig … Deliberately empty”  
> — same, lines 426–432

+SURE Yet stage iii includes “availability computation from begin/end descriptions,” fresh incarnation minting, lifecycle crossing, and a pivot-book payoff (lines 387–394). Those cannot be meaningfully integrated until the engine knows:

- what exact oracle utterance identifies the affected context;
- whether a description is at-least, at-most, or exact;
- how conditional declines affect event recognition;
- which party vouches that begin/end succeeded;
- how a failed or partial lifecycle command changes availability;
- whether crossing marks concern dimension change, lifetime change, or both.

~SUSPECT The plan correctly says §10 must settle first, so this is not an unnoticed contradiction. The problem is staging: stage iii groups the mechanically conservative availability work with the open continuity and authored-contract work, then offers only “no lifecycle events is byte-identical” plus one strawman render as its gate. That gate cannot expose incorrect joins, repeated lifetimes, partial failures, or wrong cross-incarnation carry.

~SUSPECT Stage iii should be divided into at least:

1. context-qualified facts and definite availability, with silence/unknown always walling;
2. lifecycle-event authoring and CFG transfer semantics;
3. incarnation crossing and, later, correlation/equivalence.

Confidence: medium-high.

---

### 6. “One model” is a useful implementation metaphor but not a semantic unification

Severity: low  
Plan sections: §0 `syn-one-context-two-planes`

+SURE The plan claims both planes obey one discipline:

> “piecewise-constant truth between events, region-scoped shadowing, and per-name/per-kind crossing claims”  
> — `Research/plans/28Q-context-kernel-unification.md`, lines 45–49

~SUSPECT This is structurally suggestive but semantically overstated. Load-plane facts are analyzer-owned consequences of shell semantics. World-plane facts combine probe measurements, author vouches, at-most claims, admin consent, staleness, and hostile-host boundaries. A function-definition event deterministically changes a modeled function environment; a lifecycle command merely attempts to change an external context and can fail, partially succeed, or be contradicted by concurrent state.

+SURE The project’s root correctness model requires those provenance and trust classes to remain visibly distinct:

> “two angles on ‘provenance’: … where something came from … and how much we trust it”  
> — `IMPLEMENTATION.md`, lines 106–118

> “Measured … Vouched … Claimed … Derived … Consented”  
> — `USER_STORY.md`, lines 765–774

+SURE The detailed plan generally preserves that distinction, particularly its integrity/analysis split and controller-minted scope. Therefore this is presently a framing defect, not necessarily an implementation defect.

~SUSPECT Calling it a shared event-indexed infrastructure would be more accurate than claiming one semantic model. That wording would reduce the risk that future implementers reuse load-plane certainty or join rules on the world plane.

Confidence: medium.

## Overall assessment

~SUSPECT The three-pillar direction fits Dorc’s goals well: positional shell-native resolution serves the admin’s existing mental model; source-based factoring improves engineer packaging without introducing manifests; and explicit context availability is necessary for local execution, wrappers, and lifecycle-heavy ops books. P1 and P2 also align with established environment and provenance practice in broad outline, while P3 correctly defaults unknown context transitions toward guard/run.

+SURE The plan is not yet sound enough to execute as one foundational refactor. The first two high-severity issues are representation errors: helper behavior cannot be keyed only by its root definition, and overlapping source closures cannot be treated as a single comparable custody identity without further definition. P3 additionally needs a real CFG availability lattice rather than four linear statuses.

~SUSPECT The safest disposition is “direction accepted, kernel representation not ratified.” Stage 0 appears separable; stage i should stop at frame-aware root resolution until frame-aware helper closure is designed; stage ii should separate inclusion provenance from unsettled custody policy; and stage iii should be decomposed around definite availability, lifecycle authoring, and incarnation correlation.