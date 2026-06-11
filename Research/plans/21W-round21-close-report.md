# 21W — Round-21 close report (the 20K analogue)

> The durable round-close synthesis, written by the close-out conductor (the round's
> THIRD session — two predecessors died to the harness; see §8). Evidence basis: the
> full 21x note series, the 21Xa sweep ledger, and an unusually heavy close-out
> verification campaign (~15 agents; §4) whose convergent re-derivations corrected one
> recorded headline number. Trust the root README/DESIGN/IMPLEMENTATION/KNOBS and the
> human rulings over this; AI gates and crosschecks are PROCESS EVIDENCE, never proof
> (the never-vouch hard limit). Confidence-marked. HEAD at writing: see §10 FILL.
> Suite at writing: 93 e2e cases × six gates; ~450 unit/integration tests; four lint
> gates. Companion documents: 218+218a (the doors design pair), 21Z (the spike-4
> error/provenance inventory), 21L (the harness-pass note).

## §1 Outcome against the charter

Round-21's charter (211 §1, quarantine-free restatement) had seven arcs. Disposition:

- **arch-1 leaf-exact render — LANDED** (214 + the 21E P1 fix). The carve-out family
  and `StatusRenderFloor` deleted-not-bypassed; if/elif guards became an ordinary
  substitution class; `StatusIterated` added; the hostile pass found the pre-flagged
  adjacent-multi-line P1 (region-grouping + quote-state fixes, 21E).
- **arch-2 budget-bounded inlining — LANDED, with one refuted prose claim** (216,
  corrected by 217 §3): depth-2 positional threading never worked; now a loud refusal
  (`dq-depth-2-positional-unthreaded`); literal-arg depth-2 inlines; single-level
  attacked with no break found.
- **arch-3 errexit doors — SPLIT.** door-3 (213) and door-1 cascade verification (215:
  fully general at base, zero engine edits needed) LANDED; door-4/door-2/precedence
  DEFERRED to round-22 as a crosschecked design-only record (218 + 218a; §6). The 212
  mid-round ruling (dashboard-before-door-4) is what made the deferral evidence-driven.
- **arch-4 cmdsub — design note landed** (219); its q-2 diagnostics floor BUILT (21H);
  the q-3 Query-shaped-`$()` prize deferred with its four forks intact (the human's).
- **arch-5 partial-member list-rewriting — DEFERRED on evidence** (21F imp-2
  corpus-thin population; 21K d-6 entry-gate verified: the impl is global-pristine,
  matching its own doc, ambiguity upstream in 20S §3.1's wording; the authorized
  re-scope + expanded obligations transfer to whichever round builds it).
- **arch-6 H2SaLS coverage dashboard — LANDED** (21B), with the headline INTEGER
  corrected at close (§3) and the render-refusal honesty fix (#13) + the bridge
  tripwire (§4) hardening it.
- **arch-7 hostsim differential harness — LANDED** (21D; stretch goal): 600 seeded
  trials clean, all six in-flight findings were harness bugs, engine held.
- Plus, unplanned: y-1 redirect-write cells closing the imp-1 stale-guard hole (21H,
  a candidate P1-at-HEAD found by the r1A direction report), the q-2 diagnostic
  CATALOG (the layer-1 embryo), and the #13 wave-2 fixes (render-refusal demotion,
  depth-2 dedupe+refusal).

The round also absorbed two conductor crashes (§8) and still closed with every landed
arc gated and every deferral evidence-trailed. The fork decision at close (human):
fork-b-shaped — harness pass + close, with the doors as design-only rather than the
originally-floated leave-behind mini-spike branch ("leave the complexity out of the
spike codebase at the end").

## §2 Rulings ledger (made or executed this round; authority noted)

- **dq-errexit-1** (human, 212): stays OPEN; adjudication evidence-driven — candidates
  arrive as constructed strawmen/corpus shapes, never sign-off assertions;
  run-evidence (apt history/auditd/atime) is ledger entry #1; this close adds the
  partial-canary grading (218 §3) and door-4's apply-lane read (218a u-12) as entries.
- **dq-errexit-2** (human, 212): LEANING-yes DOWNGRADED to genuinely-open (kSILO in
  ownership terms); everything in 218/218a keeps all three bare-middle ownership
  models live; 222 §7's prior-art support for oracle-default is recorded as
  evidence-for-one-side, human unconvinced.
- **dq-errexit-3** (human, 212): directional ruling executed-in-design — door-4 is
  CLI-flag-gated for-sure, default `Never` provably-zero-transforms, builds LAST,
  product hard-defers regardless; the trust-boundary taxonomy (a bad oracle must never
  cause novel apply-phase actions) and the stacked-correlated-failure frame are the
  design's §7 attack list.
- **Fork + mini-spike pivot** (human, this close): round closes lean; doors become a
  design-only path-not-taken record (218); the mini-spike branch idea was considered
  and replaced by the human ("design but don't implement, durable note").
- **d-1..d-6** (human, 21K, PROVISIONAL): r22 lean errors+provenance incl. the
  derivation-dump/`why` direction; light OTel early; first-contact reframed (hybrid
  feel-test probe); d-4 builder-commit policy (executed throughout this close);
  r-5 stays deferred; arch-5 entry-gate verified.
- **tc-fix3-severity** (orchestrator, 217 §6, human may overrule): keep catalogued
  Note this round; per-code severity is the r22 catalog retrofit's business.
- **tc-door1-door3-composition d×d** (orchestrator, this close): CLOSED — two
  independent hostile passes, convergent SAFE verdicts across the full cell
  enumeration (§4); pinning fixture specified (the host-flip case), not yet authored.
- **Close-session orchestrator judgments** (human may overrule): the 218a verbatim
  preservation of an agent deliverable as a note; the 21B/21H IB corrections (both
  pass the two-part amendment test: re-derived-twice wrong integer; stale pin line);
  21Z's scoped exception to append-only (status columns of a LIVING inventory).

## §3 The corrected headline (and what it does to the doors)

The dashboard's recorded H2SaLS headline was **wrong in its integer and right in
everything else**. Re-derived independently TWICE at close (once additionally at 21B's
own commit over the unchanged corpus): **172 sites** (crit-wt 238), not 195 — the 195
was a stale mid-development snapshot that never reproduced. Reproduced and stable:
**0.0% full-elision in BOTH probe modes** (no-probe ≡ all-converged — convergence
changes nothing, which is the finding), the four-cause decomposition
(4/172 sites oracled · written-upstream poison via the unmodeled `apt-get update` ·
ONE needs-declaration site · all five `|| true` sites wrap un-oracled tools),
`unattributed = 0`, `needs-declaration = 1`. e2e corpus at close (93 cases): fold=12 ·
dead-invariant=3 · replace-converged=42 · query-substituted=12 · runs=162 ·
unattributed=0; exactly one `runs(render-refusal)` site corpus-wide — the post-#13
demotion arm working (the 21B-era binary reported a false 100% on that case; HEAD
reports the honest 0%).

The L58 needs-declaration site was then characterized to the bottom, and the story
sharpened twice: it is **two-to-three blockers deep** (errexit-⊤, which a declaration
WOULD clear · non-self-reach from its un-oracled `getent` guard, which NO declaration
clears · ~SUSPECT the in-loop Query-probe exclusion behind that), it is ALREADY
admin-guarded, and an honest declaration for `groupadd`-as-spelled fails the
conformance gate anyway (the `||` short-circuits on converged hosts — there is no
converged-run to declare). Net: **door-2's demonstrated population on the
measuring-stick corpus is ZERO sites**; the rescue lever for L58 is one more QUERY
oracle (rung r-2, zero new trust). The one genuine door population found: door-4 under
the broader mint policies (m-b/m-c) plausibly reaches the three written-upstream
installs — the guard re-measures live, so upstream-write staleness is irrelevant —
CONDITIONAL on the flagship oracle being able to honestly declare claim-noop at all,
which the independent design's hunt-A challenges (apt-get install upgrades
installed-but-outdated packages; "converged ⇒ no-op" is false as naively probed).
That conditional question is the implementing round's first task.

North-star statement, per the standing clause (never a target): the reachable ceiling
remains oracle coverage × declaration coverage; this round moved the DENOMINATOR's
honesty (attribution complete, integer corrected, refusal-demotion live) and the
doors' design-readiness, not the percentage — which is the correct outcome for a
deliberately-unannotated corpus whose binding constraint is oracle coverage (the
thesis's intended dependency).

## §4 The close-out verification campaign (what the ~15-agent sweep bought)

Run under the human's spend-the-budget directive, class-disciplined (Opus mechanical /
Fable adversarial), all in-place with rev-parse tripwires after the round's wrong-base
scars. Convergence was deliberate: the two most load-bearing results were each
produced by two independent agents (a cancel-survivor + a relaunch) — the
dashboard re-derivation (172, byte-identical site sets) and the span-bridge attack
(verdicts agree; remediation rankings agree).

- **d×d composition: CLOSED.** Two hostile passes, convergent: no composition hole
  between the two deadness mechanisms; the spine is the disposition precedence
  (fold-Omit pre-empts the door-3 mint per leaf) + known-vs-⊤ rc disjointness; one
  hypothesized smuggling path (door-3 carrying an invalid Query past its gates) found
  ACTIVELY DEFENDED by independent double-gating. Residuals are over-conservatism,
  not holes. Pinning fixture specified (host-flip of `door1-door3-inner-elides`).
- **Two genuine constructible wrong-elisions, OUTSIDE the composition** (both
  builder-ready, neither composition-caused): **find-I** — `right_is_bare_true` is
  funcdef-blind; a book defining `true() { mutator; }` mints door-3 on a false
  inertness premise and the substituted stand-in is itself captured by the function
  (dash resolves functions first). Pathological-input-gated; internally inconsistent
  at HEAD (arch-2's funcdef index exists and door-3 ignores it). **FIXED at close**
  (`f5c9972` + `aa6b3a1`, harvested): `right_is_bare_true` refuses on any book-defined
  `true()` funcdef (file-wide, deliberately non-positional — loop re-entry makes a
  textually-later definition live), plus crate-local `cfg-builtin-shadowed` WARNING on
  any funcdef shadowing an engine-relied builtin (blessed-pure set ∪ `exit` — both the
  classification reliance and all three minted stand-in words). Anti-masking honored:
  pins written first, shown failing at pre-fix HEAD (the exact wrong-elision
  reproduced). r22 tc-residuals: render-side defense-in-depth refusal needs a
  cross-crate shadowing fact; a shadowing funcdef whose inline is budget-refused
  still classifies Pure at the call word (under-poisoning, now disclosed, unchanged).
  (Original spec language, superseded: consult `funcdefs`; diagnostic on shadowing —
  every `StandIn::True/False` emission shares the
  assumption). **find-J** — pipe-consumer reader-liveness is unmodeled: a converged
  oracled establish as a pipeline's LAST stage mints (base-latent; door-3 merely
  unmasks it into `set -e` books); the artifact's producer then writes into a closed
  pipe (EPIPE/SIGPIPE on real hosts) where the bare book drained it. A MODEL question
  (no reader-liveness channel in inv-one-observable's set — which is declared
  extensible) — tc-shaped, the human's: conservative-refusal candidate (a converged
  establish in last-pipe-stage position refuses) loses a rare legitimate elision.
- **The span-bridge (217 §5 obs-3): safe today, zero-margin, partially hardened.**
  Both attackers: the bridge cannot be made to lie at HEAD (the two sides evaluate the
  same expression — a tautology), but it is an accident of co-derivation, not a
  checked contract; four near-future engine changes break it silently, and the worst
  (an unmirrored second refusal class in `collect_edits`) is also cli-dark and
  tripwire-blind. Tier-1 (count-invariant `bridge_suspect`, loud in report+TSV) BUILT
  at close; tier-2 (keyed readout — the diagnostics loop already holds `step.leaf`
  and discards it; ~5 lines on `Plan`) and tier-3 (single-source-of-truth refusal —
  the only fix for the unmirrored-class hole) are r22's, recommended together.
- **The omit-safety letter-violation (render-refused controller over an omitted
  body):** one attacker called it a gate-pierce, the other outcome-equivalent-but-
  letter-violating; a dedicated Fable builder was dispatched to adjudicate empirically
  and fix if real — §5 FILL.
- **Hardenings landed** (B3 harvest, this tree): the obs-2 arena-ordering
  `debug_assert` (`ac1d4f6`) · the discriminating var-resolved redirect pin closing
  21H §9's real residual (`ecf326d`) · the bridge tripwire (`d09c949`).
- **Design crosscheck of 218** (neutral + adversarial + independent-design synthesis):
  three DESIGN-BREAKING finds fixed inline (consumer-provenance tags are real engine
  work; the declared-rc fold fence; spelling-matched anti-stacking), the rest
  dispositioned in 218 §9. The independent design (218a) corrected the declaration
  keying to (provider, verb) and contributed both conformance gates + hunt-A.

## §5 The harness pass (task #12) — FILL AT HARVEST

**LANDED in full** (note 21L is the build record; commits `deadd3a`/`a5c1bef`/
`59bb8b7`/`bf07208` + the orchestrator's omitsafe21 protocol patch). The four slices:
the `EXIT_RC=<n>` marker (closes tc-exec-nonzero-exit; `door1-and-form` converted to
exec-asserted; negative-tested four ways) · the determinism rail (`env -i` +
LC_ALL/TZ/umask on all three exec sites, zero churn, the deliberately-unpinned
residual documented — the DST position) · **gate-6**, the dual-rail license judge —
cm-1 at corpus tier (bare-vs-apply run-sets, every delta license-attributed to the
engine's own replace/omit ledger, TOP-as-position-wildcard) with a four-confound
self-test that aborts the harness if the judge can't scream, and which caught a real
lying-judge bug (IFS splitting) before first commit · the newline-safe mock-log
protocol (shared builtin-only dot-helper, 157 shims swept, zero churn), whose sweep
EXPOSED a latent gate-6 false-pass on two multiline-argv cases (un-encoded newlines
had split the log into a coincidentally-matching fragment) — honestly excluded via
`DUAL_RAIL=multiline-argv` markers. Two carried tc-flags, both corpus-tier
limitations, not engine findings: tc-gate6-inlining (ledger argv is CALL-surface,
bare log is inlined-body — the hostsim differential owns that attribution) and
tc-gate6-multiline-argv. The harness now has SEVEN gates, and for the first time a
gate whose own judge is self-tested before any case runs.

**The omit-safety pierce (F1) — confirmed, divergent, FIXED** (note 21N; commits
`5e50572`/`5f6a873`/`4736c14`, harvested). The two span-bridge attackers had split on
whether a render-refused (heredoc-bearing) Query guard left LIVE over its `:`-omitted
body was a real hazard or letter-only; the dedicated builder adjudicated empirically:
constructible in `||`, `&&`, AND if-form; the `||` form is outcome-equivalent to the
sanctioned frozen render in every world (the letter-violation reading), but the
**`&&`-flipped world genuinely diverges beyond the TOCTOU-sanctioned class** — the
substituted `:` fabricates a success rc the bare book never produces (consumable by
errexit, `$?`, the script's exit), i.e. inv-probe-sourced-values pierced via the
render seam. Fix: `is_neutralised`'s Replace arms now consult the same heredoc-refusal
predicate the render uses (2 functional lines; the Omit arm deliberately untouched —
transitive recursion is the honest gate, preserving the heredoc-inside-dead-block
elisions); dispositions unchanged; kFAIL-perform direction. 5 unit pins + 3
`omitsafe21-*` e2e cases, each demonstrated to FAIL at pre-fix HEAD. tc-flag for the
human: the fix removes one licensed elision class (~SUSPECT value-free — the live
guard executes regardless, so the `:` saved zero remote commands and under-executed
in the flipped world); plus two cosmetic pre-existing flags (the dead-region refusal
diagnostic's misleading prose; the comment-drop-on-heredoc-lines disclosure class).

## §6 Deferral record + open-flags reconciliation

**Deferred WITH design (door-2/door-4/precedence):** 218 (the synthesized design,
crosschecked) + 218a (the independent design, verbatim) are the pickup point; the 212
obligations (flag default-Never provably-zero, build LAST, mandatory hostile pass —
218 §7 is its pre-registered attack list), the conformance gates, the consumer-tag
engine prerequisite, and hunt-A's claim-noop question all transfer. **Deferred with
evidence (arch-5):** 21F imp-2 + 21K d-6; the re-scope authorization + expanded
obligations + 20T's did-not-survive prior-art transfer intact. **Deferred design
forks (arch-4 q-3):** 219's four forks, the human's; fork-capture-claim-type is
load-bearing.

Open-flags table (carried → status at close):
- tc-fix3-severity → RESOLVED-keep-Note (217 §6), human may overrule; per-code
  severity = r22 catalog retrofit.
- detached-funcdef asymmetry → recorded-untouched (217 §6).
- door1×door3 d×d → CLOSED this close (§4); pinning fixture specified, unauthored.
- obs-2 arena-ordering → CLOSED (`ac1d4f6` debug_assert; the verification also
  surfaced a second, distinct entry-population-timing invariant the assert does not
  cover — recorded, --WONDER grade).
- obs-3 span-bridge → tier-1 CLOSED (`d09c949`); tier-2/3 carried to r22.
- 21B seam wishlist → seam-1 (public ⊤-reason readout) still the highest-value item;
  seam-2's verdict-reconstruction coarseness now has a demonstrated masking instance
  (the L58 in-loop-floor masking, §3); carried.
- imp-5 refusal-poison ceiling-bound → carried, now with the concrete dashboard
  context to surface it against.
- h-1 CoLiS STTT fetch + h-2 Hermit Linux box → carried, human-pending.
- 222 m-5 sampled cross-check → carried to the door-implementing round (its seam is
  designed: the DoorChoice policy swap).
- 21H §9 stale line → IB-corrected; its real residual (the ⊤-vs-concrete test
  ambiguity) FIXED (`ecf326d`); the optional var-resolved e2e case remains authorable.
- tc-query-bare-elision · tc-perselector-wrapper-scheme · root-KNOBS kELISION edit
  (uncommitted at repo root, human-owned) · 207 errexit/YOLO tension · st-2
  one-vs-two-declarations at dq-kOOB · 20U §4's four autonomy-rulings → all carried
  unchanged, human-pending.
- NEW at close, the human's: find-J's reader-liveness model question (§4) · the
  fact/consent split for declarations (218 §9 find-7) · door-2-behind-flag ambiguity
  (218a div-2) · SyncThing syncing `.git/worktrees/` internals (§8 — infrastructure,
  urgent-ish).

## §7 Seeding feedback (for the next priming prompt)

Carried: fb-1 copy-vs-cleanroom · fb-2 pre-state broken tooling · fb-3 known-tensions
list at seed · fb-4 delegated-judgment pattern · fb-5 SyncThing worktree exclusion
(now ESCALATED — see fb-9b) · fb-6 unread-source claims cap at ~SUSPECT · fb-7
reconcile-by-source-not-vote (executed throughout this close; it is what caught the
195 and the L58 misreads).

New, from this round's scar tissue:
- **fb-8** (212 meta): relayed rulings carry a one-line `[spike]`/`[product]`
  disposition marker — the one batch-1a ambiguity class, eliminated.
- **fb-9** (21Y/21Xa, ≥2 session deaths): the sec-gate discipline must be IN the
  priming prompt — analyzer-as-subject, corpus-as-inert-data, never enumerate
  hardening material, sentinel-ledger protocol for suspect reads.
- **fb-9b** (this close): SyncThing is syncing git INTERNALS (`.git/worktrees/*/
  sequencer` ghost-husks blocked a cherry-pick mid-close). Exclude `.git` and
  `.claude/worktrees` from sync, or expect plumbing corruption classes beyond the
  file-resurrection one.
- **fb-10** (21Y, executed here): never the harness's worktree isolation; explicit
  orchestrator-created worktrees at verified bases; first-action rev-parse-or-STOP.
  Worked 7/7 this close.
- **fb-11** (21Y): d-4 harvest-by-reapply leaves original commits in builder
  worktrees; audit by content-diff scoped to the builder's paths, never ancestry.
- **fb-12** (21Xa phase-4): the 210→21Y resumption-prompt skeleton (role → safety →
  ordered orientation → verified-state → forks → GATE-with-ask → rulings → process →
  open-flags → meta-goal) demonstrably onboards a cold conductor; reuse it.
- **fb-13** (21Y): agent `.output` transcripts are empty on disk; never infer
  liveness from mtimes.
- **fb-14** (21K d-4): state the builder-commit policy AND fb-11's audit gotcha
  together; round-20 notes still describe the old no-commit posture.
- **fb-15** (this close, new): background agents may SURVIVE a user-side cancel —
  three "cancelled" agents completed and delivered. Treat cancellation as
  advisory until a terminal notification arrives; bank late results (two of ours
  became the round's most load-bearing evidence).
- **fb-16** (this close, new): per-task token logging died with the conductor crash
  (r21 build notes carry none; 20K §5's table was the round-20 norm). The close
  session's own ledger (§8) restores the practice; keep it in builder briefs.
- **fb-17** (this close, new — a near-miss): the CONDUCTOR's own gate-chain is a
  harness too and lies the same ways. The close's harvest-verification chain had two
  defects — piped gate steps masking exit codes (`… | tail` returns tail's 0), and no
  explicit `cargo build` before e2e (cargo test refreshes libs and test binaries, NOT
  `target/debug/dorc` — the m-7 stale-binary class, recurring at close: F1's harvest
  showed 3/96 "failures" that were the stale bin exhibiting pre-fix behavior while the
  fixed code's unit pins passed). Canonical chain for every harvest: build → fmt →
  clippy → deny → test → e2e ×2 with unmasked exits → typos; and read the OUTPUT, not
  the chain's final echo.

## §8 Process & dispatch (the meta-goal deliverables)

The round's process story is dominated by the two conductor deaths (cumulative
sec-vocabulary hypothesis, 21Xa sweep: no single bomb document) and the recovery
machinery that worked: the caretaker handoff (21Y v1), the resumption sweep ledger
(21Xa), the rebuilt singular prompt (21Y v2), and this close session onboarding cold
from it through a human-adjudicated GATE.

Close-session dispatch ledger (the fb-16 restoration; tokens are subagent-reported):
obs-2 verify Opus 106k · 21H pins Opus 139k · 21Z survey Opus 168k · dashboard
re-derive Opus 175k + 147k (two independent) · span-bridge Fable 218k + 179k (two
independent) · d×d Fable 285k + 225k (two independent) · 218-neutral Opus 193k ·
218-adversarial Fable 248k · independent design Fable 263k · evidence pack Opus 110k ·
B3 hardenings Opus 155k · F1 omit-safety Fable 236k (the close's only P1-class engine
fix — found by crosscheck disagreement, settled by execution) · B4 find-I fix Fable
213k (anti-masking exemplar: pins written first, shown failing at HEAD) · B2 harness
Opus 378k (the close's largest build; 4 slices, 331 tool-uses, ~92 min — the
pre-spelled-contracts heuristic at its limit, and it held).
Class discipline held: Fable ONLY on correctness-critical adversarial/design work;
every build was Opus; nothing graduated. The deliberate-redundancy pattern (two
independent agents on the two most load-bearing questions) paid both times:
convergence raised confidence where they agreed (172; bridge verdicts), and the
DISAGREEMENTS were each settled by whichever agent ran the decisive experiment
(the 21B-era-binary reconstruction beat the engine-change guess; the with-guard
strawman beat the without-guard one) — reconcile-by-source, operationalized.
Heuristics confirmed: pre-spelled contracts make big builder tasks mechanical (B2/B3
briefs); hostile passes remain the highest value-per-token spend (every novel
correctness finding this close came from one); the GATE-then-go rhythm survived three
human redirections (fork-a → mini-spike → design-only) without rework because nothing
irreversible happened before the ask.

## §9 Note index (round-21 complete)

211 plan-of-attack · 212 rulings batch-1 + r1A arrival · 213 door-3 · 214 arch-1 ·
215 door-1 cases · 216 arch-2 (§1.2/§6 REFUTED — trust 217 §3) · 217 wave
reconciliation + #13 harvest · 218 doors design (synthesized) · 218a doors design
(independent, verbatim) · 219 arch-4 design · 21B dashboard (integer IB-corrected) ·
21D differential harness · 21E arch-1 P1 fix · 21F r1A absorption · 21G direction
batch-2 · 21H y-1 + q-2 catalog (§9 IB-corrected) · 21K direction batch-3 · 21L
harness pass [FILL] · 21Xa resumption sweep ledger · 21Y conductor prompt (v2) ·
220/221/222 round-22 research · plans/21W this report · plans/21Z spike-4 inventory.

## §10 Final state

**Code HEAD at close: `43978ec`** (the omitsafe21 protocol fixup; the notes/plans
commits carrying this report follow it with no code delta). Final gate run, executed
by the conductor on exactly this tree, real exit codes: `cargo build` · `fmt --check`
ok · `clippy --workspace --all-targets -D warnings` exit 0 · `cargo deny` bans/
licenses/sources ok · `cargo test --workspace` **463 passed / 0 failed** (1
pre-existing ignore) · `sh e2e/run.sh` ×2 = **96/96 both runs, SEVEN gates** (dash-n,
apply-exec, probe-exec, redirect sandbox, ordered run-set, stderr floor, argv-echo
differential, dual-rail license judge) · `typos` clean. Round-21 commit span:
`e8808e0..43978ec` (+ the plans commits).

**Left for the human (look-don't-touch inventory):**
- `%TEMP%\dorc-r21\{b2-harness, b3-hardenings, b4-findI, f1-omitsafety}` — this
  close's builder worktrees; all content harvested by cherry-pick (originals remain
  in them per the d-4 audit gotcha — verify by content-diff, never ancestry);
  removal is yours. Likewise the historical `{cmdsub, coverage, door1, hostsim,
  p1fix, xcheck2, shapecheck}` (audited drained, 21Y).
- Pre-round clutter: `worktree-agent-*`/`bridge-*` branches + three
  `*.sync-conflict-*` branch ghosts — not round-21's; cleanup candidates.
- `ai/r1A-H2SALS` — deliberately unmerged (212); the dashboard reads the sibling
  path read-only.
- **SyncThing is syncing git INTERNALS** (fb-9b): a `.git/worktrees/spike3/sequencer`
  ghost-husk blocked a cherry-pick mid-close (cleared). Exclude `.git` and
  `.claude/worktrees` from sync before round-22.
- Untracked, human-owned, never committed: `STALENESS-AUDIT.md`, the three
  quarantine seeds.
- Ready-to-author r22 starters (specs complete, in the cited notes): the d×d
  host-flip pinning fixture (`door1-door3-dead-block-folds`) · the var-resolved
  redirect e2e case (21H residual-2) · the span-bridge tier-2/tier-3 hardening ·
  the 17-code catalog retrofit.

**Round-22 send-off pointer:** the declared lean is errors + provenance (21K d-1) —
seed from `plans/21Z` (the primitive inventory) + 220 (the receipts-plane research) +
21G §2 (the two-layer intent). The doors program resumes, when chosen, from
218 + 218a + the 212 obligations. The corpus story to carry: 172 sites, 0.0%,
oracle-coverage-bound, doors-population-zero — the number moves when oracles and
declarations do, which is the thesis working.
