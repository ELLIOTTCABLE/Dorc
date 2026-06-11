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

**finding-1 (the gate is mainstream, not exotic) (+SURE).** Run-the-tool-twice-with/without-the-
metadata-and-compare is a *shipped, decades-old* discipline in at least two production compilers:
GCC's `-fcompare-debug` (compile twice, `-gtoggle` between, error if the final RTL dumps differ;
§1) and GCC's `bootstrap-debug` stage2-vs-stage3 object compare (§9). The motivating invariant is
stated identically to Dorc's, by Alexandre Oliva in 2007: "enabling debug information isn't
supposed to modify the executable code in any way whatsoever … If debug information disables any
optimization, that's a bug that needs fixing" [A-gcc-wiki-vta-2007]. LLVM states the same policy
and verifies it with `debugify`/`check-debugify` (§2). reproducible-builds.org generalizes it:
output "MUST … depend only on the source code" [A-rb-source-date-epoch-2017]. Dorc's erasability
gate is a faithful instance of a fundamental pattern — proceed with confidence.

**finding-2 (make it ADVERSARIAL — variance injection — not just run-twice) (+SURE, the key
direction-shaper).** The mature systems do not run the *same* build twice; they inject *maximal
variance* between the two and assert invariance. Debian's tests.reproducible-builds.org varies
~20 axes (hostname, TZ 26h apart, locale, uid/gid, umask, `/bin/sh` dash↔bash, date +398 days,
…; full table §6/§0-table-below) [A-debian-variations-2026]. LLVM ships `-reverse-iterate`
(force worst-case container order) and `llvm::sort`'s pre-shuffle and an *ASLR-derived hash seed*
so order-dependence breaks immediately [B-llvm-discourse-43131-2016, B-maskray-hyrum-2026]. Go
gates *every release* on a Linux-vs-Windows double-build: "must produce bit-for-bit identical
archives or else we do not proceed with the release" [B-go-rebuild-2023]. For Dorc: don't strip
receipts and re-run identically — *populate the stripped run's receipts with adversarial values*
(sentinels, reversed origin-set order, randomized receipt IDs, a different hash seed) and assert
decisions are unchanged. The variance axes that map to an analyzer: receipt-ID assignment order,
origin-set iteration order, `HashMap` seed (for any non-`BTreeMap` survivor), thread/codegen
parallelism if the analyzer parallelizes, and allocation order.

**finding-3 (where gates ROT: silent no-op, not deletion) (+SURE).** The failure mode is not
someone deleting the gate — it's the gate *silently becoming a no-op while showing green*.
Abrahms: a flaky-test auto-quarantine, through a JIRA-tagging fluke, skipped 80% of e2e scenarios
with "the defect rate remained consistent" — an accidental controlled experiment proving the gate
caught nothing [B-abrahms-citheater-2026]. Meiklejohn killed a per-PR proof-gate that "caught
exactly nothing … the plain test jobs wouldn't have caught five minutes later," whose cost grew
with merge activity while its benefit didn't [B-meiklejohn-tax-2026]. GCC pre-empted this with a
*coverage canary*: `GCC_COMPARE_DEBUG=-fcompare-debug-not-overridden` makes the compiler *error
if the gate didn't actually run* [A-gcc-developer-options-2024]. Mandate for Dorc: (a) NO
auto-retry/auto-quarantine on the gate; (b) ship a coverage-canary so "did the gate run AND
assert?" is itself asserted; (c) ensure the gate catches a class *no functional test can* — and
it does, because a receipt-into-decision leak is invisible to decision-only tests, which is
exactly the property that makes it pass Meiklejohn's two-question test where his gate failed.

**finding-4 (the comparison partition is the real design work) (+SURE).** "Identical" is never
raw-byte-identical; every system defines a *partition* of identity-relevant vs exempt output, and
that definition IS the spec. Three distinct partition strategies recur (all three usable for
Dorc): (i) *strip the noise* — GCC's `-fdump-noaddr`/`-fdump-unnumbered` erase addresses and
instruction-numbers before diff ("makes it more feasible to use diff … with and without `-g`")
[A-gcc-developer-options-2024]; (ii) *named sanctioned-absence reasons* — LLVM's four
`DebugLoc::get{CompilerGenerated,Dropped,Unknown,Temporary}()` enumerate *why* a divergence is
legitimate, biased so "an absent location can be detected and fixed, while an incorrectly
annotated instruction is much [harder]" [A-llvm-howtoupdatedebuginfo-2025]; (iii) *canonicalize
the volatile field* — r-b.org's timestamp-clamping rewrites volatile values to a deterministic
function of source rather than exempting them [A-rb-source-date-epoch-2017]. diffoscope
operationalizes (i)+(ii): recursively unpack/normalize each known format, hexdump-fallback only
for unknown ones [A-diffoscope-2026]. See the partition-language recommendation below.

**finding-5 (enforce ordering beyond convention) (+SURE).** Dorc's BTreeMap-everywhere mandate is
*convention*; the prior art mandates it *mechanically*. rustc ships the `potential_query_
instability` lint (fires at the call site of `HashMap` iteration in query code) and a *type-level*
split — `FxIndexMap` (ordered) vs `UnordMap`/`UnordSet` (iteration API *removed* so a leak won't
compile) [A-rustc-fx-src-2026, A-rustc-potential-query-instability-2025]. LLVM ships the
`bugprone-nondeterministic-pointer-iteration-order` clang-tidy check and a coding-standard rule
[A-llvm-coding-2026, B-clang-tidy-nondeterministic-2026]. Recommendation in §0-mechanisms: a Dorc
`Unord*`-style newtype is stronger than convention because the leak becomes a *type error*.

**finding-6 (rustc is the cautionary "no in-tree gate" case) (+SURE).** rustc *can* reproduce but
relies on Debian's *external* checker and has no in-tree CI gate — issue #75362 is still OPEN
since 2020 — and consequently regressed the moment it was achieved (1.44.1 reproducible → 1.45.0
not), with maintainers noting contributors "are not aware of all of the details needed"
[B-rust-issue-75362-2020]. This is the empirical argument for Dorc shipping the gate *in-tree from
day one*: the regression pressure is constant and silent without it.

### §0-table — the leak-category taxonomy (compact, cited)

Each category = one environment/state variable that leaks into the artifact; the fix is either
*normalize-to-constant* or *freeze-into-declared-perimeter*. The Dorc-analogue column maps each to
the receipts-into-decisions risk. (Full verbatim defs+fixes in §8a.)

| leak-category | what leaks | canonical fix | Dorc analogue (receipts→decisions) |
| --- | --- | --- | --- |
| **ordering** (stable-inputs) | filesystem/`readdir`/container iteration order | `LC_ALL=C sort`; explicit list; `--sort=name`; deterministic containers | origin-set / receipt-ID iteration order leaking into a decision — THE central risk; [A-rb-stable-inputs-2026] |
| **timestamps** | current time embedded | `SOURCE_DATE_EPOCH`; clamp; strip | a probe-record timestamp feeding a decision; [A-rb-timestamps-2026] |
| **build-path** | source path in debug info | `-ffile-prefix-map`; remap | a receipt's source-locator string reaching a decision; debugedit war-story: in-place strip fails if presence perturbed ordering; [A-rb-build-path-2026] |
| **locales** | collation/format vary by `LC_*` | `LC_ALL=C` | any locale-sensitive comparison in decision code; [A-rb-locales-2026] |
| **timezones** | tz-dependent rendering | `TZ=UTC` | (subsumed by timestamps); [A-rb-timezones-2026] |
| **value-initialization** | uninitialized memory / struct padding | zero-init; Valgrind | reading uninitialized receipt fields into a decision; [A-rb-value-init-2026] |
| **randomness** | PRNG / hash seed | seed from source; `-frandom-seed` | a `HashMap` seed (non-`BTreeMap` survivor) affecting a decision; [A-rb-randomness-2026] |
| **version-information** | build counter; *abbrev-hash length* | pin from VCS; `--abbrev=12` | a monotonic receipt counter leaking; [A-rb-version-info-2026] |
| **archive-metadata** | file order, uid/gid, mtime, *tar PID* | `tar --sort=name --owner=0 --numeric-owner`; `ar` det-mode | serialization-order/identity of decision output; [A-rb-archives-2026] |
| **volatile-inputs** | network/remote data | checksums; lockfiles; vendoring | (less applicable — Dorc input is the sh + probes) |
| **stripping** | unavoidable metadata (atime, ownership) | `strip-nondeterminism` post-process | normalize residual receipt fields before compare; [A-rb-stripping-2026] |

Cross-cutting (+SURE): **ordering is the most entangled category** — a naive `sort` is itself
locale-dependent, so the fix is always `LC_ALL=C sort`; it recurs in locales, stable-inputs, and
archives. For Dorc (BTreeMap already mandated) the residual ordering risks are: (a) the *order
receipts are assigned/emitted*, (b) any `HashSet`/`HashMap` that escaped the mandate, (c) any
`Vec` built by iterating a non-deterministic source then not sorted.

### §0-mechanisms — recommended enforcement for an analyzer-inertness gate (Rust workspace)

Ranked; effort is a -GUESS in engineer-days for a first cut in a plain Rust workspace (no
proc-macros; build-scripts/tests/lints in scope).

- **mechanism-erasability-gate (the core, model on `-fcompare-debug` + `reprotest`) — effort
  ~3-6d.** A `#[test]` (or `cargo xtask`) that runs the analyzer twice on a fixture corpus: run-A
  with the real receipts plane, run-B with receipts **stripped AND adversarially varied** (see
  finding-2 — reversed origin-set order, sentinel receipt IDs, different `HashMap` seed via
  `RUSTC`/env or a DI'd seed), then asserts the *canonicalized decision output* is byte-identical
  (explanation output excluded by construction — serialize only the decision plane). DST-clean:
  the two runs are pure given fixed inputs + injected seed; clock/net/disk/rand already DI'd per
  AGENTS, so the gate just varies the DI'd seed. Cost ~2× one analyzer run per fixture —
  network-free, cheap. The hard part is *defining the canonical decision serialization* (the
  partition — see below), not the harness. ~SUSPECT this is the single highest-value deliverable.
- **mechanism-coverage-canary (model on `GCC_COMPARE_DEBUG=-fcompare-debug-not-overridden`) —
  effort ~0.5d.** Make the gate *prove it ran*: a sentinel that fails if the comparison was
  skipped (e.g. the gate writes a marker the CI step greps for; or the test panics unless it
  actually performed ≥1 comparison). Pre-empts finding-3 (silent no-op rot). Cheap, do it.
- **mechanism-unord-newtype (model on rustc `UnordMap`/`untracked_query_information`) — effort
  ~2-4d.** A newtype wrapper around decision-internal maps that *does not expose iteration*, only
  `get`/`insert`/`into_sorted_vec` — so "iterate this in decision code" is a *compile error*, not
  a review catch. Optionally a `clippy`/`dylint` lint firing when a decision-producing fn takes a
  receipt-typed argument (the `untracked_query_information` analogue). The newtype is pure-Rust
  and cheap; the custom lint (`dylint`) is the higher-effort, higher-assurance option. Strengthens
  the BTreeMap mandate from convention to type-enforcement (finding-5).
- **mechanism-per-stage-localization (model on `-debugify-each`/`-verify-each`) — effort ~1-2d on
  top of the core gate.** Run the erasability check after each analyzer *stage* (parse → taint →
  elision-decide), not just end-to-end, so a leak is attributed to one stage. Optional;
  defer until the end-to-end gate exists and a leak is hard to localize.
- **mechanism-decision-digest (model on Zephyr's per-build checksum line) — effort ~0.5d.** Emit a
  one-line hash of the canonicalized decision output on *every* analyzer run (not just CI). Cheap
  always-on signal; "accelerates the investigation of temporary reproducibility issues"
  [B-zephyr-50205-2022]; complements the in-CI gate.

### §0-partition-language — the cleanest comparison-partition definition found

The single cleanest definitional vocabulary is **LLVM's named sanctioned-absence reasons**
[A-llvm-howtoupdatedebuginfo-2025]: rather than a boolean "exempt/not-exempt," enumerate the
*specific reasons* a divergence is legitimate (`getCompilerGenerated`, `getDropped`, `getUnknown`,
`getTemporary`), each set *at the point the divergence is created*, with the governing bias:

> the most important rule is to not apply any of these if it isn't clear which, if any, is
> appropriate — an absent location can be detected and fixed, while an incorrectly annotated
> instruction is much [harder].

Translated to Dorc's gate spec, the recommended partition language: the decision output is split
into an **identity plane** (must be byte-identical across the two runs — the verdict per command:
replace/keep/refuse, the chosen substitution, the per-host applicability) and an **exempt plane**
(may differ — explanation text, receipt IDs, origin-set *ordering*, timing, any diagnostic). The
exempt plane is defined by a *closed enumeration of named reasons* (e.g. `Exempt::Explanation`,
`Exempt::ReceiptId`, `Exempt::OriginOrdering`, `Exempt::Timing`), each applied *at the field's
definition site*, and the gate **fails on any field not explicitly assigned an exempt reason** —
the LLVM bias (unknown ⇒ not-exempt ⇒ loud-but-fixable, never silently-exempt-and-leaking). This
beats a strip-list (GCC style) because new decision fields are *included by default* and must be
*deliberately* exempted — the safe direction for a correctness gate. Pair with r-b.org's
*canonicalize-don't-exempt* for fields that legitimately vary but must still be compared (clamp/
normalize them in both runs rather than exempting).

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

---

## §7 ordering-nondeterminism scars & the named deterministic-container types

Dorc already mandates `BTreeMap` everywhere by convention. This section brings back *enforcement
beyond convention*: the named container types other projects mandate, the lints that fire at the
leak site, and — most valuable — the *adversarial* reverse-iteration mechanism that actively
hunts for residual ordering-dependence.

**finding-llvm-ordering-effort (+SURE).** The foundational LLVM statement of the problem
[B-llvm-discourse-43131-2016], verbatim:

> There is non-determinism in LLVM codegen in the following scenarios: 1. Between back-to-back
> runs of the same LLVM toolchain 2. Between Release vs Release+Asserts toolchains 3. Between
> Linux vs Windows toolchains. The main reasons for the non-determinism in codegen are: 1.
> Iteration of unordered containers (like SmallPtrSet, DenseMap, etc) where the iteration order
> is undefined 2. Use of non-stable sorts (like std:sort which uses quicksort) where the
> relative order of elements with the same key is undefined.

**finding-reverse-iterate-adversarial (+SURE).** The mechanism — and this is the key
direction-shaper for Dorc's "make the gate adversarial" question. Verbatim
[B-llvm-discourse-43131-2016]:

> Given a flag (`-mllvm -reverse-iterate`) this patch will enable iteration of SmallPtrSet in
> reverse order. The idea is to compile the same source with and without this flag and expect
> the code to not change. If there is a difference in codegen then it would mean that the
> codegen is sensitive to the iteration order of SmallPtrSet.

This is *exactly* variance-injection applied to ordering: don't just hope iteration order is
irrelevant — *force the worst-case order* and assert the output is unchanged. It is shipped as a
build mode `-DLLVM_REVERSE_ITERATION=ON` and still catching bugs in 2026 (MaskRay documents 2026
fixes triggered by it: MLIR SSA-value completion order, SROA alloca order [B-maskray-hyrum-2026]).
The 2016 patch series broke ~11 real tests (Clang static-analyzer, LoopVectorize, MemorySSA),
each fixed by switching to a deterministic container.

**finding-named-deterministic-types (+SURE).** The concrete types each project mandates — Dorc
should adopt the *naming discipline*, not just "use BTreeMap":

- **LLVM** [B-llvm-progman-2026]: `SetVector` — "the order of iteration is guaranteed to match
  the order of insertion … This property is really important for things like sets of pointers.
  Because pointer values are non-deterministic … iterating over the pointers in the set will not
  be in a well-defined order. … Use it **only** if you need to iterate over the elements in a
  deterministic order." `MapVector` — "iteration order is guaranteed to be the insertion order,
  making it an easy (but somewhat expensive) solution for non-deterministic iteration over maps
  of pointers." Plus `SmallSetVector`/`SmallMapVector`.
- **rustc** [A-rustc-fx-src-2026]: `FxIndexMap`/`FxIndexSet` (indexmap-backed, insertion-
  ordered = deterministic), AND a deliberately order-*suppressing* pair `UnordMap`/`UnordSet`
  ("unordered, so you can't accidentally leak order"). The genesis issue #63713 [B-rust-issue-
  63713-2019] states the design intent verbatim: "It should not provide iteration support, but
  only insert/remove/get/get_mut and conversion to a sorted vec. … This could prevent
  accidentally causing hashmap related non determinism in most cases."

The rustc split is the sharper idea than Dorc's blanket BTreeMap: provide *two* named types —
an ordered one for when you iterate, and an `Unord*` one that *removes the iteration API
entirely* so a leak is a compile error, not a code-review catch. finding-unord-split: ~SUSPECT
Dorc should consider an `Unord*`-style newtype around its decision-internal maps so that
"iterate this in decision code" doesn't typecheck.

**finding-llvm-coding-standard (+SURE).** The codified rule [A-llvm-coding-2026], verbatim
(I confirmed the section headers exist in the live doc; the rule body as gathered):

> **Beware of non-determinism due to ordering of pointers.** In general, there is no relative
> ordering among pointers. As a result, when unordered containers like sets and maps are used
> with pointer keys the iteration order is undefined. Hence, iterating such containers may
> result in non-deterministic code generation. … In case an ordered result is expected, remember
> to sort an unordered container before iteration. Or use ordered containers like
> `vector`/`MapVector`/`SetVector` if you want to iterate pointer keys.
>
> **Beware of non-deterministic sorting order of equal elements.** `std::sort` uses a non-stable
> sorting algorithm … To uncover such instances of non-determinism, LLVM has introduced a new
> `llvm::sort` wrapper function. For an `EXPENSIVE_CHECKS` build this will randomly shuffle the
> container before sorting. Default to using `llvm::sort` instead of `std::sort`.

**finding-llvm-sort-shuffle (+SURE).** `llvm::sort` is itself a variance-injection mechanism: in
checked builds it *pre-shuffles before sorting* to surface any code that depends on the order of
equal elements [A-llvm-coding-2026, B-maskray-hyrum-2026]. The parallel `llvm::stable_sort`
"deliberately does not pre-shuffle; it is the explicit opt-in for code that legitimately needs
ordering of equal elements." This is the comparison-partition idea applied to sorting: stable
vs unstable sort is the "is this ordering identity-relevant?" decision, made explicit at each
call site.

**finding-clang-tidy-check (+SURE).** The rule is *mechanized as a linter*
[B-clang-tidy-nondeterministic-2026]: `bugprone-nondeterministic-pointer-iteration-order` —
"Iteration of a containers of pointers may present the order of different pointers differently
across different runs … This check only detects range-based for loops over unordered sets and
maps. It also detects calls [to] sorting-like algorithms on containers holding pointers." This
is the proof that "ban ordering leaks" can be a *static check*, not just a convention — directly
adoptable idea for Dorc (a clippy/dylint rule; see §0 mechanisms).

**finding-hash-seed-perturbation (+SURE).** LLVM's most aggressive anti-Hyrum move
[B-maskray-hyrum-2026]: the hash seed in `llvm/include/llvm/ADT/Hashing.h` is "non-deterministic
per process (address of a function in LLVMSupport) to prevent having users depend on the
particular hash values." I.e. they *deliberately randomize* the hash seed each run so that any
output depending on hash-order breaks *immediately and locally* rather than mysteriously later.
This is variance-injection as a *default runtime behavior*, not a test mode. (Rust does the same
for `std::HashMap` via random seeds — the very reason rustc's query code can't use it.) Dorc
analogue: if any `HashMap` survives in non-decision code, randomizing its seed in tests forces
any accidental decision-dependence to fail loudly.

---

## §8 the leak-category taxonomy (r-b.org) & cross-toolchain determinism flags

### §8a the reproducible-builds.org taxonomy (one cited row per category)

The unifying frame the site itself states [A-rb-deterministic-systems-2026], verbatim: "Ensure
stable inputs. Ensure stable outputs. Capture as little as possible from the environment." And
the *perimeter* concept [A-rb-perimeter-2026]: reproducibility is defined relative to a declared
*build environment* — some variables are normalized away, others explicitly *frozen into the
declared environment* (legitimately pinned: OS, arch, build path, user, locale, timezone,
`SOURCE_DATE_EPOCH`). This perimeter/normalize split maps one-to-one onto Dorc's decision-plane
(must be invariant) vs explanation-plane (may carry the frozen context). The compact taxonomy is
tabulated in §0; per-category verbatim definitions + fixes:

- **leak-category-timestamps** [A-rb-timestamps-2026]: "Timestamps make the biggest source of
  reproducibility issues." Fix: `SOURCE_DATE_EPOCH`; else post-process (strip-nondeterminism) or
  `libfaketime`. War-story: libfaketime + parallel compilation broke the Tor Browser build.
- **leak-category-timezones** [A-rb-timezones-2026]: build output varies with build timezone.
  Fix: `TZ=UTC LC_ALL=C`. Subtle: zip has no tz field → must unpack in a fixed tz.
- **leak-category-locales** [A-rb-locales-2026]: "tools which output is influenced by the current
  locale". Fix: `LC_ALL=C`. The case-fold sort surprise (`fr_FR` sorts `a B c`, C sorts `B a c`).
- **leak-category-ordering** ("stable inputs") [A-rb-stable-inputs-2026]: "Most filesystems do
  not guarantee that listing files in a directory always results in the same order." Fix: sort
  explicitly — but `LC_ALL=C sort`, because a naive sort is itself locale-dependent. Canonical
  trap: GNU Make `$(wildcard *.c)` → `$(sort $(wildcard *.c))`. THE most cross-cutting category
  (entangled with locales + archives). Adversarial tool: `disorderfs`.
- **leak-category-build-path** [A-rb-build-path-2026]: "Most compilers write the path of the
  source in the debug information." Fix: `-ffile-prefix-map`/`-fdebug-prefix-map`/
  `-fmacro-prefix-map`. War-story (directly relevant to Dorc): `debugedit` "rewrites bytes in
  place. As this does not reorder the hash table of strings, the resulting bytes are still
  depending on the original build path" — i.e. *post-hoc normalization of a leaked field can
  fail if the field's POSITION was itself order-dependent.* This is a sharp warning: stripping a
  receipt value isn't enough if its *presence* perturbed an ordering.
- **leak-category-value-initialization** [A-rb-value-init-2026]: "In languages which don't
  initialize values, this needs to be explicitly done in order to avoid capturing what random
  bytes are in memory." The coreboot fix: `struct … data_hdr = { 0 };`. Detect with Valgrind.
  (Uninitialized struct padding leaking into output — the "garbage from the environment" leak.)
- **leak-category-randomness** [A-rb-randomness-2026]: "Random data will make builds
  unreproducible and must be avoided." Fix: seed the PRNG from source/changelog/VCS; GCC LTO →
  `-frandom-seed`. Admission: "There's no general solutions for [embedded temp paths], better
  fix the code directly."
- **leak-category-version-information** [A-rb-version-info-2026]: build counters / dates embedded
  as version. Surprising sub-case: *abbreviated git-hash length is non-deterministic* (depends on
  total object count, changes with shallow clones) → pin `--abbrev=12`.
- **leak-category-archive-metadata** [A-rb-archives-2026]: "file ordering, users, groups, numeric
  ids, and permissions" captured by archive formats. Fix: `tar --sort=name --mtime=@$SDE
  --owner=0 --group=0 --numeric-owner`; `ar` deterministic mode (`--enable-deterministic-
  archives`, `ARFLAGS=Dcvr`). Surprise: pax-format tar with `POSIXLY_CORRECT` "adds the PID of
  the tar process" (a process-ID leak into an archive).
- **leak-category-volatile-inputs** [A-rb-volatile-inputs-2026]: network inputs are volatile.
  Fix: checksums + backups + lockfiles + vendoring + `snapshot.debian.org`.
- **leak-category-stripping** [A-rb-stripping-2026]: normalize the metadata you can't avoid
  emitting (uid/gid, atime) via `strip-nondeterminism`, "a temporary workaround which should not
  be needed in the long term; upstream software should be reproducible even without using such a
  tool."

**finding-diffoscope-partition (+SURE).** The comparison tool's strategy IS a comparison-
partition design [A-diffoscope-2026], verbatim: "diffoscope tries to get to the bottom of what
makes files or directories different. It will recursively unpack archives of many kinds and
transform various binary formats into more human-readable form to compare them." Plus "Fallback
on hexdump comparison" and "Fuzzy-matching to handle renamings." The design principle: don't
compare raw bytes — *normalize each known format into a canonical/readable representation,
recurse into nested containers, fall back to hex only for unknown formats.* For Dorc: the gate
shouldn't diff raw decision dumps; it should canonicalize the decision output (sorted, receipt-
IDs stripped, into a stable serialization) and diff *that* — and treat an unknown/unnormalized
field as a *loud failure*, not silently hexdump-compared.

### §8b cross-toolchain determinism flags (the same move, elsewhere)

**finding-go-reproducible-default (+SURE).** Go made reproducibility *default* by banning/
normalizing several sources [B-go-rebuild-2023], all verbatim:

- Map/goroutine randomness: "Map iteration and running work in multiple goroutines … both
  introduce randomness in the order that results may be generated. … To make the build
  reproducible, we had to find each of these and sort the relevant list of items before using it
  to generate output."
- Sort-equal ambiguity banned: "the comparison function used must never report two distinct
  elements as equal."
- Timestamps banned; paths via `-trimpath` (default in release builds since Go 1.21); user IDs
  cleared from archives.
- The end-user recipe: "a reproducible build is as simple as compiling with `CGO_ENABLED=0 go
  build -trimpath`."
- **The CI double-build gate (contrast with rustc's missing one):** "we now build all Go
  distributions on both a trusted Linux/x86-64 system and a Windows/x86-64 system. … The two
  systems must produce bit-for-bit identical archives or else we do not proceed with the
  release." Plus a published verifier (`gorebuild`) run nightly. This is variance-injection
  (different OS/arch between the two builds) gating *every release*.

**finding-dotnet-deterministic (+SURE).** Roslyn (C#/VB) `/deterministic` [B-roslyn-det-2026]:
"The C# and VB compilers are fully deterministic when the `/deterministic` option is specified
(this is the default in the .NET SDK). This means that the 'same inputs' will cause the
compilers to produce the 'same outputs' byte for byte." The nondeterminism it removes is named:
"the compiler also depends on the time of day and random numbers for GUIDs." And `/pathmap`:
"can be used to normalize [the full path of source files] between compiles of the same code in
different root directories." (MSVC C++ spells `/DETERMINISTIC` + `/PATHMAP` as *linker* options;
not captured verbatim — minor gap, the C#/Roslyn equivalents above are the on-point ones.)

---

## §9 where compare-gates rot — the war stories (highest-value direction-shaper)

The human flagged this explicitly: find the war story where a compare-gate gets disabled from
flaky-diff fatigue. I found two clean, recent, numeric accounts plus the GCC bootstrap-compare
mechanism and a live cost-debate. These shape *how Dorc's gate must be built to survive*.

**finding-war-tax-on-happy-path (+SURE, the single best account).** Christopher Meiklejohn,
2026-04-21 [B-meiklejohn-tax-2026], built a per-PR gate ("Caucus Permit Gate") that re-ran
allowlisted build/test commands and validated a proof fingerprinted against the working tree —
then *removed it*. Verbatim:

> Over the last hundred CI runs … there were four Caucus Permit Gate failures. Every one of them
> was a 'Proof failed' result… The same commands run in the Build Backend, Unit Tests, and E2E
> Tests jobs, which are separate CI jobs on every PR. In every case, those dedicated jobs also
> failed, for the same reason, on the same PR. … The gate caught exactly nothing in that window
> that the plain test jobs wouldn't have caught five minutes later.

And the cost-scaling argument that maps *directly* onto a run-twice gate:

> Every `git merge origin/main` changes that fingerprint, which invalidates the proof, which
> means the proof has to be regenerated, which means the allowlisted commands have to be re-run
> end to end. … A gate whose cost grows with how much work is happening, and whose benefit does
> not grow at all, is not a guardrail. It is a tax on the branches doing the right thing,
> collected in service of nothing.

The decision rule he distills (verbatim) is the one Dorc's gate must pass:

> Every gate has to answer two questions. Does it catch failures the other checks wouldn't catch?
> And does the cost of passing it, summed over the lifetime of the repo, stay below the cost of
> whatever it's preventing?

For Dorc this is reassuring on *both* axes — but only because of a property the Caucus gate
lacked: the erasability gate catches a failure class *no other check can* (a receipt leaking
into a decision is invisible to functional tests, which only see decisions), and its cost is
~2× one analyzer run (cheap, network-dominated per AGENTS), *not* growing with merge activity.
finding-gate-survives-meiklejohn-test: +SURE the erasability gate passes both questions, which
is exactly why it's worth shipping where the Caucus gate wasn't. The danger to avoid is making
it *fingerprint-coupled to unrelated churn* (the thing that killed Caucus).

**finding-war-ci-theater (+SURE, the flaky-quarantine-rot account).** Justin Abrahms / Thrive
Market, 2026-02-26 [B-abrahms-citheater-2026]. Numbers: "~150 flaky tests and a 76% deploy
success rate … one in four deploys failed. Not because the code was wrong, but because the
pipeline couldn't be trusted." The quarantine-rot accident, verbatim:

> Those flaky test detectors we put in would quarantine the flaky tests and bugs would be
> filed.. but through a fluke of jira tagging conventions, those bugs went unnoticed. By the
> beginning of this year, 80% of scenarios in our e2e test suite were being skipped. The defect
> rate remained consistent. We'd accidentally run a controlled experiment: remove most of the
> tests, measure what happens. The answer was nothing.

The mechanism lesson (verbatim): "Adding retries and delays or adjusting test ordering to solve
race conditions doesn't work … It just adds a new balance point to a wobbly system that can tip
over at any moment." And the emotional driver that keeps dead gates alive: "Even an ineffective
blanket can feel comforting." finding-quarantine-is-the-rot: +SURE the failure mode isn't the
gate being *deleted* — it's the gate being *silently auto-quarantined into a no-op* while
appearing green. Dorc's gate must therefore (a) have NO auto-quarantine/auto-retry path, and (b)
borrow GCC's coverage-canary (finding-compare-debug-coverage-enforcement) so "did the gate
actually run and assert?" is itself checked — otherwise it rots into CI theater.

**finding-gcc-bootstrap-compare (+SURE).** GCC's *own* shipped deterministic-compare gate: it
compiles itself in stage2, recompiles with the stage2 compiler in stage3, and the `.o` outputs
must be bit-identical or the build aborts with "Bootstrap comparison failure!"
[B-gcc-bootstrap-failures-2026]. Real instances are common on toolchain/libc/host mismatches
(LLVM-gcc, Gentoo musl, go/expressions.o). This is the proof that a byte-identity compare gate
*can* be load-bearing and survive for decades in a production compiler — but note it compares
*stripped* objects (the partition: debug info stripped before compare) and is a known source of
build-abort friction on heterogeneous hosts. Combined with GCC's `compare-debug-failure` keyword
carrying **253 tagged bugs** [B-gcc-bugzilla-kw-2026], the evidence is: these gates catch a
large, real, otherwise-invisible bug class, at the cost of recurring friction that must be
actively managed (hence the `bootstrap-debug-lean` vs `-big` vs `-ckovw` knobs balancing
coverage against disk/time).

**finding-war-zephyr-cost-debate (+SURE).** Zephyr RTOS issue #50205 "Verify builds are
reproducible in the CI" [B-zephyr-50205-2022] is a live statement of the per-PR-vs-nightly cost
tradeoff for a double-build gate, verbatim: "reproducible builds were broken for an unknown
amount of time" (gate-rot via *absence*); and the explicit reasoning: "Running this check
against every PR will incur additional computing time … Alternatives: Run the reproducible build
check less frequently, such as nightly. However, this will require a significant bisect effort
to identify the culprit PR … The incremental cost of some additional builds on each PR seems
worth the trouble." They also shipped a *cheap always-on signal* alongside the gate — a single
checksum line per build log — justified as "makes a non-measurable build time difference" and
"accelerates the investigation." finding-cheap-signal-plus-gate: the dual pattern (expensive
gate + cheap always-visible fingerprint) is worth copying — Dorc could emit a one-line
decision-digest on every analyzer run, with the full erasability gate in CI.

**finding-gcc-oliva-design-goal (+SURE, the cross-compiler convergence).** I read the GCC
var-tracking design doc directly [A-gcc-wiki-vta-2007]. Its goals are stated as a clean
*separation invariant* identical to Dorc's: under "Run-time efficiency" — "Stop missing
optimizations for the sake of preserving variable location debug information"; under "Compile-
time efficiency" — "Avoid using additional memory and CPU cycles that would be needed only to
generate debug information when compiling without generating debug information"; and the headline
"enabling debug information isn't supposed to modify the executable code in any way whatsoever.
… If debug information disables any optimization, that's a bug that needs fixing"
[A-gcc-wiki-vta-2007, via gathering]. The doc *also* independently arrives at LLVM's named-
special-value partition: "A special value needs to be specified for each debug annotation
representation that denotes an unavailable variable … because it was completely optimized away
… or because the compiler has been unable to … keep track." Two compilers, two decades apart,
converging on: (1) the metadata-must-not-change-the-artifact invariant, and (2) a small set of
*named sanctioned-absence values* as the comparison partition. That convergence is strong
evidence the pattern is fundamental, not incidental — and that Dorc's gate spec should center on
the same named-exempt-reasons partition.

