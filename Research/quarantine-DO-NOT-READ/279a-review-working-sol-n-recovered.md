# 279a — review working notes for the 270–278 package (sol-n)

Scratch only. These are hypotheses under test, not findings. Authority order for this
review: root docs and typed rulings, then current plans/notes; predecessor material is
used to test whether the current package actually closes its inherited obligations.

## Package claim map

- `270` gates the rebuild on block-settle, then schedules algebra/recipe/wire churn,
  context+payload, stdlib, field trial, and multi-host work.
- `271` records the human rulings; `272`–`275` redesign topology, wrappers, eval'ers,
  and value captures; `276` welds the marked/oracle dialect floor; `277` assembles the
  entity algebra; `278` is the one-page language reference.
- Knife-tier decisions are supposed to pass through the ternary coordinate comparison;
  silence licenses nothing; only `--risk-faultless-skips` admits the open-world
  at-most residue.

## Suspicions to chase

- `279-s1 executable-spec-vs-admission`: `276`/`278` say no written language spec will
  exist and define the base dialect by two shells parsing *and running* identically.
  The analyzer must admit/reject text without executing arbitrary user commands, so the
  calibration oracle may have been mistaken for an implementable language definition.
  Check `021`, `055`, parser code/tests, and the exact floor battery.
- `279-s2 pipefail-floor-fracture`: bare `set -o pipefail` is legal dorc-lang, while the
  stripped oracle must meet a pinned posh/dash floor chosen specifically because both
  lack pipefail. The self-gating idiom is blessed but not required, and strip does not
  rewrite pipefail. Test the claimed off-ramp/spec coherence.
- `279-s3 capture-built-before-semantics`: `270` schedules the read-value slice, while
  `275` leaves binding-site disposition open, artifact-entering substitution postponed,
  and `277` reserves rather than selects the required second value pass/fold-time route.
  Check `219`, current pipeline order, and whether an executable outcome is actually
  specified.
- `279-s4 book-parser-lockin`: marked/oracle syntax is welded but unmarked book tolerance
  stays tabled even though the parser's error-tolerance posture is recorded as cheap only
  at rebuild time. Check whether the scheduled rebuild crosses that seam.
- `279-s5 topology-knife-attribution`: verify whether selector-token inequality and the
  store/invariance lines always reduce wrong cross-context/survival decisions to a false
  authored line, rather than manufacturing a new faultless under-execution class outside
  the survival flag.
- `279-s6 trap-as-wall`: test whether treating `trap` registration as an opaque wall is
  enough for control-flow/observable correctness, rather than only fact staleness.
- `279-s7 package-fixpoint`: `278` still labels itself draft/awaiting delta acks and calls
  settled grammar unsettled, while `271` says it was reviewed and task 12 closed. Decide
  whether this is harmless stale ceremony or makes the gated implementation input
  non-determinate.

## Strengths already surviving first read

- The admin/engineer split is consistently carried through wrapper, topology, and value
  surfaces; ordinary books acquire no new configuration syntax.
- The ternary same/disjoint/unknown comparison correctly captures the phase inversion:
  overlap and separation are each unsafe for one consumer, while unknown is safe for both.
- `cmd__lend_map()` and the explicit invariance speech act repair prior silence-as-license
  holes; missing dimensions degrade to walls.
- The value-prediction design correctly separates byte provenance from per-channel backing
  and notices that the captured question can stale independently of its answer.
- The package repeatedly preserves DST/DI requirements and keeps host interaction outside
  the correctness kernel.
