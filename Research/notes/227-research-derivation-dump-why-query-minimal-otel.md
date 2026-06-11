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

**Best postmortem user-story found (partial; the hunt is still under-served).**
`userstory-1` (~SUSPECT): the recurring real use is **non-determinism / unexpected-cache-miss
hunting via dump-diff**, not free-form postmortem. tbaing's motivating ask is verbatim "which
actions needed a rebuild (rather than being reusable from cache) and which dependencies drove
that need" [B-bazel-execlog-issue-2023]; the entire `coeuvre/bazel-find-non-deterministic-actions`
tool and Buck2's `diff action-divergence` exist for this one job. acecilia proposed a concrete
CI gate at BazelCon: compare cache-miss set to git-diff-derived target set; "if the list of the
cache misses is not a subset … the pull request is introducing non-determinism"
[B-bazel-execlog-issue-2023]. This is a *prevention* story (catch non-determinism in CI), which
is closer to Dorc's golden-TRACE-fixture use than to ad-hoc postmortem. A clean "the always-on
log solved a production mystery nothing else could" narrative remains NOT yet found — flagged as
the top open thread.

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

## Graded sources

Grades assigned by gathering subagent (R2'); conductor re-verification pending. Scale A>B>C>D.

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

