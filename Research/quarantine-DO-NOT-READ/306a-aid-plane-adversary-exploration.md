# 306a — The aid plane under an adversarial host: an exploration, not a ruling

> Tier: **LLM-authored, lower-reasoning agent** (Opus, not Fable; the session began under
> Fable, branched, and switched models before the quarantine was opened to it). Quarantined
> security material: never expose to an ordinary out-quarantine conductor. Its sanitized
> sibling is `notes/306b`.
>
> **CONFIDENCE POSTURE — read before anything else.** This document contains exactly two
> classes of hard content: (1) **things the human said**, marked [HUMAN]; and (2) **things a
> subagent reported a cited source says**, marked [SOURCE] with the graded slug. *Everything
> else is my own supposition and should be read as suspect.* I reversed myself twice during
> the sitting on load-bearing points (§6, §5b), in both cases because I had reasoned
> confidently from a premise I had not tested. That is the calibration to apply to every
> unmarked claim below. Confidence markers are `+SURE / ~SUSPECT / -GUESS / --WONDER` and
> I have tried to use them honestly rather than decoratively; where I write +SURE it is
> almost always because I read the tree or the human typed it, not because I reasoned well.
>
> Scope carve, deliberate: the specific engineering invariants and steering-law edits this
> sitting produced are NOT restated here — they live out-of-quarantine (see §9). The
> research subagent's full report is likewise pointed at, not reproduced.

## §0 — The headline: we learned nothing decisive

The honest summary of a long sitting is that **this problem space is extremely subtle and
almost every signal we found pulled against another signal we found.** [HUMAN, stated
outright at the close: the syntheses are mutually contradictory, and that is validation
that the topic is genuinely hard with strong locally-correct pulls in both directions;
this will require the human's attention, tuning, and taste rather than a mechanism.]

I record that as the primary finding because my own instinct throughout was to converge —
to present tidy syntheses of findings that do not actually reconcile. The human caught
that. ~SUSPECT that an LLM working this space will *always* drift toward premature
synthesis, and that a future reader of this document should treat any confident-sounding
resolution in it as an artifact of that drift rather than as a conclusion.

What we *did* establish is a vocabulary, a set of failure shapes, one process finding
about the quarantine architecture itself, and a small number of human rulings. Those are
below in roughly that order of durability.

## §1 — What the human ruled (the load-bearing content)

These are the parts of this document that are not supposition.

- **`rul-induced-authority-spend-is-the-worse-evil`** [HUMAN] — an attacker causing us to
  *do nothing* is categorically better than an attacker causing us to *reach out and touch
  a host with authority-blessed action*. Suppressing a repair leaves the adversary with
  what they already had; inducing privileged action hands them something new. Chosen
  eyes-open as the lesser of two evils, not as a mitigation.
- **The control-plane conclusion** [HUMAN] — security-critical attention *usually does not
  respond* to the elide ≻ guard ≻ run ladder. For a single host, where the stake is only
  what we do to that host, choosing among those three is not a security control: the
  adversary can induce any of them. Demoting buys no bastion, and doing so on the strength
  of poisoned input is close to security theatre — real user-value discarded for no taint
  win.
- **Continue, but refuse to mutate** [HUMAN] — the resolution to the above. On an
  integrity failure: no fail-fast; keep probing; keep analyzing; **and this reverses the
  partial-consumption ban** — consumption proceeds into a variant that structurally cannot
  produce a plan step. The whole plan for that host dies, granular probing and analysis
  continue, and the product is a full analytic report and root-cause analysis, threaded by
  taint so it can be correctly disclaimed.
- **Debug-triggered probing needs a defensive mode** [HUMAN] — any probing outside the
  ordinary plan/apply cycle must refuse to deploy licensure, authority, or secrets.
- **On secret-avoidance** [HUMAN] — we cannot promise not to record secrets, because we
  are referentially agnostic and cannot know what a secret *is*. Anything of that shape
  must therefore be brutal and high-level rather than a filter: store no freeform probe
  output; keep host-text on the host; pull it at debug time; accept that the material may
  be lost if the host dies or is corrupted. The `dorc-records` stream we are forced to
  accept can only be env-free **by contract**, never by construction, testing, or proof —
  nothing stops an oracle author writing `env >> $DREP`.
- **On statelessness and re-ingestion** [HUMAN] — because Dorc is stateless there is no
  meaningful difference between "this run" and "a run fifty days ago"; multiple stateless
  runs are one user-run in a single sitting. Aid state must therefore survive across runs.
  (This corrected a schema-incapacity rule I had proposed; see §4c.)
- **The conceptual/durable distinction** [HUMAN] — "the whylog is the spine" is a claim
  about data structures and purity architecture, not durability. The conceptual whylog is
  the single owed output of the entire engine; every apparent product (the executable
  artifact, user output, orchestrator connections) is a function taking it as input. The
  `.whylog` file is *itself* one of those functions: a lossy projection. The durable is
  **not permitted to be poor — it may be forced to be poor**, and that is a high cost that
  must not be bought cheaply.
- **The five-part architecture** [HUMAN, stated as a whole and asked to be checked for
  coherence] — (1) engine state is one product, designed through the lens of
  aid/knowledge/statements/certainty/reporting; (2) most or nearly all of it becomes
  durable and re-ingestible for owed explanations; (3) every component is tagged in various
  ways, host-influence-taint being the relevant one here; (4) re-ingestion must *rehydrate*
  taint (global invariant: no laundering during re-ingestion) plus a defense-in-depth
  constraint that we never take actions based on re-ingestion; (5) a tiny class of
  too-risky/referent-agnostic-impossible material gets a last-ditch opt-in door of
  additional acts past re-ingestion, via two lanes — freeform host-side cruft, and
  structured reprobing.
- **`rul-induced-*` is modulo kSTATE** [HUMAN] — security concerns add justification to
  kSTATE; they did not mint it, and because kSTATE is a knob the human can turn, this whole
  discussion stays kSTATE-agnostic.
- **Taint has a computable frontier** [HUMAN] — we hold the tools to know precisely where
  taint starts; a report is not uniformly tainted. The render consequence is dualistic
  (trusted region, banner, derived-from-host-claims region rooted at a named node),
  influencing sorting and graph disposition. Deferred.
- **Positioning** [HUMAN] — we are an orchestrator; operator-deception is more our home
  turf than anyone's, because we sit in front of a user wearing their largest-blast-radius
  hat (tunnels open, hardware key unlocked, password manager unlocked, hosts listening) —
  maximum attacker leverage. But our product positioning is the *opposite* of our siblings':
  we promise gradual enhancement, maximal aid, maximal attribution, so we have the *hardest*
  time disclaiming or dispositioning that responsibility.

## §2 — Archaeology: how the two-parser split happened

### §2a — What I verified in the tree (+SURE; this is reading, not reasoning)

- `records::deframe` is `pub` in `dorc-plan`, and `LegacyPolicy::Tolerate` — the mode whose
  own doc-comment says it "would bypass EVERY integrity key" — is `pub` alongside it.
- Every surviving call site is in `crates/cli/src/main.rs` at lines 4027, 4268, 4384, 5151,
  5605, all inside the `mod tests` that begins at 4016 under `#[cfg(test)]` at 4015. Two of
  them pass `Tolerate`.
- Three lexical fences exist and are well built — `fixture_intake_is_unreachable_from_production`,
  `fixture_payloads_are_unreachable_from_production`, `foreign_edge_constructor_is_fenced` —
  and the first asserts a non-empty walk so a broken scan fails rather than passing
  vacuously. **None of them names `deframe` or `LegacyPolicy`.**
- Production intake runs through `read_host_evidence` → `admit_unscoped_host_records`, which
  reports every problem through one code with a closed engine-owned reason enum, and whose
  posture is refuse-the-whole-attempt.
- `plan/src/whylog.rs` stores the records stream **as received**, raw, byte-count-prefixed
  (`raw_results: String`), and `whylog::parse` is `pub` over raw input and re-reads it on
  replay. Book and oracles are stored **by reference** (path + digest, re-read from disk,
  mismatch ⇒ `WhylogBookDesync`).

### §2b — What the round-29 record says (+SURE on the reading)

`29-reviewA` returned **ACK** with two `~SUSPECT` residues, one of which is verbatim ours:
"legacy/raw surfaces surviving beside the new admission route," recorded in the closeout
handoff as *"fenced, not blocking."* The catchup ledger's own §3 is more accurate:
`LegacyPolicy::Tolerate` "remains reachable in principle"; `deframe` and `whylog::parse`
"remain public over raw input"; all phase-five work. §8 item 3 lists **"legacy-parser
fencing"** as outstanding.

The tree agrees with the ledger, not the handoff. I have added a supersession marker at the
handoff pointing here (`ai/main` `059a6f51`).

### §2c — The process findings (~SUSPECT on the generalization, +SURE on the instance)

- **`finding-quarantine-cannot-order-deletions`** — the opaque architecture can issue
  *construction* constraints to builders ("when you build X, satisfy Y") but has no channel
  for *deletion mandates*, because a deletion is a product decision, product decisions
  belong to a conductor, and a conductor cannot be told why. ~SUSPECT this is the single
  largest structural gap in the arrangement, because hardening a system consists largely of
  deleting the permissive path you replaced. The round-29 instance is the evidence:
  `sinv-production-fences` named "legacy/headerless records" *explicitly, in writing*, and
  the cleanup still did not happen.
- **`finding-residue-surfaced-in-the-wrong-costume`** — when the residue eventually crossed
  the firewall, it arrived as nine orphaned catalog codes inside a prose-authoring census,
  i.e. as a *hygiene* item. The conductor (me, earlier in this same session, running as
  Fable) accordingly analyzed it as a **taxonomy** question and demoted the parser
  divergence to a closing rider. ~SUSPECT this is the more insidious half: the firewall did
  not merely withhold rationale, it handed the decision-maker a misleading frame, and
  competent reasoning then reached the wrong altitude.
- **`finding-tripwires-do-not-schedule`** — a trigger keyed to "when you build X" never
  fires if nobody builds X. Tripwires protect future work and do nothing for standing
  residue. -GUESS that any real fix needs a channel carrying *priority* without *rationale*
  to a conductor; I do not have a design for one.

## §3 — The taint model, as far as it got

Not built, not fully acked, recorded as vocabulary.

**Three influence tiers, monotonic, absorbing** (my proposal, refined by [HUMAN] pushback):

- `authored-before-contact` — invocation, flags, scope, nonce, the book and oracle bytes,
  the round-zero analysis. Untainted because it predates any host contact.
- `host-spoken` — raw bytes a managed host produced.
- `host-influenced` — anything computed from host-spoken material: our subsequent
  questions, our schedule, our decisions, our narration. **Absorbing**: anything touching it
  becomes it.

[HUMAN] **nearly nothing after the first probe dispatch escapes the third tier** — once the
controller starts receiving and acting on host output, the layer has shifted. Not a hard
rule; tunable by structurally fencing regions from tainted input.

Two corrections the human made to my reasoning here, both of which I think are right and
both of which I had gotten wrong in a way that flattered our own machinery:

1. **I laundered attacker control through controller authority** [HUMAN nack]. I classified
   arrival order, deadline firings, and partial-arrival decisions as "controller acts, not
   analysis conclusions," and therefore authoritative. The human named this as exactly the
   dangerous class of assumption the sitting existed to examine. On inspection they are
   right: the controller minted the *timestamp*, but the adversary chose the *state of the
   world at that timestamp* — a host that stalls forces the deadline, one that answers fast
   avoids it, and in the multi-host case a host picks its position in the arrival order.
   ~SUSPECT the general form: `sinv-controller-attribution` is a rule about *who is
   speaking* (anti-forgery); it says nothing about whether content is adversarially
   influenced. Attribution, sensitivity, and **influence** are three orthogonal axes and I
   collapsed the third into the first. The corpus already insists on the first two being
   orthogonal; the third appears to be unnamed.
2. **Input-versus-output is round-relative, not absolute** [HUMAN]. I argued for
   "store inputs, re-derive conclusions" and then immediately required storing probe sets
   "for correspondence" — but a probe set *is* a conclusion, computed from attacker bytes.
   The human's point: we are forced to durably store conclusions-derived-from-attacker-output,
   because those conclusions are what make subsequent bytes interpretable. There is no
   reproducible multi-round design that avoids it. The consequent split, which I think is
   the useful part: **the sledgehammer (store no freeform host output) solves *sensitivity*
   and does nothing for *taint*,** because the attacker-shaped material we must keep is not
   their bytes but our conclusions about their bytes.

~SUSPECT the taint is *computable* rather than merely declarable, because the kernel is
pure: a value is host-influenced exactly when it depends on host-spoken input, which is a
dataflow property over a pure function. If that holds, declared tags are a cheap always-on
approximation and computation is the ground truth to audit against. **Not acked**; the
human explicitly left "trust-tiers with monotonic demotion" unsettled ("maybe? needs
attention"), and per the silence-is-not-ack discipline nothing should build on it.

One hardening I proposed and the human acked in substance: an absent or unverifiable taint
tag must **rehydrate as tainted**, so that stripping tags is a self-defeating attack that
can only demote. Plus [HUMAN] a cryptographic hash over the durable as cheap
defense-in-depth, competing with nothing.

## §4 — The durable projection

### §4a — What I proposed and what survived

My framing was "the security lives on the read side, so the write side is nearly
unconstrained." ~SUSPECT this was *directionally* useful and *overstated*, and the human
demoted it correctly: the authority distinction must hold at runtime, in memory, in exactly
the same way; writing it down is a promise, not a new discipline. The only genuinely
replay-specific addition is the recorded-versus-re-derived comparison, which has no runtime
analogue.

What I still believe (-GUESS to ~SUSPECT):

- Storing the book and oracle bytes **by value** rather than by reference is a net win on
  three axes simultaneously — aid (the hot loop means the book has usually changed by the
  time anyone asks `why`, and by-reference answers `WhylogBookDesync` at exactly the moment
  of maximum need), sharing (a by-reference durable is meaningless to a colleague), and
  security (an input that is re-read from mutable disk at replay time is an input an
  attacker can change after the fact).
- Per-site decision records rather than one scalar digest turn "this does not reproduce"
  from a wall into a diff. The human's `rul-whylog-is-the-spine` already demands the
  `SiteId` keying for unrelated reasons.
- Storing **both** inputs and conclusions, and re-deriving to compare, makes tampering,
  version skew, and engine nondeterminism into a *detected* condition. ~SUSPECT this is the
  cheapest defense-in-depth available in the whole design, because it reuses machinery that
  already exists (the sparing re-derivation's Confirmed-or-Demoted shape).
- Replay's chain of authority must run **through the records, never through the stored
  conclusions**: re-derive each round from the previous round's re-derived state plus the
  stored records, using stored questions only to align and diff. Round zero is genuinely
  clean, which is what makes the chain anchorable. It degrades honestly — reproduced
  through round two, diverged at round three, everything past labelled.
- A four-way replay status axis (re-derived now / recorded then / both agreeing / both
  disagreeing) is computable and keeps stored conclusions safe to *consume* by never
  letting them wear a re-derivation's clothes.

### §4b — The two point-five lanes

[HUMAN] the last-ditch door: freeform host-side cruft, and structured reprobing. My
observations, ~SUSPECT:

- Lane one's material has been sitting on **host-controlled storage** since the run, so it
  is not merely tainted but tainted *at a later timestamp than the decisions it purports to
  explain*.
- Lane two interrogates a world that has moved.
- Both will be reached for precisely when someone is reconstructing a past decision, and
  both will *feel* like they supply past state. Rendering either as contemporaneous with the
  run is a temporal-attribution error. [HUMAN acked, and noted this is well-known in
  firefighting-tool design; any such mode will be aware of it.]
- Lane two additionally needs a consent gate distinct from plan/apply — the operator asked a
  question, they did not authorize a run — at a moment when no plan is on screen to disclose
  anything. [HUMAN acked and extended: a defensive mode that refuses to deploy *any*
  licensure, authority, or secrets.]

### §4c — Two of my proposals the human rejected, recorded because the reasons generalize

- **`must-not-write-controller-env-values`** — withdrawn. I proposed writing env names and
  grades but not values, with an opt-in flag. [HUMAN nack]: this violates the standing
  never-promise-not-to-record-secrets posture; we are referentially agnostic and cannot know
  what an env value *is*. ~SUSPECT the generalizable lesson is that **any rule of the form
  "detect the sensitive thing and omit it" is unavailable to us by construction**, and
  reaching for one is a recurring failure mode I should expect to repeat.
- **`must-not-express-cross-run-state-in-the-schema`** — withdrawn. [HUMAN nack]: because
  the engine is stateless, `dorc plan && dorc why` and `dorc plan; sleep forever; dorc why`
  are the same thing to it; aid state must survive across runs. I had reasoned from the
  wrong side of rec-5, conflating "no cross-run *license* input" with "no cross-run *aid*
  reading."

The second of those also surfaced a live documentation defect: `rul-whylog-is-the-spine`'s
boundary (3), as typed, read "consumers read the whylog OF THEIR OWN RUN, never a stored
one," which outlaws `dorc why --last` — a feature the corpus has specified since r27. [HUMAN]
identified this as their own term-collapse between the conceptual whylog and the durable.
Narrowed in place on the sibling conductor's branch (`ai/r30-conduct` `6190e600`).

## §5 — The control plane

### §5a — The ladder does not respond to security attention

[HUMAN, hard ack] — for a single host where the only stake is what we do to it, the
adversary can induce elide, guard, or run at will, so choosing among them is not a control.
My contribution, ~SUSPECT but I think it holds: **our safe-direction doctrine is calibrated
against ignorance, not malice.** Unsure-⇒-run is correct when the unknown is the world's
messiness; against an adversary "run" is simply one of the outcomes they can select. The
ladder is a correctness ordering under uncertainty and not a security ordering under
adversity.

### §5b — Where I think there is still a mechanical residue (~SUSPECT, and I reversed here)

I initially recommended full-host demotion on integrity failure; the human's argument
against it (above) is what moved me. What I believe survives:

- There is a **fourth cell not on the ladder**: withhold. Elide, guard, and run are all ways
  of *spending our authority on the host*; withhold's induced outcome is that no authority
  is exercised at all. That converts the adversary's win from "use the operator's privilege
  on my terms" into "deny the operator's orchestration," which is the cheapest failure in
  the set and the only *loud* one. The machinery exists (`Refused` returns before plan
  construction).
- **The run direction is not neutral.** Where the attacker's foothold is *less* privileged
  than our connection — unprivileged web user, Dorc connecting as root — inducing a run is a
  privilege-escalation primitive they did not otherwise have: a root-context command, at a
  time of their choosing, possibly carrying controller-side material. Inducing an elide gives
  them nothing, since they already control the host's state. -GUESS the sharpest corollary:
  a guard is `( check ) || original`, i.e. an **attacker-answerable switch in front of a
  privileged command**, so "demote everything to guards" may hand them a *cleaner* trigger
  than either alternative. Conditional on the privilege asymmetry; where the attacker is
  already root-equivalent the human's symmetry holds completely.
- ~SUSPECT the remaining mechanical decision is therefore small and specific: for each
  admission failure, does it route to withhold (no plan emitted) or to conservative planning
  (everything runs)? Those are genuinely different postures, not two flavours of one
  fallback.

### §5c — The resolution the human reached

[HUMAN] **continue-but-refuse-to-mutate**, with partial consumption *into a report-only
variant*. I record my own error here because it generalizes: I had banned partial
consumption globally, when the justification was only ever plane-scoped. Accepted records
are dangerous because they become facts and facts license plan steps; point the consumption
at a type that cannot produce a plan step and the hazard evaporates, leaving only aid-plane
taint, which is handled by prose discipline. **The two-plane law was always the right cut;
I applied a license-plane rule to the aid plane and called it a ban.**

My implementation cautions, ~SUSPECT: the containment wants to be a *type* rather than a
flag (a flag eventually goes unchecked), and it wants to sit at the *analysis output* rather
than at plan emission, because facts are cross-cutting and a fold that produces ordinary
facts leaks into survival, wall-walk, and the decision record even when no plan is emitted
for that host.

And one bound I proposed on "keep probing" [HUMAN acked and generalized]: continuing to
probe an integrity-failed host still executes oracle-authored bodies under our connection's
authority, and under the context-entry dial that execution may *shift context*. Gathering
read-only forensics is defensible; escalating into shifted contexts on a host we have just
declared untrustworthy is a different act.

## §6 — The records-8 arc, recorded as a worked example of my unreliability

I recommended deleting the nine `records-*` codes. [HUMAN nack.] On re-derivation I found
two errors in my own reasoning, and I record them because the *shapes* seem likely to recur:

1. **I treated "attacker-controlled" as equivalent to "worthless or harmful."** Attacker-
   controlled evidence is still evidence — about what the attacker did, which is what
   forensics wants. The hazard was never that material is influenced; it is that we might
   present influenced material as though it were not. That is a rendering discipline, and I
   converted it into a deletion.
2. **I invoked a catalog law without applying its own test.** `rul-reason-enums-not-sibling-codes`
   tests for *different world-states with different repairs*. Applied honestly, the nine
   pass: torn (writes truncating — channel, buffering, memory pressure), glued (interleaved
   atomic writes — concurrency), alien (something not-us writing, or a stale attempt — retry
   hygiene), late (records after the sentinel — a wrong `wait`, an inherited fd),
   integrity-refused (**we may not be talking to the machine we think we are**), and so on.
   The law licenses them; I cited it as though it forbade them.

The decisive argument, which I had simply never run: **collapsing removes the operator's
discrimination, not the attacker's control.** With one code the adversary still chooses
whether records arrive, whether they are malformed, whether the stream refuses; the operator
merely has less material with which to notice something is strange. The anti-steering
benefit was illusory.

Two source findings then make collapsing arguably harmful rather than merely useless
[SOURCE]: T1070 documents adversaries curating selectively to leave "sufficient data intact
to maintain the appearance of normal system behavior" [A-mitre-attack-indicator-removal-2026],
so a design in which every wire pathology renders identically is one in which deliberate
manipulation is indistinguishable from a flaky pipe; and AF447 is the same shape one level up
(§7).

What survives: `deframe`'s sin was never its vocabulary but its **acceptance semantics** —
tally-and-continue returns the records that did parse, so an adversary can pad a stream with
garbage and have a forged record believed. ~SUSPECT that under [HUMAN]'s §5c ruling the
disposition is now *re-home behind a type boundary* rather than delete: a report-only lane
has a legitimate use for forgiving parsing, provided the plan lane cannot name it.

## §7 — The research: where the signals contradict

Twenty-four sources, sixteen A-grade, all fully read by the research subagent (same
capability tier as me, less context). **Full report and graded manifest:
`.claude/research/quarantine-DO-NOT-READ-patronizing-aid/` — pointer, not repeated here.**
Everything in this section is [SOURCE] as reported by that subagent; I read the lifted
findings and the AF447 and expert-population citation blocks directly, not the whole notes
file.

**The contradictions, which are the point:**

- **Expert caution, opposite signs by method.** Field telemetry over 25.4M impressions:
  skilled populations click through *more* — Firefox phishing 34.8% on Linux vs 8.9% on
  Windows; Chrome malware 54.8% on Canary vs 23.2% stable; the paper itself floats "feel
  patronized by warnings" as a candidate cause [A-akhawe-felt-warningland-2013]. Survey
  work says the opposite: experts 13% vs non-experts 41% proceeding on unknown-CA
  [A-sunshine-crying-wolf-2009]. Unresolved. The subagent leans field data and so do I, but
  this is load-bearing for the entire "our users are experts" tune and neither of us settled
  it.
- **AF447 versus the whole warning-fatigue literature, head-on.** The stall warning was
  designed to go inoperative below 60kt because "the airflow must be sufficient to ensure a
  valid measurement… especially to prevent spurious warnings" — i.e. *don't render what your
  measurement source can no longer vouch for*, stated by its designers, in a safety-critical
  system. Outcome: nose-down (correct) input made the alarm sound; nose-up (wrong) input
  silenced it. The BEA's remedy was **more raw material** — permanent warning operation plus
  display of an angle-of-attack parameter never shown to pilots at all
  [A-bea-af447-final-report-2012]. Meanwhile habituation, alert-fatigue and SOC evidence all
  say suppress harder. ~SUSPECT this is the single most important tension in the base for us,
  because the suppression rule AF447 indicts is *verbatim* the rule I proposed for tainted
  narration.
- **Verbosity helps action and destroys attention.** Verbose notifications outperformed
  terse and *reduced* follow-up questions [A-li-vulnerability-notifications-2016]; alarm and
  ops literature say over-monitoring is harder to fix than under-monitoring. The subagent's
  reconciliation, which I find plausible but unproven: one generous push plus wide-open pull,
  never a middle setting.
- **Positive indicators: withdrawn in browsers, added in supply chain.** Chrome removed the
  EV badge on a stated principle of "neutral, rather than positive, display"
  [A-chromium-ev-to-page-info-2019]; npm simultaneously shipped a positive provenance badge
  [B-github-npm-provenance-2023].

**Findings that pulled one way without an obvious counter:**

- Progressive disclosure is near-dead: 1.6% opened Chrome's "Help me understand", *zero*
  Firefox users opened "Technical Details", 16.8% of notified *network operators* followed a
  detail link [A-akhawe-felt-warningland-2013, A-li-vulnerability-notifications-2016].
  ~SUSPECT the direct consequence for us: nothing safety-relevant may live only behind a
  pull.
- Habituation at *realistic* rates: five warnings a day, adherence 87%→55% over fifteen
  working days; varying the warning's appearance held 76%, and participants consciously
  knowing about the variation did not defeat it [A-vance-tuning-out-security-warnings-2018].
- Experts trust static analysis *more*, not less — expertise correlates with higher use and
  higher belief the tool catches real faults [A-christakis-bird-program-analysis-2016].
  ~SUSPECT this kills the comforting assumption that our users will catch our mistakes.
- The tolerance envelope: only 24% of developers tolerate a 20% false-positive rate; Google's
  compile-time bar is under 10% *effective* FPs; Tricorder auto-disables an analyzer whose
  not-useful ratio exceeds 10% [A-sadowski-static-analysis-google-2018]. And the reframe:
  an **effective false positive** is a true finding the operator does not understand and does
  not act on — *the operator sets the rate, not the tool author*.
- Diagnostic steering is measured in the **benign** case: a truthful message naming the
  subclass rather than the parent sent 49 participants to the wrong fix and 1 to the right
  one [A-barik-compiler-error-messages-2017]. ~SUSPECT this is the most transferable single
  result in the base for us — the adversarial case is a corollary of a demonstrated benign
  effect, and it means even perfectly honest aid steers repair.
- Explanations increase acceptance of a recommendation *regardless of correctness*;
  confidence display, not explanation, drives the mental model; truth-default means that
  absent an explicit suspicion trigger a diagnosis is simply believed
  [B-bansal-ai-explanations-2021]. -GUESS the design consequence: a provenance mark's job is
  to be the *suspicion trigger*, not decoration and not hedged prose.
- Per-user output customization killed static-analysis adoption at Google; because each user
  saw a different view, no issue could be relied on to have been seen. Configuration moved to
  project scope [A-sadowski-static-analysis-google-2018].
- Operator-console deception is shipping and unpriced: Tomcat CVE-2025-55754, ANSI injection
  into an admin log console inducing an attacker-chosen command, rated "low" by the vendor and
  9.6 CRITICAL by CISA [B-tomcat-console-manipulation-cve-2025]. [HUMAN] observed that this
  signal pulls hard for us specifically, since we sit at maximum operator leverage; -GUESS we
  should adopt the CISA end as our internal calibration, since there is no industry price to
  borrow.
- Bainbridge's followability constraint binds the *analyzer*, not the renderer: the machine
  should decide "using methods and criteria, and at a rate, which the operator can follow,
  even when this may not be the most efficient method technically"
  [A-bainbridge-ironies-of-automation-1983]. ~SUSPECT this sits in real tension with our
  spend-analysis-freely perf doctrine, and bites hardest at the survival tier, whose chains
  are already six links deep.

**The gap the base does not close** [SOURCE, subagent's own assessment]: nobody measured
whether *marking provenance* changes expert behaviour. Mark-of-the-web, Gatekeeper,
mixed-content, external-sender banners — no effectiveness studies surfaced. The mechanism
this sitting leaned on hardest is **unevidenced in either direction.**

[HUMAN] noted the corpus already holds prior art on warning fatigue and error filtering, and
that this round appears to have re-derived some of the same results.

## §8 — Open, and what I would not claim

- Whether influence-tiering should be a monotonic-demotion trust tier at all: **not acked**.
- Whether marking works on experts: **unevidenced** (§7).
- The stated-versus-actual expert gap: **unresolved**.
- The routing of each admission failure to withhold versus conservative-plan: **owed, human's**.
- Cross-host: **explicitly carved out** by the human and untouched here. Note that the aid
  channel is a cross-host vector *without* any engine-level cross-host dependency — a host
  that steers the operator's diagnosis affects the operator's whole estate — so ~SUSPECT the
  carve is narrower than it sounds.
- Host power that is not power-over-the-pwned-host (host-controlled secrets, etc.):
  **explicitly out of scope** this sitting.
- The uncomfortable structural observation I could not resolve, recorded because it is the
  reason no clean answer exists: **our attribution promise is itself the attack surface.**
  Every sibling can say "we only report what the host told us"; we cannot, because we promised
  that every failure names a pointable line and a responsible author. An adversary who
  influences the diagnosis does not merely mislead — they *aim our finger*, at an innocent
  oracle, author, or line. The flagship differentiator and the top-ranked sin are the same
  mechanism seen from two sides.

## §9 — Pointers, not repetition

- Research base and graded manifest:
  `.claude/research/quarantine-DO-NOT-READ-patronizing-aid/` (24 sources; `sources.json` +
  `turn01-2026-08-15-notes.md`; committed `d8be13a2`, archive gitignored).
- Round-29 lineage: `299` (with the marker added this sitting), `29A` §3 and §8, `29-reviewA`.
- Out-of-quarantine sibling: **`notes/306b`** — carries the sanitized engineering statement
  and the tripwire invariants in pure-engineering language. The specific invariants and
  steering-law edits are deliberately NOT restated in this document.
- Corrections landed this sitting: `ai/r30-conduct` `6190e600` (rec-5 boundary narrowed to
  the license plane); `ai/main` `277868f2` + `059a6f51` (residue marker at `299`).

## §10 — How to read this document

Weight it as follows, and I mean this literally rather than as ritual modesty. The [HUMAN]
rulings are the artifact. The [SOURCE] findings are as good as the subagent's reading, which
I spot-checked in two places and did not verify in full. **My own analysis is the weakest
layer**: within a single sitting I proposed and then withdrew a secret-omission rule, a
cross-run schema prohibition, a full-host demotion posture, a global partial-consumption ban,
and a delete-the-codes recommendation — five reversals, every one of them caught by the human
rather than by me, and in each case because I had reasoned fluently from an untested premise.

A future reader should therefore treat this document as a **map of the problem space and a
record of which arguments were tried**, not as a source of rules. The rules, such as they
are, are the human's, and are few.
