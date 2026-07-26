# _w3-map-DRAFT — the W3 surface-fold map (MAP HALF; conductor working file)

AI-authored (Opus builder `lane-w3-fold`, 2026-07-26), branch `ai/r28-w3-fold` off
`ai/r28-unify` @ `15edfb05`. This is the map-then-execute split's FIRST half
(`spike/CLAUDE.md` map-then-execute-split): inventory, sizing, proposals. ZERO
engine/render/test edits made. Not a durable — the conductor disposes of it.

Charter: `plans/28G` §1 Phase W3. Gates: `28D:must-default-durable-lands-with-its-hardening`
(all-or-nothing) · `28D:must-retention-is-one-decision` · `28E:nack-whylog-stores-book-bytes`
(annotation-tier fence) · `28E:lean-why-is-whylog-reconciliation` ·
`28E:lean-git-source-tracking-secondary`.

All code anchors are file + item name (a parallel `TrustTier`→`SpeechAct` /
`tier_word`→`verb_word` rename lane will move line numbers).

---

## A. THE HARDENING BILL, itemized against as-built code

The write path is ONE function: `spike/crates/cli/src/main.rs` :: `write_whylog`,
called from `run()` under `if let Some(dir) = &args.whylog_dir`. Its helpers are
`whylog_entries` / `newest_whylog` (same file). The read path is
`spike/crates/cli/src/main.rs` :: `load_whylog_replay` →
`spike/crates/plan/src/whylog.rs` :: `admit_unscoped_whylog` /
`admit_unscoped_whylog_replay`.

Reference implementation already in-tree for four of the seven:
`spike/crates/dorc-loom/src/receipt_store.rs` :: `FsReceiptStore`. It is NOT reachable
from `cli` — `dorc-loom` depends on `dorc-cli` (its `Cargo.toml` says so, for the
worldless-route parser seat), so `cli → dorc-loom` is a dependency CYCLE. See
`ask-where-the-safe-store-lives` below.

| # | bill item | state | where | size | platform honesty |
|---|---|---|---|---|---|
| A1 | exclusive creation | **ABSENT** | `write_whylog` uses `std::fs::write` (= `create`+`truncate`, no `O_EXCL`); the target index comes from `whylog_entries(dir).last()+1`, a read-then-write TOCTOU | **S** | symmetric; `OpenOptions::create_new(true)` is cross-platform |
| A2 | restrictive file mode | **ABSENT** | neither `write_whylog`'s file nor its `create_dir_all(dir)` sets any mode | **S** on unix, **BLOCKED** on Windows | see `fnd-windows-mode-collides-with-no-ffi` |
| A3 | atomic replacement | **N/A-as-designed → S** | the design appends `whylog-NNNN.txt`, so nothing is ever *replaced*; A1 supplies the atomicity. Becomes **M** only if the fold wants a stable `latest` name | **S** (or **M**) | POSIX `rename(2)` replaces atomically; Windows does not — `FsReceiptStore::publish`'s ~150-line `#[cfg(windows)]` backup dance is what that costs |
| A4 | reads bounded independently of the writer | **PARTIAL** | see the sub-table below | **S** | symmetric |
| A5 | trusted-directory rule | **ABSENT** | `write_whylog` does `create_dir_all` on an arbitrary argv path; no component is checked for symlink / reparse-point / non-directory; the durable file itself is opened by name with no `symlink_metadata` no-follow check | **M** | `unsafe_metadata` in `receipt_store.rs` handles both (`is_symlink` + Windows `FILE_ATTRIBUTE_REPARSE_POINT` 0x400) — liftable |
| A6 | visible persistence failure | **ABSENT** | `write_whylog` has FIVE silent `return`s (dir-create fail · `try_serialize_v2` refusal · over-`WHYLOG_CAP` · `fs::write` fail · retention-unlink fail, the last `let _ =`). Its own doc-comment says "a write failure is swallowed". Zero write-side `DiagCode`s exist — only the four read-side ones (`WhylogVersionRefused`/`BookDesync`/`Absent`/`Corrupt`) | **S-M** | symmetric |
| A7 | stated sensitivity contract | **ABSENT (user-facing)** | `AID-NEEDS:law-whylog-is-sensitive` exists as internal design law and is cited in `cli/src/lib.rs`'s `whylog_dir` doc-comment; nothing the USER can read states what the durable holds. The help page's `--whylog-dir` row does not mention sensitivity | **S** mechanically, **conductor/human** for the words | symmetric |

### A4 detail — what the bounded intake does and does not cover

| covered? | surface |
|---|---|
| YES | the durable's outer stream: `admit_unscoped_whylog` does `reader.take(outer_bytes+1)` + refuse-on-overflow, `WhylogLimits::spike_default()` = 16 MiB |
| YES | per-line / per-field / retained-byte / numeric-digit / argv-entry / oracle-entry / apply-entry / digest-hex bounds, all in `WhylogLimits`, all injectable via `WhylogLimits::new` |
| YES | **independence from the writer** is genuinely satisfied for the durable itself: the writer caps at `WHYLOG_CAP` = 1 MB, the reader independently bounds at 16 MiB and re-derives every field bound. A writer cap is never treated as proof |
| YES | the nested records block gets its OWN budget (`HostEvidenceLimits::spike_default()` applied in `admit_unscoped_whylog_replay`) — the "outer + inner both consume a budget" rule of `rul-host-bytes-bounded-before-admission` |
| **NO** | `whylog_entries` runs `read_dir` with no cardinality cap and collects every match into a `Vec`. Under a default-on durable in a directory an attacker (or a runaway) can populate, that is an unbounded allocation on every plan/apply AND every `why` |
| **NO** | `load_whylog_replay` does `std::fs::read_to_string(&book_path)` and one per oracle, where `book_path` / `oracle_paths` come from **the durable's own recorded hints**. Unbounded reads of durable-named paths, performed BEFORE `replay_claims_match` verifies anything |

`fnd-path-hint-is-used-as-a-capability`: `spike/crates/plan/src/whylog.rs` ::
`RecordedSourcePathHint`'s doc-comment says, twice, "It is never a source-loading
capability" / "It is not an authority to load a source." `load_whylog_replay` uses it
as exactly that. The digest check that follows makes the *outcome* safe (a wrong path
⇒ `Framing` refusal), but the READ has already happened, unbounded, at whatever path
the durable named — including a symlink or a FIFO. Today the durable is opt-in and
controller-written; default-on plus a predictable filename plus an unvalidated
directory is precisely the combination that turns this into a live edge. I'm +SURE
about the code shape; ~SUSPECT about how much it matters at spike scale.

### Two bill items that are NOT in the seven but the fold drags in

`fnd-fnv-digest-reaches-default-persistence` (**sharpest hidden item**).
`spike/crates/plan/src/invocation.rs` :: `book_digest` is FNV-1a-64.
`spike/CLAUDE.md` `rul-fixture-identity-never-production` says FNV-style digests
"must be structurally unable to reach a production boundary — remote transport,
concurrency or retry, saved approval, multi-host caching, **default persistence**, or
anything published." Default-on is default persistence, named in the list. The
durable's `book digest=` / `oracle digest=` / `digest decision=` fields all carry it,
and the replay's whole identity check rests on it. A dependency-free SHA-256 is ~150
lines plus NIST-vector pinning: **M**, and it is a correctness surface, not churn.
The one named substitution point already exists (`book_digest` is deliberately the
sole definition), so the change is localized — but it re-digests every committed
transcript that prints a digest.

`fnd-retention-becomes-load-bearing`. `WHYLOG_KEEP = 5` / `WHYLOG_CAP = 1_000_000`
are builder latitude under `tc-whylog-retention-params`. `USER_STORY.md` promises
"Ask tomorrow; ask next week" — keep-5 means a busy admin's receipt is gone after five
runs. `28D:must-retention-is-one-decision` says ONE retention design ("what is
durable, for how long, at what permissions, classified how") MUST precede the features
that inherit it, and it names the current whylog's five local decisions —
"predictable names, ambient permissions, swallowed write failures and unbounded
replay" — as the cautionary tale. `plans/28G` §2 has already banked that design for
r30. The hardening bill covers permissions + classification; **for-how-long is not in
the bill and is not scheduled before the fold.**

### A third: a product hole default-on makes common

`fnd-desync-refusal-eats-the-receipt`. `load_whylog_replay` → `replay_claims_match`
compares the durable's `book_digest` against a fresh `book_digest` of the on-disk
book; a mismatch is an `AdmissionRefusal::Framing` and the whole replay refuses.
`Mode::Why` then has a second belt-and-braces `decision_digest` check that emits
`WhylogBookDesync`. So **the moment anyone edits the book, the receipt for last
night's run becomes unreadable.** `28E` §4 acknowledges this ("as-built 22F
desync-refusal stands meanwhile"), and `28E:lean-git-source-tracking-secondary`
explicitly says the git work "must never block the plain 'I slept, why did it break
overnight' path" — but the thing blocking that path today is the desync refusal, not
git. Under opt-in this is a corner; under default-on it is the headline user story
failing on the second most likely morning.

### `fnd-windows-mode-collides-with-no-ffi`

What is actually achievable, per platform, for A2:

- **Unix**: fully achievable and cheap. `std::os::unix::fs::OpenOptionsExt::mode(0o600)`
  applies the mode at `open(2)` time, so there is no window where the file exists
  world-readable; `std::os::unix::fs::DirBuilderExt::mode(0o700)` does the same for the
  directory. Precedent in-tree: `main.rs` :: `materialize_shim_dir` already does a
  `#[cfg(unix)]` `set_permissions(0o755)`.
- **Windows**: NOT achievable within the workspace's own invariants. There is no mode.
  `std::fs::Permissions::set_readonly` maps to `FILE_ATTRIBUTE_READONLY`, which is not
  an ACL and restricts nobody. Real restriction needs a DACL via
  `SetNamedSecurityInfo`/`CreateFileW`, i.e. `windows-sys` — and `spike/Cargo.toml` sets
  `unsafe_code = "forbid"` workspace-wide with `inv-no-unsafe` stating "No FFI", over a
  workspace that currently has **zero** third-party dependencies. Adding one to the
  product binary is a design event, not a builder call.
- The honest Windows mitigations are (a) siting the default durable under a per-user
  profile root whose inherited ACL is already user-only, and (b) saying so in A7's
  contract. Anything else would be a promise we cannot keep — which is exactly the
  shape `28D:must-split-the-bundled-entries` forbids ("MAY NEVER be described as
  scrubbed").
- `spike/CLAUDE.md` `one-platform-green-is-not-cross-platform-green` binds here
  literally: the last two live bugs in this codebase were both in `#[cfg]`-gated
  file-store code, both invisible to a green Windows run, and one of them was a
  `#[cfg(unix)]` `chmod` that did not type-check. Every `cfg`-gated member of the new
  store must be gated at every member and checked on BOTH platforms.

---

## B. THE SURFACE FOLD — mechanics

### B1. `dorc why` defaults to the whylog

As-built, `Mode::Why` runs the *live* pipeline and reads records from stdin or
`--results FILE`; `--last` switches it to durable replay. The fold inverts the
default.

| piece | as-built | change | size |
|---|---|---|---|
| book becomes optional under `why` | `parse_args_from` refuses `books.is_empty() && !last` | relax to `books.is_empty() && !last && mode != Mode::Why` (the durable names the book, as `--last` already proves) | **S** |
| `--last` becomes implied | `args.last` gates `load_whylog_replay` in `run()` | derive an effective-replay predicate: replay unless a record source was named explicitly. `--last` survives as an accepted no-op spelling (it is printed in five committed transcripts) | **S** |
| a default durable directory | `whylog_dir: Option<String>`, default `None` ⇒ no write, no read | needs a controller-chosen default root. **This is the one piece I will not choose** — see `ask-where-the-default-durable-lives` | **S** code, **human** decision |
| absent-durable UX | `WhylogAbsent` already exists and fires from `load_whylog_replay` | reword toward the zero-setup story; today it names a `--whylog-dir` the user never typed | **S** + a prose act |
| `--whylog` / `--last` flag-requires-mode checks | `whylog.is_some() && (mode != Mode::Why \|\| !last)` errors | `!last` clause must relax with the implied default, else `dorc why --whylog=F` breaks | **S** |
| the receipt-first voice | LANDED in W2 (`28F`: receipt header, `--last` replayed voice, `started_at`-dated receipts, `(received …, rc N)` trailers) | nothing | — |

`records-from-argv survives as harness posture` — the mechanics question:

The eight in-corpus `dorc why` invocations that feed records do it with a bare
`< probe-results.txt` stdin redirect (five gallery looms; the e2e runner's
`run_extra_replay` only accepts literally `< probe-results.txt`). Under the fold, "no
explicit record source ⇒ read the durable" needs a way to know stdin *is* a source.
Three options, priced:

- `opt-harness-passes-results-flag`: change the eight commands to `--results
  probe-results.txt`. Deterministic, no ambient reads, honest. Cost: 8 loom command
  lines + the e2e runner's redirect handling must learn the flag form (or the flag
  simply rides as an ordinary arg — likely free). **S**. My recommendation.
- `opt-detect-stdin-is-not-a-tty`: an ambient environment read at the cli edge. Breaks
  hermeticity and `io-at-edges-only`'s spirit; a piped CI `dorc why` would silently
  change surface. **Reject.**
- `opt-keep-bare-stdin-as-a-source`: treat "stdin has bytes" as explicit. Same ambient
  problem, plus it makes `dorc why` block on a terminal. **Reject.**

### B2. `dorc plan --why` as the remediation verb

`28E:lean-why-is-whylog-reconciliation` (with the conductor's in-sitting correction,
which I confirm against the code): **as-built live `why` never probes** — there is no
apply/probe executor anywhere in the spike (`cli/CLAUDE.md` scope-boundary;
`tc-apply-report-is-prediction`), and `Mode::Why` recomputes from supplied records
exactly as `Mode::Plan` does. So `plan --why` is surface simplification, not a safety
change. Confirmed: nothing in the `Mode::Why` path can execute anything.

| piece | change | size |
|---|---|---|
| the flag | `--why` / `--why=<address>` on `Mode::Plan` (a new `Args` field, or reuse `why_address` + a bool) | **S** |
| routing | `Mode::Plan` already emits the apply artifact on stdout and the compact why-lens on stderr. The full report (`emit_why_report`) currently writes STDOUT. Under `plan --why` it must go to **stderr** — `cli/CLAUDE.md` stdout-contract is "stdout is EXACTLY probe-then-apply", and the e2e capture splits on shebangs | **S**, but see `ask-plan-why-goes-to-stderr` |
| the pull-only law | `rul-chain-is-pull-only` says the numbered chain renders only on pull surfaces (`why N` live / `--last`) and "plan stderr keeps compact attribution lines". `plan --why` is an explicit ask ON a push surface | **S** code, **conductor ruling** — see `ask-plan-why-is-a-pull-surface` |
| what `emit_why_report` needs | it already takes everything as parameters (plan, probe, walls, diags, arena, ast, book, interner, oracle paths+srcs, narratives, `Receipt`); the `Receipt` is constructed inline in the `Mode::Why` block. Nothing new to plumb | **S** |

### B3. The two argv riders (`plans/28G` §1 says they ride along)

- `289:rider-why-last-address-order` — **already FIXED in W1** (`28F` records it, plus a
  qualified-address sibling the W1 builder found). `parse_args_from` now takes the first
  bare word as the address wherever it sits, and `address_names_book` guards the
  file-qualified case. Nothing owed.
- `289:rider-sibling-note-false-fires-relative` — **STILL LIVE**. `main.rs` ::
  `emit_unloaded_sibling_oracles` builds `loaded` from the `-o` argv strings verbatim
  (`firewall.oracle.sh`) but compares against `read_dir` entry paths
  (`./firewall.oracle.sh`), so every relatively-named oracle reports itself unloaded.
  Fix: compare through one canonical form (join both sides onto the same dir, or compare
  file-name tails). **S**, ~10 lines + a unit test. It is stderr-only, which is why no
  committed transcript shows it — worth confirming a `plan` case's stderr after the fix.

---

## C. THE GIT-MATCH ANNOTATION LINE

Design target, from the flagship strawman
`Research/notes/28G-why-strawmen-v2/a-fire-morning.loom`:

```
receipt: apply 2026-07-25 02:00:37, host web1, trigger cron
   book web.sh (matches git HEAD 9f31c2e -- unchanged since the run)
```

As-built the same row renders `book book.sh (digest de40127febe86c48)` from
`main.rs` :: `receipt_banner`'s `Said::words("why-receipt-book", &[book, digest])`.

### The fence, restated so the execute half cannot drift past it

`28E:nack-whylog-stores-book-bytes` — annotation-tier ONLY. It may say "this run's
book is commit X, HEAD has drifted"; it **never** substitutes bytes into the receipt
render, never re-reads a book from git to render from, never feeds a license, never
becomes a fact. It is one display row on one surface.

### What is actually cheap, and what is not

A structural fact the sizing turns on: because of `fnd-desync-refusal-eats-the-receipt`,
**by the time the receipt renders, the on-disk book is byte-identical to the run's
book** (a mismatch refused earlier). So the "does this match git" question needs no
stored bytes and no history walk for the common answer:

| variant | mechanism | subprocesses | size |
|---|---|---|---|
| `matches git HEAD <short>` | `git rev-parse --short HEAD` + `git rev-parse HEAD:<relpath>` compared against `git hash-object -- <path>` (or `git diff --quiet HEAD -- <path>`) | 2–3, no history walk | **S** |
| `is commit X, HEAD has drifted` | requires finding WHICH commit last held these bytes: `git log --format=%H -- <path>` then a blob compare per candidate | O(history), needs a hard cap | **M** |
| anything keyed off the stored FNV digest | impossible as posed — an FNV-1a-64 is not a git object id, so "digest-keyed" cannot mean a direct lookup. The exactness comes from the byte comparison above, keyed by PATH | — | — |

`ask-git-line-scope` below asks which variant W3 ships. `lean-git-source-tracking-secondary`
says "digest-keyed, exact-or-absent … silent when it misses", and the S variant is
exactly exact-or-absent; the M variant is what buys the "HEAD has drifted" wording the
strawman shows.

### The DI seam

A git read is subprocess + filesystem nondeterminism. Placement, following the two
in-tree precedents:

- **The trait precedent** is `spike/crates/dorc-loom/src/repository.rs` :: `Repository`
  (`status_porcelain` / `current_bytes` / `head_bytes`) with `GitRepository` as the one
  real impl — a narrow, named I/O edge behind a trait. That is the shape to copy (not
  the type: `dorc-loom` is unreachable from `cli`, per the cycle above).
- **The purity precedent** is `main.rs` :: `RunClock` / `dorc_core::RunInstant`: the
  nondeterministic source lives in the binary and NOTHING but already-read pure values
  cross inward. Its doc-comment states the rule outright — "the analyzer kernel owns no
  clock type at all, so no kernel signature can accept one".

So the seam is: a small `SourceMatch` **pure data** value (`Option<{ commit_short,
drifted: bool }>` shaped), computed at the cli edge in `main.rs`, stored on the
`Receipt` struct beside `book_digest`, consumed by `receipt_banner`. DST/looms fake it
by constructing the value; no kernel crate ever sees a git type. Verified clean:
`dorc-core`, `dorc-aid`, and `dorc-plan` contain **zero** `std::fs` references today,
and `dorc-plan` has no non-path dependencies.

### Two hermeticity hazards the execute half must handle

- `fnd-git-lookup-must-be-off-under-test`. The e2e/loom sandbox is
  `std::env::temp_dir()`-rooted (`Scratch::new`), so `git rev-parse` normally fails
  there and the annotation would be silently absent — but that is *environmental*
  silence, not declared silence. A developer whose temp root sat inside a repository
  would get flapping transcripts. `spike/CLAUDE.md` `real-tools-lane-opt-in` is the
  governing posture: "default UNSET ⇒ zero external invocations, zero real-tool PATH
  probes". The lookup needs an explicit injection point that the harness sets, exactly
  as `DORC_FIXTURE_CLOCK_MS` does for the clock (`28F:rul-fixture-clock-env-accepted`).
- The lookup runs a subprocess on the plan/why hot path. Per `perf-doctrine` that cost
  is nothing next to a network round-trip, but a hung `git` on a network filesystem is
  a hang in `dorc why` — the fallback must be absence, never a wait.

### Rendering

Per `28G` §0 (every string is a row, never a `format!`): two new arrangement-registry
rows — the existing `why-receipt-book` stays for the no-git case, and a sibling
(`why-receipt-book-git-match`-shaped) renders the hit. Silent-on-miss = fall back to
the existing row. The commit short-hash is a `{{...}}`-class payload value, so it does
NOT need prose authoring; the surrounding words do (conductor/human, per
`error-authorship-tier`).

---

## D. PRICED OPTIONS AWAITING HUMAN ACK (report only — nothing done)

### D1. `--trust-footprints` → `--risk-faultless-skips`

Standing rider from W1 (`28F`: "the `--risk-faultless-skips` rename stays OWED …
isolated at `CONSENT_FLAG`"). Measured scope: **58 occurrences across 19 files.**

| what | where | note |
|---|---|---|
| the rendered flag value | `main.rs` :: `CONSENT_FLAG` — ONE const | genuinely one line. The four rendered why-chain occurrences take it as a PARAM (`Said::words("why-next-step-fix-replan", &[CONSENT_FLAG])`), so they follow for free |
| the accepted spelling | `cli/src/lib.rs` — the `arg == "--trust-footprints"` arm, the did-you-mean `known` list, and one parser unit test | 3 more code lines |
| the ONE hardcoded prose site | `crates/aid/src/arrangement_lock.rs` — a single occurrence, inside the help page's `Words::Migrated` blob | edited through `crates/aid/tests/cli-help-page.loom` + promote (`chrome-comes-from-the-registry`), never by touching the lock |
| **looms needing edit + re-bless: 6** | `aid/tests/cli-help-page.loom` · `cli/tests/whygallery-webhost-whole.loom` · `whygallery-survive-trusted-footprint.loom` · `survivebite27-naked-trust-chain.loom` · `strawman24-survive-simple.loom` · `strawman24-derived-survive.loom` | each carries `flags:` frontmatter + replay COMMAND lines, and the runner refuses a transcript whose committed command is not the invocation it drives — so frontmatter and commands must move together |
| rendered transcript lines that actually change | **~6**: help page ×1, `risk-profile:` ×1, why-chain prose ×4 | small re-bless, conductor-inspectable by eye |
| non-transcript mentions | 2 case `probe-results.txt` comments, `e2e.rs` ×3, `sweep` ×2, `plan/src/{lib,survival,whylog}.rs` ×6 (doc-comments + one argv fixture), `_typos.toml`, `e2e/yardstick.sh` | cosmetic; can ride or not |
| the Rust field `trust_footprints` (~15 sites) | out of scope as briefed ("the one-line `CONSENT_FLAG` change") — but leaving it means the flag and the field disagree | **ask** |

Total: **S**. Free under `rul-strawman-formats-no-compat` (pre-user, rename in place,
no alias). It rides best AFTER the sequenced `SpeechAct` rename lane, not against it.

### D2. Absent-but-cheap items that could ride the execute half

- `289:rider-sibling-note-false-fires-relative` (B3) — **S**, independent, no transcript
  churn expected. Ride it first.
- Bounding `whylog_entries`'s `read_dir` (A4) — **S**, no user-visible change.
- A1 exclusive creation — **S**, and it is a correctness fix on the CURRENT opt-in path
  regardless of the fold's disposition (two concurrent `--whylog-dir` runs silently
  truncate each other today).
- A2 unix-side mode — **S**, precedented, and likewise owed on the opt-in path.
- A6's write-refusal code family — **S-M**, and it is the item `28D` names by hand
  ("swallowed write failures"). Owed regardless.

---

## BOTTOM LINE — one recommendation

**Ship the fold OPT-IN, and land the whole hardening bill anyway in the same lane.**

Reasoning, not hedging:

1. The seven bill items alone total **roughly one focused lane** (S·S·S·S·M·S-M·S).
   That is not what argues against default-on.
2. What argues against default-on is that the gate's seven are not the whole price.
   Default-on additionally activates `fnd-fnv-digest-reaches-default-persistence` (an
   **M** correctness surface, named explicitly by `rul-fixture-identity-never-production`
   as a boundary the spike digest must be *structurally unable* to reach) and
   `fnd-retention-becomes-load-bearing` (which `28D:must-retention-is-one-decision`
   says must be ONE design that PRECEDES the features, and which `plans/28G` §2 has
   already scheduled for r30). Shipping default-on now is per-feature retention
   negotiation — the exact failure mode `28D` diagnoses in the current whylog.
3. A2 cannot be honestly discharged on Windows inside the workspace's own invariants.
   A gate that says "restrictive mode or opt-in" is not met by "restrictive mode on
   half the platforms".
4. `fnd-desync-refusal-eats-the-receipt` means the default-on user story fails on the
   second-most-likely morning (someone edited the book). Fixing that is its own design
   question, not a hardening item.

The opt-in fold that I think is still worth shipping in W3 is a real deliverable, not a
placeholder: **the SURFACE folds even though the DURABLE stays flag-gated.**
`dorc why` becomes receipt-first (reconciliation is the default reading), records-from-argv
becomes the explicit harness posture, `dorc plan --why` becomes the remediation verb,
the git annotation lands, and the whole bill is paid down on the `--whylog-dir` path so
that r30's default-on flip is genuinely one line plus the digest plus the retention
design. That satisfies `must-default-durable-lands-with-its-hardening` literally (the
durable is not default-on) while removing every reason it would fail next round.

Bill total if the conductor rules default-on anyway: the seven ≈ **M** (one lane), plus
digest **M**, plus retention **?** (unscoped, r30-owned). I would not sign the third.

---

## PROPOSED EXECUTE-HALF COMMIT PLAN (ordered, granular)

Each line is one commit, gitlabels-style, gates run before each. Steps 1–6 are
disposition-independent (owed on the opt-in path too); 7–9 are the fold proper;
10 is conditional on D1's ack.

1. `(AI fix)` Compare loaded and discovered oracle paths in one canonical form
   — the `289:rider-sibling-note-false-fires-relative` rider + a unit test.
2. `(AI new)` Create the durable exclusively and retry a taken index
   — A1; `create_new` + bounded attempt loop (the `TEMP_ATTEMPTS` shape), unit tests in
   `main.rs`'s existing `mod tests`.
3. `(AI new)` Open the durable and its directory user-only on unix
   — A2; `OpenOptionsExt::mode(0o600)` / `DirBuilderExt::mode(0o700)`, every member
   `cfg`-gated (never `allow(dead_code)`), checked on both platforms.
4. `(AI new)` Refuse a durable directory reached through a link or a non-directory
   — A5; per-component `symlink_metadata` walk + no-follow on the file itself.
5. `(AI new)` Report a refused durable write instead of returning quietly
   — A6; one code family over `WhylogWriteRefusal` + the I/O cases, EMPTY prose per
   `error-authorship-tier`, one defining case in `crates/aid/tests/`.
6. `(AI fix)` Bound the durable directory scan and the replay's re-read of named sources
   — A4's two gaps.
7. `(AI new)` State what the durable holds where the admin can read it
   — A7; registry row(s) via the help-page loom + promote. Words are conductor/human.
8. `(AI new)` Read the receipt by default and take records only when named
   — B1; parser relaxations, implied replay, `--last` retained as a spelling, the 8
   harness commands moved to an explicit `--results`.
9. `(AI new)` Answer the asked question fully on the plan surface
   — B2; `plan --why[=ADDR]`, report to stderr.
10. `(AI new)` Annotate the receipt's book line with its commit when git agrees
    — C; `SourceMatch` pure data + the edge trait + the harness kill-switch + two rows.
11. `(AI -)` Rename the consent flag / `(AI test)` Re-bless the six looms
    — D1, two commits, conditional on the human's ack and sequenced after the
    `SpeechAct` lane.

---

## JUDGMENT CALLS — flagged, NOT resolved

Every one of these is a question for the conductor (and several for the human).

- `ask-fold-disposition-is-the-gate` — do you take the opt-in recommendation, or rule
  default-on and accept the digest + retention riders? Everything below assumes the
  former unless you say otherwise.
- `ask-where-the-default-durable-lives` — if default-on: which root?
  `spike/CLAUDE.md` `rul-scratch-root-never-read-from-host` forbids `TMPDIR`/`HOME`/`XDG_*`
  expansion for engine scratch, but that rule binds PROBE scratch on a managed HOST, not
  a controller-side durable; `cli/src/lib.rs`'s own doc-comment says the intended posture
  is "write-quietly-beside-its-work". Beside-the-book, a per-user profile dir, and a
  literal are three different security stories and only one of them helps Windows. Human.
- `ask-where-the-safe-store-lives` — three homes for the hardened store, and the cycle
  rules one out. (a) duplicate ~150 lines into `cli/src/main.rs`, unit-testable via the
  existing `mod tests`, but a second copy of security-critical code; (b) a new tiny crate
  both `dorc-loom` and `cli` depend on — one home, `do-one-thing-well`, costs a crate and
  a `dorc-loom` refactor; (c) `cli/src/lib.rs` — **forbidden**, `lib-target-is-a-loom-seam`
  says anything in the lib that wants a file "is on the wrong side of the seam". I lean
  (b) but it is a structural call.
- `ask-write-failure-severity` — is a refused durable write advisory or error? Advisory
  is suppressed under `Mode::Apply` (`advisory = !matches!(mode, Mode::Apply)`), which is
  precisely the run whose receipt matters most; error-tier crosses the floor in every
  mode but puts a postmortem-aid failure on the off-ramp console. `28D` says "visible",
  not "fatal". Neither reading is obviously right.
- `ask-plan-why-goes-to-stderr` — confirming: `plan --why`'s report goes to stderr, since
  `cli/CLAUDE.md` stdout-contract reserves stdout for probe-then-apply bytes. Cheap to
  confirm, expensive to get wrong (it breaks every e2e capture).
- `ask-plan-why-is-a-pull-surface` — `rul-chain-is-pull-only` currently reads
  "`dorc why N` live / `--last`" and "plan stderr keeps compact attribution lines". Does
  an explicit `--why` make plan a pull surface for that law's purposes? I believe yes
  (the user asked), but the law is stated in surface terms, not ask terms, and
  `AID-NEEDS:law-pull-runs-wide-open` deserves a matching amendment if so.
- `ask-git-line-scope` — S variant (matches-HEAD, exact-or-absent, 2-3 subprocesses) or
  M variant (name the commit when HEAD has drifted, capped history walk)? The flagship
  strawman shows a sentence the S variant can render ("matches git HEAD 9f31c2e --
  unchanged since the run"); the `28E` lean's example sentence ("this run's book is
  commit X, HEAD has drifted") needs the M variant.
- `ask-harness-results-flag-churn` — accept `opt-harness-passes-results-flag` (8 loom
  command lines change, transcripts unchanged) as the way records-from-argv stays
  explicit?
- `ask-rename-field-too` — D1: flag string only, or also the ~15 `trust_footprints`
  Rust identifiers? Leaving them means flag and field disagree permanently.
- `ask-digest-substitution-timing` — even under opt-in, is `fnd-fnv-digest-reaches-default-persistence`
  worth pre-paying in W3 (an **M** that unblocks r30's one-line flip), or does it belong
  with the retention design it will ship beside?
- `tc-path-hint-capability-widening` — `fnd-path-hint-is-used-as-a-capability` is a
  type-level promise the CLI breaks. Fixing it properly (make the hint un-openable and
  route the read through a bounded, validated loader) is cross-crate and touches an
  intake type; I flag it up rather than patching it inside a render lane.

## LOOM / ERRORLOOM FRICTION (standing brief line, `28F`)

None encountered — this half made no edits and drove no tooling. Two observations from
reading the corpus, offered as data for `lane-loom-cleanup`:

- The whole-product loom's "committed replay command must equal the driven invocation"
  check is a genuinely good net, and it is what makes D1's re-bless safe. Worth keeping
  in mind that it also means every flag rename is a lockstep frontmatter + command edit
  across N looms — a `BLESS=<substring>` scoped bless (already banked as debt L1) would
  make exactly this class of change cheap.
- `run_extra_replay` accepts only the literal `< probe-results.txt` and `> /dev/null`
  redirect forms. B1's `opt-harness-passes-results-flag` sidesteps that limit, but the
  narrowness is worth knowing before anyone tries a third redirect shape.
