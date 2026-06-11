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

*(rq-E next: the note-tag API redesign discourse thread — primary-author intent to
eliminate visitors in favor of tags attached at analysis time; rustc ErrorGuaranteed
cascade-suppression. rq-G: Sentry/WER/Socorro corpus — entirely unstarted.)*

## Graded sources

*(grades assigned by gathering subagent; conductor re-verification pending.)*

- `[A-llvm-bugreporter-2025]` · LLVM project, `clang/lib/StaticAnalyzer/Core/BugReporter.cpp` (the post-analysis diagnostic-path construction, pruning, dedup, and equivalence-class machinery) · https://raw.githubusercontent.com/llvm/llvm-project/main/clang/lib/StaticAnalyzer/Core/BugReporter.cpp · 2025 (main branch) · read: targeted-full (read `removeRedundantMsgs`, `eventsDescribeSameCondition`, `removeUnneededCalls`, `markInteresting`/`isInteresting`, both `Profile()` impls, `emitReport`/equivalence-class selection in full) · grade A not B: canonical first-party implementation, the executable answer to "how path notes are constructed by re-walking the exploded graph and then deduplicated/pruned to root-cause"; no rot (main). · relevance: THE primary rq-E dedup source AND the rq-G fingerprint precedent (coarse `Profile` key + equivalence classes). · via predecessor `[A-llvm-bugreporter-2025]` (octocode `githubSearchCode` keywords `removeRedundantMsgs,removeUnneededCalls`).
- `[A-llvm-bugreportervisitors-2025]` · LLVM project, `clang/lib/StaticAnalyzer/Core/BugReporterVisitors.cpp` (the visitors that enhance/suppress reports; whole-report heuristic suppression) · https://raw.githubusercontent.com/llvm/llvm-project/main/clang/lib/StaticAnalyzer/Core/BugReporterVisitors.cpp · 2025 (main branch) · read: targeted (read `LikelyFalsePositiveSuppressionBRVisitor::finalizeVisitor` and the interestingness-gated note pattern in full; rest of file is per-visitor note-text generation) · grade A not B: canonical first-party note-construction/suppression implementation, directly answers rq-E with executable code; no rot. · relevance: layer-2 whole-report denylist suppression (`markInvalid`) — the "minimum shippable suppression set" pattern. · via predecessor `[A-llvm-bugreportervisitors-2025]` (octocode `githubGetFileContent`, matchString `LikelyFalsePositiveSuppressionBRVisitor`).
- `[B-llvm-bugsuppression-2025]` · LLVM project, `clang/lib/StaticAnalyzer/Core/BugSuppression.cpp` (the `[[clang::suppress]]` user-directed suppression path) · https://raw.githubusercontent.com/llvm/llvm-project/main/clang/lib/StaticAnalyzer/Core/BugSuppression.cpp · 2025 (main branch) · read: full (small file, ~280 lines) · grade B not A: canonical first-party source, fully read, but covers the user-directed attribute path (decl/range containment), not the automatic note-pruning that is rq-E's core; primary but narrow-scope. · relevance: layer-1 admin region-mute model (range containment); a candidate model for Dorc's per-site vouching, but user-driven not cause-driven. · via predecessor `[B-llvm-bugsuppression-2025]` (octocode `githubGetFileContent`).
