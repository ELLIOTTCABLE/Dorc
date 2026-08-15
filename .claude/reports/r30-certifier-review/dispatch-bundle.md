# Dispatch bundle — r30 certifier cross-lineage review (read-only)

Conductor: Fable, r30-conduct. One lane. Standing codex ack (2026-08-14); the review
itself is a spec'd deliverable (`302`/`300` "post-land cross-lineage review").
Vantage SHA: a1535601.

=== DISPATCH: codex-review | mode=review | base=a1535601 ===
Context: Dorc is a static-analysis orchestrator for POSIX-sh runbooks; its cardinal
sin is under-execution — a wrong analysis silently eliding a command a user needed
run. A new instrument just landed: the solve-certifier, a post-fixpoint validator
that re-checks every dataflow-solver answer (per-edge `transfer(v, state[v]) ⊑
state[w]` plus a boundary clause `init[n] ⊑ state[n]`) and, on any inconsistency,
demotes that solve's ENTIRE analysis window to pre-existing conservative floors. It
is the instrument everything downstream trusts; a quietly-wrong checker is a
false-confidence machine. You are an outside lineage reviewing it precisely because
its authors' lineage shares blind spots.

FIRST, before any other reading: read, in full,
`Research/quarantine-DO-NOT-READ/AGENTS.for-builders-only.md` (repo-relative). This
dispatch is the orchestrator's explicit pointer to that one file; read nothing else
under any quarantine directory; follow its own routing guidance.

Then: read `Research/plans/302-solve-certifier-spec.md` in full (the binding spec)
and `Research/notes/303-certifier-callsite-census.md` (the call-site census). Then
review the landed work: the commit range `5e6d6788..a1535601` (eight commits), and
the current state of `spike/crates/analysis/src/{certify.rs,solve.rs,value.rs,funcenv.rs,effect.rs}`,
the aid-plane registrations it added, and `spike/crates/cli/src/main.rs`'s new
reporting.

Find where it breaks. Full latitude on angle and method; depth over breadth;
concrete over stylistic. For each finding: file/line, the failing scenario stated as
concretely as you can, and severity as you judge it.

Two claims to verify INDEPENDENTLY — from the code, not from any report or comment:
1. Raw `solve` is unreachable from production paths (the demotion + fence actually
   close every route).
2. The funcenv floor cannot feed `never_live` subtraction or edge-folding on an
   inconsistent answer (the grant-shifting hazard the spec calls out).

FAIL FAST if your file-read tools are unavailable — do not review from priors or
general knowledge. Work solo.
=== END DISPATCH: codex-review ===
