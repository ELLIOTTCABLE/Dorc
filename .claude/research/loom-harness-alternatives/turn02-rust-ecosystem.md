# Turn 2 — Real-world Rust CLI/e2e testing practice, and errorloom positioning

Continuation of `synthesis.md` (turn 1, left as-is). Turn 1 surveyed the tool
landscape against the harness's needs profile; this turn asks what real Rust
projects actually reach for, and uses that to position the in-house
`errorloom` crate (txtar+frontmatter case files, replay sections, word-level
prose-edit transport into generated catalogs, bless orchestration, controlled
subprocess runner) heading toward publication.

## Graded sources

- [A-cargoTestSupport-docs-2026] doc.rust-lang.org nightly-rustc `cargo_test_support` crate docs, incl. its `snapbox` submodule. A (primary, generated from cargo's actual source).
- [A-uiTest-docs-2026] docs.rs/ui_test — primary crate docs (annotation syntax, `harness=false` integration). A.
- [A-compilerTeam-uiTest-issue-2026] github.com/rust-lang/compiler-team issue #536 "Rewrite compiletest out of tree" — primary team discussion confirming Miri/Clippy adoption of ui_test and the extraction rationale. A.
- [A-libRs-trycmd-2026] lib.rs/crates/trycmd — reverse-dep count, download graph, named "Users" list (typos, cargo-edit, clap). A (lib.rs aggregates crates.io registry data; "Users" list is author-curated, treat names as confirmed adopters, list as non-exhaustive).
- [A-libRs-assertCmd-2026] / [A-libRs-snapbox-2026] / [A-libRs-insta-2026] / [A-libRs-instaCmd-2026] — same source class, each crate's reverse-dep counts and monthly downloads. A.
- [A-cratesIo-revdeps-2026] crates.io API `reverse_dependencies` endpoints for trycmd/assert_cmd (raw JSON, named dependent crates e.g. cargo-sweep, stringmetrics). A.
- [A-rustcDevGuide-uiTests-2026] rustc-dev-guide.rust-lang.org "UI tests" + "Compiletest" pages. A (primary maintainer docs).
- [B-cliTestDir-docs-2026] docs.rs/cli_test_dir — states explicitly it packages the "workdir" pattern used by BurntSushi's xsv and ripgrep. B (primary docs, but crate itself is a third-party repackaging, not confirmed still in use by ripgrep today this pass).
- [B-ripgrepReadme-2026] github.com/burntsushi/ripgrep README testing section (Kagi snippet only, not fully fetched). B.
- [B-rustAnalyzer-testing-deepwiki-2026] deepwiki.com/rust-lang/rust-analyzer "Testing Strategy" (minicore fixture, test-utils crate). B (DeepWiki secondary synthesis of primary source, not the source itself — treat structure as indicative, not verbatim-verified).
- [B-alexwlchan-assertCmd-2026] alexwlchan.net "How I test Rust command-line apps with assert_cmd" (Jan 2025) — practitioner blog, corroborates assert_cmd as the default reach-for tool outside big projects. B.
- [B-0xpoe-trycmd-2026] 0xpoe.dev "How to use trycmd to test your Rust CLI?" — practitioner walkthrough referencing clap's own trycmd usage. B.
- [C-kobzol-blog-2026] kobzol.github.io "Just write a test for it" (Mar 2025) — tangential, rust-lang test-infra contributor's general testing philosophy post, used only as a maintenance-culture signal, not fetched in full. C.
- [C-rustBlog-testInfra-2026] blog.rust-lang.org "This Month in Our Test Infra" — snippet only, corroborates rustc's test infra is actively staffed/maintained. C.
- [C-nushellTesting-2026] nushell.sh/book/testing.html + community nutest/nu-test/nuUnit repos — Nushell-*script* testing (not the nushell Rust binary's own integration tests); scope mismatch noted below, weight accordingly. C.
- [C-literateProgramming-signals-2026] assorted: github.com/tlehman/litprog-skill, general "edit preservation" AI-doc-generator repos surfaced by a Kagi pass on "preserve human edits regenerate". C (used only to check the prose-transport competitor claim, not characterized deeply).

## What real Rust projects actually use

**assert_cmd is the ecosystem's default, by roughly an order of magnitude.**
[A-libRs-assertCmd-2026]: 4,791 reverse dependencies (4,727 direct), 4.67M
downloads/month, ranked #6 in lib.rs's Testing category. Compare trycmd (178
reverse deps, 450K/month, #816) and snapbox (311 reverse deps, 874K/month,
#62) [A-libRs-trycmd-2026] [A-libRs-snapbox-2026]. Practitioner writing
independently corroborates this — a 2025 blog post titled exactly "How I test
Rust command-line apps with assert_cmd" treats it as the unmarked default
choice, not a deliberated pick among alternatives [B-alexwlchan-assertCmd-2026].
assert_cmd is a thin `Command` builder + assertion layer, no case-file
format, no golden-bless workflow — it composes with whatever fixture/golden
system the project layers on top (often none at all; many projects just
assert on substrings/predicates per test function).

**insta (general snapshot testing, not CLI-specific) is even bigger**: 4,251
reverse deps, 7.27M downloads/month, #1 in Testing [A-libRs-insta-2026]. Its
CLI-specific extension, insta-cmd, is explicitly labeled "experimental" by
its own author and sits at only 52 reverse deps, 368K/month
[A-libRs-instaCmd-2026] — i.e. insta's dominance is driven by general
value-snapshotting (`assert_debug_snapshot!` etc.), and that dominance does
*not* carry over to the CLI-subprocess-snapshotting niche. insta's headline
differentiator that *does* transfer is `cargo insta review`'s interactive
accept/reject/skip TUI — already flagged in turn 1 as the sharpest bless-flow
UX in the survey.

**trycmd/snapbox (assert-rs org) are real but niche**, concentrated among
CLI-parser-adjacent and dev-tooling crates: named adopters are typos (spell
checker), cargo-edit, and clap itself (for testing its own doc examples)
[A-libRs-trycmd-2026]. This is a tight cluster around one maintainer
(`epage`, who owns both trycmd/snapbox and co-maintains clap/assert_cmd via
the rust-cli org) rather than broad independent convergence — worth weighting
down slightly versus raw download counts, since a chunk of the ecosystem
reach is one person's own projects using each other.

**cargo itself is the strongest single data point for the turn-1 recommendation.**
Cargo's `cargo_test_support` crate — its internal, decades-old, hand-rolled
test harness (bespoke `Project`/`ProjectBuilder`/`ProcessBuilder`/`Execs`
types, fake registries, git/Docker test support) — has a `snapbox` submodule
directly in its public module tree [A-cargoTestSupport-docs-2026]. That is:
cargo rolled its own harness because its domain-specific needs (building
throwaway Cargo projects, fake crates.io registries, cross-compilation
matrices, Docker-container tests) are not something any generic CLI-testing
crate could serve — then, once snapbox matured as a diffing/comparison
toolbox, cargo adopted *that one layer* rather than continuing to hand-roll
string comparison and diff rendering. This is close to a direct precedent for
turn 1's recommendation #2 (wrap snapbox for the diffing engine only, keep
the rest in-house) — it is what the reference Rust CLI tool itself already
does.

**rustc did the analogous thing at the compiler-testing layer, one step
further along.** `compiletest` [A-rustcDevGuide-uiTests-2026] is rustc's own
decades-old hand-rolled harness (thousands of tests, `//~`-style inline
annotations, revisions, target/bitwidth conditionals). Miri needed the same
shape of tool but didn't want to depend on compiletest (an in-tree,
not-really-standalone tool), so it built `ui_test`
[A-uiTest-docs-2026] as an extracted, standalone, `cargo test`-integrated
(`harness=false`) crate — then Clippy adopted it too, and there was an open
rust-lang/compiler-team proposal to rewrite compiletest itself on top of it
[A-compilerTeam-uiTest-issue-2026]. ui_test's design is notable for this
project specifically: it is annotation-based (`//~`, `//@`) rather than
txtar-based, but it has independently grown several of the "forward features"
flagged in turn 1 — per-test `//@ignore-*`/`only-*` conditions (host/target/
bitwidth), `//@revisions` (a lightweight env/config matrix), and `.stdin`
sidecar files — none of which trycmd/snapbox/insta have. **This is the
closest Rust-native precedent for "an in-house test harness extracted into a
publishable, `cargo test`-integrated crate, then adopted by sibling
projects"** — i.e. it's the trajectory errorloom is already on, previously
walked by exactly the kind of high-standards Rust tooling this project wants
to be judged against.

**ripgrep/xsv (BurntSushi) predate assert_cmd's dominance and rolled their
own "workdir" pattern**, later packaged third-party as `cli_test_dir`
[B-cliTestDir-docs-2026]. This is a smaller, mostly-historical precedent:
unlike ui_test, `cli_test_dir` did not become a broad convergence point (no
reverse-dep count was found suggesting wide adoption), suggesting the
ecosystem's actual convergence, once assert_cmd matured, was onto assert_cmd
rather than onto the repackaged ripgrep pattern. Read this as a caution
against extracting too early/too narrowly — the pattern that "won" ecosystem-
wide wasn't the first extraction, it was the one with the broadest, most
general API surface (assert_cmd) plus critical-mass maintainer backing
(rust-cli org).

**rust-analyzer's testing doesn't fit the CLI-golden-file model at all**
[B-rustAnalyzer-testing-deepwiki-2026]: it's an LSP server, tested via
synthetic LSP request/response fixtures and a hand-built `minicore` (a
minimal stdlib stand-in, for fast compilation of test fixtures), through a
`test-utils` crate — a fully bespoke fixture system with no golden-CLI-tool
overlap. Useful mainly as a boundary case: not every "large, serious Rust
project's test infra" is evidence for or against CLI-testing-crate adoption;
some domains (language servers) are structurally CLI-shaped)-adjacent but
not CLI-shaped enough for this survey to say anything about.

**Nushell scope mismatch, noted and set aside**: the several community test
runners found (nutest, nu-test, nuUnit) [C-nushellTesting-2026] test *user
Nushell scripts*, not the `nushell` Rust binary's own integration tests —
wrong layer for this question, not pursued further.

## Positioning: what should errorloom actually ship

Given the adoption data above, the shape of a maximally-useful,
minimally-bloated published errorloom:

**Cede to the ecosystem, integrate/compose instead of reimplementing:**
- *Subprocess execution/assertion primitives.* assert_cmd is the de facto
  standard at 20-something× trycmd/snapbox's reach — if errorloom's
  "controlled subprocess runner" is doing plain `Command`-wrapping-and-
  asserting (as opposed to the PATH-shim-mock-stub half, which is genuinely
  underserved, see turn 1), that's exactly the territory assert_cmd already
  owns; reimplementing it invites the "why not just use assert_cmd" question
  every reviewer will ask first, since it's now essentially unmarked-default
  knowledge in the ecosystem [B-alexwlchan-assertCmd-2026].
  Whether errorloom depends on assert_cmd directly, or simply is
  *interoperable* with it (exposing its subprocess layer as swappable, or
  documenting migration from assert_cmd), the download/reverse-dep gap means
  a large fraction of prospective adopters already have assert_cmd-shaped
  tests they'd want to bring across, not rewrite.
- *Byte-diffing/golden-comparison engine.* Turn 1 already recommended
  wrapping snapbox for this; this turn's cargo precedent
  [A-cargoTestSupport-docs-2026] makes it a stronger recommendation, not a
  weaker one — the highest-scrutiny Rust CLI tool in existence made the same
  call.
- *General value/data snapshotting* (as opposed to CLI-subprocess output) —
  insta already dominates this by download count; errorloom shouldn't grow a
  generic "snapshot any Rust value" feature, that's solved territory with
  4,251 dependents already depending on someone else solving it.

**Keep in-house — genuinely underserved, per both turns:**
- *PATH-shim mock-stub generation with argv-recording and plan verification.*
  Turn 1's core finding stands: nothing in the Rust ecosystem does this
  (assert_cmd explicitly punts to rexpect for interactive programs and to
  nothing at all for command-mocking; ui_test doesn't touch subprocesses this
  way; cargo's own fake-registry/git-test infra is domain-specific to cargo,
  not a generalizable stub mechanism). This is errorloom's most defensible
  differentiator among Rust-native tools specifically.
- *txtar+frontmatter case-file format itself.* No Rust tool in either turn
  uses txtar; the closest analog (Go's testscript) is Go-only and, per turn
  1, shows some staleness signal. A Rust-native txtar-case-file reader with
  frontmatter metadata and inline replay sections is novel in the Rust
  ecosystem specifically — trycmd's `.toml`+sidecar-file format and cram-
  style `.t` prose-transcript format are the two nearest ideas, neither
  matches txtar's multi-file-archive-in-one-file shape.
- *Bless orchestration with per-case review*, following cargo-insta's
  interactive-TUI precedent [B-cargoTestSupport... / turn-1 cargo-insta
  finding] rather than trycmd/snapbox's blunter `OVERWRITE=1` env-var
  rewrite-everything model — this is a place to *exceed* what the dominant
  tools do, not merely match them, since only insta (not trycmd/snapbox) has
  invested in review UX.
- *Forward features ui_test already validated as worth having*: revision-style
  matrix expansion and target/host-conditional case-skipping are the two
  concretely proven-valuable (by rustc/Miri/Clippy's own usage) features
  neither trycmd nor snapbox has bothered with — if errorloom wants to
  future-proof against the turn-1 "treadmill", these two are validated asks
  from adjacent, highly-scrutinized Rust tooling, not speculative.

## The prose-transport claim, checked

The brief's specific claim — "word-level prose-edit transport back into
generated catalogs has no known competitor" — was checked two ways.

Within the CLI/e2e-testing-tool survey (both turns, ~20 tools): **confirmed,
no competitor.** trycmd's README-pull-in is one-directional (test data flows
*into* docs via mdbook, nothing flows back); insta's review TUI operates at
whole-snapshot accept/reject granularity, not word-level prose merge into a
separately-maintained generated catalog; ui_test, ShellSpec, cmd-mox, and
every other tool surveyed have no catalog/prose-generation concept at all —
they compare bytes to bytes, not prose to a structured source-of-truth that
then needs re-rendering.

Widening the search to adjacent domains (docs generators, literate
programming) surfaced structurally similar but domain-distinct problems: the
"tangle/reverse-sync" pattern in literate-programming tools (org-babel,
noweb, and an emergent Claude-skill-era tool, `litprog-skill`, doing
hook-driven reverse-sync of hand-edited generated source *back* into a
`.lit.md` literate document [C-literateProgramming-signals-2026]) is the
closest conceptual cousin — it solves "generated artifact gets hand-edited,
edits must flow back to the authoritative generator input" in the
source-code-tangle domain. Several "AI documentation generator, preserve
human edits on regeneration" projects surfaced in the same search explicitly
flag this as **unsolved** ("Edit preservation - Will overwrite human changes
(not implemented)"), which is corroborating evidence this is a genuinely hard,
not-yet-commoditized problem class generally, not just within CLI testing.

Confidence: high that no CLI-testing-tool competitor exists (both turns'
combined survey is reasonably wide for that specific category); moderate-only
that no competitor exists *at all* — the literate-programming and docs-
tooling space was a single shallow search pass, not a dedicated survey, and
a dedicated pass there could plausibly surface a closer match (e.g. some
docs-as-code round-trip tool, a wiki-based specification-by-example tool like
FitNesse's editable-wiki-page model, or a translation-memory-style CAT tool
adapted to code). The claim as scoped to "test-harness ecosystem" is solid;
as a universal claim it should be treated as a lean, not a verified fact.

## Recommendation spectrum (unchanged framing from turn 1 — no decision)

1. **Publish errorloom narrowly**: subprocess-runner and diff-engine become
   thin wrappers/optional-integration points over assert_cmd and snapbox
   respectively (or explicit non-goals, documented as "bring your own, we
   compose"); errorloom's real surface area is the case-file format, the
   mock-stub generator, bless-review UX, and the prose-transport leg. Lowest
   bloat, clearest one-sentence pitch, easiest to get taken seriously by
   people already holding assert_cmd/snapbox-shaped test suites.
2. **Publish errorloom as a fuller framework** (own subprocess runner, own
   diffing, everything vendored/reimplemented) for zero-dependency
   simplicity and full control — higher bloat, harder sell against
   assert_cmd's ecosystem gravity, but avoids coupling to two other
   maintainers' release cadences and API-stability choices (both assert_cmd
   and snapbox are pre-1.0-feeling in places despite their download counts;
   verify current SemVer stability before committing to depend rather than
   compose).
3. **Split the publication**: two crates — a small `errorloom-stub` (the
   PATH-shim mock-stub mechanism, genuinely novel, could stand alone and be
   useful even to assert_cmd users) and the fuller `errorloom` harness
   (case-files, bless, prose-transport) that depends on `errorloom-stub`
   internally. Mirrors ui_test/Miri's extraction pattern and maximizes reach
   of the one truly novel piece independent of whether people adopt the
   whole harness.

## Report

Files: `C:\Users\ec\Sync\Code\Dorc\.claude\research\loom-harness-alternatives\synthesis.md`
(turn 1) and `C:\Users\ec\Sync\Code\Dorc\.claude\research\loom-harness-alternatives\turn02-rust-ecosystem.md`
(this file).
