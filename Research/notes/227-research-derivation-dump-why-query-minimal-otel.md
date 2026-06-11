# 227 — research: derivation-dump + `why`-as-query, and minimal OpenTelemetry adoption

> Deep-research note serving round-22 fronts **rq-C** (derivation-dump + `why`-as-query)
> and **rq-D** (minimal OTel adoption), against the PROVISIONAL human leans d-1 and d-2.
> d-1: a DURABLE-PRODUCING dump mode (full derivation log per run) serving (a) postmortem
> and (b) a DST plugin-point (golden-TRACE fixtures for critical-tier logic, tiered so the
> rest pins only verdicts); `why` becomes a query over the dump — one producer, many lenses.
> d-2: the provenance durable wants LIGHT OTel — tracing + trace-propagation early
> (controller→host; verdict-lane as carrier candidate), "import the ideas, not the machinery".
>
> CRITICAL REFRAME (human, this gate): he leans AWAY from promising trace-stability upfront —
> suspects it is "making it easy on the test harness" not "benefiting the user", and wants a
> concrete user-story to sell the cost. So the golden-trace half of rq-C is NOT a confirmation
> exercise: hunt regret as hard as success; default no-promise.
>
> Does NOT re-cover (held by wider corpus, see 220/222 and plans/111): PROV vocabulary; OTel
> span-link/fork-join basics + Cramer's import-ideas-not-machinery critique; Puppet/K8s/Terraform
> reporting; SQL EXPLAIN data-shape; Soufflé/ProvSQL receipt internals; Bazel cache-trust.
> New ground: the DURABLE-DUMP economics, QUERY-UX in practice, golden-trace pinning regret,
> and tracecontext-WITHOUT-SDK propagation practice.
>
> Findings slugged `finding-N` / topical (`size-economics-1`, `regret-signal-2`); sources
> `[grade-slug-year]`, graded list at end. Confidence marks per project convention
> (+SURE / ~SUSPECT / -GUESS / --WONDER). Banks predecessor turn-01 (Buck2 log, Bazel #18643,
> bazel_conduit) and continues.

## §0 Conclusions up front

**Strongest evidence AGAINST golden-trace PINNING (default no-promise holds).**
`regret-against-1` (+SURE the *always-on full dump* is a known anti-pattern, first-party):
Bazel's own maintainer who shipped the compact exec log, tjgq, is on record (relayed by
brentleyjones) that he is "not sure it would ever be fine for this to always be collected by
default" [B-bazel-execlog-issue-2023]; sluongng, a heavy execlog user, "agree[s] that we
should not enable the compact log by default, at least not yet." The naive durable cost 99GB
uncompressed / 20GB zstd on one large build and made that build take **75 min vs 7 min (>10×)**
pre-optimization [B-bazel-execlog-issue-2023]. Even the *shipped* compact format only "aim[s]
for 2-3% at most" overhead and is explicitly "not designed with human readability in mind"
[B-bazel-execlog-issue-2023] — i.e. the consumer is always a tool, never an eyeball, which is
the capture-without-a-(human)-consumer hazard restated.
`regret-against-2` (~SUSPECT pinned derivations rot into ignored channels): SQL plan-baseline
practice — the closest decade-scale analog of "pin a golden derivation" — shows forced/pinned
plans silently failing-open and being abandoned [see plan-pinning section]; that is the rot
hazard d-1's golden-trace tier would inherit.

**Strongest evidence FOR a durable dump (but NOT for upfront trace-stability).**
`finding-for-1` (+SURE one-producer-many-lenses is proven and loved): Buck2's always-on
`buck2 log` is exactly d-1's architecture — a single per-invocation event log with ~15 query
lenses (`what-ran`, `what-failed`, `what-materialized`, `critical-path`, `replay`,
`diff action-divergence`, …) [B-buck2-log-2024]. The *query-over-one-durable* shape is not
speculative; it ships and is cited by Bazel users as the thing to copy [B-bazel-execlog-issue-2023].
The split that matters: the durable-dump idea is well-supported; the *upfront stability promise*
on its byte-format is the costly part everyone defers (Bazel kept the compact format
`experimental` across the entire 7.x line precisely to avoid that promise [B-bazel-execlog-issue-2023]).

**Best postmortem user-story found — and it argues for receipts, NOT for trace-pinning (§9).**
`userstory-1` (~SUSPECT): in the *build/derivation* register the recurring real use is
**non-determinism / unexpected-cache-miss hunting via dump-diff**, not free-form postmortem.
tbaing's motivating ask is verbatim "which actions needed a rebuild (rather than being reusable
from cache) and which dependencies drove that need" [B-bazel-execlog-issue-2023]; the
`coeuvre/bazel-find-non-deterministic-actions` tool and Buck2's `diff action-divergence` exist for
this one job; acecilia's BazelCon CI gate ("if the list of the cache misses is not a subset … the
pull request is introducing non-determinism") is the prevention version. The STRONGEST affirmative
story is the runtime-ops *silent-green-dashboard* failure class (§9 `userstory-2/3`): a cluster of
incident teardowns where every instrument was green while data was silently lost, cracked only by
*wider, timestamp-correlated, event/derivation-shaped* data — "Three independent, individually-
reasonable decisions … stack into one silent data shredder … The failure is emergent. That is why
it survives every test you have" [C-silentconsumer-2026]. That maps onto Dorc's own compositional-
elision risk (note 222 `c-2`) and is the best case for the durable — BUT it argues for the
per-value *receipts / why-query* (note 220) far more than for *golden-TRACE pinning*. A clean
"always-on log solved a production mystery nothing else could" narrative *for a static analyzer*
remains NOT found; the runtime analogs are graded C (vendor-adjacent, ops-not-analysis) — top open
thread. Net: d-1's two halves separate — the receipts/dump half has affirmative support; the
golden-trace-pinning half has cost evidence (§4, §6) but no affirmative user-story of its own.

**rq-D (minimal OTel) — what to keep, what to drop.**
`finding-otel-1` (+SURE the working minimal shape is: emit a neutral event stream, map to OTel
at the edge): bazel_conduit keeps the OTel *span model* + a semantic-convention-style attribute
namespace + W3C-ish trace identity, but as a downstream sidecar over Bazel's neutral BEP event
stream — it does NOT instrument Bazel in-process with the SDK [B-bazelconduit-readme-2026]. The
build tool emits neutral events; OTel is a projection. Direct architecture lean for Dorc.
`finding-otel-2` (+SURE hand-rolling span emission has a sharp, enumerated pitfall set, several
of which are exactly Dorc's determinism seams): stale clocks (server-start time leaking in as
span start), epoch-1970 from `start_time=0`+duration, clock-skew negative durations,
ID-derivation-from-UUID, and silent span-drop past a queue bound [B-bazelconduit-readme-2026].
Every one is a clock/RNG/ordering concern → maps onto Dorc's DI seams.
`finding-otel-3` (+SURE the provenance/span plane is a live secret-leak surface): conduit ships
an in-process scrubber that runs BEFORE any attribute is set, because command-lines / `--client_env`
/ workspace-status routinely carry CI tokens [B-bazelconduit-readme-2026]. Dorc's receipts plane
is the same hazard class.

## §1 — Buck2 `buck2 log`: the living one-producer-many-lenses durable (rq-C)

[banked from predecessor turn-01; re-verified against the #18643 citation below]

`buck2-1` (+SURE). `buck2 log` is an always-on, per-invocation durable event log with ~15 query
lenses over ONE producer. Subcommands [B-buck2-log-2024]: `what-ran`, `what-failed`,
`what-materialized`, `what-uploaded`, `critical-path`, `replay`, `show` (JSON per line),
`show-user` (JSONL user-event log), `cmd`, `what-up` (spans open when the log ended), `path`,
`diff`. This is d-1's "one producer, many lenses" already shipping.

`buck2-2` (+SURE). Machine-readable output is baked in: `--format readable|tabulated|json|csv`.
`diff action-divergence` is precisely a built-in golden-trace-style diff: it "[i]dentifies the
first divergent action between two builds … Useful for identifying non-determinism"
[B-buck2-log-2024]. So the *diff-two-dumps* lens — the one Dorc would need for a golden-trace
fixture comparison — is a first-class shipping command, independent confirmation that the
durable-as-diff-substrate is practical.

> [B-buck2-log-2024] (buck2 log subcommand listing; relevance +SURE)
> Subcommands: what-ran … what-failed … what-materialized … what-uploaded … critical-path …
> replay … show … show-user … cmd … what-up … path … diff
> `--format <FORMAT>` … Possible values: readable / tabulated / json / csv

> [B-buck2-log-2024] (diff action-divergence; relevance +SURE)
> action-divergence: Identifies the first divergent action between two builds. Divergence is
> identified by the same action having differing outputs. Useful for identifying non-determinism

## §2 — Bazel exec-log #18643: the durable-dump SIZE/OVERHEAD economics and the always-on regret (rq-C)

The single richest primary thread on the cost of a voluminous always-on derivation durable.
Full comment thread re-fetched via GitHub API (predecessor had only the issue body). All quotes
verbatim, attributed by GitHub login + author_association.

`size-economics-1` (+SURE). meisterT (Bazel **MEMBER**), 2023-06-20, measured one large build's
exec log and the savings from each technique. Verbatim table:

> | what | without compression | zstd compressed |
> | before | 99GB | 20GB |
> | artifact indexing | 5.1GB | 610MB |
> | artifact indexing plus lean fields | 2.3GB | 450MB |

So: naive 99GB → 20GB by zstd alone → **450MB** with artifact-indexing + dropping fields
(command-lines, listed outputs, textual status, progress message). The requester tbaing reports
their own logs are "~120GB (and ~150GB once run through the parser)" [B-bazel-execlog-issue-2023].
Lesson for Dorc: a flat full dump is enormous; the leverage is (1) a shared/indexed
side-structure (don't repeat artifacts inline) and (2) field-tiering (store a hash, reconstruct
detail on demand) — the exact `value × set-of-origins` + lazy-rederive shape note 220's
`r-1` already converged on, now corroborated in the dump-format register.

`overhead-1` (+SURE, the >10× pre-optimization cost). meisterT, same comment:

> I noticed that action execution was quite a bit slower with the execution log turned on
> (regardless of which variant). … For the large build from above, the with the exec log it
> took ~75mins, without ~7mins.

A second user corroborates independently — sheldonneuberger-sc: "massive exec logs (50GB+), and
the build is slower if exec log is on. I'd prefer to have exec logging always on, but this makes
it unworkable" [B-bazel-execlog-issue-2023]. The fix was an async producer/consumer write path +
the compact format; tjgq's shipped result "aim[s] for 2-3% at most, often less" and "a reduction
of 100x relative to the old format" [B-bazel-execlog-issue-2023]. The lesson is NOT "dumps are
cheap" — it is "dumps are cheap ONLY after deliberate format + write-path engineering; the naive
version is unworkable."

`regret-against-1` (+SURE, the always-on caution from the people who shipped it). The decisive
anti-always-on signal, brentleyjones relaying tjgq, 2024-06-26:

> @tjgq and I discussed this, and he's not sure it would ever be fine for this to always be
> collected by default. But there could be a flag … which people could flip …

and sluongng (heavy user) the same day:

> Therefore, I do agree that we should not enable the compact log by default, at least not yet.

Also note tjgq's framing of readability — the durable is a *tool-only* artifact:

> As a drawback, the new format is not designed with human readability in mind, and is expected
> to be analyzed with the aid of specialized tools. … 7.1.0 will also provide an offline
> conversion tool (`//src/tools/execlog:converter`) …

`format-stability-1` (+SURE, directly on the trace-stability-promise question). Bazel kept the
compact format `--experimental_execution_log_compact_file` across the whole 7.x cycle and made a
*backwards-incompatible* format change mid-stream — tjgq, 2024-09-20:

> we had to make backwards-incompatible changes to the format in order to fix a flaw in the
> original design … you must make sure not to use an old tool with the new format, or vice-versa.

They deferred declaring stability to 8.0.0. This is concrete evidence for the human's reframe:
even a first-party team treats *byte-format stability of a derivation dump* as a costly promise
to be deferred and broken when the design improves. A Dorc golden-trace tier that pins exact
derivation bytes would inherit exactly this churn-and-break dynamic.

`userstory-1` (~SUSPECT, the closest thing to a postmortem story — it is a non-determinism hunt).
The motivating question, tbaing (issue body + first comment), verbatim need:

> which actions needed a rebuild (rather than being reusable from cache) and which dependencies
> drove that need

and the toolchain that exists for it: `coeuvre/bazel-find-non-deterministic-actions` (cited by
meisterT) plus Buck2 `diff action-divergence`. tbaing's detailed UX wishlist for such a tool is
itself a finding (see query-UX section). acecilia's BazelCon proposal is the prevention-gate
version:

> If the list of the cache misses is not a subset of the list generated by git diff, the pull
> request is introducing non-determinism

`query-ux-1` (+SURE, what floods vs what helps — from a real heavy user). tbaing, 2023-09-01,
on what a dump-diff tool must do to be usable on 100+GB logs — paraphrase-free highlights:

> An action for which all changed inputs are outputs from other actions is far less interesting
> when you're trying to understand why you rebuilt something … the tool would benefit from the
> ability to 1) focus on the non-transitive changes, and 2) walk easily down through the
> transitive changes to find interesting non-transitive changes. The ability to filter in/out
> branches of that graph would also be helpful

and a concrete flooding failure he hit — irrelevant diffs from cache-hit vs locally-built
actions reporting different fields:

> when an action is a cache hit, that content is not provided, and instead there is a `digest`
> block given. So when diffing two successive runs … we see irrelevant differences … which makes
> it harder to zero in on the actual differences

This is note 220's `r-2` (minimal-witness-first, fragment-and-expand, suppress-the-cascade)
reconfirmed from the dump-diff angle: the raw diff floods; the value is in *transitive-vs-
non-transitive filtering* and *normalizing away uninteresting field-differences* (cache-hit vs
local). Direct input to Dorc's `why`-query lens design AND to any golden-trace normalization.

`bes-tension-1` (~SUSPECT, the local-file-vs-stream packaging tension — relevant to Dorc's
controller↔host split). Multiple users (chancilasnap, minor-fixes, brentleyjones, sluongng)
push hard for the durable to be available over Bazel's event-stream (BES) / remote cache rather
than only as a controller-local file. minor-fixes verbatim:

> only offering this info via client-generated local files could very well mean that users need
> to supply: mechanisms to unconditionally dump these files … without filling up local disk …
> mechanisms to exfiltrate these files from automated build machines … The various streaming
> protocols that bazel supports (BES, RBE) have been a godsend for doing this sort of analysis/
> reporting across builds

For Dorc this maps to: the per-host derivation evidence wants a transport off the host, not just
a controller-local dump — and is a point in favor of d-2's "verdict-lane as carrier" idea (the
stream IS the exfil channel). Resolution in-thread: compact log gets attached to BES, but ONLY
when explicitly enabled (`--execution_log_compact_file=path`), never by default
[B-bazel-execlog-issue-2023] (fmeum, 2025-01-25).

## §3 — bazel_conduit: hand-rolling OTel from a neutral event stream (rq-D)

[banked from predecessor turn-01; full README read from disk copy]

The single best hand-rolling-OTel-without-the-SDK pitfall catalogue found: a Rust tool that
converts Bazel's BEP event stream into OTel traces, with 11 numbered lessons-learned. It is NOT
an in-process SDK instrumentation — it is a downstream sidecar that ingests a neutral event
stream and projects it to OTel spans. That architecture *is* the rq-D lean.

`finding-otel-1` (+SURE, the architecture). Conduit's shape [B-bazelconduit-readme-2026]:
"1 trace = 1 Bazel invocation (trace ID derived from Bazel's invocation UUID)"; root span over
`BuildStarted`→`BuildFinished`; target/action/test/spawn spans nested. It keeps the OTel span
model, a `bazel.<component>.<field>` attribute namespace (~100 typed constants, semantic-
convention-style), and W3C-ish trace identity — but Bazel itself emits only neutral BEP; the
OTel mapping lives in `otel/mapper.rs` downstream. Lean for Dorc: emit a neutral durable, convert
to OTel at the edge, do not couple the kernel to the SDK.

`finding-otel-2` (+SURE, the determinism pitfalls — each maps to a Dorc DI seam). Verbatim
lessons [B-bazelconduit-readme-2026]:

> Lesson 3: Bazel's `start_time` in `BuildStarted` Can Be Stale … can hold the Bazel server
> start time (potentially weeks old on long-lived daemons), not the current invocation start.
> Conduit prefers the BES-level `event_time` …

> Lesson 4: Timestamp Validation Is Essential. Bazel sometimes sends `start_time_nanos = 0` …
> Without validation, this produces spans starting at Unix epoch (1970) with multi-day durations.
> Conduit rejects timestamps below a `MIN_ABSOLUTE_NANOS` threshold (~year 2001) …

> Lesson 5: BatchSpanProcessor Can Silently Drop Spans. The default … queue size is 2048 spans.
> Large Bazel builds … overflow this easily. Conduit configures … queue 65,536 …

Plus from Data Quality: "Some BEP events report zero-length or negative durations due to clock
skew. Target-level durations (derived from action buffering) are more reliable" than action-level
[B-bazelconduit-readme-2026]. The pattern across all of these: a span is `(id, parent, start,
end, attrs)`, and every failure is in `start`/`end` (clock) or `id` (RNG/derivation) or
buffering order — precisely the non-deterministic inputs Dorc routes through DI. A Dorc dump
built on injected-clock + injected-RNG sidesteps Lessons 3/4/6 by construction (no wall-clock
leak, no proto-default-zero timestamps), and the queue-drop is a back-pressure/ordering concern
the DST harness can exercise.

`finding-otel-3` (+SURE, secret-leak surface). Conduit ships an in-process scrubber
(`otel/redact.rs`) that runs BEFORE any attribute is set on a span [B-bazelconduit-readme-2026]:

> Bazel surfaces environment variables and user-defined values on the command line via flags
> like `--client_env=NAME=VALUE` … Workspace status entries … routinely carry CI-injected
> tokens. … Without intervention all of these end up verbatim in the `bazel.command_line` …
> span attributes — and from there into whatever backend the OTLP exporter is wired to.

Default sensitive-name substrings: `TOKEN, SECRET, PASSWORD, PASSWD, CREDENTIAL, COOKIE, APIKEY,
API_KEY, ACCESS_KEY, PRIVATE_KEY, AUTH`; "intentionally narrow (e.g. plain `KEY` is excluded
because it would match `MONKEY`)." Defense-in-depth via Datadog `replace_tags` / OTel Collector
`redactionprocessor`. Dorc's receipts plane carries sh command-text and host responses — the
same leak class — and per the global security rule the scrub must happen at capture, before the
value enters the durable, not at export.

`finding-otel-4` (~SUSPECT, what conduit deliberately did NOT do — the minimalism boundary). Its
own Limitations section [B-bazelconduit-readme-2026]: "No span links / DAG representation … All
relationships are parent-child"; "No sampling policies … All qualifying events produce spans";
"No phase spans". So even a serious OTel-projection tool drops span-links (the very fork/join
mechanism note 220 holds from the OTel basics) and sampling — it keeps a strict tree. For Dorc
this is a datapoint that the *tree* projection is the cheap, sufficient default and span-links
are an advanced extra, not a day-one need.

## §4 — SQL plan-pinning: the decade-scale "pin a golden derivation" analog, and its regret (rq-C, the AGAINST case)

A pinned/forced SQL execution plan is structurally a *golden derivation*: you captured the
"right" plan once and told the engine to reuse it instead of re-deriving. DBAs have done this for
~15 years (Oracle SQL Plan Baselines/SPM since 11g; SQL Server Plan Guides → Query Store forced
plans → Automatic Plan Correction). This is the single best evidence base for d-1's golden-trace
question, and the verdict is heavily cautionary. Three distinct rot-modes, all of which a Dorc
golden-trace tier would structurally inherit.

`regret-against-2a` (+SURE, outright abandonment by an expert practitioner). Kendra Little, after
shipping a paid course on the feature, 2024-01-17 [B-kendra-apc-regret-2024]:

> Today I'm updating that course with a note: after using Automatic Plan Correction in anger for
> a good amount of time, I do not recommend enabling the feature. I've had it cause too many
> performance problems, and there are not a ton of options for an administrator when it's causing
> those problems.

And the lock-in trap — the pinned-derivation mechanism becomes load-bearing and can't be removed:

> becoming reliant on the feature for the places where it does help makes it difficult to disable
> the feature. You end up stuck with a very weird set of problems that are oddly similar to the
> problems the feature was designed to solve.

`regret-against-2b` (+SURE, the silent key-drift rot — the most Dorc-relevant). A forced plan is
keyed to a query fingerprint; when the fingerprint changes the pin silently applies to nothing,
with NO failure reported. Milos R., "Dude, Where's My Forced Plan?" [B-milos-forcedplan-2019]:

> a forced plan is associated not to a particular query, but to a given query_id … our SELECT
> query got a new query_id! … The forcing plan is correctly configured – there is no forced plan
> failure, it is just waiting for execution of the query with the query_id 1 … Forced plans are
> stable and will remain in the system only if query_id does not change.

This is *exactly* the failure d-1's golden-TRACE fixture would hit: pin a derivation keyed to
some hash of the analysis inputs; the moment the script or oracle-claim or probe-shape changes
the hash, the golden silently matches nothing and the "passing" fixture is verifying air. Note
220's `r-2(v)` (suppression heuristics are the deliverable) inverts here — the danger isn't noise,
it's a *silent no-op* that reads green. Kendra Little's GENERAL_FAILURE finding is the same rot
with teeth [B-kendra-genfail-2024]: a forced plan that fails to apply leaves the query with
"slower or potentially infinite compile time" (measured: 28s → >1 hour), and "Automatic Tuning
doesn't notice and clean that up" — the pinned artifact rotted into an actively-harmful,
unmonitored channel. Her standing remediation advice is to run a scheduled job that *finds and
un-forces failed pins* — i.e. the pin needs its own janitor.

`regret-against-2c` (+SURE, the pin gets silently ignored / can't be reproduced — soft-pin
gradient). Oracle's softer pin (SQL Profile) "provide[s] guidance to the optimizer — not
enforcement. They can be ignored" [B-oracle-ckpt-baselines-2025]; the team escalated through SQL
Patch to a hard SQL Plan Baseline to get enforcement. But even the hard baseline can fail to
reproduce: Jonathan Lewis documents a baseline "accepted, enabled, and fixed … it clearly
'belongs' to our query. So it should have been used" — yet "the optimizer says it can't
reproduce the plan we wanted" because the stored hints don't actually regenerate the captured
plan [B-jlewis-fakebaselines-2020]. The golden-derivation analog: even when the pin matches and
is honored, the engine may be unable to *reconstruct* the pinned artifact from the stored
representation — pinning the *output* and pinning the *means to reproduce it* are different, and
the gap is a bug-farm.

`regret-against-2d` (~SUSPECT, pinning corrupts adjacent observability). Paul White: forcing a
plan (any method) overwrites the plan's `query_hash` with its `query_plan_hash`, so "[t]his will
break anything you have that uses `query_hash` for any purpose, including scripts and tools"
[B-paulwhite-queryhash-2024]. A pinned-derivation mechanism mutated the very identifiers other
tooling keys on. For Dorc: a golden-trace plane that rewrites or shadows provenance identity
could break the `why`-query lenses that read it — keep the pin a *separate, additive* artifact,
never a mutation of the live receipt identity.

`autocapture-bloat-1` (~SUSPECT, auto-capturing every derivation bloats the store — the
capture-without-curation hazard). Oracle's own optimizer blog warns against leaving SPM
auto-capture on indefinitely [snippet, B-oracle-spm-autocapture-2023]; a user reports "SYSAUX
grows huge after SQL Plan Baseline capture turned on. I only had auto capture on for a couple
hours" [snippet, forums.oracle.com]; sqlmaria (Maria Colgan) "wouldn't recommend automatic
capture because it would result in a SQL plan baseline being created for every repeatable SQL
statement executed" [snippet]. The directly-applicable lesson, consistent with Bazel's
default-OFF ruling: capture-everything-by-default bloats and is regretted; capture is opt-in and
curated. (Graded ~SUSPECT: read from search snippets, not full posts — capped per note rules.)

Net for d-1's golden-TRACE half: the decade-scale analog says PIN SPARINGLY, EXPECT SILENT
DRIFT, and BUILD A JANITOR. It is consistent with the human's reframe — the value of pinning a
derivation (vs pinning the *verdict* and re-deriving) is real only for a small critical set, and
even there the pin needs active staleness-detection or it rots green. This corroborates the
round-22 d-1 tiering instinct (critical-tier pins traces, the rest pins verdicts) but pushes the
critical tier *smaller* and demands the staleness-detector be designed in from day one, not bolted
on. It does NOT, on its own, supply the affirmative user-story; it supplies the cost side.

## §5 — auto_explain: the dump-everything-vs-filter economics, in the SQL register (rq-C)

`size-economics-2` (+SURE, the threshold lesson, measured). pgMustard benchmarked PostgreSQL's
`auto_explain` (log execution plans automatically) [B-pgmustard-autoexplain-2021]. Logging EVERY
plan (`log_min_duration = 0`, no timing): "about 26% slower than our baseline … This test
generated ~6GB of logs, which is another good reason to not log everything!" Logging only the
slowest (`log_min_duration = 10`ms): "only 0.8% slower … the threshold of 10ms meant we avoided
logging ~99.9% of the query plans." With per-operation timing (`log_analyze = on`) the cost rises
further — third-party summary cites "10–30% execution overhead for logged queries"
[snippet, B-jusdb-autoexplain-nd]; PostgreSQL 13+ adds `sample_rate` precisely to cap volume.

This is the Bazel execlog finding in a second, independent system: **the always-on full dump is
expensive (perf + volume); a threshold/sample makes it nearly free; the lever is filtering at
capture, not after.** For Dorc the analogy is imperfect in a *favorable* direction — Dorc's
analysis runs once per orchestration and is dominated by network round-trips, so the per-run dump
overhead the SQL/build worlds fight is largely a non-issue (per AGENTS.md: the analyzer cost is
dominated by the SSH tunnels that follow). The volume/retention problem (6GB-class artifacts
nobody prunes) does carry over, and is the real reason to tier and to default the full dump OFF.

## §6 — rustc UI tests / `--bless`: the canonical golden-OUTPUT churn-fighting machinery (rq-C, golden-trace FOR-mechanism + churn-cost)

The Rust compiler's UI test suite pins the compiler's diagnostic output (`.stderr`/`.stdout`
snapshots) for tens of thousands of tests. It is the most battle-tested golden-output system in
open source and the closest *mechanism* prior art for d-1's golden-trace tier. The decisive
lessons are about HOW they keep goldens from churning — and the answer is heavy normalization +
a regenerate workflow + redundant non-golden assertions. [B-rustc-uitest-2026]

`golden-mechanism-1` (+SURE, the bless workflow). "UI tests store the expected output … in
`.stderr` and `.stdout` snapshots next to the test. You normally generate these files with the
`--bless` CLI option, and then inspect them manually to verify they contain what you expect"
[B-rustc-uitest-2026]. So the golden is machine-regenerated, human-reviewed — never hand-written.
Direct shape for a Dorc golden-trace: `dorc test --bless` regenerates the pinned derivation; the
human reviews the diff. The pin is a *checkpoint of reviewed output*, not a hand-authored spec.

`golden-churn-1` (+SURE, normalization is the entire game, and it is explicitly a churn-vs-
readability tradeoff). The compiler is run under `-Z ui-testing` and the harness normalizes:
test dir → `$DIR`, stdlib dir → `$SRC_DIR`, "[l]ine and column numbers for paths in `$SRC_DIR`
are replaced with `LL:COL`", build dir → `$TEST_BUILD_DIR`, tabs → `\t`, backslashes → forward
slashes, CRLF → LF, error annotations stripped, and "[v]arious v0 and legacy symbol hashes are
replaced with placeholders like `[HASH]`" [B-rustc-uitest-2026]. The load-bearing rationale,
verbatim:

> Line and column numbers for paths in `$SRC_DIR` are replaced with `LL:COL`. This helps ensure
> that changes to the layout of the standard library do not cause widespread changes to the
> `.stderr` files.

and the deliberate *non*-normalization, with its own rationale:

> the line and column numbers for `-->` lines pointing to the test are not normalized … This
> ensures that the compiler continues to point to the correct location, and keeps the stderr
> files readable. Ideally all line/column information would be retained, but small changes to the
> source causes large diffs, and more frequent merge conflicts and test errors.

This is the single most direct datapoint for the human's reframe. The rustc team — who pin traces
(diagnostic output) at enormous scale and clearly find it worth it — pay for it with a *large,
carefully-tuned normalization layer* whose explicit purpose is "small changes don't cause
widespread golden churn / merge conflicts." For Dorc: pinning a derivation is viable ONLY with a
matching normalizer that erases the volatile coordinates (host-specific paths, timestamps,
probe-ordering, hashes) — and designing that normalizer is the real cost, not the pinning. Without
it, golden traces "cause large diffs and more frequent merge conflicts," i.e. they rot into churn.
Note 220's `r-2` query-UX habits reappear here as *golden-fixture* habits: normalize away the
uninteresting, keep the legible witness.

`golden-redundancy-1` (~SUSPECT, the golden is not the sole oracle). UI tests *also* require inline
`//~ ERROR <substring>` annotations next to the source line, redundantly with the `.stderr` file:
"This redundancy helps avoid mistakes since the `.stderr` files are usually auto-generated … they
ensure that no additional unexpected errors are generated" [B-rustc-uitest-2026]. The lesson: an
auto-blessed golden can silently bless a *regression* (you regenerate, the bad output becomes the
new "expected"); the inline human-written assertion is the guard against rubber-stamping. For a
Dorc golden-trace tier, the analog is: pin the full trace for diffing, but ALSO keep a small
human-written assertion on the *verdict* (the thing that actually matters) so a blessed-in
regression in the trace can't pass unnoticed. This directly supports the d-1 tiering split
(critical pins trace; everyone pins verdict) — but says even the critical tier should keep the
verdict assertion alongside the trace, never trace-only.

## §7 — rr / Pernosco: the store-minimal-recompute-the-rest economics (rq-C)

rr records a program once and replays deterministically; Pernosco builds an "omniscient database"
over an rr recording. The relevant axis for d-1: what do they STORE vs RE-DERIVE, and how does
that scale? Authoritative because the source is Robert O'Callahan, rr's author.
[B-ocallahan-debt-2024]

`recompute-1` (+SURE, the split: record cheap, re-derive heavy). Community summary of Klock's
demo: "Pernosco uses record and replay. Record is fast and records just enough to be able to
replay, and all heavy computation is [re-derived at replay]" [snippet, confirmed by author's
framing]. O'Callahan on the limit, verbatim [B-ocallahan-debt-2024]:

> Scalability of omniscience to very compute-heavy workloads is a problem (although not as
> problematic as everyone assumes).

The shape: store the minimal nondeterminism-capture (enough to deterministically replay), then
recompute everything else (register values at time T, last-modification) on demand via massive
parallelism. This is precisely note 220's `r-1` rule in the debugging register — a constant-ish
capture + lazy backward re-derivation — and it is the affirmative model for d-1's dump: you do NOT
need to store the full derivation if you can deterministically *re-run the analysis* to
reconstruct it. Dorc's DST determinism is exactly what makes "re-derive instead of store" sound:
given the same seed + recorded probe-responses, the analyzer reproduces the identical derivation.
That argues the durable can be *thin* (the seed + the probe-response tape), with the full trace
reconstructed on demand — a strong cost-reducer for the always-on worry.

`negative-query-1` (+SURE, independent confirmation that "why NOT" is the universal hard gap).
O'Callahan's open-problems list, verbatim [B-ocallahan-debt-2024]:

> We still lack direct approaches to understanding why things didn't happen. … we need good tools
> for understanding why something happened in one situation … but not another very similar
> situation, assuming we have complete recordings of the two runs.

Note 220's `r-2` already claimed Dorc dodges the why-NOT problem because refusals are positive
events. Here the frontier of omniscient debugging — with *complete recordings of both runs* —
still calls why-didn't-it the unsolved problem. That strengthens 220's claim into a genuine
structural advantage: Dorc's "why did this NOT get replaced" is answerable precisely because the
failing license-check is a recorded positive event, where rr/Pernosco (which record everything)
still can't cheaply answer the counterfactual. The diff-two-runs framing (acecilia's CI gate,
Buck2 `diff action-divergence`) is the practical substitute everyone reaches for instead.

`ubiquity-1` (~SUSPECT, the always-on dream and its cost). O'Callahan's ubiquity vision
[B-ocallahan-debt-2024]: "I dream of a world where, when you hit a bug … your test runs are
always being recorded and any necessary debuginfo is always available." This is the *pro*-always-on
voice — but note it is aspirational, gated on cost, and from the vendor of a *non*-always-on tool
(rr recording is opt-in precisely because always-on record has overhead). The honest read: even
the strongest omniscient-debugging advocate frames always-on capture as a not-yet-achieved goal,
not a solved default. Consistent with Bazel/SQL: the always-on durable is desirable and costly;
nobody ships it on-by-default.

## §8 — OTel file-dump formats + tracecontext-WITHOUT-SDK (rq-C dump-shape, rq-D propagation)

`dumpformat-1` (+SURE, the OTLP file exporter is JSON-lines, immature, order-unstable). The OTel
spec for file output is explicitly `Status: Development` and "provides a placeholder" — it
"only describes the serialization … to the OTLP JSON format" [B-otel-fileexporter-2026]. Shape:

> This file is a JSON lines file … UTF-8 … Each line is a valid JSON value … The line separator
> is `\n` … preferred file extension is `jsonl`.

and crucially for a dump consumed later:

> There is no guarantee that the data in the file is ordered. There is no guarantee in particular
> that timestamps will be monotonically increasing.

Read back via the "OTLP JSON File receiver" in the collector [B-otel-fileexporter-2026]. For Dorc:
JSON-lines is the natural append-only dump shape (matches Buck2 `show-user` JSONL and conduit's
NDJSON intake), tooling exists to re-ingest, BUT the format is young and the spec itself disclaims
ordering — reinforcing that a Dorc dump should NOT promise byte/order-stability and should carry
its own version tag. The example payload is one `{"resourceSpans":[...]}` object per line, spans
carrying `traceId/spanId/parentSpanId/name/startTimeUnixNano/endTimeUnixNano/events/links/status`
— a compact, well-understood span record if Dorc wants OTLP-shaped output at the edge.

`propagation-1` (+SURE, the on-the-wire format is trivial and SDK-free — but the spec is HTTP-header
framed). W3C Trace Context is a W3C Recommendation [B-w3c-tracecontext-2021]; the carrier is two
headers, `traceparent` (core identity: version-traceid-spanid-flags) and `tracestate` (vendor
data). The format is a fixed-width hex string — `00-<32hex traceid>-<16hex spanid>-<2hex flags>`
— with no library required to emit or parse it. The catch for Dorc: the spec is *defined over HTTP
headers*, so propagating it over a non-HTTP carrier (Dorc's controller→host stdout protocol / env
vars / the verdict lane) is a hand-roll — you adopt the *value format* and the propagate-unchanged
+ mutate-your-own-span semantics, but you choose the carrier. This is exactly d-2's "import the
ideas not the machinery": the traceparent string is an idea (16 bytes of id + 8 of span + flags)
you can carry in an env var or a protocol line.

`propagation-2` (~SUSPECT, CI systems already do this — context into shell steps). General practice
(multiple secondary sources, e.g. uptrace/dash0 explainers [snippet]) is to inject `traceparent`
as an environment variable into child/shell steps so a build step joins the parent trace; the
recurring hand-roll pitfalls are the SAME determinism seams conduit hit — ID generation (needs
RNG), timestamp capture (needs clock), and sampling-flag handling. For Dorc's deterministic kernel
this is a clean fit ONLY if id-generation and timestamps route through the existing DI seams (seed
→ deterministic traceid; injected clock → span times); a naive `rand()`/`now()` traceparent emitter
would break DST. (Graded ~SUSPECT: synthesized from explainer snippets + the conduit primary, not a
single dedicated hand-rolled-shell-propagation primary — flagged as an open thread to harden with a
concrete CI-propagates-to-shell source, e.g. GitLab/Jenkins OTel docs.)

`propagation-3` (~SUSPECT, the minimalism-regrows-into-the-SDK counter-thesis — partially observed).
The conduit datapoint [B-bazelconduit-readme-2026] is itself a mild instance: it set out to "just
map BEP to spans" but had to re-import BatchSpanProcessor tuning, redaction, clamp_time_range,
flush-timing workarounds — i.e. minimal-OTel accreted real SDK-adjacent machinery once it hit a
real backend (Datadog/Jaeger). The lesson is not "don't go minimal" but "the machinery you skip
at the model layer reappears at the *export/backend* layer" — so Dorc keeping OTel as an
edge-projection (not a kernel dependency) is the right boundary: the accretion stays in the
optional exporter, never in the deterministic core. (A dedicated written "we did ideas-only OTel
and here's what regrew" report was NOT found as a single strong primary — open thread.)

## §9 — the postmortem user-story hunt: what an always-on event/derivation record actually buys (rq-C)

The prompt's highest-value gap. The honest result: a clean "an always-on DERIVATION log solved a
production mystery nothing else could" narrative for a static analyzer does NOT exist in the
public record (the closest, the Bazel non-determinism hunt, is in §2). But a strong *cluster* of
runtime-ops incident write-ups converges on one pattern that transfers to Dorc by analogy, and it
is sharper than "dumps are good."

`userstory-2` (~SUSPECT, the event-log-as-diagnostic-key story, with caveats). "The Spot Instance
That Killed Our Payments Service" [C-spotinstance-postmortem-2026] is the cleanest "event log was
the thing" narrative. A 47-minute incident where the fix was 2 lines of YAML; the breakthrough,
verbatim section title: "Minute 27: The Event Log (Should Have Started Here)". The author's
post-incident rule:

> Start every investigation with events, not logs. `kubectl get events --sort-by='.lastTimestamp'`
> is now the first command in our runbook. Logs show what happened to the process. Events show
> what [the system] did about it. Start wider, then drill down.

and the diagnosis of WHY the dump didn't help sooner — the load-bearing nuance for Dorc:

> The signals that cracked it were all there from minute zero … The problem is that these signals
> live in three different places, and under pressure … humans don't naturally start with the most
> diagnostic view. We start with the most familiar one (logs) and dig deeper instead of wider.

So the value is NOT the dump existing — the events existed and were ignored for 27 minutes. The
value is a *single correlated, timestamp-ordered, derivation-shaped view* presented as the FIRST
thing. For Dorc's `why`-query this is the design lesson: the dump only pays off if the query lens
makes the most-diagnostic slice (the failing license-check + its contributing origins, ordered)
the default first view — exactly note 220's `r-2(i)` minimal-witness-first. (Caveat: this post is
partly a vendor pitch for an automated-investigation product, and it is a RUNTIME-ops story, not a
static-derivation one — graded C, transfer is by analogy.)

`userstory-3` (~SUSPECT, the silent-green-dashboard failure class — the real recurring shape).
Across an independent set of 2026 incident teardowns the SAME structure recurs: every instrument
green, the thing the instruments protect being silently destroyed, and the crack coming from
*wider/correlated* data. Verbatim, the "Silent Consumer" teardown [C-silentconsumer-2026]:

> Every instrument you trust is reporting green while the thing the instruments exist to protect
> is being lost in real time. You cannot follow an error trail because there is no error trail. You
> have to debug a system that is lying to you with a straight face.

and the emergent-composition diagnosis (strikingly close to Dorc's own compositional-elision risk,
note 222 `c-2`):

> Three independent, individually-reasonable decisions … stack into one silent data shredder. Each
> one passed review on its own. Nobody reviewed the composition, because the composition does not
> live in any single file. … The failure is emergent. That is why it survives every test you have.

The "Schrödinger's Event" teardown [C-schrodinger-2026] supplies the timestamp-correlation crack:
"We found our ghost because someone noticed a six-millisecond gap in the timestamps … Log your
transaction boundaries. Correlate timestamps across services. The bug you can't see is the bug you
can't fix." For Dorc, this is the affirmative case for the durable, reframed precisely: the dump
earns its keep against *emergent, silent-success* failures — where each step "passed" but the
composition is wrong and no error fired. A per-value derivation record that shows *which origins
actually contributed to a ⊤* is exactly the artifact that surfaces an emergent wrong-elision that
"survives every test." This is the strongest affirmative argument found for d-1 — but it argues for
the *receipts/why-query* (per-value provenance, note 220) more than for *golden-TRACE pinning*; the
two halves of d-1 do not stand or fall together.

`userstory-4` (-GUESS, the canonical fate-sharing caution, for the propagation/transport design).
AWS's Kinesis 2020 outage (reconstructed from AWS's official post-event summary)
[C-kinesis-2020-reconstruction-2026]: CloudWatch ingested through Kinesis, so when Kinesis failed
"the system designed to detect failures was itself impaired by the failure" — a "blindness loop".
For Dorc's d-2 (verdict-lane as trace carrier): a caution that the observability/derivation channel
must not share fate with the thing it observes. If the controller→host derivation record rides the
same channel whose failure it must diagnose, a host-comms failure blinds the very postmortem. Argues
for the dump being durable-locally-first (survives the channel), then exfiltrated — consistent with
§2's `bes-tension-1` resolution. (Graded -GUESS for Dorc-transfer: the source is solid on AWS, the
mapping to Dorc's transport is my inference.)

`incremental-debug-1` (-GUESS, salsa/incremental-engine debug surfaces — lightly covered, corpus
already holds the mechanism). The corpus already holds rust-analyzer's durability/early-cutoff
machinery as `[B-ra-durability-2023]` (note 220 `r-1`). On the *debugging-surface* sub-question
(how people inspect a live incremental derivation graph): salsa exposes per-query event callbacks
and the rustc query system supports `-Z dump-dep-graph`/query-DAG inspection, but I did NOT surface
a strong dedicated primary on an *accumulated-query-dump* debugging UX this turn — the public
material is mostly the durability blog (held) and API docs. Flagged as a thin spot; -GUESS that the
salsa debug-event API is the closest analog to a Dorc "stream the derivation as it happens" mode,
worth a dedicated dig if the conductor wants it. Did NOT over-invest, per the corpus already
covering the durability core.

## Graded sources

Grades assigned by gathering subagent (R2'); conductor re-verification pending. Scale A>B>C>D.
Cross-corpus citations carried from earlier notes (here: `[B-ra-durability-2023]`, note 220 `r-1`)
are referenced inline with their origin note and are intentionally NOT re-graded in this list.

- `[B-buck2-log-2024]` · Meta/Buck2 team, official `buck2 log` command reference ·
  https://buck2.build/docs/users/commands/log/ · 2024 · read-depth full (disk copy + API-confirmed
  via #18643 quote) · grading: not A because it is a CLI reference listing (the *surface*, not design
  rationale or postmortem narrative), so it grounds the architecture but answers none of the why/regret
  questions; well above C — canonical first-party docs, directly the d-1 analog · relevance: the closest
  *shipping* one-producer-many-lenses always-on durable, incl. `diff action-divergence` golden-diff lens
  · via predecessor `B-buck2-log-2024` (Kagi "Buck2 event log format what-ran").
- `[B-bazel-execlog-issue-2023]` · bazelbuild/bazel issue #18643, thread incl. members meisterT,
  tjgq, lberki, fmeum + heavy users tbaing/sluongng/brentleyjones · https://github.com/bazelbuild/bazel/issues/18643
  · 2023-06 → 2025-01 · read-depth full (entire comment thread via GitHub API; predecessor had only the
  body) · grading: not A because it is a feature-request discussion, not a peer-reviewed/canonical doc;
  but it is the richest *first-party* primary on always-on-durable economics — measured size table,
  measured >10× overhead, explicit maintainer reluctance to default-on, real-user UX wishlists; the
  measurement + named-member attribution lift it above a typical issue thread, hence B not C · relevance:
  the spine of rq-C — size economics, overhead regret, format-stability deferral, the closest postmortem
  (non-determinism hunt), query-UX flooding · via predecessor (Exa "debugging a production incident using
  an always-on build event log").
- `[B-bazelconduit-readme-2026]` · JSGette/bazel_conduit README (Rust BEP→OTel converter) ·
  https://github.com/JSGette/bazel_conduit · 2026-05 · read-depth full (disk copy, entire README) ·
  grading: not A — a single young-repo README, not peer-reviewed nor canonical; but an unusually detailed
  first-party engineering account (11 numbered lessons) of hand-rolling span emission from an event
  stream in Rust — exactly the rq-D shape; depth + specificity (named modules, thresholds, secret-list)
  lift it above ephemeral C/D, hence B · relevance: best hand-rolling-OTel pitfall catalogue — clock/RNG/
  ordering seams + secret-scrub-before-set + emit-neutral-convert-at-edge architecture · via predecessor
  (Exa, same query as above).
- `[B-kendra-apc-regret-2024]` · Kendra Little (independent SQL Server expert/educator),
  "Automatic Plan Correction Could Be a Great Auto Tuning Feature … Here Is What It Needs" ·
  https://kendralittle.com/2024/01/17/automatic-plan-forcing-could-be-great-but-isnt-sql-server/ ·
  2024-01-17 · read-depth full · grading: not A — a single practitioner blog, not peer-reviewed; but a
  named, reputationally-staked expert reversing her own published course recommendation after extended
  production use, with concrete mechanism (lock-in, temp-table sniffing) — high-trust regret evidence,
  hence B · relevance: the strongest abandonment narrative for auto-pinning derivations; lock-in trap ·
  via Exa "engineer regret about forcing SQL execution plans".
- `[B-kendra-genfail-2024]` · Kendra Little, "General Failure Failed Forced Plans in Query Store Cause
  Even Slower Compile Times" · https://kendralittle.com/2024/08/12/query-store-failed-forced-plans-general-failure-even-slower-compile-time/
  · 2024-08-12 · read-depth targeted (Exa highlights, not full body) · grading: same author/venue as
  above but read only via highlights, so capped; the reproduction (28s→>1hr compile, "Automatic Tuning
  doesn't notice and clean that up") is specific and load-bearing · relevance: the silent-rot-with-teeth
  mode — a failed pin actively harms and is unmonitored; motivates a staleness-janitor · via Exa (same).
- `[B-milos-forcedplan-2019]` · Milos R., "Dude, Where's My Forced Plan?! – Part 1" ·
  https://milossql.wordpress.com/2019/10/21/dude-wheres-my-forced-plan-part-1/ · 2019-10-21 · read-depth
  targeted (Exa highlights) · grading: practitioner blog, read via highlights → capped; but the
  query_id-drift mechanism is precisely demonstrated and directly analogous to golden-trace key drift,
  hence B not C · relevance: the silent key-drift no-op — pin matches nothing, no failure raised; THE
  golden-trace cautionary mechanism · via Exa (same).
- `[B-oracle-ckpt-baselines-2025]` · Nassyam Basha (Oracle CKPT), "Oracle SQL Plan Instability: Why SQL
  Profiles Are Not Enough and SQL Plan Baselines Are the Reliable Solution" ·
  https://oracle-ckpt.com/oracle-sql-plan-instability-why-sql-profiles-are-not-enough-and-sql-plan-baselines-are-the-reliable-solution/
  · 2025-05-13 · read-depth targeted (Exa highlights) · grading: practitioner case study via highlights →
  capped at B; concrete plan-hashes + the profile-ignored→baseline-enforced escalation are load-bearing ·
  relevance: the soft-pin-gets-ignored gradient; enforcement requires the hard pin · via Exa (same).
- `[B-jlewis-fakebaselines-2020]` · Jonathan Lewis (Oracle optimizer authority), "Fake Baselines (2)" ·
  https://jonathanlewis.wordpress.com/2020/02/24/fake-baselines-2/ · 2020-02-24 · read-depth targeted
  (Exa highlights) · grading: among the most authoritative Oracle-optimizer voices, but read via
  highlights → capped at B; the "accepted/enabled/fixed baseline the optimizer can't reproduce" finding
  is specific · relevance: pinning the output ≠ pinning the means to reproduce it; reproduction-gap
  bug-farm · via Exa (same).
- `[B-paulwhite-queryhash-2024]` · Paul White (SQL Server internals authority), "SQL Server Forced Plans
  Overwrite the Query Hash" · https://www.sql.kiwi/2024/11/forced-plans-query-hash/ · 2024-11-28 ·
  read-depth targeted (Exa highlights) · grading: top-tier internals author, highlights-only → capped at
  B; the query_hash-overwrite is a precise, reproducible claim · relevance: pinning corrupts adjacent
  observability identity — keep the pin additive, never a mutation of live receipt identity · via Exa.
- `[B-oracle-spm-autocapture-2023]` · Oracle Optimizer team blog, "What you need to know about SQL Plan
  Management and auto-capture" · https://blogs.oracle.com/optimizer/what-you-need-to-know-about-sql-plan-management-and-auto-capture
  · 2023-12-01 · read-depth snippet (search result only) · grading: first-party Oracle, but read only via
  search snippet → capped at ~SUSPECT-tier evidence; the "don't leave it enabled indefinitely" caution is
  the cited line · relevance: capture-everything-by-default bloats; corroborates default-OFF · via Kagi
  "Oracle SQL Plan Management SPM automatic capture SYSAUX bloat".
- `[B-pgmustard-autoexplain-2021]` · pgMustard, "Can auto_explain (with timing) have low overhead?" ·
  https://www.pgmustard.com/blog/auto-explain-overhead-with-timing · 2021-03-16 · read-depth full ·
  grading: vendor engineering blog but a careful, reproducible pgbench benchmark with method + numbers,
  not marketing; not peer-reviewed → B · relevance: the dump-everything-vs-filter economics in SQL — 26%
  + 6GB for log-all vs 0.8% for threshold; the filter-at-capture lever · via Kagi "auto_explain log
  overhead postgresql always on".
- `[B-jusdb-autoexplain-nd]` · JusDB blog, "PostgreSQL auto_explain" · https://www.jusdb.com/blog/postgresql-auto-explain-automatic-query-plan-logging
  · n.d. · read-depth snippet · grading: secondary blog, snippet-only → capped low; cited only for the
  "10–30% with log_analyze" figure, corroborating the timing-overhead direction · relevance: per-node
  timing raises dump cost · via Kagi (same auto_explain query).
- `[B-rustc-uitest-2026]` · Rust project, rustc-dev-guide "UI tests" chapter · https://rustc-dev-guide.rust-lang.org/tests/ui.html
  · living doc (read 2026-06-11) · read-depth full · grading: not A — it is project developer
  documentation, not a peer-reviewed source or a retrospective; but it is the canonical, authoritative
  description of the most battle-tested golden-output system in OSS, with the churn-vs-readability
  tradeoff stated in the maintainers' own words; depth + authority → high B, just shy of A on the
  golden-trace-mechanism question · relevance: THE golden-output prior art — `--bless` regenerate
  workflow, the normalization layer as the real cost, redundant inline assertions guarding blessed-in
  regressions · via Kagi "rustc UI test --bless normalization churn".
- `[B-ocallahan-debt-2024]` · Robert O'Callahan (author of rr; co-founder of Pernosco), "Advanced
  Debugging Technology In Practice" (DEBT/ECOOP 2024 keynote summary) ·
  https://robert.ocallahan.org/2024/10/debt-workshop.html · 2024-10-01 · read-depth full · grading:
  not A — a personal blog summary of a talk, not a paper; but it is the primary author's first-hand
  account of omniscient-debugging economics and open problems, uniquely authoritative on the
  store-vs-recompute split and the why-NOT gap; → strong B · relevance: store-minimal-recompute-the-rest
  model for a thin durable; independent confirmation that why-didn't-it is the universal hard gap (Dorc's
  positive-refusal advantage); the honest always-on-as-aspiration framing · via Kagi "rr Pernosco
  omniscient debugging storage vs recompute".
- `[B-otel-fileexporter-2026]` · OpenTelemetry, "OTLP File Exporter" specification (Status: Development)
  · https://opentelemetry.io/docs/specs/otel/protocol/file-exporter/ · living spec (read 2026-06-11) ·
  read-depth full · grading: first-party canonical OTel spec, but explicitly Development/"placeholder"
  status → not A on stability grounds (it disclaims maturity itself); the JSON-lines shape + the
  order/timestamp non-guarantees are authoritative → B · relevance: the candidate dump shape (JSONL,
  re-ingestible) AND direct evidence the format is young + order-unstable → don't promise byte-stability,
  carry a version tag · via Kagi "OpenTelemetry OTLP file exporter JSON format stability".
- `[B-w3c-tracecontext-2021]` · W3C, "Trace Context" Recommendation · https://www.w3.org/TR/trace-context/
  · 2021-11-23 (W3C Rec) · read-depth targeted (search result + prior corpus knowledge of the header
  format) · grading: canonical W3C standard → would be A on authority, but read only targeted this turn
  (format + HTTP-header framing confirmed via search, not a fresh full read of the Rec) → capped at B per
  full-read-before-grade rule · relevance: the SDK-free wire format (`traceparent` fixed hex string) is
  trivially hand-emittable; the spec is HTTP-header-framed so a non-HTTP carrier (Dorc stdout/env/verdict
  lane) is a sanctioned hand-roll — exactly d-2's import-the-idea · via Kagi "W3C trace context
  traceparent propagation without SDK shell".
- `[C-spotinstance-postmortem-2026]` · "The Spot Instance That Killed Our Payments Service (And Why It
  Took Us 47 Minutes to Find It)" (dev.to, Peter/Infranexis) · https://dev.to/peterinfranexis/the-spot-instance-that-killed-our-payments-service-and-why-it-took-us-47-minutes-to-find-it-2ehp
  · 2026-04-26 · read-depth full · grading: C not B — a real, specific, well-told incident narrative
  (exact commands, timeline, fix) BUT partly a vendor pitch for the author's automated-investigation
  product (Causa) and a runtime-ops story not a static-derivation one; bias + register-mismatch cap it at
  C · relevance: the cleanest "event log was the diagnostic key" story; the load-bearing nuance that the
  dump's value is the correlated first-view, not its existence · via Exa "always-on event log diagnosed a
  hard production failure".
- `[C-silentconsumer-2026]` · "The Silent Consumer: A Teardown of the Incident That Loses Data While
  Reporting Success" (Substack, Devrim Özcay) · https://devrimozcay1.substack.com/p/the-silent-consumer-a-teardown-of
  · 2026-05-17 · read-depth targeted (Exa highlights) · grading: C — vivid practitioner teardown read via
  highlights only, paywalled-teardown framing, single-author; the emergent-composition + silent-success
  framing is load-bearing and quotable · relevance: the silent-green-dashboard failure class; emergent
  compositional failure that "survives every test" — the best affirmative case for receipts vs the
  silent-wrong-elision risk · via Exa (same).
- `[C-schrodinger-2026]` · "Stack Crash Investigations: Schrödinger's Event" (Medium, Karan Saklani) ·
  https://medium.com/@karansaklani20/stack-crash-investigations-schrodingers-event-6e5fb2661ce0 ·
  2026-01-22 · read-depth targeted (Exa highlights) · grading: C — practitioner war-story via highlights;
  the timestamp-gap crack ("six-millisecond gap in the timestamps") is the cited, load-bearing detail ·
  relevance: timestamp-correlation across an event record as the breakthrough; "the bug you can't see is
  the bug you can't fix" · via Exa (same).
- `[C-kinesis-2020-reconstruction-2026]` · Sujeet Jaiswal, "AWS Kinesis 2020 Outage: Thread Limits,
  Thundering Herds, and Hidden Dependencies" (reconstruction grounded in AWS's official post-event
  summary) · https://sujeet.pro/articles/aws-kinesis-2020-outage · 2026-02-16 · read-depth targeted (Exa
  highlights) · grading: C as cited here — a secondary reconstruction (the primary is AWS's post-event
  summary, not read this turn) read via highlights; the fate-sharing/blindness-loop pattern is the cited
  point · relevance: the observability channel must not share fate with what it observes — caution for
  d-2's verdict-lane-as-carrier; durable-locally-first then exfiltrate · via Exa (same).

