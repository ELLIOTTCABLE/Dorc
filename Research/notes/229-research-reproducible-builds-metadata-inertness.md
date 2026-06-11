# 229 — research: reproducible-builds & metadata-inertness discipline (the erasability gate)

> Deep-research round, 2026-06-11, serving research-front **rq-F** (round-22 provenance
> "receipts" plane). The round-22 receipts plane carries per-value origin metadata used ONLY
> to refuse-or-explain; it must NEVER influence the analyzer's *decisions* (join order, fold
> results, license verdicts). The planned CI enforcement — the **"erasability gate"** — runs
> the analyzer twice on identical input, once with the receipts plane stripped, and asserts the
> DECISION outputs are byte-identical (explanation output exempt). This is structurally the
> compiler world's "debug info must not affect codegen" invariant and the reproducible-builds
> world's "metadata must not leak into the artifact" discipline. This note mines that prior art
> for (a) enforcement MECHANISMS adoptable in a plain Rust workspace (no proc-macros; build
> scripts/tests/lints fine), (b) the recurring leak/violation TAXONOMY, and (c) how "identical"
> is principledly DEFINED (what is stripped/normalized before comparison) — which directly
> shapes our gate's spec of which outputs must match.
>
> Findings slugged `finding-N`; leak categories `leak-category-NAME`; mechanisms
> `mechanism-NAME`; sources `[grade-slug-year]`, graded list in final section. Confidence marks
> per project convention (+SURE / ~SUSPECT / -GUESS / --WONDER). Seed handoff from a predecessor
> turn (downloaded GCC/LLVM/r-b.org/rustc sources, read from disk) banked below and re-cited.

## §0 Conclusions up front

*(filled incrementally; see per-question sections for evidence)*

### The leak-category taxonomy (compact, cited)

*(table — filled after the r-b.org sweep)*

### Recommended enforcement mechanisms for an analyzer-inertness gate (Rust workspace)

*(2-3 mechanisms with effort guesses — filled after mechanism evidence collected)*

### The cleanest comparison-partition definition language found

*(filled after the partition evidence is collected)*

---

## §1 GCC `-fcompare-debug`: the literal shipped erasability gate

The single closest prior-art to Dorc's gate. `-fcompare-debug` compiles **twice** — once
normally, once with debug-info toggled — dumps the final internal representation of both, and
**errors if they differ**. This is the run-twice-with/without-metadata-and-compare gate,
shipped in a production compiler for ~17 years.

**finding-compare-debug-semantics (+SURE).** The flag's verbatim definition
[A-gcc-developer-options-2024]:

> `-fcompare-debug[=opts]` — If no error occurs during compilation, run the compiler a second
> time, adding `opts` and `-fcompare-debug-second` to the arguments passed to the second
> compilation. Dump the final internal representation in both compilations, and print an error
> if they differ.
>
> If the equal sign is omitted, the default `-gtoggle` is used.

So the *default* metadata-variance injected is `-gtoggle`, and the *comparison object* is "the
final internal representation" (GCC's RTL dump), not the shipped binary — they compare an
intermediate, which is a deliberate partition choice (see §1 normalization below).

**finding-compare-debug-variance-axis (+SURE).** The variance is injected by `-gtoggle`, whose
verbatim semantics [A-gcc-developer-options-2024]:

> `-gtoggle` — Turn off generation of debug info, if leaving out this option generates it, or
> turn it on at level 2 otherwise. The position of this argument in the command line does not
> matter; it takes effect after all other options are processed, and it does so only once, no
> matter how many times it is given. This is mainly intended to be used with `-fcompare-debug`.

Key engineering detail: the variance toggle is **applied last, exactly once, position-
independent** — i.e. the variance axis is normalized so the gate can't be defeated by option
ordering. Maps to Dorc: the "strip receipts" transformation must be applied deterministically
at one well-defined point, not threaded through config.

**finding-compare-debug-coverage-enforcement (+SURE).** GCC ships an *anti-evasion* mechanism
so the gate can't silently become a no-op — verbatim [A-gcc-developer-options-2024]:

> The environment variable `GCC_COMPARE_DEBUG`, if defined, non-empty and nonzero, implicitly
> enables `-fcompare-debug`. … To verify full coverage during `-fcompare-debug` testing, set
> `GCC_COMPARE_DEBUG` to say `-fcompare-debug-not-overridden`, which GCC rejects as an invalid
> option in any actual compilation (rather than preprocessing, assembly or linking). To get
> just a warning, setting `GCC_COMPARE_DEBUG` to `-w%n-fcompare-debug not overridden` will do.

This is a *coverage canary*: a sentinel value that MUST error if the gate actually ran, so a
build harness can prove the gate wasn't quietly skipped. Directly relevant to "where compare-
gates rot" — GCC pre-empted gate-disabling by making "did the gate run?" itself checkable.

**finding-compare-debug-second-isolation (+SURE).** The second compilation is run with options
to suppress side effects and renamed temp files — verbatim [A-gcc-developer-options-2024]:

> `-fcompare-debug-second` — This option is implicitly passed to the compiler for the second
> compilation requested by `-fcompare-debug`, along with options to silence warnings, and
> omitting other options that would cause the compiler to produce output to files or to
> standard output as a side effect. Dump files and preserved temporary files are renamed so as
> to contain the `.gk` additional extension during the second compilation, to avoid
> overwriting those generated by the first.

Two design lessons: (i) the second run is *output-suppressed* (warnings silenced — i.e. the
explanation/diagnostic plane is exempted from the comparison, exactly Dorc's "explanation
output exempt"); (ii) the two runs' side artifacts are *namespaced apart* so they don't
clobber each other — the gate harness must isolate the two analyzer runs' scratch.

### §1 normalization — the comparison partition (what gets stripped before diff)

**finding-compare-debug-partition (+SURE).** GCC does NOT diff raw dumps; it provides dump
modifiers that **erase identity-irrelevant noise** so `diff` is meaningful. Verbatim
[A-gcc-developer-options-2024]:

> `-fdump-noaddr` — When doing debugging dumps, suppress address output. This makes it more
> feasible to use diff on debugging dumps for compiler invocations with different compiler
> binaries and/or different text/bss/data/heap/stack/dso start locations.
>
> `-fdump-unnumbered` — When doing debugging dumps, suppress instruction numbers and address
> output. This makes it more feasible to use diff on debugging dumps for compiler invocations
> with different options, in particular with and without `-g`.
>
> `-fdump-unnumbered-links` — When doing debugging dumps … suppress instruction numbers for
> the links to the previous and next instructions in a sequence.

This is the comparison-partition design in miniature: **addresses and instruction-numbers are
declared identity-irrelevant** (they legitimately differ run-to-run / with-vs-without-`-g`) and
normalized away; everything else must match. The partition line is drawn at "would this differ
for a non-bug reason?" → strip it. For Dorc: receipt IDs, origin-set ordering, and any
monotonic counters in the decision dump are the analogues to strip; the residual decision
content must match exactly.

**finding-random-seed-determinism (+SURE).** GCC also exposes the determinism knob for symbol
naming — verbatim [A-gcc-developer-options-2024]:

> `-frandom-seed=string` — This option provides a seed that GCC uses in place of random numbers
> in generating certain symbol names that have to be different in every compiled file. … You
> can use the `-frandom-seed` option to produce reproducibly identical object files. … The
> string should be different for every file you compile.

Relevant as a *variance-injection axis* (vary the seed between the two builds to flush out
seed-dependence) and as the canonical "pin the one nondeterminism source" move — Dorc's analogue
is the hash-seed for any `HashMap` that survived the BTreeMap mandate.

---

## §2 LLVM/Clang: `debugify` / `check-debugify` and "debug info must not affect optimizations"

LLVM's mechanism differs from GCC's in a way that is *more* adoptable for Dorc: instead of
comparing two whole compilations, it **injects synthetic metadata, runs the transform, then
checks the metadata survived** — a single-run, inject-then-verify shape. All verbatim from
[A-llvm-howtoupdatedebuginfo-2025].

**finding-debugify-mechanism (+SURE).** The core utility:

> The `debugify` testing utility is just a pair of passes: `debugify` and `check-debugify`. The
> first applies synthetic debug information to every instruction of the module, and the second
> checks that this DI is still available after an optimization has occurred, reporting any
> errors/warnings while doing so. The instructions are assigned sequentially increasing line
> locations, and are immediately used by debug value records everywhere possible.

So `debugify` *manufactures a known, dense, checkable metadata plane* (every instruction gets a
synthetic source line) precisely so that any pass that drops/corrupts it is caught. The Dorc
analogue: rather than relying on real receipts being present, the gate could *synthesize* a
maximal receipt plane (tag every value) and assert decisions are unchanged AND all receipts are
accounted-for — denser coverage than real input provides.

**finding-debugify-each (+SURE).** The per-pass variant, explicitly modeled on `-verify-each`:

> `$ opt -debugify-each -O2 sample.ll` — Prepend `-debugify` before and append `-check-debugify
> -strip` after each pass on the pipeline (similar to `-verify-each`).

This localizes the violation to the *individual pass* that broke the invariant — the gate runs
between every transformation, not just end-to-end, so the blame is attributed to one pass.
Maps to Dorc: run the erasability check after each analyzer *stage* (parse → taint → elision-
decide) to localize which stage leaked receipts into decisions, not just "somewhere".

**finding-debugify-regression-stability (+SURE).** The gate's own output is held stable:

> The output of the `debugify` pass must be stable enough to use in regression tests. Changes
> to this pass are not allowed to break existing tests. … Regression tests must be robust.
> Avoid hardcoding line/variable numbers in check lines.

I.e. the *injected metadata itself* is deterministic and the comparison avoids brittle exact
numbers — a flaky-diff-avoidance design decision baked in from the start.

### §2 the sanctioned-divergence partition (DebugLoc special values)

**finding-debugloc-partition (+SURE).** This is the cleanest comparison-partition vocabulary I
found anywhere. LLVM does NOT treat "no source location" as one thing; it has four *named*
reasons an instruction can legitimately lack a location, so the verifier can tell a sanctioned
absence from a bug — verbatim:

> - `DebugLoc::getCompilerGenerated()`: This indicates that the instruction is a compiler-
>   generated instruction, i.e. it is not associated with any user source code.
> - `DebugLoc::getDropped()`: This indicates that the instruction has intentionally had its
>   source location removed, according to the rules for dropping locations; this is set
>   automatically by `Instruction::dropLocation()`.
> - `DebugLoc::getUnknown()`: This indicates that the instruction does not have a known or
>   currently knowable source location, e.g. that it is infeasible to determine the correct
>   source location, or that the source location is ambiguous in a way that LLVM cannot
>   currently represent.
> - `DebugLoc::getTemporary()`: This is used for instructions that we don't expect to be emitted
>   (e.g. `UnreachableInst`), and so should not need a valid location; if we ever try to emit a
>   temporary location into an object/asm file, this indicates that something has gone wrong.

And the design rule that follows (the partition-design principle, verbatim):

> Ordinarily these special locations are identical to an absent location, but LLVM built with
> coverage-tracking … will keep track of these special locations in order to detect
> unintentionally-missing locations; for this reason, the most important rule is to not apply
> any of these if it isn't clear which, if any, is appropriate — an absent location can be
> detected and fixed, while an incorrectly annotated instruction is much [harder to detect].

The load-bearing insight for Dorc's gate spec: **make the EXEMPT category an explicit, named,
narrow allowlist of reasons, and bias toward "not exempt" when unsure** — because a wrongly-
exempted divergence hides a real leak forever, whereas a wrongly-flagged divergence is a noisy
but *fixable* false positive. This is the inverse of the usual "suppress false positives"
instinct, and it's the right bias for a *correctness* gate.

**finding-debugify-coverage-tracking (+SURE).** False-positive suppression is opt-in and
build-gated, not default:

> there are valid reasons for instructions to not have source locations. Therefore, when
> detecting dropped or not-generated source locations, it may be preferable to avoid detecting
> cases where the missing source location is intentional. For this, you can use the "coverage
> tracking" feature … by setting the CMake flag
> `-DLLVM_ENABLE_DEBUGLOC_COVERAGE_TRACKING=COVERAGE`. When this has been set, LLVM will enable
> runtime tracking of `DebugLoc` annotations, allowing debugify to ignore instructions that
> have an explicitly recorded reason for not having a source location.

**finding-debugify-origin-tracking (+SURE).** The provenance-of-the-bug feature — directly
mirrors Dorc's own receipts-as-debugging-aid idea, applied recursively to the gate itself:

> set the CMake flag to enable "origin tracking", `-DLLVM_ENABLE_DEBUGLOC_COVERAGE_TRACKING=
> COVERAGE_AND_ORIGIN`. This flag adds more detail to debugify's output, by including one or
> more stacktraces with every missing source location, capturing the point at which the empty
> source location was created, and every point at which it was copied to an instruction, making
> it trivial in most cases to find the origin of the underlying bug.

So when the gate fires, LLVM can tell you *where the metadata was lost AND every hop it was
propagated* — a worked example of "receipts on the receipt failure." Effort note: this is a
heavyweight build-mode feature, not free.

### §2 regression routing and cost management

**finding-verify-each-preserve-routing (+SURE).** The original-DI-preservation mode exports
machine-readable findings and renders them human-readable — the regression-routing pipeline:

> `$ opt -verify-debuginfo-preserve -verify-di-preserve-export=sample.json -pass-to-test
> sample.ll` … and then use the `llvm/utils/llvm-original-di-preservation.py` script to
> generate an HTML page with the issues reported in a more human-readable form: `$
> llvm-original-di-preservation.py sample.json --report-file sample.html`.

JSON for machines (CI gating), HTML for humans (triage) — the two-audience split Dorc's note
220 already identified for provenance UX, here applied to the inertness gate's own output.

**finding-verify-each-cost (+SURE).** The honest cost caveat — a "where compare-gates rot:
they get too slow" data point, with the shipped mitigation:

> Please do note that running `-verify-each-debuginfo-preserve` on big projects could be heavily
> time consuming. Therefore, we suggest using `-debugify-func-limit` with a suitable limit
> number to prevent extremely long builds. … `-debugify-func-limit=100` [tests] up to 100
> functions (per compile unit) per pass.

The per-pass gate is acknowledged as expensive enough to need a sampling knob. For Dorc this is
reassuring rather than alarming (per AGENTS: analyzer-local big-O is dominated by network), but
it's the documented failure-pressure: a per-stage gate that's too slow gets a sampling escape
hatch, which is itself a coverage hole.

**finding-mir-debugify-secondlevel (+SURE).** The same mechanism is re-implemented at a second
IR level (machine IR), with `mir-debugify` / `mir-check-debugify` — evidence that the inject-
then-verify pattern generalizes across representation levels. Relevant only as confirmation the
pattern is robust, not as a distinct mechanism.

**finding-known-false-positives (~SUSPECT).** The doc admits residual unsoundness:

> Please do note that there are some known false positives, for source locations and debug
> record checking, so that will be addressed as a future work.

Even LLVM's mature gate has acknowledged false positives — a realistic expectation-setter: a
metadata-inertness gate is rarely perfectly clean, and the team treats remaining FPs as known
debt rather than a reason to abandon the gate.

