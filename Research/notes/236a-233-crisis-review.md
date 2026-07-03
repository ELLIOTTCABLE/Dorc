# 236a — external review of the 233 crisis + ternary-verdict recovery

> AI-authored review note (clean-context reviewer, 2026-07-02), commissioned to assess
> `plans/233-rubber-ducking-the-oracle-contract.md` against the project's own goals/metrics.
> Reading base: R/D/I/K/AGENTS + ANALYZER-NEEDS headers, `plans/{17N,19H,20K,21W,22W,230}`,
> `notes/{19B,19E,19G,218a}`, TODO/TODO-ADDTL, spot-verification in `spike/crates/analysis/src/effect.rs`
> and `notes/093`. Deliberately did NOT read other 23x-series notes (per brief; clean slate).
> Confidence marks per house convention. Process evidence, never proof; trust R/D/I/K over this.
>
> Verdict up front: **the crisis is real and correctly diagnosed; the ternary recovery is sound,
> and is ~SUSPECT the only remaining move in the design-space; the document if anything
> UNDER-claims what the recovery retains** (236a-fd1). Findings below are redirects and
> reconnections, not kill-shots.

---

## §1 Verification of the crisis (233 §0–§4)

- **236a-v1 · the §0 unsoundness is real, and mechanically as described.** +SURE.
  `effect.rs` module doc + `command_effect` (verified this session): a modeled command
  resolves to exactly its declared `Establishes/Kills/Queries` cells; the reaching-defs
  ambient gate gens only those cells; un-mentioned cells stay ambient downstream. Opaque ⇒
  ⊤-join-all (safe). So the baseline is "safe floor, dangerous middle," exactly as 233's
  2026-07-01 correction paragraph states. A clean priority-1 construction: probe-time
  `fs.Path:X#exists` holds (stale artifact); book runs `apt-get purge nginx` (oracle declares
  only the package cell) then `[ -f X ] || restore`; the query-substituted rc-0 elides the
  restore; purge deletes X at apply. Under-execute, the worst class.

- **236a-v2 · both horns are genuine; the dilemma is fundamental, not a design accident.** +SURE.
  hard-1 (mutation un-analyzable from sh) is Rice + the welded referent-agnostic stance;
  hard-2 (oracles rot unattended) is uncontroversial. The "enumerate-the-world" horn (233 §1)
  is the frame problem: non-effects are unenumerable. The corpus's own round-9 wall-map
  already said so — `notes/093` f21 [B-sep-frame-problem-2004]. No cleverness escapes this;
  the only known industry escapes are (a) *own the write-path* (RDBMS/Terraform-state),
  (b) *enforce/observe the footprint* (Bazel sandbox; redo/tup tracing), (c) *re-check
  in-sequence at execution time* (make, Ansible modules). Dorc constitutionally lacks (a);
  (b) fails on remote heterogeneous no-privilege hosts as a *default* (the `078` analysis:
  daemon-mediated ops break per-process tracing); (c) is the 233 proposal. The state-space
  is genuinely this small; 233's "I don't see what else to do" is correct, not resignation.

- **236a-v3 · BUT the crisis is older than 233 frames it, and the corpus self-contradiction
  should be written down.** ~SUSPECT this matters for weld-hygiene. 233 §1 says "the majority
  of our research-corpus assumed [poison-if-undeclared]." Half true: the corpus held BOTH
  postures simultaneously since round 9. `093` f21 explicitly *blesses the closed-world frame
  axiom* — "assume-unchanged-unless-declared … the unsound-but-necessary core of the whole
  skip-thesis … the oracle declares the small footprint; Dorc assumes the vast frame" — i.e.
  the §3/§4 "dangerous middle," named as load-bearing, in round 9, with a SEP citation. The
  ⊤⇒run / "unknown ⇒ run" rhetoric everywhere else is the opposite posture. The two were
  never squared against each other or against priority-1 (never under-execute); the spike
  implemented f21's posture at the modeled level and poison at the unmodeled level; and
  `1AA` head-3 ("refusals are sound but not free… worth a declared-effect-no-probe cell so a
  refusal can stop poisoning") was actively steering FURTHER into trust-silence when 233
  caught it. 233 does not cite `093`/f21 at all — it re-derives the frame problem from
  scratch (arriving at the same "fundamental" verdict, which is corroboration). Recommend:
  a short archaeology paragraph in the eventual synthesis, citing f21, recording that the
  round-9 closed-world weld is hereby *revoked for the elide tier* — this is fM3-ACCRETION's
  mirror image (an early unsound call that never got overturned), and leaving it uncited
  invites some future round re-discovering f21 and "restoring" it.

- **236a-v4 · the wish-list is the right test-set, and the ternary passes it.** +SURE on the
  mechanics (traced): wish-A holds (silence ⇒ poison for elide; guards re-measure live);
  wish-B holds *better than before* (a lone local-claims oracle buys guard-tier at its own
  sites, zero coordination — the pre-crisis design gave a lone oracle nothing until the
  cross-oracle vocabulary existed); wish-C/D hold (guards need no cross-oracle agreement at
  all — "vocabulary necessary only to be fast, never correct" is verified: entity-resolution
  for a guard is single-oracle); wish-E partially (see 236a-fd4 — the accrual incentive
  weakens); wish-F untouched. Monotonicity claim verified: a partial oracle's declared
  establishes only ever *block* downstream elision (written ⇒ non-ambient ⇒ refuse), never
  license it; the vouch is fenced to own-sites; so no path exists by which adding a bad
  partial oracle endangers another site. The §3/§4 landmine (silence-as-vouch) is gone.

---

## §2 Verification of the recovery mechanics (the 2026-07-01 update)

- **236a-v5 · "in-sequence ⇒ frame-free" is correct**, and it has two under-claimed corollaries:
  - **TOCTOU inversion.** rul-toctou (probe→apply staleness, deferred-to-WONTFIX, 20K §2)
    becomes *moot at guarded sites* — the guard re-measures at execution time; `218a` d4-3
    world-2 already proved this ("strictly dominates door-2-static under TOCTOU drift").
    On the correctness axis, guard beats elide; it is not merely elide's consolation prize.
  - **The errexit-canary narrowing** (`218a` world-3): probe-visible sicknesses fall through
    to the real mutator. Both belong in the proposal's Upsides list; neither is there.
- **236a-v6 · the converged-vouch license is the strongest part of the design.** +SURE. The
  same-day refutation of the universally-quantified license ("re-running does *nothing*" —
  vacuous over unattended observables) is exactly right, and notably it retroactively
  corrects `218a` d2-1's own claim-noop wording ("would mutate nothing"), which carried that
  quantifier. The replacement — attributable fallible judgment, claimed-tier, disclosed —
  is precisely the semantics the dominant prior art already runs on: an Ansible module
  author *judges* `state=present` converged for an installed-but-outdated package (upgrade
  ⇒ `state=latest`, a different declared intent). `218a` hunt-A (apt-get upgrades outdated
  packages; "converged ⇒ no-op is false as naively probed") is thereby *answered*, not
  dodged: the oracle author makes the same call the ansible-apt author made, and Dorc
  attributes it. The vouch-on-a-path mechanization ("the engine has no notion of a verb")
  and the own-sites-only fence (never enters the fact-plane, cannot soften poison) are the
  two load-bearing invariants; both are stated. The fence is what prevents rebuilding §0
  "one storey up" — flag ANY future feature that wants to read a vouch cross-site.
- **236a-v7 · weld-accounting is honest.** The update names its posture shifts (apply-lane
  executes unspelled reads; `inv-probe-sourced-values` carve-out; door-4 ruling reversal
  to be re-welded consciously, not drifted into). Cross-checked against `spike/CLAUDE.md`
  inv-list via 22W §8: no *unnamed* weld is breached. ru-11 untouched (the vouch is a
  license-plane witness, not a receipt). kFAIL-perform's trust-shape is unchanged from the
  pre-crisis design (an oracle's check-verdict already licensed Replace(converged); the
  guard moves the same trust to a fresher read).

---

## §3 Findings — redirects, reconnections, gaps (ranked)

- **236a-fd1 (headline) · the attention-product is more recoverable than the document
  concedes — via the project's OWN two-plane doctrine.** ~SUSPECT, load-bearing for r23
  effort-allocation. 233 frames the trade as "give up the attention-product to save the
  performance-product," defining attention = artifact-elision, and grades the alternative
  display-compression ("expected: 1 change, 96 no-op") as "objectively of less value" because
  it is claims-not-proof. But r22 already split the surfaces (ru-12/ru-20/rec-1): the
  *artifact* is byte-floored; the *plan-render* is the sanctioned overlay surface. A folded
  plan-render over guard-tier facts ("3 changes · 96 converged, guarded — fold/expand") IS
  the attention-product for the review workflow — and, crucially, **the guard underneath
  changes the epistemics of the fold**: a wrong fold-claim is no longer a correctness error
  (the elide-tier's failure mode) but a display surprise that the in-sequence guard catches
  at apply. The claims-vs-proof value-gap the document leans on largely evaporates *because
  of its own mechanism* — "verified-at-apply" folding is arguably more trustworthy than
  "statically proven from oracle testimony" folding, and it is exactly the `ok: 96 / changed:
  1` summary Ansible admins already demonstrably trust. What true elision still uniquely
  buys: a short *editable* artifact (the plan-is-a-script-you-edit aesthetic), removal of the
  check-tax, and future apply-reordering freedom. Those are real but narrow. Recommend
  re-basing the attention-product on the render (a fold keyed on guard-sites × probe-claims,
  zero new trust) and re-grading artifact-elision — and with it the whole ACK/completeness-
  vouch tier and the bottom open fork — from product-critical to optimization. The honest
  product statement then improves from "elsewhere … fast and safe, but not shorter" to
  "the plan view is always short; the artifact is only shorter where the world is described."

- **236a-fd2 · the guard's payoff population is anti-correlated with guardability — measure
  before building the thin pole.** ~SUSPECT. The command classes where a guard saves the
  most are skewed toward exactly the classes 233 excludes or kVOLATILES forbids:
  refresh/freshness semantics (`apt-get update`, `docker pull`, `git clone/pull`), run-delta
  restarts, consumed-stdout sites. The reliably-guardable population (package install,
  user/group, enable, file-exists) saves ~0.05–3s per site. That is real (and aggregates),
  but the "retained enough value" claim is currently a hope, and the r21 dashboard
  infrastructure (21B; the four-cause decomposition, the 172-site denominator) exists to make
  it a measurement: re-derive the headline under the ternary — guardable-site count ×
  per-site saved wallclock on the measuring-stick book, seeds-only vs enriched. Do this
  before any investment in apply-guard-thin render machinery (which 233 already correctly
  demotes to "must not drive the design"). Also note the check-tax is bounded below by the
  probe-vs-just-run banding (kPROBING) — a guard should be subject to the same banding as a
  probe, i.e. cheap-command sites should refuse guards and just run (233 gestures at this;
  make it explicit in the license so `mkdir -p` never gets a guard costlier than itself).

- **236a-fd3 · the elide-tier's realistic habitat is the curated stdlib as a mutually-vouched
  closed world — name it, or the tier is dead weight.** ~SUSPECT. Under the sound elide
  license, site N elides iff every interposed command's oracle ACKs N's cells. Across
  independent community authors this requires ACKing types the author has never heard of
  (233 §1's "new type enters the universe"), which is coordination-shaped and will
  approximately never happen organically (DefinitelyTyped worked because types buy a lot;
  an ACK buys a marginal guard→elide upgrade — see 236a-fd4). But within the
  `effort-allocation` bootstrap set (~40–50 curated oracles, one community, bounded cell
  vocabulary) the m×n ACK matrix is a maintainable engineering artifact. That yields a crisp,
  user-legible product boundary: books staying inside stdlib vocabulary get the full
  attention-product (short artifact); one exotic command drops the tail to guards. 233
  treats completeness-vouching as a generic community mechanism; scoping it to the stdlib
  is what makes it real. (fs.Path stays the honest exception — nearly nothing can truthfully
  ACK it, so `[ -f ]`-dependent sites live at guard-tier; that falls out emergently and
  matches ops intuition.)

- **236a-fd4 · wish-E (accrual) is weakened by the proposal and the document doesn't face
  it.** -GUESS on magnitude. Pre-crisis, completeness was the entry-fee (unaffordable, hence
  the crisis); post-ternary it is an opt-in upgrade whose marginal value (guard→elide ≈
  artifact brevity + ~50ms/site) may be below any author's effort threshold — especially if
  236a-fd1's render-fold delivers the attention value without it. The likely ecosystem
  equilibrium is local-claims-only oracles, permanently. That is an acceptable equilibrium
  (guards are the proven Ansible-grade floor) but it should be *chosen*, not discovered:
  if elide-tier adoption ~never happens, the ACK spelling, the Seam's cross-oracle
  coherence machinery, and the vouch-blast-radius design all right-size downward.
  Sequencing implication: build guard-tier end-to-end first; defer the vouch/ACK tier and
  the bottom open fork until guard-tier demand data exists.

- **236a-fd5 · the open fork's pessimism ignores two parked mechanisms the corpus already
  owns.** +SURE they exist; ~SUSPECT they help. The fork ("can a human vouch 'this opaque
  touches only X and Y'?" — worried about pinkie-promises, rot, cargo-culting) should cite:
  (a) **trace-grounded testimony** — kDEPS' own text ("runtime-trace … a backstop to derive"),
  `plans/077`/`plans/deferred/078`: the containerized/eBPF harness generating an *observed*
  footprint the human blesses. This does not collide with the banked "tooling never rescues
  a contract" principle — the tool grounds the testimony's authorship; the claim remains a
  fallible attributed judgment (and traces under-approximate — input-dependent effects —
  which is exactly why it stays claimed-tier). (b) **rot-detection via the parked
  version-drift spike** (TODO.md: binary-content-hash guard) — a vouch pinned to a content
  hash lapses on drift instead of rotting silently, converting the fork's worst failure mode
  (rot-activated cross-site deletion) into a loud lapse-to-guard. With both named, the fork's
  answer plausibly shifts from "hard no" to "yes, narrowly: trace-grounded, hash-pinned,
  per-cell, admin-authored vouches" — still the sharp-knife tier, but no longer a
  pinkie-promise.

- **236a-fd6 · plan-approval semantics under guards need one deliberate sentence.** -GUESS
  cheap. A guarded artifact's execution-set is world-dependent: the user approves a policy,
  not a command list. Formally fine (every command they approved is in the artifact, guarded)
  but two cheap knobs follow from the project's own doctrine and are unmentioned: guard-miss
  disclosure at apply (the drift report — surely intended, rides the diag spine), and a
  conservative-admin option to *abort on guard-miss beyond a budget* (the AGENTS fail-fast
  rephrasing: don't keep executing once the world diverges from what the plan claimed —
  knowledge the bare book never had, so barreling-on is the floor, not the ceiling).

- **236a-fd7 · fat-pole description is internally inconsistent with st-2/inv-g3 — decide
  which bytes ship.** +SURE on the inconsistency, low stakes now. 233's fat pole shows a
  full-args `__predict`-style invocation (`apt_get_check install -y nginx || …`) but the same
  section's license mechanics ("unpropagatable argv ⟹ no path reached ⟹ no vouch ⟹ run")
  and `218a` inv-g3 (guards ship `resolve_probe` bodies with compile-resolved entities;
  the `__predict` argparse never ships — st-2, and it isn't valid sh once annotated) imply the
  compile-resolved form (`package_installed__predict 'nginx' >/dev/null 2>&1 || …`). If
  full-args checks ever ship instead, three latent hazards activate: annotation-stripping
  becomes a correctness-critical transpile of shipped code (the F-OFFRAMP class), the check
  dialect's OOB idioms (`${DORC_VERDICT:?}` aborts when unset) crash in the apply lane, and
  runtime re-argparse re-opens entity-resolution divergence between plan and apply (the "one
  source of truth" claim currently holds precisely because both lanes use the compile-time
  evaluation). Also carry `218a`'s already-designed mechanics forward explicitly: call-site
  output-silencing, preamble funcdef + name-collision refusal, the `set -u` question (u-11),
  and the already-guarded/no-double-guard convergence rule (d4-6) — 233 cites 218a generally
  but these five are the ones that bite in-build.

- **236a-fd8 · guard-tier pins apply to book-order — note the collision with the 22H time
  axis.** --WONDER. Frame-freedom is purchased by sequencing; a guarded site is a
  serialization point. True elision (and only it) ever buys apply-phase reordering/
  concurrency. If the deferred live-plan/concurrent round later wants apply parallelism,
  the ternary bounds it. One line in `plans/22H`'s seed prevents the collision.

- **236a-fd9 · kSILO gains a new shove.** -GUESS. Machine-inserted guards reduce the admin's
  incentive to hand-write the `dpkg -s || install` idiom (Dorc writes it for you); source
  books lean out even though the *artifact* stays defensive. The kSILO entry's mitigation
  list should gain this case (the off-ramp artifact-vs-source distinction).

- **236a-fd10 · prior-art framing worth one paragraph in the synthesis: the ternary lands
  Dorc exactly on the industry matrix.** +SURE as description. Guard-tier ≡ make / Ansible-
  module semantics (local footprint declarations, author-judgment convergence, in-sequence
  re-check — the most battle-tested incremental-execution contract in existence, including
  its known failure mode: under-declared prereqs ⇒ stale skips ⇒ `make clean` ≡ `dorc
  reconcile`, the already-reserved trivial-convergence backstop). Elide-tier ≡ what only
  world-owning/hermetic tools (Terraform state, Nix/Bazel) achieve, via ownership Dorc
  constitutionally renounces. This is both a validation (the recovery converges on the one
  cell the industry left open to a non-owning tool) and a positioning asset (guard-tier
  Dorc = "Ansible semantics, plain sh, one connection, parallel preview, attributable
  why" — still differentiated with zero elision).

---

## §4 Self-deception audit (the commissioned question)

- On "is the crisis real": no self-deception found. If anything the document *under-cites*
  its own corpus (236a-v3) — the failure was real, structural, implemented, and its
  perverse-incentive form (adding an oracle degrades safety) is the community-product-killing
  kind; catching it pre-launch is the falsification-first charter working as designed.
- On "does the recovery retain enough value": the honesty is unusually good (the trade
  stated bluntly up front; the aspirational thin-pole explicitly forbidden from driving
  design; the reversal-of-ruling flagged for conscious re-weld; the correction from the
  crosscheck incorporated). The one graded misjudgment runs in the *safe* direction:
  over-mourning the attention-product (236a-fd1) and over-crediting the elide/vouch tier's
  future (236a-fd4), which together risk wasted r23+ effort on the vouch/ACK/fork machinery
  before guard-tier value is measured (236a-fd2). Redirect, not retraction.

## §5 One-line disposition

Proceed with the ternary; re-base attention on the render surface; measure guard-tier value
on the r21 dashboard before designing the vouch spelling; scope the elide-tier's ACK story
to the curated stdlib; name trace-grounding + hash-pinning in the open fork; write the
`093`-f21 archaeology into the synthesis so the revoked weld stays revoked.
