Analyzer/engine data-needs
==========================

The living registry of *what the Dorc analysis-engine must represent, compute, track, or expose* — and
*which part of Dorc needs it*. This is `ananeed-1` (consumers) × `ananeed-2` (information), the input to
`ananeed-3` (which algorithms/data-structures satisfy it — see `Research/plans/170`). Human-maintained;
add a row when a feature implies a new datum. Elaboration, prior-art, and the algorithm-matching live in
`plans/170` (do not put prose here).

Seeded round-17 by a 7-way neutral fan-out + a 3-way adversarial gap-hunt over the planning corpus
(rounds 00–16, all worktrees); raw extraction + method in `Research/notes/172`. Rows are deliberately
over-inclusive.

**Columns** — `need` (slug, `an-*`) · `information` (the datum) · `needed-by` (consumer/phase/feature) ·
`dual` (paired half, if any — many needs come in tension-pairs) · `refs` (pointers into `Research/` /
root docs; not exhaustive) · `st` (status).
**`st`** — `B` built in the round-16 spike · `S` specified in design, not built · `D` designed-but-deferred
by the spike · `O` open decision (high-lock) · `W` welded (settled).
Inside cells, `/` replaces `|`; `↔` marks a dual.

---

## A · Core dataflow lattice & the two soundnesses

| need | information | needed-by | dual | refs | st |
|---|---|---|---|---|---|
| an-may-set | forward over-approx fact-set (⊥-start, ∪/⊔-merge): truth ⊆ this | elision-soundness; necessary-probe set | an-must-set | 055§1A, 052, 16P-T4 | B |
| an-must-set | under-approx fact-set (order-dual: ⊤-start, ∩-merge): truth ⊇ this | apply-3 elision license; convergence | an-may-set | 055§1A, 16P-T4, 096 | D |
| an-merge-op | the confluence/meet operator per analysis (∪ for may, ∩ for must); IFDS *requires* it be exactly union/intersection — wrong choice = silent unsoundness | solver; an-distributive-split | — | 052, 093-f20, 16P-T4 | B |
| an-transfer-fn | the per-node transfer function itself — `(in-state, command-effect) → out-state` that the solver iterates; the thing oracles parameterize | solve(); every analysis | — | 055§1A, 052, 021§2 | B |
| an-orientation-coercion | `Must→May` legal, `May→Must` a compile-error; degraded belief never re-licenses | elision-soundness lock | — | 16P-T4/T12, note-165 | B |
| an-effect-class | per-command class: Pure / Establishes(F) / Kills(F) / Opaque / ⊤ — where Establishes/Kills carry a *location-set* (an-mutated-location-set), and Opaque (poisons ambient-ness locally) ≠ ⊤ (absorbing CFG node) | the transfer function; oracle interface | — | 055§1A, 021§2, 16P-T7/T8 | B |
| an-establish | command gen's fact F (`pkg:nginx` present) | reaching-defs; state-closure | an-kill | 055§1A, 16P-T7, 099§0 | B |
| an-kill | command invalidates fact F | reaching-defs; written-stale | an-establish | 055§1A, 16P-T8 | B |
| an-require | command/guard requires fact F (precondition; consumer-guard) | dependence edges; ordering | an-establish | 099§0, 094-g1 | S |
| an-conflict | facts/commands that mutually conflict ("foos don't bar") | reorder-legality; scheduling | — | 099§0, 055 dec-2 | S |
| an-reaching-ambient | reaching-defs of oracle gen/kill over the *system store* (not vars): is F's resting probe authoritative | the cardinal wrong-elision guard | an-written-stale | 16P-T8/DP-7, 090§0 | B |
| an-written-stale | upstream same-fact mutation reaches here ⇒ resting probe stale ⇒ MustRun (`purge;…;install`) | apply correctness | an-reaching-ambient | 16P-T8, 16A | B |
| an-entry-reachability | command unreachable-from-entry / in a detached region ⇒ MustRun (vacuous-⊥ ≠ ambient) | ambient gate (detached-region guard) | — | 16P-T8/DP-8, note-167 | B |
| an-convergence-trust | a capped/non-converged solve is partial; `!converged ⇒ ⊤` is a *per-consumer* obligation | every analysis consumer | — | 16P-DP-9, note-167 | B |
| an-reachable | forward reachability over CFG (dead-code ⇒ don't probe); unknown control ⇒ reachable | dead-code prune; probe-set | an-dead-code | 055§1B, 054 | S |
| an-top-unknown | ⊤ = unmodeled/unknown/unanchored/transient ⇒ un-probeable ∧ un-elidable | both phases' fail-safe | an-bottom-pure | 055§1A, 099§0, 16P-T2 | B |
| an-bottom-pure | ⊥ = provably no prestate mutation (scratch it owns) | probe-soundness; inert classify | an-top-unknown | 055§1A, 051 | B |
| an-scratch-ownership | state this run *created* vs state that pre-existed (Salcianu inside-node vs param/load-node); a `/tmp` file it also removes is ⊥-pure | an-bottom-pure classification; effect summary | — | 051, 055§1A | S |
| an-must-may-grade | per-belief MUST (idiom-implied or oracle-declared) vs MAY (mined/distributional) | elision rides MUST only | — | 099§0/§7, 096, 16P-grade | B |
| an-observable-liveness | which observables (rc/stdout/stderr/effect) are consumed downstream; replace = cheapest stand-in reproducing live observables | elision (replace-not-skip) | an-vouch-default | 16P-T10, AGENTS "skip"→replace | B |
| an-vouch-default | a stub default is sound iff dead-or-vouched: effect←convergence, status←establishes-contract, stdout/stderr←nothing | observable-liveness gate | an-observable-liveness | 16P-T10, note-16F | B |
| an-output-consumed-enclosing | output-consumption is a property of the *enclosing* construct (`{cmd;}>f`, `(cmd)|grep`), not leaf-local; `/dev/null` exempt | observable-liveness (kill-shot fix) | — | 16P-T10, note-16I | B |

## B · Shell-execution-environment state (CFG-coupled — "must model or be silently unsound")

| need | information | needed-by | dual | refs | st |
|---|---|---|---|---|---|
| an-errexit-state | `set -e`/`+e` (and `$-`-conditional) on/off per point — *alters the CFG itself* (failure→exit edges) | reachability; elision soundness | — | 021§2, 16P-T9, 092-spec | B |
| an-errexit-precise-edges | prune failure-edges where shell won't abort (`!`-negation, if/while cond, `&&`/`\|\|`-left, `\|\| true`); extend where it does (failing redirection) | forward skip *and* backward apply-slice | an-errexit-spurious | 16P-T9, note-166 | B |
| an-errexit-spurious-hazard | a *spurious* fail→exit edge is unsound *backward* (apply-slice sees always-reached mutation as bypassable) | apply-3 minimization | an-errexit-precise-edges | 16P-T9, note-166 | D |
| an-conditional-toggle | option/trap state set *inside a branch* (`if …; then set +e; fi`); the dataflow path-*merge* of option-state, not just its presence | errexit/option precision | — | 09A§3a, 021§2, 092-spec | S |
| an-shell-options | `pipefail` / `nounset` / `noglob` / `IFS` state (affect reachability + arg-expansion); conditional toggles | reachability; word-splitting | — | 092-spec, 16Q dq-envstate | S |
| an-word-expansion | argument/word-splitting + glob + unquoted-expansion as a first-class computed hazard ("80% of scripts have ≥1 smell"); unquoted `$x` ⇒ argv is ⊤/uncertain | entity-extraction; operand binding; ⊤-surface | — | 021§2 (first-class hazard), 09A§3a | S |
| an-cwd-state | current working dir; subshell-scoped `cd`; `pushd/popd` stack | path resolution | — | 092-spec, 09A§3a | S |
| an-trap-state | trap handlers (EXIT/ERR/signal); canonical + *conditional* registration; a contract, not a detector | transient-cleanup; detached edges | an-trap-fires | 092-spec, 09A§3b, 16P-§3.2 | S |
| an-redirection-effect | a redirection is a mutation site independent of the command word (`: > /etc/x`, heredoc-write); read `redirs` not just `words` | effect map; CFG hazard | — | 021§2, note-16G | S |
| an-fd-state | file-descriptor table; `2>&1`/`>&3` fd-dup resolution (beyond structural floor) | observable-liveness precision | — | 092-spec, 16P-§3.2 | D |
| an-exit-status | per-command `$?` and how it composes with errexit/`&&`/`\|\|`/`\|\| true` | guard eval; reachability | — | 092-spec, 16O | S |
| an-scope-containment | `( )`/`$( )` contain env/var/cwd/option mutations (don't escape); FS does; brace `{ }` leaks | effect scoping (inverse of transient) | — | 021§2, 16P-T9, 09A§3a | B |
| an-expansion-internal | `$(…)`/backtick body commands are effect-bearing *non-leaves* (run at word-expansion); kept in dataflow, excluded from leaf-set | leaf-seam vs effect-scope split | an-leaf-id | 16P-T9/T14, note-16B | B |
| an-concurrency-edge | `&` background + `wait $!`; pipeline-stage subshell isolation; a backgrounded flip can perturb state untraced | CFG hazard; ambient invalidation | — | 021§2, 16A | S |
| an-strict-mode-signal | `set -euo pipefail` preamble = native signal "ordered · fail-fast"; `set -e` presence ≈ "treat as ordered" | order/reorder-legality inference | — | 09A§3c, specimen-090 | S |

## C · System-state fact model & entity-algebra (the precision keystone)

| need | information | needed-by | dual | refs | st |
|---|---|---|---|---|---|
| an-fact-domain | partition state into kinds: package / file / service / user / port / mount (sparsity, per-domain tuning) | fact-store keying | — | 053-Q3, 055 dec-5, 16P core | B (decorative) |
| an-host-identity-fact | host self-identity a guard reads: hostname / host-group membership / `$(uname -m)` arch ("is our hostname in <set>") — a fact-kind beyond pkg/file/svc | per-host guard eval; fact-domain | — | README, DESIGN, 09A | S |
| an-entity-shape | flat (`package:nginx`) vs structured w/ selectors (`{installed, version, held}`) — re-keys every transfer + the substrate | precision/recency layer | — | 16Q dq-entity-algebra, 055§1B | O |
| an-strong-weak-update | strong (overwrite — needs provably-unique entity) vs weak (accumulate, ⊤-ward); the recency lever (TAJS 87%→<2%) | precision; skip-rate | — | 16Q q1-precision, 055§1B, 054 | O |
| an-fresh-vs-summary-entity | recency partition: the most-recently-touched entity (singleton `@ℓ`, strong-updatable) vs the summarized older/unknown bucket (weak only) — the *representational shape* `16Q§1` calls retrofit-hostile (distinct from the mechanism) | strong-weak-update; the substrate shape | — | 054, 055§1B, 16Q§1 | O |
| an-entity-uniqueness | singleton/uniqueness/linearity gate licensing a strong update; literal arg ⇒ likely unique, `$var`/glob/loop ⇒ maybe-aliased | strong-update soundness | — | 099§3, 092, 16Q dq | O |
| an-entity-coref | intra-script co-reference: do two opaque tokens denote the *same* entity (engine compares, never decodes) — the equality relation uniqueness + reaching both stand on | an-entity-uniqueness; an-reaching-ambient | — | 16P-T7, 055 dec-5 | B |
| an-per-entity-selector | `installed` vs `version`; `svc#enabled` vs `#active` — strong/weak operates per-selector | precision layer | — | 16P-§3.2, 092 seq-1 | O |
| an-partial-convergence | one leaf over a *set* (`apt install nginx curl jq`) needs a per-entity verdict map, not one verdict | multi-entity elision | — | note-162-O4 | D |
| an-mutated-location-set | an effect is "mutates *these* state-paths" (Salcianu regex), not a y/n bit — the granular footprint that buys probe-only-what's-touched | an-effect-class richness; 1B precision | — | 051, 055§1B | S |
| an-modified-since-entry | per-function/role "what state touched since entry" (TAJS `Modified`) — discards infeasible cross-call flow AND *is* the role-summary footprint | an-summary-instantiate; precision | — | 054, 051, 055-Q3 | S |
| an-benign-mutation | which fields/facts a cmd touches so a human/oracle can vouch a mutation *benign* (cache/timestamp, semantically-preserving) — distinct from volatile | calibration adjudication; pure-classify of caches | an-volatile | 051 | S |
| an-opaque-token | fact = (opaque-token, source-expr); engine never decodes token text — compares for intra-script co-ref, resolves for display only | referent-agnostic core | an-named-kind | 055 dec-5, 16P-T7, 099-W4 | B |
| an-named-kind | cross-oracle identity binds to a *named kind*, never a shared arg-token (token-equality collides across oracles) | cross-oracle coherence | an-opaque-token | 099-C3/W4, 16P-T7 | B |
| an-ambient-vs-transient | per-fact temporal stability: ambient (resting value, probeable) vs transient (no resting value, un-probeable) | hoist-safety; probe-vs-just-run | an-volatile | 099-W5, 090§0 | B (via reaching) |
| an-volatile | nondeterministic state (clock/`$RANDOM`/network) — distinct from transient; ⊤; canonicalize for any cache | kVOLATILES; memo hermeticity | an-ambient-vs-transient | 090§1, 076#10, 099-C4 | S |
| an-cross-host-kind | facts host-local by default; genuinely shared state (LB↔backends) needs an oracle-declared cross-host kind (write-skew) | multi-host consistency | — | 090 D5, 16A | S |
| an-hoist-safety | guard hoistable to T0 probe iff its fact is ambient ∧ invariant entry→guard (no in-script gen/kill reaches; no external mutation) | kFLATTEN-hoist | an-maintain-cfg | 090§0, 099§5 | S |
| an-maintain-cfg | non-hoistable (transient/written-upstream) guard keeps its control-flow in the shipped probe | kFLATTEN-maintain-cfg | an-hoist-safety | 090§0, 16P probe | S |

## D · The oracle contract & cross-oracle

| need | information | needed-by | dual | refs | st |
|---|---|---|---|---|---|
| an-kind-index | the 3-place relation `(kind, provider, verb) → effect` (NOT a 1-place naming convention — that clobbers) | effect map; cross-oracle | — | 16P-T7/DP-1, note-162 | B |
| an-fact-probe | oracle's read-only probe of `kind:entity`, three-outcome (holds/absent/can't-tell) | probe compilation | — | 16P-T7, 099§6 | B |
| an-probe-shape | probe must capture the tool's *own* rc — `cmd \| grep -q` can't emit Unknown (no-match rc ≡ tool-fail rc) | three-valued verdict | — | 16P-DP-3, note-162 | S |
| an-effect-polarity | accumulating `(provider, verb, establish/kill)` tuples; multi-verb without clobber (`apt install` vs `purge`) | kind-index / gen-kill | — | 16P-T7, note-162 | B |
| an-fact-centric | probe a *fact*, never dry-run the mutator (named-kind index was decorative in the command-centric strawman) | the oracle-contract pivot | — | 16P-DP-1, note-161→162 | B |
| an-version-applicability | derive the *version-correct* `.check`/effects from `install foo@3.5` (version-bearing-fact normalization, API-node resolution) — the MH2 layer | oracle-contract; effect map; MH2 | an-oracle-decls | TODO-ADDTL MH2, 064:19 | O |
| an-privilege-fact | a command requires privilege (root/become/sudo) — a Tier-B `permission` fact the engine *knows*, never escalates | apply ordering; plan UI; tier-B | — | 064 become, 09A tier-B | S |
| an-oracle-decls | per-oracle `.check` / effect-class / `.diff` / `.version` declarations the engine lifts statically (never sources/runs) | the whole pipeline | — | 021§0, 16P-T7 | B |
| an-cross-oracle-coherence | multiple oracles grounding one kind must *agree* (type-class coherence; kind=class, oracle=instance) | cross-oracle soundness | — | 099-C3, 09A§1 | S |
| an-provide-equivalence | oracle-declared `provide`/equivalence/wrinkle relations (`apt≡dpkg`, `neon≡ubuntu`, `arm64≡aarch64`) in plain sh; the m×n abdication mechanism | cross-provider identity | — | 09A§2, specimen-091, 16Q X3 | O |
| an-managed-vs-runnable | cleave "is X runnable?" (`$PATH`, bakeable core fact) from "is X managed?" (abdicated kind) | availability vs state-mgmt | — | 09A§2, specimen-091 | S |
| an-tier-a-forms | structurally-blessed idioms recognized for everyone, no oracle (`[ -f X ]`, `command -v X`, `[ A -nt B ]`, `(cd X &&…)`, `PROBE\|\|ESTABLISH`, `\|\| true`) | Must-grade without oracle | an-tier-b | 09A§3c, 16Q tier-A | D |
| an-tier-b-declared | oracle-declared probe/establish commands (plural-by-design, no canonical form) | the abdicate bucket | an-tier-a-forms | 09A§3c | S |
| an-guarded-establish | normal-form `if ! PROBE; then ESTABLISH` ≡ `PROBE \|\| ESTABLISH`; shared arg = entity-link; guard polarity = probe-vs-establisher | spec-carrier recognition | — | 094-g1, 099§4, 16Q | D |
| an-reflexive-inertness | run the effect-analyzer on a lifted *probe body*; flag any cmd another oracle calls Establishes/Kills — a *detector* (Opaque must PASS), not a gate | cross-oracle probe-inertness backstop | an-withhold-sandbox | 16Q dq-reflexive, TODO 2026-06-06 | S |
| an-enrichment-nudge | "company-it-keeps" record of an *unrecognized* guard/probe-shaped command → hint "this looks like a guard, write an oracle" — the best-effort output when grounding is absent | oracle-bootstrap UX; kBURDEN gradient | — | DESIGN inference-limits, 096 | S |
| an-oob-config-redline | config must be spelled in sh (no YAML/frontmatter/pragma/comment-parse); OOB *metadata* transport is fine | kOOB redline | — | KNOBS kOOB, 111§0 | W |
| an-two-user-tag | per signal/contract: serves admin/deployer (we-infer) vs engineer/oracle-author (declares); gradient, no cliff | kBURDEN; UX | — | KNOBS kBURDEN, AGENTS | S |
| an-contract-triple | a contract's {what-promise · who-declares · where-in-sh} | contract notation | — | 090§0.5, 099§4 | O |
| an-inventory-input | the host *list* + group structure, *read* from existing sources (ssh config / inventory / flat file); Dorc consumes, doesn't author | per-host instantiation; fan-out; soundness | — | 064 SEAM-inventory | S |

## E · Provenance — certainty / taint sense

| need | information | needed-by | dual | refs | st |
|---|---|---|---|---|---|
| an-claimed-vs-proven | per-fact: oracle-CLAIMED (distrusted, conditional on oracle correctness) vs engine-PROVEN | the soundness-boundary doctrine | — | 102 soundness-boundary, 110-f4 | S |
| an-distrust-propagation | a fact derived from an oracle-claimed fact inherits distrust (taint flows through derived facts) | trust accounting | — | 102, 110-f20 | S |
| an-grounding-boundary | marks where soundness stops (analyzer-proven propagation vs author-grounded behaviour); eliminate only in Dorc's own code | transfer-to-contract decision | — | 102 soundness-boundary, DESIGN | S |
| an-grounded-wrapper | `Grounded<T>`/`OracleConditional<T>` making oracle-trust dependency *type-visible* (no silent conflation); discharged by kFAIL-fold | best-effort-under-degradation | an-claimed-vs-proven | 16P-T12, note-16D | D |
| an-hint-applicability | machine-readable confidence on a proposed fix/hint (MachineApplicable→MaybeIncorrect), separate from the message | auto-apply gate; lint nudges | — | 110-f16 | S |
| an-tamper-bit | a security/trust status bit on a node, carried through transforms | supply-chain provenance | — | 110-f20 | S |

## F · Provenance — derivation / why sense (the locator DAG)

| need | information | needed-by | dual | refs | st |
|---|---|---|---|---|---|
| an-locator-dag | provenance is a per-host-forking N-tier DAG of typed loci, a *variable-length list*, never pre-flattened (composition collapses to coarsest tier) | error reporting; plan UI; faithful mode | — | 111 dac-A, 110-f30, KNOBS kFIDELITY | S |
| an-loc-host | which host qualifies a downstream locus | fan-in; per-host attribution | an-loc-user-src | 110-f30 | S |
| an-loc-session | per-host probe-run/session id minted by the *transport* and threaded with the shipped unit (traceparent-style) so a result self-correlates at the controller | fan-in correlation; backward resolution | — | 140-f38, 112-f49, 111 di-3 | S |
| an-loc-user-src | literal user-FS file+line/span — the *editable* thing they go fix (the off-ramp) | surfacing | an-loc-probe | 110-f30 | S |
| an-loc-probe | line in the per-host *mutated* probe script (remote-debug) | remote debug | an-loc-user-src | 110-f30 | S |
| an-loc-surface | the dependent mutation line where a diagnostic is *shown* (carries its own loc-user-src) | surfacing | — | 110-f30 | S |
| an-edge-derived-from | transform/lift edge (user-sh → oracle-lift → probe line) | why-this-probe-line | an-edge-ran-on | 111 di-2, 110-f33 | S |
| an-edge-ran-on | distribution edge (which host executed) | host attribution | an-edge-depends-on | 111 di-2 | S |
| an-edge-depends-on | dataflow/CFG/taint causal link — *this edge IS the analyzer's own dependency graph* | why-elided; impact | an-edge-derived-from | 111 dac-B, di-2 | S |
| an-fork-discriminator | one source line → N host-specific probe images, tagged by a discriminator (dup-factor + copy-id) | per-host provenance | — | 111 dac-A, 110-f45 (LLVM) | S |
| an-bidir-resolution | forward query (source → its fork-set of probes/dependents) ↔ backward query (host failure-id → editable source) | both resolution directions | — | 110-f37/f46 | S |
| an-origin-handle | every node carries a cheap interned origin-handle (span-id), NOT the sh text; resolve lazily; store *relative* (survives edits) | hot analysis path; incremental | an-lazy-snippet | 111§1, 110-f14/f25 | S |
| an-lazy-snippet | full sh snippet fetched from a controller-side SourceMap only at report time | reporting | an-origin-handle | 110-f14, 111 dac-C | S |
| an-graft-provenance | a transform *grafts* input-provenance onto produced nodes (detachable metadata), never regenerates positions — how provenance survives oracle-lift/probe-compile | locator-DAG construction; faithful mode | — | 110-f20/f25, 111§1 | S |
| an-why-elided | per-leaf record: what the probe elided/replaced and *why* (reason-class) | plan-preview; deopt | an-why-probed | 113-f64, 055-Q2 | S |
| an-why-probed | derivation: why a line *was* included in the compiled probe (host not skippable) | plan/diff UI | an-why-elided | 055-Q2, 113-f64 | S |
| an-oracle-src-line | which oracle source file+line established a given fact ("your oracle line 12, lifted for nginx") | blame; derivation | — | 110-f19/f31 | S |
| an-blame-rootcause | correlate a detection point back to the causing user edit, across distance/intervening errors | root-cause reporting | an-cascade-suppress | 110-f26 | S |
| an-cascade-suppress | a ⊤/poison fact propagates as "unknown — conclude nothing downstream," silencing follow-on diagnostics in the lattice | report quality | an-blame-rootcause | 110-f23, 111§1 | S |
| an-meta-provenance | which pass/oracle/rule/tool-source emitted a given diagnostic | debugging the engine | — | 110-f17 | S |
| an-result-diagnostics | every stage yields (best-effort result × accumulated diagnostics) and *never throws* | every phase (dn-7) | an-diag-accretion | 111§1, 110-f1 | S |
| an-diag-accretion | that accumulator is carried *through* per-host transform/distribution, accreting a diagnostic at each tier (the warn that "rode along") — a locator-DAG payload | faithful upstream-warning surfacing | an-result-diagnostics | 110-f30, 111§1 | S |
| an-error-node | one explicit error/poison node-kind (⊤), not an "invalid" flag everywhere; carries span + partial facts + best-guess; see-through is a traversal parameter | recovery; reporting | — | 110-f3/f22/f24, 111§1 | S |
| an-diag-catalog | analysis-failure messages as a separate declarative catalog, kept complete-vs-the-engine by a mechanical coverage gate | reporting; message maintenance | — | 110-f6, 111§1 | S |
| an-one-diag-spine | parser/analyzer/planner/orchestrator faults funnel into ONE diagnostic representation, not four silos | uniform reporting | — | 110-f9, 111§0 | S |
| an-output-sanitization | which displayed-plan/diagnostic bytes are attacker-influenceable (hostile filename / terminal-escape) ⇒ strip/escape control chars before TTY render | plan-as-shell display; report layer | an-diag-secret-taint | 102-E5 | S |
| an-graph-type-agreement | analyzer and error/provenance layer must agree the graph types *first* (depends-on edges ARE the dataflow output) or build two incompatible graphs | architecture precondition | — | 111 dac-B (highest-leverage) | O |
| an-diag-vs-verdict | transient chronological diagnostic *stream* kept distinct from the durable structured *verdict* | reporting model | — | 111§0, 112-f50 (K8s) | S |
| an-diag-secret-taint | runtime stderr leaks secrets/paths; the controller aggregating every host's stderr *is* the whole-fleet target | provenance transport security | an-output-sanitization | 111§0, 102-E1 | S |

## G · Verdict, memo & freshness

| need | information | needed-by | dual | refs | st |
|---|---|---|---|---|---|
| an-verdict-3val | per-probe verdict ∈ {ok / fail / unknown}; non-response/timeout/killed ⇒ unknown, never clean | probe→apply handoff; elision | — | 111§0, 16P-T5, 112-f50 | B |
| an-verdict-unknown-range | Unknown attaches to a leaf *range* (leaves after the last received marker), bounded by last-marker | truncation handling | — | 128 fc-2, 126-f39 | S |
| an-verdict-phase-keyed | verdict read per-phase: `PhasedVerdict<Probe>` unreadable as `<Apply>`; one lattice can't carry the two opposite kFAIL ⊤s, so Unknown's *orientation* must be phase-keyed too (151-SF3 flagged hole) | kFAIL separation | an-kfail | 16P-T5/T11, 151-SF3 | B (tension O) |
| an-verdict-reason-id | each verdict carries a programmatic Reason id (machine-stable) + human Message, distinct — Reason required (K8s metav1.Condition) | programmatic verdict-handling; UI | — | 112-f50, 111§0 | S |
| an-verdict-last-transition | when the verdict last *changed* (LastTransitionTime), distinct from freshness/TTL | diff/resumability; flap detection | an-freshness | 112-f50, 111§0 | S |
| an-verdict-noop | a why-run/noop flag on the verdict/report — "observed in a no-mutate pass" vs "produced by applying" | probe-vs-apply provenance; plan UI | — | 112-f49, 141-g7 | S |
| an-verdict-failsafe-default | a verdict defaults "failed/unknown until finalized," so a crashed/dropped run never reads clean | kFAIL-perform; crash-safety | an-host-as-adversary | 112-f49, 151-SF3 | S |
| an-content-key | dependence-derived hash of the *fact-slice* a verdict reads; under-keying = unsound reuse; canonicalize (strip volatiles) *before* hashing | cross-host memoization | an-freshness | 076#9, 16P-§3.2, 110-f40 | D |
| an-freshness | freshness/TTL/staleness + observed-spec-version on every verdict; reuse only in-window | memo invalidation; incremental | an-content-key | 076#9, 111§0 (observedGeneration) | D |
| an-verdict-memo | the content-addressable `{verdict, content-key, freshness}` store shape | kSTATE-persist; reuse | an-stateless-recompute | 076#9, 083§5 | D |
| an-stateless-recompute | host reality / on-disk truth as the one ground truth (recompute, no central stale state) | kSTATE-recompute | an-verdict-memo | KNOBS kSTATE, rust-analyzer | O |
| an-equivalence-class | fleet equivalence-class key (hosts sharing a fact-slice content-hash) for N·H→H reuse | cross-host memoization | — | 076§4b, 083 Q-HOMOGENEITY | D |
| an-report-correlation-id | run-level correlation/transaction id linking a compiled plan to its N per-host reports (Puppet transaction/catalog uuid) | fan-in roll-up; plan↔result join | — | 112-f49, 140-f38 | S |
| an-hermeticity | verdict = pure function of declared, canonicalized host-state; volatiles striped *before* hashing | sound caching; DST | — | 076#10, 099-C4, 110-f40 | W |
| an-early-cutoff | post-mutation state-fact unchanged ⇒ dependents still skip; "changed?" as a first-class value (handlers dissolve into if-changed) | apply precision; notify/handlers | — | 076§4b, 075, 064 | S |
| an-toctou-window | the probe→apply gap is a TOCTOU window; option to OCC-revalidate the hoisted read-set at apply before mutating | hoist soundness across the gap | — | 090 P3, 16A | D |
| an-host-reachability | reachable/unreachable/timed-out per host; unreachable ⇒ unknown (not converged); a missing host is a diagnostic node | fleet elision-soundness; fan-in | — | 076§3a, 111 dac-D | S |

## H · Leaf-seam, identity & fidelity

| need | information | needed-by | dual | refs | st |
|---|---|---|---|---|---|
| an-leaf-id | stable per-leaf id ↔ source AST node (oracle file+line) back-map; carried through both runners | provenance; plan-as-shell; tracer | an-expansion-internal | 077§3, 16P-T14, 083§5 | B |
| an-wrappable-seam | each leaf executable via a process-level indirection (`<wrapper> -- <leaf>`), pluggable, *never one opaque `sh -c`* | both runners; tracer; DST | — | 077 core-req, 083§5 (dn-3) | B |
| an-leaf-env-aug | per-leaf env injection (`DORC_LEAF_ID=…` correlation token, `LD_PRELOAD=…`) | trace/syscall attribution | — | 077 core-req-2 | S |
| an-dorc-exec | the single `dorc_exec(host, leaf)` chokepoint is *triple-use*: prod-spawn / trace-point / DST fault-substitution | the one retrofit-hostile day-1 seam | — | 128 se-1, 124-f27 | O |
| an-fidelity-mode | optimized (batched/hoisted/elided) / faithful (1-leaf-1-exec, control-flow preserved) / simulated (DST) | kFIDELITY; debug; tracer | an-batch-attribution | KNOBS kFIDELITY, 16P-T14, 124-f29 | B (faithful partial) |
| an-batch-attribution | even when batching, a recoverable map of which source leaves a batched check stands in for (don't collapse provenance for ms) | optimized-mode provenance | an-fidelity-mode | 077 opt-hazard, 151-M3 | O (tension) |
| an-leaf-scope | what *is* a leaf: top-level / branch-body / subshell-body / group-body ARE; `$()`-internal is NOT; pipeline-as-one-leaf unsettled | leaf-seam; apply executor | — | 16P-T14, note-16B | B (pipeline O) |
| an-leaf-text | a leaf's runnable text isn't always one `[lo,hi)` slice — must cover heredoc body, offset-corrected `$()` spans | faithful/runnable render | — | 16P-§3.2, note-169 | D |
| an-render-runnable | the rendered apply artifact must be executable / `sh -n`-clean POSIX (the spike's `if true; then # …; fi` is a syntax error) | apply-2 acceptance | — | 16Q ap-2, 16O | S |
| an-render-modes | flat source-ordered render vs line-granular book-faithful render (comments converged lines in place) | plan output | an-fidelity-mode | 16P-T14, 16N | B |
| an-content-hash | binary content-hash identity ("grounded against jq@<hash>"); runner hashes `$PATH` at probe/apply, fail-safe on mismatch | version-drift + injection gate | an-version-coord | 102 dn-9, TODO versioning | S |
| an-version-coord | purl-style namespace/distro/version coordinate — comparable *intent* for hints/UX (distinct job from the hash) | MH2 version layer; UX | an-content-hash | TODO-ADDTL MH2, 102 | O |
| an-oracle-ref-sha | oracle pinned by commit-SHA, not a mutable tag (integrity-in-transit; no registry) | supply-chain (kTRUST seam) | — | 102 E4 | S |

## I · Cost, scheduling & the plan

| need | information | needed-by | dual | refs | st |
|---|---|---|---|---|---|
| an-cost-class | per-leaf apply-cost band: cheap-idempotent / expensive / dangerous-irreversible | probe-vs-just-run (kPROBING) | an-check-depth | 076§5, 083 Q-BAND, 16Q | D |
| an-check-depth | shallow (cheap guard fully captures the need) vs deep (hidden/daemon-mediated deps) | VALUE-band sizing; kDEPS split | an-cost-class | 083 Q-BAND/Q-ANTICORR | O |
| an-probe-vs-run-predicate | the kPROBING decision itself — elide-the-probe-and-just-run iff cheap-idempotent ∧ probe-cost ≥ work-saved; half static, half runtime; the decision-point must *exist* | probe construction | an-elision-predicate | KNOBS kPROBING, 076§5 | O |
| an-cost-vector | cost as a vector: (local-cost, reach-frequency, shared-resource-id) — not a scalar | cost-model; throttle; schedule | — | 076#7, 083 Q-COSTVEC | S |
| an-network-cost | per-leaf "opened AF_INET socket" fact (seccomp-BPF, scalar reg); keep-under-guard; the only big-O that bites is across the network | cost tier; undeclared-net backstop | an-undeclared-net | 077 seccomp, AGENTS perf | S |
| an-undeclared-net | "leaf X opened a network socket the oracle didn't declare" surfaced to the deployer (both phases) | oracle-quality backstop | an-network-cost | 077 backstop, 102-E3 | S |
| an-cost-inferred | infer cost-class from the check body (`curl`/`ssh`/`nc` ⇒ network) with no annotation | cost-model tier-2 | an-cost-profiled | 076§7, 083 Q-INFER | S |
| an-check-depth-inferred | infer depth (shallow/deep) from guard *shape* — a leading structurally-blessed guard / single read ⇒ shallow; no annotation | VALUE-band sizing; kBURDEN floor | an-cost-inferred | 083 Q-INFER/Q-BAND | S |
| an-cost-profiled | probe self-profiling (timing, exit-codes) harvested from realtime output (PGO/AutoFDO) | cost-model feedback | an-cost-inferred | 076#7, 074, 113 | S |
| an-leaf-timing-actual | a host-side clock-read per leaf emitted into the stream (per-leaf actual duration) — the one genuine per-leaf instrumentation cost; faithful-mode opt-in | estimate-vs-actual; PGO | an-estimate-vs-actual | 113-f61 | S |
| an-estimate-vs-actual | planned cost-estimate vs measured actual per leaf; the gap *is* the deopt signal | plan/apply EXPLAIN; PGO | — | 113-f57/f60 | S |
| an-urgency-intent | a coarse user "urgency/thoroughness" intent (deployer "fire NOW" ↔ engineer "careful") — the one dial the optimizer derives objective + probe-width + precision from | kOBJECTIVE/kPROBING defaults | — | 074, KNOBS kOBJECTIVE | S |
| an-schedule-dag | derived apply DAG; achievable parallelism = DAG width (computed, not authored) | apply scheduling | — | 076§4, 073 | S |
| an-critical-path | critical-path-first priority (Graham anomaly: more workers can ↑ makespan — naive scheduler is worse-than-serial) | schedule quality | — | 076§4, 073, 151-X2 | S |
| an-resource-contention | shared-resource → throttle groups (RCPSP); batch boundaries / canary / readiness-gate | resource-aware scheduling; rolling | — | 076§4, 064 | S |
| an-no-op-establish | an establisher that detects already-satisfied and is a behavioural no-op (idempotent `apt install` of current pkg) — feeds early-cutoff | early-cutoff; JUST-RUN classify | an-establish | 075, 083 Q-BAND | S |
| an-still-dirty-set | per-host residual mutations after probing — the narrowed plan | apply minimization; plan UI | — | 076§0, DESIGN apply | S |
| an-narrowed-plan | the rendered plan: state-mutators relevant to the goal, still as shell, dynamically updated as probes return | Terraform-style plan/approve UI | — | DESIGN approach-3, 064 | S |
| an-differential-reprobe | report only *changed* facts since last probe | plan/diff UX; resumability | an-forward-slice | 076§3d, 064 | S |
| an-realtime-stream | live structured per-leaf/host output (hard requirement); verdicts/errors never dropped, verbose progress droppable | debuggability; plan UX; cost-feedback | — | 064, 076#8 | S |
| an-result-rollup | per-host→controller fan-in roll-up (batched/streamed, never per-result ack); heterogeneous hosts never pre-averaged | fan-in aggregator | — | 113-f62/f63, 111 dac-D | S |

## J · Transport / on-wire (kCOMMS)

| need | information | needed-by | dual | refs | st |
|---|---|---|---|---|---|
| an-ssh-budget | SSH per channel = exactly 2 byte-streams + 1 int (stdout/stderr/exit-status), all owned by the wrapped tool; richer = the kCOMMS problem | transport design | — | 140-f1, 142 | S |
| an-status-lane | a reserved 4th status channel (FIFO *or* append-file *or* 2nd channel — NOT interchangeable, see an-append-corruption) drained by one reader; fds don't transit ssh (create remote-side) | live OOB gating | an-diag-sidefile | 141-g1, 140-f9/f14 | S |
| an-marker-protocol | nonce-prefixed colon-delimited status records (`keyword:field:…`, apt `pmstatus` shape); consumed by leaf-runner + provenance-parser + DST-synth + executor — *one shape, 4 seams* | the physical knot | — | 141-g8, 151-M3/X2 | O |
| an-marker-atomicity | status writes < PIPE_BUF (512 POSIX / 4096 Linux) to stay atomic across concurrent writers; only short records, not freeform | FIFO mux correctness | — | 141-g2, 150-R13 | S |
| an-append-corruption | regular-file `O_APPEND` is NOT FIFO-atomic; treating the two as one carrier is a latent concurrency-corruption bug — atomicity differs by carrier | status-lane carrier choice | — | 150-R13, 141-g2 | S |
| an-backpressure-decouple | the status-writer must not block on controller liveness — a full FIFO (64KiB) stalls the probe's own printf; file+append/tail decouples at a latency/artifact cost | probe-never-blocks-on-controller | an-status-lane | 141-oq2, 142, 140-f10 | O |
| an-end-sentinel | explicit end-of-stream sentinel (NOT FIFO-EOF); a backgrounded child inherits the reserved fd and hangs the reader | live drain correctness | — | 141-g5, 142 | S |
| an-diag-sidefile | large rich diagnostics as per-leaf files, single-writer, demuxed by filename (leaf-ids known a-priori) | rich-diagnostics channel | an-status-lane | 142 resolution | S |
| an-diag-file-backpressure | per-leaf diag files grow unbounded ⇒ size-caps + cleanup (the one thing an executor's flow-control buys) | rich-diag channel hygiene | — | 142 residual | S |
| an-no-pty | emitted exec must be no-pty (a pty merges stdout+stderr + injects control codes); invites block-buffering ⇒ `stdbuf -oL` discipline | clean stream split; livestream | — | 140-f5, 141-g4 | S |
| an-channel-budget | channels = batches (≤ MaxSessions); MaxStartups throttles connections; ControlMaster muxes N+1 over one auth; ProxyJump hop cost | transport topology/pacing | — | 140-f2/f3/f4, 072 | S |
| an-batch-channel-attribution | a channel = a *batch* of leaves run with internal `&`, NOT one-per-leaf; normal-mode within-batch interleaving is un-attributable (raw/tossed), only `--faithful` (one-leaf-per-batch) is per-leaf-clean | per-host topology; freeform attribution | an-fidelity-mode | 142 resolution, 140-f13 | S |
| an-controlmaster-absent | per-target "ssh multiplexing unavailable" (native Win32-OpenSSH lacks ControlMaster) ⇒ in-process connection-pooling, not OS-delegated reuse | transport pacing; platform precond | — | 139§4, 140 | S |
| an-host-key-trust | per-push host-identity verification (host-key verify, never blind-accept); pushing to a spoofed imposter is the failure | transport push gate (kAGENTLESS) | — | 102 E2/PM-5 | S |
| an-comms-pole | in-band transpiled markers (`kCOMMS-transpilation-inband`) vs bootstrapped executor reporting OOB (`kCOMMS-executor-OOB`); only live∧concurrent forces an executor | the kCOMMS knob | — | KNOBS kCOMMS, 140-f15, 142 | O |
| an-inband-spoof | in-band markers let attacker stdout forge control unless nonce-delimited; lane-separation prevents injection by construction | transport security | an-marker-protocol | 140-f-sec, 142 | S |

## K · Platform & precondition

| need | information | needed-by | dual | refs | st |
|---|---|---|---|---|---|
| an-target-sh-id | the target already evaluates POSIX sh — specific evaluator identity (dash / busybox-ash / bash) | platform precondition; tier | an-tier-ab | 139§3, 132 | S |
| an-target-sh-version | exact target-sh version/binary (same version-string ≠ same bytes; distro-backport) | oracle grounding; content-hash | — | 139§6, 128 qn-10 | S |
| an-tier-ab | per-target tier: A (full-oracle POSIX env) vs B (sh-syntax-only; degraded perm/path/dev oracles) | oracle applicability | — | 139§3 | S |
| an-busybox-corrupts-truth | a degraded target (busybox-w32 bogus perms/`/dev`) can corrupt the probe's *derived* truth, not just coverage → threatens kFAIL-withhold | tier-B soundness | — | 150-R14, 139 | O |
| an-crlf-hazard | detect CRLF in authored `.dorc.sh` (`\r` in shebang = exec failure, un-guardable in-script; corrupts compares/heredocs/`read`/`case`) | emitter; fail-fast | an-wire-transform | 134, 139§5 | S |
| an-wire-transform | the engine controls the wire and may transform bytes on ship (CRLF→LF) — and must *record* that a transform happened | faithful-mode fidelity | an-crlf-hazard | 134, 139§5 | S |
| an-bootstrap-cmd | ≤1 fixed inspectable native command to mechanize an sh-less target (scp-then-invoke-by-path; `sh -s` is Win32-buggy) | target bootstrap | — | 139§3 | S |
| an-parse-feasible | clean-parse vs hits-an-unsafe-boundary (which constructs, at what frequency) | language def; ⊤-boundary; go/no-go | — | 083 Q-PARSE, 021 | S |
| an-unsafe-boundary | hard ⊤: `eval` / dynamic command-name / `. "$dyn"` / heredoc-codegen / arithmetic-command-position / lvalue-builtins | ⊤-rejection (syntactic) | an-top-surface | 021§2, 055§1A, 16P-T2 | B |
| an-top-surface | the *real* ⊤-surface is dynamic *arguments* + command-substitution (the #2-frequency construct), not just dynamic command names | ⊤-rate sizing; substrate | — | 150 fN-ANALYZABILITY | O |

## L · Execution modes, scope & the three applies

| need | information | needed-by | dual | refs | st |
|---|---|---|---|---|---|
| an-apply-1 | full unconditional run — the trivial fallback floor (always available, never worse than not using Dorc) | the floor | — | 16P-T13, 090§0.5 | B |
| an-apply-2 | converge + safe-elide: probe the host, elide what's provably converged (forward-only) — the default | the headline mode | — | 16P-T13 | B |
| an-apply-3 | targeted desired-set (`dorc try`): apply-2 + a *backward* relevance-reduction; `apply-3 ⊃ apply-2` | hot-loop; kELISION | an-apply-2 | 16P-T13, 16Q q1-backward | D |
| an-elision-predicate | elide leaf iff `probe=Converged ∧ ambient ∧ Must ∧ no-consumed-unvouched-observable ∧ ¬⊤-contained`; `can't-probe ⇒ can't-elide` | apply-2 contract | an-probe-vs-run-predicate | 16P-T13 | B |
| an-backward-slice | backward slice from the dirty-set/skip-decision (drop commands whose effect can't reach it); interprocedurally needs *two-pass* realizable-path reachability, not plain closure | apply-3 minimization | an-forward-slice | 055§1B, 053, 16Q q1-backward | D |
| an-forward-slice | forward slice from a diff: "this role edit affects these facts on these hosts" | diff-time impact; `dorc update` | an-backward-slice | 055§Q2, 064 | S |
| an-elision-scope | user-mode scope: scoped (update/git-diff, accept staleness) vs full (reconcile-all); changes *scope*, never *soundness* | kELISION mode | — | KNOBS kELISION, 064 | W (mode) |
| an-objective | optimizer objective: latency (time-to-first-action) vs throughput (fleet makespan), from mode + urgency | kOBJECTIVE defaults | — | KNOBS kOBJECTIVE | S |
| an-retry-until | `retries`/`until`/`delay` as a first-class construct; the retry condition *is* a check | convergence-by-retry | — | 064, TODO-ADDTL language | S |
| an-kfail | phase-keyed safe-fail: probe withholds on ⊤, apply performs on ⊤ — opposite directions, never traded | the one welded invariant | an-verdict-phase-keyed | KNOBS kFAIL, 16P-T5 | W |
| an-host-as-adversary | a managed host can forge convergence verdicts flowing into elision → silent suppression of a needed apply | verdict trust; kFAIL-perform | an-verdict-failsafe-default | 150-R10, 151-SF3 | O |

## M · Engine substrate & structure (the `ananeed-3` inputs — representational needs)

| need | information | needed-by | dual | refs | st |
|---|---|---|---|---|---|
| an-cfg | parse sh → CFG; whole-program across `. /path` source-following + alias resolution (not per-file) | all analysis | — | 021§2, 16Q q1-interproc | B (intra-file) |
| an-build-then-solve-coupling | there is NO clean build-then-solve split — CFG construction must itself run a small dataflow pass (errexit edges materialized during build) | engine staging | — | 16P-T9, 021§2 | B |
| an-call-return-edges | intra-file function call/return edges (the supergraph); bodies currently detached (seeded errexit-Off) | interprocedural (un-⊤-ing functions) | — | 16P-§3.2, 16Q q1-interproc | D |
| an-dependence-graph | retained PDG/SDG (control + data + effect dependence) + value-flow edges, built once, queried many | slicing; impact; provenance; sparsity | — | 055-Q2, 053 | S |
| an-h-sparsity | the empirical "each statement touches a small slice of state" property — collapses IFDS O(E·D³)→~O(E·D); *why* the network-dominated big-O argument holds | substrate cost-justification | — | 052, 053 | S |
| an-summary-instantiate | per-role compositional summary (require/establish/may-mutate, parameterized) ↔ per-host instantiation against host facts | scale (analyze-once, H cheap instantiations) | an-summary-edge | 055-Q3, 16P-§3.3 | D |
| an-summary-edge | the on-the-fly "how value-after-call depends on value-before" edge — IFDS summary edges *are* the compositional procedure-summaries | interprocedural reuse | an-summary-instantiate | 052, 053 | D |
| an-two-pass-context | interprocedural slicing needs two-pass realizable-path reachability to respect calling context (naive closure over summary edges is imprecise) | apply-3 interproc slice | — | 053, 055§1B | D |
| an-distributive-split | which facts are IFDS-distributive gen/kill (poly-time summaries) vs a non-distributive effect layer (⊤-on-unknown) — get it wrong and the engine is imprecise *or non-terminating* | engine correctness + termination | — | 052, 055 dec-2 | O |
| an-ide-value-layer | IDE environment-transformers for *value-carrying* facts (`nginx@1.24`), above the IFDS boolean gen/kill layer — the second engine layer | version-aware elision; an-version-coord | — | 052, 055§1A, 055 dec-1 | O |
| an-substrate | hand-rolled monotone worklist (built) vs IFDS/IDE-demand vs Datalog/Soufflé-materialized — the `kFACTS` substrate decision (coupled to the why-tree query model) | scaling to the supergraph; provenance | — | 16Q dq-substrate, 055 dec-1, plans/170 | O |
| an-queryable-factbase | facts as relations + analyses as rules (Datalog) — yields queryable provenance ~free vs hand-built | extensibility; why-trees | an-demand-query | 055-Q2, plans/170 | O |
| an-graph-relations-duality | the graph view (slicing/sparsity) and the relational/Datalog view (extensibility/provenance) are two views of *one* substrate, kept in sync (edges ≅ relations) | substrate coherence | — | 055-Q2, 052 (IFDS≡Datalog) | O |
| an-demand-query | compute only the queried slice ("does host h need probing for fact F?") — the memory lever / query-planner framing | scale; memory wall | an-queryable-factbase | 055-Q3, 16Q | O |
| an-incremental | diff the CFG → affected-set = transitively-reachable-from-changed; clear + re-propagate only it (Salsa/Reviser) | diff-time `dorc try`/`update` | — | 055-Q3, 053 | S |
| an-context-key | context tag (per-host? per-role-invocation?) for k-CFA; default insensitive | precision dial (kCONTEXT) | an-flat-domain | 055 dec-4, KNOBS kCONTEXT | O |
| an-flat-domain | keep the abstract domain *flat* (no closure recombining across contexts) — the redline that dodges k-CFA EXPTIME | decidability; perf | an-context-key | 071, 083 Q-FLAT, KNOBS kCONTEXT | O |
| an-finite-domain | fact-set finite (bounded by the script's literal commands/paths) — the IFDS decidable floor | termination | — | 093-f20, 099§3 | S |
| an-analysis-unit | unit of analysis/skip/diff-recompute: fine (per-function) vs coarse; Dorc *derives* cross-unit deps | kUNIT; incremental scope | — | KNOBS kUNIT, 083 Q-MODULARITY | O |
| an-ir-schema | a lossless, serializable IR/verdict contract; analysis-plane ⊥ execution-plane (impl-language never reaches hosts) | component seam; zero-runtime-on-target | — | 041, 083§5, 076#1b | S |
| an-monotonicity | transfer monotonicity + finite-height + semantic `Eq` — un-type-enforceable; a violation *hangs* (empirically 435/783 CPU-s) → DST + iteration-cap + loud non-convergence | engine termination | — | 16P-T3/DP-2 | B |
| an-solver-direction | one worklist generic over Graph + Lattice + Direction{Forward, Backward}; only the dep-edge (succ/pred) changes | forward now, backward (apply-3) later | — | 16P-T3, 16Q q1-backward | B (fwd only) |
| an-async-vs-statemachine | the orchestrator kernel is async-native XOR a state-machine (deps-as-messages) — mutually-exclusive DST cost-dodges; pick one day-1 | DST seam; controller scale | — | 150-fM3b, 151-M3, 076#1 | O |
| an-di-seams | clock / randomness / network / disk reached only through DI primitives (mockable, fuzzable); correctness kernels stay dep-free | DST hermeticity | — | AGENTS DST, 128 L0 | S |

## N · Calibration / DST / verification

| need | information | needed-by | dual | refs | st |
|---|---|---|---|---|---|
| an-calibration-delta | observed actual command effect (container-fixture / eBPF) vs predicted — the differential calibration harness (replaces proof) | kVERIFY; cost/effect hints | an-differential-vs-shell | 055 dec-6, 086, KNOBS kVERIFY | S |
| an-differential-vs-shell | trust the un-provable parser + sh→IR translation by differential testing against dash/bash (a *different* harness/ground-truth than effect-calibration) | parser/IR correctness (no-Coq) | an-calibration-delta | 021§1/§5 | S |
| an-roundtrip-identity | parse∘pretty-print = identity (the lossless-IR invariant) as a property-tested obligation | an-ir-schema acceptance; parser trust | — | 021§5, 055 dec-6 | S |
| an-render-executability-check | the acceptance test must *execute or `sh -n`-check* the rendered artifact, never string-diff it (text-diff shipped a non-runnable `then`-clause green) | an-render-runnable acceptance; harness design | an-render-runnable | 16Q ap-2, 16P-T14 | S |
| an-withhold-monitor | DST detection of a probe attempting a *modeled* mutation — a recorded-and-refused violation (stand-in for the real sandbox) | probe-soundness testing | an-withhold-sandbox | 16P-T15/DP-4 | B |
| an-withhold-sandbox | real `kFAIL-withhold` enforcement (seccomp/sandbox) — the contract frame provably *cannot* enforce probe-inertness itself | probe non-mutation (runtime) | an-reflexive-inertness | 16P-DP-4, 102-E3 | S |
| an-replay-seed | a seed (+ commit) that deterministically reproduces a simulated run; the highest-value agent-feedback signal | DST; agent loop | — | 128 L0/L1, 16P-T15 | B |
| an-host-fault-model | the DST host-sim's injectable per-host fault space: unreachable / timeout / wedged-read-only / truncated-stream / forged-verdict | DST; an-host-as-adversary testing | an-probe-flakiness | IMPLEMENTATION, 128, 16P-T15 | D |
| an-sometimes-assert | assert a state (drop-after-mutate / Unknown path) *is* reachable — the coverage-reachability half (coverage itself stays unsolved) | DST quality | — | 126-f41, 128 fc-5 | S |
| an-probe-flakiness | model an unreliable oracle: probe returns Unknown with seeded probability / host transiently unreachable | unreliable-oracle DST | an-host-fault-model | 16P-T15/§3.2 | D |
| an-pure-kernel | the analysis pipeline is a total function (no clock/RNG/IO reachable) — lets the whole kernel run under DST with no DI ceremony | DST; reviewability | — | 16P-T1/DP-6 | B |

---

## Z · Design-discipline tags (carried *about* needs, not runtime engine-data)

Not engine facts, but the table/process must carry them — they decide how each row above gets resolved.

| need | information | needed-by | refs |
|---|---|---|---|
| an-exclusion-check | before excluding any edge/case as irrelevant, re-test under 4 cells: reverse-direction · other-phase · other-user · other-reliability; deferred ≠ irrelevant | every design decision | AGENTS exclusion-check, TODO |
| an-adversarial-target-rotation | a verification pass finds only what it's aimed at; rotate the target across core / harness / synthesis / charter-adherence, not just re-run on the core | verification process quality | 16Q ap-3, 16P-T17 |
| an-knob-vs-contract | per finding: a KNOB (A-vs-B tension we tune) vs a CONTRACT (author obligation we rely on for soundness) | design-space classification | 090§0.5 |
| an-weld-vs-adjust | per finding: weld-now (settle+bake) vs user-adjusts-but-we-design-the-mechanism | classification | 090§0.5 |
| an-lockin-tag | per finding: retrofit-hostile (decide the shape now) vs reversible | sequencing the decisions | KNOBS kLOCKIN, 083§5 |
