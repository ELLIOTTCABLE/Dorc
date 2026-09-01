# 30Rn — secure-durables review, lane fable-a

> Tier: independent engineering review of the receipt-family arc, first-parent range
> `7693ac6f..4414af7a` on `ai/r30-receipt` (230 first-parent commits; five merges from `ai/main`
> reviewed only for integration defects). Remit: engineering, software quality, maintainability,
> cross-platform behaviour, UX. Security is deliberately and entirely absent from this document —
> nothing below is a security claim, and nothing quarantined was read. Findings are ordered by
> user-visible consequence. Certainty is marked per the project's vocabulary.
>
> Evidence: `spike/crates/cli/tests/r30_review_evidence.rs` (five cases, EXPECTED RED at the tip;
> each names the finding it pins) and the focused runs recorded under "Evidence" at the foot.

## the-verdict-in-one-screen

The arc replaced a ~3K-line durable that could answer `dorc why 10 --last` with the full
explanation chain with a ~22K-line receipt family whose post-hoc `dorc why` prints an inventory of
digests and tokens, ignores the address it is given, silently answers about the wrong run after
any plan that admitted no records, and gates `dorc apply` on the ability to create a per-user key
profile. The local gates are green on both legs; the product surface the arc was built to serve
does not exist in the binary. The typed internals are careful and consistent; the delivery order
was wrong: the old lane was deleted before the replacement rendered anything, and the rendering
was handed to "the next conductor".

## findings

### 30Rn:fnd-posthoc-why-regressed-to-an-inventory — highest

+SURE. At the base (`7693ac6f`), `dorc why 10 --last --whylog-dir=whylog` rendered the same
OUTCOME / ANALYSIS / NEXT STEPS chain the live `dorc why 10 --results=…` renders (the committed
transcript in `spike/crates/cli/tests/whygallery-survive-trusted-footprint.loom` at the base carried
both invocations, byte-identical). At the tip, every receipt-rooted `why` — bare `dorc why`,
`--receipt-last`, `--receipt-id` — prints `cli::recorded::recorded_plan_listing`'s lines and
nothing else:

```
receipt 56ad46…
signing-key aa5102…
records 14
source 0 book 6b796e… 34
sites 1
site run host-influenced
regions 0
presented-plan 06df17…
opaque 0 target-name localhost
opaque 1 source-path book.sh
opaque 1 source-content #!/bin/sh\x0ahork tune --profile web\x0a
opaque 4 site-locator dorc-receipt-locator/1\x0astages 1\x0ahead 0\x0astage authored 0 10 33 none 0 -\x0a\x0alocator-end\x0a
opaque 4 shell hork tune --profile web
```

The module header says so itself (`recorded.rs`: "A listing, not prose"), and so does the lane
ledger (`30Rk:the-recorded-why-surface-is-the-next-round`). What the ledger does not say:

- The address is parsed and discarded. `main.rs:187-202` routes every `reads_the_receipt()`
  invocation to `engine::report_recorded_store(reading, receipt_root, label, sink)`, which has no
  address parameter. `dorc why book.sh:2 --receipt-last` and `dorc why --receipt-last` produce
  byte-identical stdout (evidence case `review_evidence_why_address_changes_the_recorded_answer`).
  `--all` is likewise inert on this path.
- The sealed model that was supposed to feed the render — `dorc_receipt::report::RecordedWhyFacts`
  (five files, ~1.5K lines) and `cli::recorded_facts::facts_for` — has no production caller
  (`grep facts_for\(` → one definition, four test uses). It reaches the binary through a test
  battery only.
- USER_STORY's "Recovery" section — `dorc why 9` on the bad morning — is the flagship pull surface
  of the whole product, and it is exactly the interaction that the shipped binary no longer has
  post-hoc. The live path still works only if the admin kept the probe-results file.

Why this happened, as far as the range shows: `plans/30R` §implementation-direction step 5 says
"route minimal current plan values through receipt write/read and `dorc why`", step 9 says delete
the old format. The builders satisfied step 5 with a listing, satisfied step 9 literally
(`b0bceb97`, `d1e8e3db`, `ce6ce48c` remove the writer, reader, replay lane, thirteen cases, five
codes), and rehomed the render to "the next conductor" (`30R:recorded-versus-rederived`,
`30Rk`). AGENTS.md's warning about "large, overengineered multi-phasic plans" names this shape.

Consequence for the admin: after the receipt cutover, "why did line 9 not run last night" has no
answer unless the results file was saved; the only post-hoc artefact is a listing they must
decode by hand.

### 30Rn:fnd-no-observation-plan-writes-nothing-silently — highest, mis-attribution class

+SURE. A plan publishes a receipt only when `receipt_eligible` — only for
`Authorised::Admitted` (`engine.rs:1394-1403`, `1977-1979`). Every other run — no `--host` and no
`--results` (`main.rs:321-322` answers `NoObservation`), a framed stream with zero site records
(`plan/src/records.rs:1100`), any book with no probeable site — skips the publish block entirely.
Skipping is not refusing: no `durable-receipt-unwritten` fires, because publication was never
attempted.

Then bare `dorc why` "asks about the last run" (`lib.rs:672-674`), which the store resolves as its
newest document — a different run. Evidence case `review_evidence_why_last_answers_about_the_last_run`
captured the shape in full: the `other.sh` plan prints

```
dorc: run `dorc why` for the per-site cause-chains, or `dorc why other.sh:N` to query a source line
```

and the very next `dorc why` prints the earlier run's receipt (`opaque 1 source-path book.sh`), with
empty stderr. The tool's own push-surface advice leads to a wrong answer about a different book. In
the project's own ordering (`271:rul-sin-ordering`) a confidently mis-attributed answer is the worst
class of aid failure, and this one is reachable from the on-ramp world (USER_STORY stage 0/1: no
oracles, nothing probeable) where a new admin lives first.

~SUSPECT the intended fix is small: either publish a (thin) receipt for every completed run so
"last" means last, or make the plan seat say, in its own stderr, that this run wrote no receipt and
`why` will answer about an older one. Either closes the gap; the current state does neither.

### 30Rn:fnd-no-receipt-is-ignored-by-apply and 30Rn:fnd-receipt-file-selector-is-inert

+SURE, both. Two flags parse, are listed in the help page, and change nothing.

- `--no-receipt` under `apply`: `ship_consented_apply` (`main.rs:2439-2561`) never reads
  `args.no_receipt`; only the plan/round-trip route does (`main.rs:213`). The parser mode-gates the
  three root selectors to `why` (`lib.rs:1098-1111`) and gates nothing for `--no-receipt`. Evidence
  case `review_evidence_apply_honours_or_refuses_no_receipt`: `dorc apply --host … --plan plan.sh
  --no-receipt` exits 0 and the store holds an `apply-intent-v1-…` and an `apply-outcome-v1-…`.
  The design rules no bypass for V1 (`30R:publication-and-dispatch-boundary`); a ruled no-bypass
  should be a loud `cli-flag-requires-mode`-style refusal, not silence. Silence is the one option
  the CLI's own discipline (`30Rk:a-known-flag-suggests-nothing`, `invocation-errors-are-registry-codes`)
  forbids.
- `--receipt <file>`: `ReceiptRoot::File` answers `false` for every stored entry (`engine.rs:2154-2160`),
  and `read_receipt_store` (`main.rs:474-525`) walks the store and never opens the named path.
  Evidence case `review_evidence_why_receipt_file_opens_the_named_file`: naming the store's own file
  by absolute path yields empty stdout and `warning[durable-receipt-unreadable]` about the store the
  user did not ask about. The only test touching the flag (`main.rs:3596-3607`) pins that it parses.
  `30R:receipt-rooted-attention-and-cli` describes this selector as the report-only root for a
  receipt outside any store; nothing implements that.

### 30Rn:fnd-apply-is-gated-on-a-per-user-profile

+SURE on the mechanism; ~SUSPECT on how often it bites real deployments.

`dorc apply` refuses (`apply-plan-not-dispatchable`, reason `intent-not-published`) whenever the
durable edge cannot open for write: no resolvable configuration/state root (`durable.rs:74-104` —
no fallback to cwd, temp, or the other role; `HOME` unset on Unix, `%APPDATA%` unset on Windows),
an unwritable root, or a keyset that cannot be created (`main.rs:2489-2497`). `dorc plan` in the same
environment proceeds and warns (`durable_route::a_run_with_no_clock_publishes_nothing_and_says_so`
pins the plan side; `an_apply_that_cannot_publish_its_intent_never_reaches_the_transport` pins the
apply side).

The environments where a per-user profile is absent or read-only are the ones USER_STORY sells
into: cron/systemd drift monitoring (`--exit-code`), the `dorc-run` shebang under chezmoi, CI
containers. In each, the first `dorc apply` now fails on the controller before touching a host,
for a reason that has nothing to do with the book, with `[unwritten:]` prose, and with the one
obvious flag (`--no-receipt`) silently ignored (previous finding). DESIGN's off-ramp ("if Dorc
fails you… `ssh host 'dash -s' <book`") is intact, but "Dorc refuses to run your plan until it can
write a key file in `~/.config`" is a new friction class the human-written roots never priced.

Adjacent, same seat: a first `dorc plan` creates three private key files under
`<config>/dorc/receipt-keys-v1/keyset-v1/` and a store under `<state>/dorc/receipts-v1/` and says
nothing (evidence case `review_evidence_first_run_announces_what_it_created`; the plan's stderr
names neither path). USER_STORY: "no second state database appears. Dorc remembers nothing." The
receipt store is report-only, so the *decision* promise holds; the *side-effect* promise ("nothing
you had to set up beforehand") now has an undisclosed first-run footprint.

### 30Rn:fnd-help-describes-the-previous-durable

+SURE. `crates/aid/tests/cli-help-page.loom` at the tip:

- "receipts -- every plan/apply/round-trip writes one" — false for every `NoObservation` plan
  (finding 2).
- "Holds: book path + digest, oracles, flags, probe records, per-line disposition -- never book
  text, never a command's output" — false: rich receipts carry the exact bytes of every acquired
  general-sh source (`30R:receipt-species-and-correlation`; `19cb0bc8`; the listing above prints
  `opaque 1 source-content #!/bin/sh…`; `recorded_facts_route.rs:194-200` asserts
  `MaterialState::Held` for the book).
- Five option rows (`--receipts`, `--no-receipt`, `--receipt-last`, `--receipt-id`, `--receipt`)
  and two codes (`durable-receipt-unwritten`, `durable-receipt-unreadable`) render `[unwritten:]`.

`30Rk` lists the seven unwritten registers as owed prose and does not list the two sentences that
are now wrong. Under the project's authorship law an AI may not rewrite those sentences, but it
must flag them; the arc shipped a help page that contradicts its own durable and left no note.

### 30Rn:fnd-volume-versus-delivered-surface

+SURE on the numbers; the judgement is mine.

| | lines |
|---|---|
| deleted: `plan/src/whylog.rs` + `cli/src/whylog_store.rs` | 2,541 + 466 |
| added source: `receipt/src` · `receipt-local/src` · `receipt-crypto/src` | 16,280 · 5,055 · 806 |
| added source in `cli` for the durable (`durable`, `receipt_edge`, `recorded`, `apply`, `recorded_facts`) | ~2,850 |
| added tests across the three crates + `cli` routes | ~12,000 |
| `Cargo.lock` package entries added / removed | 159 / 13 |

Roughly seven times the code, ~160 new packages (`age` alone pulls `fluent`/`i18n-embed`/`rust-embed`,
`nom`, `scrypt`/`pbkdf2`, `p256`, `ml-kem`, `hpke`, `futures`), a widened duplicate-version carve in
`clippy.toml` (`26c0252e`), and the duplicated `sha2`/`digest`/`thiserror`/`block-buffer`/`const-oid`
families in the lock — for a durable whose shipped read surface shrank. Much of the *shape* is
human-ruled in `30R` (one grammar, rich/plain projections, the reverse overlay, the apply-image
container), so the shape is not this review's to fault. What is faultable is the share of the
volume that serves nothing reachable: the `report` module and its plumbing (next finding), the
plain projection arm, and the rehydration/locator read side that only `report` consumes. Against
the project's own priority order (maintainability, then simplicity, then validation) this arc
spent heavily on the third at the expense of the first two.

### 30Rn:fnd-unreachable-production-code

+SURE for each item (checked by caller grep at the tip):

- `dorc_receipt::report::*` (`report.rs`, `report/{build,address,states,value}.rs`) and
  `cli::recorded_facts::facts_for`: no production caller.
- The plain projection: `receipt_edge::publish_plan_receipt` (plain) and the three
  `place_plain_*` methods on the production `StorePlacement` (`durable.rs:491-539`) have no caller
  outside `receipt_route.rs`; production always takes the rich arm (`main.rs:427`). Per `30R` V1
  ships rich only, so the dead arm is by design — but then the trait need not demand it of the
  production placement.
- `core::spine::InvocationMode` is a one-variant enum (`Unstated`) whose accessor
  `SpineInvocation::mode()` has no caller in the workspace (`30Rk` says so; confirmed). A one-variant
  enum kept "for a later, reviewed, one-arm widening" is dead code with a doc-comment.
- `PublicationGrade` has three variants; the production placement stamps `Synchronized`
  unconditionally (`durable.rs:474`), including on Windows where the store's own typed answer is
  `DirectorySync::UnavailableOnPlatform` (`store.rs:104-118`). The value is "discarded on purpose
  TODAY" (`engine.rs:2102`), so nothing renders it yet; the day something does, Windows will report
  a synchronization it never performed. The typed baseline machinery in `store.rs` exists precisely
  to prevent this claim, and the composition root bypasses it.
- `ReadEdge::read_plan` / `read_intent` / `read_outcome` (`durable.rs:263-314`) are the same body
  three times with a type parameter; `ingest_recognized` in `main.rs` repeats the species match a
  fourth time.

### 30Rn:fnd-stale-citations-to-deleted-modules

+SURE. The tree at the tip cites `plan::whylog`, `whylog_store`, `ACCOUNT_EXPORT`, `--whylog`,
`--no-whylog` — all deleted in this range — as live:

- `spike/CLAUDE.md:477` (`influence-is-carried-by-the-object`: "switched OFF at
  `plan::whylog::ACCOUNT_EXPORT`… `ExcludedContent::InfluenceGrade` is held by that switch") — the
  switch no longer exists; the invariant text now points an agent at a constant it cannot find.
- `spike/CLAUDE.md:837` (`whylog-write-only-replay` describes the deleted durable and
  `dorc why --last`) and `:571` (`probe-tape-not-a-cache` likewise).
- `crates/core/CLAUDE.md:182` (`plan::whylog::DurableAccount`).
- `crates/core/src/spine.rs:5,24,31,214,358-359` (the `.whylog` durable, `plan::whylog`'s
  `DurableView`, `ACCOUNT_EXPORT is false`).
- `crates/plan/src/spine.rs:771` (`plan/src/whylog.rs`).
- `crates/cli/src/artifact_store.rs:14,174,221` ("`whylog_store` … is the reference here").
- `crates/cli/src/engine.rs:120`, `main.rs:3006` (`--no-whylog`), `lib.rs:1288` (`--whylog`).
- `crates/receipt-crypto/Cargo.toml:11-13` and `crates/cli/tests/receipt_route.rs:19-20`: "the
  shipped binary cannot sign a document" — false since `40977599` (`durable_route.rs` proves the
  opposite in a subprocess).

`30Rk:steering-lines-that-name-the-deleted-module` names five of these and leaves them for "the
conductor"; the tip commit `4414af7a` "Tighten the comments this lane added" did not touch them.
IMPLEMENTATION.md's whole agent strategy is "extremely localize invariants"; steering that cites a
deleted switch as the thing holding an invariant is the failure mode that strategy exists to
prevent.

### 30Rn:fnd-harness-residue-and-lexical-fences

+SURE on the residue's existence; ~SUSPECT on which runs produced it.

- The suite wrote real keysets and receipts into the developer's profile for part of the arc:
  `durable_route.rs:526-536` says the corpus harness "minted a real keyset in the runner's
  profile" and the differential sweep "pointed neither" root, "found twice by hand". Sandboxing
  was retrofitted (`a75f992f`, `6e95bd06`, `hostsim/differential.rs:sandbox_profile`). On this
  machine `%LOCALAPPDATA%\dorc\receipts-v1\` exists (dated 2026-08-31) beside twenty-five
  old-format `whylog-NNNN.txt` files from 2026-08-28 that no binary can now read. Nothing in-tree
  inventories or mentions this (`mise run doctor` lists lane caches, not the product profile), and
  the old-format files are dead by design (`rul-strawman-formats-no-compat`) with no note to the
  human that they exist.
- The retrofit is a lexical census: `every_seat_that_drives_the_binary_sandboxes_the_profile_it_writes_into`
  greps every `.rs` under `crates/` for three spawn spellings and requires a sandbox call within a
  1,200-byte window. Alongside: `durable.rs:656` parses its own source with `include_str!` to assert
  no `DORC_*` variable is read; `receipt/tests/crate_boundary.rs` (927 lines) and
  `receipt-local/tests/crate_fences.rs` (309 lines) are source-text fences; 43 fence-shaped lines
  were added in the range. These are the grep-shaped gates the project has learned to distrust —
  an agent edits both sides. The sandbox census has a real motivation; the crate fences duplicate
  what `Cargo.toml` dependency direction already enforces at compile time.

### 30Rn:fnd-store-lifecycle-edges

+SURE on the code paths; the timelines are arithmetic.

- The store walk is bounded at 4,096 entries (`receipt-local/src/limits.rs:40`); past it
  `enumerate` answers `EnumerateFailure::OverEntryBound` (`store.rs:1032-1034`), and
  `read_receipt_store` maps EVERY enumerate failure to the one word `walk-failed`
  (`main.rs:482-484`), so the user sees `durable-receipt-unreadable walk-failed` with no hint that
  the cure is deleting files in a directory nothing told them about. Publication does not
  enumerate (`store.rs` has one `enumerate` seat, the read side), so writes continue past the bound:
  the store keeps growing while `why` is permanently dead. With receipts default-on per run and no
  retention (`30R:receipt-store-contract` rules none), the cron drift-monitor USER_STORY describes
  (`dorc plan --exit-code` every five minutes) reaches the bound in about two weeks; a daily
  `dorc-run` reaches it in eleven years. The design accepted "no automatic cleanup"; it did not
  accept "the reader dies first and says `walk-failed`".
- `remove_owned` is `IoFault::Unavailable` on every platform (`io.rs:603-606`), so every interrupted
  publication strands a partial file forever; partials count against the same 4,096 and surface in
  `why` only as `unread` entries with no listing line.
- The store's `PublicationGrade` misstamp on Windows is in the previous finding; the typed
  `PlatformBaseline` split itself is well done and the Windows leg is green (all
  `receipt-local` native tests pass on this Windows host at the tip).

### 30Rn:fnd-receipts-are-legible-only-where-written

+SURE on the mechanism. The readable skeleton carries digests, counts, and closed tokens
(`plan-plain-render-axes.skeleton` is the whole vocabulary); everything a person would read —
argv, target, paths, source bytes, the shell of each site — is in the armored overlay. `cat` yields
nothing; `--receipt <file>` is inert (finding 3); there is no export and no key import
(`30R:provider-and-storage-location`: "later"). So a receipt is legible only on the machine that
wrote it, by the binary that wrote it, through a listing. The `recorded.rs` header imagines "a
drifted tree, an old binary, a vendor handoff" as the durable's reason to exist; none of the three
is served in V1. The human-ruled line "directly readable structural content remains a product
goal" is met in letter — the structure is readable — and not in effect. This is a consequence of
the ruled shape, recorded here as a consequence, not as a fault with the ruling.

### 30Rn:fnd-small-nits-worth-a-line

- The CLI vocabulary puts `--receipt <file>` one character from `--receipts <folder>` with
  unrelated meanings; `68b8af57` fixed the did-you-mean loop this caused, which is the symptom, not
  the cause.
- `dorc why --receipt-last` and bare `dorc why` are the same selector; the flag exists to be
  spelled, which is fine, but the help page will need to say so and currently says `[unwritten:]`.
- No committed transcript in the corpus shows a receipt-rooted `why` (grep for `signing-key ` /
  `opaque ` across every `.loom`/`expected.out`: none). The one `run: round-trip` case that drives
  it (`durable-receipt-ambiguous.loom`) redirects stdout to `/dev/null`. The listing's shape is
  therefore pinned only by substring assertions in `durable_route.rs`, and can drift without any
  transcript reddening.
- `dorc-loom`'s `publish_receipt` returns `Ok(None)` (`consumer.rs:1765-1773`), so the loom corpus
  — the project's authoritative e2e surface — never exercises receipt projection or serialization.

## did-not-hold — suspicions checked and dropped

- **DST discipline / dependency direction.** `dorc-receipt` depends on `sha2` alone; `dorc-plan`
  depends on the pure crate only; `dorc-receipt-crypto` and `dorc-receipt-local` are reachable only
  from `dorc-cli`; every filesystem/clock/randomness act in the local edge goes through the sealed
  `LocalIo` vocabulary with a deterministic model beside the native one; OS entropy enters at two
  named `cli` seats. Held, and better than the average crate in the tree.
- **Receipts feeding decisions.** No path from `Reingested<…>` to `PlanAuthority`, a licence, or
  an apply input; `probe-tape-not-a-cache` holds. Held.
- **`dorc plan` blocked by an unwritable store.** It is not: plan proceeds and reports
  `durable-receipt-unwritten` (pinned). Only `apply` blocks (finding 4).
- **Merge integration defects.** `f98f65a7` and `34e44ae2` applied exactly what they imported;
  `8905a3f7` differs from a clean import by two files (`apply-plan-not-dispatchable.loom`,
  `receipt_route.rs`) and small doc/loom line-count deltas consistent with conflict resolution in
  files both sides touched. ~SUSPECT nothing evil; no code-level integration defect found.
- **Comment bloat.** Comment density of the new source is 24-33% (`receipt/src` 24%,
  `receipt-local/src` 33%, the `cli` durable seats 28%) against 27% for the pre-existing `plan`
  crate. Not an outlier for this codebase.
- **The undated refusal.** A clockless run refuses placement rather than sorting an undated
  document below every dated one (`durable.rs:450-458`, pinned through the binary). Sound.
- **`--receipts` orthogonality.** The folder is the store root exactly, keys stay under the standard
  configuration root, two stores never cross-read, `why` never creates a named folder. All pinned
  through the binary and green here.
- **Windows leg.** The three `(AI fix Win)` commits and the typed `PlatformBaseline` show the leg was
  exercised; `receipt-local`'s native batteries pass on this Windows host at the tip.

## evidence

Focused runs on this Windows host at `4414af7a` (plus the evidence file), via
`mise exec -- cargo nextest run …` from `spike/` because no task selects individual targets:

- `-p dorc-cli --test durable_route --test recorded_facts_route -p dorc-receipt-local --test
  native_store --test native_keyset --test store_sweep`: 58/58 passed.
- `-p dorc-cli --test r30_review_evidence`: 0/5 passed, each red on the assertion its finding
  predicts:
  - `review_evidence_apply_honours_or_refuses_no_receipt` — store holds an intent and an outcome,
    exit 0.
  - `review_evidence_why_receipt_file_opens_the_named_file` — empty stdout,
    `warning[durable-receipt-unreadable]`.
  - `review_evidence_why_address_changes_the_recorded_answer` — identical bytes with and without
    `book.sh:2`.
  - `review_evidence_why_last_answers_about_the_last_run` — the `other.sh` plan advises
    `dorc why other.sh:N`; the next `why` lists `source-path book.sh`, stderr empty.
  - `review_evidence_first_run_announces_what_it_created` — both roots created, stderr names
    neither.

The evidence file is registered as a `[[test]]` in `crates/cli/Cargo.toml`. It is meant to be
turned green by the repairs or deleted with them; it is not a regression suite.

No `gate*`, `both`, bless, Lean, or Kani lane was run. No product code was changed.

## handoffs

- The real profile on this machine holds `%LOCALAPPDATA%\dorc\receipts-v1\` and twenty-five
  unreadable `whylog-*.txt` files; the corresponding keyset under `%APPDATA%\dorc\` was not
  inspected. Cleanup is the human's.
- The five `[unwritten:]` option rows and two codes, plus the two now-false help sentences, are
  prose the project's law reserves for a human.
