# 228 — research: suppression / root-cause dedup, and fleet-scale error fingerprinting

> Deep-research round, 2026-06-11. Fronts **rq-E** (suppression / root-cause dedup
> *engineering*) and **rq-G** (fleet-scale error grouping/fingerprinting). Serves
> round-22's two live problems: (rq-E) the ⊤-cascade — one command going ⊤ ("unknown/
> unmodeled, poisons downstream") makes every downstream consumer emit its own Note, so
> one unmodeled command sprays N notes; round-22 gives ⊤ a cause-pointer so diagnostics
> can be deduplicated to the root, and this note finds the *machinery* + prior art.
> (rq-G) one wrong oracle-declaration fails identically across M hosts × N scripts; the
> north-star is "one rot event reads as ONE cause, fleet-aggregable" — the
> error-grouping/fingerprinting problem o11y vendors have a decade of practice in.
> Extends `plans/111` (round-11 error/provenance) and notes 220/222; does NOT re-cover
> the Clang note-explosion *story* (10→170, the GSoC heuristic overview), ninja/Bazel
> explain-flood, SQL-Server plan-warning fatigue, or Lee&See trust psych — those are
> held elsewhere in the corpus. New ground: Clang's suppression/dedup AS CODE, rustc
> `ErrorGuaranteed` cascade-*suppression* use, abstract-interp alarm clustering, and the
> o11y fingerprinting/bucketing corpus (Sentry, WER, Socorro). Findings slugged
> `finding-N`; sources `[grade-slug-year]`, graded list at end. Confidence marks per
> project convention (+SURE / ~SUSPECT / -GUESS / --WONDER).

## §0 Conclusions up front

*(filled incrementally as evidence lands; see per-question sections below.)*

**finding-clang-three-layers.** +SURE Clang's static analyzer separates suppression
into THREE independent layers that map cleanly onto distinct Dorc needs, and conflating
them is the trap: (1) *user-directed range suppression* (`[[clang::suppress]]` →
`SourceRange`, suppressed iff `fullyContains(range, bug)`) — the ADMIN "shut up about
this region" knob, NOT cause-dedup [B-llvm-bugsuppression-2025]; (2) *whole-report
heuristic suppression* — a hand-curated ~denylist of known-imprecision constructs
(`std::`, `list`/`basic_string`/`shared_ptr`, `sys/queue.h` macros) that calls
`BR.markInvalid()` to kill the entire report [A-llvm-bugreportervisitors-2025]; (3)
*path-piece dedup + interestingness-pruning* — the post-analysis re-walk that is the
real analog of Dorc's "⊤ carries a cause-pointer; only the cause-site note is
interesting" [A-llvm-bugreporter-2025]. Dorc's ⊤-cascade is layer (3); the cause-pointer
is Dorc's version of *interestingness propagated backward from the sink*.

**finding-clang-dedup-is-coarse-fingerprint.** +SURE Clang collapses the
combinatorial explosion of exploded-graph paths into user-visible reports with a
deliberately COARSE fingerprint: `FoldingSetNodeID Profile()` over `(BugType*,
short-description string, uniqueing-location, source-ranges)` — NOT the path, NOT the
node. N reports with the same Profile join one `BugReportEquivClass`; exactly ONE
representative is rendered [A-llvm-bugreporter-2025]. This is the same move rq-G needs
("M identical failures → one rendered cause"): *the grouping key omits the volatile
path and keys on the stable site identity*.

## §1 (rq-E) Clang Static Analyzer suppression & dedup AS CODE

Three first-party sources, all `llvm/llvm-project` main branch, read from local copies
the predecessor downloaded (re-cited to canonical raw URLs). The analyzer builds a bug
report at a *sink* node in the exploded graph, then walks the graph backward to the
root constructing a `PathDiagnostic` (a list of `PathDiagnosticPiece`s — events,
control-flow edges, call/macro subpaths), then prunes. The machinery is in three files.

### Layer 1 — user-directed range suppression (`BugSuppression.cpp`)

`[B-llvm-bugsuppression-2025]` (the `[[clang::suppress]]` attribute path). A
suppression attribute on a decl/stmt yields a `SourceRange`; a bug is suppressed iff
its location is lexically contained:

> ```cpp
> bool BugSuppression::isSuppressed(const PathDiagnosticLocation &Location,
>                                   const Decl *DeclWithIssue, ...) {
>   ... return llvm::any_of(SuppressionRanges, [BugRange, &SM](SourceRange Suppression) {
>     return fullyContains(Suppression, BugRange, SM); });
> ```

Lexical-parent walk: an attribute on a class covers its inline methods. Carries a FIXME
("Introduce stable IDs for checkers") — Clang itself wants per-checker suppression
granularity and lacks it; suppression is all-or-nothing per range. **Dorc read:** this
is the admin's region-mute, the analog of an oracle/book line saying "I vouch for this
command; don't diagnose it" — *not* the cascade-dedup mechanism. Keep it conceptually
separate (the round-21 vouching work is the right home), exactly as Clang does.

### Layer 2 — whole-report heuristic suppression (`LikelyFalsePositiveSuppressionBRVisitor`)

`[A-llvm-bugreportervisitors-2025]`, `finalizeVisitor`. A hardcoded denylist of
known-imprecision sites; if the bug's stack-frame decl matches, `BR.markInvalid()`
kills the ENTIRE report:

> ```cpp
> void LikelyFalsePositiveSuppressionBRVisitor::finalizeVisitor(
>     BugReporterContext &BRC, const ExplodedNode *N, PathSensitiveBugReport &BR) {
>   ... if (AnalysisDeclContext::isInStdNamespace(D)) {
>         if (Options.ShouldSuppressFromCXXStandardLibrary) {
>           BR.markInvalid(getTag(), nullptr); return; }
>   ...
>   if (CD->getName() == "list")  { BR.markInvalid(getTag(), nullptr); return; }
>   ... "basic_string" ... "shared_ptr" ... "__independent_bits_engine" ...
>   while (Loc.isMacroID()) { Loc = Loc.getSpellingLoc();
>     if (SM.getFilename(Loc).ends_with("sys/queue.h")) {
>       BR.markInvalid(getTag(), nullptr); return; } }
> ```

This is the "minimum shippable suppression set" pattern: don't *model* the imprecision,
just enumerate the few constructs the analyzer is empirically wrong about and silence
reports whose path bottoms out there. Tiny (~100 lines), hand-curated from real
false-positive bug reports. **Dorc read:** the analog of a curated "these sh constructs
reliably go ⊤ for boring reasons; don't even surface the ⊤" denylist — a stopgap that
is cheap and effective but is *not* principled cause-dedup. A candidate for Dorc's
shippable v0 only where a construct is known-uninteresting.

### Layer 3 — path-piece dedup + interestingness pruning (`BugReporter.cpp`)

`[A-llvm-bugreporter-2025]`. THIS is the direct analog of Dorc's ⊤-cascade dedup. The
pruning pipeline (from `generatePathDiagnostics`, in execution order, lines 2082-2118):

> ```cpp
> if (R->shouldPrunePath() && Opts.ShouldPrunePaths) {
>   bool stillHasNotes = removeUnneededCalls(Construct, ...pieces, R);
>   assert(stillHasNotes); }            // 1. interestingness-gated subpath pruning
> ...removePopUpNotes(...);
> adjustCallLocations(...); removePiecesWithInvalidLocations(...);
> ...optimizeEdges(...) ... dropFunctionEntryEdge(...);   // 2. edge aesthetics
> removeRedundantMsgs(Construct.getMutablePieces());      // 3. two-visitor dedup
> removeEdgesToDefaultInitializers(...);
> ```

**(3a) two-engines-one-fact → one note** (`removeRedundantMsgs` +
`eventsDescribeSameCondition`, lines 378-450). When `ConditionBRVisitor` and
`TrackConstraintBRVisitor` both emit an event at the *identical* `getLocation()`, keep
one by a fixed tag-preference order (prefer `ConditionBRVisitor` unless its message is
the generic fallback):

> ```cpp
> static PathDiagnosticEventPiece *
> eventsDescribeSameCondition(PathDiagnosticEventPiece *X, PathDiagnosticEventPiece *Y) {
>   const void *tagPreferred = ConditionBRVisitor::getTag();
>   const void *tagLesser    = TrackConstraintBRVisitor::getTag();
>   if (X->getLocation() != Y->getLocation()) return nullptr;
>   if (X->getTag() == tagPreferred && Y->getTag() == tagLesser)
>     return ConditionBRVisitor::isPieceMessageGeneric(X) ? Y : X;
>   ...
> ```

This is *precisely* Dorc's per-⊤-operand multiplication shape: N consumers each want to
say something about the same ⊤ fact at the same site; the resolver keeps one by a
priority rule keyed on *who is speaking* (the visitor tag) and *whether its message is
generic*. The generic-fallback exception is notable: a more-specific speaker wins, but
a speaker reduced to its generic message yields to the other.

**(3b) recursive interestingness-gated pruning** (`removeUnneededCalls`, lines 452-505).
A call/macro subpath survives only if it `containsSomethingInteresting`, computed
recursively; an event is kept wholesale only if `!event.isPrunable()` (events default
`isPrunable()==true`, i.e. droppable) OR it lies on an interesting stack frame:

> ```cpp
> static bool removeUnneededCalls(const PathDiagnosticConstruct &C, PathPieces &pieces,
>                                 const PathSensitiveBugReport *R, bool IsInteresting) {
>   bool containsSomethingInteresting = IsInteresting;
>   ... case Call: if (!removeUnneededCalls(C, call.path, R,
>                        R->isInteresting(C.getStackFrameFor(&call.path)))) continue;
>                  containsSomethingInteresting = true; break;
>   ... case Event: containsSomethingInteresting |= !event.isPrunable(); break;
> ```

The "interesting" set is built during the backward walk by `markInteresting(SymbolRef
| MemRegion | SVal | StackFrame)` (lines 2277-2335), recorded in
`InterestingSymbols`/`InterestingRegions`/`InterestingStackFrames` maps; queried by
`isInteresting(...)` (2389-2405). **Dorc read:** this is the mechanized form of "only
root-cause is reported." The bug *site* seeds interestingness; the backward walk marks
the symbols/frames that contributed to it; everything not on that chain is prunable and
dropped. Dorc's ⊤ cause-pointer is the seed; the dedup keeps only notes whose subject
is on the chain from the consumer back to the ⊤ origin.

**(3c) note-level dedup via FoldingSet** (lines 2054-2069). Even within one node's
visitor notes, identical pieces are de-duplicated by profiling each into a
`FoldingSetNodeID` and skipping repeats:

> ```cpp
> std::set<llvm::FoldingSetNodeID> DeduplicationSet;
> for (const PathDiagnosticPieceRef &Note : VisitorNotes->second) {
>   llvm::FoldingSetNodeID ID; Note->Profile(ID);
>   if (!DeduplicationSet.insert(ID).second) continue;   // identical note → drop
>   ...
> ```

### Report-level dedup = a coarse fingerprint (rq-E *and* rq-G hinge)

`[A-llvm-bugreporter-2025]` lines 2210-2246, 2983-2997. The single most transferable
mechanism. Each bug report computes a `Profile` hash and is filed into a
`BugReportEquivClass` (a `FoldingSet` bucket). The key is deliberately COARSE:

> ```cpp
> void PathSensitiveBugReport::Profile(llvm::FoldingSetNodeID &hash) const {
>   hash.AddInteger(static_cast<int>(getKind()));
>   hash.AddPointer(&BT);                       // the BugType (checker identity)
>   hash.AddString(getShortDescription());      // the message text
>   PathDiagnosticLocation UL = getUniqueingLocation();
>   if (UL.isValid()) UL.Profile(hash);
>   else hash.AddPointer(ErrorNode->getCurrentOrPreviousStmtForDiagnostics());
>   for (SourceRange range : Ranges) { ...; hash.Add(range.getBegin());
>                                            hash.Add(range.getEnd()); }
> }
> ```
> ```cpp
> // emitReport:
> llvm::FoldingSetNodeID ID; R->Profile(ID);
> BugReportEquivClass *EQ = EQClasses.FindNodeOrInsertPos(ID, InsertPos);
> if (!EQ) { EQ = new BugReportEquivClass(std::move(R)); EQClasses.InsertNode(...); }
> else EQ->AddReport(std::move(R));            // N paths, same key → one class
> ```

Then `findReportInEquivalenceClass` picks ONE representative report from the class to
render (lines 3037+). The grouping key is `(checker, message, uniqueing-location,
ranges)` — it OMITS the exploded path entirely. The *path* is the volatile,
high-cardinality artifact; the *site + checker + message* is the stable identity. This
is exactly the rq-G lesson: **group on stable site identity, not on the volatile
trace**. The `getUniqueingLocation` indirection is itself a deliberate
stability-control: a checker can declare a *uniqueing location* distinct from the bug
location so that, e.g., all leaks of an object uniqueed at its allocation site collapse
to one report regardless of which return path leaked it.

### The note-tag redesign — generate the note WHERE the fact is known

`[B-llvm-eliminating-visitors-2019]` (discourse) + `[B-llvm-d58367-2019]` (the patch).
The analyzer's primary author (Artem Dergachev, "NoQ") opened "[analyzer] On
eliminating BugReporterVisitors" (2019-02-19) to publicize patch D58367, landed as
`rL357323` "[analyzer] Introduce a simplified API for adding custom path notes":

> "it's an improved checker API that eliminates the need to write bug reporter
> visitors in checkers, but instead enables squeezing note-generating code directly
> into `addTransition`. This is cool because the message is generated within the part
> of code that already has all the information that's necessary to generate the
> message, which is much easier than reverse-engineering what happened 'a posteriori'
> in order to re-construct that information." — NoQ

This is a deep design lesson for Dorc, and it cuts AGAINST a naive "re-walk and dedup"
implementation. The visitor model = emit nothing during analysis, then *after* analysis
re-walk the exploded graph and have each visitor reverse-engineer what happened to
produce a note. The note-tag model (`NoteTag`, attached at the `addTransition` that
creates the state change) = capture the explanation *at the moment and site the fact
becomes known*, carry it on the transition, and render it later only if that node ends
up on the surfaced path. The note-tag closure even returns an *empty* string to
self-suppress when the surrounding report makes it irrelevant — interestingness-gating
folded into the note itself. **Dorc read (load-bearing):** when a command goes ⊤, the
cause is known *at that site, at that moment*. Attaching the cause-explanation to the ⊤
value at its origin (round-22's cause-pointer) is the note-tag pattern, and it is
strictly easier and more accurate than the alternative of letting each downstream
consumer reconstruct "why is my input ⊤?" after the fact. The cascade-dedup is then a
*consequence* of carrying the explanation on the value rather than re-deriving it N
times: downstream consumers don't generate competing notes because they were never the
ones holding the explanation. This reframes rq-E: the win is less "dedup N notes" and
more "only the origin ever had standing to emit the note."

### rustc `ErrorGuaranteed` — the type-level proof that suppresses the cascade

`[B-rustc-errorguaranteed-2026]` (rustc-dev-guide). rustc's mechanism for "only
complain once" is a zero-sized token threaded through the types:

> "`ErrorGuaranteed` is a zero-sized type that is unconstructable outside of the
> `rustc_errors` crate. It is generated whenever an error is reported to the user, so
> that if your compiler code ever encounters a value of type `ErrorGuaranteed`, the
> compilation is _statically guaranteed to fail_."

The suppression USE: once an error is emitted, the compiler manufactures error
placeholder values (`Ty::new_error(tcx, guar)` / `TyKind::Error(ErrorGuaranteed)`) and
keeps going; downstream type-checking that touches an error type is taught to stay
silent rather than emit derived nonsense ("this `_` has unknown type" cascades). The
*proof an error was already emitted* travels inside the poisoned value — which is
exactly Dorc's ⊤-with-cause-pointer shape, with one crucial caveat the guide states:

> "It does _not_ convey information about the _kind_ of error. For example, the error
> may be due (indirectly) to a delayed bug or other compiler error. Thus, you should
> not rely on `ErrorGuaranteed` when deciding whether to emit an error, or what kind of
> error to emit."

And the *delayed-bug* discipline (`span_delayed_bug`, formerly `delay_span_bug`):
record a diagnostic that MUST be flushed iff compilation actually fails; if the compiler
finishes without ever surfacing a real error, an un-flushed delayed bug is itself an ICE
("no errors encountered even though `delay_span_bug` issued" —
`[C-rustc-101869-2022]`). **Dorc read:** two transferable rules. (rule-a) the poisoned
value carries only "an error/⊤ was established and points HERE", deliberately NOT the
full classification — downstream code must not branch on the *kind* of the upstream
failure, only on the fact of it (this protects Dorc from consumers second-guessing the
root cause). (rule-b) the delayed-bug pattern is a safety net for the over-suppression
failure mode: if you suppress a cascade on the assumption a root error was reported, and
it turns out NO root error ever surfaced, that silent success is itself a bug to be
caught — Dorc's analog: a ⊤ that suppressed downstream notes but whose cause-note never
actually rendered should trip an internal invariant, not vanish.

## §2 (rq-G) Fleet-scale error grouping & fingerprinting

The o11y/crash-reporting corpus. The shared problem with Dorc's rq-G: a single fault
manifests as a flood of events across many sites/hosts/paths; group them to ONE so the
operator sees one cause, while not over-merging genuinely distinct faults. The recurring
tension is **stability vs precision**: a coarse key survives code movement but over-
groups; a precise key (full stack/path) splits one bug into many groups and churns on
every refactor.

### Sentry — the most directly transferable design

`[A-sentry-grouping-dev-2026]` (Sentry's own developer docs, the implementing team).
Sentry groups events into "issues" by a *fingerprint*; the default is computed from the
stack trace, NOT the whole event. The findings most relevant to Dorc:

**(g-caller-vs-callee) the fundamental ambiguity, stated cleanly:**

> "Sentry has a general tendency to group different paths towards an issue separately…
> Take a hypothetical function `get_current_user`. Let's imagine this function has a bug
> where it now starts failing with a `DataConsistencyError`… each of these callers will
> now create a different group. We can thus think of grouping as a problem of the source
> of the error. At any point the question can be asked if the source of the error is
> 'how we call a function' (caller error) or 'in the function' (callee error). Making
> this decision is impossible to make in a general sense."

This IS Dorc's rq-G problem verbatim: one bad oracle-declaration (the callee) fails
across M hosts × N scripts (the callers). Sentry's admission that caller-vs-callee "is
impossible to make in a general sense" is the strongest evidence that Dorc must not try
to auto-decide it — it must key on the thing it KNOWS is the single source (the
oracle-declaration site / the ⊤-origin site), i.e. choose callee-grouping by
construction because Dorc's analyzer actually knows where the rot originates, unlike a
post-hoc crash grouper. ~SUSPECT this is a genuine structural advantage: Dorc's
cause-pointer gives it the callee identity that Sentry has to *guess*.

**(g-stability-hierarchical-hash) survive code movement by emitting MULTIPLE hashes:**
Sentry's answer to "a frame gets reclassified / code moves" is to compute *both* a
fine and a coarse hash and associate the event with any group matching *either*:

> "consider a stack trace with three frames 'A1 B1 A2 A3'… they are all feeding into the
> fingerprint `[A1, B1, A2, A3]`. At a later point… marks `B1` as not in-app. The
> grouping algorithm if it were to fully ignore the `B1` frame now would create a new
> hash… However because we still create the full hash anyways the new event… would
> still find the already existing group. The hashes created are `[A1, B1, A2, A3]` as
> well as `[A1, A2, A3]`."

**Dorc read (load-bearing for site-keyed receipts):** the stability mechanism is
*emit a set of keys at varying coarseness and match on any*. When the site-identity
scheme changes (a script is edited, a line moves), keying receipts on a *hierarchy* —
e.g. `(oracle-decl-identity)` coarse + `(oracle-decl-identity, call-site-span)` fine —
lets a moved fine-key still collapse to the surviving coarse-key group. This is the
concrete grouping-key design the briefing asked for: do NOT pick one granularity; emit
nested keys and match the coarsest stable one.

**(g-source-line-overgroup-tradeoff) why including the source line both helps and
hurts:** Sentry's Python grouping feeds `module + function + context-line`:

> "modules and functions are relatively coarse indicators and a function can often fail
> from different branches, so taking the source code into account in addition is less
> likely to over-group. This however also means that when a refactoring takes place in a
> line that does not change the functionality but the source code, it can cause a new
> [group] to be created unnecessarily."

This is the direct statement of the receipts-stability tension: the more precisely you
key (include the literal sh text of the command), the more a no-op edit churns the
grouping. Dorc should key on *site identity* (which command, structurally) over *site
content* (the exact bytes) wherever it wants stability — content belongs in the
displayed evidence, not the grouping key.

**(g-merge-split-asymmetry) over-suppression is hard to UNDO:**

> "The system does not cope particularly well with merges and splits because the events
> in Snuba are generally considered immutable… a split fails to fully reconstruct the
> original state as some information… was lost in the process."

**Dorc read:** evidence for the over-suppression failure mode. Once you merge two
causes (or root-cause-suppress a second independent fire), recovering the lost
distinction is lossy. Argues for *under*-merging at capture time (keep the receipts that
would let you split later) and merging only in the *rendering* layer — never destroy the
per-site receipt just because it deduped to a root in one view.

**(g-secondary-sweep) the north-star aggregation pattern — group fine, sweep coarse
LATER:** Sentry's stated future direction is precisely Dorc's rq-G north-star:

> "the grouping algorithm could continue to just fingerprint the stack trace but a
> secondary process could come in periodically and sweep up related fingerprints into a
> larger group. If we take the `get_current_user` example the creation of 50 independent
> groups is not much of an issue if no alerts are fired. If after 5 minutes the system
> detected that they are in fact all very related (eg: the bug is 'in
> `get_current_user`') it could leave the 50 generated groups alone but create a new
> group that links the other 50… and let the user work with the larger group instead."

**Dorc read:** this is the "one rot event reads as ONE cause, fleet-aggregable"
requirement, and Sentry's framing tells Dorc HOW to get it cheaply: don't force the
single-group decision at capture; capture per-site (M hosts each get their receipt), and
aggregate to "one cause across the fleet" as a *separate, later, non-destructive* pass
keyed on the shared cause-identity. The alert/noise cost is what makes over-grouping
*feel* mandatory; if the fleet view aggregates without destroying the per-host detail,
both "one cause" and "which hosts" survive. Note the explicit cost signal: group
creation is "relatively expensive… due to the number of additional actions" (alerts,
regression detection) — the economic driver of grouping is alert-noise, not storage.

**(g-ai-embedding-caveat) the newest layer, and why it's a *fallback* not a *primary*:**
Sentry added an embedding-similarity merge (`should_call_seer_for_grouping`) that runs
*after* hash lookup and *before* new-group creation, gated by a circuit breaker and rate
limits. It exists to paper over the caller-vs-callee problem when the deterministic hash
over-splits. **Dorc read:** keep ML out of the correctness path; if Dorc ever wants
fuzzy aggregation it belongs strictly as a non-authoritative suggestion layer atop
deterministic site-keyed receipts (and Dorc, unlike Sentry, has the analyzer-known cause
and likely never needs it).

*(rq-G next: WER bucketing condensing/expanding failure taxonomy (CACM "Debugging in
the Very Large" — paywalled, see fetch-requests; ReBucket + Wikipedia for the
mechanics); Mozilla Socorro signature-rules config-as-maintenance; lighter-touch
journald/log dedup. rq-E: abstract-interp alarm clustering (Astrée/Frama-C/Infer);
ESLint/clippy duplicate-suppression.)*

## Graded sources

*(grades assigned by gathering subagent; conductor re-verification pending.)*

- `[A-llvm-bugreporter-2025]` · LLVM project, `clang/lib/StaticAnalyzer/Core/BugReporter.cpp` (the post-analysis diagnostic-path construction, pruning, dedup, and equivalence-class machinery) · https://raw.githubusercontent.com/llvm/llvm-project/main/clang/lib/StaticAnalyzer/Core/BugReporter.cpp · 2025 (main branch) · read: targeted-full (read `removeRedundantMsgs`, `eventsDescribeSameCondition`, `removeUnneededCalls`, `markInteresting`/`isInteresting`, both `Profile()` impls, `emitReport`/equivalence-class selection in full) · grade A not B: canonical first-party implementation, the executable answer to "how path notes are constructed by re-walking the exploded graph and then deduplicated/pruned to root-cause"; no rot (main). · relevance: THE primary rq-E dedup source AND the rq-G fingerprint precedent (coarse `Profile` key + equivalence classes). · via predecessor `[A-llvm-bugreporter-2025]` (octocode `githubSearchCode` keywords `removeRedundantMsgs,removeUnneededCalls`).
- `[A-llvm-bugreportervisitors-2025]` · LLVM project, `clang/lib/StaticAnalyzer/Core/BugReporterVisitors.cpp` (the visitors that enhance/suppress reports; whole-report heuristic suppression) · https://raw.githubusercontent.com/llvm/llvm-project/main/clang/lib/StaticAnalyzer/Core/BugReporterVisitors.cpp · 2025 (main branch) · read: targeted (read `LikelyFalsePositiveSuppressionBRVisitor::finalizeVisitor` and the interestingness-gated note pattern in full; rest of file is per-visitor note-text generation) · grade A not B: canonical first-party note-construction/suppression implementation, directly answers rq-E with executable code; no rot. · relevance: layer-2 whole-report denylist suppression (`markInvalid`) — the "minimum shippable suppression set" pattern. · via predecessor `[A-llvm-bugreportervisitors-2025]` (octocode `githubGetFileContent`, matchString `LikelyFalsePositiveSuppressionBRVisitor`).
- `[B-llvm-bugsuppression-2025]` · LLVM project, `clang/lib/StaticAnalyzer/Core/BugSuppression.cpp` (the `[[clang::suppress]]` user-directed suppression path) · https://raw.githubusercontent.com/llvm/llvm-project/main/clang/lib/StaticAnalyzer/Core/BugSuppression.cpp · 2025 (main branch) · read: full (small file, ~280 lines) · grade B not A: canonical first-party source, fully read, but covers the user-directed attribute path (decl/range containment), not the automatic note-pruning that is rq-E's core; primary but narrow-scope. · relevance: layer-1 admin region-mute model (range containment); a candidate model for Dorc's per-site vouching, but user-driven not cause-driven. · via predecessor `[B-llvm-bugsuppression-2025]` (octocode `githubGetFileContent`).
- `[B-llvm-eliminating-visitors-2019]` · discourse.llvm.org, Artem Dergachev (NoQ), "[analyzer] On eliminating BugReporterVisitors" · https://discourse.llvm.org/t/analyzer-on-eliminating-bugreportervisitors/51206 · 2019-02-19 · read: full (short thread; OP is the load-bearing statement) · grade B not A: primary-author statement of design intent, fully read, but it is a brief announcement pointing at the patch rather than the implementation itself; the executable detail lives in D58367/the NoteTag code. · relevance: the note-tag-at-analysis-time rationale ("generate the message where the info already exists, not by reverse-engineering a posteriori") — directly motivates attaching Dorc's ⊤-cause at origin rather than re-deriving downstream. · via Kagi `Clang static analyzer note tag API redesign eliminate BugReporterVisitor`.
- `[B-llvm-d58367-2019]` · reviews.llvm.org, Phabricator review D58367 → `rL357323` "[analyzer] Introduce a simplified API for adding custom path notes" (author NoQ; reviewers dcoughlin, xazax.hun, Szelethus, et al.) · https://reviews.llvm.org/D58367 · 2019 (landed Apr 2019) · read: targeted (review metadata, commit list, changed-files; inline diffs not rendered by fetch) · grade B not C: first-party patch record confirming the NoteTag API landed and which files it touched (`BugReporterVisitors.h/.cpp`, `CheckerContext.h`, `ExprEngine`, `MIGChecker` as the first adopter); not A because the diff body itself didn't render, so the mechanism detail is corroborated rather than read line-by-line. · relevance: confirms the eliminate-visitors proposal became shipped code; MIGChecker as the migration exemplar. · via the discourse thread `[B-llvm-eliminating-visitors-2019]`.
- `[B-rustc-errorguaranteed-2026]` · rust-lang/rustc-dev-guide, `src/diagnostics/error-guaranteed.md` · https://github.com/rust-lang/rustc-dev-guide/blob/main/src/diagnostics/error-guaranteed.md (raw read) · 2026 (main) · read: full · grade B not A: authoritative maintained dev-guide, fully read, but it is the *guide* prose; the actual `Ty::new_error`/`TyKind::Error` suppression code in `rustc` proper is one level deeper (not read this turn) so the suppression-USE mechanics are corroborated-from-docs not source-verified. · relevance: the "poisoned value carries proof an error was already emitted" pattern = Dorc's ⊤-with-cause-pointer; the explicit caveats (does NOT convey kind; don't branch on it; flush-or-ICE delayed-bug) directly inform Dorc's cause-pointer discipline. · via Kagi `rustc ErrorGuaranteed span_delayed_bug suppress cascade`.
- `[C-rustc-101869-2022]` · rust-lang/rust#101869, "internal compiler error: no errors encountered even though `delay_span_bug` issued" · https://github.com/rust-lang/rust/issues/101869 · 2022-09-15 · read: snippet (title + symptom from search result) · grade C: real-world artifact of the delayed-bug discipline failing, but read only as a snippet, so capped low. · relevance: concrete evidence that the over-suppression net (delayed bug never flushed) is itself treated as a bug — the safety-net pattern for Dorc's "suppressed cascade whose root never rendered". · via Kagi result alongside the ErrorGuaranteed query.
- `[A-sentry-grouping-dev-2026]` · develop.sentry.dev, Sentry "Grouping" backend developer documentation (the implementing team's own internals doc) · https://develop.sentry.dev/backend/application-domains/grouping/ · 2026 (live docs) · read: full · grade A not B: first-party engineering documentation by the team that built it, read in full, exceptionally candid about failure modes (caller-vs-callee impossibility, merge/split lossiness, the secondary-sweep future plan, native-platform mangling instability); reads as primary design rationale, not marketing. · relevance: THE most transferable rq-G source — every load-bearing rq-G finding (hierarchical-hash stability, callee-by-construction, group-fine-sweep-coarse, merge irreversibility, ML-as-fallback) comes from here. · via Kagi `Sentry error grouping fingerprinting algorithm evolution`.
