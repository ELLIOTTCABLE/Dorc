> QUARANTINED RAW LANE MATERIAL - 24K cross-model language-design crosscheck (2026-07-05).
> NOT adjudicated findings. The adjudicated verdict is Research/notes/24Kc-language-crosscheck-adjudication.md.
> RAW final report: DeepSeek V4-Pro (foreign lineage), ADVERSARIAL stance lane. Un-adjudicated.
> Archived verbatim from the session scratchpad post-compaction, at the human's direction.
> Do not cite as authority; do not read during future crosscheck skill-ups (contaminates pre-registration).

Now I have comprehensive evidence across the corpus. Let me finalize.

---

# FINAL REPORT

## PART 1 — PRE-REGISTERED LESSONS

As fixed 2026-07-05, before any corpus file was opened beyond the task brief. No lessons were added after corpus exposure.

**L1 — Annotations Must Be Additive.** Adding type/metadata annotations must never change runtime behavior, require rewriting existing code, or break execution. Source: Hejlsberg TS Design Goals ("fully erasable, structural type system"); PEP 484.

**L2 — Syntax Is Forever; Defaults Even More So.** Once real code exists against a syntax, it becomes permanent. Source: Eich at Strange Loop 2012; Ilya Sher on Bash's error-continuation default; Ink PL mistakes retrospective.

**L3 — The Strictness Ladder Must Have Discrete, Named Rungs.** Without explicit levels, tightening strictness is a political negotiation. Source: Oils LDP ("Global configuration options are minimized"); Meta Python Typing Survey 2025.

**L4 — One Concept, One Spelling.** Multiple ways to do the same thing cause community fracture and cognitive load. Source: Eich JS regrets; Oils LDP ("syntax and semantics should consistently correspond").

**L5 — Error Handling Is Signature, Not Afterthought.** Must be designed in from the start, especially for a language built around process execution. Source: Ilya Sher on Bash; Go error-handling regrets; Ink PL mistakes.

**L6 — Reserved Namespaces Are Precious; Spend Them Before Users Do.** Every identifier a user writes that the language might later want is a future conflict. Source: Hejlsberg TS Design Goals; langdev.SE on backwards compatibility.

**L7 — The Migration/Compat Story Is Part of the Language Design, Not a Tool.** Tooling and compat boundaries are design decisions, not implementation details. Source: Oils Retrospective 2024; Rust editions with `cargo fix`.

**L8 — Shell's Type Impedance Mismatch Is the Whole Game.** Any type system for shell must reckon with exit codes, byte streams, and filenames as its type universe. Source: Elvish Unique Semantics; HOTOS 2025 shell static analysis paper.

**L9 — Error Messages Shape the Mental Model More Than Specs.** Users learn through feedback, not documentation. Source: Coblenz PLIERS (2021); Python Typing Survey 2025.

**L10 — Every Annotation Syntax You Mint Is a Permanent Tax on Both Authors and Readers.** Source: Python Typing Survey 2025 ("type system usability ... remain significant hurdles"); Hejlsberg "7 learnings."

**L11 — The Two-Audiences Problem Is Real and Sharp.** Shell languages have bifurcated users (interactive vs. script authors). Source: Nushell/Elvish/Oils design docs; Hejlsberg on accommodating workflows.

**L12 — Language Features Compose; Their Interactions Are Where Bugs Live.** Accretion systematically under-invests in interaction testing. Source: Eich on JS parser circular dependencies; Ilya Sher on Bash word-splitting × quoting interaction surface.

---

## PART 2 — FINDINGS

### F1 — The authoring surface is not "just sh"; it's a Dorc dialect requiring a correctness-critical transpiler for off-ramp (instantiates L1, L10, L2)

**Severity:** HIGH · **Confidence:** HIGH

The oracle author must write non-POSIX syntax that cannot be parsed, let alone executed, by any stock shell:

- `.` in function names: `otelcol.is_converged()` — `dash -n` rejects this. Documented in the corpus itself: `spike/crates/syntax/CLAUDE.md:97` "dotted `frobctl.check` → `dash -n` 'Bad function name'."
- `-` in command-name prefixes: `apt-get.is_converged()` — the `-` is also non-POSIX in a function name.
- `pkg : package = "$1"` — inline annotation: `pkg` is parsed as a command name, `:` is its first argument — a stock shell fails at runtime ("command not found").
- `:? otelcol:.v0155` trailing a `dpkg-query` invocation — creates extra args the real command would choke on.

The strip pass (`spike/crates/oracle/src/predict.rs:83-196`) handles all of these: it replaces the entire function name span with `to_funcname_segment(provider) + suffix` (handling both `.` and `-`), strips annotations via `collect_strip_edits`, and deletes bare-mark lines. The implementation IS correct. But:

1. The author writes code they cannot `dash -n` or test without Dorc. This breaks DESIGN.md's floor promise: the off-ramp means "ssh host 'dash -s' < script" — that works for the STRIPPED output (the plan artifact), but NOT for the oracle SOURCE. An oracle author who writes `foobar.is_converged() { ... }` and tries to test it with `dash -n foobar.oracle.sh` gets a syntax error.

2. The strip pass is correctness-critical: a bug in the strip pass silently corrupts the off-ramp artifact. This is a permanent tax on every new annotation type, every new role-function suffix, every new mark form — each requires corresponding strip logic.

3. The commentary claims "the period name is the only dash-rejected form" (`predict.rs:73`), correctly predicting the `-` would also fail is omitted — the code handles both, but the understanding documented in comments is incomplete.

**Kill attempt:** The strip pass works. The `dash -n` gate in `e2e/run.sh` tests it on all rendered artifacts. The implementation handles both `.` and `-` correctly via full-span replacement. The mechanics are sound. BUT — mechanics don't make the authoring surface "just sh." An author writing in the dialect must learn that their source files won't parse under stock shells, must trust the strip pass for off-ramp correctness, and must accept that their authored code is Dorc-dialect, not sh. The design documents' claim "it's all just executable shell-script" (README.md:78) and "stripped, it runs on any POSIX box with no Dorc in sight" (USER_STORY:379-380) is true of the OUTPUT but not the INPUT. This is the exact gap between what the documents say and what the fixtures show.

**Files:** `spike/crates/oracle/src/predict.rs:73-74,83-196` · `spike/crates/oracle/src/lib.rs:205-207` · `spike/crates/syntax/CLAUDE.md:97` · `Research/plans/17N-named-kinds-discipline-and-cooperation.md:35-49` · `spike/e2e/cases/guard23-vouch-gates-elision/package.oracle.sh:6,20`

---

### F2 — The `: ` annotation sigil carries five distinct semantic roles with position-dependent meaning (instantiates L4, L10)

**Severity:** MEDIUM · **Confidence:** HIGH

The `:` character in oracle bodies means different things depending on syntactic position and trailing punctuation:

| Form | Meaning | Position | Example |
|---|---|---|---|
| `name : Kind = value` | Type bind (entity declaration) | Inline in `predict()` body | `pkg : package = "$1"` |
| `cmd : Kind:entity.prop` | Establish mark (this exit code measures this cell) | Trailing a probe command | `dpkg-query ... : package:"$pkg".installed` |
| `cmd : Kind:entity.prop!` | Inverted establish mark | Trailing, with `!` | `dpkg-query ... : package:"$pkg".installed!` |
| `cmd :? Kind:entity.prop` | Observe/read-only mark | Trailing, with `?` | `otelcol --version ... :? otelcol:.v0155` |
| `printf ... : service` | Typed emission (kind annotation on stdout) | Trailing in `reaches()` body | `printf '%s\n' "$1" : service` |

One sigil, five roles. The meaning is determined by: presence/absence of `=` sign, trailing position vs. inline position, `!` or `?` modifiers, and which role-function body it appears in. A learner encountering `: package:"$pkg".installed` must distinguish "this is an establish mark, not a type bind" from position alone.

**Kill attempt:** These are all in oracle-author code only — the admin never sees them. And each form genuinely expresses a different concept (declaring identity, establishing a measurement, observing without establishing, emitting typed output). The overloaded `:` is a deliberate visual-consistency choice: "this thing is a kind-typed element of the annotation layer." It could be argued that having ONE sigil for "annotation territory" with sub-modifiers (`?`, `!`, `=`) is better than five distinct sigils. The individual forms are learnable, and the engineer persona is expected to invest learning effort. But the total semiotic burden is non-trivial, and a wrong reading of one `:` form as another is a genuine bug source.

**Files:** `Research/notes/24G-kind-owner-family-design-round.md:76-81` · `ORACLE_PROVIDES.md:49-111` · `spike/e2e/cases/guard23-vouch-gates-elision/package.oracle.sh:10,13` · `spike/e2e/cases/strawman24-pipe-guard-oracle-converged/otelcol.oracle.sh:12`

---

### F3 — Kind namespace is unregulated; collision between independent authors' vocabulary is undetected and potentially silent (instantiates L6)

**Severity:** MEDIUM (at scale) · **Confidence:** MEDIUM

ORACLE_PROVIDES.md:52-53: "Nobody approves kind names; there is no registry; a kind only has to agree with itself." The `resolve()` mechanism handles "one entity, two names" (synonym resolution within a kind). But there is NO mechanism for "one name, two different concepts." If oracle author A declares `kind: file` meaning "filesystem paths" and author B declares `kind: file` meaning "file descriptors," their oracles silently share a namespace with no detection. The footprint-survival machinery uses kind names for coordinate comparison — a kind-name collision means cross-oracle elision decisions rest on a false premise of shared vocabulary.

The design explicitly acknowledges this: ORACLE_PROVIDES.md:65-67 calls it "the design's residual naked spot, owned by the kind's one author-of-identity" and 17N §F2 says identity "is declared, never inferred." The engineer persona (the kind's author) is intended to own this coordination burden, but there is no tooling to HELP them coordinate — no collision detection, no namespacing convention, no registry.

**Kill attempt:** This is a known, acknowledged limit. Reverse-DNS kind names are mentioned as a convention (`24G §4`: "reverse-DNS kind names typed once per arm") but not enforced. The problem is small at single-author scale and probably tolerable at small-community scale. However, if Dorc achieves its stated goal of a "library of contributed components" (DESIGN.md:14), this becomes the hardest kind of problem: a social/coordination problem disguised as a technical one, with failure modes that are silent (wrong elisions, not loud errors).

**Files:** `ORACLE_PROVIDES.md:49-67` · `Research/plans/17N-named-kinds-discipline-and-cooperation.md:83-93` · `Research/notes/24G-kind-owner-family-design-round.md:82-98`

---

### F4 — Eight-condition corner case for silent under-execution: the design's honesty about complexity is a feature, but the complexity itself is a consequence of accretion (instantiates L12)

**Severity:** MEDIUM · **Confidence:** HIGH

USER_STORY.md:653-679 enumerates eight conditions that must ALL be true before the "bought unsoundness" corner can produce a silent under-execution. The enumeration is admirably honest. But the fact that EIGHT independent conditions must be simultaneously satisfied for wrong behavior is also evidence of the interaction surface's complexity: `--trust-footprints` × `is_converged()` vouch × probe convergence × running mutator wall × at-most footprint claim × footprint-claim correctness × coherence canary miss × unlucky entity overlap.

The accretion pattern is visible in the evolution path:
1. `predict()` alone → elision when converged, walls when diverged
2. +`is_converged()` → guard insertion for downstream sites past unmodeled walls
3. +`touches()` → footprint survival past modeled-but-running walls (the bought unsoundness)
4. +`resolve()` → entity identity normalization for footprint comparison
5. +`reaches()` → footprint expansion across kinds

Each stage added a mechanism that interacts with all previous stages. The 8-condition corner is the interaction product of stages 3–5. Stage 6 ("resolve splits one referent") and stage 7 ("reaches misses an edge") each introduce NEW ways for the corner to manifest. The design's honesty about this in the "bought unsoundness" section is the right posture, but documented complexity is still complexity.

**Kill attempt:** This is inherent to the domain — you can't have the survival tier's attention savings without its trust surface. The design correctly fences the corner behind an opt-in flag and full attribution. The enumeration itself is a mitigation (every stakeholder who reads it knows what they're trusting). But the combinatorial growth in interactions as each new shape is added is the exact failure mode L12 predicts. If a hypothetical `touches()` v2 with per-path claims or a `predict()` with declared stdout values is added, the interaction surface grows again.

**Files:** `USER_STORY.md:641-703` · `ORACLE_PROVIDES.md:113-141` · `KNOBS.md:143-147 (kSURVIVAL)`

---

### F5 — Dual naming conventions in the fixture corpus expose transitional debt with no stated migration path (instantiates L7, L2)

**Severity:** LOW · **Confidence:** HIGH

The 187+ oracle fixture files predominantly use `apt_get__predict()` (double-underscore). The round-24 fixtures introduce `apt-get.is_converged()` (period-separated) alongside the old naming. The design documents (USER_STORY.md, spike/CLAUDE.md) exclusively describe the period convention as settled. This means:

1. Any new oracle author who reads the design documents writes `foobar.is_converged()` — but the fixture corpus they'd copy from mostly shows `foobar__predict()`.
2. The `guard23-vouch-gates-elision/package.oracle.sh` fixture uses BOTH conventions in the SAME file:
   - Line 6: `apt_get__predict()` (old)
   - Line 20: `apt-get.is_converged()` (new)
3. The strip pass handles both (the `name_span` replacement is format-agnostic), but the RENDER output always uses the mangled form (`apt_get__predict`, `apt_get__is_converged`).

This is transitional debt, not a design flaw. But it has a real consequence: the first published oracle library will have to pick a convention, and if the convention changes later (as it already has once), every published oracle must migrate. The design has no stated migration path or compatibility story between dialect versions.

**Kill attempt:** This is pre-1.0, no users, no published oracles. The dual conventions exist only in test fixtures. The strip pass handles both formats. This finding would drop to trivia if a single convention was selected and applied consistently before the first real oracle is published. But the fact that the transition ALREADY happened once and required no migration tooling is evidence that dialect changes are currently managed by bulk fixture edits rather than by design — which is exactly the habit that becomes expensive post-1.0.

**Files:** `spike/e2e/cases/guard23-vouch-gates-elision/package.oracle.sh:6,20` · `spike/e2e/cases/strawman24-pipe-guard-oracle-converged/otelcol.oracle.sh:10,15` · `spike/crates/oracle/src/predict.rs:59,83-116`

---

### F6 — The "two minutes of sh" narrative in USER_STORY understates what a real oracle looks like (instantiates L11)

**Severity:** LOW · **Confidence:** MEDIUM

USER_STORY.md stage 3 claims the minimal oracle is nine lines of sh and "two minutes of work" (line 248, 331). The actual fixture oracles are systematically more involved:

- Every oracle fixture includes a flag-strip loop (`while [ "${1#-}" != "$1" ]; do shift; done`) that the user story's minimal oracle omits. This handles `-y`, `--quiet`, and other flags — without it, the entity resolution is wrong for flag-bearing invocations.
- Every oracle fixture includes a `[ "$2" = "" ]` multi-operand guard (required by the `R2-MULTIOP` regression class in `spike/crates/oracle/CLAUDE.md`). Without it, `apt-get install nginx curl` silently resolves only the first operand and never installs curl.
- The `*) return 2` catch-all and explicit per-verb declines require understanding the exit-status partition and the "decline = ordinary control-flow" idiom.

The actual minimal oracle (counting the `is_converged()` body from `guard23-vouch-gates-elision/package.oracle.sh`) is ~12 lines, not 9 — and more importantly, those extra 3 lines contain the flag-strip and operand-guard that are correctness-critical, not optional polish. The USER_STORY's example omits them for pedagogical clarity, but a new author copying that example would produce a silently-broken oracle.

**Kill attempt:** The USER_STORY is a walkthrough, not a tutorial. It's pedagogically appropriate to show the simplest form first and note that real oracles grow additional guards. The flag-strip idiom is boilerplate that could be factored into a helper (the design mentions this possibility). A published "write your first oracle" guide would presumably include these details. But the gap between the walkthrough's "two minutes" and the fixtures' actual content is evidence of the L11 cliff: the admin→engineer graduation is real, and the first oracle is more work than the marketing suggests.

**Files:** `USER_STORY.md:236-245` · `spike/e2e/cases/guard23-vouch-gates-elision/package.oracle.sh:20-30` · `spike/crates/oracle/CLAUDE.md` (R2-MULTIOP section)

---

### F7 — The plan render's per-line annotation comments are machine-generated shell injected into user-facing output (instantiates L5-adjacent; lesson-independent)

**Severity:** LOW · **Confidence:** MEDIUM

The plan render (visible in expected output files) inserts Dorc-generated commentary into what is otherwise user-authored shell:
```
# apt-get install -y nginx   # dorc: elided (already converged / dead branch)
```

This is "machine-generated shell injected into user-facing output" — one of the precise concerns in the task brief ("machine-generated shell injected into user-facing output"). The comment discipline is specified (rul-attention-honesty: "lines that will execute are never hidden"), and the own-goal scenario (generated shell corrupting syntax) is tested (`observable_matrix.rs` tests: `dash -n` failures from mangled `done` tokens, embedded `'` in disclosure comments flipping quote state).

**Kill attempt:** The `dash -n` gate in the e2e harness catches syntax corruption. The tests in `observable_matrix.rs` specifically test edge cases where machine-generated commentary could break syntax. The design is aware of this risk and has built defenses. The risk is inherently limited: these are comments, not commands. But the OWN-GOAL scenario — a comment containing a `'` that corrupts a quoted string — is real and tested against.

**Files:** `spike/e2e/cases/strawman24-survive-simple/expected.out:41` · `spike/crates/plan/tests/observable_matrix.rs:1550-1611` · `spike/CLAUDE.md:147-149 (rul-attention-honesty)`

---

## PART 3 — CLEARED

### C1 — The error-handling posture is well-designed (L5 cleared)

The fail-toward-run posture ("everything fails toward run"), the explicit exit-status partition (0/1/≥2), the `||` guard pattern ("broken check falls through to execution"), and the `return 2` decline pattern are all coherently designed and consistently applied. The USER_STORY's explicit documentation of the guard's anatomy ("a broken or confused check falls through to running the command") and the structural safety guarantee are well-reasoned.

### C2 — The monotonic-coverage model is a valid alternative to explicit strictness rungs (L3 cleared)

The design commits to monotonicity: every new oracle shape buys value, none removes prior value. ORACLE_PROVIDES.md's opening obligation — "every partial-effort-increase yields vaguely-corresponding value; no partial-effort-increase removes previously-won value" — is the right property. The coverage-gradient model (you get out what you put in) IS the strictness ladder, implemented through the shape system rather than through configuration flags.

### C3 — The kind/entity/selector coordinate system is a reasonable abstraction for shell's type impedance mismatch (L8 cleared)

The three-place `(kind, entity, selector)` coordinate vocabulary maps naturally to shell's observable world: kinds name categories of state (packages, services, files), entities anchor to specific operands (nginx, /etc/hosts), and selectors name specific facts (installed, active, content-match). The `resolve()` and `reaches()` mechanisms pragmatically handle the impedance mismatch within acknowledged limits. The design explicitly embraces rather than hides the mismatch.

### C4 — The `.is_converged`/`.is_diverged` sense-by-name dual is a genuine design feature, not duplication (L4 cleared)

Having two verdict-function names (`is_converged` and `is_diverged`) is NOT "two ways to do the same thing" — it's "one concept, expressed from the sense the author finds natural, with the sense made explicit in the function name." The `is_diverged` glue `( foo_is_diverged args; [ $? -eq 1 ] )` is a mechanical inversion that preserves the function's sense-by-name contract while producing the correct guard semantics. This is thoughtful, not redundant.

### C5 — The `dash -n` compliance for stripped output is verified by implementation (F1 partial mitigation)

The `to_funcname_segment` function (`spike/crates/oracle/src/lib.rs:205-207`) handles `-` → `_` normalization, and `strip_role` replaces the entire function name span with the normalized form. The `dash -n` gate in `e2e/run.sh` tests every rendered artifact. The implementation correctly produces POSIX-valid output from non-POSIX input. The authoring surface remains non-POSIX (F1), but the OUTPUT artifact is clean.

### C6 — The two-audiences design (admin vs. engineer) is explicitly designed for, not an accident

The design names and designs for both audiences throughout: the admin gets implicit guard-lifting from hand-written shell; the engineer gets the role-function family and annotation dialect. The USER_STORY walks one person through both hats. The graduation cliff (F6) is real, but the fact that it's explicitly designed for — with the user story showing the transition — is better than a cliff that wasn't even seen.

### C7 — The "strawman" shield is partially legitimate for spelling, not for structure

The design explicitly flags unsettled spellings (kTYANNOT in KNOBS.md: "de-facto kTYANNOT-inline; the formal weld is human-reserved"; USER_STORY stage 5 FIXME markers). Where my findings touch SPELLING (the `:` annotation sigil overload, the `DORC_` env var namespace), the strawman shield partially applies — these could change before stabilization. Where my findings touch STRUCTURE (the role-function family, the exit-status partition, the strip pass contract, the monotonic-coverage model), these are welded in spike/CLAUDE.md and the strawman shield does NOT apply. Findings F1, F4, F5, F6 are structural; F2, F3 straddle the boundary; C1–C6 are structural-level cleared items.
