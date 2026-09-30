# 30Xc — the macOS burn-down: working ledger

> Tier: working ledger (Opus, one session with the human, opened 2026-09-30), a subdoc of `30X`
> beside `30Xb`, which left macOS red by ruling. This ledger records the burn-down toward a green
> macOS `rust` lane. One section per sitting. `[TYPED]` items are the human's rulings; `[PROPOSED]`
> items are mine and carry no ack until one is typed. Authority: `IMPLEMENTATION.md`,
> `spike/CLAUDE.md`, `30X` and `30Xa` outrank this file. Line numbers are at `49085b27`.

## §0 — state

- `ai/main` at `49085b27`, two commits past `77df48b5`: `30487a89` (the staging-store fixture) and
  `49085b27` (the first platform-regression tests). macOS reds: 19 → 8.
- Left: the six receipt-root tests (`fnd-receipt-roots-diverge-on-macos`; the §2 plan awaits the
  human's ack) and the two looms (`fnd-load-keys-split-by-spelling`; held behind
  `fnd-duplicate-declarer-composes-contested-helper`, `ask-engine-fix-before-green`).
- A local `gate:full*` on macOS also needs `ask-floor-shell-on-macos`.

## §1 — sitting one (2026-09-30): the first macOS run

### Typed record

- `[TYPED]` Read the failing CI run, then investigate on this machine (Intel, macOS 26.6.2). Build
  and test runs are allowed; nothing else mutates.
- `[TYPED]` macOS runs for the first time; its reds are not regressions. The burn-down toward
  green macOS supersedes `30Xb` §5's "macOS stays red" for this arc.
- `[TYPED]` Take the most straightforward item that is one standalone commit fixing one concrete
  bug. No over-engineering; ask when simplicity and value pull apart. Work on `ai/main`; no other
  builder runs in parallel.

### Findings

- **fnd-macos-first-run-nineteen-reds** — CI run `36741484726` (`ai/main` at `b5748323`): only
  `rust (macos-15)` is red, at `mise run test`: 3145 run, 19 failed. The same 19 fail on this Intel
  machine (the suite takes about six minutes here), so the cause is the platform, not the
  architecture. Four causes follow.
- **fnd-staging-root-behind-var-link** (11 `dorc-loom` `staging_store` unit tests, "unsafe
  staging target root") — `validate_directory_tree` (`dorc-loom/src/staging_store.rs:306`)
  refuses a link at any component of the root's path. The test fixture rooted itself in
  `std::env::temp_dir()`, and macOS keeps `$TMPDIR` below `/var`, a link to `/private/var`. The
  production `-C` seat already resolves its path for this reason (`dorc-loom/src/roots.rs:68`).
  `Roots::built_in()` (`roots.rs:50`) deliberately does not, so a checkout below a link refuses
  there; untested.
- Six receipt-root tests
  (`receipt_state::the_shipped_binary_draws_live_os_identities_and_ignores_harness_seams` and five
  in `recorded_facts_route`) — `fnd-receipt-roots-diverge-on-macos`, in §2.
- **fnd-load-keys-split-by-spelling** (two looms: `emit30-ambient-dependency-narrates`,
  `load30-speaker-minting-is-observable`) — load keys are lexical (`core/src/loadpath.rs:26`,
  `resolve_operand` at `:135`). The cwd comes from `current_dir()`, which is the resolved path
  (`cli/src/compose.rs:1299`). The e2e runner hands `--book` and `--pre-source` over as absolute
  `/var/…` spellings (`shared_args`, `cli/tests/e2e.rs:2098`, used by `framed_results` at `:823`
  and by the gate re-drive in `run_round_trip` at `:2138`). One file then carries two keys:
  `/var/…/x.oracle.sh` when pre-sourced, `/private/var/…/x.oracle.sh` when an oracle's
  `. ./x.oracle.sh` reaches it.
  - load30: delta's guarded `.` is not recognized, so the framing probe emits `sites=0`. The
    session's `probe-results.txt` is rewritten from that framing (`e2e.rs:1582`), and delta runs
    (inferred, not traced: the session gets no usable site-0 record). Gate-1 produces no records.
  - emit30: the session is unaffected. Gate-1's absolute re-drive probes site 2 (beta).
  - Method: the loom's top-level sections materialized into a directory; `dorc-harness probe`
    rendered under `env -i` with `DORC_SEAM_ROOTS=pinned:<scratch>`, once with relative and once
    with absolute spellings. Render only; nothing rendered was executed. The relative and the
    `/private/var` spellings render byte-identically; the `/var` spelling differs. A link made in
    a scratch directory reproduces it, so the split is not specific to macOS.
- **fnd-duplicate-declarer-composes-contested-helper** — the engine finding under emit30,
  reproduced with no link: a byte-identical copy of `beta-common.oracle.sh` at `./copy/`, sourced
  by beta while the original is pre-sourced and `rogue.oracle.sh` redeclares `sm_beta_check`
  later. `helper-declaration-contested` fires ("Dorc withholds them all and the sites needing
  `sm_beta_check` run"). Yet beta's vouch composes, the probe checks site 2, and the plan guards
  `beta sync b.conf` with beta-common's body (`common cmp --`), not the body a shell binds
  (rogue's `common cmp --strict --`). With the one-file spelling, beta runs.
  - This contradicts `loadpath.rs`'s header ("Two spellings that do not normalize alike simply
    do not match, which WITHHOLDS"), `core/CLAUDE:contested-is-write-once` ("can under-fire but
    never un-withhold") and `cli/CLAUDE:withdrawal-is-applied-once-never-consulted`.
  - Triggers seen: a copy at a second path, a `/var` spelling, and a case-variant spelling
    (`fnd-case-variant-spelling-splits-keys`).
  - Root cause not traced. Linux and Windows CI cannot see it: no loom loads one declaring file
    under two keys.
- **fnd-discriminating-tmpdir-run** — with `TMPDIR` spelled `/private/var/…`, the staging tests
  (12/12) and both looms pass, and the six receipt-root tests still fail. This splits the 19 into
  the `/var`-spelling group and the root-layout group.
- **fnd-posh-absent-on-macos** — `mise.toml:365` gives `test:floor` `dash,posh` everywhere but
  Windows. This machine has `/bin/dash` and no `posh`, so 36 floor cases refuse locally and
  `gate:full*` is red on any macOS machine without `posh`. CI runs the floor lane on Linux only
  (`30Xb` §1).

### Landed

- `30487a89` `(AI test fix)` — the fixture resolves `temp_dir()` before use
  (`staging_store.rs:465`), as `Roots::at` does; `validate_directory_tree` is unchanged. macOS
  reds: 19 → 8. `gate:full-quiet` here is red only on known causes: the eight tests, and the
  floor lane (`fnd-posh-absent-on-macos`).

## §2 — sitting two (2026-09-30): the burn-down rules, the platform home, the receipt roots

### Typed record

- `[TYPED]` Fix a trivial item, granular and careful. Investigate and report an item with
  design-affecting aspects instead, the least consequential first. Correctness before velocity.
- `[TYPED]` Write regression tests for nearly everything, above all for macOS, which few will run
  soon. Give platform tests a dedicated home, for two kinds: tests that run only on the platform,
  and tests that run on any platform and keep what the platform needs from breaking.
- `[TYPED]` Writing through one root and reading through another is a bug.
- `[TYPED]` A test may create the product's per-user configuration directory (not its contents) on
  a contributing developer's machine, if it must. A test that uses the real per-user profile
  belongs to a separate, higher, non-automatic tier ("install it as a user would"): probably CI
  only, though a person may run it.
- `[TYPED]` Doc fixes to AI-authored text are authorized: STE100, very basic; removal before
  addition; a removal never becomes a prohibition.
- `[TYPED]` Apply no further fix until acked. The regression tests for the landed fix may go
  first, as their own commit.
- `[TYPED]` Open this ledger as `30Xc` and record everything so far.

### Answers given

- **`30Xa:rul-roots-pinned-is-a-literal`, explained** — `dorc-harness` takes its roots as
  `DORC_SEAM_ROOTS=pinned:<absolute dir>`. It writes configuration below `<dir>/config` and state
  below `<dir>/state`, and it reads no `HOME`, `APPDATA` or `XDG_*`. Without the variable it
  refuses.
  - Origin: `30Xa:dev-roots-nominal-fence`, REJECTED ("a nominal fence is no fence once the scrub
    is absent"). A harness that resolved through the platform variables would, on any spawn that
    forgot to scrub and re-point them, write a keyset and receipts into the developer's own
    profile. The census `every_seat_that_drives_the_binary_sandboxes_the_profile_it_writes_into`
    (`cli/tests/receipt_state.rs:624`) records that shape as found twice.
  - The `30Xa` close re-cut "runner-owned" paths to "selected" paths; the rule stands as "never
    consult the platform variables", not as ownership (`30Xa` §0).
  - Load-bearing here: it rules out making the harness write where `standard_roots` looks. A
    reader must come to the harness's root, not the reverse.
- **The shipped-binary test is isolated** — `ProfileSandbox::apply` points `HOME`, `XDG_*` and
  `APPDATA` into a throwaway directory. After three full runs here, no `dorc` directory exists
  under `~/Library/Application Support`, `~/.local/state` or `~/.config`. On macOS the binary
  resolves `<throwaway>/home/Library/Application Support`, which the sandbox never creates, so it
  publishes nothing and the test finds no documents. The isolation rests on production reading
  `HOME` from the environment. A resolution that read the user database instead would not be
  redirected.
- **Kagi** — not configured on this machine: no Kagi tool in the session, no server in
  `claude mcp list`, no CLI, no `KAGI*` variable name. The one trace is the allow-entry
  `mcp__kagi-ken-mcp__kagi_search_fetch` in the synced `.claude/settings.local.json`; server
  definitions live per machine in `~/.claude.json`. The reading on macOS conventions for CLI
  tools waits on it (`ask-kagi-setup-here`).

### Findings

- **fnd-receipt-roots-diverge-on-macos** (from §1) — three layouts meet:
  - production on macOS puts both roles in `$HOME/Library/Application Support`
    (`cli/src/durable.rs:93`);
  - the harness always writes `<root>/config` and `<root>/state` (`cli/src/seam.rs:408`);
  - the test sandbox points `APPDATA`/`XDG_*` at `<root>/config` and `<root>/state` and `HOME`
    at `<root>/home`, and creates no `home/Library/Application Support`
    (`cli/tests/sandbox.rs:37`, `:79`).
  - The shipped-binary test: `durable-receipt-unwritten (temporarily-unavailable)`, exit 0, no
    documents where it expects two. By hand, with that directory created: two distinct receipts
    and a keyset land below `…/Application Support/dorc/`. The test's `store_root` and
    `keyset_dir` (`receipt_state.rs:178`, `:182`) describe the pinned layout, which the
    harness-driven tests in the same file rely on.
  - `recorded_facts_route` publishes through `dorc-harness` (`:85`) and reopens through
    `standard_roots` (`:106`). `a60602fe` (2026-09-02) moved publishing from the shipped binary to
    the harness and left the reader; `30Xa` then made the pinned root a literal. Writer and reader
    agree only where the sandbox's platform variables equal the pinned layout: Linux and Windows.
    The module doc still says "standard roots"; `publish`'s doc still says "through the shipped
    binary".
- **fnd-case-variant-spelling-splits-keys** — this volume is case-insensitive.
  `--pre-source BETA-COMMON.oracle.sh` (the file is `beta-common.oracle.sh`) reproduces
  `fnd-duplicate-declarer-composes-contested-helper` with no link and no copy. Unicode
  normalization is a likely fourth trigger; untested.
- **fnd-macos-root-arm-untested** — `durable.rs` tests the XDG arm (`:694`) and the Windows arm
  (`:735`) of `standard_roots`; the macOS arm has no test.
- **fnd-hostsim-sandbox-lacks-macos-base** — `hostsim::differential::sandbox_profile`
  (`hostsim/src/differential.rs:818`) is a third copy of the sandbox roots. On macOS its
  shipped-binary runs take `durable-receipt-unwritten` silently.
- **fnd-help-page-omits-macos-location** — `cli-help-receipt-kept`
  (`aid/src/arrangement_lock.rs:51`; loom `aid/tests/cli-help-page.loom`) names
  `$XDG_STATE_HOME/dorc`, `~/.local/state/dorc` and `%LOCALAPPDATA%\dorc`. On macOS the location
  is `~/Library/Application Support/dorc`. Human-facing prose: the human's to reword.
- **fnd-stale-root-layout-comments** — the `pinned_roots` doc (`seam.rs`) says the pinned roles
  stay separate "exactly as every platform keeps them" and land "precisely where the platform
  route would"; both are false on macOS. `30Xb` §5 (lines 122–124) puts the receipt reds on the
  `/var` link; they are the layout divergence above, and they survive a resolved `TMPDIR`.
- **fnd-macos-ci-later-steps-unexercised** — after `test`, the macOS job runs `test:hooks`,
  `test:real-tools` and `verify:check`. None has run on macOS yet.

### Landed

- `49085b27` `(AI test new)` — `dorc-loom/tests/platform.rs`, the first platform home
  (`dec-platform-home-per-crate`). Its `for_macos` module compiles on every Unix:
  - `a_root_below_a_link_is_refused_until_resolved` — a link at a component above the root is
    refused, and the resolved spelling is accepted. The existing unit test covered only a root
    that is itself a link.
  - `a_tree_named_through_a_link_resolves_before_the_store_checks_it` — `-C` through a link
    reaches a staging root the store accepts.
  - Each went red under a deliberate break (the store resolving links itself; `Roots::at` keeping
    the typed spelling); both breaks were reverted.
  - Not caught on Linux: a revert of `30487a89` itself. The twelve staging unit tests catch it,
    on macOS only.
- **dec-platform-home-per-crate** `[PROPOSED]` — `crates/<crate>/tests/platform.rs`, with
  `for_<platform>` modules (they make the platform's condition in a scratch directory and run
  wherever they can) and `<platform>` modules (gated to the platform). One per crate, not one
  central crate: a central crate would name every crate it tests, and human-ack rosters fence
  some of those names (`receipt-local/tests/crate_fences.rs` lets only `cli` name
  `dorc_receipt_local`).

### The plan, awaiting the human's ack

1. `[PROPOSED]` `recorded_facts_route` reopens through the harness's own root rule:
   `HarnessSeams::from_env` over `DORC_SEAM_ROOTS=pinned:<root>`, then
   `compose::production_receipt_edge_over` (`cli/src/compose.rs:959`), the seat the in-process
   loom driver already uses (`dorc-loom/src/consumer.rs:1970`). Its two stale doc comments are
   fixed. No Linux-side guard exists for this one; macOS CI is the net.
2. `[PROPOSED]` The shipped-binary test: the sandbox creates `home/Library/Application Support`
   on macOS, and the test's expectations restate the platform's location. Two tests that run on
   any platform: the macOS arm of `standard_roots` (`fnd-macos-root-arm-untested`), and a check
   that the sandbox redirects every variable any platform's root rule reads.
3. `[PROPOSED]` The `pinned_roots` doc loses its two false claims (`fnd-stale-root-layout-comments`).

## §3 — open decisions

- **ask-reopen-through-harness-roots** — plan item 1 as proposed. `standard_roots` coverage then
  lives in the shipped-binary test and in unit tests only.
- **ask-restate-or-derive-macos-rule** — the shipped-binary test restates the macOS location
  (independent of production; proposed) or derives it from `standard_roots` (one source).
- **ask-platform-home-steering-line** — one line in `spike/CLAUDE.md` that names the platform
  home.
- **ask-one-home-precedence** — where a test already has a one-home rule (receipt state lives in
  `receipt_state.rs`), that home wins and `platform.rs` takes the rest.
- **ask-duplicate-declarer-priority** — whether tracing and fixing
  `fnd-duplicate-declarer-composes-contested-helper` comes before the rest of the burn-down, and
  whether it also takes a row in `ANALYZER-NEEDS.md`.
- **ask-engine-fix-before-green** — resolving the e2e scratch root would turn both looms green
  and hide the engine finding on every platform. Proposed: land the engine fix, or an `xfail:`
  pin for it, first.
- **ask-key-identity-at-edge** — whether one file under two spellings (`/var` and
  `/private/var`; different case on a case-insensitive volume) is one source to Dorc, resolved by
  file identity at the CLI edge and fed to a still-lexical kernel, or stays two sources that
  withhold once the engine finding is fixed.
- **ask-floor-shell-on-macos** — install `posh` on macOS, provide it through mise, or run a
  dash-only half floor there, as on Windows.
- **ask-annotate-30xb-misattribution** — an adjacent correction note at `30Xb` §5.
- **ask-kagi-setup-here** — set up Kagi on this machine, allow another search tool for the
  macOS reading, or defer the reading.
