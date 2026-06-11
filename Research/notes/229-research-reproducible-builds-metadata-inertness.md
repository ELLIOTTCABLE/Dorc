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

---

## §3 reproducible-builds.org: the invariant, SOURCE_DATE_EPOCH, and the leak taxonomy

**finding-rb-invariant (+SURE).** The canonical statement of the metadata-inertness invariant,
in RFC2119 keywords [A-rb-source-date-epoch-2017]:

> The value MUST be reproducible (deterministic) across different executions of the build,
> depending only on the source code. … Build processes MUST use this variable for embedded
> timestamps in place of the "current" date and time.

"Depending only on the source code" is the reproducible-builds north star, and it is exactly
Dorc's gate spec restated: decision output MUST depend only on the analyzed sh + probe records,
NOT on the receipts plane (the build's "current time" analogue — present, useful, but must not
leak into the artifact).

**finding-rb-normalize-dont-prohibit (+SURE).** The *timestamp-clamping* pattern is the key
engineering move — you don't forbid the volatile input, you *normalize* it to a deterministic
function of source [A-rb-source-date-epoch-2017]:

> Where build processes embed timestamps that are not "current", but are nevertheless still
> specific to one execution of the build process, they MUST use a timestamp no later than the
> value of this variable. This is often called "timestamp clamping". … One can reasonably
> assume that all source timestamps are before SOURCE_DATE_EPOCH and all builds take place
> after it. This means we can efficiently both preserve source-based timestamps and omit
> build-specific timestamps, by rewriting timestamps more recent than SOURCE_DATE_EPOCH back to
> the latter. See for example the `--clamp-mtime` option to GNU tar.

This is the third comparison-partition strategy (alongside GCC's strip-the-noise and LLVM's
named-exempt-reasons): **canonicalize the volatile field to a deterministic value before it can
leak.** For Dorc: if some decision-adjacent field legitimately varies (a timestamp in a probe
record), clamp/canonicalize it in BOTH runs rather than trying to exempt it from comparison.

**finding-rb-deferred-formatting (+SURE).** A subtle partition rule worth noting verbatim:

> Formatting MUST be deferred until runtime if an end user should observe the value in their own
> locale or timezone. … Build processes MUST NOT unset this variable for child processes if it
> is already present. … If the value is malformed, the build process SHOULD exit with a
> non-zero error code.

"Defer locale/timezone formatting to runtime" = keep the volatile *presentation* out of the
*artifact*; the value travels as a normalized integer, rendered only at the human boundary.
Direct analogue: Dorc's receipts may be richly formatted in *explanation* output (the human
boundary) but must travel as normalized data that never touches decisions.

*(The broader r-b.org leak-category page sweep — env-variations, volatile-inputs, stable-inputs/
ordering, value-initialization, stripping, version-information, timestamps, timezones, locales,
archives — is gathered in §4 below and tabulated in §0.)*

---

## §4 rustc reproducibility & the ordering-nondeterminism lint

**finding-rustc-query-instability-lint (+SURE).** rustc ships a *first-class compiler lint*
against the canonical ordering leak — iterating a `HashMap` whose order leaks into output.
Verbatim from the internal-lint definition [A-rustc-potential-query-instability-2025]:

> The `potential_query_instability` lint detects use of methods which can lead to potential
> query instability, such as iterating over a `HashMap`. Due to the incremental compilation
> model, queries must return deterministic, stable results. `HashMap` iteration order can
> change between compilations, and will introduce instability if query results expose the
> order.

This is the strongest *enforcement-beyond-convention* mechanism for Dorc's ordering discipline:
not "we agreed to use BTreeMap" but a lint that *fires at the call site* of an order-exposing
iteration. rustc's own codebase denies this lint and routes all order-sensitive iteration
through deterministic wrappers (`FxIndexMap`/sorted helpers) — the structural ban, not the
convention. (The mechanism's adoptability for a plain Rust workspace — via `clippy` or a custom
`dylint` — is assessed in §0 mechanisms; the rustc lint itself is `rustc`-internal and not
directly usable, so Dorc would reimplement the *idea*.)

*(rustc's `--remap-path-prefix`, the reproducible-build tracking issues, and the other known
nondeterminism sources — codegen-unit parallelism, incremental fingerprints — are gathered by
sub-subagent in §5 and folded into §0.)*

---

## §5 rustc reproducibility: tracking issues, path-remap, and the determinism rationale

**finding-rustc-remap-path-prefix (+SURE).** The build-path normalization flag, verbatim
[B-rustc-cli-args-2026]:

> `--remap-path-prefix`: remap source paths in output — Remap source path prefixes in all
> output, including compiler diagnostics, debug information, macro expansions, etc. It takes a
> value of the form `FROM=TO` where a path prefix equal to `FROM` is rewritten to the value
> `TO`. This flag may be specified multiple times.

There is also `--remap-path-scope` defining *which* output scopes get remapped — i.e. the
normalization is *scoped*, not all-or-nothing. (Dorc analogue: if any decision-adjacent string
carries a path, it must be remapped in both gate runs.)

**finding-rustc-tracking-issues (+SURE).** Rust's reproducibility effort is tracked under the
`A-reproducibility` label. Two issues are load-bearing for Dorc's gate question:

- **#34902** `Bit-for-bit deterministic / reproducible builds` [B-rust-issue-34902-2016] —
  opened 2016, closed *completed* 2020-08-06 (76 comments). Notably the issue body is a
  *discussion*, not a checklist: "Much of the diff output is due to build-id differences, which
  can be ignored since they are caused by other deeper issues and will go away once these deeper
  issues are fixed." (No verbatim source-taxonomy in the body; it lives scattered in comments.)
- **#75362** `CI for deterministic / reproducible builds` [B-rust-issue-75362-2020] — opened
  2020, **still OPEN** as of 2025-04-16. This is the directly-relevant evidence: *rustc itself
  has no standing CI reproducibility gate.* Verbatim:

  > #34902 was finally closed as we got a positive result on tests.r-b.org for rustc 1.44.1 on
  > Debian Unstable, where we test for build-path-independent reproducibility. However for rustc
  > 1.45.0 the test turned negative again. … Since many/most contributors are not aware of all
  > of the details needed to retain build-path-independent reproducibility, it would be good to
  > have some CI to ensure this property in the long run. Running a full build twice is costly,
  > but perhaps some other solution would be just as effective, e.g. running a stage1 build
  > twice, or running it for the beta channel, and/or running with a pre-built LLVM.

The lesson for Dorc (finding-rustc-no-ci, ~SUSPECT→+SURE on the absence): a project that *can*
reproduce but relies on an *external* checker (Debian's) and lacks an *internal* gate watched
its property regress immediately (1.44.1 reproducible → 1.45.0 not) and acknowledges
contributors "are not aware of all the details." This is the strongest argument FOR Dorc
shipping the erasability gate in-tree from day one rather than relying on after-the-fact audit:
the regression pressure is constant and silent without a gate.

**finding-rustc-nondeterminism-sources (+SURE).** Concrete, separately-filed nondeterminism
bugs (the de-facto source taxonomy):

- **LLVM-codegen drift on toolchain bump:** #90301 `reproducible builds broken in rustc 1.56.0
  due to LLVM 13 update` [B-rust-issue-90301-2021] — bisected to the LLVM-13 upgrade,
  "differences seem to start in `rustc_codegen_ssa`."
- **`-C metadata` hash differs across OS:** #71361 `Non-reproducible builds depending on the
  compiling OS` [B-rust-issue-71361-2020] — same crate getting `-C metadata=92487154152022d3`
  on Linux vs `9fc982d890c0358d` on macOS.
- **Parallel-codegen nondeterminism survives `codegen-units=1`:** #50556 `reproducible builds:
  non-deterministic use of cmpq` [B-rust-issue-50556-2018] — "still non-deterministic … I'm not
  sure if this decision is made by llvm or rustc, but it seems this happens in a
  non-deterministic way."
- **Source-path leak into panic/debug strings:** #97955 [B-rust-issue-97955-2022] — full paths
  like `/Users/runner/.cargo/registry/src/…` embedded in the binary; the `--remap-path-prefix`
  motivating case.

**finding-rustc-determinism-rationale (+SURE).** The incremental-compilation model is *why*
rustc must be deterministic — and it makes the inverse explicit (verbatim, rustc-dev-guide
[B-rustc-devguide-incr-2026]):

> First, if all the inputs to query Q are colored green, then the query Q **must** result in
> the same value as last time and hence need not be re-executed (or else the compiler is not
> deterministic). … One key point is that the query DAG also tracks ordering; that is, for each
> query Q, we not only track the queries that Q reads, we track the **order** in which they
> were read.

So rustc's determinism isn't aesthetic — incrementality *depends* on it (a non-deterministic
query breaks the green-means-skip optimization). Dorc parallel: the *receipts plane is the
"debug info" that must not change the "green" decision* — if receipts leaked into a decision,
two runs differing only in receipts would diverge, exactly the failure rustc's model forbids.

**finding-rustc-untracked-query-lint (+SURE).** Beyond ordering, rustc lints a *second* leak
class — reading state the query system doesn't track. Verbatim from the lint source
[A-rustc-fx-src-2026 / internal.rs]:

> The `untracked_query_information` lint detects use of methods which leak information not
> tracked by the query system, such as whether a `Steal<T>` value has already been stolen. In
> order not to break incremental compilation, such methods must be used very carefully or not
> at all.

This is a *direct analogue of Dorc's gate*: a lint against "decisions reading un-tracked side
information." The receipts plane is precisely "information the decision must not read." ~SUSPECT
a custom Dorc lint modeled on `untracked_query_information` (fire when a decision-producing
function takes a receipt-typed argument) is the cleanest static enforcement — see §0 mechanisms.

---

## §6 distro-scale CI economics & the variance-injection practice

This is where "run twice and compare" becomes *adversarial* — Debian doesn't run the same build
twice, it runs it twice with **deliberately maximized environmental variance** between the two,
so any environment-dependence surfaces as a diff. This is the single most important
direction-shaper for Dorc's gate.

**finding-debian-variance-table (+SURE).** The complete variation set Debian injects between
"first build" and "second build" [A-debian-variations-2026], verbatim-derived (page self-dates
to 2026-05-27):

| axis | first build | second build |
| --- | --- | --- |
| hostname | a real builder name (infom01-amd64, ionos1-amd64, …) | `i-capture-the-hostname` (sentinel) |
| domainname | `debian.net` | `i-capture-the-domainname` (sentinel) |
| `CAPTURE_ENVIRONMENT` env | *unset* | `"I capture the environment"` (sentinel) |
| `TZ` | `Etc/GMT+12` | `Etc/GMT-14` (26h apart) |
| `LANG` / `LANGUAGE` / `LC_ALL` | `C.UTF-8` / `en_US:en` / unset | `et_EE.UTF-8` (amd64) or `nl_BE.UTF-8` (arm64) |
| `PATH` | normal | normal + `/i/capture/the/path` |
| build user (uid/gid/name/HOME) | 1111 / `pbuilder1` / `…/first-build` | 2222 / `pbuilder2` / `…/second-build` |
| `niceness` | 10 | 11 |
| `/bin/sh` | dash | bash |
| login shell, GECOS | `/bin/sh`, "first user,…" | `/bin/bash`, "second user,…" |
| `DEB_BUILD_OPTIONS` parallel | e.g. `parallel=16` | `nocheck parallel=15` (different count) |
| UTS namespace | shared with host | `unshare --uts` |
| kernel version | one of a set | systematically varied (amd64) |
| umask | 0022 | 0002 |
| date/time | today | +398 days (and "future builds" run +6h23min ahead) |

Two structural lessons beyond the list:

- **Sentinel/canary strings** (`i-capture-the-hostname`, `"I capture the environment"`): the
  varied value is a *recognizable marker*, so a diff doesn't just say "differs" — it says "your
  output literally contains the hostname." This is a strictly better failure-mode than an opaque
  byte-diff, and it's cheap. (Dorc analogue: when varying a decision-irrelevant receipt field
  between runs, set it to a sentinel so any leak into a decision is *self-identifying* in the
  diff.) +SURE this is adoptable and high-value.
- **The variance set was WALKED BACK over time** [A-debian-variations-2026, finding-variance-
  walkback]: "build path … *(not varied anymore)*" and filesystem-order via disorderfs
  "*temporarily not* varied." These were *historically* varied and got tuned down — direct
  evidence that maximal variance injection has a noise cost that even the flagship project
  retreated from on specific axes. ~SUSPECT the lesson is: variance axes need individual
  enable/disable, because some are higher-noise-per-bug-caught than others.

**finding-reprotest-tool (+SURE).** The variance-injection is packaged as a reusable tool,
`reprotest` [A-rb-tools-2026]: "reprotest builds the same source code in different environments
and then checks the binaries produced by the builds to see if changing the environment, without
changing the source code, changed the generated binaries." And the *adversarial filesystem*,
`disorderfs`: "an overlay FUSE filesystem that deliberately introduces non-determinism into
filesystem metadata. For example, it can randomize the order in which directory entries are
read." disorderfs is the purest "variance injection for the ordering leak" — it actively
randomizes `readdir` order to flush out ordering-dependence. (Dorc analogue: a test harness that
*randomizes* the iteration order of any collection feeding a decision — the dynamic complement
to the BTreeMap static mandate; see §7.)

**finding-distro-repro-rates (+SURE).** Current reproducibility fractions, with dates:

- **Debian forky/amd64: 95.4%** (36403/38170); unstable/amd64 94.0%; experimental only 67.1%
  [A-debian-reproducible-2026, fetched 2026-06-11]. Debian micronews 2025-05-03: "Debian
  testing/trixie release on amd64 is now reproducible for over 95%" [B-debian-micronews-2025].
- **NixOS GNOME ISO: 95.18%** (4621/4855); minimal ISO historically ~99.77% [A-nixos-r13y-2026 /
  B-r13y-2021]. At nixpkgs scale, a Télécom-Paris study rebuilt 709,816 packages and found
  "between 69 and 91% with an upward trend" bitwise, ">99%" rebuildability [B-malka-nixpkgs-2025].
- **Yocto: 100.00%** (38127/38127) [B-yocto-repro-2026].
- **Arch:** dashboard is JS-rendered; live % not captured (gap).

**finding-rebuilderd (+SURE).** The independent-rebuild verifier, `rebuilderd`
[B-rebuilderd-2026]: "monitors the package repository of a linux distribution and uses rebuilder
backends … to verify the provided binary packages can be reproduced from the given source code
… optionally generates a report of differences with diffoscope." Distinct from Debian's
double-build: rebuilderd compares a fresh rebuild against the *actually-shipped* binary (the
supply-chain-attestation use case). Per-worker cost: "at least 16 GiB RAM … closer to 32 GiB"
for all packages [B-archwiki-rebuilderd-2026]. Used by `reproduce.debian.net` and
`reproducible.archlinux.org`.

**finding-repro-recurring-offenders (+SURE).** The ranked cause data:

- **Timestamps / embedded build dates are #1.** A Java study of "12,803 unreproducible
  artifacts": "Timestamps are the most common cause of unreproducibility in our dataset"
  [B-arxiv-java-2025]. The nixpkgs study: "about 15% of failures are due to embedded build
  dates" [B-rb-report-2025-01].
- **Debian's machine-ranked #1 single issue: "gcc captures build path" — 1,842 packages
  affected** (popcon-summed score 13.8M) [A-debian-issues-2026].
- **Cost/utility perception:** a survey of 17 experts — "reproducible builds had a very high
  utility rating from 58.8% participants, but also a high-cost rating from 70.6%"
  [B-wikipedia-reproducible-2026]. The compute is sponsor-donated (IONOS VMs, OSUOSL, Codethink),
  no published budget (finding-repro-cost-gap).

