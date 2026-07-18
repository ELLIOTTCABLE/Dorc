# 27Xn — context-entry crosscheck: correctness digest (Sol/Codex, outside-lineage)

Working digest of an outside-lineage (OpenAI Codex / gpt-5.6-sol) adversarial
crosscheck of the context-entry design (`plans/27C`), run during block-context
lane-planning. Foreign lineage — decorrelated from the in-house lanes, but it
over-flags severity and ranks below them on precision, so every item below is an
UNADJUDICATED input, to be weighed under the standing crosscheck skepticism
(coherence across lanes + own verification before crediting; kill-shots are
rare, most findings are state-space exploration). The raw report is archived
out-of-line under quarantine (`27Xm-...`) per the purity protocol — raw foreign
output is digested, not propagated; this note carries the correctness-relevant
findings. Scope: `plans/27C` context-entry (distinct from the sibling
pre-implementation-plan crosscheck at `27X`).

## Findings surfaced (weigh, do not credit on sight)

### finding-elided-wrapped-site-not-revalidated  (~SUSPECT already-answered)
A wrapped site the plan elides on a plan-time in-context measurement is not
re-checked at apply; if the world changes between plan and apply, the elision
could drop a still-needed command (under-execution). ~SUSPECT this is simply the
standing plan-then-apply staleness the design already accepts — every elision
carries a plan/apply window, and the design deliberately does not chase
unattributed between-run drift (mid-apply divergence is proceed-and-flag, not a
revalidation obligation). Conductor action: confirm the wrapped-site case stays
inside that accepted residual and adds no wrapped-specific excess; if it does,
that is the real finding.

### finding-context-key-dimensional-completeness  (bears on this block)
Questions whether the context key's four dimensions {user, fs-view, netns, ρ} are
complete. If two contexts differ along some dimension the key omits, they can
alias under one key — a fact measured in one wrongly consumed for the other, or
two genuinely-distinct contexts wrongly batched into one segment. Directly
relevant to the context-slot / FactKey design landing this block. Principle worth
pinning: the context key must carry every dimension that can make two contexts
return different answers, else aliasing risks wrong fact transport. Conductor
action: settle the dimension set and the FactKey widening decision against this.

### finding-batched-checks-share-mutable-shell-state  (~SUSPECT load-bearing)
If several oracle checks execute in one shared shell segment, one check's
process-local changes — working directory, umask, shell options, variables,
functions, file descriptors, traps — can silently alter the environment later
checks run in, yielding a wrong verdict and a wrong elision. Recommendation: each
oracle check should run in a fresh, normalized subprocess (independently
constructed argv, env, cwd, descriptors, options), never a shared segment.
~SUSPECT genuinely load-bearing for any batched-check execution model.

### finding-conditional-tail-crash-consistency  (mechanism is STRAWMAN)
The §5 conditional-tail flag ("set iff the fallback body executed") is undefined
for a fallback that exits nonzero, is signal-killed, runs partially, or whose
result never returns. A command that changes state then fails, or an indeterminate
completion, could leave downstream lines holding plan-time elisions that are no
longer valid. Recommendation: the disturbed branch should activate BEFORE the
fallback runs (regardless of its return code), and an indeterminate completion
should poison affected downstream facts or stop apply. §5 already marks this
mechanism STRAWMAN, so this shapes its eventual design.

### finding-fallback-self-heals-framing-overstated  (product-framing tier)
The §4 framing that a wrong cross-dimension answer "self-heals on the next plan"
is overstated: the next plan may never run; the skipped command may be needed
before a dependent command that irreversibly consumes the absent state; and the
same incomplete measurement can repeat the same wrong answer. The
`--risk-faultless-skips` flag likely satisfies the explicit-user-dictum exception
(so this is plausibly no soundness breach), but the containment/self-healing
WORDING should be tempered to "bounded, but not self-healing." Framing, not
mechanism.

## Challenged and holds (recorded so the conductor needn't re-derive)
- The fail-closed direction — missing/unmodeled entry, missing keys, and failed
  in-context checks all fall to can't-say / guard / run, never elision — was
  challenged and holds (the sound direction).
- Unresolved mapped values are treated as unknown, not equal — the safe direction
  for preventing wrong fact transport; holds.
- Preserving the original apply bytes under guarding avoids a rewrite-equivalence
  bug; holds (does not, by itself, cure the staleness item above).

## Adjudication posture
Foreign-lane severities are NOT adopted. Each item is an input; credit only what
survives coherence with the sibling crosscheck (`27X`) plus own verification
against the built design and the standing rulings. The strongest candidates for
real design consequence this block are the context-key completeness and the
batched-check isolation items; the rest map to known positions or to STRAWMAN
mechanisms not yet built.
