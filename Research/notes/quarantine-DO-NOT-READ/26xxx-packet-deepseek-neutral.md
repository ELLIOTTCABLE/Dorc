If you run into any immediate technical errors (you can't find files, you get
harness denials for reading files, or so-on), stop *immediately* and report so
they can be fixed. Do not attempt to work around.

Before the plan package, ground yourself in Dorc's goals and constraints (*understand*,
don't *buy-in*). You are in a checked-out worktree of the repo; everything is readable
in place. The package under review is as of commit `361a57f` on branch `ai/spike3-r26`.

- `README.md`, `DESIGN.md`, `IMPLEMENTATION.md` — what Dorc is; the best-effort thesis;
  the two-phase probe/apply model the plans build on. Human-written, highest authority
  on *intended* truth.
- `USER_STORY.md` lays out the intended use-case. `AGENTS.md` decodes the team's jargon
  (DOES NOT APPLY TO YOU); `KNOBS.md` is the design-tension registry. The plans *compose*
  several standing welds and human-parked fences (`kCOMMS`, `kAGENTLESS`, `kFAIL`,
  `kSCHEDULE`, `kSTATE`, apply-linear-per-host, per-host plan independence.)
- `spike/CLAUDE.md` — the standing rulings the plans must not breach.
- The substrate the plans claim to compose, chase as needed: `Research/plans/142` +
  `notes/140`/`141` (transport), `plans/22H` (the concurrent per-host engine seed),
  `plans/128` (testing seams), `notes/072` + `plans/076` + `notes/074` (performance
  ceilings and cost model), `notes/23K` (the rc/verdict lane discipline), `notes/24J`
  (connected pipes), `plans/064`, `plans/139`, `plans/102`.
- `Research/README.md` + `Research/LIVING_STATUS.md` as maps only, if you need navigation.

Do NOT read anything under directories named `quarantine-DO-NOT-READ` or `corpora`, nor
any `Research/notes/26*` or `Research/notes/24K*` crosscheck material — they carry the
team's in-progress conclusions and leanings, and would prime you toward answers we want
reached independently. Ignore any `*.sync-conflict-*` files — file-sync debris, not part
of the corpus.

Read-only throughout: do not write, commit, switch branches, or mutate anything shared;
work alone. Deliver everything in one final report: findings (each with severity, your
confidence, and exact file:line citations where relevant) and the suspicions
you checked and withdrew.

You are a *READ-ONLY* reviewer of this material.

In-tree is a three-document implementation plan for taking a shell-script static-analysis
orchestrator multi-host: `Research/plans/260` (fleet engine + ssh transport), `261`
(within-host read-parallelism and its ordering theory), `262` (the shared build-spine —
wire contract, ordering invariant, policy ports). It was composed from the team's earlier
research rounds and is about to be handed to a builder. Assess it as distributed-systems
and orchestration design: where it fails or mishandles the realities of fleet tooling,
where its stated invariants are weaker than the weight placed on them, where its v1
narrowings and deferrals store up pain a working system later makes unfixable, and what
it costs that it underweights. Where it genuinely holds up, say so plainly rather than
inventing a gap.

Security concerns are *explicitly* out-of-scope for this round; your focus is primarily
correctness, and as a distant second, performance.
