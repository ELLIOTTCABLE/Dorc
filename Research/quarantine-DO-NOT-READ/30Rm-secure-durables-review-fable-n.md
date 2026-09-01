# 30Rm — review of the landed secure-durable receipt implementation (reviewer lane N)

> Tier: independent review record. Range `7693ac6f..4414af7a`, first-parent line of
> `ai/r30-receipt` (230 commits, ~48k insertions of the lane's own; the five merges imported
> ~12.5k). Security is entirely outside this review by brief; nothing below assesses a
> security property, and no quarantined material was read. Reviewer: Fable, on branch
> `ai/r30-secure-durables-fable-n`; scouts were read-only Sonnet lanes gathering facts.
> Confidence marks: +SURE / ~SUSPECT / -GUESS.

## the-verdict-in-one-screen

The receipt family is unusually well-engineered as a *format and a store*: one table-driven
grammar both writer and reader consult, a monotone reader, a `Reingested` seal with no exit,
an affine publication chain, an immutable store with exclusive create and a bounded walk, a
deterministic I/O model driven by exhaustive fault sweeps, two-way-bound vector corpora, and a
cross-process battery that spawns the shipped binary into a sandboxed profile and asserts on
what it printed. Windows `cargo check --all-targets` is clean; every focused suite passes.

What it is not, today, is an explanation. The cutover (D5) deleted the only posthoc `why`
that explained, before the replacement explains anything: a bare `dorc why` — the invocation
every `plan` run's own stderr tells the admin to type — now prints a hex-and-token record
listing, ignores a `book.sh:N` address entirely, and the `--receipt <file>` selector the help
page lists reads no file. The `apply --host` route swallows post-dispatch durable failures
and never names the receipts it wrote. An interrupted publication leaves a prefix file that
nothing can remove and that turns every later `--receipt-last` into a wrong-worded refusal.
`mise run livetest` is broken by the flag rename. The tree's own steering law, root
registries, help page, and code comments still describe the deleted durable in dozens of
places.

The format work is high-lock and worth keeping. The product-facing half — what the admin
sees at "the moment they are angriest" (USER_STORY, Recovery) — regressed, and the lane's own
ledger says so in one line (`30Rk:the-recorded-why-surface-is-the-next-round`) without
weighing it as a regression. This review weighs it as the top finding.

## findings, in importance order

### fnd-default-why-regressed-to-a-record-listing

+SURE. At the base commit, `dorc why 10 --last --whylog-dir=…` rendered the flagship chain:
an OUTCOME block, an ANALYSIS block with `reported` / `vouches` / `claims` / `derives` links,
and a NEXT STEPS block naming the suspect link, the fix, and the verify command (retired
transcript: `git show 7693ac6f:spike/crates/cli/tests/whygallery-survive-trusted-footprint.loom`,
second replay block). That is the USER_STORY "Recovery" promise, built.

At the tip, the same question — through the shipped binary in a sandboxed profile — prints:

```
receipt 244e5be9…6130d
signing-key 0ed7e65c…dbb8b
records 14
source 0 book 6b796e0c…9a79e 34
sites 1
site run host-influenced
regions 0
presented-plan 06df1751…3eb0
opaque 0 target-name localhost
opaque 1 source-path book.sh
opaque 1 source-content #!/bin/sh\x0ahork tune --profile web\x0a
opaque 4 site-locator dorc-receipt-locator/1\x0astages 1\x0ahead 0\x0astage authored 0 10 33 none 0 -\x0a\x0alocator-end\x0a
opaque 4 shell hork tune --profile web
```

Three aggravations beyond the listing itself:

- It is the DEFAULT. `Args::reads_the_receipt` (`cli/src/lib.rs:732`) routes a bare `dorc why`
  and `dorc why book.sh:N` to the store unless `--results` is named; that default predates
  the lane, but at the base it replayed into the chain and now it lands here. Every `plan`
  prints `dorc: run \`dorc why\` for the per-site cause-chains, or \`dorc why book.sh:N\``
  on stderr, pointing the admin at this output.
- The address is silently ignored. `engine::report_recorded_store` (`cli/src/engine.rs:2173`)
  takes no address; `dorc why book.sh:2` printed byte-identical output to `dorc why`
  (reproduction below). The lane built `report::address::resolve` and `RecordedWhyFacts`
  to answer exactly that question, and wired neither: `recorded_facts::facts_for`
  (`cli/src/recorded_facts.rs:70`) has no caller outside `cli/tests/recorded_facts_route.rs`.
- The listing breaches the project's own `AID-NEEDS:law-plain-language-surfaces` (no jargon
  on user-facing surfaces): raw digests, `host-influenced`, a binary locator payload and a
  whole source file rendered as `\x0a`-escaped text through the 240-byte terminal cap
  (`cli/src/recorded.rs:112-133`, `EncodedStructure` takes the text encoder).

The design accepted no coexistence (`30R:v1-spike-scope-and-exit`: "temporary coexistence
… is not a product state"), so the deletion is by design; the sequencing is what this review
disputes. `30R:implementation-direction-and-order` places "route … through `dorc why`" at
step 5 and the deletion at step 9. What shipped is step 9 with step 5's render missing. The
retired `whylog-unwritten` message was `ProseTier::Slop` (agent prose), so no human prose was
lost — but the replacement codes (`durable-receipt-unwritten`, `-unreadable`, `-ambiguous`,
`apply-plan-not-dispatchable`) all render `[unwritten:]`, and five help rows do too.

User-visible consequence: on the worst morning the durable exists for, the admin types the
command the tool told them to, and gets a record dump with no line addressed, no chain, no
next step. Recommendation: before this counts as "one live durable implementation", either
wire `facts_for` into `report_recorded_store` so the address resolves and the four
recorded-vs-current and partial states render (the model is built and sealed; only the join
is missing), or make the default posthoc `why` say plainly that the explanation is pending
and put the record dump behind an explicit spelling. Emitting the dump as the answer is the
one option `law-selection-is-goal-derived` refuses ("curating WRONG is the one forbidden
thing").

### fnd-explicit-receipt-file-selector-does-nothing

+SURE. `--receipt <file>` is parsed (`lib.rs:954-957`), listed on the help page, has a
`ReceiptRoot::File` arm whose doc says "an explicit file is admitted by PATH at the edge"
(`engine.rs:2150-2153`), and is pinned mutually-exclusive with its siblings. No edge admits
it. `main.rs::run_analysis` (`main.rs:187-202`) calls `read_receipt_store(edge)` for every
selector and never touches `args.receipt_file`; `ReceiptRoot::File::takes` answers `false`
for every stored entry, so the output is the graph listing or, on an otherwise-empty store,
`durable-receipt-unreadable`. Reproduced against a real receipt the binary had just written,
with and without `--receipts <its store>`: stdout empty, stderr
`receipt: warning[durable-receipt-unreadable]`. No test drives the flag (scout census; the
cross-process battery covers `--receipt-last`, `--receipt-id`, `--receipts`, `--no-receipt`).
A red test is committed beside this report (`cli/tests/review_red_evidence.rs`).

`rul-strawman-formats-no-compat` says no dead surface: either read the file (bound → locate →
`read_plan` under the local keyset → same listing seat) or drop the flag and its help row
until it exists.

### fnd-apply-route-swallows-durable-failure-and-never-names-its-receipts

+SURE. `cli/src/apply.rs::published_route` returns `ConsentedApply { shipped, intent,
outcome, durable_failure }` and documents `durable_failure` as "narration". The one
production consumer, `main.rs::ship_consented_apply` (`main.rs:2510-2560`), matches only on
`reached.shipped`; `intent`, `outcome` and `durable_failure` are dropped on the floor. So
after a real `dorc apply --host`, the operator is never told which intent/outcome documents
were written, and an outcome that failed to land (the exact case `30R:publication-and-dispatch-boundary`
says "remains reported") is silent. This is the opposite posture from the plan route, whose
`durable-receipt-unwritten` is error-floor precisely because "a durable that silently vanished
is exactly the run somebody comes back asking about" (the code's own loom).

Compounding it: every pre-dispatch refusal on the apply route collapses to one word.
`production_receipt_edge(args).map_err(|_| intent_not_published())` and
`open_for_write(..).map_err(|_| intent_not_published())` (`main.rs:2492-2497`) discard the
edge's closed refusal word (`no-controller-root`, `store-permission-refused`,
`keyset-missing-with-existing-store`, …), so `apply-plan-not-dispatchable` carries
`{reason}=intent-not-published` and no `{store}`, where the plan route carries both.

Also in this seat: `apply.rs:478` constructs `PostDispatchFailure::DurableOnly(f)` and
immediately narrows it with `.durable_only()`; the `None` arm is unreachable by construction.
The comment says the narrowing "proves" an integrity failure cannot reach the continuation,
but no integrity failure is constructible on that path — this is type theatre, not a proof.

### fnd-interrupted-publication-bricks-last-selection

+SURE, reproduced. Publication creates the final name directly and writes into it
(`receipt-local/src/store.rs:953-985`), so an interrupted write leaves a prefix under a
valid receipt name. `remove_owned` is the crate's one removal and it always answers
`CleanupFailure::Unavailable` (`native.rs:204-212`, documented as "a DEFECT that is being
carried"); nothing calls it anyway. The prefix therefore stays forever, and because
`maximum_order_cohort` selects by FILENAME (`store.rs:703`), it is the newest document. Then:
`main.rs::ingest_recognized` maps every read failure to `RecordedDocument::unread(id)` and
discards the `PartialReceipt` (`Err(_) => None`); `graph.ingest_partial` has no production
caller, so the `partial …` line `recorded_graph_listing` can print is dead in production;
`report_recorded_store` finds no listing for the terminal member and answers
`durable-receipt-unreadable` with reason `no-receipt` (`engine.rs:2208-2210`).

Reproduced: two plans into a sandboxed store, newest truncated to 200 bytes →
`dorc why --receipt-last` prints nothing and warns `durable-receipt-unreadable`; the intact
older document is reachable only by `--receipt-id <hex read off a filename>`. The reason
word is wrong (the store holds a complete document), no finding names the damaged file, and
the tool "removes and repairs nothing" by policy — so the admin has no path but a directory
listing. The never-fall-back rule is sound; what is missing is the *report*: ingest the
partial with its reason, distinguish `no-receipt` from `newest-damaged`, and name the file.

### fnd-livetest-passes-a-retired-flag

+SURE, reproduced. `spike/e2e/livetest.sh:246` and `:257` pass `--no-whylog` to the binary.
The parser knows only `--no-receipt` (`lib.rs:960`); the built binary answers
`dorc: error[cli-unknown-flag]: Dorc does not recognize the flag \`--no-whylog\``, exit 2.
The lane edited this file (a comment about receipt publication was added above
`apply_from`) and left the invocation. CONTRIBUTING.md's live-machine acceptance loop
(`mise run livetest`) cannot run.

### fnd-fixture-clock-reaches-published-order

+SURE on mechanism; ~SUSPECT on whether it was weighed. `main.rs::clock_for_invocation`
(`main.rs:1211-1224`) honours `DORC_FIXTURE_CLOCK_MS` in every build — unlike `DORC_TRANSPORT`,
which is `cfg!(debug_assertions)`-gated (`transport_edge.rs:86`). The lane routed the
store-selection order token through that clock (`receipt_edge.rs::RunClockOrder`), so in a
release binary a stray harness variable pins every receipt's order (two runs at one order =
`durable-receipt-ambiguous` forever) and a malformed value makes the run clockless, which the
lane's own composition root then refuses as `undated` — no receipt written. The tree's own
`rul-fixture-identity-never-production` names "default persistence, or anything published"
as the boundary a fixture must be structurally unable to reach. The pin predates the lane;
the reach into published durables does not. Gate it as the transport pin is gated.

### fnd-argv-uncollected-drops-the-consent-link

+SURE. Both projections mark `argv` `uncollected` (`plan/src/receipt.rs:370`,
`receipt/src/project.rs:192`), by deliberate deferral ("a durable rendering that is not
designed"). The old whylog recorded `argv value=--risk-faultless-skips`. Consequence: no
receipt records whether `--risk-faultless-skips` was set, so the "consented" link — link 6 of
the USER_STORY chain, in the one corner the design admits is naked trust — cannot be
rendered posthoc from a receipt at all. The survival rows imply the flag only when a
survival happened. This is a content gap the previous durable did not have; whatever the
right rendering is, the flag is a controller-authored boolean, not host bytes.

### fnd-skeleton-scalars-fabricated-not-absent

+SURE. The grammar offers `absent` for `OptionalWide`/`OptionalCount` and the projection
uses `uncollected` for opaque slots, but three skeleton scalars are `Wide`/`Count` and are
written as measurements they are not: `admission … bytes=0` is hardcoded
(`plan/src/receipt.rs:480`); `admission … records=` counts *timed instants*, so a clockless
(loom) run records `records=0` whatever was admitted (`receipt.rs:479`, and the grammar names
the field `records`); `narrative … operands=0 dropped=0` is emitted for 12 of 14 kinds that
carry no operand count (`receipt.rs:755-779`). A reader of the "directly readable skeleton"
sees zeros indistinguishable from measured zeros. `30R:standing-invariants` ("missing …
never reads complete") and `30R:canonical-readable-envelope` ("one representation for every
scalar") both point the same way: make them optional and write `absent`. Also `render_row`
maps any import verb other than `"inlined"` to `import-repointed` by string comparison
(`receipt.rs:714-720`), where the sibling `certification_row` refuses an unknown token.

### fnd-help-page-misdescribes-the-durable

+SURE; human-owned prose, so this is a flag-up, not an edit. `cli-help-page.loom:101-108`
says the receipt holds "never book text, never a command's output" and is kept at
"$XDG_STATE_HOME/dorc … unix writes it 0600 in a 0700 directory". At the tip a rich plan
receipt carries every general-sh source's exact bytes (`SourceContent`, populated at
`plan/src/receipt.rs:391-397`; visible in the reproduction as `opaque 1 source-content …`),
the `site-outcome` kind provisions `stdout`/`stderr` slots (unpopulated in production today),
and the layout is two roles under two roots (keys under the configuration root at
`dorc/receipt-keys-v1/keyset-v1/`, documents under the state root at `dorc/receipts-v1/`).
Five new option rows render `[unwritten:]`. The paragraph describes the deleted durable.

### fnd-dead-contracts-and-stale-comments

+SURE, each verified against the tree:

- `EntropyReceiptIds::intact()` (`receipt/src/ids.rs:166`): the type's doc says "a caller
  checks it before spending anything an identity reached"; zero callers anywhere. Production
  mints identities through it (`main.rs:422`, `:2488`) and never asks.
- `PartialReceipt::with_structure` (`reader.rs:263`): zero callers, including tests.
- `publish_plan_receipt`, `publish_plain_apply_intent`, `publish_plain_apply_outcome`
  (`receipt_edge.rs:597/711/777`) and the three `place_plain_*` methods: test-only. The ruled
  V1 policy is "no automatic rich-to-plain fallback" and "V1 does not include alternative
  provider/configuration surfaces" (`30R`), so this is a shipped API for a route the design
  excludes. `narrow_and_sign` also spends two identities per plain document (one for
  scaffolding it discards).
- `receipt_edge.rs:383-384`: "The seat is `LocalReceiptEdgeV1`, which does not exist yet;
  nothing in the binary publishes today" — false; it is `durable.rs:153` and `main.rs:407`
  publishes through it.
- `receipt-crypto/Cargo.toml:12-13` and `receipt/tests/crate_boundary.rs:21-24`: "DEV-ONLY
  … the shipped binary cannot sign a document" — false; `cli/Cargo.toml:53` names it under
  `[dependencies]` with a comment saying the opposite, and the boundary test checks only the
  manifest allow-list, not the dependency section.
- `cli/src/main.rs:3006` test doc names `--no-whylog`; `cli/src/artifact_store.rs:14/174/221`
  cite `whylog_store` as "the reference here" for a module that no longer exists;
  `plan/src/spine.rs:771` enumerates `plan/src/whylog.rs`; `core/src/spine.rs` cites
  `plan::whylog::ACCOUNT_EXPORT` six times.

### fnd-steering-and-registry-drift-names-the-deleted-durable

+SURE (scout census, spot-verified). Builders read these as law:

- `spike/CLAUDE.md`: `whylog-write-only-replay` (`:837-841`) describes the retired durable
  as live; `influence-is-carried-by-the-object` (`:477`) cites `plan::whylog::ACCOUNT_EXPORT`;
  `rul-durable-contents-reviewed-before-design` (`:453`) and
  `inv-debugging-detail-has-no-sensitivity-guarantee` (`:451`) name "the whylog";
  `rul-strawman-formats-no-compat` (`:665`) lists `dorc-whylog/1` as a live example;
  `rul-chain-is-pull-only` (`:904-906`) spells `--last`.
- `cli/CLAUDE.md:51/154/185/206` ("whylog writing"); `core/CLAUDE.md:182`
  (`plan::whylog::DurableAccount`); `receipt/CLAUDE.md` lists a "trust" reader state that no
  longer exists and `receipt-local/CLAUDE.md`'s cleanup clause is stale (both named by the
  lane's own `30Rk` and left).
- `KNOBS.md:315` lists whylog as a current aid category; `USER_STORY.md:913-914,1099` spell
  `dorc why --last`; `TODO-ADDTL.md:78,82` carry whylog items; `catalog_lock.rs` `why:` rows
  for four live codes (`durable-receipt-unwritten` included) invoke "the whylog" as a live
  reproducer; `Research/plans/30R` itself still says "replace the current whylog format" and
  "stage extraction through existing `plan::whylog`" — a plans-tier document that AGENTS.md
  requires to be current-tense.
- The lane's ledger (`30Rk`) lists part of this as owed steering. The rest was not found by
  the lane.

### fnd-quarantined-rationale-cited-from-readable-code

~SUSPECT as to remedy; +SURE as to fact. 61 code comments across 43 non-quarantined files
cite `30Ra`/`30Rb`/`30Rc`/`30Rd`/`30Rh`/`30Ri` (25 of them `30Rd`), and none of those
resolves to a document a builder may read. The crate `CLAUDE.md`s restate the *rules*; the
*reasons* ("why is the locator span in the acquired-byte domain", "why is there no
`receipts-v1` component beneath a named folder") live only behind the fence. Every future
maintainer of ~27k lines is told, in the code, where the answer is and that they may not go
there. Where a reason is engineering rather than security, it can be restated in place.

### fnd-golden-rebless-inside-a-merge-commit

+SURE. Merge `8905a3f7` carried a re-bless of 44 pre-existing looms (the `decision-digest`
line 16→64 hex; values derivable from neither parent) together with substantial hand
integration: `ship_consented_apply` rewritten, `record_new_arm`'s classification loop and
`transport_edge::{ship_apply, apply_to_host}` deleted, the `30Mc F3` regression test moved.
The move was verified (`plan/src/settle.rs:1244`, test intact), and the other four merges
are clean auto-merges — so no integration defect was found. But a merge commit that is also
a bless and also a refactor is invisible to `--first-parent --no-merges` review and to
`bisect`; the human's ruling that goldens churn freely assumes the churn is inspectable at a
fold, which this shape defeats.

### fnd-real-profile-residue-and-orphan-whylogs

+SURE on facts (read-only listing). This machine's real profile holds
`%APPDATA%\dorc\receipt-keys-v1` and `%LOCALAPPDATA%\dorc\receipts-v1`, both created
2026-08-29 16:44:19, alongside ~60 `whylog-<pid>.txt` files from the retired durable
(2026-08-28/29). The lane's own test comment (`durable_route.rs:526-532`) records that an
earlier harness state "minted a real keyset in the runner's profile". Whether the human ran
the binary by hand that day is unknowable from the tree; the census that now prevents the
harness shape is good. Nothing in the product tells an operator the old `whylog-*.txt` files
are inert.

### fnd-dependency-weight-and-dual-versions

~SUSPECT as a concern; +SURE on numbers. 146 packages added, 0 removed; `deny.toml`
unchanged (`multiple-versions = "warn"`); nine names now at two versions (`sha2`, `digest`,
`block-buffer`, `crypto-common`, `const-oid`, `cpufeatures`, `hybrid-array`, `thiserror`,
`syn`). `age` alone pulls a 12-package `i18n-embed`/`fluent`/`rust-embed` localization stack
and nine `futures*` crates into a CLI that uses none of them. All of it sits outside the
kernel (`dorc-plan` depends on `dorc-receipt` and `sha2` only; the fence tests hold), which is
the rule that matters. Worth knowing when the next `cargo deny`/audit surface grows.

### fnd-lexical-fences-minted-in-bulk

Low. Roughly twenty grep-style fences across `receipt/tests/crate_boundary.rs`,
`receipt-local/tests/crate_fences.rs`, `durable_route.rs`, and `recorded_why_facts.rs`. They
are the strongest form (two-way, non-empty floors, several verified falsifiable by the
lane), and one of them caught the real-profile write above. Noted because the standing
preference is that agent-minted allow-list gates rarely earn their keep; these are better
than most, and still an agent editing both sides satisfies them.

### fnd-minor-render-and-vocabulary-warts

- `recorded.rs:93/179/203/236` print `{refusal:?}` / `{:?}` Debug output into user-facing
  lines (`model-unavailable …`, `partial …`).
- `apply-plan-not-dispatchable` carries a three-word reason enum of which two are
  "structurally unreachable" in the binary (its own loom says so), and its defining case
  rides the `dorc-loom --this defect` route with a comment claiming human authorization for
  a code that IS reachable in production over a live host. The tree cannot confirm the
  authorization; the comment itself says the code "is owed a split".
- `admission … stream=uncollected` and `record-stream` omission both exist for one thing.

## what held

- Format: `grammar.rs` is one field table consulted by writer and reader; canonical
  integers, closed tokens, fixed width order token; the reader is monotone
  (`bound → locate → check → parse → open → validate`) with no skip; `Reingested<T>` has
  no unwrap, deref, or generic accessor (compile-fail doctests pin each); recorded-vs-current
  is decided from the values, never chosen by a caller.
- Publication: the intent → accounting → publication → permit → spend chain is ownership,
  not checks; the bypass arm was removed before the tip; the plan route refuses an undated
  document at the one production seat.
- Store: exclusive create under the final name, no replace, no mutable pointer, bounded
  enumeration counting foreign entries, the one selection (maximum-order cohort) with
  ambiguity reported rather than tie-broken; read-only `why` creates nothing (verified
  through the binary).
- Tests: no `#[ignore]` or xfail in scope; every vector corpus bound two-way to a refusal
  table; fault sweeps over closed operation lists with coverage floors; the cross-process
  battery sandboxes the platform's own roots and asserts a stdout sentinel that only an
  opened region can produce; Windows `cargo check --workspace --all-targets` clean; 229 +
  105 + 37 + 17 + 36 focused tests pass, all compile-fail doctests included.
- Platform: every `#[cfg]`-gated item in `native.rs` has a same-named twin; the Windows
  baseline is stated as weaker rather than simulated (the Unix-only refusal tests have no
  Windows analogue by that design).
- Process: no AI trailers in any commit; gitlabels headliners throughout; granular history;
  the root registries (`AID-NEEDS`, `ANALYZER-NEEDS`, `KNOBS`, `TODO-ADDTL`) were updated
  in the same range; the lane's own residue ledger is candid about the render gap.

## concerns that did not survive

- "A merge dropped a regression test": the `30Mc F3` test was relocated to
  `plan/src/settle.rs:1244`, intact.
- "The lane authored a pre-commit hook side-quest": `.githooks/pre-commit`,
  `.pi/settings.json` and `precommit_gate.rs` were imported from `ai/main` via `f98f65a7`;
  the lane's own tooling contribution is `gate_floor.rs` plus three `precommit_gate.rs`
  fixes, all in scope of "fix the tooling".
- "Human prose was deleted with the whylog codes": the retired messages were
  `ProseTier::Slop`.
- "cfg-gated code rots on the other platform": census found every reachable item paired.
- "The store walk decrypts every document per `why`": true, and fine under the project's
  perf doctrine (controller-local, no network in the loop).

## reproduction

Sandboxed profile (the platform's own variables pointed at scratch, exactly as
`cli/tests/sandbox.rs` does), the built `dorc` at the tip, no host contact:

```sh
printf '#!/bin/sh\nhork tune --profile web\n' > book.sh
d=$(sha256sum book.sh | cut -d' ' -f1)
printf 'dorc-records/1 nonce=dorc attempt=1 host=localhost book=%s sites=1 @@dorc@@\ndorc site 0 effect=holds rc=0 @@dorc@@\ndorc-records-end/1 nonce=dorc @@dorc@@\n' "$d" > records.txt
dorc plan --book=book.sh --results - < records.txt >/dev/null   # twice
dorc why                        # the listing above; rc 0
dorc why book.sh:2              # byte-identical listing; the address is ignored
dorc why --receipt "$STORE/plan-v1-…-08e9d450….dorc-receipt"   # stdout empty; warning[durable-receipt-unreadable]
head -c 200 "$STORE/<newest>" > t; cat t > "$STORE/<newest>"      # an interrupted publication
dorc why --receipt-last         # stdout empty; warning[durable-receipt-unreadable]
dorc why --receipt-id 08e9d450… # the intact older document still answers
dorc plan --book=book.sh --no-whylog --results - < records.txt   # rc 2, cli-unknown-flag (livetest.sh:246)
dorc why book.sh:2 --book=book.sh --results - < records.txt      # the live route still renders OUTCOME/ANALYSIS
```

The on-disk document is the readable skeleton the design promised (14 `record` lines, then
the armored region, then one `signature` line); the `admission … records=1 bytes=0` row
shows the fabricated scalar.

## red evidence committed

`spike/crates/cli/tests/review_red_evidence.rs` (one `[[test]]` stanza in `cli/Cargo.toml`):
`an_explicit_receipt_file_is_read_and_answered_about` drives the shipped binary in a
sandboxed profile, publishes one receipt, then asks `dorc why --receipt <that file>` and
asserts the listing names it. RED at the tip by design (`fnd-explicit-receipt-file-selector-does-nothing`).
It follows the sandbox census's spelling so that fence stays green. Delete it with the fix
or the flag; it is evidence, not a regression guard.
