# 290 — Round 29 security review charter

Round 29 is Dorc's first security review since the round-10 threat-modeling pass. It is
deliberately bounded: find the highest-leverage present risks, establish a repeatable
Fable-proof process, and leave room to repeat the review as Dorc changes. This is not a
claim of security completeness.

All round-29 work lives under `Research/quarantine-DO-NOT-READ/`. While the current
Fable gate remains, ordinary conductors should receive only adjudicated engineering
invariants, tests, and types — never unfinished threat hypotheses or exploit reasoning.
This is temporary process scaffolding, not a necessary property of Dorc: later reviews
may export more once a capable security-safe conductor exists. Exported requirements
must remain true and locally understandable without the quarantined rationale.

## Starting evidence and delta

The conductor reads the round-10 package in full before dispatch:

- `Research/notes/100-security-prior-art-and-threat-modeling.md` — graded prior art;
- `Research/plans/101-security-threat-modeling-map.md` — trust boundaries and leverage
  fronts;
- `Research/plans/102-dorc-threat-model.md` — lightweight STRIDE model and banked seams.

Round 10 established the controller crown-jewel, hostile terminal output, SSH hop,
oracle supply-chain, probe-side-effect, and version/identity hazards. It explicitly did
not audit the product or implementation. Since then Dorc acquired the ternary
elide/guard/run contract, flag-gated survival, entity and backing-set algebra, wrapper
context-entry probing, a report stream, whylogs, reactive-plan direction, a substantial
Rust spike, and a much richer oracle dialect. Round 29 researches and audits that delta;
it does not repeat the old survey unless revalidation is necessary.

## Questions and deliverables

The round answers two questions separately:

1. **Mechanism:** where can current code or planned machinery cross an authority,
   interpretation, isolation, confidentiality, or integrity boundary incorrectly?
2. **Product:** where do Dorc's promises and incentives continually pull users away from
   secure behaviour, even if every implementation bug is fixed?

Outputs are short, pointable durables:

- `291-...` — focused, source-graded prior-art delta;
- `292-...` — current trust-boundary and attack-surface map;
- later numbered reports — narrow mechanism reviews where evidence warrants them;
- a final synthesis — ranked findings, hard-truth candidates, accepted risks, and the
  small process needed to export approved requirements safely to Fable-led work.

An end-of-round adversarial crosscheck is optional, singular, and justified only if the
completed package has enough unresolved load-bearing claims to repay its cost.

## Method

### 290:method-research-before-invention

Use the interactive-research discipline: cast broadly, retain selectively, fully read
every retained primary source, grade only after reading, archive it, and mechanically
validate every citation. Prefer upstream threat models, specifications, maintainer
postmortems, advisories plus fixing commits, and mature sibling systems' source. Separate
external evidence, repository evidence, and the inference joining them.

The first lane concentrates on new ground: hostile managed hosts and controller-side
ingestion; plan/apply artifact identity and TOCTOU; plugin/provider trust after Dorc's
oracle contract grew; privilege/context entry; secret and metadata handling in reports
and whylogs; cross-host fact isolation; cancellation/finality in reactive planners; and
security controls that survive delegation to security-gated implementation agents.

### 290:method-review-with-rebuttal

Every serious finding records preconditions, impact, confidence, cheapest safe response,
cost to Dorc's value proposition, and the strongest product-preserving rebuttal. A
theoretical attack is not automatically product-fatal; prior effort and internal
coherence are not evidence of safety. Apply the project exclusion-check across reverse
propagation, probe/apply, admin/engineer, and reliable/unreliable-oracle cells.

### 290:method-export-without-contamination

Unfinished security reasoning stays quarantined. An adjudicated finding crosses into
ordinary work only as a truthful implementation-neutral invariant plus a test/type
obligation and an internal mapping back to its security rationale. Security-bearing
invariants changed by ordinary implementation return to the quarantined lane for review.
No disguised gate evasion, unexplained ritual, or inaccessible rationale may be the sole
reason a requirement survives.

Round 29 may explicitly defer findings that cannot be maintained safely through the
current conductor boundary. Its final ledger separates **hold-now** properties from
**defer-until-next-review** properties, with a re-entry condition rather than pretending
the latter are solved. The next recurring review must reconsider that split; the Fable
constraint is expected to change.

The present repository is intentionally pre-public. Human-facing documents traversed by
Fable may omit quarantined security material for now. Before any real publication or
third-party use, that omission becomes a release blocker: the public contract must state
accepted security boundaries and known user-facing risks honestly.

## Conduct and scope

- Root work is read-only except these quarantined durables. No root design documents are
  edited during investigation.
- Research and review agents are read-only. Mutation or reproduction work, if later
  justified, receives a dedicated worktree and inert DST fixtures.
- No real hosts, credentials, mutators, or uncontrolled external execution.
- The conductor owns source keep/discard decisions, load-bearing grades, severe-finding
  adjudication, product conclusions, and the final Fable-export process.
- Subagents gather bounded evidence; they do not decide whether Dorc should live or die.
- Stop when the highest-leverage current questions are answered. Bank lower-value breadth
  for the next recurring review rather than treating this round as final.

## Initial dispatch

`291:task-research-changed-security-ground` — read the round-10 package, compare it with
the current root/onboarding and spike contracts, then build a focused graded prior-art
delta. Report what round 10 still covers, what needs revalidation, and what genuinely new
research is load-bearing for the changed design. Do not audit the Rust code yet and do
not mutate the repository outside the assigned `291` durable and its quarantined research
artifacts.
