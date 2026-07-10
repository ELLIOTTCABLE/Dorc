# 252 — Round 25 field-trial protocol + Track-A fan-out spec (SKETCH for review)

> Tier: AI-authored (Fable conductor), 2026-07-04. Consolidates `plans/250` (charter) + `notes/251`
> (slurp synthesis + every settled fork) into one reviewable, red-teamable, fan-out-ready document.
> **SKETCH — for human review.** The players + their guardrails + the interface contracts (§1–§3) are
> the primary content: they gate BOTH the adversarial review (task #5) and the parallel Opus fan-out.
> Confidence-marked. Trust the root docs + `250`/`251` where they conflict.

> **⚠ ROUND TABLED (2026-07-10, r24 close-out — `notes/24U` §6; revival per `plans/270` §5):**
> the protocol stands unchanged; only the schedule moved. Translation for the revival
> conductor: every "rides-r24" dependency now reads "rides round-27" — the stdlib
> (`270:stdlib-authoring`, né P5) lands in `270:block-stdlib`; §7's B4 gate ("Stage-4/5 having
> landed") is SATISFIED (`24C` Stages 4/5); the wrapper/payload/read-value machinery
> (`270:block-context`) newly un-walls the book's `su -c` and `$(hostname)` lines, so re-check
> `255 §5`'s predicted numbers against the revived tier before Phase B (the owed
> fold-255§5-into-§4 remains the correction vehicle).

## §0. The shape

Two phases (`251`: protect-first-contact + the phase-A-serves-phase-B razor):

- **Phase A — mechanical, LLM-driven, containers/VPS, iterate freely.** Builds the instrument and
  burns down all mechanical risk, so Phase B is pure signal. Most of it is spike-INDEPENDENT ⇒ runs
  **parallel with r24** (§3).
- **Phase B — the human's first-blooding, single fire, wrapped in the HHHF.** Provision a real
  game-server on Vultr, scrappy book + 1–2 hand-oracles. Fires **ONCE**, after Phase A's exit-gate
  (A3) passes — the human is never the framework's first exercise.

Welded (`251`): substrate = **Vultr VPS** + snapshots + cloud-init; book = **scrappy-first**;
multi-host = **MVP + human-exercised**; executor = the **welded ssh-a-script floor**, throwaway.

## §1. The players

Each: *what* · **durability** (product-content / durable-ish-framework / one-off) · **dep**
(parallel-now / rides-r24) · *interface* · *guardrail*.

**P1 · Vultr substrate.** VPS provision + cloud-init + snapshot/restore/destroy automation (crib the
human's System/Infrastructure repo). · **one-off** · **parallel-now** · emits `provision→{host,ip}`,
`snapshot→id`, `restore(id)`, `destroy(host)` (C-vps). · *Guardrail:* the **resource-guardrail (§5.1)** —
isolation + spend-envelope + mandatory auto-teardown + clean key handling + per-spin authorization.
Human: eyes-open, expects a small deliberate spend; Vultr is non-critical (migrated off it, perfect
for exactly this); no paid resource before the guardrail exists + a collated plan is acked.

**P2 · The comprehensive observer (the anti-self-greening eyeball).** Full dorc-INDEPENDENT
ground-truth machine-delta: broad filesystem diff (content + metadata + xattrs) + a syscall/write
trace (auditd or eBPF) + a wide system-state snapshot (units, sysctl, modules, nft/iptables, users,
cron, ports, a "does the game-server respond?" health check). · **durable-ish** (a real
acceptance-instrument — the `kVERIFY` calibration-harness's real-machine tier) · **parallel-now**
(builds against a bare VPS). · emits a normalized `machine-delta` per apply-run (C-delta). ·
**RULE (load-bearing — `251` cont.3):** the observer MUST be **wider than dorc's own state-model.**
Capturing only dorc's modeled dimensions is self-greening ≈ in-memory DST; the whole value of real
hardware is catching state dorc *doesn't* model — which IS the `target-adequacy` bite (converged≠no-op
= a trigger/reaction/quota/xattr/module/sysctl dorc had no concept of). · *Guardrail:*
**agent-oneshot / turnkey** — if eBPF/auditd setup threatens the day-window it is an agent's problem
or it is dropped, NEVER the human's (human hard-flag: must not become the round's all-consuming goal).

**P3 · The ssh-apply runner (spike-executor stub).** Wires dorc's existing probe→results→apply
round-trip to real ssh — an EXTERNAL shim around the current CLI (probe on stdout → `ssh host dash` →
results on stdin → apply artifact → `ssh host dash`), no spike-code change. · **spike-durable stub**
(non-kCOMMS-committing: the welded ssh-a-script floor, nothing fancier — no fan-out, no scheduler, no
OOB signalling) · **parallel-now** (external wrapper; consumes the CLI I/O framing, C-cli). · exposes
`apply-run(host,script)→{transcript,rc}`, invoked with P2's observer wrapped around it (C-run).

**P4 · The differential harness (the correctness net).** Orchestrates: snapshot → bare-full-apply
(P3) under observer (P2) → capture delta → restore → dorc-elided-apply (P3) under observer → capture
delta → **diff-of-deltas** (bare-delta △ dorc-delta; safe ⟺ ∅). · **durable-ish** (the reusable
acceptance-test shape) · **dep:** P1+P2+P3 (scaffolding builds against stubs; final wiring needs
them). · consumes `{book, oracles, host}`; emits `differential-verdict = ∅ | categorized
divergence-set` (C-verdict). Catches under-execute **even in dimensions dorc doesn't model**, via P2's
width — that is its entire reason to exist over the in-memory sweep.

**P5 · The LLM oracle-stdlib (product-content).** ~40 bootstrap oracles (apt/dpkg, systemctl, ufw,
file/cp, …) + game-server-specifics, authored to r24's **settling** dialect (verdict-functions,
strip-fidelity; Stages 1–3 welded — author against the Stage-3+ form). · **durable product-content**
(= the `effort-allocation` bootstrap set; the Stage-2c battle-oracle seed) · **CARVE-OUT: rides r24's
tail** (dialect churn ⇒ parallel-authoring = rework). · dorc-loadable `-o *.oracle.sh` (C-oracle).

**P6 · The Human-Handholding Framework (HHHF).** Session instrumentation: asciinema + an
ANSI-stripping extractor → an LLM-readable timestamped transcript; checkpoint / think-aloud prompts
fired at seams (a transcript shows *what*, not *why it annoyed*); a `HISTTIMEFORMAT` history spine; a
gap-taxonomy emitter (the aperture channel). · **one-off DISPOSABLE** (dead-simple, ZERO
forward-compat tax — a rewrite is cheap on a disposable spike — just don't gratuitously hardcode the
human as the only subject) · **parallel-now** (wraps a terminal session). · emits `session-bundle =
{transcript, checkpoint-answers, gap-log}` (C-session). · *Guardrail (human, 2026-07-04):* shaped to the human's **zsh + dotfiles +
System/Infrastructure repo** — wraps his native environment and introduces ZERO unfamiliar tooling
(annoyance at tools he doesn't normally use = noise that confounds the friction signal; disposable ⇒
be maximally This-Specific-Human-shaped). · *Banked:* the harden-for-a-friend upgrade is
a **post-r25 fast-follow** (`251` cont.4), not built now.

**P7 · dorc-binary r25 surfaces (in-spike).** The machine-readable disposition/why output P4 consumes
(`--debug-argv`-style, one structured line per site: leafid · disposition · argv · why — 21D
precedent); the plan-summary yardstick (**EXISTS**); the fleet-plan-aggregation surface (only IF P8's
loop proves insufficient). · **in-spike / product-adjacent** · **rides-r24** (touches spike code;
mostly already exists). · C-cli / C-debug.

**P8 · The multi-host MVP (fleet) — DEFERRED post-r25 (human, 2026-07-04).** r25 is **single-host.**
The human judges multi-host almost certainly can't be a mere shell-loop: it needs tiny dorc-binary
features (the fleet-plan-view — a loop yields N separate plans, not one unified "here's what each box
needs"), and bolting dorc features is spike-work that must not gate the r25 trial. Per the razor
(don't build what can't be cleanly exercised in r25), P8 + its `B5` question move to a post-r25 round.
(Substrate note: Vultr makes N-boxes nearly free, so the deferral is scope/dep, not cost.)

**Agents (who builds).**
- **Track-A parallel Opus subagents** — one each for P1/P2/P3/P6 (independent); P4 after its deps
  (§3). Each briefed against the §2 contracts + the r24 build gotcha (force `cargo build --workspace`
  before trusting `e2e/run.sh`; stale-binary false-fails — LIVING_STATUS r24 note).
- **r24-tail Opus** — P5 (stdlib) + P7 (dorc surfaces where they touch spike).
- **The rehearsal-agent** (A3) — an Opus plays-admin through P6 in a container; the Phase-A exit-gate.
- **Fable (me)** — conducts: this doc, the contracts, the builder briefs, review. (Human: "contracts
  are what fable is for; conduct.")

## §2. The interface contracts (the anti-collision seams — pinned by the conductor)

Parallel builders build incompatible pieces unless these are frozen first:

- **C-vps** (P1↔all): `provision→{host,ip}` · `snapshot→id` · `restore(id)` · `destroy(host)`.
- **C-delta** (P2→P4): the `machine-delta` schema — a set of typed changes `{path/entity, kind,
  before, after, source∈(fs-diff|syscall|state-probe)}`; the normalized union of fs-diff +
  syscall-trace + state-snapshot, made comparable across two runs.
- **C-cli** (P7→P3): dorc's probe-on-stdout / results-on-stdin / apply-artifact framing (exists — pin
  the exact bytes).
- **C-run** (P3→P4): `apply-run(host,script)→{transcript,rc}`, with P2 observing around it.
- **C-verdict** (P4→report): `∅ | divergence-set`, each divergence categorized
  (under-execute / over-execute / unmodeled-dimension).
- **C-debug** (P7→P4): the disposition/why line (`leafid · disposition · argv · why`).
- **C-session** (P6→report): `{transcript, checkpoint-answers, gap-log}`.
- **C-oracle** (P5→dorc): `-o *.oracle.sh`, Stage-3+ dialect.

## §3. The Track-A build-DAG + parallelization

- **Parallel-now (r24-independent) — the fan-out leaves:** P1, P2, P3, P6 (+ P4-scaffolding against
  stubs).
- **Rides-r24:** P5 (stdlib), P7's spike bits, P8's fleet-plan-view.
- **Order:** `[P1 ∥ P2 ∥ P3 ∥ P6]` → P4 wires P1+P2+P3 → **A3 rehearsal** (Opus-admin through P6 in a
  container, on a stub/early stdlib) → (r24 delivers P5/P7) → **Phase B ready.**
- **Fan-out precondition:** THIS doc reviewed + the §2 contracts frozen. (Human: not right-this-second;
  once the plan firms. The Vultr guardrails, P1, are their own gate before any paid resource is
  created.)

## §4. The pre-registered question set

Each: *observation → decision* · *signal* · *confound-isolation*. (`dorc why` is the confound-isolation
spine throughout — and where it can't reach, the `+something` above-analyzer attribution layer is
itself a B6 finding.)

**Phase A — mechanical.**
- **A1 · adequacy-bite-rate (PRIMARY).** Seed P5 oracles with realistic subtle-wrong vouches (the
  ufw-regex / apt-`-o` / converged≠no-op class) and run P4 across a strawman family + the game-server
  book on real VPS state. *Signal:* # under-executes P4 catches, by category (modeled vs
  **unmodeled-dimension**). *Decision:* high bite-rate on *plausible* oracles ⇒ adequacy is a real
  product risk ⇒ name the tightening (e.g. MH2 version-gate); near-zero across realistic oracles ⇒
  adequacy holds empirically at this scale (bounded reassurance). *Confound:* P4's diff-of-deltas is
  dorc-independent — a bite is a bite regardless of what dorc's `why` claims.
- **A2 · real-machine soundness (gate).** Does ANY elision under-execute on a real box that the
  in-memory sweep (24B-C) passed? *Signal:* P4 red/green on books the sweep greened. *Decision:* red ⇒
  a real-world hole the mocks structurally couldn't model (the trial's unique reach + a sweep-generator
  gap); green across the family ⇒ the mock↔real gap is empirically small here.
- **A3 · exit-gate (mechanics rehearsal — REVISED twice; the human's HHHF-vs-dorc split).** Two
  sub-parts:
  - *dorc side (LLM-fine):* an Opus plays-admin driving dorc's **plain CLI** (no TUI yet, by intent)
    through a canned provisioning script with planted friction — validates the end-to-end flow +
    dorc's outputs. No interactivity problem.
  - *HHHF side (the human's real catch):* faithfully exercising interactive-zsh watching (history
    hooks, `preexec`/`precmd`, the asciinema PTY, the extractor) is genuinely LLM-hostile — a plain
    `zsh -c` script **FALSE-GREENS** it (interactive-shell paths don't fire; the `ap-2` "validate the
    interactive path, not a proxy" trap). Two routes: **(a) a ~5-min HUMAN smoke-test on a throwaway
    dummy task (RECOMMENDED)** — a real human at a real zsh IS the faithful instrument; cheap;
    non-contaminating (NOT dorc/game-server ⇒ first-contact stays pristine, only HHHF-familiarity
    spent); and it protects the **priority-1 gap-log** from a broken-recorder first-blood. Or
    **(b)** an LLM driving a real interactive zsh via `tmux send-keys`/`expect` (feasible — fires the
    hooks — but an interactive-test-harness for *disposable* scaffolding is itself the `ap-1`
    anti-pattern). **Lean (a):** the human genuinely is the right tool here, once, for 5 min on a
    dummy — the correct instrument, not a concession, and nowhere near first-blood.
  *Signal:* session-bundle legible + complete + ergonomics-OK. *Decision:* pass ⇒ Phase B cleared. The
  human-shaped residual (do the prompts elicit good data from a *confused* human) stays soft — the
  dummy covers most, the rest is discovered live. **The human does NOT co-develop with the authoring
  Opus; P6 stays subagent-built.**

**Phase B — human first-blood, single fire.**
- **B1 · works-at-all (precondition / first test).** Does dorc provision the box idempotently (apply;
  re-apply = converged)? *Signal:* worked / didn't + what broke. *Decision:* fails ⇒ that IS the day's
  headline (and a Phase-A escape — why didn't A3 catch it?).
- **B2 · admin-loop (co-primary).** The lazy-admin loop: write scrappy book → hit friction → write a
  minimal oracle → get value. *Signal (P6):* friction-log + time-per-oracle + did-the-hint-guide-me +
  felt "was it worth it." *Decision:* authoring miserable / hints useless / no value-per-oracle ⇒ the
  gradual-enhancement thesis (DESIGN priority 2 / `kBURDEN`) is in trouble — a foundational finding.
  *Confound:* `dorc why` each non-eliding line — separate "engine can't see my careful code"
  (careful-book-paradox) from "authoring is just hard."
- **B3 · felt-product vs blind-run (high).** Does dorc feel meaningfully safer/saner than
  `ssh host 'dash -s' <script`? *Signal:* honesty-scaffolded reflection anchored to P6-timestamped
  moments + behaviour (time-on-task, re-run confidence). *Decision:* no felt delta over blind-run ⇒
  the attention/sanity value-prop is weak for the target user. *Held loosely* (n=1) and NOT trusted as
  a number (the perception gap).
- **B4 · value-locus (i), decomposed (`251` cont.3).** (1) **frequency** — how often does
  full-elision fire vs guards on the real book (mechanical, = the yardstick); (2) **bite-rate** (=
  A1); (3) **felt-when-it-fires** (n=1, weak). *Decision:* fires-often × bites-rarely ⇒ full-elision
  earns its accepted unsoundness; barely-fires or bites-often ⇒ guards suffice, yeet it. **The
  mechanical (1)+(2) are the check on the human's own 233-bias** — present decomposed, let the numbers
  talk, don't further inflate (i).
- **B5 · heterogeneous-fleet — DEFERRED post-r25** (see P8; r25 is single-host). The question (mixed
  fleet, one book, honest per-host plans) is real and banked for the round where multi-host earns its
  dorc-side features.
- **B6 · aperture / gap-log (priority-1).** Everything that is NOT elision/guard — DX friction, `dorc
  why` quality (and where it needed the `+something` above-analyzer layer), error-message quality,
  unexpected value OR anti-value. *Signal:* P6's gap-taxonomy. *Decision:* the round's unknown-unknowns
  harvest; findings here seed the next round. Explicitly counters the human's self-flagged
  233-tunnel-vision.

## §5. Guardrails (consolidated)

- **Vultr (P1):** the resource-guardrail — full sketch in **§5.1**; no paid resource created before it
  exists + a collated per-spin plan is human-acked.

### §5.1 · The Vultr resource-guardrail (firmed 2026-07-04 — human-set caps)

The standing policy governing any Opus subagent that touches paid Vultr resources. Six parts:

1. **Isolation.** Everything lives in a dedicated, tagged bucket — every instance / snapshot / firewall
   stamped `dorc-r25` (a Vultr tag + a naming prefix), so the whole trial is enumerable and
   bulk-destroyable in one call, and nothing an agent does can touch the human's other Vultr resources.
2. **Spend-envelope (human-set 2026-07-04).** (a) *count-cap:* **≤ 3 concurrent `dorc-r25` instances**
   (single-host trial ⇒ expect 1 live/run; 3 is pure slack). (b) *size-cap:* **cheapest tier that runs
   the workload, never a large/expensive plan** — mechanical Track-A boxes (differential/observer) fit
   the absolute smallest; the full 3-service homelab (windmill+postgres+HA-in-docker) wants ~2 GB, so
   an agent may pick the cheapest ≥2 GB plan *only if* the smallest OOMs — all still < $0.03/hr. A box
   OOMing is itself a finding to log, not a licence to size up freely. (c) *total-cap:* **< $10 for the
   whole day** — trivially held (cheapest tiers bill fractions-of-a-cent/hr); the cap exists to catch a
   box left running, not to budget per-run.
3. **Teardown (load-bearing).** Every provision is paired with a teardown that fires ALWAYS — success
   or failure (trap/finally in the runner); a run cleans up after itself, handling the common case.
   Backstop against a *crashed*-agent leak: **the human is the manual reaper** — an automated
   long-lived reaper is OUT (this environment can't host a durable cron; the human's Windmill is
   shaky/maybe-dead) — via a one-shot `destroy-all-dorc-r25` + a self-set reminder (a leaked
   hourly-billed small box costs pennies until the nightly sweep). (Human, 2026-07-04.)
4. **Key handling — expiration-bound key in an env-file (REVISED 2026-07-04; `op run` RETIRED here —
   no op session-caching on Windows, `§5.2`).** The Vultr key lives in **`~/.temp/vultr.env`**
   (`export VULTR_API_KEY=…`), OUTSIDE the Sync + repo trees, icacls'd to the user only (inheritance
   stripped). Agents **`set -a; . ~/.temp/vultr.env; set +a`** (allexport — the file assigns `VULTR_API_KEY=…`
   WITHOUT `export`, so a plain `.` sets only a shell var the child `vultr-cli` never sees) then run `vultr-cli` (it reads `VULTR_API_KEY` from
   env); the literal is **never inlined, echoed, printed (`env`/`set`/`printenv`), or logged** —
   sourced-into-env only, so it never enters a command line, process arg, or transcript. No GUI popup,
   no timeout (the §5.2 op-run-timeout machinery no longer applies). The key is **expiration-bound**
   (human rotates on expiry/leak); an agent hitting an auth error (`token expired` / 401) treats it as
   a **HARD STOP** — no key ⇒ create no resources — and relays up via `SendMessage "main"` (the
   conductor holds the human-notify path, `§5.2`). **Mechanism proven 2026-07-04** (source→env→
   vultr-cli→API round-trips cleanly); the **key is CONFIRMED WORKING** (full-account auth); the earlier `token expired` was the
   missing-`export` bug (plain `.` didn't reach the child), NOT an aged-out key — the file needs
   `export` (or the `set -a` sourcing above).
5. **Authorization gate.** No paid resource is created until (a) this guardrail is implemented AND
   (b) a collated per-spin plan (how many boxes, what size, expected spend, teardown proof) is
   human-acked. Once auto-teardown + reaper are proven, a *bounded standing* authorization is possible
   ("up to N boxes of size S, always auto-torn-down") so each run needn't re-ask — human's call.
6. **Observability.** A trivial `dorc-r25 status` (live instances + snapshots + a rough accrued-spend
   estimate) any agent or the human can run, so the bill is never a surprise.

### §5.2 · op-run / Vultr connectivity-test findings (2026-07-04, live)

Conductor self-test (`op run` resolves the key — PRESENT, no leak) + an Opus subagent (mise + vultr-cli
+ read-only API + notification-capability + a deliberate op-run-timeout). Results:

- **Subagents CANNOT push-notify (the headline).** No notification/push tool is surfaced to subagents
  (confirmed across ToolSearch name-lookups; `Monitor`'s docs *reference* PushNotification but it's not
  exposed to subagents). A subagent's only cross-party channels are `SendMessage` (agent↔agent, not a
  device push) + its final report. **Consequence:** credentialed / `op run` work must be
  **conductor-owned** (the conductor tier can hold PushNotification), OR **frontloaded into a
  keyboard-window**; a subagent hitting an op-run timeout must relay it up (`SendMessage "main"`) — it
  cannot page the human itself. (Validates the human's earlier frontload instinct.)
- **op-run timeout signature (for the guardrail):** exit **1** + stderr `error initializing client:
  authorization timeout` + ~**60s** block + the child command never runs. Clean + detectable. The
  wrapping runner timeout MUST be **>60s** (use ~300s) or the tool kills op first and masks its own
  error-shape.
- **Each op-run needs a fresh desktop approval** (~13–15s, interaction-shaped; no silent caching).
  **Batch multiple secrets / read-only calls into ONE op-run** to minimize popups (and the human's
  keyboard-time under frontloading).
- **Vultr CLI under mise: WORKS** — `mise use github:vultr/vultr-cli` (NOT `ubi:`, which is deprecated,
  removal mise 2027.1.0); vultr-cli 3.10.0, ~2s.
- **The Vultr key is VALID; IP Access-Control now RESOLVED.** The key's allowlist initially excluded
  the controller's egress IP (auth'd endpoints 401'd; `/v2/regions` was exempt). Human allowlisted the
  IP → re-test CLEAN: `/v2/account` 200, `/v2/regions` 200, `op run` exit 0 — full chain green. NB the
  controller (the op-run machine) calls the Vultr API — the VPSes never do — so it's the controller's
  IP that must stay allowlisted (a new controller/egress IP = re-allowlist).
- Secret discipline held: the key value never echoed/logged/written; git footprint net-zero (mise.toml
  restored).
- **op session does NOT persist on Windows (caching unavailable — confirmed LIVE):** `op signin`
  returns success but the very next process reports `account is not signed in`, so conductor-signin-once
  fails and every op secret-access re-pops the desktop dialog (untenable for an unattended fan-out — the
  popup even steals input focus). **[SUPERSEDED 2026-07-04: substrate REVERSED to Windows/git-bash — human had too much in-flight to
  migrate; `op` sidestepped entirely via the key-file-in-env (`~/.temp/vultr.env`, sourced
  `set -a; . …; set +a`) — see §5.1 + LIVING_STATUS. The macOS resolution below is historical.]**
  **RESOLUTION: the EXECUTION controller moves to macOS** — UNIX has op
  caching, so `op signin` persists across processes + into subagents (native no-churn, signin-once).
  Windows-specific; dissolves on macOS. Bonus: removes MSYS/CRLF friction, matches the `kWINLOCAL`
  nix-controller lean, and runs in the human's *normal* zsh/dotfiles env (valid HHHF signal, no
  unfamiliar-env confound). The extract-scoped-key-to-file workaround is MOOT on macOS. Pending: the
  human confirming his macOS box's prior blocking-bugs are fixed. NB the Vultr IP-allowlist is keyed to
  the home egress IP (likely unchanged behind the same router; re-check on macOS).
- **anti-self-greening (P2):** the observer must be wider than dorc's own model, or the VPS is theatre.
- **turnkey-or-drop (P2):** heavy tracing must not eat the human's day — agent-invisible or dropped.
- **protect-first-contact (Phase B):** fires once, after A3; B never iterates.
- **stopping rules:** A iterates until A3 passes; B is one time-boxed session; a mid-run "clearly
  broken" halts early with the finding banked (don't burn first-contact debugging).
- **don't-over-inflate-(i):** value-locus stays decomposed; the mechanical thirds carry the verdict.

## §6. Owed / open

- Vultr guardrail specifics (dedicated later turn).
- The `+something-else` above-analyzer attribution layer — a B6 *finding*, not pre-built.
- The fleet-plan-view (P7) — built only if P8's loop proves insufficient, discovered live.
- Whether A3's rehearsal needs the real stdlib (P5, r24-tail) or can run on a stub — ~SUSPECT a stub
  suffices to validate P6; confirm when P4 lands.

## §7. Adversarial-review revisions (2026-07-04)

Two Fable crosscheck passes (prompts `quarantine/25xxx`; full adjudication + retrospective ledger in
`notes/254`) converged on one through-line: the protocol was **instrument-rich, decision-poor** — it
built the measuring apparatus but under-specified the decisions and left its greens un-calibrated. Per
the human's ruling (apply trivial sharpenings freely; **at most one non-trivial addition**; small
round), the accepted deltas:

**Trivial pre-registration sharpenings (now binding on Phase A):**
- **Every §4 decision gets a number + an observation→action, set BEFORE the run** (numbers from the
  dry-run below). Vibe-word forks ("high/near-zero/often") are post-hoc-gradeable = the woo-cool
  adversary + a `signal-reducibility` violation.
- **B3:** pre-commit the asymmetry — a negative felt-verdict ("theatre") COUNTS (088 A-VALUE); a
  positive confirms NOTHING (`n1-honesty` + the METR gap). Else the escape-hatch runs backwards.
- **B4:** gate the *yeet-full-elision* decision on **Stage-4/5 having landed**; unbuilt at `2e1fdc0`,
  so a low frequency-number is an artifact of an under-built tier + thin stdlib and would falsely
  confirm the 233-bias B4 exists to check. Until then B4 is evidence-gathering, not a fork.
- **A1:** planted bugs are a **100%-sensitivity GATE**, never a "rate"; organic bites yield
  existence-results only (n≈one-box). Plant with a *different* LLM lineage than P5's author. The
  seeded-bad oracles are a **separate, labeled set, NEVER merged into the Phase-B stdlib** (an unlucky
  merge hands the human a sabotaged stdlib on the one-shot day).
- **Differential:** `∅` is fiction on a real box (fs-diff never empty; world-drift when mirrors refresh
  between runs; probe/apply asymmetry). Add a **world-drift `C-verdict` category**; derive the
  noise-exclusion list **dorc-INDEPENDENTLY via A/A runs** (bare-vs-bare from one snapshot — the
  measured envelope IS the list), valid only while planted canaries still show through it. (Repairs the
  self-greening P2's width-rule guards, relocated into the subtraction. The *build* is Phase-A; the
  *approach* is fixed here.)
- **Gap-log forcing-functions** replace the vague "prompts at seams": a per-agent gap-ledger in every
  Track-A brief; a zsh friction-button (one keystroke → timestamped marker, at the instant); a
  same-evening cued-recall debrief (replay the asciinema, narrate the *why* while memory's hot);
  recorder redundancy (SPOF on a one-shot day).
- **Reinstate the two confound conditions** 251 had and 252 dropped (hand-perfected-vs-LLM oracle;
  scrappy-vs-careful book): `dorc why` *alone* can't separate executor-bug / too-careful-book /
  engine-⊤ / unfamiliarity — the owed `+something-else`, confirmed.
- **Freeze the §2 contracts AFTER this review** (before instruments build against them). Log wallclock
  anyway (088 A-WIN — a free number).

**The one non-trivial addition (per the at-most-one lean): the paper dry-run of the trial book.**
**Target LOCKED (human, 2026-07-04): a single-box mini-homelab — an nginx reverse-proxy fronting
Windmill (+ postgres) and Home Assistant.** (HA is on the human's todo-list and is itself blocked on
Dorc — a genuine dogfood want, the strongest felt-product-validity multiplier; more services = more of
the analyzer's composition machinery exercised.) Before the day, do for this book what USER_STORY did
for the webhost — predicted plan-shape per stage + the perfect-oracle ceiling. Highest-leverage of the
whole review: the *missing denominator* (the homelab mixes base-stdlib-oracle-able infra with genuine
walls — docker, HA's installer — so the design predicts a specific ceiling; without it a low count
misreads as "tool weak"), AND the *pre-registered predictions* (the first bullet), turning the day into
prediction-vs-observation — the strongest anti-woo instrument at n=1. **Repeatability-first (the
differential demands it):** pin versions; local/self-signed certs, never live Let's-Encrypt (network
chaos breaks the diff); HA-Container not Supervised. Real-shell-adjacent ⇒ an Opus single-pass
deliverable, pre-Phase-A.

**Recorded-only / deferred to Phase-A (NOT this round; `notes/254` ledger):** the A/A-calibration +
canary + `086`-sensitivity *build*; the probe-inertness differential arm (the sole test of the "plan
doesn't mutate" weld against real tools); the full drift arm; the `do-4` hot-loop re-apply scenario.
**Discarded as noise:** the "first-contact is already-spent, stop protecting it" reframe.

## §8. First-blood emphasis + findings (2026-07-05)

From the current-doc review (DESIGN/IMPLEMENTATION/USER_STORY) + the OTel re-point + two findings. Human-directed refinements to this live plan.

**Emphasis reframe (human): the first-blood day's SPINE is the felt WORKING experience, not worst-case
failure-hunting.** The docs spend most of their ink fencing the worst-case (the horizon / bought-
unsoundness / un-attributable under-execution) because it's the hard part to *fence* — not because it's
the common part. If the design is good, that corner is small; a one-shot day built to hunt it measures
the tail and under-exercises the body. The day's spine is three felt moments in the *common* case:
- **`dorc why` illuminating** — when a line elided or survived a wall and you ask `why`, does the answer
  *land* ("ah — that's why"), honest and legible, or do you squint? (the core `target-felt-product`).
- **admin-loop reward** — write the one small oracle, re-run `plan`, watch it *shrink* (the +N cascade);
  does it feel like the effort paid?
- **plan-preview trust** — does the elided plan feel like *relief*, or do you re-read the commented-out
  lines anyway (DESIGN priority-3 trust-coupling)?
Adequacy is NOT dropped — it's **mechanized**: the differential + drifted-day runs own adequacy-
correctness silently underneath (fire only if something's wrong), which is what *frees* attention for
the felt-experience. Weight: `target-felt-product` + `target-admin-loop` lead the day; `target-adequacy`
is the background net.

**Finding — `version-guard-lift`** (renamed from the incoherent "u3"): does the built spike lift a
*stdout-consuming* version-check guard (`cmd --version | grep -q X || download`, decision flowing through
stdout) vs only the bare-rc *presence* guard (`dpkg -s || install`) it provably handles? Load-bearing —
all 3 OTel vendor oracles ride it; if it doesn't lift, the vendor-install admin-loop buys nothing.
**STATUS (human, 2026-07-05): an r24 in-flight item** — the value-flow machinery is built + validated,
the final tie-together is pending, scheduled to land BEFORE r25 fires. (An in-repo probe 2026-07-05
confirmed the *current* build does NOT yet lift it — `grep`-on-stdin binds no entity, deciding leaf is
⊤ — consistent with "machinery there, tie-together not done.") NOT an r25-settle item; confirm it lands
(human syncing the r24 conductor). *(Annotation 2026-07-10: LANDED — the `24J` connected-pipes lift;
the tripwire XFAIL promoted GREEN; evidence `24C` §pipe-guard-LIFT.)*

**Finding — multi-wall-cascade → `touches()` is load-bearing.** In the multi-service OTel book, one
stale *middle* vendor (prometheus) re-walls its whole *tail* by CFG position alone — a converged grafana
*past* a running prometheus can't elide — dropping elision ~15→9 on a drifted day. A real box is almost
never fully-converged, so the all-green ceiling is the *rare* day; the typical day is "something mid-book
drifted, tail collapsed." The **footprint tier (`touches()`, stage-5, r24-built)** is exactly the fix
(disjoint footprint ⇒ grafana survives). So the trial's **drifted-day runs exercise `touches()`
load-bearingly** — real validation the footprint tier isn't architecture-astronauting. Scoping: this
validates **`touches()`** (the single-box half); **`reaches()`/`resolve()`** (stages 6-7) are
FLEET-scoped by design (USER_STORY: "do not move a single book's numbers; value shows across the fleet /
multiple authors") and a one-box/one-author trial structurally cannot exercise them — correctly
un-tested here, justified by the collaboration story. (Presumes `version-guard-lift`.)

**Watch-item (background, NOT a day-organizer): the attribution / LLM-authorship collision.**
IMPLEMENTATION's horizon now states "errors we can't attribute are necessarily *our* fault," and the
whole fault-model (concentrate-and-attribute-the-bite) rests on an *answerable human* behind each oracle.
The trial's stdlib is LLM-authored — so a wrong LLM-oracle's silent under-execution attributes to an
author who can't be paged. The `llm-authoring-twist` may quietly break the *attribution* half — which
IMPLEMENTATION now calls the half that matters most. Notice if it bites; don't organize around it.

## §9. Sibling-conductor drift-audit folds (2026-07-05)

A sibling advisor session's corpus drift-audit (EPHEMERAL memo) routed findings; adjudicated
grain-of-salt, verify-not-trust. **Accepted:**

- **memo-1 → in-repo verify errand DISPATCHED** (no VPS, runs `dorc plan` on the book): (a) `set -eu`
  may block *eliding* the bare `apt-get update` hl-1 ⇒ **pred-1 could be 4 not 5** (`strawman24-errexit-
  defeats`=0.00; the `dpkg -s ||` lines are `||`-left errexit-exempt, unaffected); (c) NEW risk — the
  host-guard `case "$(hostname)"` (un-oracled command in a substitution) may ⊤-poison the whole book ⇒
  **Stage-B ~0 elide** (an unmodeled `hostname` walls everything below line 22). NB no stdlib exists yet
  (P5 unbuilt) — so the errand tests the *mechanic* (does an unmodeled `$(hostname)`-in-a-case-guard
  wall downstream?); if it does, the fix is a coverage-REQUIREMENT for when P5's stdlib is written
  (memo-2's book-command-surface point), not a claim about a current stdlib. Errand settles both empirically +
  amends `255 §5` (the owed fold-into-§4 becomes the correction vehicle; anti-woo — a known-wrong
  prediction pre-day contaminates the prediction-vs-observation instrument, 254 F1/F5). **NOT
  re-litigated: u3/pred-2** — the memo confirms it's r24's (pipe-guard XFAIL, doesn't-lift-at-HEAD),
  consistent with §8's r24-owned status.
- **memo-2 → the P5 brief carries an oracle quality-bar checklist** (grep-able slugs, when P5 fires):
  17O regression classes (R2-SHADOW `command -v`≠no-shadow · R2-IDCACHE `getent group` not stale `id` ·
  R2-ORTRUE never read a `||true`-masked rc · F-GETENT-HOSTS `getent hosts`=live-DNS-not-hermetic) ·
  R2-MULTIOP arity-gate-or-decline (ungated multi-operand = priority-1 under-execute) · an-probe-shape
  (no bare `cmd|grep -q` bodies — no-match rc ≡ tool-fail rc; capture the tool's own rc) · the 151-X4
  live bug-classes (regex-live sanitiser: `10.0.0.1` matches `10X0X0X1`; option-passthrough making the
  probe mutate = a kFAIL breach) · verdict-function negative contract (rul-rc-partition: no
  `!`/`||true`/pipeline-tail status-collapse) · **coverage against the BOOK's actual surface**
  (`hostname` [memo-1c], test/`[`, install/ln/chmod/rm — not "trust coreutils") · restate §7's
  A1-seeded-set-disjoint + different-lineage + strawman-vocab. Rationale: 151's lesson — the contract
  needs machine-enforcement, not author-discipline; an LLM author is worse.
- **memo-3 CRLF gate ACCEPTED** (apply-path — P3 amend / P4 requirement): the book is Windows-authored
  (SyncThing pair); a `\r` in the shebang is a kernel-level exec-fail no in-script guard catches
  (F-CRLF). The apply path must verify shipped bytes are LF before the remote `dash` sees them,
  recording any normalization (an-wire-transform).

**Dropped (nack'd):**
- **memo-3 provenance-scope-cut code-comment** (C-run flat `{transcript,rc}` vs 111's locator-DAG) —
  marginal note on already-built throwaway tooling; not worth a touch.
- **memo-1(b)/u3 as an r25 item** — it's r24's (above); not re-opened here.

**Conditional (accepted, checked pre-Phase-B):**
- **memo-4 → B2 confound-guard:** pre-register B2 so "the tool never pointed me anywhere" (a missing
  first-wall hint) routes to **B6 (gap-log: unbuilt feature)**, NEVER to B2's "gradual-enhancement
  thesis in trouble" fork — a missing *feature* must not grade the *thesis*. (Check pre-Phase-B whether
  r24 landed the stage-3 hint line. *Annotation 2026-07-10: LANDED — the first-wall hint with the
  un-wall-M counterfactual, `339189a`; evidence `24C` §firstwall-hint.*)
