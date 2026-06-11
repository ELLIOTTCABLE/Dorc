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

