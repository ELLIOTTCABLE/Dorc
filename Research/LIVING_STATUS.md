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

## ⏩ COMPRESSION-HANDOFF SNAPSHOT (2026-07-05) — read this first

**Branch:** r25 lives on **`ai/spike3-r25`** (worktree `.claude/worktrees/spike3-r25`), forked from r23
at `990d966`; **r24 continues on `ai/spike3-r23`**. LIVING_STATUS/252 fork with them. Conductor commits
go to r25; isolated agents commit granularly on their own branches → conductor cherry-picks in.

**Live agents (background):** **P2 observer** (`a3557…` — dorc-INDEPENDENT on-box snapshot+diff, wider
than dorc's model, §7 noise-governance [A/A envelope, world-drift, planted canaries]; spins its own
`dorc-r25` box; HARD-fenced out of `crates/`; catalogues dorc-side differential needs). **⚠ P2 HALTED mid-run by the
harness security-net (flagged its box-spend as an unauthorized real-world transaction — the net is
PER-SUBAGENT and can't see the conductor's standing spend-auth); its box was orphaned then
conductor-REAPED (`3d34d700` destroyed, eurydice safe, key confirmed valid). Its harness is likely
INCOMPLETE — salvage its commits. PROCESS-FIX: the CONDUCTOR provisions/destroys boxes going forward
(subagent-spend trips the net → halt+orphan); agents work on a GIVEN box. Bugs found: `vultr.sh`'s
auth-recheck fails on a VALID key (re-sources without `set -a`?); `vultr-cli instance delete` needs the
FULL UUID (short id → 400 invalid-resource-format).** **verify-errand**
(`a114…` — in-repo `dorc plan` on the book: settle pred-1-errexit + the host-guard-wall MECHANIC;
amends `255 §5`). Both report/cherry-pick back to conductor.

**BUILT + on r25** (`Research/trial/`): **P1** Vultr substrate (`vultr/vultr.sh`, tag-scoped, proven),
**P3** ssh-runner (`apply/`, the `usekeychain` `-F` fix + C-run flags), **P6** HHHF capture tooling
(`hhhf/`, interactive paths await a human 5-min zsh smoke-test). **The OTel book is VALIDATED**
(`notes/255-homelab.book.sh` + 5 sidecars — comes up clean rc0/96s, ~$0.007, 2 GB adequate; ops-val
fixed grafana `--config`, prometheus `--web.route-prefix=/`, nginx subpath). Docs: `plans/252`
(protocol §1–§9), `notes/254` (review ledger), `255` (book+dry-run), `256` (recon).

**BLOCKED / owed (future conductor):**
- **P5 oracle-stdlib — BLOCKED** on the oracle-contract churn (touches/reaches/cross-oracle post-wall
  elision, r24-building). **No stdlib exists yet.** When written: brief carries the `252 §9` memo-2
  quality-bar checklist + MUST cover the book's real command surface (`hostname`, test/`[`,
  install/ln/chmod/rm — not "trust coreutils").
- **P4 full differential — BLOCKED** on P2 landing + dorc-side `crates/` mutations (r24's; P2 catalogues).
- **version-guard-lift (u3) — r24-OWNED** (pipe-guard doesn't-lift-at-HEAD, fix in flight, lands before r25).
- **Fold `255 §5` predictions → `252 §4`** (owed; the verify-errand's amendment is the correction vehicle).
- **Human-gated:** `_assert_tagged` eyeball (before bulk `destroy-all`); HHHF zsh smoke-test; the B2
  hint-confound check (did r24 land the stage-3 hint? `252 §9` memo-4).

**Cost/oversight:** standing auth ≤3 cheapest `dorc-r25` boxes, <$10/day, human-reaper. Baseline **0
`dorc-r25`**; 1 pre-existing (**eurydice = OFF-LIMITS**, tag-filter). ops-val ~$0.007. Poll
`vultr-cli … | grep dorc-r25` for orphans (watch P2's + the errand's boxes).

**Findings (durable, `252 §8/§9`):** first-blood spine = felt-WORKING experience (`dorc why`-illuminates
+ admin-loop-reward + plan-trust), adequacy mechanized to the background differential; multi-wall-cascade
→ `touches()` load-bearing (validated on drifted days); rc≠health (differential must check `is-active`);
attribution/LLM-authorship watch-item; the host-guard-wall/hostname-coverage item (verify-errand).

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

**Ops-target LOCKED:** single-box mini-homelab — nginx reverse-proxy + an **OTel stack** (otel-collector + prometheus + grafana-on-postgres, native
SEPARATE units — **SWAPPED from Windmill 2026-07-04**: windmill-native is admin-invented/undocumented,
and compose would hide the multi-service behind one opaque `docker compose up` [Dorc can't exercise it]
+ is redundant with HA's docker wall; OTel = genuine multi-service as separate native units Dorc SEES,
documented installs, + the human's familiar ground. Re-point **DONE** (`a8af1b6`+`fc1595e`, cherry-picked to r25; 30 sites, 3 vendor walls; **needs a 2–4 GB VPS** now — HA+prometheus+grafana+otel+postgres — still <$10/day, consistent with P1's "cheapest-that-runs-it"); keeps postgres/su-wall,
HA-docker stays the sole hork) +
Home-Assistant(Container), self-signed certs. (HA genuinely blocked-on-Dorc = the dogfood-want; more
services = more composition-exercise; corral network-chaos, don't eliminate. Rubric: many-parts>few; mostly bog-standard-oracle-able + a few walls (tractable-vendor→admin-loop,
opaque-hork→residue); real-drift→adequacy; corral-chaos-not-control.)

**DRY-RUN DONE + committed** (research-firming deferred to the on-VPS validation, per human):
`notes/255-homelab.book.sh` (runbook, ~21 sites) + `notes/255-homelab-dryrun.md` (per-stage ledgers +
ceiling + decisions-log; **§5 pred-1..5 ARE the pre-registered §4 predictions** — F1/F5 satisfied).
**Numbers (OTel, 30 sites — re-pointed from windmill 21):** Stage-B steady **5 elide** (same ambient
floor); **+3 vendor-oracles → 15 elide** — the value-curve walks **3× (+10, vs windmill's +3)**; ceiling
**~23/30 (~77%)** (unreachable — footprint tier unbuilt); floor **~4 run** (`su`×2, `nginx reload`,
HA-internal). **u3 now gates ALL 3 oracles** (3× load-bearing); NEW **multi-wall-cascade** (a stale
middle binary re-walls the later vendor by position, 15→9). All 3 installs **documented-native** ⇒
won't eat the day (the tarball+version-guard form is now a deliberate Dorc-exercise choice).

**OPS-VAL DONE (2026-07-05): the OTel book COMES UP** on a fresh Debian-12 box (rc=0 / 96s; all services
active, HTTPS on self-signed, otel→prometheus data-path live, grafana-on-postgres [87 tables, not
sqlite]). **Cost ~$0.007** (vc2-1c-2gb @ $0.0136/hr; **2 GB adequate** — ~910 MB used / ~1 GB free, no
4 GB needed; 0 orphans conductor-verified). **3 fixes cherry-picked** (`fbef730`/`ddccee7`/`28da459`:
grafana `--config`→`conf/defaults.ini` [tarball ships no grafana.ini; explicit-missing is fatal];
prometheus `+--web.route-prefix=/` [external-url had 404'd remote-write]; nginx grafana-subpath
prefix-preserve). **4 known day-risks left faithful:** docker-run non-idempotent (the intended
poison-wall — good), HA-UI websockets-through-nginx, box-must-be-named-`homelab`/`hl-*` + cert-CN-only,
`ufw allow` without `ufw enable`. **2 process findings (design-relevant):** (a) **book rc=0 does NOT
prove services healthy** — `systemctl enable --now` returns 0 on a crash-loop (dead grafana rode along
under rc=0) ⇒ the differential (P4) MUST check `is-active`, not just rc/fs; bears on how Dorc reads
apply-success; (b) killing local ssh does NOT halt the remote apply (day-UX abort note). **P5 now
UNBLOCKED** (book validated + firm). Throwaway ssh key left at `~/.ssh/dorc-r25{,.pub}` (inert; human
deletes at will). Conductor sanity-check: **sound + honest** (its confidence-
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

**BRANCH FORK + WAVE-2 (2026-07-04):** round-25 forked to its OWN branch **`ai/spike3-r25`** (worktree
`.claude/worktrees/spike3-r25`); **r24 Stage-4/5 continues on `ai/spike3-r23`**. They share history to
`990d966`, then diverge — **this LIVING_STATUS forks with them** (r24 status here goes stale; the live
r24 view is on r23). Reason: r24+r25 sharing one branch churned LIVING_STATUS/root-docs repeatedly (the
`990d966` Stage-4 commit swept the P1-done edit above — it landed, but the collision kept recurring).
**P1 + `~/System` scout DONE** (P1 block above; `256`). **Wave-2 — all ISOLATED, committing granularly
in their own worktree branches → cherry-pick onto `ai/spike3-r25` after base-verify when they land:**
web-pass **DONE + cherry-picked to r25** (`255` FLAGs firmed: WM_VER→**1.747.0** [`1.470.2` was a fake
404-tag], asset real, MODE/PORT/DATABASE_URL confirmed + **`BASE_URL` added** to the unit; **native-binary
windmill = admin-invented territory** — upstream docs only docker/compose, so dec-2's compose-fallback
is the documented escape; the `su-postgres` block auto-creates windmill's roles ⇒ book compatible),
P3 (ssh-runner + the Windows
`ssh -F` fix for the `usekeychain` gotcha; live-box integration deferred), P6 (HHHF capture tooling per
`256`: preexec-JSONL spine + ZLE friction-button + asciinema-extractor). **Blocked-until:** P2 (live
box + human's `_assert_tagged` eyeball), P4 (P3/P2 interfaces), P5 (rides r24 — Stage-4 now landed, so
P5 unblocks soon). Poll `vultr-cli … | grep dorc-r25` for runaways (baseline 0; eurydice off-limits).

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

## Round-24 status (2026-07-04 late — Stages 1–4 COMPLETE; Stage 5 next; detail `notes/24A`–`24E` + `24C` §Stage-4)

**Stage 4 (derived footprints) LANDED + merged** (this branch; **145 e2e, 25 suites, all gates —
conductor-verified on a fresh build**). The golden-hill move now works for PAYLOAD-BOUND tools: a
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
