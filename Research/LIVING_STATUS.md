# LIVING STATUS — the conductor's resumption document

> **Purpose (durable — this header outlives every round):** the single always-current on-ramp
> for a fresh conductor. This file is *state*, never history (the numbered `notes/` are the
> chronological record) and never authority (the human-written root docs, stamped `plans/`, and
> `spike/CLAUDE.md` rulings outrank it). **Nothing important may live ONLY here** — rulings and
> findings get a durable numbered-note home; this file carries pointers.
>
> **How to maintain:** update judiciously — direction-changes, discoveries, refutations,
> deferments; never per-turn chatter. The bar: nothing lost to a context-collapse. Density is
> **vaguely logarithmic in age**: the newest work sits at the TOP, rich enough for a new
> conductor to skill up on; day-old context compresses to a paragraph; older history decays to
> a line, a pointer (a note slug, `git log`), or deletion. Reverse-chronological, always.

---

## NOW (2026-07-04 — round 25: protocol REVIEWED + hardened + dry-run DONE; rotating conductor, Stages-4-5 landing)

**Round 25 = the methodology for the first human-driven real-machine field trial of Dorc** (the human
has never invoked the built CLI — first real contact). Deliverable: a pre-registered,
adversarially-reviewed *playbook* so the human's day against a real machine yields *decisions*, not a
"woo cool" hit. Fable conducts; real-machine EXECUTION hands to a narrow-brief Opus.

**Durable artifacts (`ai/spike3-r23`):**
- `plans/250` — charter (Goals, grounding scenario, targets: `target-adequacy`+`target-admin-loop`
  primary, `target-gap-log` priority-1, `target-felt-product` don't-slip; refined constraints).
- `notes/251` — slurp-synthesis + every settled fork (two-track structure, protect-first-contact,
  Vultr substrate, value-locus/durability/HHHF/multi-host rulings + corrections).
- `plans/252` — **THE protocol** + Track-A fan-out (players P1–P8, §2 contracts, §3 DAG, §4 questions,
  §5/§5.1/§5.2 guardrails+op-run-findings, **§7 = adversarial-review revisions + the LOCKED ops-target**).
- `notes/254` — crosscheck adjudication + **retrospective ledger** (F1–F11 + dispositions; re-read
  F2/F3/F7 first if a Phase-A green later proves hollow). `quarantine/25xxx` = the crosscheck prompts.

**State:** DRAFTED → adversarially cross-checked (two Fable passes; converged: *instrument-rich,
decision-poor*) → hardened (`252 §7`: pre-register §4 as numbers; B4 gated on Stage-4/5 landing; B3
asymmetric felt-signal; A1 planted=sensitivity-gate + disjoint stdlib; differential noise-governance /
A-A calibration; gap-log forcing-functions; reinstate confound-conditions). Per the human's
*at-most-one-non-trivial* lean, the one non-trivial fix = the **paper dry-run** below.

**Ops-target LOCKED:** single-box mini-homelab — nginx reverse-proxy + Windmill(+postgres) +
Home-Assistant(Container), self-signed certs. (HA genuinely blocked-on-Dorc = the dogfood-want; more
services = more composition-exercise; corral network-chaos, don't eliminate. Rubric: many-parts>few; mostly bog-standard-oracle-able + a few walls (tractable-vendor→admin-loop,
opaque-hork→residue); real-drift→adequacy; corral-chaos-not-control.)

**DRY-RUN DONE + committed** (research-firming deferred to the on-VPS validation, per human):
`notes/255-homelab.book.sh` (runbook, ~21 sites) + `notes/255-homelab-dryrun.md` (per-stage ledgers +
ceiling + decisions-log; **§5 pred-1..5 ARE the pre-registered §4 predictions** — F1/F5 satisfied).
**Numbers:** Stage-B steady **5 elide / 7 guard / 9 run**; +hand-oracle **8** (modulo u3); ceiling
**~14** (unreachable — footprint tier unbuilt); floor **~4 never-elide** (`su -c` postgres,
docker/HA-internal, `systemctl reload`). Conductor sanity-check: **sound + honest** (its confidence-
marks hold; the 5-elide is +SURE, the +3 rides u3). **Three takeaways for the day:**
1. **u3 = the sharpest uncertainty** — does the built spike lift a *stdout-consuming* pipe-guard
   (`cmd | grep -q X || fallback`)? If not, Stage-C == Stage-B (a finding). **Checkable IN-REPO NOW
   (no VPS)** — worth de-risking pre-Phase-A.
2. At the built tier **attention barely moves** (16→13 of 21 face the user) — the footprint tier
   (Stages 4–5, *unbuilt*) is what buys the headline value-prop; walk in knowing, or "barely helped"
   misreads as failure.
3. Genuine finding: the idiomatic Debian postgres idiom `su - postgres -c '…'` is a permanent
   opaque-wrapper wall.
Owed nicety: fold 255 §5's predictions into `252 §4` (cheap; they live durably in 255 regardless).
The `# FLAG:` runnability specifics firm on the VPS (compose-fallback = pre-registered contingency).

**Substrate + creds (REVERSED 2026-07-04 — NO platform switch):** stay on **Windows/git-bash** (human
has too much in-flight to move; swallowing the friction). op-session-caching is broken on Windows, so
**sidestep `op` entirely**: the expiration-bound Vultr key lives in a permissions-scoped file OUTSIDE
the Sync + repo trees; agents `. keyfile` → `VULTR_API_KEY` in env → `vultr-cli` (that path is PROVEN,
`§5.2`; only the op-injection changed). **Human places the key — conductor must NOT handle the raw
secret.** **`§5.1` guardrail FIRMED** (≤3 instances · cheapest-that-runs-it · <$10/day · `dorc-r25` tag ·
human-reaper · always-teardown). Key **CONFIRMED WORKING** (full-account auth; file
icacls'd user-only) — the earlier `token expired` was the **missing-`export` bug**: the file assigns
`VULTR_API_KEY=…` without `export`, so agents MUST source via **`set -a; . ~/.temp/vultr.env; set +a`**
(plain `.` doesn't reach the child `vultr-cli`). **Vultr agents UNBLOCKED.**

**SWEEP LAUNCHED (2026-07-04):** spend **AUTHORIZED** — standing: ≤3 cheapest `dorc-r25` boxes, <$10/day,
**human = manual reaper** (next Vultr check ~late tonight; not watching minute-to-minute). **Oversight
model:** the conductor owns ack + per-agent limits + periodic `vultr-cli` polling for runaways;
subagents get the key but NOT ack. Live agents: **P1** (Vultr substrate — isolated worktree; provision/
snapshot/restore/destroy + prove trap-teardown, `dorc-r25` tag) + **`~/System` scout** (read-only recon
→ firms `255` windmill FLAGs from the human's real Ansible + scopes P6/zsh → `notes/256`). `~/System` =
human's public system-repo (windmill Ansible + a complex zsh setup + a bit of Vultr mechanization P1
cribs from), read-only reference for the sweep. **Isolation:** agents and any manual reap filter STRICTLY on the `dorc-r25` tag — never touch
pre-existing untagged instances (the human's own infra). Baseline: **0 `dorc-r25`**
instances/snapshots. Orphan-detection keys on `… | grep dorc-r25` (a raw line-count is unreliable — the
text table's footer rows inflate it).

**P1 DONE (2026-07-04, conductor-verified clean — 0 dorc-r25, eurydice untouched):** disposable Vultr
substrate `Research/trial/vultr/vultr.sh` (C-vps: provision/snapshot/restore/destroy + `destroy-all`
reaper + `status` + always-teardown `run`). Proven live: provision→ssh-reachable→snapshot→destroy on
one real Debian-12 box, 0 orphans, $0 left, guardrail-compliant. On worktree branch
`worktree-agent-a0618edd45959e557` (based `427b36d`, **UNMERGED**, unpushed). **Flags:** (1) plan =
`vc2-1c-1gb` (~$0.007/hr — cheapest *broadly-available IPv4*; the $0 / $2.50 / $3.50 tiers are
free-limited / IPv6-only / one-region, each breaking the differential or SSH — sound). (2) **Windows
ssh gotcha (real, will hit P3):** the human's `~/.ssh/config` `usekeychain` (macOS-only) breaks git-bash
`ssh`; P1 dodged via `ssh-keyscan`, but **P3 must use a trial-local `ssh -F` config, not his global
one** (Windows-substrate ↔ macOS-dotfiles friction; cf. the P6-zsh fork). (3) **SAFETY OWED:** P1
(correctly) never pointed `destroy` at the real box — the tag-scoped-destroy guard `_assert_tagged`
rests on construction + offline tests ⇒ **human must eyeball `_assert_tagged` before trusting
`destroy-all` near his infra**; `restore` is untested-live. **Next (human-gated):** eyeball
`_assert_tagged` → integrate vultr.sh onto `ai/` → resolve P3's ssh-config → dispatch P3 + P2 onto a box. Agents relay trouble via
`SendMessage`→conductor→human (the conductor holds the notify path); the `dorc-r25` tag covers a
hard-died agent's orphan.

**HANDOFF (fresh conductor, rotating 2026-07-04):**
- **Stages 4–5 (derived footprints — the tier that buys the ATTENTION value-prop) LAND FIRST, before
  Phase B** (human-confirmed; parallel r24 work, ~EOB). So the trial WILL test the full value-prop,
  not attention-light (resolves the B4-gate; `255` takeaway 2). Confirm they're in before scheduling B.
- **Parallelizable prep to spin up now** (all r24-independent, `252 §3`): Track-A leaves **P1**
  (Vultr provision + guardrail — do the `§5.1` specifics turn first), **P2** (comprehensive observer),
  **P3** (ssh-apply runner), **P6** (HHHF, zsh/dotfiles-shaped); **u3 in-repo de-risk** (does the built
  spike lift a *stdout-consuming* pipe-guard `cmd | grep -q X || fallback`? `255` takeaway 1 — the
  highest-value cheap check, NO VPS needed); fold `255 §5` pred-1..5 into `252 §4`; freeze `§2`
  contracts.
- **Then:** execution on **macOS** (op-caching works there; fresh Opus from `252`) → **Phase B** (the
  human's single first-blooding).
- **su-c note:** `255` dec-4's `su - postgres -c '…'` wall is standard Debian-postgres peer-auth
  (real-ops-knowledge, NOT a corpus-crib/hallucination); the finding stands; the exact line's
  runnability firms on-VPS (a `# FLAG`).
- **On-ramp:** THIS block → `252` (protocol + §7) → `254` (review + retrospect ledger) → `255` (dry-run
  + predictions). All committed (`90ee2e2`). Live task: #6 (execution handoff).

## Round-24 status (2026-07-05 — Stages 1–5 COMPLETE (A+B) + owncoord + pipe-guard pins; polish pass IN FLIGHT; then e2e-migration + #12 + Stage 6)

**Since the entry below (all conductor-verified + merged, tip ≈`aa31081`+):** **Stage 5 Part B
(`reaches()`) LANDED** — typed emission held both promises (kind from the LIFTED annotation, never
host stdout — the arm-index demux; static arms trace/ship-nothing, dynamic escalate); cross-author
composition REAL (hork's `package:nginx` expanded through the owner's `reaches()`, the downstream
file-fact demotes where pre-B it wrongly survived); four lying-nets green (static 579 / derived 220
/ alias 147 / reach 97; poison-attribution 95); `resid-kindfn-derived` = the one asymmetry (dynamic
arms over DERIVED coords need a 2nd round-trip; deferred with resid-resolve-derived). **owncoord
LANDED** (24G §8: engine unions own effect-coord into non-empty footprints; the derived boilerplate
printf died; empty-emission = no-claim boundary unit-pinned). **pipe-guard pins LANDED** (24C
§pipe-guard: the r25 trial-shape floor is SAFE; gap = the check-side pipe precisely; owed value =
check-tax+attention only; two Stage-6 forks parked: flag-pipe-status-unit / flag-filter-blessing).
**rul24-warnings-tune-high + rul24-boilerplate-cargocult** minted (24G §8). **Git housekeeping**
(human-delegated): 70 worktrees + husks removed, 71 absorbed branches deleted, ~115GB freed;
keep-uniques listed in the cleanup report; `ai/snapshot` = unknown provenance, untouched, human
eyeball owed. **IN FLIGHT: the first-contact polish pass** (charter `24H` + all 8 human
ack-rulings; caret-art IN-SCOPE; elision-render greenlit — original-bytes-commented where rc
unconsumed; golden churn UNBLOCKED by human ruling, conductor still inspects at merge).
**QUEUED:** e2e de-graduation (audit DONE — **`24I`** = the execution spec; ~100/152 movable,
5 safest-first batches; THE flag: the in-memory tier must add a one-shot `dash -n` per artifact
or it re-opens the ap-2 text-diff trap) → `touches()` typed-emission migration (#12, human-set
LAST) → Stage 6 (measure/conclude/extract; carries the two pipe-guard forks + adequacy-bite).

## Round-24 EARLIER (2026-07-04 — Stages 1–5A; kind-owner design round)

**Stage 5 Part A (the aliasing closure) LANDED + merged** (**147 e2e, 25 suites, all gates —
conductor-verified by own hand**). `resid-aliasing` is CLOSED for resolver-bearing kinds: the
kind-owner ships `package.resolve()` (KIND-keyed, host-run, fork-4A rails); the engine
canonicalizes BOTH sides before `disjoint`; can't-resolve ⇒ may-alias ⇒ run (`may-alias=N`
instrumented); resolver-less kinds keep the token-equality floor byte-identical. **The three
lying-nets coexist green: static 579 / derived 220 / alias 147** (per 3000 seeds, all attributed).
Flagship `strawman24-alias-provides`: a converged `nginx-full` victim past a running `nginx` wall
canonically HITS and DEMOTES where token-equality wrongly elided. Evidence + residue: **`24C`
§Stage-5A** (resid-resolve-derived · resolv-lane parity gap · strain-coreference-crosskind = the
post-trial co-reference design seed).

**The kind-owner family design round SETTLED (live human dialogue, five volleys):** the record is
**`notes/24G`**; the surface story is **USER_STORY stages 6–7** (human-directed update; rarefied
<10%-of-authors, high community-effect framing). The rulings: two families (per-TOOL trio /
per-KIND pair) · ONE function per question (the graduation test killed the edge/valuation split) ·
**typed emission** (kind = trailing annotation, entities = raw stdout; vocabulary closes at lift;
the `| sed` wart dies) · error-posture = un-annotated emission MEANS NOTHING (smell, never refusal;
hard errors stay syntax + genuine static conflicts) · **`reaches()` not `manifest()`**
(name-as-contract; omission-bias). Sequencing (human-set): **Part B (`reaches()`) next** →
`touches()` stringly-emission migration **LAST** (FIXME rides USER_STORY stage 5) → Stage 6.

## Round-24 EARLIER same-day (Stage 4 — derived footprints; detail `24C` §Stage-4)

**Stage 4 LANDED + merged** (145 e2e at the time). The golden-hill move works for PAYLOAD-BOUND tools: a
`touches()` body that reaches a host tool ESCALATES (ships strip-only into the probe lane, runs
read-only, stdout → the footprint) — the frame rule through a *dynamic frame*. **Yardstick 0→1
derived** (`strawman24-derived-survive`: a converged nginx install elides past a RUNNING oldpkg
install, licensed by a probe-derived footprint from the natural `dpkg -L "$1" | sed 's|^|file:|'`
idiom); **the sweep soundness net has teeth — `derived_lying_divergences=220`/3000 seeds** (too-narrow
derivation → wrong survival → end-state RED, attributed). Spec = **`notes/24E`** (+§13 fork-resolutions,
§14 pipes); landing evidence + residue = **`24C` §Stage-4** (new `resid-derive-coherence`, the
kill-coherence e2e-net owed, `resid-derive-adequacy` → the r25 field-trial's primary target).
**Posture-lift (human-directed, first of the round):** pipes now PARSE in the oracle dialect —
parse-permissively/trace-conservatively (`24E §14`; the ⊤ moved to the trace layer; touches-pipe =
the escalation trigger, predict-pipe = run). **NB for the r25 conductor: §14 ≠ u3** — §14 is
ORACLE-dialect pipes; u3 asks whether a *book-side* stdout-consuming pipe-guard
(`cmd | grep -q X || fallback`) lifts — a different parser (`dorc-syntax`), still un-checked.
NEXT: **Stage 5 (grounding — resid-aliasing, the r25 gate's second half)**, then Stage 6
(measure/conclude/extract). Loose: battle-oracle corpus (#5, feeds the r25 stdlib) · Lcg low-bit
fix (#6) · the lying-KILL-footprint sweep scenario (owed, `24C`).

**Process notes (durable):** after any cherry-pick, force `cargo build --workspace` before trusting
`e2e/run.sh` (stale-binary false-fails). Cross-session branch discipline: two conductors share this
branch — GATE any ref-move on the expected tip IN THE COMMAND (`[ "$(git rev-parse …)" = <sha> ] &&`),
not by eyeballing printed output (a check-then-act race orphaned an r25 commit this session;
recovered by cherry-pick, nothing lost).

## Round-24 EARLIER (2026-07-03 — Stage 1 complete; the test-suite arc commissioned)

**Stage 1 fully built, merged, green** (`ai/spike3-r23`; e2e **135/135**, 9 standing guard23
xfails; family elide-fraction **0.32**, post-wall elisions = 0 — the charter's honest
baseline, mechanically true). The three sub-stages: 1a yardstick (CLI `dorc: plan-summary`
stderr line + `strawman24-*` family + `sh e2e/yardstick.sh`); 1b fd10/silence=wall (plan-time
wall walk in `build_plan_walled`; running modeled mutators wall downstream, elided cast no
shadow); 1c riders (strip-fidelity — fixed a REAL latent rc-clobber bug; kill-wall; the first
verdict-function-carrying floors; errexit-honesty row). Full evidence: **`notes/24A` §3**.

**The round's durable homes (rotation discipline — 24A is CLOSED as the early-rulings ledger;
new chunks get fresh notes):**
- **`notes/24A`** — Stage-1 rulings + evidence + the 231 disposal. The typed rulings:
  rul24-wall-placement · rul24-threefunc-monotonic (the `touches()` 3rd role-sibling;
  supersedes the two-contract sentence) · **rul24-mode-gate** (survival tier FLAG-GATED
  `--trust-footprints`, never default; marketing-at-best/theatre-at-worst) ·
  rul24-divergence-is-the-game (license-site≠elision-site + claim-subject≠blast-subject;
  attribution co-primary for Stage 2; MH2 version-gate = the missing tether, seeded
  not-this-spike) · **rul24-vouch-is-verdict-authoring** (§1c — the tilde is DEAD; authoring
  `is_converged()`/`is_diverged()` IS the vouch; provisos: read-erasure, family-open,
  marks-survive) · **rul24-overtype + the ARC-WIN** (the claim-tier trust algebra:
  `Claim<Tier,Payload>` Fact/Judgment/Silence; license-mints DEMAND tiers in signatures;
  births Stage 3, pays hardest Stage 5; "uncheckable invariants ride WITH the types").
- **`notes/24B`** — the TESTING ARCHITECTURE: three flavours (kernel-unit / real-path corpus
  / the new in-memory sweep); battle-oracle corpus = STANDALONE fixtures, not e2e case-dirs;
  the sweep is a NEW composition-root crate (hostsim stays the model). §5 reconciles against
  the round-12 DST conclusion (`plans/128`): the net is elision-soundness DST at the
  fact-verdict seam (NOT round-12 coordination-DST — that's round-25/22H); the determinism
  guard is mandatory; approach-#3/cross-platform-by-purity; coverage humility.
- **`notes/24C`** — STAGE LANDINGS + RESIDUE ledger (accretes per stage). Stage 2 (golden
  hill lit 0→1) + its residue (resid-aliasing = the Stage-5 under-execute cell that MUST be
  professed at the horizon; kill-coherence; argparse-drift contained). Stage 2b (the sweep
  has teeth — 3 planted bugs caught) + two findings: **find-lcg-thinning** (Host::seeded's
  low-bit coin correlates → existing DST tests under-explored; cheap fix owed, task #11) and
  **find-net-covers-what** (survival-tier coverage rests entirely on the lying-attribution
  net — honest survival is provably sound, so the lying scenarios are load-bearing).
- **`notes/24D`** — the STAGE-3 TYPE-ARCHITECTURE SPEC (conductor-authored, rul24-overtype):
  the claim-tier trust algebra `Claim<Tier,_>` (Fact/Judgment/Silence; 4 unrepresentability
  props: one-way demotion, mints-demand-tiers, no-judgment→fact-plane, the OPEN rung reserve)
  + the guard tier + the elide-weld + verdict-fn lift. **The round's most foundational
  type-decision — the reviewable one.**
- **`ORACLE_PROVIDES.md`** (NEW root doc, pending human audit) — the ledger of information
  shapes an oracle hands Dorc (provides-decoding/vocabulary/reading/binding/behavior/
  convergence/license/margins); the license-LADDER (rung-0 display / rung-1 guard / rung-2
  elide) is OPEN — the wary-engineer hatch, sibling of the admin's mode-gate flag.

**Landed since the Stage-1 refresh:** Stage 2 (frame-rule machine) · Stage 2b (the chronology
net / `dorc-sweep`) · **Stage 3 FOUNDATION** (the claim-tier trust algebra `core/src/claim.rs`
+ verdict-fn lift + guard type-architecture — merged, all 4 unrepresentability props
conductor-VERIFIED as compile-errors; the earmarked-reviewable decision, `24C`/`24D`).

**⚑ HELD FOR THE HUMAN (durable — survive a context-collapse):** (Stage-3 Part B is DONE — the
foundation was blessed, the elide-weld landed 2026-07-04.) Parked design threads, none blocking:
`aliasing-horizon-wording` (the survival tier's silent under-execute owed a horizon disclosure in
the human's voice; reverse-DNS-namespace-owner is the confirmed bounded approach — Stage 5 closes
the mechanism) · `rung-split` (the wary-author hatch — license the guard but not the skip; reserved,
seam built) · `analyzer-taint-gate` (a consume-only-provable-inputs typesystem constraint, distinct
from report-only provenance; human-tabled) · `ORACLE_PROVIDES.md` first audit.

**Stage 3 Part A LANDED (guards FIRE):** the mint-wiring merged; **all 9 `guard23-*` XFAILs
PROMOTED** (conductor-inspected the renders + authorized). Dorc now produces real
`( oracle-check ) || command` guards on a real book past a real wall — **the guard half of the
two-halves doctrine is REAL.** Two emitter bugs found+fixed end-to-end (double-comment; a
redirect refuse-home that was suppressing a `>>log` side-effect). One latent soundness gap
found → **#12 find-return-vouches** (the verdict-lift mis-reads a `*) return 2 ;;` decline as a
vouch — runtime-safe in guards today, but bites Part B's elision, and it's the USER_STORY's own
decline idiom; folded into the Part B brief).

**CONDUCTOR HELD HERE — a deliberate checkpoint, not a stall.** After Stages 1/2/2b/3-foundation/
3-Part-A (six builders, all merged green), the next moves each want the human or conductor
design-work first, so this is the clean review point:
- **Part B (elide-weld)** — held for the human's foundation go/no-go (corpus-wide churn).
- **#6 Stage 4 (derived footprints)** — unblocked + foundation-light, BUT it ships `touches()`
  into the PROBE lane (host-executed `dpkg -L`), a NEW execution surface that must preserve
  probe-inertness (structural self-vouch, like predict bodies) — a real design point the
  conductor should spec (rul24-overtype) or the human should eye before dispatch. NOT blind-
  dispatchable.
- **#10 Stage 2c (battle-oracle)** — blocked on Stage 3 (Part B) + #12.
- **#7 Stage 5** (grounding — claim-tier pays hardest) → **#8 Stage 6** (adequacy-bite / conclude).
- Loose: **#11** (Lcg fix), **#12** (return-vouches, rides Part B).

Resume options on the human's return: nod the foundation → dispatch Part B (+#12); or direct
Stage 4 after its probe-inertness spec; or redirect. Nothing is broken or blocked-on-a-fix.

**Conduct addenda this session:** worktree-isolated agents base on ancient `main` — brief
step-zero `git switch -C <branch> ai/spike3-r23` (`reset --hard` hook-blocked) + tip verify ·
Fable conducts / Opus codes (type-contract design is the ONE conductor-code carve-out;
mechanical verification rides Opus) · research grounds in-corpus first (the round-12 DST round
is authoritative + current; no fresh Kagi) · new test coverage is in-memory/Rust/
performant-by-design, NOT more e2e case-dirs.

## Earlier today (2026-07-03 — round 23 CLOSED; round 24 charter + Stage 1)

**Round 23 is closed.** Its complete, durable, single-narrative history is **`notes/23O`** (the
closeout — read it first). The crisis (233 = the frame problem, permanent) resolved to the
**ternary verdict {elide, guard, run}** with the **converged-vouch** license and **silence = wall**;
the interface was settled (**role-split** `predict`/`is_converged`/`is_diverged` + **rc-partition**
0/1/≥2 + strip-fidelity); the spike was **realigned** to the design (marker fiction retired, 123/9/0/0
green); the guard tier was **pinned** (24 `guard23-*` cases); and the elide-half design reached a
**permanent floor** (`23M`/`23N`): the mechanism is the **separation-logic frame rule through a
dynamic frame** = **lazy code motion for shell**, and its one live risk is the converged-vouch's
**adequacy** (converged≠no-op), calibrated-never-proven. All settled law lives in `23O` §2 +
`spike/CLAUDE.md`'s rulings blocks. Do not relitigate it.

**Round 24 = "head off 233 by building something and seeing what happens"** — the plan is
**`plans/240`**. The theory is exhausted at its floor; the work is now empirical. Build the
**elide machine** (the golden hill — the attention product Dorc exists for) FIRST, on hand-authored
**strawman** books, because in the spike the **DST exec-differential is the correctness net** (run
the elided plan under mocks, diff the bare book; a wrong elision goes red). The guard is the
*production* net — deferrable. The yardstick, CLI-runnable at every stage: **elision frequency on
a strawman family, differential-verified.**

The six-stage ladder (full detail in `plans/240`; golden hill lights up at Stage 2):
1. **yardstick + honest baseline** — build the elision-count-plus-differential mode; land the
   `fd10` fix so *silence = wall* actually holds (the "dangerous middle" is still live at HEAD).
   Baseline: post-wall elisions = 0.
2. **the frame-rule machine — first line vanishes past a wall.** authored footprint + backing +
   disjointness + `elide-when-disjoint-else-run` (no guard; differential is the net). Yardstick 0→N.
3. **the guard tier** (the 9 `guard23-*` xfails) — the production net; a side-quest off the golden
   hill's critical path, slotted here per the human's ordering.
4. **derived footprints** (`dpkg -L`) — elide past payload-bound tools.
5. **grounding + collaboration** — coordinate-kinds, bridges, the `scan_cve` story; synonym =
   dynamic-points-to-or-wall.
6. **maximize + measure + conclude** — the ~80% question on strawmen; extract conclusions to the
   human docs; then the spike can die.

**Build on Opus** (mechanical-ish; reserve cheap-Fable for breadth + the round-25 reactivity
design). The spike is freshly realigned — build now, before it drifts.

**Live task state (reconciled at round-23 close):**
- DONE: #7 pins · #14 spike reconciliation · #17 interface rulings · #18 rename · #19 golden-hill
  design (floor reached → `23M`/`23N`/`23O`) · #20 closeout · #21 `plans/240`.
- CARRIES INTO ROUND 24: **#15** — the repair pass; its `fd10`/silence=wall fix IS Stage 1, the
  strip-fidelity implementation (ruled: bare marks deleted whole) rides along, plus the small pins
  + the 231 disposal paragraph. **#16** — the human's root-doc queue (line-fixes + adopt the `23N`
  vocabulary + the "lazy code motion for shell" README line).
- TABLED: **#11** — the placement-spectrum / barrier round = the *performance* product; parked to
  round-25+ (by the consent-wall it offers the attention goal nothing; `23O` §4).

**Deferred-work ledger** (durable now in `23O` §5 — 22H reactivity is round-25 and wants Fable
ASAP; provenance-DAG is reorderable and may ride this spike; MH2 versioning; the language +
`unsafe` hatch; kSTATE; DX tooling; `.diff`; the deferred surfaces incl. lane-privilege).

**Conduct fences (standing; bind any successor):** word-slugs in full words, explain prior-art
inline (the human is often on mobile); silence ≠ ack (only what he TYPED counts); **HARD
QUARANTINE on corpus/H2SaLS** (the `quarantine-DO-NOT-READ/` dir + `Research/corpora/` stay
unread; strawman measurement only, never the corpus); crosscheck adjudication under maximum
skepticism (convergence = signal; a corpus doc's *existence* is never authority — reverse-
sycophancy is a live failure mode); adversarial framing = exclusions-not-inclusions; Fable
dispatch = ask-first, goals-not-instructions (Opus gets full enumeration); code-modifying agents
→ isolated worktrees with a baseline-check + explicit-pathspec commits; never edit
README/DESIGN/IMPLEMENTATION/TODO/AGENTS/root-CLAUDE (human-only); notes are append-only EXCEPT
this file; **never use the AskUserQuestion tool** (his vi-mode breaks it — ask in prose); dump
the full numbered TaskList when it changes or when he's remote; the method is now
**build → measure the yardstick → let the evidence pick the next stage.**

**On-ramp order for a fresh round-24 conductor:** root docs (`README`/`DESIGN`/`IMPLEMENTATION`) →
`spike/CLAUDE.md` → **`notes/23O`** (the closeout — everything that happened + the settled law) →
**`plans/240`** (the round-24 plan) → THIS FILE → then, as the build needs them: `23M`/`23N` (the
elide mechanism's landmines + vocabulary), `23A`+`23G` + `spike/e2e/run.sh` (the guard-tier spec),
`23H` (the spike's reconciliation record + the strip-fidelity residue for #15).

---

## Yesterday-scale (round 23, 2026-06-15 → 2026-07-03 — compressed; full record `notes/23O`)

Opened on `plans/230` (best-effort), intercepted by the human's `plans/233` crisis (the oracle
poison contract was broken — the frame problem). Resolved to the ternary verdict + converged-vouch
(crosschecked `236a/b/c` → `237`; ceiling `238`; signed closure `239` + the two-halves doctrine).
Interface settled (`23K`/`23L`: role-split, rc-partition, strip-fidelity) after the rc-soundness
cluster surfaced in the direction-crosscheck (`23I`/`23J`). Spike realigned to the design over five
sessions (`23E`/`23H`: marker fiction retired). Guard tier pinned + reviewed (`23A`–`23G`). Elide
half worked to its floor (`23M`/`23N`: 233 permanent; frame-rule/dynamic-frames/lazy-code-motion;
consent-wall; survival-settled-vs-adequacy-is-the-risk). Turned empirical → `plans/240`.

## Ancient (pre-round-23)

See `Research/README.md`'s per-round map (rounds 1–22).
