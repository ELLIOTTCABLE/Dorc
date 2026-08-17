# 30H — the blind review of the verified core (raw report, filed verbatim)

> Tier: the RAW report of a deliberately-blind Opus review over the dorc-verify /
> minispec flows (2026-08-17). The reviewer was forbidden the design docs, ran
> tooling-only, and mutated nothing. Filed VERBATIM — conductor-unedited, including its
> own preamble and closing line. Adjudication and the repairs it drove: the
> binder-hygiene lane, this same day (its report is on the branch history). Chafe items
> #1 (harness-level blindness) and #2 (`mise tasks` UX) are harness/UX-tier and remain
> in the HUMAN queue.

---

Sanity check done: I re-ran the binder's own coherence test (`the_catalogue_and_the_corpus_agree` — PASS), confirmed `git status --porcelain` is empty (zero mutations, zero commits), and re-read the brief. One heavy lane ran narrowly (`verify:kani -- flat_is_associative`, green in WSL); I declined `verify:lean` for a stated resource reason. Report follows.

---

# 1. CHAFE LOG

**#1 — The blindness protocol is unenforceable in this harness (cost: contaminated the whole experiment, turn 1).**
My first two tool calls were `ls` + the mandated quarantine read. The harness responded by auto-injecting `AGENTS.md` and the root `CLAUDE.md` as a system-reminder. Later, reading `minispec/REPORT.md` injected `minispec/CLAUDE.md`; reading `spike/verify/src/badge.rs` injected the whole of `spike/CLAUDE.md` (~600 lines of design law). I did not open any of them. Directory-scoped `CLAUDE.md` auto-load fires on file reads, so "don't read the docs" cannot be honoured by an agent's own discipline. **Everything below is grounded in code and tool output, which I checked deliberately because I could no longer claim ignorance.** A future blind run needs the auto-load disabled at the harness level, not by instruction.

**#2 — `mise tasks` is ~40 entries of paragraph-length prose, alphabetical, with no entry point (cost: ~10 min of reading before I knew where verification lived).**
The one task actually named for newcomers — `hello`, "First run in a fresh clone" — sorts between `gate:quick-quiet` and `hk-install`. Several descriptions are 6+ lines of caveat (`bless` is ~10). I expected a grouped or tiered listing; I got a flat wall where the `verify:*` family is discoverable only by reading to the bottom. There is no single `verify` task, so "what is verified?" has no command to ask it.

**#3 — `verify:report` prints a clean 70-line report, then fails on line 71 (cost: one wrong conclusion, ~2 attempts).**
```
$ mise run verify:report 2>&1 | head -40      # my first attempt
... 40 lines of perfectly healthy report ...
EXIT=0                                         # this is head's exit, not the task's
```
I concluded "the report is current." It is not:
```
$ mise run verify:report > rep.txt 2>&1; echo "EXIT=$?"
EXIT=1
$ tail -2 rep.txt
FAIL  minispec/REPORT.md is stale — re-run with --write and review the diff
```
Filtering a 70-line dump is the natural move, and the failure is the last line. I own the mistake (the repo has a rule against it), but a check whose verdict is buried under its own artifact invites exactly this.

**#4 — `minispec/REPORT.md` is permanently stale, on every machine, for three unrelated reasons, and the error text names none of them (cost: ~15 min diffing to find out why).**
```
$ diff minispec/REPORT.md rep.txt
< - statement: `C:\...\worktrees\r30-conduct\minispec\Minispec\JoinIsAssociative.lean` (3726 bytes)
> - statement: `C:\...\worktrees\agent-ac6923ea4501b17fe\minispec\...` (4592 bytes)
< | pinned | todo | absent(seam-kani-pairing-unbuilt: no paired harness) |
> | pinned | todo | not-recomputed-here |
```
(a) `report.rs` writes `u.path.display()` — an **absolute** path — into a file that is then **byte-compared**. The committed copy names a different worktree (`r30-conduct`), so it can never match anywhere else. A commit titled *"fix the stale paths"* (47ce486f) already tried to fix this class once.
(b) The `.lean` statements grew after the last report write (3726 → 4592 bytes; `d541c5fa`, 2026-08-16, is later than the last REPORT.md commit).
(c) The committed `pinned` cell reads `absent(seam-kani-pairing-unbuilt: no paired harness)`. `grep -rn "seam-kani-pairing-unbuilt"` over the whole tree returns **zero hits in any `.rs`/`.toml`** — the current binary cannot produce that string. The committed artifact is output from a deleted code path.
The message says only "stale — re-run with `--write`". I expected it to say *what* diverged; it made me reconstruct all three by hand.

**#5 — Nothing gates the report, so #4 went unnoticed (cost: the finding is only interesting because of this).**
```
$ grep -rn "REPORT.md" spike/ --include=*.rs      # only inside dorc-verify.rs itself
$ mise run test -- the_catalogue_and_the_corpus_agree
        PASS [   0.116s] dorc-verify::corpus_is_coherent the_catalogue_and_the_corpus_agree
```
The suite is green while the coverage report is stale. No test asserts freshness; `verify:report` appears in no gate. The one reader-facing summary of what is verified is decorative.

**#6 — The documented way to add a law points at a command that does not exist (cost: dead end, ~10 min).**
`catalogue_lock.rs` line 1: *"@generated by `dorc-verify promote` — do not edit by hand."* `check.rs` failure text: *"is not in the catalogue (promote it, or delete the unit)"*. But `main()` in `dorc-verify.rs` matches only `check | report | materialize | lean-build | kani`, and its usage line confirms it:
```
tasks: check, report [--write] [--with-lean] [--with-kani], materialize, lean-build, kani [<harness>]
```
`grep -c "verify:promote" mise.toml` → `0`. So the only route to adding a law is hand-editing the file that says not to. (I did not attempt it — read-and-run only. The classifier also blocked my `cargo run -- promote` probe, so this rests on source reading, which is decisive enough.)

**#7 — The gate everyone runs cannot falsify the only two badges that claim coverage (cost: none to find, high to interpret).**
`Evidence::NotAtThisTier.agrees_with(_) == true` for every expectation. At `Tier::Cheap`, `elaborated` and `interrogated` both return `NotAtThisTier`. Those are the two badges the catalogue promotes as `Earned` for all three laws. The code is candid about it ("a tier that did not look cannot contradict"), and a test pins the semantics deliberately. The consequence stands anyway: **the cheap gate confirms the corpus's shape, never its coverage claims.** Confirming them requires `verify:lean`, which is opt-in, Linux/WSL-only, and in no gate. Also worth noting: `gate:quick-quiet` runs `--lib --bins`, which excludes the binder's integration tests entirely — only `gate:full*` reaches them.

**#8 — The lane that earns those badges costs a multi-GB fetch, per worktree (cost: I declined to run it).**
`lean_build_root()` keys on the worktree directory name; I measured an existing sibling root at **7.7 GB**, and `lakefile.toml` pulls aeneas from git (mathlib transitively). A fresh worktree therefore re-downloads gigabytes before the first `elaborated` badge can be checked. `DORC_MINISPEC_BUILD_ROOT` overrides it, but that name appears only in `verify/src/lib.rs` — not in the task description, not in `mise tasks`. I stopped here rather than spend the network, so **"does the Lean actually build?" is the one thing I could not determine.**

**#9 — Kani, by contrast, was frictionless and honest (no chafe; recorded because it's the counterexample).**
```
# Windows
$ mise run verify:kani
preflight kani: not applicable here — this lane is Linux/WSL only
dorc-verify kani: the Kani lane is Linux/WSL only (upstream publishes no Windows asset); run it from the WSL leg
EXIT=2
# WSL
$ mise run verify:kani -- flat_is_associative
preflight kani: ok — disk 838.5 GiB free (needs 2.0 GiB, warm), ram 13.8 GiB free (needs 8.0 GiB)
    5.26s  green        flat_is_associative
kani: 1 green, 0 failed, 0 over budget, of 107 harness(es)
$ mise run verify:kani -- no_such_harness_exists
dorc-verify kani: no harness named `no_such_harness_exists` — the toolchain lists 107
EXIT=2
```
It names its denominator ("of 107"), separates *lane could not run* (2) from *finding* (1), refuses a bogus name against the real harness list, and the preflight prints its actual numbers. This is the standard the rest of the surface does not meet.

**#10 — The two halves of the verified core are not connected to each other.**
All three rows in `catalogue_lock.rs` carry `harness: None`, so every report reads `pinned | todo | absent(no paired harness)`. Meanwhile `lattice_laws.rs` contains `flat_is_associative`, and `flat_obeys_the_binary_laws` (which asserts `a.join(&a) == a` and `a.join(&b) == b.join(&a)`) — i.e. Kani exhaustively verifies the *same three join laws* over `Flat<u8>`. A reader trusting the report concludes nothing pins these laws. Something does; the catalogue just doesn't say so.

**#11 — Nothing binds the committed derived Lean to the current Rust.**
`spike/verify/aeneas/src/lib.rs` `#[path]`-includes the real `crates/analysis/src/lattice.rs` and `crates/core/src/sorted.rs` — genuinely strong, the translated thing *is* the shipped thing. But `grep -rn "sha\|digest\|hash\|fingerprint" spike/verify/src/` finds no source digest anywhere. Edit `Flat::join` in Rust and: cheap gate green, `lake build` (if run) still green against the **old** derived definition, report unchanged. The drift alarm is "somebody re-runs `verify:translate` on Linux and reads the diff" — a human habit, not a mechanism.

**#12 — The seat citation, the anchor between a law and the code it's about, is a line-grep.**
`seat::resolve` splits `dorc_analysis::lattice::Lattice::join`, opens `spike/crates/analysis/src/lattice.rs`, and looks for a line with the tokens `fn` then `join`. `lattice.rs` contains **five** `fn join` declarations (`Flat`, `Powerset`, `MapL`, `Product`, the trait). Any of them satisfies the citation. Nothing checks that the Lean statement is about the same one.

**#13 — `interrogated` is earned by naming, not by content.**
`unit.rs`: `has_nonvacuity_probe = text.contains("theorem {slug}_nonvacuous")`; `battery_entries` counts lines starting with `example`/`#guard`; `Statement::Stated = text.contains("def {slug} : Prop")`. Combined with a green `lake build`, that means the named theorem *typechecks* — real, but `theorem JoinIsIdempotent_nonvacuous : True := trivial` earns it. The badge's own doc-comment promises far more: *"at least one positive witness with the precondition genuinely satisfied."* The units do hold themselves to that (each has a `_specializes_at_u32` coupling theorem tying the battery to the law's `Prop`) — but `grep -rn "specializes" spike/verify/src/` returns nothing, so **the coupling is a convention, not a requirement.**

**#14 — From the Rust side you cannot tell verified code from unverified code.**
`grep -i "minispec" spike/crates/analysis/src/lattice.rs` → nothing. The only in-source hint is a `#[cfg(kani)] mod kani_support`. `seat.rs` lists "the rustdoc backlink" as one of three seat consumers and says only resolution is built. So the boundary is discoverable exclusively from a generated file that is currently stale (#4) and ungated (#5).

**#15 — Minor tooling friction.** The worktree-isolation guard rejected two compound commands containing redirects as "too complex to verify" and told me to re-run them "without the redirect", which was the whole point of the command. Cost: two rewrites.

---

# 2. Honest answer to the premise

**"Parts of the core are formally verified and machine-checked" is defensible for one narrow claim and misleading for the one the words evoke.**

What actually exists, in descending order of what I'd trust:

**Trust it: the Kani lane.** 107 harnesses over `analysis::lattice`. `Flat<u8>`'s `Arbitrary` yields exactly ⊥ / `Elem(any u8)` / ⊤, so `flat_is_associative` and `flat_obeys_the_binary_laws` are **genuinely exhaustive over the entire lattice**, not sampled — that is real machine-checked verification of associativity, commutativity, idempotence, absorption, the ⊥/⊤ identities, and the ⊑/⊓ agreement, for `Flat`, `Product`, `May`, `Must`. The collection-shaped combinators (`Powerset`, `MapL`, mixed products) are verified **only at 0 and 1 members**, and the file states plainly, unprompted, that associativity over them is *unjudged at any size* because it composes two merges. That kind of volunteered limitation is what makes me believe the rest of it. The driver is disciplined too: one harness at a time, per-harness wall-clock budget, CBMC reaped between, and a budget kill classified as a **finding** rather than something to wait out.

**Trust it with a caveat: the Rust→Lean derivation.** The translation unit `#[path]`-includes the actual shipping sources, so `Generated/` is derived from real code, and the strict pipeline refuses rather than emitting a silent `sorry` (hole census: 0; 13 named external axioms, each fenced with a stated reason in committed TOML). The caveat is #11: nothing detects that the committed derivation has gone stale relative to the Rust.

**Do not read as verification: the Lean law corpus.** All three catalogued laws carry `proof: None`. Nothing is proved. What is machine-checked per law is: the statement elaborates against the derived definitions, and roughly three concrete `rfl` evaluations at `u32` (⊥⊔⊥, ⊤⊔⊤, 7⊔7). That is **unit tests written in Lean and executed by the kernel** — worth having, because it proves the derived definition computes what the authors think it does — but it is not a proof of any universally-quantified law. The generic statements additionally rest on two named hypotheses (`LawfulClone`, `LawfulEq`) which the translation cannot supply and which are assumed by name.

**The scope is three laws about `join` on a flat lattice.** Not "the core." The report's own "verified boundary" section lists exactly one seat: `dorc_analysis::lattice::Lattice::join`.

**Credit where due, and it is substantial:** the report renders *every* badge for *every* law including the absent ones, distinguishes "looked and found nothing" from "did not look" as separate values in the type system, refuses badge disagreement in both directions (rot *and* over-claim), forbids `--write --with-lean` so an engine-tier verdict can never be committed as if the cheap gate had confirmed it, and defines a `kill-tested` badge that renders `todo` purely so the absence nags. Somebody deliberately built this to resist exactly the over-claim my premise made. The system's *design* is more honest than my brief was.

**What undercuts it in practice:** that honest report is stale everywhere (#4), ungated (#5), unreproducible by construction (absolute paths), contains an evidence string from deleted code, and the two badges it marks `earned` are unverifiable by any gate anyone routinely runs (#7, #8). The rigour lives in the design and the Kani lane; the *reporting* layer that a newcomer would use to assess trust has quietly rotted.

**What I could not determine blind:** whether `lake build` over `minispec/` is currently green — the multi-GB per-worktree cost put it out of scope for this run (#8). Since `elaborated` and `interrogated` are exactly the badges that depend on it, and the cheap gate cannot contradict them, **every "earned" badge in this repo is currently unconfirmed by anything I could run.** I also could not test the add-a-law flow end-to-end without planting files, so #6 rests on source reading rather than a live dead-end.

---

# 3. The three observations that would most have improved my first hour

1. **`verify:report`'s "stale" message told me nothing about what was stale** — and it was stale for three independent reasons at once (a foreign worktree's absolute path, changed statement byte-counts, and an evidence string no longer in the codebase). Reconstructing that took a diff, a `git log`, and a tree-wide grep. The one artifact whose whole job is telling a reader what to trust is the one artifact that cannot say why it distrusts itself.

2. **Nothing on the Rust side told me `Flat::join` was a verified seat** — and the generated file that would have told me is stale and ungated, so the honest report I eventually found was also a report I had no reason to believe. I learned what was verified by reading `lattice_laws.rs` and `catalogue_lock.rs` directly, which is the opposite of the intended flow, and it is how I found #10: the Kani harnesses verify the three catalogued laws, and the catalogue records `harness: None` for all three.

3. **Both places that told me how to add a law named a command that does not exist** — the generated header (`@generated by dorc-verify promote — do not edit by hand`) and the gate's own failure text (`promote it, or delete the unit`). The binary's usage line lists five subcommands and `promote` is not among them. The one flow the system most wants to be ceremonious about is the one with no ceremony available.

result: Blind review of Dorc's "verified core" complete — Kani exhaustively verifies the lattice join laws over `Flat<u8>` and the Rust→Lean derivation is genuine, but zero Lean proofs exist (all laws `proof: none`), `minispec/REPORT.md` is permanently stale/ungated with an evidence string from deleted code, the cheap gate structurally cannot falsify the only two `earned` badges, and the documented `dorc-verify promote` authoring path does not exist; 15 chafe items logged, zero repo mutations.
