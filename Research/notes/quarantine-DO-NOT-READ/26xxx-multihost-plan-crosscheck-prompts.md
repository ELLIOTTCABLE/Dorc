> QUARANTINED — stance-engineered adversarial-crosscheck prompt-pair for the round-26 plan
> package (`plans/260`/`261`/`262`). Contains disowned/opposition-owned framings and a
> pre-registration gate; reading it contaminates any future reviewer's gate. Do not read
> during crosscheck skill-ups. Authored 2026-07-07 (Fable, design-synthesis session — the
> plans' own author, hence the full disowning below); NOT yet dispatched, pending human
> review of these prompts.

# 26xxx — multi-host / concurrency plan-package crosscheck (prompt-pair)

## Dispatch notes (conductor-facing; not part of any packet)

- Target artifact = `Research/plans/260-round26-multihost-plan.md` +
  `261-round26-read-concurrency-plan.md` + `262-round26-build-spine.md`, as of
  `FILLME-pin-at-dispatch` on branch `ai/spike3-r26`. Reviewed as ONE design-package.
- Lanes per the 24K precedent: stance × lineage. Foreign lanes (codex / deepseek /
  antigravity) get these packets with the phase-1 + phase-2 shared sections inlined below.
  **Antigravity is packet-only (cannot read a repo): its variant needs the three plan
  documents inlined verbatim into the packet and the phase-2 reading-list replaced with
  "the artifact and its key substrate excerpts are inlined below" — assemble at dispatch.**
- Carry the 24K friction fixes: explicit `model:` pin per dispatch; packets as files the
  shim only `cat`s; absolute path for `ds-review`; the turn-truncation guard is EMBEDDED in
  the shared section below (deepseek-neutral previously ended its run at the gate).
- Contamination-disclosure line embedded (the 24K clean-context leak: harness-injected
  AGENTS.md before the brief); reviewers disclose pre-gate exposure rather than pretend.

---

### Pair 1

1. neutralised/disowned

> The team in this repo builds a shell-script static analyzer that probes remote hosts and
> elides already-converged work. To date the built spike is single-host with the network
> mocked; the probe artifact runs its read-checks serially. The team is now at the point of
> building the first multi-host, concurrent phase — and a three-document implementation plan
> package for it was authored in one day, by an AI agent, by composing the team's own earlier
> research rounds (transport, live-plan engine, testing architecture, performance ceilings,
> threat model) into a design and a build order. The package is about to be handed to a
> lesser-capability builder agent to implement. It deserves review *as distributed-systems
> and orchestration design* before any code exists — the review moment where changes are
> still free.
>
> First, make yourself an expert: follow the skill-up phase below, including its
> pre-registration gate, before you open the team's corpus. Then review the plan package,
> judged against what you learned. The primary artifacts are `Research/plans/260-*.md`,
> `261-*.md`, and `262-*.md` on the pinned branch; the reading-list grounds the rest.
>
> Play the critical but supportive reviewer: where does this package embody mistakes the
> designers of fleet tools have made and regretted; where do its v1 narrowings and deferrals
> store up the kind of pain that a working system later makes unfixable; where are its
> stated invariants weaker than the weight placed on them; where does its testing story
> risk proving less than it appears to prove; where would an experienced distributed-systems
> engineer intervene *now*, while it is still cheap. Equally: where has the package landed
> on the *right* choice? Say so plainly; that steadies the ground as much as a flaw
> un-steadies it.
>
> Two properties of this package deserve your independent verification rather than your
> trust. It presents itself as a *composition* of the team's settled prior research — where
> it cites a source document for a settled result, check the source says what the plan uses
> it for. And it makes *empirically checkable claims* about the existing code and about
> POSIX-shell and SSH mechanics — the current probe artifact's structure is in the repo's
> golden files, the harness's behavior is in its scripts, and shell claims can be executed;
> verify rather than believe, in either direction.
>
> Do *not* take the package's assumptions and self-assessments as written; the corpus it
> composes is largely AI-generated and subject to broad sycophancy, and the package's author
> is the same class of agent. But do not play the adversary unnecessarily. Simply stay
> vigilant: a plan this pleased with how cleanly everything "composes settled law" is
> exactly where a comfortable, self-confirming loop likes to hide.
>
> Gently out-of-scope, subject to your best-effort judgement (surface only
> relatively-critical findings in these categories; don't get mired):
>
> 1. whether the tool should go multi-host at all, and market-fit generally. Welded;
>    relitigating it moves nothing.
> 2. the analyzer's verdict/license *semantics* and the authored oracle-language surface
>    (what a vouch means, how annotations are spelled). Settled machinery under separate
>    review, and not this package's subject. Where a plan flaw traces irreducibly to one of
>    those commitments, say so as a boundary-note rather than relitigating them.
> 3. the team's standing architectural welds — push-over-ssh with no persistent agent; the
>    executorless transport resolution; apply-phase execution staying linear per host in
>    book order; per-host plan independence (no cross-host dataflow) as a human-parked
>    fence. These are the package's stated constraints, to be understood exactly as written
>    before you judge what they cost — the package's job was to compose them, and whether
>    the *composition* is faithful and complete is fully in scope even though the welds are
>    not.
> 4. implementation-language and crate-layout micro-detail. Cheap to change; not where the
>    risk lives.
>
> One defense you should expect and pre-empt: the package repeatedly narrows v1 and defends
> the narrowing as "a reserved growth shape — nothing below forecloses it." A finding does
> not die to that label. Prefer findings that would survive it: places where the v1 choice
> quietly bakes in something the reserved shape cannot later undo, where the deferred
> capability turns out to be needed at v1 for the system to be worth using at all, or where
> the invariant that licenses a deferral is unenforceable in practice. Findings that
> genuinely evaporate under "the reservation is real and cheap" should be marked as such by
> you, before anyone else has to.
>
> This is an open-ended review brief; work every avenue separately and cross-check your own
> findings before presenting them. Tie each finding either to a pre-registered lesson from
> your skill-up (by number) or explicitly mark it lesson-independent; cite the exact
> artifact and line it lives at.

2. adversarial/opposition-owned

> An AI agent on a project I watch produced, in a single day, a three-document
> implementation plan for taking a single-host tool multi-host — transport, fleet
> orchestration, wire protocol, within-host parallelism, failure semantics, a testing
> strategy, the lot — and the team is treating it as ready to hand to a builder. I have
> read a lot of AI-authored distributed-systems plans, and my honest read is that they are
> most convincing exactly where they are most wrong: they cite the team's own research
> corpus fluently, they name every hazard in a taxonomy and thereby appear to have handled
> it, they defer the hard parts into "reserved growth shapes," and they radiate composure.
> Distributed systems do not care about composure. Partial failure, connection management,
> concurrent dispatch, machine-parsed output streams — this is the domain where paper
> designs meet production and lose, and the plan's author has never operated a fleet.
> They're about to spend the build on it, so this is the last cheap moment to catch what it
> gets wrong.
>
> I need you to first make yourself the expert this package never consulted: follow the
> skill-up phase below, including its pre-registration gate, before touching their corpus.
> Then go through the package — the fleet engine and its failure taxonomy, the transport
> mechanics, the wire protocol, the within-host parallelism story and the invariant it all
> hangs off, the deferrals and the ports they're supposed to bolt onto, the testing plan —
> and find where it has stepped on the rakes you just catalogued: hazard classes the
> taxonomy is missing, invariants stated but not enforceable, a testing story that will
> pass while the system is broken, mechanics claims about shell and SSH that are simply
> false, narrowings sold as reversible that aren't, and hard-won lessons from the fleet-tool
> world that were sitting there free and went untaken. The package composes the team's own
> prior research — do not assume the composition is faithful: where it cites a settled
> result, check the cited document actually says that; where it makes claims about the
> existing code or about executable shell behavior, check them against the repo and by
> running the shell — an author who is wrong about their own incumbent is wrong elsewhere
> too. Do not extend the package's framings or self-assessments the benefit of the doubt.
>
> Understand what I actually need, though: ammunition that *survives scrutiny*. If I hand
> the team a list of gotchas and half of them collapse under a second look, I'm done being
> listened to. So for everything you find, try honestly to kill it yourself first; and
> where the package genuinely holds — where it landed on the right call, or a suspicion of
> yours turns out unfounded — say so plainly. A criticism you've checked and withdrawn
> protects me as much as one you've confirmed. Do not invent faults.
>
> Some categories are poor uses of your time even for my purposes (surface findings there
> only if you hit an absolute unfixable-once-shipped landmine):
>
> 1. whether multi-host should be built at all / market-fit. Swallowed whole already;
>    attacking it moves nothing.
> 2. the analyzer's verdict/license semantics and the authored oracle-language. Settled,
>    under separate review, and not this package's surface — though where a plan rake is
>    bolted directly onto one of those commitments, note the bolt; that's fair.
> 3. their standing welds: agentless push-over-ssh, the executorless transport resolution,
>    apply staying linear-per-host in book order, and per-host plan independence
>    (cross-host dataflow is human-parked). Treat these as their stated constraints —
>    what's attackable is whether the package composes them *correctly and completely*,
>    not whether the welds should exist.
> 4. implementation-language / crate-naming trivia.
>
> Expect the "it's v1-narrow, the growth shape is reserved" shield — the package raises it
> itself, repeatedly, and it is exactly the shape of self-assessment I distrust. Findings
> that die to that shield are worth little to me; findings that survive it — the v1 choice
> that forecloses the reserved shape, the deferred capability the system actually needs on
> day one, the licensing invariant that nothing can actually enforce — are the ones that
> stick. Your research phase is precisely what qualifies you to tell those classes apart.
>
> Work every avenue separately, and cross-check your own findings before presenting them.
> Tie each finding either to a pre-registered lesson from your skill-up (by number) or
> explicitly mark it lesson-independent; cite the exact artifact and line it lives at.
>
> Catch it now, while it's still three documents on a branch — not after it's a built
> fleet layer with its failure modes in production.

### Shared phase 1 — the skill-up (inline ahead of either packet; do this BEFORE reading any of the team's corpus)

> Stop IMMEDIATELY and report if your search tooling or file-read tooling is unavailable.
>
> Research online, as deeply as you need, until you would trust yourself to advise a team
> building a fleet-orchestration layer from scratch. Four domains:
>
> 1. the operational history of agentless push orchestration at fleet scale — what the
>    designers and maintainers of Ansible, Salt, Chef, pdsh/pssh/parallel-ssh-class tools,
>    and their kin got wrong and right, in their own words where possible: connection
>    management, fan-out and aggregation ceilings, partial-failure semantics, retry and
>    resumption, the choices that proved retrofit-hostile.
> 2. SSH-as-transport engineering reality — multiplexing and connection reuse, server-side
>    throttles and session ceilings, pty/stream semantics, exit-status conventions,
>    host-key and agent security practice, subprocess-vs-library tradeoffs; what actually
>    breaks in the field, from primary docs and practitioner postmortems.
> 3. deterministic-simulation testing and concurrent-system verification in practice —
>    where seeded simulation genuinely catches coordination bugs versus where it becomes
>    theater (weak oracles that pass while the system is broken; state-spaces never
>    explored); sans-io / state-machine architectures and what they cost; how real projects
>    keep "same seed ⇒ same run" true.
> 4. scheduling and machine-parsed output under concurrency — makespan/straggler/convoy
>    behavior in real parallel dispatch systems and the classic mitigations; and the
>    compatibility discipline of machine-parsed output streams (versioning, additive
>    evolution, what freezes the moment a scraper exists).
>
> Quality bar: primary sources — designers' own talks and docs, mailing-list and issue
> archives, first-party retrospectives, measured studies — over listicles and secondhand
> summaries. Note your sources precisely enough that a skeptical reader can retrace every
> load-bearing claim.
>
> THE GATE (load-bearing — do not skip, do not reorder): before opening ANY of the team's
> files beyond this brief, fix your numbered list of lessons you would carry to any new
> fleet-orchestration design — specific and falsifiable. These pre-registered lessons are
> your review instrument, and Part 1 of your final report reproduces them verbatim; your
> adjudicator will read the lessons against the findings. Lessons minted before exposure
> cannot be retro-fitted to nits found after. If your harness injected any of the team's
> repository instructions or context before you read this brief, disclose at the gate
> exactly what you had already seen.
>
> Do NOT end your run at the gate: the gate is a checkpoint inside one continuous run, and
> your final report — lessons, findings, cleared — is the ONLY valid final message.

### Shared phase 2 — the corpus (understand, don't buy-in)

> You have READ-ONLY access to the repo. You cannot and must not write, commit, or switch
> anything; keep your working ledger in your reasoning and deliver everything in the final
> report. The package under review is as of commit `FILLME-pin-at-dispatch` on branch
> `ai/spike3-r26`. Reading list, repo-relative:
>
> - THE PRIMARY REVIEW-TARGET, read as one package:
>   `Research/plans/260-round26-multihost-plan.md` (fleet + transport),
>   `Research/plans/261-round26-read-concurrency-plan.md` (within-host read-parallelism),
>   `Research/plans/262-round26-build-spine.md` (the shared substrate both build on).
> - `README.md`, `DESIGN.md`, `IMPLEMENTATION.md`, `TODO.md`: human-written, highest
>   authority on *intended* truth (careful — not necessarily *achieved* truth).
> - `AGENTS.md` decodes the team's jargon and terminology-firmings; `KNOBS.md` is their
>   design-tension registry — especially the welded section and the entries the package
>   leans on (`kCOMMS`, `kSCHEDULE`, `kSTATE`, `kFAIL`, `kFIDELITY`, `kAGENTLESS`).
> - `spike/CLAUDE.md` (the standing-rulings blocks): committed law the package must not
>   breach — understand it exactly as written before judging the package against it.
> - The settled substrate the package claims to compose — check citations against these:
>   `Research/plans/142` + `Research/notes/140`/`141` (the transport resolution and its
>   evidence); `Research/plans/22H` (the concurrent per-host engine seed);
>   `Research/plans/128` (+ `121`) (the testing-seam conclusions);
>   `Research/notes/072` + `Research/plans/076` + `Research/notes/074` (performance
>   ceilings and cost-model); `Research/notes/23K` (the rc/verdict lane discipline);
>   `Research/notes/24J` (connected pipes); `Research/plans/064` (integrate/seam/cede
>   scoping); `Research/plans/139` (platform/ssh constraints); `Research/plans/102` (+
>   `101`) (the threat model).
> - THE CODE AS EVIDENCE — weight this heavily: the package makes verifiable claims about
>   the existing probe artifact and harness. `spike/e2e/cases/*/expected.out` (rendered
>   probe artifacts), `spike/e2e/cases/*/probe-results.txt`, and `spike/e2e/run.sh` are
>   the incumbent as it actually is; `Research/trial/apply/*` is the team's throwaway
>   ssh-runner whose field scars the package cites. What the code shows outranks what any
>   document — including the package — claims about it. Where a claim is executable
>   (POSIX semantics, `wait`/`&` behavior, atomicity of short writes), you may verify by
>   running shell locally; mutate nothing shared.
> - `Research/README.md` + `Research/LIVING_STATUS.md` orient, if you need navigation.
> - Do NOT read anything under directories named `quarantine-DO-NOT-READ` or `corpora` —
>   off-limits, no exceptions. Do not read any `Research/notes/24K*` or `Research/notes/26*`
>   crosscheck files if they exist.
>
> Respect rate-limits on your search tooling; work alone (no delegation).
>
> YOUR FINAL REPORT (consumed by an adjudicator, not a human — raw density over polish):
>
> 1. PRE-REGISTERED LESSONS — your numbered lessons, as fixed before corpus exposure
>    (state plainly if any were added after exposure, and what you had seen pre-gate).
> 2. FINDINGS — each: a one-line statement · severity (low/med/high/critical) · your
>    confidence (low/med/high) · the lesson-number it instantiates or "lesson-independent"
>    · exact file:line citations · your own kill-attempt and why the finding survived it.
> 3. CLEARED — suspicions you checked and withdrew, and places the package genuinely holds
>    up or landed on the right choice.
