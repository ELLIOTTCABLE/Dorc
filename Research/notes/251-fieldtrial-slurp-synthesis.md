# 251 — round-25 slurp synthesis: what the reading changes about the field trial

AI-authored (Fable conductor), 2026-07-04. The reading-yield from the round-25 onboarding slurp
(the `plans/250` reading list), filtered through the field-trial-methodology lens. Confidence-marked;
every finding carries doc-anchors. Durable: a fresh conductor reads `250` (the charter) then this
(what the corpus already knows that constrains the protocol). NOT the protocol itself — the inputs to it.

## The headline: the spike has no real executor — "provision a machine" isn't a thing it can do today

- `find-no-executor` (+SURE). The spike compiles a probe and an eliding apply and **executes
  NEITHER** — there is no host executor, no apply-over-time, no networking (`16P` T13, §3.2:
  "the apply executor (Option C) … NOT BUILT"). Everything green to date is in-memory/mocked
  (`24B` flavours A/C; the `21D`/`24B-C` differentials evolve *mocked* host copies). `088`'s `do-4`
  *planned* a "thin/mocked executor — local containers or ssh-to-one-box" but it was never built
  against a real box.
  - **Consequence — a scoping fork the whole trial pivots on:**
    - *full-apply trial* — build a throwaway serial-one-box executor (ship probe over ssh → run →
      feed results back → ship apply → run), enabling the **real-machine differential** (below),
      which is the only instrument for the primary target. Costs an Opus build.
    - *plan-only trial* — generate the plan against the real machine's real probed state; judge the
      plan (does it look right, feel valuable) without applying. Cheaper; **cannot test adequacy**
      (under-execute bites at apply).
  - **M3 warning (`151` X2/M3; +SURE it's a live trap):** a thin executor "commits a kCOMMS
    [transport] shape by accident, and `088` doesn't flag that it's doing so." There is "no safe
    intermediate executor between serial-one-box and the rich scheduler." So the trial executor must
    be **explicitly throwaway + non-committing**, dead-simple serial-one-box, flagged NOT-DESIGN — or
    it silently bakes the single most retrofit-hostile undecided thing in the corpus (`kCOMMS`).

## The trial's unique niche (why it isn't redundant with the in-memory sweep)

- `find-unique-niche` (+SURE). DST/the sweep **structurally cannot** test "whether the opaque program
  actually worked, or real-host behaviour — the mocked edge" (`128` se-2). The in-memory sweep
  (`24B` flavour C) already runs the differential-spine — evolve two host copies (bare vs plan),
  assert **end-state equality** + attribution-under-lies — but against a *mocked* host. The field
  trial is that same differential with a **real** host: it validates whether the oracle claims are
  *true of reality*. That is the one thing no amount of in-memory testing reaches, and it is exactly
  the primary target (`target-adequacy`).

## do-4 IS this trial, pre-registered in round 8

- `find-do4-is-the-trial` (+SURE). `088`'s `do-4` = a **dogfood existence-proof** (NOT a market
  estimate — the corpus go/no-go is representativeness-doomed, `087` §5), three threats with
  pre-registered KILLS: **A-VALUE** (does it skip a meaningful *expensive* fraction on converged
  re-run + one-line-change? *watch the felt value*), **A-WIN** (wallclock vs `ansible --check`+tags
  and vs `pdsh`), **A-ORACLE** (how hard to write each oracle correctly + hermetically?). `24A`'s
  `rul24-acceptance-shape` already named "point an Opus at real stuff … absent that there is
  effectively no written acceptance-goal." The protocol should inherit `do-4`'s structure wholesale.

## The felt-product adversary is empirically real (not a quirk)

- `find-perception-gap` (+SURE, load-bearing). METR RCT (`128` §6): 16 experienced devs on their own
  repos were **19% slower** with AI while **feeling 20% faster** — a ~39pt perception gap. So
  `target-felt-product` is precisely the signal most subject to systematic self-deception; the "woo
  cool" is a documented cognitive failure, not a personality trait. **Instrument behaviour
  (time-on-task, the differential, elision-count), never trust the feeling** as the measurement.

- `find-value-locus` (+SURE, the sharp falsifiable form). `088` `do-1`/`do-2` + `087` §4: is the value
  the **sound analyzer** or the **cheap UX** (git-diff greying)? "Most UX needs no oracles —
  greying-out by git-diff delivers most of the *felt* win for ~5% of the engineering." So the felt
  question becomes a **comparison, not a vibe**: does *sound* elision feel more valuable than a dumb
  git-diff greyer would? If the admin can't tell the difference, the entire analyzer cathedral (and
  the `A-ORACLE` existential risk) is unjustified. This is the most falsifiable version of
  felt-product on offer.

## The careful-book paradox (a confound that will masquerade as "Dorc is bad")

- `find-careful-book-paradox` (+SURE, sharp). "The scrappy book with bare `systemctl enable --now
  nginx` is richly inferable; the careful, correctness-heavy book — functions, `readonly`
  constant-folding, temp-staged atomic `cp`+`mv`, heredoc desired-state — drives straight to ⊤. Every
  cheap inference handle is exactly what good structure removes" (`151` X4 THE-ONE). The spike already
  can't elide on its one realistic fixture (a bare un-oracled `apt-get update` poisons everything
  downstream — `16Q` §1). **The human is a careful functional programmer**; his instinct to write
  clean code will choke Dorc, and confound *"the tool is weak"* with *"I wrote code the analyzer
  can't see through."* Book-idiom quality is a first-class confound axis (`24A`: "book-IDIOM quality
  is an axis alongside oracle-coverage quality"). The protocol must hold it as a deliberate variable
  (scrappy vs careful), not let it float.

## target-adequacy: the deepest finding + its structural limit

- `find-adequacy-limit` (+SURE). "Converged" ≠ "no-op" is THE calibration target (`240`) and is
  **dead to prove** — for *every* command an un-horizoned authored claim quantifies over
  third-party/per-version/per-host code the author cannot attend to (`238` claim-4; `cp` trips
  inotify/FUSE/quotas too). Measurable only by **bite-rate**. And the sharpest part: the admin
  **structurally cannot judge a vouch's adequacy** — "the license-site diverges from the elision-site;
  the completeness-claim escapes the context that could falsify it; the admin can't evaluate it (not
  knowing the tool is *why* they oracle it), the author isn't there" (`24A` rul24-divergence-is-the-game).
  So you cannot *reason* adequacy out — you must **run it and diff the real end-state**. This is what
  makes the real-machine differential the *only* adequacy instrument, and why the trial (not more
  thinking) is the move.
  - Concrete adequacy bugs to hunt (`151` X4, the most trustworthy findings — actually executed):
    hand-rolled oracle arg-parsing holes — a ufw sanitiser's `.`-as-regex matches `10X0X0X1` →
    wrongly-converged → **wrongly skips adding a firewall rule** (silent under-execute); an apt-get
    `-o` flag leak makes the *probe itself mutate*. An LLM-authored stdlib will ship these. Hunt them.
  - `resid-aliasing` (`24C`): the primary silent under-execute cell — `disjoint` is token-equality on
    entities, so `nginx`/`nginx-full` (via provides) or symlinked paths come up wrongly-disjoint →
    wrong-survival. Deferred to Stage 5, so it is a **known open hole** a real machine (real symlinks,
    real `provides`) could trigger under `--trust-footprints`.

## The differential-spine is well-established prior art — inherit its discipline

- `find-differential-discipline` (+SURE). `21D` (run-set differential, drives real `dorc`+`dash`,
  500 seeds, 0 engine bugs) → `24B` flavour C (end-state differential, in-memory). The real-machine
  version is the same idea with a real host. Two disciplines it MUST inherit:
  - **Anti-drift (`21D` §2):** the two runs (bare-full-apply vs Dorc-elided-apply) must differ *only*
    by elision, never by environment drift → they must start from **identical fresh state**
    (VM-snapshot / reprovision). This is the reproducible-state (`kVOLATILES`) requirement of the
    differential.
  - **Behavioural-not-textual (`16Q` `ap-2`):** the corpus's sharpest self-inflicted wound — the e2e
    string-compared stdout *text* and never executed the artifact, so non-runnable POSIX shipped
    green. **The acceptance signal must execute the artifact / diff the machine end-state, never
    compare plan text.** This is the whole justification for the differential over "read the plan."

## Method lessons the protocol (and its adversarial review) must obey

- `find-method-lessons`:
  - **ap-3 (`16Q`):** "an adversarial pass finds only what it is aimed at; rotate the *target*, not
    the cadence." For the part-2 adversarial review: rotate across the core question, the instrument,
    the confounds, the felt-signal — don't re-attack one face.
  - **silent-caps-read-as-coverage (`21D` find-6 = `24C` find-lcg-thinning):** a low-bit PRNG
    correlation silently zeroed whole elision classes while looking green. The trial's analog: "it
    elided nothing" is honest if the book hit a poison wall early, but could equally be a broken
    executor/oracle — so **confound-isolation is mandatory** (below).
  - **existence-proof-only (`087`/`088`):** N=1 dogfood is "value for *my* profile," never a market
    claim; pre-commit to only the decidable question.

- `find-confound-isolation` (+SURE, the crux of "questions that yield decisions"). "It didn't elide /
  it felt bad" has ≥5 disjoint causes: (a) CLI/executor bug, (b) LLM-oracle-slop adequacy bug,
  (c) engine limit (poison-wall / value-flow ⊤), (d) book-too-careful (`find-careful-book-paradox`),
  (e) admin unfamiliarity. The protocol needs **layered conditions** to separate them — e.g. same
  book with a hand-perfected oracle vs the LLM oracle (isolates b from c); a scrappy book vs a careful
  book (isolates d); cross-run the in-memory sweep on the same book (isolates c from a). Without
  isolation the day yields a feeling, not a finding.

## The emerging protocol shape (seed for the co-design step — NOT settled)

Pre-registered dogfood existence-proof, `do-4` structure. Primary targets `target-adequacy` (via the
real-machine differential) + `target-admin-loop` (the lazy admin writing 1–2 oracles for a
game-server). Each question carries: observation→decision map · signal (mechanical where reducible;
honesty-scaffold where qualitative — `signal-reducibility`) · confound-isolation · a stopping rule
(protect the day-window). `target-felt-product` sharpened into the `find-value-locus` comparison
(sound vs cheap). Behaviour instrumented, feeling distrusted (`find-perception-gap`). Book-idiom held
as a deliberate variable (`find-careful-book-paradox`).

## The three decisions owed to the human (before the protocol can be drafted)

1. **The executor scoping fork** (`find-no-executor`): build the throwaway serial-one-box executor
   (enables the real-machine differential = the only adequacy instrument) vs plan-only. Conductor
   rec: build it — adequacy is primary and un-reachable without apply — but explicitly-throwaway +
   non-`kCOMMS`-committing (M3).
2. **Reproducible fresh state** (`find-differential-discipline`): VM-snapshots on the box (cheap
   reset, game-server fidelity is fine on a VM) vs bare-metal reprovision (slow; forfeits the clean
   differential). Conductor rec: VM-snapshot.
3. **Book-idiom** (`find-careful-book-paradox`): write the game-server book scrappy, careful, or
   deliberately both. Conductor rec: consciously both (or scrappy-first), so his FP instinct doesn't
   confound the verdict.

## Human corrections & refinements (2026-07-04, live — these SUPERSEDE the findings above where they touch)

- **corr-executor-is-welded (corrects the `find-no-executor` framing).** No-executor is not a gap —
  it was a **pre-supposed, welded-FOR-THIS-SPIKE boundary**: "the human ensures dorc can ssh in; all
  dorc does is ssh-apply shell-scripts" (= the `DESIGN` off-ramp `ssh host 'dash -s' <script`; the
  `128` whole-script-per-host transport; `088` "ssh-to-one-box, don't build the scheduler"). NOT
  settled-to-ship-as-product, but true for the spike. So the trial's runner *is* that welded floor
  (ship script → run under dash → capture end-state — trivial), and **because it's the welded-trivial
  floor, the M3/`kCOMMS` trap is avoided by construction** (M3 was about a *fancy* executor). Human
  may slot the throwaway executor build + a one-shot half-ass LLM oracle-stdlib at the **tail of round
  24** (an Opus conductor with code-context), as prep for this round.
- **corr-container-first (reshapes the substrate).** Hard pivot: **containers / a VPS FIRST; the
  bare-metal live-machine is step TWO.** Nothing in the methodology depends on bare metal except
  exotic hardware-trace (Intel-PT-class; not needed — eBPF works on a VPS's real kernel). Sweet spot:
  **a VPS (e.g. Vultr) with snapshots + cloud-init** — real kernel/systemd (game-server fidelity),
  cheap snapshot reset (the differential's reproducible-state), and cloud-init bootstraps the box into
  ssh-apply-ready state, **mechanically sidestepping the kCOMMS/bootstrap question**. Containers for
  the cheaper pure-soundness runs where systemd isn't needed.
- **corr-protect-first-contact (LOAD-BEARING; the biggest structural refinement).** The human's *first
  personal use of the tool* is a **non-renewable, ~order-of-magnitude-more-valuable instrument** — the
  whole clean-room "leave the LLMs to it" discipline exists to protect it. Every later use, he is
  contaminated (knows the tool; no longer a naive first-contact admin). **Consequence: the protocol
  SPLITS into two tracks.** Track A (freely-iterable, mechanical): containers/VPS, LLM-driven,
  disposable — the differential, adequacy-bite-rate, executor/stdlib rehearsal — run endlessly, no
  human, dress-rehearsed to invisibility. Track B (one-shot, human): felt-product, admin-loop,
  gap-log — fires **ONCE**, after Track A is turnkey, so his single run is pure signal spent on
  provisioning the game-server, not on mechanical friction. The `find-perception-gap` "woo cool"
  defense is *doubly* critical here: there is no clean re-do of first-contact.
- **corr-confound-is-provenance (reframes `find-confound-isolation`).** Confound-isolation is NOT
  test-scaffolding to bolt on — it **IS the product's own provenance/attribution angle** (`dorc why`;
  the why-lens — `111`, `24C`'s run-cause / survives-attribution lanes). It is true of ops in general
  and is a *product-goal*, to be frontloaded 24/7. So the trial **dogfoods and stress-tests** it: when
  "it didn't elide," the first move is `dorc why <line>`, and **whether that answer is adequate is
  itself a primary finding** (does the product explain itself well enough to separate
  executor-bug / oracle-slop / poison-wall / ⊤ / unfamiliarity?).
- **corr-git-diff-calibrated (down-weights `find-value-locus`).** The "git-diff greyer" is a
  felt-product **baseline** — a hypothetical dumb, unsound competitor that greys lines unchanged in
  git since last apply (no probe, no oracle), capturing much of the *felt* "here's what I touched"
  win for ~5% of the work. It is the sharpest way to make felt-product **falsifiable** (does *sound*
  Dorc feel better than the dumb fake?), NOT the round's most-important finding — over-billed in the
  first pass.
- **corr-careful-book-softened (updates `find-careful-book-paradox`).** `151` is round-15 and partly
  superseded: the value-flow work (round 19+) and especially the **guard tier** (round 23/24) mean a
  careful book that goes ⊤-for-*elision* still gets **guards** (check-then-run) — it loses the
  attention product, keeps perf/safety. Human's read: the engine is "in a pretty good place," most
  holes known-to-him, capability better aligned to desired user-behaviour. Held under skepticism
  (never-vouch: "in a good place" is exactly what green tests + design-language inflate) — **the trial
  is the external test of that claim, and surfaced holes are prime round-products.**
- **corr-book-scrappy.** Human will write **scrappy-first** (sees the attention/elision product;
  deliberately counters his own over-engineering lean).

## Settled + near-settled this exchange (2026-07-04, cont.)

- **weld-vultr (SETTLED).** Substrate welded to **Vultr VPS** over containers: higher fidelity (real
  kernel/systemd), low-cost for non-overnight exploration, wipeable + snapshottable, human has an
  account + prior experience + cribbable structure in his System/Infrastructure repo. This opens
  **multi-target** (3–5 VPSes ≈ the cost/effort of 1) — the multi-host fork below.
- **punt-git-diff-greyer + the market fence (SETTLED).** The git-diff-greyer felt-product baseline is
  **punted — explicitly out-of-target-market.** Human's framing (durable, DESIGN-adjacent): *if
  you're comfortable with unsound git-diff greying, you're not afraid/defensive, so you're not a Dorc
  user — there is no value-prop for the unafraid.* Dorc rides **between** "people who'd just use
  Terraform" and "people who'd never bother with an orchestrator," subsuming neither — it's for those
  who hunger for less chaos / more sanity but have lives + dayjobs (or are the lone ops-person at a
  200-head company who must get business done). **Consequence:** the felt-product baseline is NOT the
  git-diff greyer but **blind-run** (`ssh host 'dash -s' <script` — the real off-ramp, the DESIGN
  "no worse than running it blind" floor): *does Dorc feel meaningfully better than running the script
  blind, for someone afraid of breaking things?* The cathedral-justification / value-locus frame is
  dropped (a market question the human has YOLO-GO'd past).
- **aperture-widening (SETTLED requirement; counters a human-flagged bias).** Elision-vs-guard is very
  in-scope and front-of-mind (r23/r24) — but the human flags his own risk: 233-fear may crowd out
  OTHER value-prop discoveries. The protocol MUST carry a deliberate **wide-aperture channel** — a
  first-class "what else did we learn that ISN'T elision/guard" capture (DX friction, provenance /
  `dorc why` quality, the authoring loop, error messages, unexpected value *or* anti-value) — to
  counter the tunnel-vision.
- **phase-A / Human-Handholding-Framework (PROPOSED by human, my read affirms; not yet his ack).**
  Track A = a mechanical **phase-A, the first half of r25**: LLM/agent-driven, containers/VPS,
  disposable — builds the real-SSH-apply runner, the snapshot differential, the first-pass oracle
  stdlib, AND the **Human-Handholding Framework** (the ocean of logging/watching = the gap-capture
  wrapper). Human stays hands-off until handed a turnkey package. Track B = his **first-blooding**,
  wrapped in that framework, fired once. Conductor sharpening: phase-A's **exit-gate is an LLM playing
  admin** running Track B's whole script in a container first — proving the framework captures what we
  need — BEFORE the human is invited (his first-contact must not be the framework's first exercise).
  This resolves gap-capture (it IS the framework) and operationalizes protect-first-contact.
- **multi-host fork (OPEN; human: light consideration, not critical).** A punt / B minimal-serial
  (loop over hosts, quality-of-multi-host-code explicitly out-of-quality-scope, just to USE
  heterogeneous fleet to exercise per-host-plan value) / C reorder r25 after the concurrency work.
  Conductor lean: **B** — a serial `for host; do apply; done` is nearly free and unlocks the
  never-touched heterogeneous-fleet value-prop (host converged / drifted / bare → honest per-host
  plans); put multi-host in Track A (mechanical), keep Track B first-blooding single-host, optionally
  extend to a small fleet as a second act. C over-reorders (the real concurrency work / 22H is big +
  separate); A leaves free value on the table.

## cont. 2 (2026-07-04)

- **razor-phaseA-serves-phaseB (SETTLED principle).** Phase-A's ONLY justification is making Phase-B
  (the human first-blooding) useful. Anything the human does NOT exercise in Phase-B is not worth
  building — defer to later rounds. (Corrects the conductor's "multi-host in Track-A only" split: no
  mechanical-only gold-plating.)
- **multi-host = B + human-exercised (SETTLED, gently welded).** Build whatever the MVP of multi-host
  is — starting an external-to-dorc shell-loop (`for host in $fleet; do dorc-apply $host; done`) — AND
  the human exercises heterogeneous-fleet in first-blooding (per the razor). Human suspects the loop
  won't suffice. ~SUSPECT (conductor) the insufficiency is the **fleet-plan-view**: an external loop
  yields N separate plans, not the unified "here's what each host needs" that IS the fleet value-prop;
  dorc may need to accept N hosts and render one combined plan. Discover the floor by trying.
- **value-locus still in scope, re-banded (OPEN — band pending human).** The git-diff band is punted
  (out-of-market); the equivalently-shaped X-expensive-vs-Y-cheaper question survives at a different,
  in-market band. Conductor candidates: (i) **cheap guard-tier vs expensive full-elision** (stages
  4–5 footprints/grounding) — which carries the felt value, i.e. whether the top of the ladder earns
  its build-cost; (ii) **plan-preview-only vs full-sound-analysis** — which delivers the felt safety.
  Human to pick the band (or a third).

## cont. 3 (2026-07-04) — value-locus resolved, durability tiers, tool-list, why-plus

- **value-locus = band-(i); (ii) punted.** (ii) preview-vs-analysis is answered by prior-art
  scuttlebutt (plan/apply is universally raved-about; no n=1 test earns its keep). **(i)
  guard-tier-vs-full-elision** is THE toothy open question — full elision is the *first intentional
  acceptance of unsoundness into the codebase* (the oracle-vouch r24 is building; the 233 surrender).
  Human self-flags heavy 233-trauma bias (over-defensive toward "guards are enough, yeet the
  unsoundness"), asks to be protected from himself, and can't see how to test it (n=1, market-shaped).
  **RESOLUTION — do NOT test (i) as market-fit; decompose into three inputs, two mechanical:**
  1. **frequency** (mechanical): on a real scrappy book, how often does full-elision even *fire* vs
     guards? Near-zero ⇒ golden hill unreachable ⇒ guards win by default, question dissolves. (= the
     Stage-6 yardstick on a real book.)
  2. **bite-rate** (mechanical, = A1 adequacy): how often does the accepted unsoundness under-execute?
  3. **felt-when-it-fires** (n=1, weak, held loosely): the line that vanished past a wall — did it
     feel meaningfully better than the same line guarded?
  Decision: full-elision earns its keep iff fires-often × bites-rarely enough to beat the felt-magic
  vs unsoundness cost. **The two mechanical thirds ARE the protection-from-himself** — his trauma
  can't override an empirical "fires a lot, rarely bites" (nor a "barely fires, bites often" that
  would vindicate yeeting it). Present decomposed; do not further LLM-inflate (i)'s importance.
- **durability division of the phase-A builds (Q1) — THREE tiers:**
  - *product-content (durable, into spike3 as the pretend-product):* the LLM oracle-stdlib (= the
    `effort-allocation` bootstrap ~40 oracles; the Stage-2c battle-oracle seed). Build it *well*.
  - *durable-ish testing-framework (a mirror plausibly reaches the product-repo — the `kVERIFY`
    calibration-harness top tier DESIGN promised):* the snapshot end-state differential (real-machine
    analog of the 24B flavour-C sweep). The ssh-apply runner is a notch below — a spike-executor
    STUB (rough, real-transport-exercising, non-kCOMMS-committing), spike-durable but NOT the eventual
    executor.
  - *one-off r25 scaffolding (disposable):* the HHHF, cloud-init provisioning, the multi-host
    shell-loop.
- **tool-list starter (Q2), three buckets:**
  - *session capture:* asciinema (fidelity/replay) + a small ANSI-stripping extractor → a
    **timestamped transcript** (LLM-readable; raw `.cast` is not) + lightweight think-aloud/checkpoint
    prompts the HHHF fires at seams (a transcript shows *what*, not *why it annoyed*) + `HISTTIMEFORMAT`
    history spine.
  - *VPS eyeballs (mechanical ground-truth):* the differential's end-state capture (`dpkg -l` /
    `systemctl list-units` / config `sha256`s / open ports / **a real "does the game-server respond?"
    health check**) + `dorc` plan-summary + `why` logs. Optional-iff-turnkey: inotify/auditd "what did
    apply touch" footprint ground-truth (gated hard on "an agent sets it up invisibly or skip it").
  - *built into the dorc binary (nothing kCOMMS):* the machine-readable disposition/why output the
    differential consumes (21D used `--debug-argv`); the plan-summary yardstick (exists); and IF the
    shell-loop proves insufficient, the fleet-plan-aggregation surface.
- **dorc-why-plus (Q3, human intuition):** `dorc why` explains the ANALYZER's own reasoning; but
  confound-isolation must also attribute failures OUTSIDE it (oracle lied / executor broke / book too
  careful / human unfamiliarity). The `+something-else` is that *above-the-analyzer* attribution
  layer `why` structurally can't give from its own derivation graph. "How good was `dorc why`, and
  where did it need enrichment" is a small first-class round-deliverable (a provenance/DX finding).

## cont. 4 (2026-07-04) — HHHF durability (invited stay-on-target ruling)

Human tempted to upgrade the HHHF from disposable to DURABLE, to "wrap a friend in it later" — a
moderated-usability / think-aloud-over-shoulder test on a true-naive ops friend when the spike is
mature. Multiplies the non-renewable first-contact instrument (n=1 → n=few); a fresh friend is
arguably a *better* market-signal than the human, who is design-polluted. Human explicitly invited
the "scope-creep, stay on target" reply.

**RULING (conductor, invited):** mild scope-creep to build durable NOW; the INSIGHT is a keeper; the
resolution preserves it cheaply and *de-risks* the friend-test. **Build HHHF disposable for r25**;
the human's own first-blood is the **rehearsal + spec-discovery** for what a durable version must
capture; **harden-for-friend is a de-risked post-r25 fast-follow**, its own scoped call. Reasons:
(a) we don't yet know what HHHF must capture — r25's run teaches it; building durable-blind = the 16Q
`ap-1` "scaffold the deferred hard part" anti-pattern (⇒ disposable-first has *less* rework); (b)
protect-first-contact applies to the FRIEND too (a second non-renewable instrument — the human's run
must shake the framework down first); (c) spike-is-disposable ethos + the human's simplicity
priority; (d) timing — the friend-test is "when the spike is mature," a long runway during which both
the spike and the instrument-needs move. **Nuance:** dead-simple disposable, ZERO forward-compat tax
(a rewrite is cheap on a disposable spike), just don't gratuitously hardcode the human as the only
subject. **BANKED as a post-r25 fast-follow** — deliberately NOT on the r25 task list (tasks #4–6
stay pointed at shipping the human's own first-blood).
