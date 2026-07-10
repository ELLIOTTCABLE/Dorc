# 250 — Round 25 charter: the first real-machine field trial (methodology design)

> Tier: AI-authored (Fable conductor), 2026-07-04, capturing HUMAN rulings from the session that
> opened round 25. Discrete human rulings are slugged `human-<name>`; everything else is
> conductor-synthesis, confidence-marked. **Durable restart-point:** a fresh conductor can resume the
> round from this file alone. Does NOT edit root docs (human-only). `LIVING_STATUS` carries the pointer.

> **⚠ ROUND TABLED (2026-07-10, r24 close-out — `notes/24U` §6):** the trial never fired; it
> waits on the round-27 consolidation (`plans/270`) — the stdlib gates it hard, and the book's
> two permanent walls (`su - postgres -c`; the `$(hostname)` host-guard) are exactly what
> `270:block-context` builds, so the revived trial measures MORE product than this charter
> assumed. Revival conditions + owed-on-revival items: `270` §5. The methodology herein stands
> unchanged.

## What round 25 is

Round 24 built the elide machine on synthetic strawmen. Round 25 is the **first time a human drives
the built spike against a real machine** — and the deliverable is NOT the trial but the
**methodology for it**: a pre-registered, adversarially-reviewed *playbook* that makes the human's
day-of-effort yield *design decisions* instead of a good feeling.

`human-playbook-not-script`: the methodology is a **human-run playbook / documentation /
thought-process, not a script** — the human runs it. `human-durability`: it is durable (this round,
"round 25"), written down so a subsequent conductor can start over.

The human has **never invoked the built CLI**; all testing to date is "LLM-slop testing LLM-slop,
intentionally." This is first real contact.

### The adversary (why this needs design)
`human-woo-cool-adversary`: "Once I have expended such time, I will likely go 'woo, this is cool' and
be happy." A trial not pre-committed to falsifiable questions will *feel* like success and decide
nothing. The entire value of the planning phase is (1) deciding **what the test is** / the mechanical
acceptance-goals, and (2) subjecting that to **adversarial review** — so a painful day yields maximal
design-direction takeaway.

## The grounding scenario (`human-game-server-scenario`; end-goal mutable)

- **Given:** the Dorc CLI (spike) + an **LLM-authored stand-in stdlib** of oracles, built the way LLMs
  built the whole spike (`human-llm-writes-stdlib`: the stdlib is authored by LLMs, not the human).
- **The human plays THE ADMIN** whose real goal is a working **video-game-dedicated-server** on a
  freshly-provisioned clean machine.
- **The human hand-writes 1–2 minimal oracles**, grudgingly, kept "stupid and minimal" — because in
  the real world he wants the game-server, "not fancy Dorc crap." This hands-on 1–2-oracle authoring
  "is very nearly the entire point."
- This lives out USER_STORY stages 1–3 from the **lazy admin's motivational stance** — the first real
  test of the gradual-enhancement value-loop and the DESIGN "tool you use when you want to be lazy" bet.

## The targets (`human-target-priorities`)

- **`target-adequacy`** *(primary)* — do the oracle convergence-vouches (LLM-stdlib + his-minimal)
  *tell the truth* on the real machine, across re-run and drift? The one correctness question DST
  structurally cannot reach: DST tests the engine *given* oracle claims; only a machine tests whether
  the claims are *true of reality*.
- **`target-admin-loop`** *(co-primary)* — does the lazy admin, wanting a game-server, get value from
  Dorc+stdlib while writing only 1–2 minimal grudging oracles? Is that loop tractable and rewarding?
- **`target-felt-product`** *(high but hard; slightly secondary — "don't let it slip out of view")* —
  does it *feel good* to use? "Kinda the whole game," irreducibly qualitative, "definitely cannot be
  reduced to a number."
- **`target-works-at-all`** *(precondition / "a freebie, or rather the first test")* — does the CLI
  provision a bare box idempotently at all? If it *fails*, that failure is the day's finding.
- **`target-gap-log`** *(priority ONE, but hardest to capture)* — what does reality reveal that 24
  synthetic rounds missed? "My engineer-experience will drive me *hard* towards reflexively solving a
  problem and not notating that I had to" — fix-and-forget erases gaps. **Needs a forcing-function that
  captures gaps at the moment of friction, before the fix-reflex.** Owed, unsettled (see below).

## Structural constraints (as refined by the human this session)

- **`signal-reducibility`** — prefer **mechanical / numerical** acceptance-signals wherever a criterion
  reduces to one (CLI output; the differential; agent-built dashboards over synthetic data — more
  trustworthy than post-hoc impressions, less woo-yay-gameable). But **explicitly allow
  irreducibly-qualitative criteria** — `target-felt-product` cannot be a number; honesty-scaffold it,
  don't fake one. "Tradeoffs all the way down"; some criteria genuinely can't be a number.
- **`differential-spine`** *(one option, NOT mandated)* — Dorc-elided-apply vs bare-full-apply, diffed
  = a rigorous, objective under-execute verdict. "Mechanically annoying." Needs reproducible fresh
  state. The human *will* fully provision a clean machine; heavy instrumentation (eBPF, containers,
  VM-snapshots, root tools) is **on the table iff agent-automatable + opaque/turnkey to him**. Dominant
  constraint: the **limited day-window** — "it'd be easy for me to get mired... whereas I could have
  spent those six hours running a bunch of plain-SSH interesting experiments that teach us more."
  Plain-SSH breadth beats fancy-tooling that eats the day; powerful tools "won't magically solve any of
  the important questions."
- **`n1-honesty`** — N=1, self/LLM-authored, non-representative. CAN falsify, gap-discover,
  adequacy-calibrate. CANNOT value-rate, prove soundness-*absence*, generalize. Non-numerical criteria
  are less number-gameable but more woo-yay-subject.
- **`llm-authoring-twist`** — LLMs author the stand-in **stdlib** only; the human hand-authoring 1–2
  minimal oracles is in-scope and nearly the whole point. Adequacy risk = naked-vouch-at-scale (LLM
  stdlib) + the grudging-minimal hand oracle.

## The gap-capture mechanism (owed, unsettled — `target-gap-log`)

The hard problem: the human's engineer-reflex fixes-and-forgets, erasing the highest-value output
(what reality reveals that synthesis missed). A live agent dictating/observing every command is
impractical — **models are too slow for an interactive CLI loop**. Favor **async capture over
live-in-the-loop**. Candidate substrates (none settled; figure out later in 25):
- a lightweight loop that polls the shell history / session log every N seconds and diffs it;
- an asciinema-style terminal-session recording, processed after the fact;
- mechanizing the product itself to emit verbose/intense cross-session logging.
A processing agent turns any of these into a structured gap-taxonomy. ~SUSPECT async-capture +
after-the-fact processing is the right shape; the capture substrate is the open call.

## Deliverable shape

A **pre-registered protocol skeleton**: the question set, each item carrying (a) an
observation→decision mapping (what result moves which decision), (b) its signal — mechanical where
reducible, honesty-scaffold where qualitative, (c) confound-isolation (so "it didn't elide" tells us
*which* of CLI-bug / oracle-slop / engine-error / admin-unfamiliarity failed), (d) a stopping rule
(protect the day-window). Then an **adversarial review** of that protocol.

## Reading list (reverse-priority: #1 first → #N last/freshest; ⏳ older-foundational ballast; ⭐ citation-count add)

Synthetic/strawman measurement only, never the corpus (round-24 method, inherited).

**Head (skim / on-demand):** `000` [skim] · `076` · `111` · `17N`.
**Middle (current-state law):** `spike/CLAUDE.md` · `16P` §3 + ⭐`16Q` · `23D` · `23O` · `239` ·
⭐`233` (human crisis log, #1-cited) · `24A` + `24C` + `24D` · ⭐`19H` (value-plane).
**Tail (evaluation core + ballast):** ⏳`055` · ⏳`099` · ⏳`077` · `087` + `088` · `086` · `151`
(+`150`) · `238` · `23M` + `23N` · ⭐`124` + ⭐`125` (DST-seam / containerizability — the
reproducible-state fork) · ⏳`128` (DST) · ⭐`21D` (differential-harness — the `differential-spine`
prior-art) · `240` · `24B` ← freshest.

Citation in-degree (load-bearing proxy, top hits): `233`(31) · `111`(30) · `19H`(27) · `16P`(23) ·
`127`(22) · `17N`(20) · `055`(19) · `077`(19).

## Process / sequence
1. **[done — this file]** durability charter.
2. **[Fable]** absorb the reading list through the field-trial lens.
3. **[Fable + human]** draft the protocol skeleton; sharpen together.
4. **adversarial review** of the protocol.
5. → hand the settled protocol to a **narrow-brief execution conductor (Opus)** for the real-machine
   run. Fable conducts the methodology; Opus executes.

## Round-open refinements (2026-07-04 — human live; full detail in `notes/251` §human-corrections)

- **protect-first-contact (load-bearing constraint).** The human's *first personal use* is a
  non-renewable instrument worth ~10× any later run. The protocol splits **two tracks**: A =
  freely-iterable mechanical (containers/VPS, LLM-driven — differential, adequacy-bite, rehearsal),
  B = one-shot human first-contact (felt-product, admin-loop, gap-log), fired ONCE after A is turnkey.
- **container/VPS-first; bare-metal is step two.** VPS + snapshots + cloud-init is the sweet spot
  (real kernel/systemd, cheap reset, bootstrap sidesteps kCOMMS). Nothing needs bare metal.
- **no-executor is welded-for-spike** ("human ensures ssh; dorc ssh-applies scripts" = the off-ramp);
  the throwaway executor + a half-ass LLM oracle-stdlib may land at the **tail of round 24**.
- **confound-isolation ≡ the product's provenance (`dorc why`)** — dogfood + stress-test it, don't
  bolt on test-scaffolding.

## Conduct fences (bind any successor)
Fable conducts the methodology; **real-machine execution and any code-writing hand off to a
narrow-brief Opus** (the standing Fable-conducts / Opus-executes split). **The human runs the actual
trial.** Synthetic/strawman measurement only — never the `corpora/` or quarantine dirs. Agent-built
dashboards over synthetic data are the trustworthy channel for numbers. No AskUserQuestion (ask in
prose). Silence ≠ ack (only typed counts). Notes append-only (this is a plan, not the living doc).
Full-word slugs. Explain prior-art inline (human often on mobile).
