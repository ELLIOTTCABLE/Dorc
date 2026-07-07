# 26A — the r26 plan-package crosscheck: adjudication + amendment ledger

AI-authored (Fable conductor = the plans' author; the conflict is why the panel exists),
2026-07-07, round 26. Dual purpose per the `254`/`24Kc` precedent: (1) the adjudicated
verdict on `plans/260`/`261`/`262` @ `361a57f`; (2) a retrospective-attribution record —
every accepted finding carries its disposition, so a later running issue matching a
deferred/rejected item is the signal to revisit. Raw lane reports + the as-run packets +
the conductor's working ledger live in `quarantine-DO-NOT-READ/` (26xxx-*); this note is
the only citable synthesis.

## Setup

Prompt-pair `26xxx` (quarantined; 23xxx-shape, trimmed at human direction: no skill-up
gate, no exclusion lists; human tweaks: READ-ONLY headers, security out-of-scope,
correctness-primary/perf-distant-second, fail-fast-on-technical-errors). Five lanes
dispatched in the `spike3-r26` worktree: Fable adversarial-only (human held the neutral
for usage), DeepSeek V4-Pro ×2 (neutral + adversarial), Codex/GPT-5.5 ×2. **Four
completed:** ds-neutral, ds-adversarial, codex-adversarial (v2 packet), fable-adversarial.
**codex-neutral died** on OpenAI quota after three fail-fast false-alarm half-runs
(dispatch friction, not review content — §Process). Coverage note: three of four completed
lanes are adversarial-stance; the only neutral read is DeepSeek's.

Credal stance: deflationary (crosscheck-adjudication-skepticism); the author adjudicating
his own review discounts flattery and weights code-verified claims over prose claims. The
fable lane verified against spike code + goldens (cited `cli/src/main.rs` lines, re-ran
pin-equivalence); the foreign lanes were mostly prose-level with two exceptions noted.

## The cleared core (lead result — the architecture survived)

No lane could kill the package's shape, and the hostile lanes each said so explicitly
after real kill-attempts (13 withdrawn attacks in the fable lane alone). Multi-lane
cleared: the sans-io fleet kernel + seeded-interleaver DST posture · per-host partition
made unrepresentable (s3-1) · kFAIL phase-keying at the transport (probe-retry-yes /
apply-retry-never) · re-probe-as-the-only-recovery (no resume-cursors, no idempotency
stores) · unreachable-is-never-converged as law+pin · ssh-subprocess-with-reserved-swap ·
the explicit rejection of the r25 trial's security shortcuts · the merge-disjointness
contract's concreteness · standing-rulings compliance across the board (the hostile fable
lane called 261's rec-5/kSTATE timing-cache handling "a model of fence-respecting
deferral"). Citation-fidelity held: every chased citation checked out; several places the
plans adopt prior warnings against themselves were confirmed (22H's vacuity warning, 141's
EOF hazard, 072's redlines).

## The verdict — one stop-the-build item, two pre-stage-3 items

**stop-1 (before spine S1): rewrite 262 §2 — the records-lane wire spec is the package's
one genuinely broken section.** Four lanes, every stance, all three lineages converged on
it; the fable lane found the kill-shot that names WHY: **the spec's universal safety
argument ("record loss folds toward run") is inverted for the deriv lane.** Footprints are
*at-most* claims — losing deriv records shrinks the claim, which licenses MORE survivals
past running walls, in exactly the `--trust-footprints` tier with no runtime net
(under-execution, the cardinal sin). Compounding, each verified: deriv is a variable-count
multi-record family with no completion marker, so the site census cannot detect a
mid-family cut and "received sites keep their facts" keeps the partial footprint (the
engine's safe floor covers only *total* absence); PIPE_BUF line-atomicity is unenforceable
for coordinate content (unbounded tool output — a torn record's leading fragment parses as
a *valid* record with a prefix-truncated coordinate ⇒ wrong disjointness ⇒ wrong
survival); the incumbent parser whitespace-truncates space-bearing coordinates *today*;
the sentinel's `seen=` count is unimplementable by concurrent pure-sh subshells (no shared
counter); the parser's duplicate-record handling is last-write-wins — the exact tie-break
262 §1 says to police forever; and no test tier as specced operates at the byte
granularity where any of this lives. The amendment set (all cheap, ~a day of spec work,
no re-architecture): per-record terminal token (unterminated ⇒ reject); per-task
end-records for variable-count families, partial family ⇒ ⊤/wall-total; coordinate fields
last-to-EOL or length-prefixed; drop `seen=` (controller-side census + leafid accounting +
family end-records replace it); duplicate-record merge-by-meet; sim driver feeds BYTES
through the production deframer with torn/glued/oversize mutations in the fault vocabulary
+ an acceptance pin specifically for partial-deriv ⇒ demote-to-wall.

**stop-2 (before stage-26-3): the severed-apply classification is a law breach as
specced.** The rc-255 ∧ stderr-heuristic conjunction classifies a sever whose stderr
misses the 10-pattern English grep as FailedApply — assumed-failed-and-complete — where
law-fail-direction requires Unknown (the operator won't re-probe a host they believe
merely errored). Fix adopted: a wrapper-level completion sentinel (the remote command line
runs the artifact then prints an end-marker carrying `$?`; artifact bytes stay floored):
marker present ⇒ genuine remote exit; absent ⇒ UnknownAfterLoss regardless of rc/stderr;
the heuristic demotes to diagnosis. Subsumes the EOF-without-exit-status gap.

**stop-3 (before stage-26-3): state and pin the host-identity assumptions.** HostId ↔
physical-host bijection-and-stability across probe→apply is assumed and unverified
(DNS round-robin / DHCP churn ⇒ facts from box X licensing elisions applied to box Y —
not covered by the TOCTOU fence, which is same-host drift). Adopted: host-key continuity
(apply refuses a fingerprint differing from the probe's), verbatim host-list dedupe,
artifact-filename collision refuse, alias-collision named as operator hazard.

## The amendment ledger (accepted; to apply to 260/261/262 on human ack)

- **amend-h1-mechanism** (3 lanes; a genuine composition error): 261 §2 h1's
  "consumer runs in a later wave" is mechanism-free — probe values return to the
  CONTROLLER; a later wave in the same shipped artifact cannot consume them. Rewrite: h1
  edges resolve by (a) in-artifact connected-unit composition (the 24J shape generalized —
  producer value captured host-locally inside ONE compiled unit) or (b) controller-fold
  consumption; waves exist for width/pacing only. Plus (2-lane): the h1-edge extraction
  pass is built at P0/S0 as a real compile step — zero edges on today's corpus as a pin,
  one synthetic injected edge proving compiler→schedule wiring. Antichain-by-proof, not
  by accident.
- **amend-262§1-invariant** (3 independent corrections, one rewrite): scope to *final,
  fold-complete* content (intermediate live-render tightening/loosening is outside it);
  scope to *a fixed compiled artifact* (width legitimately changes artifact bytes); add
  the deadline caveat as a monotone weakening — timing policy may only move content
  *toward run* (plan_B's run-set ⊇ plan_A's), asserted by the rig on deadline-crossing
  seeds; one defaults table (flag default width=1 until the P4 flip package; product
  target 4 — and say plainly the 261 latency win ships dormant this round).
- **amend-wire-honesty** (both adversarial foreign lanes): dec-26-wire-v1 and
  law-transport-shape re-labeled — v1 is the sanctioned degraded-start instance of 142's
  architecture table, and the security property changed from *structural separation* to
  *parser rejection*; name the migration steps to the 142 layout. Design stands for v1
  scale; the claim as written was dishonest.
- **amend-22H-register** (the real kernel of a part-misread CRITICAL): 260 §1/§2 state
  plainly that the 22H engine does NOT exist — this round builds it; what is consumed is
  its *analysis* (static-half purity; merge commutativity — the fable lane re-verified
  commutativity in code at the fact tier), re-verified at stage-26-0; carry 22H's own
  "likely UNDER-scoped" warning into 260's sizing prose.
- **amend-timeouts** (2 lanes): per-task `timeout` for ALL classes where the binary
  exists (drop class-gating; dead-NFS stat-class is the classic hang); document the
  untimed-fallback loss shape; sentinel-on-artifact-timeout (trap) so a wall-clock kill
  yields clean probed-partial instead of losing later waves; state partial-keep semantics
  explicitly.
- **amend-pacing** (1 lane + conductor sharpening): the global open-cap was wrong-shaped —
  MaxStartups binds per-target sshd; redesign as per-target open pacing + the global
  width-cap for controller concurrency; keep a global cap only for the bastion-transit
  case, named.
- **amend-retry-hygiene**: per-attempt nonce (or attempt= key); a retry discards the
  prior attempt's records wholesale; zombie-writer late records un-foldable by nonce.
- **amend-abort-row**: s3-4 gains the operator-abort/controller-death row (SIGINT ⇒ sever
  in-flight ⇒ UnknownAfterLoss per host + bad-news-first summary during shutdown; the
  does-the-remote-die-with-the-channel question answered explicitly; post-crash awareness
  honesty — recovery of *state* is re-probe, recovery of *awareness* is operator memory).
- **amend-smalls**: CRLF gate re-runs on the SHIPPED bytes (apply consumes user-edited
  plan files the parser never saw) · LPT wording = assign-to-least-loaded-lane ·
  `sites=` documented as sites-in-THIS-artifact · fleet-e2e goldens get a RAN_ORDER=lax
  analogue · render aggregation-by-plan-hash noted as the at-scale attention answer +
  one honest consent-is-N-files sentence · privilege assumption stated (probes run as the
  ssh user; trial ran root) · DORC_REPORT's v1 remote story = stderr capture, named ·
  acc-forged-verdict-contained wording trimmed to what exists · h3-lite at P2
  (class-as-resource-key, daemon-class concurrency ~1/wave) · DST-boundary tier (gated
  real-ssh smoke) fattened: real sshd, MaxStartups behavior, sever-mid-line kill.

## Rejected / held (retrospective hooks — a matching later issue re-opens these)

rej-oscillation-breaks-invariant (ds-adv F3 as stated: conflates intermediate render with
final content; the doc-fix absorbs the kernel) · rej-additive-keys-need-negotiation
(same-binary-both-ends at v1; becomes real only for persisted long-lived artifacts) ·
rej-deriv-probes-dont-exist (factually wrong at pin — Stage 4/5 landed 2026-07-04) ·
rej-match-blocks-override-o (~SUSPECT wrong on ssh first-obtained-wins semantics; verify
only if it ever becomes load-bearing) · held-fleet-render-scale (aggregation noted; real
design deferred to the TUI/live round) · held-142-growth-timing (the fable lane: future
stdout-value readback wants the per-leaf-file layout sooner than the plans imply — watch
at the first readback-class oracle).

## Process notes (for the harness ledger + future crosschecks)

The trimmed 23xxx-shape prompt worked; the human's fail-fast note surfaced real
environment friction but codex interpreted it with pathological literalism (four
fail-fast false-alarms: benign HOME git-config warning; literal `Test-Path plans/260`
shorthand; pin-ahead-of-HEAD after packet commits; its own mistyped rg glob) — v2/v3
packets pre-cleared each; the final codex-neutral retry then died on provider quota,
plausibly exhausted by its own half-runs. Pin discipline: minting the pin BEFORE
committing packets moved HEAD past it — either commit packets first or pre-clear
pin-ahead from the start. Two shim stalls (backgrounded foreign run + ended turn)
un-stalled by resume-with-foreground-wait; DeepSeek lanes ran clean start-to-finish.
Lane quality: fable-adversarial (code-verified, found the only CRITICAL that survived
adjudication) > codex-adversarial (one genuine composition error + the crispest honesty
phrasing) ≈ deepseek-neutral (most fix-shaped mediums) > deepseek-adversarial (widest
sweep; both its CRITICALs part-misread but each carried a real kernel). The
verify-the-incumbent behavior the human cut from the packets re-emerged unprompted in
both strong lanes — evidence it belongs in future briefs.
