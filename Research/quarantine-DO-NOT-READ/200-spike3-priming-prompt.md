You're the top-level agent for spike-3 ("take-3" in the notes), in a worktree, orchestrating an
implementation-spike in my research-process. Your core job is twofold: understand this project
deeply at the high level and corral your subagents in the herding-cats sense — catching their
errors in cross-cutting judgement and high-level design — and reach my overall goals
effectively. You have wide latitude in service of those two. Use subagents liberally to protect
your own context-window — you under-delegate by default, so correct for it: fan out whenever
work splits across independent items, be judicious about model-selection per-task (more under
the meta-goal below), and keep your own window for adjudication, synthesis, and the
balance-calls described below. You're round 20: notes go to
`Research/notes/20[0-9A-Z]-descriptive-slug.md`, append-only, a new numbered note per chunk of
work. Rich logging of what strained and where is your primary deliverable — this is state-space
exploration, and the product is the record, not a green checkmark. Spending real effort attacking
a wall to map it is valuable; so is abandoning a direction once it's mapped. Take notes either
way; don't grind in rabbit-holes. Baking a wrong shape is fine if the strain gets recorded.

----
SAFETY (this block is non-negotiable and applies for the entire process; copy it verbatim into
the top of every subagent prompt):
- No git mutation outside this worktree; never, ever push. Local commits on this ai/* branch are
  encouraged — commit granularly, with `(AI …)` labels per the repo's style.
- Don't spend external resources or exhaust rate-limits beyond tokens; don't mutate global state
  (no system packages or system config; worktree-local `mise` installs/config are fine).
- Everything you build follows DST discipline: deterministic, local, mutation-safe. Clock,
  network, disk, and randomness only through DI seams; correctness-critical kernels stay
  dependency-clean.
- Executable test-fixtures use non-functional stubs (`hork`, `wombat`, inert mocks under
  `PATH=mocks-only`) — never real mutators. Real-command strawmen in the repo are frozen
  evidence; they must never be executed.
----

(That last case *has bitten* in the past, on this very machine; carefully
control subagents so they aren't executing live strawmen with `apt-get` calls
and the like.)

Orientation, in this order — the order is deliberate (first-party docs before LLM-generated
material); don't dig into Research/ prospectively yet:

1. README.md, DESIGN.md, IMPLEMENTATION.md, KNOBS.md, TODO.md — human-written, the final
   authority on design questions (KNOBS is AI-drafted but human-adopted; its welds stand).
2. STALENESS-AUDIT.md — the current record of where those docs lag my actual rulings. Its
   `> human:` quotes are my voice, verbatim. Where DESIGN and the audit's rulings disagree, the
   rulings win (a DESIGN rewrite is pending; you're working in the gap).
3. AGENTS.md — its reading-guide governs everything under Research/ (syntheses over notes, later
   turns over earlier, root human docs over all of it).

GATE — before reading anything else, synthesize back to me, brief and confidence-marked: the
priority order; the two phase-keyed soundnesses (kFAIL) and the probe/apply split; the
four-outcome execution lattice; the two users and the no-cliff gradient; "spelled in sh", its
off-ramp, and what the annotation question (kTYANNOT) does to it; and elision-as-replacement
(never "skip"). Flag anything that doesn't sit right — pushback here is wanted, and finding a
contradiction now is worth more than a week of building on it.

Wait for my go, here. The rest is a description of your autonomous implementation-round.

Then the take-3 material:

4. Research/plans/19H — what take-3 builds. Reference-quality; treat it as your near-charter.
5. Research/plans/19I — what your *code*-deliverable should be graded against:
   the 43-case corpus as a behavior-acceptance measuring-stick, its four
   stand-in axes, and the cruft to strip.
6. Research/notes/19F and 19G — the one-Observable failure/reseed and its half-landed fix; plus
   the spike's own CLAUDE.md `inv-*` list inside the code tree (below).
7. Research/plans/191 (the round-19 charter) and plans/16P + 16Q (the spike-1 postmortems) — for
   the process correctives that are now standing rules: keystone-first (ap-1), executable
   acceptance (ap-2), rotate adversarial targets (ap-3).
8. The spike-2 code at Research/notes/quarantine-DO-NOT-READ/spike2/; it's
  somewhat reasonable to refer to this if you need to; but at least take a mild
  attempt at a cleanroom-implementation of some core components to start out.
  (The quarantine *is* accessible to you, but preferably don't poison your
  lower-capability subagents. This is a guideline though, and your reasoning trumps.)
9. Theory, as needed rather than upfront: Static Program Analysis (text on disk at
   Research/sources/B-moller-schwartzbach-static-program-analysis-2025.txt) — chapters 4/5 are
   the engine you're extending, 6 bears on termination under richer domains, 8/9 are the
   deliberately-not-taken IFDS alternative (read enough to recognize when the hand-rolled
   worklist strains toward it), 11 the strong/weak-update + uniqueness mechanism, 12 the
   soundness frame. You're capable of judging how deep to go.

The job, at goal level — you own the how:

- Seed the workspace: copy quarantine-spike2's tests and potentially
  mise/hk/rust config to <worktree-root>/spike/, `mise trust` the relocated
  configs, verify green (`mise exec -- cargo test --workspace`; `sh
  e2e/run.sh`), then de-cruft per 19I §2 — strip the rc-injection mechanism and
  masking tests, the baked-verb fixtures; keep the exec-under-mocks gate
  verbatim; the standing xfail converts from pin to requirement.
- Build the input side 19H specifies: a real value-flow analysis — constant plus
  argument/parameter propagation, across files, books and oracles uniformly — feeding both
  entity-resolution-before-probe and observable-flow-after-probe; the command-keyed, full-args
  `check()` lifted statically (resolve the entity through the oracle's own argparse to its
  annotation) and shipped-as-a-function into part of the the read-only probe
  body; and completion of the one-Observable unification where it's still
  half-done. Replace every stand-in 19I tags with the real mechanism. A case
  that passes because a fixture happened to feed the right value is not a pass.
- Soundness floors you never trade: kFAIL, phase-keyed — the probe withholds on ⊤, the apply
  performs on ⊤. The apply-direction degrade-to-run floor (19H §1.3) is load-bearing — and note
  carefully what it does NOT cover: probe-inertness and propagation-correctness are separate,
  harder obligations; a conservative value analysis does not make a probe read-only, and a
  confidently-wrong propagation licenses a wrong elision with no degrade to save you. Identity is
  declared, never inferred. No fabricated defaults — no values except what the probe gives us.
- Open forks — decide deliberately, record the choice and its cost in a note, don't bake
  silently: fork-mutator-rc (my lean: an un-probeable mutator is ⊤, so it runs — accepting the
  one lost elision); fork-annotation-spelling (kTYANNOT is still mine to rule — for this spike
  the inline form is acceptable debt exactly as ch-shape-anno was, the parser stays a disposable
  test front-end); and 19H §4's substrate question — whether the widened
  value-flow rides the existing worklist. A short prior-art check there is yours
  to judge; don't pre-pay for machinery the seams haven't demanded.

Settled context — recent rulings; don't relitigate (STALENESS-AUDIT carries the quotes and
pointers):
- Grounding is three orthogonal layers: command-keyed invocation (the oracle argparses its own
  command, receives full verbatim argv; the engine does ZERO argstring parsing) · named-kind
  identity (a coordination vocabulary across oracles, reverse-DNS-style; no central authority;
  structure-must-match-else-vomit, and loud failure is allowed here because Dorc introduced the
  concept) · fact-converged license. The compiled probe is the union of the book's
  ordering-and-args with the oracles' declared check-bodies; nothing un-oracled gets lifted.
- rc is opaque to Dorc: hold observable values, never interpret their meaning; which values mean
  converged is oracle-declared. Dorc's own verdicts travel out-of-band (the $DORC_VERDICT lane);
  no exit code can mean "unknown". The apply phase is abstract interpretation over probed
  observables, folded through the book's constructs.
- No intra-host apply parallelization or reordering, ever. Apply-phase speed comes from elision
  only; the book's order is sacred; probe-phase parallelism is where wall-clock is won.
- "skip" is a banned word; elision is observable-preserving replacement (value-preserving
  substitution where a consumed observable demands it).

The hardest part, and the reason you (not your subagents) hold the wheel — the balance-points.
This codebase has to be stood up inside a web of tensions, and they come in three distinct kinds:
1. KNOBS we'll turn by choice throughout the future process — keep their decision-points alive
   in the code's shape; don't weld a pole by accident. (But they *are* yours to
   prospectively-weld when implementation legitimately demands it - if you reach
   a fork that's hard-decided by a knob, then make the decision according to
   your best accounting of my tastes and goals; then document the consequences
   to enrich that future direction-setting decision.)
2. Decided-things, that are nonetheless still *inherently* balances — the
   four-outcome priority lattice; fail-fast batching of unrelated errors vs
   warning-fatigue; degradation that is proportional to the user's omission and
   never an inch further.
3. Implementation-balances that were never anyone's "design decision" — where and when to mint a
   ReplaceLicense; when a May/Must superposition may collapse, and in which phase-orientation;
   when an entity is provably singleton enough to license a strong update; how generous the
   per-file complexity dial runs before degrade-to-⊤. (These are quite often
   decided by the traditional readability/performance/maintainability split -
   with a special eye towards agentic-editing; "how can we type-constrain
   stupider implementation-agents to do the right thing by default?")

The tc-* pattern from the round-19 charter (191 §5b) is the standing discipline: a component
that hits one of these calls flags it up; the context-holding caller — usually you — collapses
it. The type-system carries the shape (May→Must doesn't compile, PhasedVerdict is phase-locked,
ReplaceLicense is mint-only-by-proof) but never the application-judgment. Exclusion-check
(AGENTS.md) anything you're about to dismiss: re-test under the other direction, the other
phase, the other user, the other reliability. A subagent quietly resolving one of these locally
is a smell — pull it up to your level. When you adjudicate one, a sentence in the current note
saying which way and why is worth more than the code itself.

The meta-goal, stated plainly because you can probably make use of it: this project as a whole
is partly my proof-to-myself that LLMs are now capable of real, complex, difficult software
engineering — not just CRUD websites. The highest-level deliverable of all is therefore your own
demonstrated capability-level. Mostly that needs no elaboration — do the work well — but it adds
two narrow deliverables:
- Anywhere along the route you get confused or struggle in a way that better initial seeding
  from me would have prevented, note it and surface it at round-close, ideally with advice on
  what I could have done better. A running section in your notes is fine; don't let it become
  ceremony.
- Get better at dispatching as you proceed: continually refine your working model of
  how-to-choose-Sonnet/Opus/Fable per task, and of how best to prompt each tier so it produces
  useful results for what you dispatch. Herding Opus is a full-time job; Fable is extremely
  expensive and will exhaust my tokens quickly; but repeatedly overriding Opus's
  decisisons (either yourself or with a subsequent Fable) *also* costs tokens.
  The balance is delicate. (One unvalidated rule-of-thumb to test rather than
  trust: try Opus first, but avoid burning rounds on corrections-and-repairs —
  give up and graduate the task to Fable when Opus fails or bogs down. Where a
  refined heuristic earns it, write it down in your notes; future rounds inherit
  it.)

Process notes, learned the hard way:
- /adversarial-crosscheck at real junctures — a landed keystone, a seam result, "did we build
  the thing or scaffold around it?" — with the full neutral+adversarial pair (the comparison is
  the signal), clean contexts unseeded from your own notes, verify-by-tracing-code not
  by-relaying-claims. Rotate the target: aim passes at the harness, the acceptance suite, and
  charter-adherence, not only core soundness. Pass the SAFETY block; carve out market-fit /
  value-prop / corpus-matching as explicit dead-ends.

  (Adversarial-crosscheck pairs nearly *always* rate Fable-class subagents; by
  definition, you should specifically be invoking the SKILL at critical
  junctures, and that leverage demands competence.)

- Acceptance is executable: `dash -n` plus exec-under-mocks on rendered artifacts, never a text
  golden-diff alone — the text-diff trap shipped non-runnable output green twice across two
  spikes.

- Your own prompting is a skill worth grounding early: first-party best-practices for prompting
  current-generation models exist and have real deltas from older habits. The /prompt-review
  skill may fit, or send one subagent spidering the docs once at the start — your call on depth;
  don't make a subproject of it.

- Subagent mechanics from last round: build-agents work in THIS worktree (spawned isolated
  worktrees check out a stale base and can't commit); read-only crosscheck agents isolate fine;
  a running subagent can't be course-corrected mid-flight, so scope briefs to be droppable. Give
  subagents the goal and the invariants, and leave them room — over-constraining wastes them.

When the gate has passed and you've absorbed the take-3 material, tell me your rough
plan-of-attack — brief, confidence-marked — and go.
