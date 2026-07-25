# 28F — the firefight/explanation tier, graded for risk

Written by the security conductor 2026-07-25, which sits OUTSIDE the Fable firewall and
may read every quarantined document directly. Quarantined because its reasoning cites
round-29 findings and the `sinv-*` construction law; its OUTPUT — eighteen letter grades
appended to `notes/28D` — is deliberately rationale-free and crosses the gate on purpose
(§7).

Authority: root docs, `spike/CLAUDE.md`, and human-typed rulings outrank this file.
Ahistorical and kept-current: if it is wrong, rewrite it. This is a *pre-build* grading
pass — "before we even promise to build the feature" — not a how-to-build-it-safely
design. Safe-construction detail belongs to whichever lane eventually owns each item.

Consumed: `notes/28D` (the firefighter needs survey — the graded artifact) ·
`notes/28E` (the why-surface design sitting) · `notes/292-why-output-strawmen/`
(the unpoisoned aspiration corpus + its invented-capability index) · `plans/288`
(aid/loom unification, executed) · `plans/286` (the explain teaching surface) ·
`notes/289` §2t–§2z (the conduct trail) · the round-29 record `290`–`295`, `297`–`299`,
`29A`, `29B`, `284`, and `AGENTS.for-builders-only.md`.

## §1 — The shape of the tension, stated correctly

The naive framing — "explanation wants more data, security wants less" — is true but
misses where the danger actually concentrates. Dorc has a structural advantage most
tools do not: `AID-NEEDS:law-two-planes-opposite-fail` and the `aid` crate seal make the
describe plane decision-inert *at the type level* (`288` §2c; the seal survived the
extraction). So the usual catastrophe — a forensic feature quietly becoming a decision
input — is, today, hard to write by accident. That is a real and valuable containment,
and it means most of `28D` is cheaper than it looks.

+SURE the residual danger lives in three places, and only three:

- **`tension-collection`** — forensics wants durable capture of precisely the bytes
  that are simultaneously hostile and secret-bearing: applied-line stdout/stderr, argv,
  executed artifacts, host paths. Decision-inertness does nothing for confidentiality.
  A whylog that captures everything is a credential aggregator on the crown-jewel host,
  and `294:hard-truth-generic-secret-scrubbing-is-false` closes the escape route:
  arbitrary shell output has no recognizable secret type, so "we'll scrub it" is not a
  promise that can be kept. This is where most of the `28D` list's real cost sits.

- **`tension-synthesis`** — forensics wants correlation across runs and across hosts
  (fleet blast-radius, trend surfaces, cross-run diffs). Building that means building
  cross-host aggregation and cross-run persistent identity — the exact machinery
  `sinv-controller-attribution` fences behind "an explicit reviewed aggregation
  constructor", and the exact thing `KNOBS:kSTATE` is parked on, under the human's own
  standing note that unparking must happen *alongside* hostile-host work. The hazard is
  not the display feature. The hazard is that once the aggregator exists and works, its
  second customer is the planner, and that conversion looks like ordinary ergonomics from
  inside a Fable-conducted round (`292:limit-maintainers-erode-boundaries`).

- **`tension-authority-inversion`** — the decision-inert plane still causes wrong
  actions, through the human. A confident explanation at 03:40 is an action-driver. The
  sin ladder (`IMPLEMENTATION`, refined: mis-attributed is worse than un-attributed) was
  written about elision attribution; it transplants exactly onto the explain plane, and
  `28D` says so itself in `ten-backward-query-inversion`. The specific failure shape:
  synthesize a confident *negative* claim ("Dorc did not touch X") out of the least-sound
  inputs in the system (at-most footprints, reach claims — the frame-problem-limited,
  naked-trust layer), and hand it to the most credulous consumer the product will ever
  have. Nothing in the type system prevents this; it is a product-form judgment.

Everything else in `28D` reduces to ordinary care: sink encoding
(`sinv-sink-encoding`; the round-29 gap `293:finding-sanitize-all-host-derived-display`
is still open, phase four unbegun), bounded ingestion (`rul-host-bytes-bounded-before-
admission`, built), and sensitive-artifact handling (`sinv-sensitive-artifacts`,
unbuilt).

## §2 — The balance, honestly

~SUSPECT the security seat's correct posture here is *substantially permissive*, and I
want that on the record before the grades, because a reader skimming letters will
otherwise read this pass as obstruction.

Three reasons the pull toward building this tier is stronger than usual:

1. **Attribution is the design's own remedy, not a nice-to-have.** The whole
   bought-unsoundness argument (USER_STORY; `KNOBS:kHALVES` welded toward elide) is
   *paid for* by attribution: the bite is tolerable because it is attributed, short, and
   narrow. `295` reaches the same conclusion from the other side and asks for
   mint-time authority witnesses and an oracle authority-diff — which are `28D`'s
   `need-withheld-action-ledger` and `need-dependency-provenance-audit` wearing security
   clothes. Refusing the forensic tier does not buy safety; it removes the thing that
   makes the accepted unsoundness survivable.

2. **The absence is itself a trust failure.** `need-emergency-distrust-levers` and
   `need-raw-greppable-receipts` exist because an admin will not hand a mutating decision
   engine authority over their fleet without an override and a ground-truth floor. A tool
   that cannot be distrusted on demand gets distrusted permanently — which, per DESIGN's
   priority 3, self-defeats the attention product. Security-by-omission here produces a
   *less* safe deployment, because the fallback becomes hand-ssh with no record at all.

3. **In the overwhelmingly common world there is no adversary.** The managed hosts are
   trusted targets, the oracles are the admin's own or a small reviewed set, and the
   controller is one laptop. Every item on this list pays out on ordinary Tuesdays. The
   hostile-host model earns its keep on one point only, and it is a narrow one:
   *one compromised node must not become fleet-scale controller authority*
   (`294:hard-truth-compromised-host-can-lie`). Grades below are calibrated to that
   sentence, not to a general suspicion of the world.

Where the honest price falls: +SURE the expensive half of this tier is **retention**, not
computation. Computing a chain from a run costs nothing anyone should worry about.
Keeping the bytes needed to compute it *tomorrow*, on disk, by default, is what buys the
confidentiality exposure, the artifact-hardening obligation, and the pull toward
cross-run state. Nearly every C and D below is a retention decision wearing a feature's
name.

## §3 — The rubric

Grades answer one question: *how much does building this, in any reasonable form, risk
the product?* They are approval grades, never demand grades — an A argues for nothing.

- **A** — build freely. No security review wanted. Either pure derivation from data the
  controller already holds and already decides with, or a reduction of authority.
- **B** — ordinary care. Real surfaces (sensitivity, sink encoding, bounded intake), all
  of them already covered by standing invariants. Build it; the fences are known and
  cheap; no dedicated round.
- **C** — split it and scope it first. Crosses a named re-entry gate, or bundles a
  low-risk half with a high-risk half under one name, or creates a genuinely new
  collection/state surface. Wants a bounded security pass *before* the build brief, not
  after.
- **D** — do not build without its own dedicated round. Touches the decision plane,
  creates cross-run/cross-host persistent authority, or manufactures confident claims out
  of the system's least-sound inputs. A Fable-conducted lane cannot safely scope this
  alone.
- **F** — do not ship, in any form. Reserved for shapes where no fence exists. Nothing in
  `28D` graded F; §6 names the two shapes that would.

Calibration note: the modal grade is B/C. An A means I would not want to be consulted.

## §4 — The grades

### Extinguish set

- **`need-exact-input-identity` — A.** This is `sinv-decision-identity` and
  `sinv-oracle-identity-trust` asking to be built, and `294:hard-truth-approval-is-not-
  freshness` says the product owes it regardless. It records identity of inputs the
  controller already holds and already decided with; it creates no new intake and no new
  synthesis. The one fence is a framing fence the security round already stated: identity
  is not trust — a hash or a version establishes *which bytes participated*, never that
  their publisher or semantics are trustworthy. Cheapest high-value item on the list.

- **`need-ground-truth-action-ledger` — C.** Two items under one slug, and they are a
  grade apart. The dispositions half (ran / short-circuited / fell-through / elided, with
  timing, rc, and the executed artifact's identity) is B-tier and badly wanted;
  fell-through-guard counts in particular are the counter-metric `294:pressure-
  optimization-success-metric` asks for. The **captured stdout/stderr of every applied
  line** is the single largest secret-bearing intake proposed anywhere in `28D`: unbounded
  by nature, unscrubbable by `294`'s hard truth, and default-on it would make the whylog a
  standing credential harvest. Split at the brief; price capture separately as opt-in,
  bounded before allocation, classified sensitive, and never described as scrubbed.

- **`need-withheld-action-ledger` — A.** Pure derivation from decision data the engine
  already computed, on the plane built to carry it; no host intake, no persistence beyond
  the receipt that already exists, and the trust-tier filter is a query over the current
  run. It is also `295:add-mint-time-authority-witness` in product clothes. One fence,
  and it is a correctness fence more than a security one: the chain must render from the
  witness recorded *at mint*, never reconstructed later from current oracle source — a
  reconstruction can honestly disagree with what actually licensed the decision, which is
  the mis-attribution cell.

- **`need-probe-phase-account` — B.** Strongly wanted and structurally friendly: it is
  the audit surface for the read-only promise, and it is where `sinv-context-siting` and
  the escalation dial become legible to the person who owns the consequences. Ordinary
  care: it records host-side execution facts (which bodies, which hosts, exit statuses,
  claimed residue) that are small and bounded, and it must tier-word claimed residue as
  *claimed*, never measured. The reason it is not A: "including for plans nobody ever
  applied" means plan-only runs write a durable, which is one of the two pressures making
  the receipt always-on (see `fnd-whylog-default-trips-a-gate`).

- **`need-world-then-versus-now` — B.** Freshness-versus-approval is a distinction the
  security round explicitly wants surfaced, and the plan-age disclosure is close to a
  control. Ordinary care, two fences: the clock is DI'd (hermeticity, unchanged), and the
  re-measure-and-diff must be a comparison rendered *to the human*, never a loop where
  stored beliefs become an input to what gets probed or planned. Keep the diff on the
  describe plane and this stays cheap.

- **`need-exoneration-or-conviction` — D.** The sharpest item in the survey, and the
  survey's own text half-sees it. Three compounding problems. It produces *negative*
  claims, and a negative claim about what a command touched is exactly the frame-problem
  ceiling — it can only be as sound as at-most footprints and reach, the naked-trust layer
  the product already prices as its one bought unsoundness. It consumes those claims for a
  purpose their authors never licensed: a `disturbs()` arm was authored to gate a survival
  under a flag, not to assert innocence to a human — an authority-map violation in the aid
  direction (`295:add-authority-map-per-knowledge-source`). And its consumer is maximally
  credulous. The counterfactual half ("would a blind run have left the host different?")
  is worse still: it is a second, unmeasured semantics with nothing behind it. If this is
  ever built, only the positive direction may be *stated* ("these lines ran; their claimed
  footprints include X, per these authors"); the exonerating direction must render as
  absence-of-evidence with the ceiling named, never as a conclusion.

- **`need-fleet-blast-radius` — C.** Split by scope. *Within one invocation* — one plan
  across N hosts, comparing verdicts and probe answers the controller minted in that
  attempt — this is nearly free and is the highest-signal debugging artifact on the list,
  as `28D` suspects. *Across runs*, it needs retained per-host receipts plus stable
  cross-run identity, which is `KNOBS:kSTATE`'s retrofit-hostile verdict shape arriving
  through a side door. Either way it builds the cross-host aggregator, and
  `sinv-controller-attribution` requires that any cross-host combination come through an
  explicit reviewed constructor rather than an ambient convenience. The thing to fence at
  design time is not the display: it is a type-level guarantee that the aggregate cannot
  reach a license mint, so the later "and use it to plan better" step has to be a visible
  act rather than a refactor.

- **`need-partial-apply-geometry` — B.** A required control wearing a feature's name:
  the converged / known-diverged / UNKNOWN classification is
  `rul-integrity-failure-withholds-mutation` made visible, and `294:hard-truth-best-
  effort-splits-at-mutation-boundary` says the product owes exactly this distinction in
  user-facing language. Not A because the failure mode is nasty and quiet: the
  classification is computed at precisely the moment attempt integrity may be lost, and if
  it is derived from a host's own late report it will be confidently wrong in the one cell
  that forbids naive re-running. UNKNOWN must be the default when controller-side attempt
  integrity is broken, never inferred from a late or partial host claim.

- **`need-suspicion-ranked-claims` — B.** Derived, in-run, and the "this arm licensed 60
  elisions" load metric is directly the counter-metric `294` asks for against
  elision-rate-as-success. Ordinary care: keep "load" scoped to the current invocation's
  loaded set unless a reviewed aggregation exists, and take "recency of change" from
  annotation-tier source-history reads (the fence `28E:nack-whylog-stores-book-bytes`
  already draws), never by ingesting historical bytes into the receipt. Mild residual: the
  ranking is a targeting list for whoever reads it, which matters only to an attacker who
  already owns the controller — noted, not priced.

- **`need-emergency-distrust-levers` — A.** Every one of the four is an *authority
  reduction*: full-distrust reconcile removes all licenses, per-oracle quarantine removes
  one supplier's, per-claim revocation removes one claim's, and the off-ramp emits the
  book. The security seat wants this built more than the firefighter does — it is the
  manual override for every unsound tier at once, including the ones no review will ever
  fully close. Two construction fences, both cheap: the levers must work when the analysis
  is degraded or distrusted (a quarantine that requires loading the oracle is worthless),
  and the family must never grow a widening sibling — there is no `--trust-everything`,
  ever, and the flag vocabulary should make that asymmetry obvious.

- **`need-raw-greppable-receipts` — C.** Split. A versioned, additive machine envelope is
  B-tier and already has a specified posture (`AID-NEEDS`'s CI-mode contract: gate on
  codes and severity, never on finding-set identity), needing only its own sink encoder —
  JSON is a sink like any other, and `sinv-sink-encoding` forbids a universal sanitizer
  standing in for one. **Retaining executed artifacts as `.sh` files on disk** is the
  D-tier half: it is `293:finding-harden-sensitive-whylog-files` with more content and
  more durability — argv, host paths, inline values, at predictable names, with the
  phase-four/five artifact hardening unbuilt. And "raw" means, by construction,
  unsanitized: `sinv-hostile-sensitive-orthogonal` requires that raw forensic bytes be a
  deliberately separate surface from anything that renders, not the same file read twice.
  The underlying demand — when the tool is the suspect, its explanations are suspect — is
  legitimate and should be met; the meeting of it is a retention design, not a dump.

### Prevent set

- **`need-root-cause-leverage-point` — B.** Largely built (the chain's leverage epilogue),
  and it is the mechanism by which one repair heals every downstream consumer — the
  product's own answer to the wrong-claim bite. Ordinary care, two small surfaces: it
  renders third-party oracle bytes and author comments to a terminal under
  `27W:rul-report-surface-massaging`, which is a sink-encoder obligation and not
  optional; and the repair-reach statement ("heals every book downstream") is a
  cross-book claim that must be scoped to what this invocation actually loaded, or it
  quietly becomes a fleet-knowledge assertion.

- **`need-incident-becomes-regression` — C.** The engine-replay half is already fenced
  and decision-inert. The hazard is the receipt-to-committed-fixture path, and it is
  mundane and very likely: a receipt may contain secrets and host topology, and the
  postmortem reflex is to commit it or paste it into a bug report. Freezing an incident
  into a test therefore needs an explicit sensitivity gate at the receipt→fixture
  boundary, and the parse must route through the bounded admission types rather than
  adding another consumer of raw parsing — that sibling-route class is live right now
  (`29A:fnd-whylog-inspect-reopens-raw-parse`). Third fence: a pinned fact is a test
  input, never a fact source; `rec-5` holds.

- **`need-dependency-provenance-audit` — A.** `295:add-oracle-upgrade-authority-diff`
  asks for this by name, and the round's product conclusions say oracles are executable,
  security-bearing authority rather than passive metadata. A read-only audit over the
  loaded set — what is depended on, at which claim tier, from where, pinned how, and what
  changed since the last good run — is free to build and is a control I would ask for
  unprompted. Two framing fences: the diff that matters is an *authority* diff (widened
  vouches, footprints, carry, entry forms, resolver equivalences, decline behaviour), not
  a byte diff, because a version bump with identical function names can expand
  under-execution authority substantially; and an acquisition or fetch mechanism is a
  different feature with a different grade — auditing pins is A, implementing pinning and
  installation is C (`294:pressure-friendly-oracle-installation`).

- **`need-granular-trust-repricing` — B.** Directionally excellent: per-scope trust is
  strictly better than one global flag, and the staleness limit is a freshness control the
  round wants. It is B rather than A because it mints a *policy artifact*, and a policy
  artifact is a license-plane input — its integrity, its attribution, and its authority map
  are load-bearing in a way a render never is. Two fences: whatever the spelling, it must
  live where `KNOBS:kOOB`'s configuration redline permits (this is admin policy, and the
  sidecar-config prohibition is a design ruling I have no standing to bend); and per-scope
  granularity must be constructed as narrowing relative to what the invocation already
  granted, so that no scoped setting can grant authority the top-level consent did not.

- **`need-standing-drift-watch` — C.** The exit-code contract, cost classes, timeouts, and
  per-host opt-outs are already the right shape and `24R` §0a priced them honestly. What
  makes this C is the posture rather than the mechanism: a scheduled plan is unattended
  probing under standing consent, using the fleet's credentials, against the pets too
  fragile to interrogate — the shape `plans/102` recorded from the Chef why-run history,
  and the same standing-consent hazard `AID-NEEDS` flags for the LSP/MCP surfaces. Scope
  the pass to: what authority a scheduled run holds versus an interactive one, whether it
  may enter contexts at all, and what its failure withholds. The "alert when a long-elided
  line re-enters" half additionally needs cross-run memory and should be graded with
  `need-near-miss-trend-surface`, not here.

- **`need-near-miss-trend-surface` — D.** The clearest `tension-synthesis` instance,
  and `28D` names the collision itself. Counting guard fall-throughs across runs, or "90
  consecutive re-runs", requires stable per-line identity across runs plus an accumulating
  store — which is the `kSTATE` verdict shape (`(verdict, content-key, freshness)`) that
  `KNOBS` calls retrofit-hostile and the human's own note fences behind hostile-host work.
  `28D`'s thread-the-needle is genuinely the right instinct and I would endorse it as
  scoped: retained receipts, diffed by an external dumb tool, is trending without a daemon,
  and the ban is receipts feeding decisions rather than receipts existing side by side.
  But that is a *different feature* from an in-controller trend store, and the grade is on
  what the entry describes. Built as an internal accumulating store this is D; built as an
  external differ over already-durable receipts it is C; wired to anything that licenses,
  it is §6.

- **`need-consent-custody-trail` — C.** The mechanism underneath is A-tier and already
  owed: bind approval to exact bytes, record the flags, and expose the delta between the
  plan approved and the plan applied (`294:hard-truth-approval-is-not-freshness`). The
  risk is entirely in the product framing. A custody trail invites the compliance reading,
  and a best-effort local file that any local principal can edit, written with ambient
  permissions and silently-swallowed failures, is *worse than nothing* when someone
  believes it is audit evidence. `295:rank-five` gates this explicitly: whylogs may stay
  opt-in and best-effort only while they are not promoted to audit evidence. Second, this
  is where person-identity enters the product, and Dorc has no authentication story and
  should not grow one — "who typed it" is the local account, which is not a security
  boundary and must not be rendered as if it were.

### Adjacent items, graded for the record (from `28E` and the `292` strawmen)

Not part of the `28D` export; recorded here because they arrive on the same lane.

- **`28E:rul-sh-rewrap-is-load-bearing-scope` / `prop-three-literalness-modes` — C.**
  The FORMATTED mode emits *valid, runnable sh* rewrapped from analyzed source, and
  `sinv-sink-encoding` singles shell generation out: it needs type-level quoting
  appropriate to the exact grammar, not a general sanitizer. A rewrapper that is
  correctness-preserving on well-formed input and subtly wrong on a hostile or exotic
  quoting shape produces output the user is invited to copy and run. The newest prior-art
  round reached the same place independently (`dont-let-the-readability-transform-be-
  unsound` — its named sharpest artifact), which is corroboration from a lane that was not
  thinking about security at all. The DESCRIPTIVE mode is safer precisely because it is
  marked non-runnable; keep that marking load-bearing.

- **`292:inv-claim-blast-radius` ("2 other books on this controller rest on this claim")
  — D.** This is `need-fleet-blast-radius`'s cross-run form arriving inside a render.
  Knowing what other books on this controller depend on a claim requires a persistent index
  of books, hosts, and claims — cross-run state, minted for display, sitting one refactor
  from the planner. If some version is wanted, scope it to "oracles loaded by this
  invocation", which is free, and refuse the controller-wide index.

- **`28E:lean-git-source-tracking-secondary` — B.** Digest-keyed, exact-or-absent VCS
  lookups are a good shape and the human already fenced them at annotation-tier
  (`28E:nack-whylog-stores-book-bytes`). Ordinary care: it spawns a VCS process and reads
  a repository whose contents are not the controller's to trust, so its output is
  host-class input for encoding purposes, and it must stay strictly optional — the "I
  slept, why did it break" path cannot depend on it.

- **`28E:ask-cell-human-description` (a `__describe`-shaped display member) — C.** The
  fenced trap in `28E` §5 (a description explaining a confusing cell is usually a missing
  model distinction) is the right one and I have nothing to add to it. The security
  question is orthogonal and unasked: if the member *executes* on the host, it is a new
  probe-time execution surface with a new output channel, and it inherits the whole
  read-only contract, the report-lane ownership rules, and the sink-encoding obligation
  for whatever it prints. If it is read statically from source, it is nearly free. Decide
  which one it is before minting the role, not after.

- **`28E:rul-tree-render-is-a-firewalled-crate` — B.** Firewalling it is the right call
  and the needs-inventory-before-shopping rule is doing real security work by accident:
  every rendering library adopted here would sit on the path that handles host-derived
  bytes. Ordinary care — `cargo deny` already runs in the gate set; keep the crate's
  dependency surface small enough to read.

- **`28E:prop-parts-at-birth` / `prop-carrier-to-the-edge` — A.** Structurally
  pro-security and worth saying out loud: keeping aid output as typed parts until the rim
  is exactly what per-sink encoding requires. Every string flattened early is a place
  where the encoder cannot be applied later. If these land, several C items get cheaper.

## §5 — Findings

- **`fnd-whylog-default-trips-a-gate`** — +SURE. `28E:lean-why-is-whylog-
  reconciliation` folds `dorc why` onto the receipt and, in `28E` §4's words, "hardens the
  always-on whylog requirement" — the `--whylog-dir` opt-in was a disclosed spike cut, and
  zero-setup-recovery makes default-on load-bearing. That is precisely
  `295:rank-five`'s named re-entry trigger: whylog filesystem and privacy hardening
  re-enters *before* default-on persistence, audit claims, or third-party use. The
  hardening is unbuilt (phase four not begun; `29A` §8 item 2). This is not an argument
  against the fold, which is a good simplification — it is a sequencing fact: the fold and
  the artifact hardening ship together or the fold ships opt-in. `28E` §4 already says
  "sensitivity fence rides along"; this is that fence, named, with its gate.

- **`fnd-retention-is-the-real-currency`** — +SURE. Eleven of the eighteen items are
  cheap to compute and expensive to *keep*. Whatever lane opens this tier should make one
  retention decision (what is durable, for how long, at what permissions, classified how)
  and let the features inherit it, rather than letting each feature negotiate its own
  storage. The alternative is what `293`/`295` found in the existing whylog: predictable
  names, ambient permissions, swallowed failures, unbounded replay — five small decisions
  that each looked local.

- **`fnd-forensic-tier-is-an-attribution-tier`** — ~SUSPECT, offered as framing. The
  overlap between `28D`'s list and `295`'s unowned re-entry gates is large enough to be
  worth exploiting: mint-time authority witnesses (`need-withheld-action-ledger`), oracle
  authority-diff (`need-dependency-provenance-audit`), decision identity
  (`need-exact-input-identity`), and integrity-failure disclosure
  (`need-partial-apply-geometry`) are one build with two justifications. A lane that
  builds them for the firefighter discharges four security gates for free, and does it
  with product motivation rather than security nagging — which, given the firewall, is the
  only way those gates are likely to get built at all.

- **`fnd-two-plane-seal-is-load-bearing-for-this-tier`** — +SURE. Every B in §4
  depends on the seal holding. `288` §2c records that the seal is type-level rather than
  co-location, and survived the `aid` extraction; `289` records the mint-hardening gate
  that caught a never-minted collapse class on day one. Both are good news. The exposure
  is that the seal now spans a crate boundary and its enforcement is a set of private
  fields plus a `compile_fail` doctest — that is adequate, and it is also exactly the
  class `295:add-boundary-erosion-review-trigger` says erodes quietly. Any change that
  gives the describe plane a path back into a license input is a security-design event
  regardless of how ordinary the feature sounds.

## §6 — The two shapes that would be F

Neither is proposed by any document read here. Named so that a later lane recognizes them
if a build drifts into one.

1. **Retained cross-run or cross-host observation licensing anything.** A trend store, a
   fleet correlation, or a receipt that feeds a verdict — any path where evidence gathered
   about host A, or during run N-1, contributes to what is skipped on host B in run N,
   without an explicit reviewed aggregation constructor and the hostile-host work
   `KNOBS:kSTATE` fences it behind. This is the one place a forensic feature can convert
   into fleet-scale under-execution authority, and it is the human's own stated redline.

2. **A confident automated exoneration.** "Dorc did not touch X", asserted rather than
   bounded, or a counterfactual presented as a prediction of what would have happened.
   There is no fence for this one because the error is in the sentence, not the plumbing:
   it is the mis-attribution sin, generated at scale, aimed at someone who has no time to
   check it. The bounded form is buildable (§4, `need-exoneration-or-conviction`); the
   confident form is not, at any effort.

## §7 — What crosses the gate

Appended to `notes/28D`: one `opaque-approve <letter>` token per item, plus a four-line
legend giving the scale's direction and stating that it is an approval grade which never
argues for building anything. No rationale, no invariant names, no threat content, no
mention of which concerns drove which letter. A reader who wants to know *why* an item
graded C learns only that it did.

Per `295`'s export process, anything that firms into a construction constraint leaves
separately, as a truthful ordinary-engineering invariant in `spike/CLAUDE.md` with its
private mapping recorded in `29A` §6 — not through this document and not through the
grades. Nothing in this pass firmed that far: these are pre-build grades on unbuilt
features, and the correct export for that is a letter.

## §8 — Confidence

+SURE: the three tensions (§1); the rubric's application to the collection and synthesis
items; `fnd-whylog-default-trips-a-gate` (read directly against `295`'s re-entry
list and `29A`'s phase table); that the two-plane seal is type-level and survived the
extraction (read in `288` §2c and `289`, not re-verified against the Rust this pass).

~SUSPECT: the split-points I assigned inside `need-ground-truth-action-ledger` and
`need-raw-greppable-receipts` — those are my reading of where the cheap half ends, and a
build brief may find the seam elsewhere; the D on `need-exoneration-or-conviction`, which
is a product-form judgment more than a mechanism finding and which the human may
reasonably price differently; that `28E`'s `__describe` candidate would execute on the
host (the note does not say, and the answer changes its grade).

-GUESS: the relative ordering within the B tier; whether `need-fleet-blast-radius`
within-invocation is really as cheap as I have priced it (it depends on multi-host
machinery that does not exist, per `293:map-reactive-and-cross-host-machinery`, so the
pricing is against a design, not a build).

NOT verified this pass: any of the round-29 code claims (taken from `29A`'s phase table,
which was itself read from the tree on 2026-07-24 and is nine-plus commits stale by now);
the current state of `spike/crates/aid` beyond what `288`/`289` assert; whether phase four
has moved since `29A` (assumed not begun).
