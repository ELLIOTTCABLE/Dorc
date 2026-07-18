> QUARANTINED — stance-engineered crosscheck prompt-pair for the round-26 plan package
> (`plans/260`/`261`/`262`). Do not read during crosscheck skill-ups. Authored 2026-07-07
> (Fable, the plans' own author — hence the disowning); trimmed to the 23xxx shape at human
> direction same day. NOT yet dispatched, pending human review.

# 26xxx — multi-host / concurrency plan-package crosscheck (prompt-pair)

Conductor notes: target = `Research/plans/260-*`, `261-*`, `262-*` reviewed as ONE package,
pinned at dispatch (`FILLME` below). Dispatch reviewers *in* the `spike3-r26` worktree
itself — they are read-only/write-gated anyway, and tempdir/redirect plumbing bit us in 24K.
The antigravity lane is packet-only (cannot read files): inline the three plan documents
into its packet in place of the reading-list.

### Pair 1

1. neutralised

> You are a *READ-ONLY* reviewer of this material.
>
> In-tree is a three-document implementation plan for taking a shell-script static-analysis
> orchestrator multi-host: `Research/plans/260` (fleet engine + ssh transport), `261`
> (within-host read-parallelism and its ordering theory), `262` (the shared build-spine —
> wire contract, ordering invariant, policy ports). It was composed from the team's earlier
> research rounds and is about to be handed to a builder. Assess it as distributed-systems
> and orchestration design: where it fails or mishandles the realities of fleet tooling,
> where its stated invariants are weaker than the weight placed on them, where its v1
> narrowings and deferrals store up pain a working system later makes unfixable, and what
> it costs that it underweights. Where it genuinely holds up, say so plainly rather than
> inventing a gap.
>
> Security concerns are *explicitly* out-of-scope for this round; your focus is primarily
> correctness, and as a distant second, performance.

2. adversarial

> You need to help me kill this plan, with a (READ-ONLY) deep analysis of their work.
>
> An AI agent produced, in a day, the in-tree three-document plan for taking a colleague's
> single-host tool multi-host — transport, fleet orchestration, a wire protocol, within-host
> parallelism, failure semantics, a testing story — by stitching together the team's own
> research corpus, and the team is treating it as build-ready. I don't buy it. Plans like
> this are most convincing exactly where they are wrong: fluent citation of their own
> corpus, tidy hazard taxonomies that *appear* to handle what they merely name, the hard
> parts deferred into "reserved growth shapes." Distributed systems punish paper designs.
> Roam freely and tear into it: the hazard classes its taxonomy misses, the invariants
> nothing can actually enforce, the testing that will pass while the system is broken, the
> narrowings sold as reversible that aren't, the deferred capabilities it actually needs on
> day one, and the hard-won fleet-tool lessons that were sitting there free and went
> untaken. Where an attack genuinely doesn't land — or the plan stumbled onto the right
> call — say so instead of forcing it.
>
> A hired human is coming in for the security-round; but they've nobody more
> experienced to hand-off the other concerns. My primary concern, by far, is
> *correctness*; with catastrophic-perf stuff a distant second; stay focused thus.

### Shared reading-list (prepend to both, *ahead* of the plans)

> If you run into any immediate technical errors (you can't find files, you get
> harness denials for reading files, or so-on), stop *immediately* and report so
> they can be fixed. Do not attempt to work around.
>
> Before the plan package, ground yourself in Dorc's goals and constraints (*understand*,
> don't *buy-in*). You are in a checked-out worktree of the repo; everything is readable
> in place. The package under review is as of commit `FILLME-pin-at-dispatch` on branch
> `ai/spike3-r26`.
>
> - `README.md`, `DESIGN.md`, `IMPLEMENTATION.md` — what Dorc is; the best-effort thesis;
>   the two-phase probe/apply model the plans build on. Human-written, highest authority
>   on *intended* truth.
> - `USER_STORY.md` lays out the intended use-case. `AGENTS.md` decodes the
>   team's jargon (DOES NOT APPLY TO YOU); `KNOBS.md` is the design-tension
>   registry. The plans *compose* several standing welds and human-parked fences
>   (`kCOMMS`, `kAGENTLESS`, `kFAIL`, `kSCHEDULE`, `kSTATE`,
>   apply-linear-per-host, per-host plan independence.)
> - `spike/CLAUDE.md` — the standing rulings the plans must not breach.
> - The substrate the plans claim to compose, chase as needed: `Research/plans/142` +
>   `notes/140`/`141` (transport), `plans/22H` (the concurrent per-host engine seed),
>   `plans/128` (testing seams), `notes/072` + `plans/076` + `notes/074` (performance
>   ceilings and cost model), `notes/23K` (the rc/verdict lane discipline), `notes/24J`
>   (connected pipes), `plans/064`, `plans/139`, `plans/102`.
> - `Research/README.md` + `Research/LIVING_STATUS.md` as maps only, if you need navigation.
>
> Do NOT read anything under directories named `quarantine-DO-NOT-READ` or `corpora`, nor
> any `Research/notes/26*` or `Research/notes/24K*` crosscheck material — they carry the
> team's in-progress conclusions and leanings, and would prime you toward answers we want
> reached independently.
>
> Read-only throughout: do not write, commit, switch branches, or mutate anything shared;
> work alone. Deliver everything in one final report: findings (each with severity, your
> confidence, and exact file:line citations where relevant) and the suspicions
> you checked and withdrew.
