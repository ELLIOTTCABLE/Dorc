# 30Ka — Effective world reach: the as-built lane record

> Tier: r30 builder lane report for the `30K` conversion. `notes/30K` is the work order and
> stays ahistorical; THIS document is the as-built record — seats, deviations left OPEN,
> findings, next steps. Root docs and `spike/CLAUDE.md` outrank it. Grades: **[+SURE]**
> measured · **[~SUSPECT]** reasoned but unmeasured · **[-GUESS]**.
>
> Docid steer: this lane's report is `30Ka`, not a major docID (human, direct, mid-lane).

## §1 — `step-1-map-effective-invalidator-ownership`: the census

The scratch census the work order asks for, kept here because its OWNERSHIP half became a
built artifact (`cfg::ExecutionOwner`) rather than a throwaway list.

### 1a — Producers of the split answer, as found

| seat | what it answered | fate |
|---|---|---|
| `analysis::effect::Reach` (`Facts(BTreeSet<FactKey>) \| Top(ProvId)`) | per-cell "was my cell written upstream" | RETAINED, origin/probe only |
| `Reach::is_pristine` | the `QueryResolvable.valid` bit | retained for the FROZEN probe; retired as apply authority |
| `SkipClass::{EstablishAmbient,EstablishWritten}` | the elide/guard discriminator | retained as origin/probe classification; no longer selects the tier |
| `plan::wall_walk_total` | flag-off total wall, `Replace -> Run` | DELETED |
| `plan::wall_walk_survival` | flag-on scoped wall + `SurvivalWitness` mint + re-derivation | DELETED; its body re-seats in `plan::world::freshness` |
| `build_plan_walled`'s `is_mutator` side channel | `class_is_establish_bearing \|\| kills.contains` | DELETED |
| `cli::fixpoint::settle_validity_fixpoint` | the W-C erasure rounds | REPLACED by the one grow-only settlement |

### 1b — The effective-invalidator population, by shape, with its explicit owner

`invalidators` is every CFG node whose post-erasure effect vector contains
`Establishes`/`Kills`/`Opaque` (`effect::gens_into_reach`). Enumerated from
`effect::node_effects`, which is total over `CfgNodeKind`:

| invalidator shape | plan leaf? | execution owner | why |
|---|---|---|---|
| `Command`, ordinary leaf | yes | `Leaf(self)` | its own decision governs it |
| `Command`, expansion-internal (`$( … )` body) | no | `Leaf(enclosing simple)` | the enclosing leaf's span carries the substitution; a replaced leaf takes the substitution with it |
| `Command`, spliced-internal (per-call body copy) | no | `Leaf(call)` | the CALL is the render unit (`i-3`); an all-or-nothing call replacement neutralises the whole body |
| `Command`, detached funcdef body | no | `AlwaysAtNode` | unreachable from entry, so its gen reaches only its own island — exactly `Reach`'s own answer |
| `Command`, Members site | yes | `Leaf(self)` | one leaf, N member cells |
| `Redir` (write-shaped, under a `Simple`) | no | `Leaf(enclosing simple)` | the redirect is inside the leaf's byte span and is span-elided with it |
| `Redir` (group/subshell-level, `attach_redirs`) | no | `AlwaysAtNode` | no leaf span covers it, so no leaf decision can remove it |
| `Top` (unmodeled construct) | no | `AlwaysAtNode` | nothing decides it away |

Ownership is RECORDED at lowering (`cfg::Builder`), never re-derived from spans or
adjacency: `lower_simple` claims every node it allocates between its entry and its command
node; `splice_funcdef_body` overwrites its whole spliced range with the CALL, so a nested
splice resolves to the OUTERMOST call in one step and no transitive walk is needed. The
DEFAULT is `AlwaysAtNode` — the total-running floor — so a new node kind walls until someone
decides otherwise (`30K` §3.7: an ownerless invalidator never guesses).

## §2 — As-built seats

- `analysis::cfg::ExecutionOwner` + `Cfg::execution_owner` — the ownership census, built.
- `plan::world` — `WallId`, `ReachingWalls` (`Powerset<WallId>`), `EffectiveWallPolicy`,
  `Freshness`, `EffectiveAct`, `NoMutationProof`, `ReplacementDeathProof`,
  `NoExecutionLedger`, `Quiescence`, the certified reach solve, and the freshness rule.
- `plan::settle` — `ProvisionalEffectiveRound` (no Spine API), `SettledEffectiveAnalysis`
  (private constructor, takes a `Quiescence` witness), `settle_effective_world` (the one
  grow-only loop), and `write_spine`.
- `plan::build_plan_walled` — now a thin driver over one settlement with a constant
  classification callback; `cli::fixpoint` supplies the reclassifying one.

## §3 — Deviations from `30K`, left OPEN for the conductor

Recorded as taken, never self-endorsed.

- `dev-replacement-death-does-not-erase-effects` — `30K` §3.5 asks for ONE `ErasedSites`
  overlay carrying both proof species at the analysis effect seam. Built otherwise: the
  DeadBranch species alone reaches that seam; the Replaced species suppresses the site's
  wall GEN and nothing else. Reason: the effect seam spells erasure as
  `CommandEffect::Pure`, which also destroys the site's own `SkipClass` — so a replaced site
  would classify `MustRun` on the next round, lose the license that replaced it, and settle
  as `Run` while every downstream decision had already been taken as if it would not execute.
  That is a wrong-elision, not a precision loss. Both species still combine into ONE
  no-execution ledger and one gen-suppression rule; the divergence is only in which
  consumer each species reaches.
- `dev-policy-borrows-round-derived-backings` — `EffectiveWallPolicy::RiskAccepted` carries
  `fact_backings` as `30K` §3.3 requires, but the policy is constructed PER ROUND (it
  borrows) because backings are derived from the residual model, not frozen.
- `dev-aggregates-take-the-call-node-freshness` — `InlineCall`/`EstablishMembers` consume the
  reaching walls at the aggregate's own node (Members through a self-suppressed re-solve,
  the existing `self_reach` shape), and keep every one of today's per-body-site conditions
  unchanged. `30K` §5.4's universal per-erased-establish effective freshness is NOT built;
  the aggregate takes the conservative single position instead.

## §4 — Findings

(see §6 for the enumerated behaviour drift)

## §5 — Next steps / residue

## §6 — Behaviour drift, enumerated
