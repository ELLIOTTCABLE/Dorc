# Lean formalization of the sparing algebra — spike report

AI-authored (solo builder, 2026-08-13). Formalizes `notes/277` **as written**
(§1 coordinate · §2 ternary compare + consumer map · §3 selector dialects ·
§5 set-lifting/universal meet) plus `notes/272` §4 (never-derive-separation),
against Lean 4 v4.33.0, core only (no mathlib, no batteries).

- Base: `ai/main` @ `9b05283f` (see tc-base-branch-missing below). Branch:
  `ai/research-lean-sparing-spike`.
- Status: **CHECKED** — `lake build` green; **zero `sorry`**; every theorem's
  axiom profile is exactly `[propext, Quot.sound]` (no `sorryAx`, no
  `Classical.choice`), verified via `#print axioms` over every law.

## (a) Per-theorem ledger

All five briefed theorem families **proved** (none landed as sorry). Names as
in `SparingAlgebra/Laws.lean`:

| # | brief name | landed as | status |
|---|---|---|---|
| 1 | `sparing_requires_every_pair_disjoint` | itself, plus `sparesSet_iff_universal` (fold ⇔ ∀-form, both directions), `one_unknown_member_collides`, `no_sparing_without_flag` | proved |
| 1g | guard-invariants, structural | `BackingSet` (`ownCell :: rest`; ∅ unrepresentable) — `backing_set_members_nonempty`, `own_cell_always_member`; ⊤ as explicit `Member.wall` — `wall_member_collides` | proved (structural + theorems) |
| 1h | vacuous-spare hazard (unguarded encoding) | `vacuous_spare_over_empty_backing` over `sparesRaw` (raw-list encoding kept solely as the cautionary target) | proved |
| 2 | `set_meet_order_independent` | itself (member permutation), `footprint_order_independent`, `unknown_member_collides_under_any_order` (the pin: an unknown member defeats every order), on a self-contained `all_perm` | proved |
| 3 | `consumer_map_safety_inversion` | bundle of `sound_transport_never_misfires`, `sound_sparing_never_misfires`, `unknown_licenses_neither`, `unknown_uniquely_idle`, `safety_inversion_witness` (a single sound world where each binary reading of `unknown` misfires exactly one consumer) | proved |
| 4 | `no_outcome_as_generator` | **API-shape, not a theorem** (as the brief anticipated): evidence types (`EntityCmp`→`EntityRes`, `Dialects`) are Verdict-free by construction; `EntityRes` is deliberately a distinct-but-isomorphic type; a `fail_if_success` compile-time tripwire breaks the build if a `Coe Verdict EntityRes` ever appears. Lean cannot forbid a client *writing* an explicit conversion — the guarantee is non-silence/greppability, not unrepresentability. | structural (statement-uncertain by nature; flagged) |
| 5 | `cross_family_monotone` | itself + `cross_family_monotone_member`; structurally rooted (`selectorTier` receives only the backing family's `DialectRow`, so foreign rows are invisible *by type*). Qualified form only, per `279f` (the absolute form is falsified by declared within-family kill-surface control — not re-stated). | proved |
| + | riders | `top_never_spares` (both sides, the `279f:fix-spare-top-backing` pin), `top_never_same`, `empty_world_no_selector_sparing` (rung-0: no rows ⇒ no manufactured disjointness), `no_self_licensing` (modulo a resolver-honesty side-condition), `vacuous_spare_over_empty_footprint` + `pre_amendment_spares_top_backing` (hazard exhibits, see gaps) | proved |

Statement-uncertainty flags (proofs are machine-checked; whether the *statement*
matches spec intent is the residual risk): #4 as noted;
`SoundCompare`'s framing (see modeling-note-conditional-soundness);
`Member.wall`'s granularity (see gap-top-member-granularity).

## (b) SPEC-GAPS

Formalization pressure found real underdetermination. Ordered by judged
severity.

### gap-compare-symmetry-vs-directionality (+SURE the shape divergence exists; ~SUSPECT about intended resolution)

`277` §2 spells the relation symmetric — "compare(cellA@ctxA, cellB@ctxB) ∈
{ same(coord) | provably-disjoint | unknown }" — but §3's sparing generator is
irreducibly directional: "claim-token ∈ dialect(the backing's **minting
family**, kind)". Only backings *have* a minting family (claim/disturbs
emissions never mint), so in the survival use the two arguments have different
**types** (`Coord` vs `Backing`), and no symmetric signature can express the
tier. Chose: build `compare : … → Coord → Backing → Verdict` directional and
say so. The "one chokepoint, everywhere" spine and the §3 algebra are not
inconsistent, but the spec never acknowledges that its one relation has
position-typed arguments. — This intersects the confirmed open gap
`28R:adj-sparing-two-position-rule` (which position's environment governs a
frame-relative claim-vs-backing comparison): the same two-position asymmetry
this formalization hit at the *type* level resurfaces there at the
*environment* level, which reads as convergent evidence the asymmetry is
load-bearing, not an artifact.

### gap-top-selector-self-sameness (~SUSPECT)

`277` §1: "The bare selector-less form permanently means \"true / occupied /
whole-entity\"" — a *definite* cell name. `277` §6: "silence never identifies;
**⊤ identifies with nothing** (including itself)". §5: a ⊤ member "blocks all
transport". These pull apart: the definite whole-entity name and the
failed/unknown-⊤ derivation share one encoding (selector-less), so the
chokepoint cannot distinguish "the whole-entity cell, twice" from "unknown
depth, twice". Chose: `top`/`top` is `unknown` — never `same` — even at
identical kind/entity/context (conservative; the §6 fence wins). Cost: two
byte-identical whole-entity coordinates do not compare `same` at the
chokepoint, so any future consumer wanting whole-entity fact transport through
`compare` loses it. The spec never says which reading is intended, and the
generator registry's token-equality row does not say whether it covers the
selector position.

### gap-top-member-granularity (~SUSPECT)

`277` §5: a failed derivation "yields an explicit ⊤ member (**collides with
every footprint**, blocks all transport)" — unqualified. But a selector-⊤ under
a *known* kind does NOT collide with every footprint: the kind-fence
short-circuits cross-kind pairs to provably-disjoint, so a cross-kind footprint
would be spared *against the failure marker*. If a backing derivation can fail
hard enough that the kind itself is unknown (plausible in the
`27D:limitation-backing-sets-same-kind-only` widening the invariant names as
its binding target), encoding the failure as `(kind, entity, ⊤-selector)`
under-collides. Chose: model BOTH — `Selector.top` (entity-scoped ⊤) and a
member-level `Member.wall` (collides with everything, `unknown` against every
footprint) — and flag that the spec does not name the wall representation. The
engine should decide explicitly which failures are entity-scoped and which are
full walls.

### gap-footprint-empty-set-unpinned (~SUSPECT; checkable exhibit in-tree)

`277` §5 pins the ∅-hazard one-sided: "(inv-backing-set-non-empty) a fact's
**backing-set** is non-empty BY CONSTRUCTION … (inv-top-never-empty-set)
⊤/unknown is never ENCODED as ∅". But the meet quantifies over
footprint×backing *pairs*, and universal-over-∅ is vacuous on the FOOTPRINT
side too: `vacuous_spare_over_empty_footprint` proves the GUARDED encoding
still spares everything when the footprint list is empty (flag on). This is
legitimate iff an empty footprint can only ever arise from an *authored* empty
at-most claim ("disturbs at most: nothing") and never as an encoding of
unknown/unmodeled disturbance (silence must stay a wall). That invariant —
footprint-side ⊤-never-∅ — is stated nowhere in `277` §5. It is probably
engine-true today (silence-licenses-nothing routes unmodeled commands away from
claims entirely), but the spec's own reasoning ("silence-as-license reached
through an *absent* claim") applies verbatim to this side and should be pinned
symmetrically.

### gap-entity-disjoint-generator-missing (~SUSPECT)

§2's generator registry has no row producing cross-entity disjointness. The
token-equality row generates only `same`; the selector-dialect row generates
only "(same entity, cross-cell)" disjoint; yet §3 asserts "Cross-entity/kind
disjointness unchanged" — i.e. the entity-granular baseline exists and spares.
Within-kind entity-inequality⇒disjoint is a *derivation of separation from
name-inequality*, exactly the move `272` §4 forbids for context values —
presumably licensed here because `kind__resolve()`'s canonicalization is a
kind-owner *vouch* that distinct canonical entities are distinct referents
(with `MayAlias` as the escape). Chose: an abstract `EntityRes.distinct`
verdict from the resolver, treated as vouch-tier. The registry should carry
this row explicitly; as written, the most-exercised disjointness source in the
system is un-generated.

### gap-both-sides-minted-redundancy (~SUSPECT)

§3: "a claim SPARES a backing iff BOTH sides carry minted selectors AND
claim-token ∈ dialect(the backing's minting family, kind) AND claim-token ≠
backing-token." The claim-side "minted" conjunct is subsumed by the dialect
membership test (any token in dialect(backing family, kind) *is* minted); the
backing-side conjunct is true by construction (backings come from minting
lines). So the phrase does independent work only in excluding ⊤/selector-less
forms — unless it intends something stronger (e.g. the claim token must also be
minted *somewhere*, or in the claim-utterer's own family — which would change
the algebra). Chose: the subsuming reading, with the backing-side membership
checked explicitly anyway (`row k c && row k b`) so the formal algebra does not
silently depend on the engine invariant.

### gap-same-verdict-payload-unspecified (-GUESS; minor)

§2 writes `same(coord)` — a payload-carrying verdict — but never says which
coordinate the payload is (the canonical? one side's?) or who consumes it.
Chose: payload-free `Verdict.same`. If the payload is load-bearing (e.g. the
transport consumer needs the canonical name), the formalization misses it.

### modeling-note-conditional-soundness (not a gap; a framing the spec should keep explicit)

§3's fences admit "physically-overlapping cells within one dialect are inherent
to narrowing" — so the dialect generator is NOT sound against a physical-
overlap world; it is sound against the family's *declared* cell structure.
`SoundCompare` in `Laws.lean` must therefore be read as conditional soundness:
IF every generator's vouch is true of the world, THEN consumers never misfire —
i.e. the machine-checked content is "misfires trace to generator-incorrectness
(the attributed knife) or contradiction, never to the composition algebra",
which matches §2's bite-class list. The algebra cannot repair a lying
`kind__resolve` (hence `no_self_licensing`'s explicit resolver-honesty
side-condition).

### note-flag-scope-resolved (~SUSPECT)

Briefly unclear whether *entity-granular* kill-misses (Package:vim's disturbs
vs a Package:nginx fact) are flag-gated — §2's consumer map gates ALL
provably-disjoint consumption behind the survive flag. Resolved by
`271:rul-flag-is-razor-residue`: every sparing verdict rests on an at-most
claim's open-world residue ("and nothing else"), which is exactly what the flag
owns; so yes, all claim-derived sparing is flagged, and the model gates
`sparesSet` wholesale. Not verified against engine HEAD behavior (out of
scope); flagged in case HEAD spares entity-granular misses unflagged.

## Known-unmodeled amendments (deliberately out of scope)

Conductor-confirmed: `notes/277` is textually current but carries post-hoc
amendments from a newer design round not present in its text. This
formalization is OF 277-AS-WRITTEN — the pre-frame-relative algebra. Named as
out of scope, cited by name only (not read):

1. `28R:rul-instantiation-hash-dedup` — identity redefined as bytes ×
   resolved-call identities, SCC-condensed (277 has no dedup concept).
2. `28R:rul-mixed-custody-suspends-vouch` — vouch attaches only when every
   reached definition lies in the vouching author's closure (new license-plane
   law, no 277 referent).
3. `28R:adj-sparing-two-position-rule` — CONFIRMED open gap: which position's
   environment governs a frame-relative claim-vs-backing comparison (unruled).
   Intersects gap-compare-symmetry-vs-directionality above, as noted there.

## (c) Toolchain experience

Total toolchain wall-clock ≈ 35 minutes (inside the 45-minute box), Windows-
native throughout; WSL never needed. The working, committed spelling:

```toml
[tools]
"ubi:leanprover/elan" = { version = "4.2.3", exe = "elan-init" }
[env]
ELAN_HOME = "{{xdg_data_home}}/elan"
_.path = ["{{xdg_data_home}}/elan/bin"]
```

plus the ecosystem-standard `lean-toolchain` file (`leanprover/lean4:v4.33.0`)
in the spike dir; elan fetches the pinned toolchain on demand; `mise exec --
lake build` from the spike dir builds.

What happened, in order:

- `mise registry` has NO lean/lean4/elan entry at all. `mise-plugins/mise-lean`
  exists but is a legacy asdf plugin (Linux/macOS only, Lean-3 era) — dead end
  on native Windows.
- Direct pin `ubi:leanprover/lean4[extract_all=true]` (≈3 min download,
  4.33.0): install succeeds, layout is correct (`bin/` with lean.exe, lake.exe,
  bundled clang), but every `mise exec` dies "cannot find binary path" — ubi's
  bin detection did not accept the extracted tree, even with `exe = "lean"`
  and a clean reinstall (it "checksum generates" against an odd asset-derived
  name instead). ≈15 min burned; abandoned rather than fought. NB mise also
  warns `ubi` is deprecated wholesale in favor of a `github:` backend
  (removal slated 2027.1.0) — `github:leanprover/lean4` might fix the
  detection, untested here (tc-ubi-backend-deprecated below).
- Human steer: find the elan-shaped approach under mise. A GitHub-wide search
  found exactly one live exemplar (`ubi:leanprover/elan` + `exe = "elan-init"`
  + `~/.elan/bin` on PATH). Adapted with two changes: `ELAN_HOME` templated to
  `{{xdg_data_home}}/elan` (resolves cross-platform; lands in
  `AppData\Local\elan` on Windows — user-local state, outside the repo/Sync
  tree, where ~1GB of toolchain churn belongs), and a one-time bootstrap
  `mise exec -- elan-init -y --no-modify-path --default-toolchain none`
  (≈30 s). The bootstrap is the one manual step a fresh machine needs; it did
  not merit a mise task yet — promote to one if the config merges.
- `elan toolchain install` of the pin: ≈6 min. First `lake build`: immediate,
  ≈0.5 s/module.
- Windows-native paper cuts that did NOT bite: long paths (the deepest
  toolchain paths stayed under the 260 limit), lake needing external cc (the
  release bundles clang), Git Bash vs PowerShell (everything ran through
  `mise exec` in Git Bash).

## (d) LLM-Lean viability observations

Strongly positive for THIS project's theorem shapes, with one caveat.

- Shapes: the sparing algebra is finite enums + Bool folds + small structural
  inductions — the best possible terrain for LLM-authored Lean. No analysis, no
  arithmetic, no dependent-type cleverness needed. Everything proved with
  `simp`, `cases`, `split`, one hand-rolled permutation induction, and one
  `decide` on a closed term.
- Volume: ~25 declarations; the FIRST compile had 2 errors (a `cases p x` that
  doesn't rewrite applications — fixed with `Bool.and_left_comm`; an `if_pos`
  rewrite defeated by simp having already normalized the condition to `True` —
  fixed with `if_true`). Second compile: green. Two subsequent additions
  compiled first-try. Total compiler round-trips: 3.
- Core-only was the right call: v4.33.0 core has `List.Perm` (with the
  nil/cons/swap/trans induction) and `List.all_eq_true`; mathlib would have
  added minutes-to-hours of build for zero benefit here. The defensive style —
  preferring self-contained inductions over recalled lemma names — is what kept
  the error count at 2; blind library-name recall is the LLM failure mode, and
  both failures were exactly the two places tactic behavior (not statement
  content) was guessed.
- The proof-checker loop is the point: both failures were caught mechanically
  in seconds and localized to exact goals — authoring the same content as
  prose-math would have left ~25 unchecked proofs with an unknowable silent-
  defect rate in the tactics layer, and no signal about WHERE. Conversely: the
  checker validates proofs, not STATEMENTS — the (b)-section gaps were all
  found while choosing what to state, before any checking, and no green build
  defends against formalizing the wrong sentence. Statement review by
  spec-owners is where human attention should go.
- The structural-encoding trick (making invariants unrepresentable — non-empty
  `BackingSet`, row-typed `selectorTier`, Verdict-free evidence) bought more
  assurance per line than the theorems did, and translates directly to the
  Rust implementation's type shapes.

## (e) Tip hash and judgment calls (flagged, not silently resolved)

- Base: `ai/main` @ **`9b05283f`** ("Bank the verification-tooling research
  round"). Branch `ai/research-lean-sparing-spike` (renamed from an
  interim round-numbered name on human instruction mid-session).
- **tc-base-branch-missing** — the briefed base branch did not exist locally
  (renamed by the human mid-session, per conductor). Initially improvised base
  = the worktree's checked-out tip `1ec5779f`; re-based onto `9b05283f` on the
  conductor's correction before any commits. No content was lost (one in-flight
  `mise.toml` edit was redone by hand).
- **tc-exp-label-not-in-gitlabels** — the brief prescribed `(AI exp)` commits,
  but `exp` is not a `.gitlabels` label (the commit-msg gate would warn-allow
  it). Human clarified mid-session that the label was an example and minting is
  my call: used in-vocabulary `(AI conf dep)` / `(AI new dsn)` / `(AI dsn)`
  instead; no new label minted.
- **tc-elan-home-outside-worktree** — `ELAN_HOME={{xdg_data_home}}/elan` is
  user-local state OUTSIDE the worktree, deviating from the original brief's
  worktree containment; judged sanctioned by the superseding mise steer (mise's
  own installs live in the same AppData region) and by keeping ~1GB of churn
  out of the SyncThing-adjacent tree. Flagging rather than assuming.
- **tc-ubi-backend-deprecated** — the committed config uses the `ubi:` backend
  that mise deprecates in favor of `github:`; `ubi:` is what was verified
  working, and the `github:` respelling (which may also fix the direct-lean4
  route) is untested. If this config merges, consider re-testing under
  `github:leanprover/elan` first.
- **tc-lean-pin-rides-lean-toolchain** — the spike pins Lean via
  `lean-toolchain` (ecosystem-standard, elan-read) while mise pins only elan
  itself; a machine with a different elan already on PATH ahead of mise's would
  still resolve the right Lean (the file governs) but would bypass the
  mise-pinned elan version. Accepted; noted for the merge decision.
